# FUS-S001: durable authenticated chat create/list/get

Status: implementation authorized; no acceptance claimed. Parent scope remains full Telegram/Fabushi migration.

## Current and target behavior

Current `server.rs` chat creation invents a timestamp ID without a write, list always returns empty, and get fabricates a title for arbitrary IDs. Existing SQLite `chats` and `chat_members` tables and `ServerDatabase::{create_chat,get_chat,add_chat_member,get_chat_member}` already exist. Extend those owners, do not add a second store or schema.

This atomic slice makes creation durable and list/get membership-scoped. Other chat features (participants, roles, forums, history, synchronization, notifications, desktop creation) remain separate required follow-ups; do not imply their completion.

## Ownership

Worker may edit ONLY `crates/gpui_telegram_server/src/db/mod.rs`, `crates/gpui_telegram_server/src/server.rs` (chat create/list/get handlers and their necessary imports), and new `crates/gpui_telegram_server/tests/fusion_durable_chat_routes.rs`. Preserve existing dirty edits. No changes to auth handlers, other fake routes, schemas, Cargo, shell, ledgers or unrelated tests. Coordinator will review and serialize builds.

## Contract

- AuthBearer remains required. `POST /api/v1/chats` validates a trimmed nonempty title, maximum 255 Unicode scalar characters; malformed/missing/oversized titles return 400. Do not silently invent titles. Do not claim optional unsupported fields took effect.
- Generate positive SQLite integer IDs using the existing table's integer allocation under transaction (no timestamp uniqueness assumption). Insert chat and creator membership atomically; transaction failure produces no success and no orphan chat.
- Return 201 with `chat_id`, normalized `title`, and authenticated `creator_id` only after commit. Use existing type conventions, timestamps and sensible existing group defaults; do not allow client-specified creator IDs to impersonate another user.
- `GET /api/v1/chats` lists only the requesting user's memberships, in deterministic order, with `chat_id`, `title`, `unread_count`, and optional known last message/avatar metadata compatible with client `ChatSummary`. Do not fabricate previews, presence or delivery state. Empty account returns empty array and total zero.
- `GET /api/v1/chats/{id}` returns persisted chat data only to members. Missing/nonmember chat returns 404, without disclosing its existence. Invalid/nonpositive IDs return 400 or 404. Database errors return 500 with generic client text; do not leak SQL/internal paths.
- Existing helpers and domain records retain compatibility. Avoid holding a DB mutex while calling a helper that locks it again. Query membership and data under one connection/transaction where appropriate. Reuse prepared statements and existing error conversions.

## Verification

Add route-level tests using the real Axum router and authenticated fixtures: create then list/get, disjoint accounts, guessed IDs denied, empty account, invalid title, concurrent creation uniqueness, persistence across database reopen, and no orphan chat on failed membership transaction. Use in-memory databases except explicit restart test; that test must use an owned RAII temp directory (can use a local Drop guard with existing dependencies), cleaned on assertion failure. Inspect existing tests for safe local authentication setup; do not access live services or credentials.

Worker must not run builds/tests. Coordinator runs tests serially against sole workspace target and immediately performs build garbage cleanup. Review source changes separately from test results. No push/release/deploy authorized in this slice. Rollback is the incremental owned patch, never reset of the dirty tree.
