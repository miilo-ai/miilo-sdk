use miilo_adapter_sdk::{
    MIILO_PROTOCOL_VERSION, ObservatoryEvent, ObservatorySnapshot, VoiceStreamEvent,
};
use serde_json::json;
use std::{fs, path::PathBuf};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../protocol/fixtures")
}

#[test]
fn reference_snapshots_are_typed() {
    for name in ["factory.snapshot.json", "codex.snapshot.json"] {
        let value: ObservatorySnapshot =
            serde_json::from_str(&fs::read_to_string(fixtures().join(name)).unwrap()).unwrap();
        assert_eq!(value.protocol_version, MIILO_PROTOCOL_VERSION);
        assert!(!value.providers.is_empty());
    }
}

#[test]
fn reference_events_are_typed() {
    for name in ["factory.events.json", "codex.events.json"] {
        let values: Vec<ObservatoryEvent> =
            serde_json::from_str(&fs::read_to_string(fixtures().join(name)).unwrap()).unwrap();
        assert!(
            values
                .iter()
                .all(|value| value.protocol_version == MIILO_PROTOCOL_VERSION)
        );
    }
}

#[test]
fn logical_model_routes_are_typed_without_deployment_details() {
    let mut value: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(fixtures().join("factory.snapshot.json")).unwrap(),
    )
    .unwrap();
    value["providers"][0]["capabilities"]["modelRoutes"] = json!([{
        "role": "specialist",
        "logicalName": "leaf-identifier",
        "inputModalities": ["image"],
        "outputModalities": ["text"],
        "specialization": "regional leaf classification"
    }]);
    let snapshot: ObservatorySnapshot = serde_json::from_value(value).unwrap();
    let route = &snapshot.providers[0].capabilities.model_routes[0];
    assert_eq!(route.logical_name, "leaf-identifier");
    assert_eq!(
        route.specialization.as_deref(),
        Some("regional leaf classification")
    );
}

#[test]
fn voice_stream_fixture_is_typed_and_terminal() {
    let events: Vec<VoiceStreamEvent> = serde_json::from_str(
        &fs::read_to_string(fixtures().join("voice-stream.events.json")).unwrap(),
    )
    .unwrap();
    assert!(matches!(
        events.first(),
        Some(VoiceStreamEvent::Transcript { .. })
    ));
    assert!(matches!(events.last(), Some(VoiceStreamEvent::Done { .. })));
}
