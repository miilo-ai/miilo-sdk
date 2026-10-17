use miilo_adapter_sdk::UserConfiguration;
use serde_json::{Value, json};

fn valid(value: Value) -> bool {
    serde_json::from_value::<UserConfiguration>(value)
        .is_ok_and(|configuration| configuration.validate().is_ok())
}

#[test]
fn accepts_minimal_and_absolute_native_configuration() {
    assert!(valid(json!({"schemaVersion": 1})));
    assert!(valid(
        json!({"schemaVersion": 1, "localConfigFile": "/fixture/local.json"})
    ));
}

#[test]
fn rejects_missing_future_malformed_and_relative_configuration() {
    for value in [
        json!({}),
        json!({"schemaVersion": 2}),
        json!({"schemaVersion": "1"}),
        json!({"schemaVersion": 1, "localConfigFile": "relative.json"}),
        json!({"schemaVersion": 1, "localConfigFile": ""}),
        json!({"schemaVersion": 1, "localConfigFile": null}),
        json!({"schemaVersion": 1, "localConfigFile": 42}),
        json!([]),
        json!(null),
    ] {
        assert!(!valid(value.clone()), "unexpectedly accepted {value}");
    }
}

#[test]
fn preserves_unknown_settings_and_omits_absent_local_path() {
    let value = json!({"schemaVersion": 1, "future": {"enabled": true}});
    let configuration: UserConfiguration = serde_json::from_value(value.clone()).unwrap();
    configuration.validate().unwrap();
    assert_eq!(serde_json::to_value(configuration).unwrap(), value);
}

#[test]
fn schema_documents_the_same_manifest_version_and_path_contract() {
    let schema: Value = serde_json::from_str(include_str!(
        "../../protocol/schemas/user-configuration.schema.json"
    ))
    .unwrap();
    assert_eq!(schema["properties"]["schemaVersion"]["const"], 1);
    assert_eq!(schema["properties"]["localConfigFile"]["type"], "string");
    assert_eq!(schema["properties"]["localConfigFile"]["pattern"], "^/");
    assert_eq!(schema["additionalProperties"], true);
}
