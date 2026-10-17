import json
from pathlib import Path
import pytest
from pydantic import ValidationError, TypeAdapter
from miilo_adapter_sdk import ChatRequest, ChatEvent

ROOT = Path(__file__).parents[2]
def test_chat_request_and_event_shapes():
    ChatRequest.model_validate_json((ROOT / "protocol/fixtures/chat-request.json").read_text())
    TypeAdapter(ChatEvent).validate_python({"protocolVersion":"0.1","requestId":"a","sequence":0,"type":"done"})
    with pytest.raises(ValidationError):
        ChatRequest(protocolVersion="0.1", requestId="a", text="x", history=[{"role":"system","text":"x"}])
    with pytest.raises(ValidationError):
        TypeAdapter(ChatEvent).validate_python({"protocolVersion":"0.1","requestId":"a","sequence":0,"type":"error"})
