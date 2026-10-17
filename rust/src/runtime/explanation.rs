use super::RuntimeError;
use crate::ObservatorySnapshot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroundedExplanation {
    pub provider_id: String,
    pub work_item_id: String,
    pub revision: String,
    pub title: String,
    pub summary: String,
    pub observed_at: String,
    pub facts: Vec<String>,
    pub evidence_ids: Vec<String>,
    pub limitations: Vec<String>,
}
pub fn explain(
    snapshot: &ObservatorySnapshot,
    provider: &str,
    work: &str,
) -> Result<GroundedExplanation, RuntimeError> {
    let item = snapshot
        .work_items
        .iter()
        .find(|w| w.provider_id == provider && w.id == work)
        .ok_or_else(|| RuntimeError::NotRegistered(format!("{provider}/{work}")))?;
    let attempts: Vec<_> = snapshot
        .attempts
        .iter()
        .filter(|a| a.provider_id == provider && a.task_id == work)
        .map(|a| a.id.as_str())
        .collect();
    let evidence_ids: Vec<_> = snapshot
        .evidence
        .iter()
        .filter(|e| e.provider_id == provider && attempts.contains(&e.attempt_id.as_str()))
        .map(|e| e.id.clone())
        .collect();
    let mut limitations =
        vec!["Observed activity is not proof of current execution or permission to act.".into()];
    if evidence_ids.is_empty() {
        limitations.push("No verification evidence was supplied for this work item.".into());
    }
    Ok(GroundedExplanation {
        provider_id: provider.into(),
        work_item_id: work.into(),
        revision: item.revision.clone(),
        title: item.title.clone(),
        summary: item.summary.clone(),
        observed_at: item.updated_at.to_rfc3339(),
        facts: vec![
            format!("Provider state: {}", item.provider_state),
            format!("Observed revision: {}", item.revision),
        ],
        evidence_ids,
        limitations,
    })
}
