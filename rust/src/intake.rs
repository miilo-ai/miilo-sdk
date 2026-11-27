//! Provider-neutral intake projections. Actions retain the existing SDK protocol.
use serde::{Deserialize, Serialize};

pub const EXTENSION: &str = "io.miilo.intake";
pub const VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceReference {
    pub connection_id: String,
    pub external_id: String,
    pub revision: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionState {
    Pending,
    Accepted,
    Deferred,
    Dismissed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionState {
    NotStarted,
    Queued,
    Running,
    Checking,
    ReadyForReview,
    Completed,
    Blocked,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SynchronizationState {
    NotRequired,
    Pending,
    Synchronized,
    Failed,
}

/// Versioned metadata in WorkItem.extensions[EXTENSION]. This contains no credentials.
/// Unknown versions must not enable client actions. Server policy remains authoritative.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IntakeDetails {
    pub version: u32,
    pub source: SourceReference,
    pub decision: DecisionState,
    pub execution: ExecutionState,
    pub synchronization: SynchronizationState,
    pub workflow_id: String,
    pub job_id: Option<String>,
    pub deferred_until: Option<String>,
}

impl IntakeDetails {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.version != VERSION {
            return Err("unsupported intake version");
        }
        if [
            &self.source.connection_id,
            &self.source.external_id,
            &self.source.revision,
            &self.workflow_id,
        ]
        .iter()
        .any(|v| v.trim().is_empty() || v.len() > 512)
        {
            return Err("missing or oversized intake identity");
        }
        Ok(())
    }
}

/// Semantic identifiers; adapters determine native transitions and actual availability.
pub mod action {
    pub const ACCEPT: &str = "intake.accept";
    pub const DEFER: &str = "intake.defer";
    pub const DISMISS: &str = "intake.dismiss";
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn future_versions_are_not_actionable() {
        let mut value = IntakeDetails {
            version: VERSION,
            source: SourceReference {
                connection_id: "source-one".into(),
                external_id: "task-1".into(),
                revision: "r1".into(),
            },
            decision: DecisionState::Pending,
            execution: ExecutionState::NotStarted,
            synchronization: SynchronizationState::NotRequired,
            workflow_id: "code-review".into(),
            job_id: None,
            deferred_until: None,
        };
        assert!(value.validate().is_ok());
        value.version = 2;
        assert!(value.validate().is_err());
    }
    #[test]
    fn scopes_and_source_identity_survive_roundtrip() {
        let value = SourceReference {
            connection_id: "tenant-a-source".into(),
            external_id: "1".into(),
            revision: "a".into(),
        };
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(
            serde_json::from_str::<SourceReference>(&json).unwrap(),
            value
        );
        assert!(
            serde_json::from_str::<SourceReference>(
                r#"{"connectionId":"a","externalId":"1","revision":"a","token":"secret"}"#
            )
            .is_err()
        );
    }
}
