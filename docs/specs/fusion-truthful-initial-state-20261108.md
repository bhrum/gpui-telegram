# FUS-D001–D004: truthful initial conversation state

Status: implementation authorized, acceptance pending. Parent: `docs/verification/fusion-20261108/coordination.md`. This slice is a prerequisite to full Telegram/Fabushi parity, not a replacement for it.

## Problem and target

Production constructors and cloud configuration invent peers, messages, members and connection success. `AppShell::refresh_chats` merges remote rows with invented rows and ignores an authoritative empty response. A configured URL is not connectivity or identity evidence.

The initial authenticated workspace must show only authoritative conversations and messages. Before data arrives it has no selected chat; it must support a legitimate empty account, errors and later server results. All existing real networking, auth, settings and inherited functionality remains in scope and must be preserved.

## Boundaries and interfaces

- D001 exclusively owns `crates/gpui_telegram_desktop/src/views/chat_sidebar.rs` and new `crates/gpui_telegram_desktop/tests/fusion_sidebar_truth_tests.rs`.
- D002 exclusively owns `crates/gpui_telegram_desktop/src/views/conversation_stage.rs` and new `crates/gpui_telegram_desktop/tests/fusion_stage_truth_tests.rs`.
- D003 exclusively owns `crates/gpui_telegram_desktop/src/views/context_inspector.rs` and new `crates/gpui_telegram_desktop/tests/fusion_inspector_truth_tests.rs`.
- D004 integration worker exclusively owns `crates/gpui_telegram_desktop/src/views/shell.rs` and new `crates/gpui_telegram_desktop/tests/fusion_shell_truth_tests.rs`. It uses the unchanged public signatures of D001–D003; coordinator handles existing test adaptation after all workers finish. No worker modifies shared models, Cargo manifests, ledgers, other tests or other workers' files.

Preserve public signatures for `new`, `configure_for_cloud_workspace`, `set_chat`, and `set_peer`. Do not add dependencies. Existing dirty content is the baseline; edits must preserve unrelated changes. If useful for old fixture-based tests, retain illustrative data only behind an explicitly named `new_demo` constructor never called by production.

## Requirements

1. D001: `new` and cloud reset contain no fabricated dialogs, story entries, top peers, premium notices or selected ID. The structural All tab may exist at count zero. Production folder definitions/counts require server data; do not seed Personal/Work/Bots counts. Clear previous account data on cloud reset.
2. D002: `new` and cloud reset have no selected ID, messages, pins or invented online subtitle. Reset clears histories, pin caches, saved scroll positions, unread count, pending reply/media/call/bot actions and context menus to prevent account bleed. Rebuild row-height state. Neutral 'Select a chat' text is allowed. Server URL/username alone must not create messages or connectivity claims. Preserve real message rendering and scrolling behavior.
3. D003: `new` and cloud reset have empty profile facts, members, shared media and encryption fingerprint; close drawer and clear pending actions. `set_peer` uses supplied display facts only; never synthesize a username from a name. Clear prior peer phone, username, members/media/security facts when selection changes. Preserve same-peer facts if no identity change is observable; do not invent an ID contract silently.
4. D004: authoritative chat list replaces only server-managed list; empty list clears selection, stage and inspector. Async results must be fenced to the requesting auth session so stale responses cannot repopulate a switched/logged-out account. Selection is coherent across sidebar/stage/inspector. Real sends require a selected authoritative chat; fixed demo IDs must not route special behavior. The latter integration may require follow-up slices and must not be claimed complete by D001–D003 alone.

D004 details: preserve the current selection if still present, otherwise leave no selection (do not silently select/send to an arbitrary peer). Explicit user selection loads history. Derive no online/last-message timestamp from chat-list absence of such metadata. Auth reset/logout clears account-bound conversation views. Fence both chat-list and history responses to the captured session token and user, and prevent older chat refreshes overwriting newer refreshes using a request generation if needed. Reject sends without authenticated session and selected server dialog before consuming composer text; never use `unwrap_or(1)` as routing. Do not expand this slice to solve backend messaging/agent routing: record any fixed ID/name based AI behavior as a required follow-up if a genuine peer-kind contract is unavailable. Meaningful model/helper tests are useful but must not be described as full GPUI runtime acceptance.

## Verification and release

Workers add meaningful constructor/reset/selection tests, including resetting a previously populated model. Do not run builds or tests concurrently: coordinator serializes verification in the sole workspace `target/` and runs mandated garbage cleanup. Test fixtures use RAII temporary paths. Do not fabricate passing results. Existing fixture-based tests that depend on default demo data must be reported and migrated to explicit fixtures by integrator.

Coordinator reviews incremental diffs against per-file preimages, runs focused tests, then checks affected integration and real GPUI empty/loading/loaded/error interactions. No release claim from unit tests alone. Rollback uses the owned incremental patch, never a repository reset. No production deployments, account messages or secret reads belong to this slice.

Source oracle: Telegram dialog/history/profile selection and empty-state behavior in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/{dialogs,history,info}`; existing specifications 01/02/04/05 provide the wider migration scope.
