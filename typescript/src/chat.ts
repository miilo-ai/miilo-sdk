/** Launcher text chat; no tool execution or credential transport. */
export interface ChatFocus { workKey: string; revisionKey: string }
export interface ChatMessage { role: "user" | "assistant"; text: string }
export interface ChatRequest {
  protocolVersion: "0.1"; requestId: string; text: string;
  focus?: ChatFocus | null; history?: readonly ChatMessage[];
}
export interface ChatReference extends ChatFocus { title: string }
interface ChatEnvelope { protocolVersion: "0.1"; requestId: string; sequence: number }
export type ChatEvent = ChatEnvelope & (
  | { type: "context"; status: string; references: readonly ChatReference[]; omitted: number }
  | { type: "text_delta"; text: string }
  | { type: "done" }
  | { type: "error"; code: string; message: string }
);
