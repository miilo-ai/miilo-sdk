from __future__ import annotations

import json
import copy
from pathlib import Path

import pytest
from jsonschema import Draft202012Validator, FormatChecker


ROOT = Path(__file__).parents[1]
SCHEMAS = ROOT / "schemas"
FIXTURES = ROOT / "fixtures"


def load(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def validate(schema_name: str, value) -> None:
    validator = Draft202012Validator(
        load(SCHEMAS / schema_name), format_checker=FormatChecker()
    )
    errors = sorted(validator.iter_errors(value), key=lambda error: list(error.path))
    assert not errors, "\n".join(
        f"{'.'.join(str(value) for value in error.path) or '<root>'}: {error.message}"
        for error in errors
    )


@pytest.mark.parametrize("name", ["factory.snapshot.json", "codex.snapshot.json"])
def test_provider_snapshots_share_one_contract(name: str) -> None:
    validate("observatory-snapshot.schema.json", load(FIXTURES / name))


@pytest.mark.parametrize("name", ["factory.events.json", "codex.events.json"])
def test_provider_events_share_one_contract(name: str) -> None:
    for event in load(FIXTURES / name):
        validate("observatory-event.schema.json", event)


def test_action_contracts_are_provider_owned_and_revision_bound() -> None:
    invocation = load(FIXTURES / "action-invocation.json")
    result = load(FIXTURES / "action-result.json")
    validate("action-invocation.schema.json", invocation)
    validate("action-result.schema.json", result)
    assert invocation["requestId"] == result["requestId"]
    assert invocation["providerId"] == result["providerId"]
    assert invocation["actionId"] == result["actionId"]
    assert invocation["expectedRevision"]


@pytest.mark.parametrize("name", ["factory.snapshot.json", "codex.snapshot.json"])
def test_snapshot_referential_integrity(name: str) -> None:
    snapshot = load(FIXTURES / name)
    provider_ids = {value["id"] for value in snapshot["providers"]}
    work_ids = {value["id"] for value in snapshot["workItems"]}
    actor_ids = {value["id"] for value in snapshot["actors"]}
    attempt_ids = {value["id"] for value in snapshot["attempts"]}
    artifact_ids = {value["id"] for value in snapshot["artifacts"]}

    for collection in ("workItems", "actors", "attempts", "artifacts", "evidence", "attention"):
        assert all(value["providerId"] in provider_ids for value in snapshot[collection])
    assert all(value["taskId"] in work_ids for value in snapshot["attempts"])
    assert all(set(value["actorIds"]).issubset(actor_ids) for value in snapshot["attempts"])
    assert all(value["attemptId"] in attempt_ids for value in snapshot["artifacts"])
    assert all(value["attemptId"] in attempt_ids for value in snapshot["evidence"])
    assert all(set(value.get("artifactIds", [])).issubset(artifact_ids) for value in snapshot["evidence"])


def test_provider_extensions_do_not_change_normalized_state() -> None:
    snapshot = load(FIXTURES / "codex.snapshot.json")
    item = snapshot["workItems"][0]
    assert item["providerState"] == "running"
    assert item["lifecycle"] == "active"
    assert item["phase"] == "implementing"
    assert item["attention"] == "none"


def test_actions_require_confirmation_for_non_read_risk() -> None:
    for name in ("factory.snapshot.json", "codex.snapshot.json"):
        snapshot = load(FIXTURES / name)
        actions = [action for item in snapshot["workItems"] for action in item["actions"]]
        actions += [action for item in snapshot["attention"] for action in item["actions"]]
        assert all(
            action["confirmation"] != "none"
            for action in actions
            if action["risk"] != "read"
        )


def validation_errors(schema_name: str, value) -> list[str]:
    validator = Draft202012Validator(
        load(SCHEMAS / schema_name), format_checker=FormatChecker()
    )
    return [error.message for error in validator.iter_errors(value)]


def test_rejects_unknown_protocol_major_or_minor() -> None:
    snapshot = load(FIXTURES / "factory.snapshot.json")
    snapshot["protocolVersion"] = "1.0"
    assert validation_errors("observatory-snapshot.schema.json", snapshot)


def test_rejects_privileged_action_without_explicit_confirmation() -> None:
    snapshot = load(FIXTURES / "factory.snapshot.json")
    snapshot["workItems"][0]["actions"][0]["confirmation"] = "none"
    assert validation_errors("observatory-snapshot.schema.json", snapshot)


def test_stale_revision_is_detectable_before_dispatch() -> None:
    snapshot = load(FIXTURES / "codex.snapshot.json")
    invocation = load(FIXTURES / "action-invocation.json")
    invocation["expectedRevision"] = "turn:2"
    item = snapshot["workItems"][0]
    assert invocation["expectedRevision"] != item["revision"]


def test_duplicate_event_identity_is_provider_scoped() -> None:
    factory = load(FIXTURES / "factory.events.json")[0]
    codex = copy.deepcopy(factory)
    codex["providerId"] = "codex-local"
    identities = {(value["providerId"], value["eventId"]) for value in (factory, codex)}
    assert len(identities) == 2
