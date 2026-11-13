# Atomic work queue

All tasks below are OPEN; static inspection is not runtime acceptance. Extend this queue from the independent source audits without an artificial task-count limit.

| ID | Owned surface | Required behavior | Acceptance oracle / dependencies |
| --- | --- | --- | --- |
| FUS-D001 | `views/chat_sidebar.rs` initialization | No invented server dialogs, online presence, or unread counts on startup. Explicit demo fixtures must not be the production constructor. | New/offline account has no fabricated peers; server dialogs appear only after authenticated response. Coordinate existing dirty changes. |
| FUS-D002 | `views/conversation_stage.rs` cloud initialization | Do not synthesize successful backend/AI connection messages as chat history. | Offline and failed connection do not display transport active or gateway online. Actual persisted messages retain identity and delivery state. |
| FUS-D003 | `views/context_inspector.rs` cloud initialization | Profile, members and online status derive from the selected authoritative peer. | Empty selection has no fabricated Agent profile; switching chats cannot leak prior peer state. |
| FUS-D004 | `views/shell.rs` initialization and selection | Bind sidebar, stage and inspector to the same real selected chat, without fixed IDs. | Empty response, delayed response, authentication failure and account switching preserve coherent selection. Depends on D001–D003; serial integrator owns shell edits. |
| FUS-V001 | acceptance ledger/check design | Distinguish file mapping, code reachability, behavior tests, actual UI evidence and packaged acceptance. | Zero regex boilerplate matches cannot alone mark parity complete. Retain historical evidence without promoting unsupported claims. |
| FUS-R001 | release candidate identity | Record exact tracked/untracked candidate inputs and package provenance before acceptance. | Tested source can be reconstructed; pre-existing uncommitted changes remain preserved and attributed. |

Implementation dispatch requires a focused behavior spec and non-overlapping write boundaries. Review negative cases and real execution before marking any row accepted. Do not remove features merely to satisfy truthfulness: empty/loading/error states are prerequisites while complete runtime integration remains required.

## Dispatch and revalidation, 2026-11-08

D001–D004 are now dispatched to separate Argon conversations (see `runs.json`); S001 is dispatched against `docs/specs/fusion-durable-chat-routes-20261108.md`. None is accepted yet. H001 cleanup scope fix has four passing isolated Python dispatch tests; it does not establish the wider cleanup implementation is safe or complete.

The scope inventory `capability-coverage.json` retains all 52 Telegram and six existing aggregate capability domains from Spec 01. Each is pending a behavior audit and further atomic decomposition, with no artificial maximum task count. The six aggregates must be expanded against prior Fabushi source owners; six labels are not proof of full inheritance.

## Next independently verifiable slices

| ID | Work / current owner | Required acceptance evidence | Dependencies |
| --- | --- | --- | --- |
| FUS-S001 | Durable authenticated chat create/list/get; server and existing SQLite owner | Reopen persistence, member-only access, invalid input, concurrent IDs, atomic creator membership | In progress |
| FUS-S002 | Transactional message commit and nonce conflict handling; existing MessagePipeline/DB | Same nonce returns same result, changed payload/chat rejected, failures leave neither sequence gaps nor orphan message | Inspect existing PTS/message transactions first |
| FUS-S003 | HTTP send and history use persisted pipeline | Authenticated member sends, reads same content after restart; nonmember rejected | S001, S002 |
| FUS-S004 | WebSocket send uses same durable command owner | HTTP/WS retry yields one persisted message and consistent ACK; subscriber receives correct update | S002, S003 |
| FUS-S005 | Persist edit | Author/role authorization, optimistic conflict handling, restart preserves edit | S003 |
| FUS-S006 | Persist deletion/tombstones | Cross-device deletion, tombstones survive restart, no unauthorized delete | S003 |
| FUS-S007 | Persist reaction state | Retry idempotence, per-user reaction replacement and cross-device updates | S003 |
| FUS-S008 | Persist per-member read cursor | No shared unread counters across users, monotonic read cursor and restart | S003 |
| FUS-S009 | Real difference recovery | Gaps/reconnect recover durable updates in order without duplication | S004–S008 |
| FUS-D005 | Single outgoing transport / durable retry identity | One user send produces one authoritative message under fallback/reconnect | S003, S004 |
| FUS-D006 | Full incoming message projection | Preserve sender, reply, reaction and media metadata for HTTP and WS | S003 |
| FUS-D007 | Reply and forwarding commands | Reply target sent and restored; forwards obey permissions and persist | S003, D006 |
| FUS-M001 | Upload part storage | Bytes committed and resumable, ownership/size/hash failures rejected | Existing CAS/upload service audit |
| FUS-M002 | Media Range stream | Uploaded bytes round-trip, valid ranges exact, invalid range 416, access enforced | M001 |
| FUS-M003 | File picker and upload composer | Actual chosen file delivered and restored; cancellation preserves draft | M001, S003 |
| FUS-M004 | Image/album decoding | Actual media pixels in running GPUI, dimensions and failure states correct | M002 |
| FUS-M005 | Voice capture | Device audio captured and sent with actual duration/waveform | M001, platform audio owner |
| FUS-M006 | Voice playback | Audible playback, real position/pause/seek, resource release | M002 |
| FUS-I001 | Peer detail hydration | Authoritative phone/username/bio, selection fencing and no stale disclosure | S001, D003 |
| FUS-I002 | Membership mutations | Real membership service operations, permissions and server confirmation | S001 |
| FUS-I003 | Shared media queries | Empty state has no synthetic grid; real media scoped to selected chat | M001, I001 |
| FUS-A001 | Auth QR payload rendering | Displayed QR decodes to actual pending challenge, expiry/restart handled | Auth contract audit |
| FUS-A002 | Auth callback verification | Actual provider verification; replay/tamper rejection and durable session | Provider configuration without secret disclosure |
| FUS-F001 | Real agent execution adapter | Actual provider/tool result, cancellation and checkpoint restart; no canned response | Inherited owner audit |
| FUS-F002 | MCP routed execution | Result comes from registered executor with approval and errors propagated | Existing RoutedMcpProtocolBridge |
| FUS-F003 | Global tasks projection | Real running/waiting/terminal tasks visible across sessions, persisted | F001 |
| FUS-F004 | Scheduled automation execution | Due execution, restart, cancellation and duplicate fencing | Existing automation owner |
| FUS-F005 | Remote device inventory | Authorized live devices and accurate connection status | Inherited network enrollment owner |
| FUS-F006 | Remote frame/input transport | Actual frame display and input effect, lease expiry and user takeover | F005 |
| FUS-F007 | Full duplex talk integration | Actual mic/STT/model/TTS path; barge-in and reconnect verified | Inherited talk owner |
| FUS-U001 | Mount real modal bodies | Each reached dialog has actual action/validation behavior | Source-specific dialog audit |
| FUS-U002 | Settings persistence wiring | Changed settings affect runtime and survive restart | Existing preferences owner |
| FUS-U003 | Sidebar resize and folders | Real pointer resize, min/max bounds, server-backed folders | D001, S001 |
| FUS-V002 | Actual GPUI UI evidence harness | Screenshots/interactions originate in tested GPUI binary, not substitute renderer | Candidate build |
| FUS-R002 | Durable storage startup failure | Inaccessible production DB fails visibly rather than silently switching to memory | AppState startup owner |
| FUS-H001 | Cleanup CLI scope | Four dispatch tests pass; target-only never invokes fixture cleanup | Implemented, focused tests passed |

Audit correction: the release audit proposed adding chat tables and described fixture-valued chat lists. Current source already has `CREATE_CHATS_TABLE` and `CREATE_CHAT_MEMBERS_TABLE`; current list returns an empty array. S001 reuses existing tables and fixes unwired handlers. All other audit suggestions remain subject to direct source revalidation.

Telegram reference revision observed: `f985723b552b30fe1c17d377d57f44e61a15a8c4`. Source-file mapping and future behavior tests must identify this version (or record an intentional version change).

S002 dispatched to Argon conversation `d68be6ed-958d-4b02-86b5-2fe4cd3d6837`, implementing the focused atomic-message-commit spec. Exclusive write ownership is pipeline.rs, pts_engine.rs and its new test file; S001 retains db/mod.rs and chat handlers. Both remain unverified. Builds wait for these server writers to finish.

## Acceptance-ledger audit decomposition, 2026-11-08

Independent FUS-V001 source audit found that the 13-module roadmap marks 100% ACCEPTED based largely on file/symbol mapping and a narrow boilerplate regex veto. Those labels are historical claims, not acceptance for this fusion campaign. The six inherited Fabushi aggregates and 52 Telegram capability entries need finer decomposition; referenced inherited ledger shards are absent. Each task below remains open until its own evidence is reviewed.

| ID | Exclusive work boundary | Required evidence |
| --- | --- | --- |
| FUS-V001a | Capability evidence stages and roadmap status only | Preserve prior claims as history; separate mapped files, reachable entry points, behavior checks, actual GPUI evidence and package acceptance; no unsupported aggregate 100% status. Argon dispatched. |
| FUS-V001b | `scripts/verify_ledger_closure.py` and focused validator tests | Fail closed when a mapped symbol is unreachable or the purported implementation is boilerplate; require direct behavior evidence before accepted status. |
| FUS-V001c | Telegram capability inventory and relevant ledger shards | Add explicit upstream versioned domains for wallet, payments/Stars/gifts, Passport, Instant View/editor, statistics, Business/Premium and photo/video editing; correct misassigned files without losing entries. |
| FUS-V001d | Inherited Fabushi capability inventory and shards | Expand six aggregates against real Rust owners and GPUI entry points; materialize missing referenced inherited shards and label excluded/reference components honestly. |
| FUS-V001e | C++ symbol extraction and non-code ledger entries | Reject primitive-type/include pseudo symbols and distinguish source documentation from product responsibilities. |
| FUS-V001f | GPUI and end-to-end evidence harness | Prove tested GPUI binary interactions and durable backend roundtrips, then record binary/source identity. |
