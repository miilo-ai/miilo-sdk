use super::domain::{
    AttributeValue, CompletedOperation, MappedSpan, Operation, Outcome, SpanKind, TelemetryError,
};

fn label(value: &str) -> Result<String, TelemetryError> {
    if value.is_empty()
        || value.len() > 128
        || value.starts_with(['/', '.'])
        || value.contains("://")
        || !value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"._-/:".contains(&c))
    {
        return Err(TelemetryError(
            "Use a bounded machine label, not content, a URL or a path.",
        ));
    }
    Ok(value.into())
}

pub(super) fn map(record: &CompletedOperation) -> Result<MappedSpan, TelemetryError> {
    if record.ended_at < record.started_at
        || record.started_at.timestamp() < 0
        || record.started_at.timestamp_nanos_opt().is_none()
        || record.ended_at.timestamp_nanos_opt().is_none()
    {
        return Err(TelemetryError("Supply valid ordered operation timestamps."));
    }
    let mut attributes = Vec::new();
    let mut text = |key, value: &str| -> Result<(), TelemetryError> {
        attributes.push((key, AttributeValue::Text(label(value)?)));
        Ok(())
    };
    if let Some(provider) = &record.provider {
        text("gen_ai.provider.name", provider)?;
    } else if !matches!(record.operation, Operation::ExecuteTool { .. }) {
        return Err(TelemetryError(
            "Inference and agent operations require a known provider.",
        ));
    }
    let (operation, target, kind) = match &record.operation {
        Operation::Inference { kind, details } => {
            if let Some(model) = &details.model {
                text("gen_ai.request.model", model)?;
            }
            if let Some(model) = &details.response_model {
                text("gen_ai.response.model", model)?;
            }
            (kind.name(), details.model.as_deref(), SpanKind::Client)
        }
        Operation::InvokeAgent { name, remote } => {
            if let Some(name) = name {
                text("gen_ai.agent.name", name)?;
            }
            (
                "invoke_agent",
                name.as_deref(),
                if *remote {
                    SpanKind::Client
                } else {
                    SpanKind::Internal
                },
            )
        }
        Operation::ExecuteTool { name } => {
            text("gen_ai.tool.name", name)?;
            ("execute_tool", Some(name.as_str()), SpanKind::Internal)
        }
    };
    attributes.push((
        "gen_ai.operation.name",
        AttributeValue::Text(operation.into()),
    ));
    if let Operation::Inference { details, .. } = &record.operation {
        for (key, count) in [
            ("gen_ai.usage.input_tokens", details.usage.input),
            ("gen_ai.usage.output_tokens", details.usage.output),
        ] {
            if let Some(count) = count {
                let count = i64::try_from(count).map_err(|_| {
                    TelemetryError("Token count exceeds the OpenTelemetry integer range.")
                })?;
                attributes.push((key, AttributeValue::Integer(count)));
            }
        }
        if let Some(streaming) = details.streaming {
            attributes.push(("gen_ai.request.stream", AttributeValue::Boolean(streaming)));
        }
    }
    match record.outcome {
        Outcome::Error(error) => {
            attributes.push(("error.type", AttributeValue::Text(error.name().into())))
        }
        Outcome::Cancelled => {
            attributes.push(("miilo.operation.cancelled", AttributeValue::Boolean(true)))
        }
        Outcome::Success => {}
    }
    Ok(MappedSpan {
        name: target.map_or_else(
            || operation.into(),
            |target| format!("{operation} {target}"),
        ),
        kind,
        attributes,
        started_at: record.started_at,
        ended_at: record.ended_at,
        outcome: record.outcome,
    })
}
