use miilo_adapter_sdk::intake::IntakeDetails;
#[test]
fn shared_fixture_decodes_and_validates() {
    let details: IntakeDetails =
        serde_json::from_str(include_str!("../../protocol/fixtures/intake-details.json")).unwrap();
    details.validate().unwrap();
    assert_eq!(details.source.external_id, "TASK-42");
}
