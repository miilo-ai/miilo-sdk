#![cfg(feature = "runtime")]
use chrono::Utc;
use miilo_adapter_sdk::{
    ObservatorySnapshot,
    runtime::{ObservatoryRuntime, RuntimeError, explain},
};
fn fixture(name: &str) -> ObservatorySnapshot {
    serde_json::from_str(
        &std::fs::read_to_string(format!(
            "{}/../protocol/fixtures/{name}.snapshot.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap(),
    )
    .unwrap()
}
#[test]
fn ingestion_is_idempotent_and_provider_removal_is_isolated() {
    let now = Utc::now();
    let mut runtime = ObservatoryRuntime::new("test", 8).unwrap();
    let a = fixture("factory");
    let b = fixture("codex");
    let aid = a.providers[0].id.clone();
    let bid = b.providers[0].id.clone();
    for snapshot in [&a, &b] {
        runtime
            .register(snapshot.providers[0].clone(), now)
            .unwrap();
        runtime
            .replace(&snapshot.providers[0].id, snapshot.clone(), now)
            .unwrap();
    }
    let cursor = runtime.cursor();
    assert!(runtime.replace(&aid, a.clone(), now).unwrap().is_none());
    assert_eq!(cursor, runtime.cursor());
    assert_eq!(runtime.snapshot(now).providers.len(), 2);
    runtime.disconnect(&aid, now).unwrap();
    let snapshot = runtime.snapshot(now);
    assert_eq!(snapshot.providers[0].id, bid);
    assert!(snapshot.work_items.iter().all(|w| w.provider_id == bid));
}
#[test]
fn history_loss_future_and_restart_require_reconciliation() {
    let mut runtime = ObservatoryRuntime::new("boot", 1).unwrap();
    let a = fixture("factory");
    let id = a.providers[0].id.clone();
    let now = Utc::now();
    runtime.register(a.providers[0].clone(), now).unwrap();
    let initial = runtime.cursor();
    runtime.replace(&id, a, now).unwrap();
    assert_eq!(runtime.events_after(Some(&initial)).unwrap().len(), 1);
    for cursor in ["boot:0", "boot:999", "other:1", "garbage"] {
        assert!(matches!(
            runtime.events_after(Some(cursor)),
            Err(RuntimeError::CursorExpired)
        ));
    }
}
#[test]
fn invalid_ingestion_cannot_mutate_state_or_history() {
    let mut runtime = ObservatoryRuntime::new("boot", 8).unwrap();
    let mut a = fixture("factory");
    let id = a.providers[0].id.clone();
    let now = Utc::now();
    runtime.register(a.providers[0].clone(), now).unwrap();
    let cursor = runtime.cursor();
    a.work_items[0].provider_id = "another".into();
    assert!(runtime.replace(&id, a, now).is_err());
    assert_eq!(cursor, runtime.cursor());
    assert!(runtime.snapshot(now).work_items.is_empty());
}
#[test]
fn explanations_only_reference_supplied_evidence() {
    let mut a = fixture("factory");
    let w = &a.work_items[0];
    let provider = w.provider_id.clone();
    let work = w.id.clone();
    let explanation = explain(&a, &provider, &work).unwrap();
    assert!(
        explanation
            .evidence_ids
            .iter()
            .all(|id| a.evidence.iter().any(|e| &e.id == id))
    );
    a.evidence.clear();
    let explanation = explain(&a, &provider, &work).unwrap();
    assert!(explanation.evidence_ids.is_empty());
    assert!(
        explanation
            .limitations
            .iter()
            .any(|s| s.contains("No verification"))
    );
    assert!(explain(&a, "another", &work).is_err());
}
