import json
from pathlib import Path
from miilo_adapter_sdk import ObservatoryEvent, ObservatorySnapshot

FIXTURES = Path(__file__).parents[2] / 'protocol' / 'fixtures'

def test_reference_snapshots_are_typed():
    for path in FIXTURES.glob('*.snapshot.json'):
        value = ObservatorySnapshot.model_validate_json(path.read_text())
        assert value.protocol_version == '0.1'
        assert value.providers

def test_reference_events_are_typed():
    for path in FIXTURES.glob('*.events.json'):
        for event in json.loads(path.read_text()):
            assert ObservatoryEvent.model_validate(event).protocol_version == '0.1'
