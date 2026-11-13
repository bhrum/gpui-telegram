# Full Telegram / Fabushi fusion reacceptance

Date: 2026-11-08. Status: active, NOT release accepted.

## Request and baseline

User requires complete source-informed Telegram Desktop Rust/GPUI parity, preservation and integration of all prior Fabushi functionality, real data and a production releasable product. Implementation is delegated through independent Antigravity CLI Argon sessions; coordinator owns review and dispatch.

Repository: `/Users/bhrum/teamwork_projects/gpui_telegram`.
Initial HEAD: `68d422c645dc4232b2b03a73325c43b9596fed4b`.
Working tree contains extensive pre-existing tracked and untracked changes; HEAD alone does not identify the current candidate. Do not reset, overwrite, commit or release unrelated work.

Source references: existing `docs/specs/01-telegram-1to1-feature-inventory.md`, `02-gpui-uiux-component-hierarchy.md`, `03-rust-server-architecture-contracts.md`, `04-existing-capabilities-routing-map.md`, `05-test-and-verification-plan.md`; Telegram `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles`; prior Fabushi `/Users/bhrum/Documents/ChatGPT/fabushi-unified-design-20261105`.

Existing architecture specifies a self-hosted Rust service. Telegram interaction parity and Telegram public-network interoperability are distinct claims; no public-network interoperability is currently accepted.

## Acceptance contract

- FUS-01: Every upstream capability maps to reachable Rust behavior with source/version references, not merely a named file or symbol.
- FUS-02: Every previous Fabushi capability maps to its runtime owner and reachable GPUI entry point. Copying inherited crates is not integration.
- FUS-03: User-visible messages, presence, task status, devices, payment state and connection status come from authoritative state. Empty/loading/error/unsupported states remain truthful.
- FUS-04: Each atomic task records dependencies, exclusive write boundaries, upstream oracle, positive and negative checks, candidate identity and evidence; implementation delivery remains pending independent review.
- FUS-05: UI acceptance covers actual running GPUI captures and interactions against Telegram reference behavior; a separate AppKit rendering or string-pattern check is not GPUI runtime proof.
- FUS-06: Release acceptance requires real account/service flows, restart/reconnect behavior, packaged application checks, signing/update/rollback evidence and retained Fabushi regressions. Missing evidence remains unverified.
- FUS-07: Single workspace target directory, lean dev/test profiles and immediate prescribed garbage cleanup after local builds/tests. Temporary data must clean up automatically. Parallel workers must not race builds or cleanup.

## Initial dispatch

Three independent `gemini-4-argon-high` CLI sessions perform read-only first-pass audits: UI/data truthfulness; inherited Fabushi reachability; networking/release. No product edits or builds authorized in these initial audit prompts.

CLI exit status SUCCESS with an empty response or denied command is NOT audit completion. Release audit conversation `5ea5fc6b-bad5-40dd-8515-57d182e514ad` and inheritance audit `651c5622-85b1-45a1-9417-5bc12ba28c70` initially encountered headless command denial; continued those same conversations with session-only read command allowances. No global permission configuration changed.

## Independently observed initial defects

`ChatSidebar::configure_for_cloud_workspace` creates fixed IDs 4, 3, 5 and marks them online; server URL text alone produces Connected/Live labels. `AppShell::new_with_client` calls this initializer. This is reachable fabricated state, even though later server refreshes may replace some rows.

`MODULE_BY_MODULE_ROADMAP.md` asserts total parity from zero matching boilerplate patterns. Such evidence cannot establish functional equivalence. Existing ACCEPTED labels are historical assertions, not acceptance of this campaign.

Next: collect concrete audit findings, create non-overlapping implementation slices with behavior contracts, then review/test each slice and integrate serially. This document does not mark any migration capability complete.

## Recovery after interruption

D001 and D003 returned terminal SUCCESS with owned source/test changes; independent compilation remains pending. D002 wrote source/tests but its final event is ERROR (provider stream unexpected EOF), so code is retained for review without treating worker completion as approval. D004/S001 original exec handles were missing and no `agy` process remained on recovery; resumed the same conversations into separate `*-resume1.events.jsonl` logs (exec handles 2348 and 37889). Original evidence was preserved.

Concurrent shell changes since the captured preimage include Alipay web authorization and timer wiring outside D004. Original D004 events contain no edit/write calls at recovery. Preserve those external changes; an aggregate preimage diff is not exclusively this campaign's patch. Review ownership from event edits and fresh source before integration.

## Initial compilation evidence

Run `.agents/fusion-20261108/test-initial-models`: cargo exit 101 because the inspector regression test called `as_deref` on `Option<(String,bool)>`. Cleanup exit 0, 78,442,112 bytes removed. Coordinator changed the assertion to compare `Option<(&str,bool)>` via `as_ref().map(...)` with a matching expected tuple. Recovery1 compiled the incomplete two-part assertion edit and failed type matching; cleanup passed. Recovery2 caught D004 mid-edit (new generation fields not yet added to its constructor), cargo 101, cleanup 0. These are not passing tests. Suspend further compilation until active writers finish, then compile the frozen integrated candidate; do not repeatedly rebuild a moving tree. Logs retained under the three run directories.

S001 resume delivered database/chat route changes and eight route-level tests (auth, empty state, create/list/get, isolation, invalid titles, concurrent IDs, reopen, forced membership failure). Final worker status is ERROR with interrupted stream; no passing result inferred. Direct coordinator inspection confirms transaction ownership and membership-scoped queries, but execution remains pending alongside S002. Remaining review concerns include shared chat unread-count fallback and compatibility of existing auth fixtures; no full chat/messaging acceptance claimed.

## 2026-11-08 21:24 CST acceptance checkpoint

The current dirty candidate passed two focused independent suites: `fusion_atomic_message_commit` (6/6) and `fusion_durable_chat_routes` (8/8). Both `cargo test` commands exited 0 and each was followed immediately by `python3 scripts/clean_build_garbage.py` with exit 0. Run logs and result JSON are in `/Users/bhrum/Documents/Codex/2026-11-08/new-chat/work/fusion-acceptance/` under run IDs `20261108T212341` and `20261108T212435`. These are focused behavior checks only; broad regressions, running GPUI behavior, packaging and release remain pending. The candidate is uncommitted and has 102 changed/untracked paths, so HEAD `68d422c` alone does not identify the tested source.

An independent Argon read-only audit found a release-blocking storage startup path: `AppState::open_database` in `crates/gpui_telegram_server/src/server.rs` catches a file-backed SQLite open error and substitutes `:memory:`. `main` can then serve health/status from an empty ephemeral store. FUS-R002 was dispatched in the same Argon conversation to write a focused spec and implement fail-visible startup within exclusive constructor/config/pool/migration/main boundaries. Its result and tests are pending; no release acceptance follows from dispatch.

## 2026-11-08 21:34 CST storage startup verification

Argon delivered the FUS-R002 spec, code and seven focused tests. Independent `cargo test -p gpui_telegram_server --test fusion_durable_storage_startup` exited 0 (7/7), including corrupt database process startup with a nonzero exit before bind. After that candidate change, the prior S002 and S001 suites were rerun together (6/6 and 8/8), and four server config unit tests passed. Every command was followed immediately by the mandated cleanup with exit 0. Exact logs and result JSON are under `/Users/bhrum/Documents/Codex/2026-11-08/new-chat/work/fusion-acceptance/` with run IDs `20261108T213349` and `20261108T213419`. This verifies a focused local source candidate; the dirty candidate still needs a reconstructable identity and full release checks. Media CAS and search startup paths are outside R002 and remain open.

Independent Argon FUS-V001 audit confirmed that the historical roadmap labels all 13 modules `ACCEPTED (100.0%)` while its verifier can accept zero matches against four boilerplate string patterns. It also found absent referenced inherited Fabushi ledger shards. These claims need behavior and runtime evidence, so FUS-V001a was dispatched to distinguish evidence stages; FUS-V001b through V001f are queued in `tasks.md`. No historical parity label is accepted for this campaign.

FUS-S003 initial Argon CLI turn returned `SUCCESS` with an empty response after permission was denied for an upstream `api_sending.cpp` read. No S003 code/spec edit was present, so this is not delivery. The same conversation was resumed with explicit instructions to respect the denied read and implement from permitted repository source/specs, recording direct upstream source parity as pending. Do not reattempt the denied resource through another route.

FUS-S003 subsequently delivered code and eight route-level tests, but independent compilation run `20261108T215352` exited 101 with 20 errors from assumed protocol variants/fields and a missing WebSocket event type. Cleanup exited 0. The same Argon conversation was dispatched a correction against the actual proto types. FUS-S003 remains unverified; the failing log is retained under the `work/fusion-acceptance/` run path above. No claimed successful worker delivery overrides this compiler result.

## 2026-11-08 22:02 CST HTTP messaging checkpoint

The corrected S003 candidate passed all four focused server suites in one independent run: atomic message commit 6/6, durable chats 8/8, durable storage startup 7/7, and HTTP durable messaging 8/8. Command and cleanup both exited 0; logs/result are under run ID `20261108T220225` in the same `work/fusion-acceptance/` directory. The coordinator corrected three S003 fixture assumptions (actual `ServerConfig.database_url`, real `/api/v1/auth/register` contract, and chat GET projection of its stored `pts`) plus one Rust borrow syntax error before this run. Passing route tests establish this local source behavior, not real installed GPUI parity, public Telegram connectivity, full Fabushi inheritance or release readiness. Direct upstream `api_sending.cpp` read was denied to the worker; source-specific parity remains pending.

FUS-F001 independent audit traced the active GPUI agent path to `AcpRunner::execute_turn_stream`, which currently invokes a test seam with canned `"Substantive analysis completed for"` text while inherited real provider, tool, cancellation and checkpoint owners are disconnected. An Argon runner-only implementation slice was dispatched; server and desktop wiring remain separate tasks. FUS-V001a had an interrupted worker stream after its spec/roadmap edits and before completing the 58-entry evidence schema, so the same conversation was resumed. Neither slice is accepted.
