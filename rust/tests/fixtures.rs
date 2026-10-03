use miilo_adapter_sdk::{ObservatoryEvent, ObservatorySnapshot, MIILO_PROTOCOL_VERSION};
use std::{fs, path::PathBuf};

fn fixtures() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../protocol/fixtures") }

#[test]
fn reference_snapshots_are_typed() {
    for name in ["factory.snapshot.json", "codex.snapshot.json"] {
        let value: ObservatorySnapshot = serde_json::from_str(&fs::read_to_string(fixtures().join(name)).unwrap()).unwrap();
        assert_eq!(value.protocol_version, MIILO_PROTOCOL_VERSION);
        assert!(!value.providers.is_empty());
    }
}

#[test]
fn reference_events_are_typed() {
    for name in ["factory.events.json", "codex.events.json"] {
        let values: Vec<ObservatoryEvent> = serde_json::from_str(&fs::read_to_string(fixtures().join(name)).unwrap()).unwrap();
        assert!(values.iter().all(|value| value.protocol_version == MIILO_PROTOCOL_VERSION));
    }
}
