import json
from pathlib import Path
from pydantic import TypeAdapter
from miilo_adapter_sdk import ObservatoryEvent, ObservatorySnapshot, VoiceStreamEvent

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
