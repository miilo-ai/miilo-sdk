#![cfg(feature = "runtime")]
use chrono::{DateTime, Utc};
use miilo_adapter_sdk::runtime::observation::Observation;
use serde_json::json;

#[test]
fn observation_uses_only_supplied_time_and_epoch() {
    let now: DateTime<Utc> = "2026-01-01T00:00:00Z".parse().unwrap();
    let mut observation = Observation::at(
        "source",
        "Source",
        "fixture",
        &json!({"id":"one"}),
        now,
        "known".into(),
    );
    assert_eq!(observation.observed_at, now.to_rfc3339());
    assert_eq!(observation.cursor(0), "known:0");
    observation.activity("2026-01-01T00:00:01Z", "Public update".into(), "active");
    assert_eq!(observation.events_after(Some("known:0")).unwrap().len(), 1);
    assert!(observation.events_after(Some("other:0")).is_err());
}
