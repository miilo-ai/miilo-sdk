import json
from pathlib import Path
import pytest
from pydantic import ValidationError
from miilo_adapter_sdk import IntakeDetails

def test_shared_intake_fixture_and_unsupported_version():
    data = json.loads((Path(__file__).parents[2] / "protocol/fixtures/intake-details.json").read_text())
    value = IntakeDetails.model_validate(data)
    assert value.model_dump(by_alias=True) == data
    data["version"] = 2
    with pytest.raises(ValidationError):
        IntakeDetails.model_validate(data)
