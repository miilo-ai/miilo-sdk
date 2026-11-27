/** Provider-neutral metadata in WorkItem.extensions[INTAKE_EXTENSION]. */
export const INTAKE_EXTENSION = "io.miilo.intake" as const;
export const INTAKE_VERSION = 1 as const;
export type IntakeDecision = "pending" | "accepted" | "deferred" | "dismissed";
export type IntakeExecution = "not_started" | "queued" | "running" | "checking" | "ready_for_review" | "completed" | "blocked" | "cancelled";
export type IntakeSynchronization = "not_required" | "pending" | "synchronized" | "failed";
export interface IntakeDetails {
  version: typeof INTAKE_VERSION;
  source: { connectionId: string; externalId: string; revision: string };
  decision: IntakeDecision;
  execution: IntakeExecution;
  synchronization: IntakeSynchronization;
  workflowId: string;
  jobId: string | null;
  deferredUntil: string | null;
}
export const intakeActions = { accept: "intake.accept", defer: "intake.defer", dismiss: "intake.dismiss" } as const;
