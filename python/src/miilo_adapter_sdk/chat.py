"""Text chat contracts. Endpoint credentials are never part of these types."""
from typing import Literal
from pydantic import Field
from .models import Contract

class ChatFocus(Contract):
    work_key: str = Field(min_length=1, max_length=128)
    revision_key: str = Field(min_length=1, max_length=128)

class ChatMessage(Contract):
    role: Literal["user", "assistant"]
    text: str = Field(min_length=1, max_length=4000)

class ChatRequest(Contract):
    protocol_version: Literal["0.1"]
    request_id: str = Field(min_length=1, max_length=80)
    text: str = Field(min_length=1, max_length=4000)
    focus: ChatFocus | None = None
    history: list[ChatMessage] = Field(default_factory=list, max_length=8)

class ChatReference(ChatFocus):
    title: str

class ChatEnvelope(Contract):
    protocol_version: Literal["0.1"]
    request_id: str
    sequence: int = Field(ge=0)

class ChatContextEvent(ChatEnvelope):
    type: Literal["context"]
    status: str
    references: list[ChatReference]
    omitted: int = Field(ge=0)

class ChatTextEvent(ChatEnvelope):
    type: Literal["text_delta"]
    text: str

class ChatDoneEvent(ChatEnvelope):
    type: Literal["done"]

class ChatErrorEvent(ChatEnvelope):
    type: Literal["error"]
    code: str
    message: str

ChatEvent = ChatContextEvent | ChatTextEvent | ChatDoneEvent | ChatErrorEvent
