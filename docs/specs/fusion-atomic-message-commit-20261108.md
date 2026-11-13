# FUS-S002: atomic durable message commit

Status: authorized, pending implementation and acceptance. Full Telegram/Fabushi scope remains unchanged.

## Evidence and goal

`MessagePipeline::process_incoming_message` currently commits PTS journal before saving the message and dedup record. Failed message persistence can therefore leave a published update without a message. Concurrent retries can allocate separate IDs; a repeated nonce with different content/chat returns an unrelated prior result. These violate real messaging guarantees regardless of UI appearance.

## Exclusive worker boundary

Modify only `crates/gpui_telegram_server/src/messaging/pipeline.rs`, `crates/gpui_telegram_server/src/sync/pts_engine.rs`, and new `crates/gpui_telegram_server/tests/fusion_atomic_message_commit.rs`. No db/mod.rs, schema, server.rs, manifests or other tests (another worker owns chat routes). Reuse ServerDatabase.pool transaction and current SQL tables. A transaction-aware helper in PtsEngine is allowed; preserve existing public advance_pts behavior and signatures. No parallel store/new schema or broad refactor.

## Required invariants

For new messages, nonce lookup, server ID allocation, message record, sequence advance, update journal, durable nonce record and chat last-message metadata must commit atomically in the same SQLite transaction. Never acquire DB mutex recursively. Failed constraint or serialization rolls back everything and publishes no memory cache entry. Existing constraints/foreign keys stay enabled.

For `(sender,nonce)`, an exact retry yields the original ID/PTS and pts_count zero, including after reopen and from separately constructed pipeline instances sharing the DB. Compare original client semantic intent (chat, content, reply, topic, forward where applicable); do not treat server-assigned IDs, server time, reactions or delivery flags as changed client intent. Different chat/content/reply with same nonce is an explicit conflict error, never silent success. Memory dedup cannot bypass conflict checking. Concurrent identical nonce produces one row and one journal event; concurrent different nonces produce distinct IDs and contiguous sequence. Check integer overflow/conversion boundaries rather than wrapping user nonce into negative SQLite values silently. Preserve existing legitimate fixtures/contracts where they do not contradict these invariants.

Do not claim this alone wires HTTP/WS or authorizes membership: transport authentication/membership integration is a separate required slice. Edit/delete transaction fixes remain follow-ups unless necessary to preserve helper behavior.

## Verification

Owned regression tests exercise real SQLite: concurrent same/different nonces, conflicting retries, restart dedup, and a forced insert failure using an owned test DB trigger proving message/journal/PTS/dedup/chat metadata roll back together. Test orphan-free failure followed by successful retry. Use in-memory DB except reopen test with an owned RAII temp directory; no live services or secrets. No fake persistence or tests that only inspect source strings.

Do not run builds/tests or cleanup while other workers edit. Coordinator serializes tests against frozen candidate, then mandated target cleanup. Return actual changed files, invariant reasoning and unrun commands; do not mark accepted. Rollback uses owned incremental patch without resetting existing work.
