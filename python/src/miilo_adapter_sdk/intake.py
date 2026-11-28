"""Typed intake metadata; execution and source policy belong to the backend."""
from typing import Literal
from pydantic import Field, field_validator
from .models import Contract

INTAKE_EXTENSION = "io.miilo.intake"
INTAKE_VERSION = 1

class IntakeSource(Contract):
    connection_id: str = Field(min_length=1, max_length=512)
    external_id: str = Field(min_length=1, max_length=512)
    revision: str = Field(min_length=1, max_length=512)

    @field_validator("connection_id", "external_id", "revision")
    @classmethod
    def nonblank(cls, value: str) -> str:
        if not value.strip():
            raise ValueError("Identity must not be blank")
        return value

class IntakeDetails(Contract):
    version: Literal[1]
    source: IntakeSource
    decision: Literal["pending", "accepted", "deferred", "dismissed"]
    execution: Literal["not_started", "queued", "running", "checking", "ready_for_review", "completed", "blocked", "cancelled"]
    synchronization: Literal["not_required", "pending", "synchronized", "failed"]
    workflow_id: str = Field(min_length=1, max_length=512)
    job_id: str | None
    deferred_until: str | None

    @field_validator("workflow_id")
    @classmethod
    def nonblank(cls, value: str) -> str:
        if not value.strip():
            raise ValueError("Workflow must not be blank")
        return value
