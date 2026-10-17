use super::RuntimeError;
use crate::{
    ActionRisk, ConfirmationRequirement, MIILO_PROTOCOL_VERSION, ObservatorySnapshot, SubjectKind,
};
use std::collections::HashSet;

pub fn validate_snapshot(
    provider_id: &str,
    snapshot: &ObservatorySnapshot,
) -> Result<(), RuntimeError> {
    if snapshot.protocol_version != MIILO_PROTOCOL_VERSION {
        return invalid(format!(
            "protocol version {} is unsupported; expected {MIILO_PROTOCOL_VERSION}",
            snapshot.protocol_version
        ));
    }
    if snapshot.providers.len() != 1 || snapshot.providers[0].id != provider_id {
        return invalid("snapshot must describe exactly the provider in the request path");
    }
    if snapshot
        .work_items
        .iter()
        .any(|item| item.provider_id != provider_id)
        || snapshot
            .actors
            .iter()
            .any(|item| item.provider_id != provider_id)
        || snapshot
            .attempts
            .iter()
            .any(|item| item.provider_id != provider_id)
        || snapshot
            .artifacts
            .iter()
            .any(|item| item.provider_id != provider_id)
        || snapshot
            .evidence
            .iter()
            .any(|item| item.provider_id != provider_id)
        || snapshot
            .attention
            .iter()
            .any(|item| item.provider_id != provider_id)
    {
        return Err(RuntimeError::ProviderBoundary(provider_id.into()));
    }

    let work = unique(
        "work item",
        snapshot.work_items.iter().map(|item| item.id.as_str()),
    )?;
    let actors = unique("actor", snapshot.actors.iter().map(|item| item.id.as_str()))?;
    let attempts = unique(
        "attempt",
        snapshot.attempts.iter().map(|item| item.id.as_str()),
    )?;
    let artifacts = unique(
        "artifact",
        snapshot.artifacts.iter().map(|item| item.id.as_str()),
    )?;
    let evidence = unique(
        "evidence",
        snapshot.evidence.iter().map(|item| item.id.as_str()),
    )?;
    let attention = unique(
        "attention item",
        snapshot.attention.iter().map(|item| item.id.as_str()),
    )?;

    for item in &snapshot.work_items {
        required("work item revision", &item.revision)?;
        if item
            .dependency_ids
            .iter()
            .any(|dependency| dependency == &item.id)
        {
            return invalid(format!("work item {} depends on itself", item.id));
        }
        references("work dependency", &item.dependency_ids, &work)?;
        validate_actions(&item.actions)?;
    }
    for attempt in &snapshot.attempts {
        reference("attempt task", &attempt.task_id, &work)?;
        references("attempt actor", &attempt.actor_ids, &actors)?;
    }
    for artifact in &snapshot.artifacts {
        reference("artifact attempt", &artifact.attempt_id, &attempts)?;
    }
    for item in &snapshot.evidence {
        reference("evidence attempt", &item.attempt_id, &attempts)?;
        references(
            "evidence artifact",
            item.artifact_ids.as_deref().unwrap_or_default(),
            &artifacts,
        )?;
    }
    for item in &snapshot.attention {
        validate_subject(
            &item.subject.kind,
            &item.subject.id,
            provider_id,
            &work,
            &actors,
            &attempts,
            &artifacts,
            &evidence,
            &attention,
        )?;
        validate_actions(&item.actions)?;
    }
    Ok(())
}

fn unique<'a>(
    label: &str,
    values: impl Iterator<Item = &'a str>,
) -> Result<HashSet<&'a str>, RuntimeError> {
    let mut found = HashSet::new();
    for value in values {
        required(&format!("{label} id"), value)?;
        if !found.insert(value) {
            return invalid(format!("duplicate {label} id {value}"));
        }
    }
    Ok(found)
}

fn required(label: &str, value: &str) -> Result<(), RuntimeError> {
    if value.trim().is_empty() {
        invalid(format!("{label} must not be empty"))
    } else {
        Ok(())
    }
}

fn reference(label: &str, value: &str, valid: &HashSet<&str>) -> Result<(), RuntimeError> {
    if valid.contains(value) {
        Ok(())
    } else {
        invalid(format!("{label} references missing id {value}"))
    }
}

fn references(label: &str, values: &[String], valid: &HashSet<&str>) -> Result<(), RuntimeError> {
    for value in values {
        reference(label, value, valid)?;
    }
    Ok(())
}

fn validate_actions(actions: &[crate::AvailableAction]) -> Result<(), RuntimeError> {
    unique("action", actions.iter().map(|action| action.id.as_str()))?;
    for action in actions {
        if !matches!(action.risk, ActionRisk::Read)
            && matches!(action.confirmation, ConfirmationRequirement::None)
        {
            return invalid(format!(
                "action {} requires confirmation for its risk",
                action.id
            ));
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn validate_subject(
    kind: &SubjectKind,
    id: &str,
    provider_id: &str,
    work: &HashSet<&str>,
    actors: &HashSet<&str>,
    attempts: &HashSet<&str>,
    artifacts: &HashSet<&str>,
    evidence: &HashSet<&str>,
    attention: &HashSet<&str>,
) -> Result<(), RuntimeError> {
    match kind {
        SubjectKind::Provider if id == provider_id => Ok(()),
        SubjectKind::Initiative | SubjectKind::Task => reference("attention subject", id, work),
        SubjectKind::Actor => reference("attention subject", id, actors),
        SubjectKind::Attempt => reference("attention subject", id, attempts),
        SubjectKind::Artifact => reference("attention subject", id, artifacts),
        SubjectKind::Evidence => reference("attention subject", id, evidence),
        SubjectKind::Attention => reference("attention subject", id, attention),
        SubjectKind::Workspace => required("workspace subject id", id),
        SubjectKind::Provider => invalid(format!("attention references provider {id}")),
    }
}

fn invalid<T>(message: impl Into<String>) -> Result<T, RuntimeError> {
    Err(RuntimeError::InvalidSnapshot(message.into()))
}
