# Project: GPUI Telegram 1:1 Desktop & Independent Rust Self-Hosted Server

## Architecture

### System Topology & Design Principles
The project reconstructs Telegram Desktop with 1:1 UI/UX fidelity in **GPUI (Rust)**, backed by an **independent, self-hosted Rust server** (`gpui_telegram_server`), with seamless, non-fragmented integration of existing advanced capabilities (AI agents, Global Task Center, MCP ecosystem, Remote Computer control, Full-duplex Talk Mode, and Rich Interactive Cards).

**TOP-PRIORITY USER MANDATE: Full Inheritance & Direct Reuse of Existing Rust Assets**:
All existing Rust crates and implementations in `/Users/bhrum/Documents/ChatGPT/fabushi-unified-design-20261105` are unconditionally inherited and reused as the core domain, storage, and runtime engine. The GPUI client replaces the previous Electron + React/TS layers (`desktop/` and `frontend/`), directly consuming the proven Rust domain layer, state machines, SQLite database (`SandAgentDb`), and execution runtimes.

**TOP-PRIORITY USER MANDATE #2 & #3: Telegram Desktop Source-File Parity Ledger Driven Implementation**:
Implementation and verification are strictly governed by a **Source-File Parity Ledger** rather than abstract conceptual descriptions.
- Primary focus: 100% focused on `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/` (including `dialogs/`, `history/`, `chat_helpers/`, `media/`, `calls/`, `boxes/`, `settings/`, `window/`, `data/`, `core/`, `main/`, `inline_bots/`, `info/`, `overview/`, `profile/`, `support/`, `api/`, and `.style` files).
- Existing ledger assets: Fully inherit and extend `/Users/bhrum/Documents/ChatGPT/fabushi-unified-design-20261105/projects/telegram-desktop-rust/parity-ledger.json` (693KB) and `module-map.md`.
- OpenClaw source ledger is paused/skipped (inherited Rust code in `inherited/` is preserved as-is).
- Every Telegram feature must map:
  `[tdesktop C++ source files]` ➔ `[Core classes / state machine responsibilities]` ➔ `[Target GPUI / Rust crate files & symbols]` ➔ `[Verification tests & status]`.

**TOP-PRIORITY USER MANDATE #4: 100% Full Source-File Enumeration Ledger (ALL 3,441 SourceFiles / 6,859 Repo Files / 16,202 Entries, Zero Omitted)**:
- In Milestone 1 (`worker_m1`), a full automated scan script over `/Users/bhrum/.gemini/antigravity/scratch/tdesktop` generates:
  * `docs/specs/tdesktop-full-source-ledger.json` (and updated `parity-ledger.json`): 100% mapping of ALL 3,441 source and `.style` files in `Telegram/SourceFiles/` (plus classification of all 6,859 repo files), with `omitted = 0, unmapped = 0, unknown = 0`.
  * Human-readable subdirectories in `docs/specs/ledger/` covering every directory (`dialogs/`, `history/`, `chat_helpers/`, `media/`, `calls/`, `boxes/`, `settings/`, `window/`, `data/`, `core/`, `main/`, `api/`, `storage/`, etc.).
  * Exact schema fields per entry: `source_path`, `subsystem`, `feature_id`, `source_symbols_and_responsibility`, `disposition`, `inherited_rust_path`, `target_rust_gpui_path`, `target_symbol`, `verification_test_id`.
  * Automated verification script `scripts/verify_ledger_closure.py` enforcing 100% closure as a mandatory gate check in M1–M6!

**TOP-PRIORITY USER MANDATE #5: Anti-Symbol-Collapse & Anti-Monolith File Structure Rules**:
1. **Anti-Symbol-Collapse Rule**: Except for C++ natural pairs (`xxx.h` + `xxx.cpp`), multiple upstream files MUST NOT collapse into generic catch-all symbols. Every distinct upstream business/UI source file must map to an explicit, distinct target `struct`, `enum` variant, or dedicated handler (`target_symbol`).
2. **Anti-Monolith Submodule Split Rule**: No module may be stuffed into a single monolithic `.rs` file. Target crates (especially `gpui_telegram_desktop`, `gpui_telegram_client`, `gpui_telegram_server`) must maintain fine-grained multi-file directory hierarchies mirroring upstream (`src/boxes/*.rs`, `src/settings/*.rs`, `src/history/media/*.rs`, `src/dialogs/*.rs`, `src/info/*.rs`, `src/chat_helpers/*.rs`, `src/calls/*.rs`).
3. **Automated Fan-In & Granularity Verification**: `scripts/verify_ledger_closure.py` enforces maximum fan-in thresholds and symbol uniqueness checks to mechanically reject collapse and monolith anti-patterns across M1–M6.

**TOP-PRIORITY USER MANDATE #6: Sharded Ledger Architecture (Anti-Bloat & Anti-Lock)**:
1. **Lightweight Master Index + Sharded Ledger Structure**:
   - Master Index: `docs/specs/ledger/index.json` & `docs/specs/ledger/README.md` (summary table with directory path, file count, mapped count, implemented count, verified count).
   - Independent Shards: in `docs/specs/ledger/shards/` matching `Telegram/SourceFiles/` subdirectories (`dialogs.json`, `history.json`, `boxes.json`, `media.json`, `calls.json`, `settings.json`, `chat_helpers.json`, `data.json`, `window.json`, `info.json`, etc., across 28 distinct shards).
2. **Lock-Free Concurrency & Anti-Context-Overflow**: Shards are compact (< a few hundred lines), allowing builders/reviewers in M2–M5 to read/update only their relevant subsystem shard without context blowup or concurrent file collision.
3. **Automated Aggregation Verification**: `scripts/verify_ledger_closure.py` automatically scans and aggregates `docs/specs/ledger/shards/*.json`, asserting 100% closure against all 3,441 source files.

**TOP-PRIORITY USER MANDATE #7: Deep Source Reading & Real Symbol/Signature Extraction (No Blind Filename Guessing)**:
1. **True Source Parsing in M1**:
   - `worker_m1` MUST genuinely read and parse every single one of the 3,441 source files in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/`.
   - Extract real source code features: `source_classes_and_structs`, `source_methods_and_signals`, `style_tokens_and_metrics` (for `.style`), plus `source_line_count` and `source_sha256`.
2. **Read-Upstream-Before-Write-Rust in M2–M5**:
   - Any Builder implementing a shard MUST read the upstream C++ source file referenced by `source_path` before writing Rust code, translating actual state machines and algorithms 1:1.
3. **Gate 5 Real Source Verification**:
   - `scripts/verify_ledger_closure.py` enforces non-empty `source_sha256`, `source_line_count`, and authentic extracted symbols/tokens for every entry.

**TOP-PRIORITY USER MANDATE #8: Massively Parallel Multi-Worker Swarm Execution**:
1. **Milestone 1 Implementation Decomposition**:
   - Decompose M1 implementation into multiple parallel workers:
     * `worker_m1_workspace`: Initializes `/Users/bhrum/teamwork_projects/gpui_telegram`, copies and wires `inherited/` Rust crates, generates root `Cargo.toml` and 9 new crate skeletons, runs `cargo check`.
     * `worker_m1_docs`: Materializes `docs/architecture.md`, `docs/specs/01`..`05` core spec documents, and `TEST_INFRA.md`.
     * `worker_m1_ledger_1` / `worker_m1_ledger_2`: Deep AST/token extraction across all 3,441 source files, generates 28 subsystem sharded ledgers in `docs/specs/ledger/shards/` + `docs/specs/ledger/index.json`, and authors `scripts/verify_ledger_closure.py` (with the 5 gates) and passes validation.
2. **M2–M5 Multi-Worker Swarm Execution**:
   - Every milestone dispatches 3–5 parallel workers mapped to specific crate/shard boundaries, reading upstream C++ files and writing fine-grained Rust code concurrently.
3. **Orchestrator Succession Management**:
   - At spawn count >= 16, execute self-succession cleanly to spawn fresh orchestrators with full concurrency quotas!

```
                      +-----------------------------------------+
                      |       GPUI Desktop Client (Rust)        |
                      |  - NavRail (52px)                       |
                      |  - ChatSidebar (288px, uniform_list)    |
                      |  - ConversationStage (SumTree list)     |
                      |  - CanonicalComposer (IME Safe)         |
                      |  - ContextInspectorDrawer (320px)       |
                      |  - Talk Mode Orb / Computer Workbench   |
                      +--------------------+--------------------+
                                           | (Direct FFI / In-Memory Channel)
                                           v
+-------------------------------------------------------------------------------+
|             Inherited Production Rust Engine (fabushi-unified-design)         |
|  - source/host (app, runner, ACP, MCP relay, tests)                           |
|  - source/mahayana/mahayana-rs (SandAgentDb, SQLite WAL, mahayana-app-host)   |
|  - source/mahayana/codex-rs (AST, grammar, code indexing)                     |
|  - native/mahayana-messaging (High-throughput message pipeline)               |
|  - source/node-agent-coordinator & box-exec-daemon (CUA & Remote execution)   |
|  - fabushi-backend (Protocol contracts & backend logic)                       |
+-------------------------------------------------------------------------------+
                                           |
                    HTTP / REST (Media)    |  WebSocket (RPC / Realtime PTS Events)
                    WebRTC (VoIP & Video)  |
                                           v
+-------------------------------------------------------------------------------+
|                      gpui_telegram_server (Axum + Tokio)                      |
|                                                                               |
|  +------------------+  +------------------+  +-----------------------------+  |
|  | Auth & Sessions  |  | Gateway & WS Hub |  | Sequence Sync Engine (PTS)  |  |
|  | (Passkeys / JWT) |  | (Heartbeat/State)|  | (Gap Detection & Catch-up) |  |
|  +------------------+  +------------------+  +-----------------------------+  |
|                                                                               |
|  +------------------+  +------------------+  +-----------------------------+  |
|  | Messaging Engine |  | Media Storage    |  | Full-Text Search (Tantivy)  |  |
|  | (Atomic Writes)  |  | (Chunked/Ranges) |  | (Sub-ms Message Indexing)   |  |
|  +------------------+  +------------------+  +-----------------------------+  |
|                                                                               |
|  +------------------+  +------------------+  +-----------------------------+  |
|  | WebRTC Signaling |  | AI & Subagents   |  | Task Center & Cron Worker   |  |
|  | (STUN/TURN/SFU)  |  | (ACP Runner)     |  | (7-State FSM / 5-Field Cron)|  |
|  +------------------+  +------------------+  +-----------------------------+  |
|                                                                               |
|  +-------------------------------------------------------------------------+  |
|  |                Persistence Layer: SQLite (WAL) / PostgreSQL             |  |
|  +-------------------------------------------------------------------------+  |
+-------------------------------------------------------------------------------+
```

---

## Feature Inventory

| # | Feature ID | Name | Description | Milestone | Source |
|---|---|---|---|---|---|
| 1 | `TCG-N01` | Auth & Registration | Phone, username, passkey login, session generation | M2, M3, M5 | survey |
| 2 | `TCG-N02` | Sessions & Security | Device list, session termination, 2FA cloud password | M2, M3, M5 | survey |
| 3 | `TCG-N03` | Dialogs & Folders | Chat list, folder tabs, unread badges, pinning | M2, M3, M5 | survey |
| 4 | `TCG-N04` | Messages & Bubbles | Incoming/outgoing bubbles, grouping, timestamps, delivery ticks | M2, M3, M5 | survey |
| 5 | `TCG-N05` | Input Composer & IME | Multi-line auto-growth, IME composition protection, drafts | M2, M5 | survey |
| 6 | `TCG-N06` | Media & Attachments | Photos, videos, files, chunked upload, range streaming | M2, M3, M5 | survey |
| 7 | `TCG-N07` | Voice Notes & Waveforms | 5-bit compact waveform, audio record gesture, playback | M2, M3, M5 | survey |
| 8 | `TCG-N08` | Audio Player | Floating audio player bar, progress scrubber, speed toggle | M2, M5 | survey |
| 9 | `TCG-N09` | Video Notes | Round video playback, recording, inline preview | M2, M3, M5 | survey |
| 10 | `TCG-N10` | Media Viewer Lightbox | Fullscreen zoom, pan, carousel, video controls | M5 | survey |
| 11 | `TCG-N11` | Groups & Supergroups | Basic groups, supergroups, admin permissions, member list | M2, M3, M5 | survey |
| 12 | `TCG-N12` | Channels & Broadcasts | Public/private channels, subscriber count, comments | M2, M3, M5 | survey |
| 13 | `TCG-N13` | Forum Topics | Topic tabs, thread breadcrumbs, topic message filtering | M2, M3, M5 | survey |
| 14 | `TCG-N14` | Reactions & Emoji | Emoji reactions, reaction picker popup, counters | M2, M3, M5 | survey |
| 15 | `TCG-N15` | Stickers & GIFs | Sticker picker, animated stickers, GIF search/playback | M2, M5 | survey |
| 16 | `TCG-N16` | Polls & Quizzes | Single/multi choice polls, anonymous/public, voting | M2, M3, M5 | survey |
| 17 | `TCG-N17` | Pinned Messages | Pin message, pinned message banner, unpin, jump to pin | M2, M3, M5 | survey |
| 18 | `TCG-N18` | Scheduled & Silent Messages | Scheduled message queue, silent send without notification | M2, M3, M5 | survey |
| 19 | `TCG-N19` | Edit & Delete Messages | Edit message, delete for self/everyone, tombstone sync | M2, M3, M5 | survey |
| 20 | `TCG-N20` | PTS Sequence Sync Engine | Atomic sequence numbering, gap catch-up via SyncGetDifference | M2, M3 | survey |
| 21 | `TCG-N21` | Full-Text Search | Tantivy FTS engine, global search, in-chat search, highlights | M2, M3, M5 | survey |
| 22 | `TCG-N22` | Shared Media Archive | Photos, videos, files, links, voice tabs in Inspector | M2, M3, M5 | survey |
| 23 | `TCG-N23` | Context Inspector Drawer | 320px unified right drawer: profile, members, media, settings | M5 | survey |
| 24 | `TCG-N24` | 1-on-1 VoIP Calls | WebRTC audio/video call signaling, STUN/TURN, mute/video | M2, M3, M4, M5 | survey |
| 25 | `TCG-N25` | Group Voice & Video Chats | Group call SFU bridge, active speaker indicators, grid | M2, M3, M4, M5 | survey |
| 26 | `TCG-N26` | Screen Sharing | Low-latency screen capture & streaming over WebRTC | M4, M5 | survey |
| 27 | `TCG-N27` | Bot Platform & Keyboards | Inline keyboards, callback queries, bot commands menu | M2, M3, M4, M5 | survey |
| 28 | `TCG-N28` | Mini Apps Sandbox | Sandboxed WebViews with CSP and custom protocol bridge | M4, M5 | survey |
| 29 | `TCG-N29` | Contacts & Address Book | Contact roster, search, human and agent profiles | M2, M3, M5 | survey |
| 30 | `TCG-N30` | User Profile & Status | Avatar, username, bio, presence status (online/offline) | M2, M3, M5 | survey |
| 31 | `TCG-N31` | Chat Settings & Themes | Day Blue, Night, Custom palettes, typography, chat background | M5 | survey |
| 32 | `TCG-N32` | Notifications & Badges | System notifications, audio alerts, dock unread badges | M3, M5 | survey |
| 33 | `TCG-N33` | Privacy & Security | Last seen privacy, phone number visibility, blocklist | M2, M3, M5 | survey |
| 34 | `TCG-N34` | Storage & Local Cache | Local SQLite database, disk cache cleanup, limits | M2, M5 | survey |
| 35 | `TCG-N35` | Proxy & Network Config | SOCKS5, HTTP proxy support with RTT ping diagnostics | M2, M5 | survey |
| 36 | `TCG-N36` | Draft Sync & Fencing | Auto-save draft per peer, draft preview in sidebar | M2, M5 | survey |
| 37 | `TCG-N37` | Archive & Filter Chips | Archive folder, mute chats, custom folder tabs | M5 | survey |
| 38 | `TCG-N38` | Stories Strip | Top stories carousel, 24h photo/video stories viewer | M2, M5 | survey |
| 39 | `TCG-N39` | Secret Chats & E2EE | End-to-end encrypted direct message exchange | M2, M5 | survey |
| 40 | `TCG-N40` | Self-Destruct Timers | Auto-delete timer on messages (1s to 1 week) | M2, M3, M5 | survey |
| 41 | `TCG-N41` | Location & Live Location | Static map card, live location updates | M2, M5 | survey |
| 42 | `TCG-N42` | Web Link Previews | OpenGraph rich link previews with thumbnails | M3, M5 | survey |
| 43 | `TCG-N43` | Export Chat History | Export messages and media to JSON / HTML | M5 | survey |
| 44 | `TCG-N44` | Language & Localization | Multi-language support (English, Chinese, etc.) | M5 | survey |
| 45 | `TCG-N45` | Keybindings & Shortcuts | Telegram Desktop native keybindings (Cmd+K, Cmd+1..9) | M5 | survey |
| 46 | `TCG-N46` | Custom Window Frame | Custom Metal title bar, traffic lights, smooth resize | M5 | survey |
| 47 | `TCG-N47` | Drag-and-Drop & Clipboard | Drop files/images onto composer, copy/paste formatted text | M5 | survey |
| 48 | `TCG-N48` | Auto-Download Rules | Automatic media download settings by connection type | M5 | survey |
| 49 | `TCG-N49` | Multi-Account Switching | Multi-session profile switcher in Left NavRail | M2, M5 | survey |
| 50 | `TCG-N50` | Scope Generation Fencing | Monotonic accountScopeGeneration isolation | M2, M5 | survey |
| 51 | `TCG-N51` | Chunked Media Upload | 512KB-1MB chunk upload pipeline with resume support | M2, M3 | survey |
| 52 | `TCG-N52` | Range-Request Streaming | HTTP 206 Partial Content audio/video streaming | M2, M3 | survey |
| 53 | `CAP-01` | AI Multi-Agent ACP | Turn-taking agent runner, thinking bubbles, swarm lineage (inherited from source/host) | M4, M5 | survey |
| 54 | `CAP-02` | Global Task Center & Cron | 7-state task lifecycle FSM, 5-field cron routines, approval gates (inherited from source/host) | M4, M5 | survey |
| 55 | `CAP-03` | MCP & Plugin Ecosystem | Stdio/SSE tool registry, permission dock, sandboxed tools (inherited from source/host) | M4, M5 | survey |
| 56 | `CAP-04` | Remote Device Control | CUA actions, low-latency VNC streaming, exclusive lease (inherited from node-agent) | M4, M5 | survey |
| 57 | `CAP-05` | Full-Duplex Talk Mode | Realtime audio, VAD, 60ms barge-in, GPUI energy orb (inherited from source/host) | M4, M5 | survey |
| 58 | `CAP-06` | A2UI Interactive Cards | Rich declarative cards, todos, inline button grids, forms | M2, M5 | survey |

---

## Milestones

| # | Name | Scope | Dependencies | Status |
|---|---|---|---|---|
| **M1** | Architecture & Specification Documentation + Source-File Parity Ledger | Author comprehensive development and architecture documents in target repository `docs/` (`01-telegram-1to1-feature-inventory.md`, `02-gpui-uiux-component-hierarchy.md`, `03-rust-server-architecture-contracts.md`, `04-existing-capabilities-routing-map.md`, `05-test-and-verification-plan.md`, `SOURCE_FILE_LEDGER.md`, `parity-ledger.json`), initialize target directory `~/teamwork_projects/gpui_telegram`, **fully copy and inherit existing Rust crates from `fabushi-unified-design-20261105`**, and scaffold Cargo workspace | none | DONE |
| **M2** | Core Protocols, Models & Sequence Sync Foundation | Implement domain entities, cryptographic session models, protobuf/serde wire types, 7-state task FSM, 5-bit audio waveform codec, and atomic PTS sequence sync types in `core`, `proto`, and `media` crates connecting directly to inherited `SandAgentDb`, verified against the Source-File Parity Ledger | M1 | DONE |
| **M3** | Independent Self-Hosted Backend Server | Implement Axum + Tokio server, SQLite WAL schemas, atomic PTS sequencing & catch-up difference queries (`SyncGetDifference`), Tantivy full-text search indexer, chunked media upload/stream service, WebSocket realtime push gateway, WebRTC call signaling in `server` crate, verified against the Source-File Parity Ledger | M2 | DONE |
| **M4** | Existing Capabilities Subsystem Integration | Wire inherited AI multi-agent ACP runner, Primary Global Task Center engine with 5-field cron routines & approval gates, MCP Stdio/SSE tool registry & permission dock, Remote computer CUA lease broker & VNC frame streaming, Full-duplex Talk Mode audio engine into GPUI Telegram services, verified against the Source-File Parity Ledger | M2, M3 | DONE |
| **M5** | GPUI 1:1 Telegram Desktop Client Application & 28-Shard Target Materialization | Implement full GPUI 0.2.2 desktop client reproducing Telegram 1:1 UI/UX: 52px NavRail, 288px ChatSidebar with folder tabs and uniform_list, ConversationStage with SumTree virtualized transcript, 11 rich message card types, CanonicalComposer with IME safety and hold-to-record voice gesture, 320px ContextInspectorDrawer, and overlays (Talk Mode orb, Remote Computer workbench, Media Viewer, Calls, Settings) in `client` and `desktop` crates, alongside multi-worker materialization of all 28 ledger shards into physical compiling Rust files on disk | M2, M3, M4 | IN_PROGRESS |
| **M6** | Full E2E Verification & Production Release Packaging | Execute full automated test suites across all crates (including inherited contract tests in `source/host/tests/`), multi-device PTS concurrency sync tests, headless GPUI transcript stress tests, verify 100% test pass on E2E test suite (Tiers 1-4), adversarial test hardening, **full line-by-line audit & sign-off against the Source-File Parity Ledger**, and package the release binary | M1, M2, M3, M4, M5, TEST_READY.md | PLANNED |

---

## Code Layout

Target workspace: `/Users/bhrum/teamwork_projects/gpui_telegram`

```
gpui_telegram/
├── Cargo.toml                       # Top-level workspace manifest (includes new + inherited crates)
├── docs/                            # Comprehensive development documentation (R1, R2)
│   ├── specs/
│   │   ├── 01-telegram-1to1-feature-inventory.md
│   │   ├── 02-gpui-uiux-component-hierarchy.md
│   │   ├── 03-rust-server-architecture-contracts.md
│   │   ├── 04-existing-capabilities-routing-map.md
│   │   ├── 05-test-and-verification-plan.md
│   │   ├── SOURCE_FILE_LEDGER.md     # Master source-file parity ledger
│   │   └── parity-ledger.json       # Machine-readable parity ledger (from fabushi)
│   └── architecture.md
├── crates/
│   ├── gpui_telegram_core/          # Pure domain types, error types, security tokens
│   ├── gpui_telegram_proto/         # Wire formats, serde types, sync events, RPC schemas
│   ├── gpui_telegram_media/         # 5-bit waveform encoder, audio/video codecs, thumbnails
│   ├── gpui_telegram_webrtc/        # VoIP & Video call signaling, WebRTC gateway, STUN/TURN
│   ├── gpui_telegram_agent/         # AI ACP runner, Global Task FSM, MCP runtime, Cron engine
│   ├── gpui_telegram_server/        # Axum self-hosted server binary, SQLite WAL, Tantivy FTS
│   ├── gpui_telegram_client/        # Client networking, local cache, sync coordinator
│   ├── gpui_telegram_desktop/       # GPUI 0.2.2 GUI desktop binary (1:1 Telegram Desktop)
│   └── gpui_telegram_test_harness/  # E2E test suite runners, mocks, integration tests
├── inherited/                       # Fully inherited existing Rust crates from fabushi-unified-design
│   ├── host/                        # source/host (app, runner, ACP, MCP relay, tests)
│   ├── mahayana-rs/                 # source/mahayana/mahayana-rs (SandAgentDb, SQLite WAL)
│   ├── codex-rs/                    # source/mahayana/codex-rs
│   ├── mahayana-messaging/          # native/mahayana-messaging
│   ├── node-agent-coordinator/      # source/node-agent-coordinator
│   ├── box-exec-daemon/             # source/box-exec-daemon
│   └── fabushi-backend/             # fabushi-backend
└── scripts/
    ├── build_release.sh             # Production packaging script
    └── run_e2e_tests.sh             # E2E test suite launcher
```
