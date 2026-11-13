# Specification: Real Routed Provider Execution in `AcpRunner` (`FUS-F001` Slice 1)

- **Document ID**: `fusion-real-agent-provider-20261108`
- **Track**: `FUS-F001` — Real Fabushi Autonomous Agent Runtime & ACP Session Lifecycle
- **Date**: 2026-11-08
- **Status**: Active Implementation Slice (Crate-Scoped to `crates/gpui_telegram_agent`)

---

## 1. Problem Statement & Audit Findings

The read-only `FUS-F001` reachability audit established that while `mahayana-host-runtime` (`inherited/host`) contains a complete, production-grade routed provider runtime (`SandAgentRunner::run_routed_provider_with_options` -> `ProductionTurnAgentOwner::run_routed_provider_with_projected_turn_input` -> `TurnAgentComposition::run` -> `run_routed_provider_in_runner` -> `run_routed_provider_text_with_lifecycle_reporting_usage_with_max_steps`), the wrapper in `crates/gpui_telegram_agent/src/acp/runner.rs` (`AcpRunner::execute_turn_stream`) bypassed that shipping path:

1. **Fabricated Thinking Event**: `AcpRunner::execute_turn_stream` unconditionally emitted `AgentTurnOutput::Thinking("Reasoning with model <model_id> for session <session_id>...")` before running the turn (`runner.rs:191-198`).
2. **Hardcoded Canned Closure (`SandAgentRunner::run_with`)**: Instead of calling `SandAgentRunner::run_routed_provider_with_options`, `AcpRunner::execute_turn_stream` invoked the contract-test seam `sand_runner.run_with(&messages, |msgs, _options| ...)` with a closure that synthesized `"Autonomous Agent Response [session=<id>, model=<model>]: Substantive analysis completed for '<prompt>'"` (`runner.rs:208-238`).
3. **Simulated Streaming Loop**: The closure split its hardcoded string on whitespace and slept `2ms` per word (`std::thread::sleep(std::time::Duration::from_millis(2))`) while invoking `on_text_delta`.
4. **Disconnected Tool & Checkpoint Owners**: Default composition used a stub `NoopRoutedToolBridge` and an in-memory `EphemeralCheckpointStore` that discarded checkpoints instead of `ProductionRoutedProviderCheckpointStore`.

---

## 2. Scope & Exclusive Write Boundary

### 2.1 In-Scope for This Slice
- **`docs/specs/fusion-real-agent-provider-20261108.md`**: This specification.
- **`crates/gpui_telegram_agent/src/acp/runner.rs`**:
  - Remove the fabricated `AgentTurnOutput::Thinking` pre-emission, the `sand_runner.run_with(...)` canned response closure, the `2ms` word-sleep loop, and `EphemeralCheckpointStore`.
  - Route `AcpRunner::execute_turn_stream` and `AcpRunner::execute_turn` through `SandAgentRunner::run_routed_provider_with_options` on a dedicated blocking task (`tokio::task::spawn_blocking`) so the synchronous `provider_io_runtime().block_on(...)` inside `mahayana-host-runtime` never blocks or panics an outer Tokio async worker thread.
  - Resolve the active `RoutedProvider` (`Fabushi`, `OpenRouter`, `Codex`, `ClaudeCode`, `Cursor`) from explicit runner configuration, environment/settings (`FABUSHI_INFERENCE_PROVIDER` / `SAND_INFERENCE_PROVIDER` / `settings.json`), or configured provider credentials, and fail explicitly when provider configuration or credentials are missing or invalid.
  - Persist tool-boundary provider checkpoints via `ProductionRoutedProviderCheckpointStore` under the runner's configured `data_dir`.
  - Track active session/turn `RoutedProviderCancellation` tokens in `RoutedProviderTaskRegistry` so in-flight turns can be cancelled deterministically via `AcpRunner::cancel_session`.
  - Emit real `AgentTurnOutput::ToolCall` and `AgentTurnOutput::ToolResult` events from actual routed tool executions, stream real provider deltas via `TextDeltaCallback`, and emit `AgentTurnOutput::FinalResponse` only when the routed provider turn succeeds.
- **Focused Tests in `crates/gpui_telegram_agent`**:
  - `crates/gpui_telegram_agent/tests/fusion_real_agent_provider.rs` (and unit tests in `crates/gpui_telegram_agent/src/acp/runner.rs`).

### 2.2 Strictly Out-of-Scope (Owned by Concurrent / Follow-Up Slices)
- **No edits to `crates/gpui_telegram_server/src/server.rs`**: Another active session (`FUS-S003`) owns `server.rs`. All existing public types and method signatures on `AcpRunner`, `AgentTurnInput`, `AgentTurnOutput`, and `TextDeltaCallback` are preserved so `crates/gpui_telegram_server` compiles without modification.
- **No edits to `crates/gpui_telegram_desktop` or `crates/gpui_telegram_client`**.
- This slice does **not** complete `FUS-F001` end-to-end; remaining server, persistence, cancellation endpoint, and GPUI desktop wiring dependencies are enumerated in Section 5.

---

## 3. Architecture & Runtime Contract

```mermaid
flowchart LR
    Caller["Server / Caller\nAcpRunner::execute_turn_stream(input)"] --> Resolve["Resolve RoutedProvider & Validate Build Input"]
    Resolve --> Registry["Register RoutedProviderCancellation\nin RoutedProviderTaskRegistry"]
    Registry --> SpawnBlocking["tokio::task::spawn_blocking"]
    SpawnBlocking --> SandRunner["SandAgentRunner::run_routed_provider_with_options"]
    SandRunner --> Owner["ProductionTurnAgentOwner::run_routed_provider_with_projected_turn_input"]
    Owner --> Comp["TurnAgentComposition::run"]
    Comp --> Provider["provider_session::run_routed_provider_text_with_lifecycle_reporting_usage_with_max_steps"]
    Provider -->|SSE delta| DeltaCb["TextDeltaCallback (live)"]
    Provider -->|tool_call| Bridge["RoutedToolBridge -> ToolCall + ToolResult"]
    Provider -->|step boundary| Ckpt["ProductionRoutedProviderCheckpointStore\n(data_dir/runner-provider-checkpoints)"]
    Comp -->|Ok(text)| StreamOut["mpsc::Receiver<AgentTurnOutput>\n(ToolCall*, ToolResult*, FinalResponse)"]
    Comp -->|Err(ProviderSessionError)| ExplicitErr["Err(TelegramError) — No Fake Success"]
```

### 3.1 Public API Compatibility Contract
To maintain seamless compilation with `crates/gpui_telegram_server/src/server.rs` (`AppState::with_db`, `agent_runs`, `agent_turn`):
- `AgentTurnInput` retains its exact 5 public fields (`session_id`, `prompt`, `parent_turn_id`, `model_id`, `thinking_budget_tokens`) so struct literal construction in `server.rs:3055` and `server.rs:3134` remains valid.
- `AgentTurnOutput` retains its exact struct variants:
  - `Thinking { token: String, elapsed_ms: u64 }`
  - `ToolCall { tool_call_id: String, tool_name: String, arguments: String }`
  - `ToolResult { tool_call_id: String, output: String, is_error: bool }`
  - `FinalResponse { text: String }`
  - `Error { code: String, message: String }`
- `AcpRunner::new(default_model: impl Into<String>) -> Self`, `with_text_delta_callback`, `with_turn_composition`, `build_static_config`, `build_static_projection`, `execute_turn_stream(&self, input: AgentTurnInput) -> Result<mpsc::Receiver<AgentTurnOutput>>`, and `execute_turn(input: AgentTurnInput) -> Result<Vec<AgentTurnOutput>>` preserve their exact signatures.
- Additional builder/control seams added in `crates/gpui_telegram_agent/src/acp/runner.rs`:
  - `AcpRunner::with_data_dir(self, data_dir: impl Into<PathBuf>) -> Self`
  - `AcpRunner::with_provider(self, provider: RoutedProvider) -> Self`
  - `AcpRunner::with_tool_bridge(self, bridge: Arc<dyn RoutedToolBridge>) -> Self`
  - `AcpRunner::with_request_context(self, request_context: RunnerRequestContextSnapshot) -> Self`
  - `AcpRunner::with_task_registry(self, registry: Arc<RoutedProviderTaskRegistry>) -> Self`
  - `AcpRunner::task_registry(&self) -> &Arc<RoutedProviderTaskRegistry>`
  - `AcpRunner::cancel_session(&self, session_id: &str, reason: impl Into<String>) -> bool`
  - `AcpRunner::run_turn(&self, input: AgentTurnInput) -> Result<Vec<AgentTurnOutput>>`

### 3.2 Error Handling & No-Fake-Success Guarantee
Because `crates/gpui_telegram_server/src/server.rs` (`agent_runs` at lines 3063–3102 and `agent_turn` at lines 3142–3181) currently maps `Ok(rx)` unconditionally to HTTP `200 OK` with `"status": "completed"` / `"status": "executed"`, `AcpRunner::execute_turn_stream(input).await` **must return `Err(TelegramError)`** whenever the routed provider turn fails (unconfigured provider, missing/invalid credentials, upstream HTTP/transport failure, provider refusal, protocol error, or cancellation).
- `ProviderSessionError::Configuration(msg)` -> `TelegramError::Validation(format!("CONFIGURATION: {msg}"))`
- `ProviderSessionError::Authentication(msg)` -> `TelegramError::AuthFailed(format!("AUTHENTICATION: {msg}"))`
- `ProviderSessionError::ProviderRefusal(refusal)` -> `TelegramError::PermissionDenied(...)`
- `ProviderSessionError::Transport(msg)` -> `TelegramError::Network(format!("TRANSPORT: {msg}"))`
- `ProviderSessionError::Cancelled(msg)` -> `TelegramError::Internal(format!("CANCELLED: {msg}"))`
- `ProviderSessionError::Protocol(msg)` / `Tool(msg)` -> `TelegramError::Internal(...)`

This guarantees that `POST /api/v1/agent/runs` and `POST /api/v1/agent/turn` in `server.rs` immediately return HTTP `500 INTERNAL_SERVER_ERROR` (`{"error": "turn_dispatch_error", "message": ...}`) instead of a fabricated `"status": "completed"` response when no provider is configured or when the provider errors.

---

## 4. Positive & Negative Acceptance Oracles

1. **Positive Oracle 1 — Real Routed Provider SSE & Multi-Step Tool Execution**:
   - Given a local loopback Responses SSE server (`127.0.0.1:0`) with a private `0600` Fabushi credential file in `tempfile::TempDir` and a `RoutedToolBridge` exposing a real tool:
   - Step 1 SSE emits a `function_call` item for the registered tool and `response.completed`.
   - `AcpRunner` executes the tool via `RoutedToolBridge::call_tool`, persists a tool-boundary checkpoint via `ProductionRoutedProviderCheckpointStore`, and sends the tool output in Step 2.
   - Step 2 SSE streams `response.output_text.delta` chunks and `response.completed`.
   - `AcpRunner` invokes `TextDeltaCallback` with the real deltas, emits `AgentTurnOutput::ToolCall`, `AgentTurnOutput::ToolResult`, and `AgentTurnOutput::FinalResponse` containing the exact provider text, and **never** emits `"Substantive analysis completed for"` or `"Reasoning with model"`.
2. **Positive Oracle 2 — In-Flight Turn Cancellation via `RoutedProviderTaskRegistry`**:
   - Given an in-flight `AcpRunner` turn connected to a slow/silent local loopback SSE endpoint:
   - Calling `runner.cancel_session(&session_id, "user requested stop")` cancels the registered `RoutedProviderCancellation` token.
   - `await_cancelable_provider_io` aborts the in-flight stream and `AcpRunner` returns an explicit `CANCELLED` error without emitting `AgentTurnOutput::FinalResponse`.
3. **Negative Oracle 1 — Unconfigured / Missing Provider Credentials Fail Explicitly**:
   - Given an isolated `TempDir` and no provider credentials (`FABUSHI_RESPONSES_URL` / `FABUSHI_HOST_ACCESS_CREDENTIAL_FILE` / `OPENROUTER_API_KEY` unset):
   - `AcpRunner::execute_turn_stream` and `AcpRunner::execute_turn` return `Err(TelegramError)` containing the explicit configuration/authentication failure reason (`CONFIGURATION` / `AUTHENTICATION`) and never fabricate a `FinalResponse`.
4. **Negative Oracle 2 — Upstream Provider HTTP / Protocol Failure**:
   - Given a local loopback server that returns HTTP `500` or an SSE `response.failed` / refusal event:
   - `AcpRunner` returns `Err(TelegramError)` with the upstream failure details and never synthesizes a fallback response.

---

## 5. Remaining Follow-Up Dependencies (`FUS-F001` Completion Roadmap)

Because `crates/gpui_telegram_server/src/server.rs` and desktop/client crates are outside this slice's write boundary, the following follow-up slices are required before `FUS-F001` can claim full parity:

1. **Server Handler & Streaming Wiring (`crates/gpui_telegram_server/src/server.rs`)**:
   - Wire `AppState::acp_runner` to share `AppState::mcp_registry`'s `RoutedToolBridge` and a persistent `data_dir` from `ServerConfig`.
   - Replace `POST /api/v1/agent/sessions/{id}/prompt` hardcoded echo (`"[ACP Turn Processed]: ..."`, `server.rs:3013`) with real `AcpRunner` turn execution.
   - Forward `AcpRunner` live deltas (`TextDeltaCallback`) and tool events to WebSocket/SSE subscribers (`S2cAgentStreamDelta` / `AgentTurnOutput`).
   - Add `POST /api/v1/agent/runs/{run_id}/cancel` (or `/api/v1/agent/sessions/{session_id}/cancel`) calling `state.acp_runner.cancel_session(...)`.
2. **Durable Run & Transcript Persistence (`crates/gpui_telegram_server` + `mahayana-host-runtime`)**:
   - Persist agent runs, tool call audit records, and `ProductionAgentStore` / `ProductionAgentStateCheckpointSink` state blobs across server restarts, and wire `resume_from` checkpoint recovery on restart.
3. **GPUI Desktop Surface Wiring (`crates/gpui_telegram_desktop`)**:
   - Connect `AiWorkspaceView`, `CanonicalComposer`, `AgentTraceDrawer`, and `TaskSplitView` to dispatch real HTTP/WS agent turn & cancel requests and render live streaming deltas, tool calls, approvals, and errors.
