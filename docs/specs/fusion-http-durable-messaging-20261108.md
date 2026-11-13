# FUS-S003: Durable HTTP Messaging Routes (`POST /api/v1/messages/send` & `GET /api/v1/messages/history`)

- **Date**: 2026-11-08
- **Slice ID**: `FUS-S003`
- **Status**: In Progress (Implementation & Route Integration Tests)
- **Depends On**: `FUS-S001` (Durable Chat Routes), `FUS-S002` (Atomic Message/PTS/Dedup Commit in `MessagePipeline`), `FUS-R002` (Durable Storage Startup & Auth Persistence)
- **Upstream Reference**: Telegram Desktop (`f985723b552b30fe1c17d377d57f44e61a15a8c4`)
  - *Conceptual Upstream Surfaces*: `Telegram/SourceFiles/api/api_sending.cpp`, `Telegram/SourceFiles/data/data_histories.cpp`, `Telegram/SourceFiles/data/data_session.cpp`, `Telegram/SourceFiles/history/history.cpp`
  - *Parity Evidence Status*: Direct file read of external path `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/api/api_sending.cpp` was denied by the workspace tool permission boundary and was not retried per governance rules. Direct upstream C++ parity verification for this slice therefore remains pending coordinator review; this specification and implementation derive all invariants from permitted repository sources (`FUS-S001`, `FUS-S002`, `MessagePipeline`, `PtsEngine`, `ServerDatabase`, and `gpui_telegram_proto`).

---

## 1. Problem Statement & Current Fabricated Behavior

In `crates/gpui_telegram_server/src/server.rs` (`lines 1483-1510`):

1. `POST /api/v1/messages/send` (`messages_send`):
   - Allocates a process-local volatile counter via `state.gateway_hub.next_pts()` and fabricates `"message_id": Utc::now().timestamp_millis()`.
   - Does not verify whether the target `chat_id` exists or whether the authenticated caller (`auth.user_id`) is an active member of the chat.
   - Bypasses `MessagePipeline::process_incoming_message`, writing zero rows to `messages`, `pts_sequences`, `pts_updates`, `chats`, `forum_topics`, or `message_dedup`.
   - Ignores `client_nonce`, `reply_to`, `topic_id`, `forward_info`, and content validation bounds.
2. `GET /api/v1/messages/history` (`messages_history`):
   - Ignores `_auth` and `_params` completely.
   - Always returns `{"messages": [], "has_more": false, "current_pts": state.gateway_hub.current_pts()}` regardless of SQLite state or chat membership.

---

## 2. Scope & Exclusive File Ownership

Per `docs/verification/fusion-20261108/tasks.md` (`FUS-S003`), edits in this slice are strictly restricted to:

1. `docs/specs/fusion-http-durable-messaging-20261108.md` (this specification)
2. `crates/gpui_telegram_server/src/db/mod.rs` (only new read/history helper on `ServerDatabase`)
3. `crates/gpui_telegram_server/src/server.rs` (only `messages_send`, `messages_history`, and narrowly necessary request/response/pipeline helpers; preserving `FUS-S001` chat route changes and `FUS-R002` startup changes)
4. `crates/gpui_telegram_server/tests/fusion_http_durable_messaging.rs` (focused route-level integration tests)

Out of scope (untouched): `pipeline.rs`, `pts_engine.rs`, other HTTP handlers (`messages_edit`, `messages_delete`, `messages_reaction`, `sync_get_state`, `sync_get_difference`), WebSocket gateway fanout (`FUS-S004`), client networking (`FUS-S005`), and desktop UI views (`FUS-S006`).

---

## 3. Target Architecture & Contract

### 3.1 Single Commit Owner (`MessagePipeline`)

- `MessagePipeline::process_incoming_message` (`crates/gpui_telegram_server/src/messaging/pipeline.rs`) is the **sole commit owner** for message ID allocation, per-chat PTS allocation, `pts_updates` journal append, `messages` insertion, `chats` summary metadata update (`pts`, `last_message_id`, `last_message_date_ms`, `updated_at_ms`), `forum_topics` summary update, and `(sender_id, client_nonce)` deduplication (`message_dedup` + Tier-1 LRU cache).
- `messages_send` MUST NOT call `state.gateway_hub.next_pts()`, `state.db.save_message()`, or `Utc::now().timestamp_millis()` to fabricate message IDs or PTS values.
- To preserve `AppState` struct layout and constructors from `FUS-R002`, `server.rs` maintains a narrow helper `pipeline_for_state(state: &AppState) -> crate::messaging::MessagePipeline` keyed by the `Arc<ServerDatabase>` pointer (`Weak<ServerDatabase>` verification). This guarantees that all HTTP requests against the same `AppState`/`ServerDatabase` instance share the same `MessagePipeline` (preserving Tier-1 `MessageNonceDedup` and `PtsEngine` striped locks), while a server restart that opens a new `ServerDatabase` instance gets a fresh `MessagePipeline` backed by durable SQLite state.

### 3.2 Authentication, Chat Existence, and Membership Authorization

Both `POST /api/v1/messages/send` and `GET /api/v1/messages/history` require a valid `AuthBearer` session token (`401 Unauthorized` if missing, malformed, expired, or revoked).

Before committing a message or reading history for `chat_id`:
1. Verify `chat_id` exists in `chats` (`state.db.get_chat(ChatId(chat_id))`):
   - If `None`, return `404 Not Found` (`{"error": "chat_not_found", ...}`).
2. Verify the authenticated user (`auth.user_id`) has an active membership record in `chat_members` (`state.db.get_chat_member(ChatId(chat_id), auth.user_id)`):
   - If `None`, or if `member.role` is `"left"` or `"kicked"`, return `403 Forbidden` (`{"error": "forbidden", ...}`).
   - For `POST /api/v1/messages/send`, if `(member.banned_rights & 1) != 0` (send messages banned), also return `403 Forbidden`.
3. Authoritative sender identity:
   - `sender_id` is ALWAYS derived from `auth.user_id`. Any client-supplied `sender_id` field in the JSON request body is ignored for identity attribution so a caller can never impersonate another user.

### 3.3 `POST /api/v1/messages/send` Contract

#### Request Validation (`400 Bad Request` on malformed input)
- Body must be a valid JSON object (`Value::Object`). Empty body, malformed JSON, or non-object JSON returns `400 Bad Request`.
- `chat_id` (required): JSON integer in `1..=i64::MAX`. Missing, null, boolean, string, float, `<= 0`, or `> i64::MAX` returns `400 Bad Request`.
- Content (required):
  - Accepts `"text": "<string>"` (with optional `"entities": [...]`) and/or structured `"content": MessageContent`.
  - If both `"text"` and `"content"` (`MessageContent::Text`) are provided and their text strings differ, returns `400 Bad Request`.
  - For text messages: `text.trim().is_empty()` returns `400 Bad Request` (`empty_message`); `text.chars().count() > 4096` returns `400 Bad Request` (`message_too_long`). Entity UTF-16 ranges (`offset + length <= text.encode_utf16().count()` and `length > 0`) are validated; out-of-bounds entities return `400 Bad Request`.
  - Non-empty text preserves exact characters (no silent trimming of valid message content).
- `client_nonce` (optional):
  - When present and non-null, must be a positive integer in `1..=(i64::MAX as u64)`. Zero, negative, string, float, or `> i64::MAX as u64` returns `400 Bad Request`.
- `reply_to` / `reply_to_id` (optional):
  - When present and non-null, accepts either a positive integer `1..MsgId::SERVER_MAX` (wrapped in `ReplyHeader::new(MsgId(id))`) or a valid `ReplyHeader` object with `reply_to_msg_id` in `1..MsgId::SERVER_MAX`. If both `reply_to` and `reply_to_id` are present with conflicting IDs, or if malformed/out-of-range, returns `400 Bad Request`.
- `topic_id` (optional):
  - When present and non-null, must be a positive integer in `1..=i64::MAX`. If target chat is a forum and `topic_id != 1`, the topic must exist and not be closed (enforced by `MessagePipeline` / pre-check).
- `forward_info` (optional):
  - When present and non-null, must deserialize as `ForwardInfo` with `date > 0`.

#### Commit & Idempotency / Conflict Semantics
- Calls `pipeline.process_incoming_message(item)`:
  - **Fresh commit (`pts_count == 1`)**: returns `200 OK` with committed `message_id`, `pts`, `pts_count: 1`, `duplicate: false`, `sender_id: auth.user_id.raw()`, `chat_id`, `date` / `date_ms`, `text`, `content`, `reply_to`, `topic_id`, `client_nonce`, and `"status": "sent"`.
  - **Idempotent retry (`pts_count == 0`, same `(sender_id, client_nonce)` and identical semantic intent)**: returns `200 OK` with the original committed `message_id`, `pts`, `pts_count: 0`, `duplicate: true`, original `date` / `date_ms`, and `"status": "sent"`, without allocating a new message ID or advancing PTS.
  - **Conflicting nonce reuse (same `(sender_id, client_nonce)` with different `chat_id`, `topic_id`, `content`, `reply_to`, or `forward_info`)**: `MessagePipeline` returns `TelegramError::Validation("Conflicting ...")`, which `messages_send` maps to `409 Conflict` (`{"error": "nonce_conflict", ...}`).
  - **Storage failure**: logs internally and returns `500 Internal Server Error` with a sanitized JSON error body (`{"error": "database_error", "message": "Internal database error"}`), never leaking SQL or trigger details.

### 3.4 `GET /api/v1/messages/history` Contract

#### Query Parameter Validation (`400 Bad Request` on malformed input)
- `chat_id` (required): positive integer `1..=i64::MAX`.
- `limit` (optional): default `50`, valid range `1..=100`. `0`, `> 100`, negative, or non-integer returns `400 Bad Request`.
- `before_id` / `offset_id` (optional): positive integer `1..MsgId::SERVER_MAX`. If both are supplied and differ, returns `400 Bad Request`. Filters `message_id < before_id`.
- `after_id` (optional): non-negative integer `0..MsgId::SERVER_MAX`. Filters `message_id > after_id`. If both `before_id` and `after_id` are provided and `after_id >= before_id`, returns `400 Bad Request`.
- `topic_id` (optional): positive integer `1..=i64::MAX`. Filters `topic_id = ?`.

#### Read Query & Ordering (`ServerDatabase::list_chat_messages`)
- Queries non-deleted messages (`deleted_at_ms IS NULL`) in `messages` for `chat_id` matching optional `before_id`, `after_id`, and `topic_id` filters, plus authoritative per-chat `current_pts` from `pts_sequences`.
- Window selection:
  - Default / backward pagination (`before_id` present or no cursor): selects newest matching messages (`ORDER BY message_id DESC LIMIT limit + 1`), determines `has_more = rows.len() > limit`, truncates to `limit`, and reverses to chronological ascending order (`message_id ASC`).
  - Forward pagination (`after_id` present without `before_id`): selects oldest matching messages after cursor (`ORDER BY message_id ASC LIMIT limit + 1`), determines `has_more = rows.len() > limit`, and truncates to `limit` in ascending order (`message_id ASC`).
- Each returned message object includes both `HttpClient` (`HistoricalMessage`) fields and rich `MessageItem` fields:
  - `message_id`, `id`, `chat_id`, `sender_id`, `text`, `content`, `date` (ms), `date_ms`, `pts`, `reply_to`, `reply_to_header`, `forward_info`, `topic_id`, `client_nonce`, `is_outgoing` (`record.sender_id == auth.user_id`), `is_edited`, `edit_date_ms`, `reactions`.
- Top-level response (`200 OK`):
  ```json
  {
    "chat_id": 1,
    "messages": [...],
    "has_more": false,
    "current_pts": 2
  }
  ```

---

## 4. Verification Plan (`crates/gpui_telegram_server/tests/fusion_http_durable_messaging.rs`)

1. `test_auth_bearer_required_for_messaging_routes`: missing/invalid token rejected with `401 Unauthorized` on both `/api/v1/messages/send` and `/api/v1/messages/history`.
2. `test_member_send_and_history_roundtrip_with_chat_metadata_update`: creator and added member send messages; verify monotonic `message_id` and `pts`, exact history payloads, per-caller `is_outgoing`, and updated `last_message_id` / `last_message_date_ms` on `/api/v1/chats` and `/api/v1/chats/{id}`.
3. `test_nonmember_and_nonexistent_chat_rejected_without_writes`: non-member gets `403 Forbidden`, nonexistent chat gets `404 Not Found`, kicked/left/banned member gets `403 Forbidden`, with zero database writes.
4. `test_client_nonce_idempotent_retry_and_conflicting_payload_rejection`: identical retry returns `200 OK` with same `message_id`/`pts` and `pts_count == 0`; conflicting payload with same `(sender_id, client_nonce)` returns `409 Conflict`; spoofed `sender_id` in body is ignored in favor of `auth.user_id`.
5. `test_malformed_requests_and_pagination_bounds_rejected`: malformed JSON, missing/invalid `chat_id`, empty/whitespace/oversized `text`, out-of-bounds entity, invalid `client_nonce`, invalid `reply_to`, and invalid `limit`/`before_id`/`after_id`/`topic_id` return `400 Bad Request`.
6. `test_history_pagination_ordering_and_topic_filtering`: verify `limit` + `before_id` backward pagination, `after_id` forward pagination, `topic_id` filtering, ascending chronological page order, accurate `has_more`, and authoritative `current_pts`.
7. `test_restart_durability_for_history_pts_and_nonce_dedup`: across a simulated server restart on a file-backed SQLite database in `std::env::temp_dir()`, verify persisted history, `current_pts`, Tier-2 SQLite `client_nonce` idempotency (`200 OK`, `pts_count == 0`), Tier-2 nonce conflict (`409 Conflict`), and continued monotonic `message_id`/`pts` allocation.
8. `test_forced_db_failure_returns_500_without_leaking_sql_or_orphan_state`: abort `INSERT INTO messages` via SQLite trigger, verify `500 Internal Server Error` with sanitized error body and complete transaction rollback across `pts_sequences`, `pts_updates`, `messages`, and `message_dedup`.
