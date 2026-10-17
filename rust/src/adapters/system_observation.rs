use crate::runtime::observation::Observation;
use chrono::Utc;
use serde_json::Value;

impl Observation {
    /// System-clock convenience adapter. Deterministic callers use `Observation::at`.
    pub fn new(provider_id: &str, provider_name: &str, source: &str, thread: &Value) -> Self {
        let now = Utc::now();
        let epoch = now.timestamp_nanos_opt().unwrap_or_default().to_string();
        Self::at(provider_id, provider_name, source, thread, now, epoch)
    }
}
