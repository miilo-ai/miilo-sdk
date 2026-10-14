import json
from pathlib import Path
from pydantic import TypeAdapter
from miilo_adapter_sdk import (
    ActionInvocation,
    ActionResult,
    ObservatoryEvent,
    ObservatorySnapshot,
    VoiceStreamEvent,
    VoiceTextTurnRequest,
)

FIXTURES = Path(__file__).parents[2] / 'protocol' / 'fixtures'

def test_reference_snapshots_are_typed():
    for path in FIXTURES.glob('*.snapshot.json'):
        value = ObservatorySnapshot.model_validate_json(path.read_text())
        assert value.protocol_version == '0.1'
        assert value.providers

def test_reference_events_are_typed():
    for path in (FIXTURES / 'factory.events.json', FIXTURES / 'codex.events.json'):
        for event in json.loads(path.read_text()):
            assert ObservatoryEvent.model_validate(event).protocol_version == '0.1'

def test_logical_model_routes_are_typed_without_deployment_details():
    snapshot = json.loads((FIXTURES / 'factory.snapshot.json').read_text())
    snapshot['providers'][0]['capabilities']['modelRoutes'] = [{
        'role': 'specialist',
        'logicalName': 'leaf-identifier',
        'inputModalities': ['image'],
        'outputModalities': ['text'],
        'specialization': 'regional leaf classification',
    }]
    value = ObservatorySnapshot.model_validate(snapshot)
    route = value.providers[0].capabilities.model_routes[0]
    assert route.logical_name == 'leaf-identifier'
    assert route.specialization == 'regional leaf classification'

def test_voice_stream_fixture_is_typed_and_terminal():
    events = TypeAdapter(list[VoiceStreamEvent]).validate_json((FIXTURES / 'voice-stream.events.json').read_text())
    assert events[0].type == 'transcript'
    assert events[-1].type == 'done'

def test_action_and_voice_request_fixtures_are_typed():
    invocation = ActionInvocation.model_validate_json((FIXTURES / 'action-invocation.json').read_text())
    result = ActionResult.model_validate_json((FIXTURES / 'action-result.json').read_text())
    request = VoiceTextTurnRequest.model_validate_json((FIXTURES / 'voice-text-turn-request.json').read_text())
    assert invocation.request_id == result.request_id
    assert invocation.expected_revision
    assert request.protocol_version == '0.1'

def test_contracts_reject_extra_fields_and_protocol_drift():
    snapshot = json.loads((FIXTURES / 'factory.snapshot.json').read_text())
    snapshot['unexpected'] = True
    try:
        ObservatorySnapshot.model_validate(snapshot)
    except ValueError:
        pass
    else:
        raise AssertionError('extra snapshot field was accepted')

    request = json.loads((FIXTURES / 'voice-text-turn-request.json').read_text())
    request['protocolVersion'] = '1.0'
    try:
        VoiceTextTurnRequest.model_validate(request)
    except ValueError:
        pass
    else:
        raise AssertionError('unsupported protocol version was accepted')
