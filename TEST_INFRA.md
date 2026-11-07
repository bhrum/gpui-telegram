# GPUI Telegram Master Test Infrastructure & Test Architecture Specification (`TEST_INFRA.md`)

**Specification Version**: 1.0.0-RELEASE  
**Date**: 2026-11-06  
**Status**: Authoritative Architectural Baseline  
**Governing Architecture**: `/Users/bhrum/Documents/ChatGPT/.agents/teamwork/orchestrator/PROJECT.md`  
**Reference Upstream**: `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/`  
**Inherited Rust Crates**: `/Users/bhrum/Documents/ChatGPT/fabushi-unified-design-20261105/`  

---

## 1. Executive Summary & Core Testing Philosophy

### 1.1 Testing Philosophy: Opaque-Box, Requirement-Driven Verification
The GPUI Telegram test infrastructure is designed around a strict **opaque-box, requirement-driven verification philosophy**. Rather than testing internal Rust private implementation details or asserting transient in-memory structures, tests interact with the application and server strictly through their public observable interfaces:
1. **Headless GPUI Interface (`gpui::TestAppContext`)**: Exercises the graphical user interface headlessly, dispatching simulated mouse clicks, scrolls, keyboard events (including IME composition events), and asserting the visual layout tree, SumTree virtualized message items, canonical composer states, and modal overlays.
2. **WebSocket Realtime RPC Channel (`ws://<server>/ws/v1`)**: Connects over standard WebSockets, streaming serialized command envelopes, receiving Presentation Sequence (PTS) update frames, ping/pong heartbeats, and handling difference catch-up queries (`SyncGetDifference`).
3. **RESTful Media & Management API (`http://<server>/api/v1`)**: Validates HTTP endpoints for passkey/password authentication, resumable chunked file uploads (512KB-1MB chunks), SHA-256 CAS file verification, and HTTP 206 Partial Content range-request streaming.
4. **WebRTC Media & Signaling Channel**: Exchanges SDP offers, answers, and ICE candidate trickles for 1-on-1 audio/video VoIP, group call SFU media forwarding, and low-latency VNC screen frame streaming.

Every single test case across all tiers is derived from authoritative behavioral contracts:
- For Telegram Desktop features (52 features, `TCG-N01` through `TCG-N52`): Upstream C++ implementation contracts in `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/`.
- For Advanced Capabilities (6 features, `CAP-01` through `CAP-06`): Inherited production Rust contracts and test oracles in `/Users/bhrum/Documents/ChatGPT/fabushi-unified-design-20261105/` (e.g., `openclaw_audio_talk_contract.rs`, `delta_sync_contract.rs`, `production_turn_agent_owner.rs`).

---

### 1.2 Mandatory Governing Constraints

#### MANDATE #1: Full Inheritance of Existing Rust Crates & Baseline Contract Test Suites
All existing Rust crates and contract test suites from `/Users/bhrum/Documents/ChatGPT/fabushi-unified-design-20261105` must be fully inherited and preserved. Specifically:
- **`native/mahayana-messaging/tests/`**: 10 contract test suites (`delta_sync_contract.rs`, `m4_media_contract.rs`, `m5_contacts_groups_contract.rs`, `m6_channels_topics_contract.rs`, `messaging_contract.rs`, `platform_contract.rs`, `service_contract.rs`, `typing_contract.rs`, `unread_projection_contract.rs`, `websocket_gateway_contract.rs`).
- **`source/host/tests/`**: 455 contract test suites, including the 12 falsifiable audio talk oracles in `openclaw_audio_talk_contract.rs` (OCRM-AUDIO-O01 through OCRM-AUDIO-O12), `automation_schedule_contract.rs`, `auto_review_gate_contract.rs`, `widget_responses_local_tool_permission_contract.rs`, and `turn_observation_contract.rs`.
- These test suites serve as the **non-negotiable verified baseline**. A failure in any inherited test is classified as an immediate release blocker.

#### MANDATE #2 & #3: Telegram Desktop Source-File Parity Focus
Verification is driven by the **Source-File Parity Ledger**. All 52 Telegram features and test cases map 1:1 to upstream `tdesktop/Telegram/SourceFiles/` behavioral contracts. OpenClaw source ledger enumeration is paused/skipped (inherited code is preserved as-is).

---

## 2. 4-Tier Test Architecture Overview

The GPUI Telegram test infrastructure is structured into 4 formal, non-overlapping verification tiers containing **>= 667 automated test cases**:

```
+----------------------------------------------------------------------------------------------------+
|                                    TIER 4: REAL-WORLD SCENARIOS                                   |
|                          >= 29 Comprehensive End-to-End Multi-User Workflows                       |
|           (Multi-account onboarding, cross-network sync, agent task pipelines, audio calls)         |
+----------------------------------------------------------------------------------------------------+
|                               TIER 3: CROSS-FEATURE COMBINATIONS                                   |
|                             >= 58 Pairwise Interaction Test Cases                                  |
|         (Task Center + Talk Mode + Group Chat; Chunked Upload + Account Switch; etc.)              |
+----------------------------------------------------------------------------------------------------+
|                              TIER 2: BOUNDARY & CORNER CASES                                       |
|                  >= 290 Test Cases (58 Features x >= 5 Boundary/Corner Tests)                      |
|         (PTS gap re-sync, 1MB chunk boundaries, network chaos, IME composition aborts, timeouts)  |
+----------------------------------------------------------------------------------------------------+
|                                TIER 1: ISOLATED FEATURE COVERAGE                                   |
|                  >= 290 Test Cases (58 Features x >= 5 Happy Path Tests)                           |
|         (Happy-path verification of every feature in isolation via Headless GPUI & RPC)            |
+----------------------------------------------------------------------------------------------------+
|                           INHERITED BASELINE CONTRACT TEST SUITES                                  |
|                 465 Inherited Test Suites (10 mahayana-messaging + 455 host runtime)               |
+----------------------------------------------------------------------------------------------------+
```

### 2.1 Test Tier Summary Matrix

| Tier | Category | Scope & Objective | Test Count | Test ID Prefix |
|---|---|---|---|---|
| **Tier 1** | Feature Coverage | Isolated happy-path functional validation for each feature in scope | **290** (58 × 5) | `TEST-T1-TCGxx-*`, `TEST-T1-CAPxx-*` |
| **Tier 2** | Boundary & Corner Cases | Extreme values, invalid inputs, network drops, PTS gaps, race conditions | **290** (58 × 5) | `TEST-T2-TCGxx-*`, `TEST-T2-CAPxx-*` |
| **Tier 3** | Cross-Feature Interactions | Multi-feature pairwise integration flows and state machine transitions | **58** | `TEST-T3-XF-*` |
| **Tier 4** | Real-World User Scenarios | Complete multi-user, multi-device end-to-end operational journeys | **29** | `TEST-T4-E2E-*` |
| **Baseline** | Inherited Contracts | Regression baseline inherited from `fabushi-unified-design-20261105` | **465 suites** | `OCRM-*`, `M4-*`, `M5-*`, `M6-*` |
| **Total** | **Comprehensive Test Suite** | **Exhaustive System-Wide Verification** | **>= 667 + 465 Baseline** | |

---

## 3. Test Runner & Harness Architecture (`gpui_telegram_test_harness`)

### 3.1 Crate Topology & Directory Structure
The test harness is implemented in `crates/gpui_telegram_test_harness/`, decoupled from production code and structured as follows:

```
crates/gpui_telegram_test_harness/
├── Cargo.toml
├── src/
│   ├── lib.rs                          # Harness root exports
│   ├── environment/
│   │   ├── mod.rs                      # TestEnvironment lifecycle manager
│   │   ├── server_instance.rs          # In-process Axum server launcher (ephemeral port 127.0.0.1:0)
│   │   ├── sqlite_fixture.rs           # In-memory SQLite WAL database initializer & migrator
│   │   └── tantivy_fixture.rs          # RAM-backed Tantivy FTS index fixture
│   ├── gpui_headless/
│   │   ├── mod.rs                      # Headless GPUI wrapper around gpui::TestAppContext
│   │   ├── window_driver.rs            # Virtual window creation, resizing, dpi emulation
│   │   ├── event_simulator.rs          # Mouse, scroll, keyboard, and IME text composition injector
│   │   └── view_inspector.rs           # SumTree transcript query, element tree assertion helpers
│   ├── network_chaos/
│   │   ├── mod.rs                      # NetworkChaosController
│   │   ├── latency_interceptor.rs      # Configurable artificial latency, jitter (Gaussian/Uniform)
│   │   ├── packet_drop.rs              # Random & deterministic packet loss, frame truncation
│   │   ├── socket_disrupter.rs         # Abrupt TCP RST, graceful WS close, TLS renegotiation drops
│   │   └── split_brain.rs              # Multi-device partition simulator
│   ├── account_fencing/
│   │   ├── mod.rs                      # AccountScopeFencingHarness
│   │   ├── generation_counter.rs       # Monotonic accountScopeGeneration manager
│   │   └── stale_payload_trap.rs       # Interceptor ensuring requests from older generations panic/reject
│   └── oracles/
│       ├── mod.rs                      # Assertions and sequence validators
│       ├── pts_sequence_oracle.rs      # Monotonic sequence verifier, gap detector, SyncGetDifference validator
│       ├── waveform_oracle.rs          # 5-bit audio waveform quantization verifier
│       └── cua_lease_oracle.rs         # Remote computer control lease FSM verifier
└── tests/
    ├── tier1_feature_coverage.rs       # 290 Tier 1 tests
    ├── tier2_boundary_corner.rs        # 290 Tier 2 tests
    ├── tier3_cross_feature.rs          # 58 Tier 3 tests
    └── tier4_real_world_e2e.rs         # 29 Tier 4 tests
```

### 3.2 Invocation Semantics & Cargo Targets
The test suite is executable via standard `cargo test` workflows with fine-grained selector flags:

```bash
# 1. Run all tests in the workspace (including inherited contract tests)
cargo test --workspace

# 2. Run the dedicated GPUI Telegram E2E test harness
cargo test -p gpui_telegram_test_harness

# 3. Run a specific tier
cargo test -p gpui_telegram_test_harness --test tier1_feature_coverage
cargo test -p gpui_telegram_test_harness --test tier2_boundary_corner
cargo test -p gpui_telegram_test_harness --test tier3_cross_feature
cargo test -p gpui_telegram_test_harness --test tier4_real_world_e2e

# 4. Run tests for a specific feature by pattern
cargo test -p gpui_telegram_test_harness -- --nocapture tcg_n20_pts
cargo test -p gpui_telegram_test_harness -- --nocapture cap_05_talk_mode

# 5. Run the inherited baseline contract suites
cargo test -p mahayana-messaging --test delta_sync_contract
cargo test -p mahayana_host_runtime --test openclaw_audio_talk_contract
```

### 3.3 Test Environment Lifecycle & Isolation
Every test execution guarantees **100% hermetic isolation**:
1. **Ephemeral Server & Ports**: The server is spawned inside Tokio test runtime binding to `127.0.0.1:0`. The operating system assigns an ephemeral port, preventing port collisions during concurrent test execution (`--test-threads=8`).
2. **In-Memory SQLite with WAL**: Each test allocates an isolated SQLite database using `file:memdb_?mode=memory&cache=shared` with full foreign keys enabled and WAL semantics emulated.
3. **RAM Tantivy Index**: Full-text search indexes are initialized in RAM (`Index::create_in_ram(schema)`).
4. **Isolated Temp Storage**: Media files, chunks, and thumbnails are written to unique temporary directories (`tempfile::tempdir()`) that are purged automatically upon test completion.
5. **Virtual Time & Headless Dispatch**: The `gpui::TestAppContext` provides a deterministic virtual clock (`cx.advance_clock(duration)`). Tests do NOT use wall-clock `tokio::time::sleep`; all asynchronous events resolve deterministically via `cx.run_until_parked()`.

---

### 3.4 Key Test Harness Components

#### 3.4.1 Headless GPUI Testing via `gpui::TestAppContext`
The harness exercises UI components headlessly without requiring a physical display or GPU rendering window:
```rust
#[gpui::test]
async fn test_harness_headless_composer_flow(cx: &mut gpui::TestAppContext) {
    let mut harness = HeadlessGpuiApp::new(cx).await;
    
    // Mount Chat Stage with specific active conversation
    harness.select_conversation(conversation_id).await;
    
    // Simulate IME text composition: user inputs 'h', 'e', 'l', 'l', 'o'
    harness.simulate_ime_input("Hello, GPUI Telegram!").await;
    
    // Assert composer auto-expansion height
    let composer_bounds = harness.find_element_bounds("canonical_composer_input");
    assert!(composer_bounds.size.height > px(36.0));
    
    // Dispatch Cmd+Enter or Send Button click
    harness.dispatch_key_combination(Keystroke::parse("cmd-enter").unwrap()).await;
    
    // Advance virtual executor until all layout mutations and network pushes settle
    cx.run_until_parked();
    
    // Assert optimistic bubble in SumTree transcript view
    let transcript = harness.query_sumtree_messages();
    assert_eq!(transcript.last().unwrap().text, "Hello, GPUI Telegram!");
    assert_eq!(transcript.last().unwrap().status, MessageStatus::Sending);
}
```

#### 3.4.2 Network Chaos Controller (`NetworkChaosController`)
Validates client and server resilience under real-world hostile network conditions:
```rust
pub struct NetworkChaosController {
    latency: Arc<AtomicU64>,
    jitter: Arc<AtomicU64>,
    drop_rate_ppm: Arc<AtomicU32>, // parts per million
    disconnect_flag: Arc<AtomicBool>,
}

impl NetworkChaosController {
    pub fn inject_latency(&self, min_ms: u64, max_ms: u64);
    pub fn inject_packet_drop(&self, drop_percentage: f32);
    pub fn trigger_abrupt_disconnect(&self); // Simulates TCP RST
    pub fn restore_network(&self);
}
```

#### 3.4.3 Account Scope Generation Fencing (`AccountScopeFencingHarness`)
Guarantees that state from previous accounts or sessions never leaks into new sessions:
```rust
pub struct AccountScopeFencingHarness {
    current_account_id: Uuid,
    current_generation: u64,
}

impl AccountScopeFencingHarness {
    pub async fn switch_account(&mut self, next_account_id: Uuid) -> u64 {
        self.current_generation += 1;
        self.current_account_id = next_account_id;
        // Broadcasts generation invalidation to all client stores & background workers
        self.broadcast_scope_generation(self.current_generation).await;
        self.current_generation
    }
}
```

#### 3.4.4 Atomic PTS Sequence Sync Oracle (`PtsSequenceOracle`)
Mathematically verifies Telegram's atomic Presentation Sequence invariants:
$$\forall i \in [1, N]: \quad \text{PTS}_i = \text{PTS}_{i-1} + 1$$
If the client detects $\text{PTS}_{\text{recv}} > \text{PTS}_{\text{local}} + 1$, it MUST enter `CatchUp` state and execute `SyncGetDifference(pts = \text{PTS}_{\text{local}}, limit = 100)`.

---

## 4. Inherited Baseline Contract Test Catalog (Mandate #1)

In strict compliance with **Mandate #1**, the existing Rust test suites from `/Users/bhrum/Documents/ChatGPT/fabushi-unified-design-20261105` are unconditionally inherited. These 465 test suites form the verified baseline and must maintain a 100% pass rate in all CI builds.

### 4.1 Native Messaging Contract Test Suites (`native/mahayana-messaging/tests/`)
Contains 10 high-throughput protocol, synchronization, and storage contract suites:

| Suite File | Scope & Capabilities Tested | Key Invariants Verified |
|---|---|---|
| `delta_sync_contract.rs` | Delta sync protocol, monotonic sequence numbers, gap catch-up | In-order delivery, gap detection, catch-up replay idempotency |
| `m4_media_contract.rs` | Media upload, chunk assembly, CAS blob storage | SHA-256 integrity, 5-bit audio waveform generation |
| `m5_contacts_groups_contract.rs` | Contact roster, basic group creation, member permissions | Roster isolation, admin promotion, member limits |
| `m6_channels_topics_contract.rs` | Supergroups, broadcast channels, forum topics | Topic hierarchy, thread breadcrumbs, public channel comments |
| `messaging_contract.rs` | Message dispatch, deduplication via `client_nonce`, formatting | Zero message loss, markdown formatting preservation |
| `platform_contract.rs` | Cross-platform OS abstractions, path normalization | Sandboxed path confinement, OS clipboard bridges |
| `service_contract.rs` | Core `MessagingService` lifecycle, state transitions | DB migration idempotency, service restart recovery |
| `typing_contract.rs` | Ephemeral typing actions, rate limits, 5s timeout | Typing indicator auto-cancellation, broadcast filtering |
| `unread_projection_contract.rs` | Unread counters, folder tabs projection, mark-as-read | Accurate unread badges, muted chat badge suppression |
| `websocket_gateway_contract.rs` | WS connection upgrade, framing, 30s heartbeat ping/pong | Graceful disconnect detection, connection pooling |

### 4.2 Host Runtime & Capabilities Contract Test Suites (`source/host/tests/`)
Contains 455 contract test suites governing execution, background tasks, ACP multi-agent runner, and Talk Mode. Key representative suites include:

| Suite File | Scope & Capabilities Tested | Key Invariants & Test Oracles |
|---|---|---|
| `openclaw_audio_talk_contract.rs` | Full-duplex Talk Mode, VAD, 60ms barge-in, CallSession FSM | Validates 12 falsifiable test oracles (`OCRM-AUDIO-O01` through `OCRM-AUDIO-O12`) |
| `auto_review_gate_contract.rs` | Security approval gates for sensitive tool execution | 10-minute approval timeout, cryptographic token authorization |
| `automation_schedule_contract.rs` | 5-field cron parsing, routine triggers, next-run prediction | Cron specification compliance, jitter-free execution |
| `widget_responses_local_tool_permission_contract.rs` | MCP dynamic tool permission dock & user confirmation | Local tool execution containment, permission elevation |
| `turn_observation_contract.rs` | Agent turn-taking lifecycle, observation feeds | Turn settling checkpoints, thinking state transitions |
| `computer_use_contract.rs` | Remote CUA computer control lease, screen capture | Exclusive lease acquisition, emergency takeover kill-switch |
| `transcript_manager_contract.rs` | SumTree transcript virtualized storage & mutations | Atomic message append, edit, tombstone soft-deletion |

---

## 5. Tier 1: Isolated Feature Coverage Tests (290 Test Cases)

Tier 1 validates the **isolated happy-path behavior** of all 58 features (5 test cases per feature = 290 test cases). Each test isolates the target capability and asserts correctness via observable outputs.

### 5.1 Telegram Desktop Parity Features (`TCG-N01` to `TCG-N26`)

#### TCG-N01: Auth & Registration
- **Upstream Anchor**: `Telegram/SourceFiles/intro/`, `api/api_authorizations.cpp`
- **Tests**:
  1. `TEST-T1-TCG01-001` [REST]: Phone number + valid SMS OTP -> Returns HTTP 200, valid session token, initial PTS = 0.
  2. `TEST-T1-TCG01-002` [REST]: WebAuthn / Passkey registration -> Creates FIDO2 credential, validates assertion signature, returns session token.
  3. `TEST-T1-TCG01-003` [REST]: 2FA Cloud Password login -> Submits Argon2id password hash, verifies against salt, issues authenticated token.
  4. `TEST-T1-TCG01-004` [REST]: Username registration -> Validates `@username` availability, reserves symbol, indexes user in Tantivy FTS.
  5. `TEST-T1-TCG01-005` [WS]: Concurrent login from second device -> Generates unique `device_id`, emits `UpdateNewSession` to existing sessions.

#### TCG-N02: Sessions & Security
- **Upstream Anchor**: `Telegram/SourceFiles/settings/settings_privacy_controllers.cpp`, `api/api_authorizations.cpp`
- **Tests**:
  1. `TEST-T1-TCG02-001` [REST]: Query active sessions -> Returns list of active device records with OS, IP, and last active timestamp.
  2. `TEST-T1-TCG02-002` [REST]: Terminate single remote session -> Revokes target session token, forces target WebSocket disconnect.
  3. `TEST-T1-TCG02-003` [REST]: Terminate all other sessions -> Invalidates all tokens except current, bumps `account_generation`.
  4. `TEST-T1-TCG02-004` [Headless GPUI]: Render Sessions settings view -> Correctly displays current device badge and remote device list.
  5. `TEST-T1-TCG02-005` [REST]: Change 2FA cloud password -> Re-hashes password with new salt, verifies subsequent login with new password.

#### TCG-N03: Dialogs & Folders
- **Upstream Anchor**: `Telegram/SourceFiles/dialogs/dialogs_inner_widget.cpp`, `data/data_folder.cpp`
- **Tests**:
  1. `TEST-T1-TCG03-001` [WS]: Fetch dialog list -> Returns ordered list of active conversations sorted by `last_message_date` descending.
  2. `TEST-T1-TCG03-002` [WS]: Create custom chat folder -> Persists folder with inclusion/exclusion filters (e.g. Non-muted, Channels).
  3. `TEST-T1-TCG03-003` [WS]: Pin dialog in folder -> Dialog pinned to index 0, emits `PinnedDialogsUpdated` event with new PTS.
  4. `TEST-T1-TCG03-004` [Headless GPUI]: Render 288px ChatSidebar with folder tabs -> Folder tabs switch viewable dialogs via `uniform_list`.
  5. `TEST-T1-TCG03-005` [WS]: Mute dialog notifications -> Suppresses notification sound and badges while retaining unread count.

#### TCG-N04: Messages & Bubbles
- **Upstream Anchor**: `Telegram/SourceFiles/history/history_item.cpp`, `history/view/history_view_message.cpp`
- **Tests**:
  1. `TEST-T1-TCG04-001` [WS]: Send plain text message -> Server assigns monotonic PTS, persists to SQLite WAL, broadcasts `NewMessage`.
  2. `TEST-T1-TCG04-002` [WS]: Send message with Markdown v2 entities -> Preserves bold, italic, strikethrough, and inline code formatting spans.
  3. `TEST-T1-TCG04-003` [WS]: Outgoing message delivery tick lifecycle -> State transitions: `Sending` -> `Sent` (one tick) -> `Read` (two ticks).
  4. `TEST-T1-TCG04-004` [Headless GPUI]: Message bubble grouping -> Sequential messages from same sender within 60s group into continuous bubble.
  5. `TEST-T1-TCG04-005` [Headless GPUI]: SumTree transcript rendering -> Renders 10,000 virtualized messages at 60 FPS without DOM overhead.

#### TCG-N05: Input Composer & IME
- **Upstream Anchor**: `Telegram/SourceFiles/chat_helpers/field_autocomplete.cpp`, `history/history_inner_widget.cpp`
- **Tests**:
  1. `TEST-T1-TCG05-001` [Headless GPUI]: Multi-line auto-growth -> Composer height expands dynamically from 36px to 140px on multi-line text.
  2. `TEST-T1-TCG05-002` [Headless GPUI]: IME composition safety -> Enter key during CJK composition commits candidate without sending message.
  3. `TEST-T1-TCG05-003` [Headless GPUI]: Draft auto-save -> Typing text auto-saves draft locally in SQLite after 300ms debounce.
  4. `TEST-T1-TCG05-004` [Headless GPUI]: Reply quote preview -> Selecting reply shows quoted message snippet above composer with dismiss button.
  5. `TEST-T1-TCG05-005` [Headless GPUI]: Mention autocomplete popup -> Typing `@` triggers popup list of matching chat participants.

#### TCG-N06: Media & Attachments
- **Upstream Anchor**: `Telegram/SourceFiles/history/view/media/history_view_document.cpp`, `storage/file_upload.cpp`
- **Tests**:
  1. `TEST-T1-TCG06-001` [REST]: Upload photo document -> Uploads JPEG, verifies SHA-256 CAS file, generates 320px WebP thumbnail.
  2. `TEST-T1-TCG06-002` [REST]: Upload video document -> Extracts duration, width, height, and first-frame thumbnail.
  3. `TEST-T1-TCG06-003` [REST]: Download attachment -> Downloads complete file, verifies byte-for-byte SHA-256 integrity.
  4. `TEST-T1-TCG06-004` [Headless GPUI]: Media grid bubble layout -> Multi-photo attachment renders in Telegram standard compact grid.
  5. `TEST-T1-TCG06-005` [WS]: Attach generic file document -> Displays filename, file size, and file extension icon.

#### TCG-N07: Voice Notes & Waveforms
- **Upstream Anchor**: `Telegram/SourceFiles/media/audio/media_audio_waveform.cpp`, `history/view/media/history_view_voice.cpp`
- **Tests**:
  1. `TEST-T1-TCG07-001` [REST]: Voice upload 5-bit waveform extraction -> Computes 100-sample 5-bit waveform vector (`Vec<u8>`).
  2. `TEST-T1-TCG07-002` [Headless GPUI]: Voice message recording gesture -> Hold mic button starts recording; slide up locks recording.
  3. `TEST-T1-TCG07-003` [Headless GPUI]: Voice recording cancellation -> Slide left discards audio recording with shredder visual effect.
  4. `TEST-T1-TCG07-004` [Headless GPUI]: Voice bubble playback -> Clicking play streams audio, animates waveform progress bars from 0% to 100%.
  5. `TEST-T1-TCG07-005` [REST]: Voice note STT transcription -> Automatic Speech-to-Text transcribes voice note into text.

#### TCG-N08: Audio Player
- **Upstream Anchor**: `Telegram/SourceFiles/media/player/media_player_instance.cpp`
- **Tests**:
  1. `TEST-T1-TCG08-001` [Headless GPUI]: Floating audio bar activation -> Playing audio spawns floating bar at top of window.
  2. `TEST-T1-TCG08-002` [Headless GPUI]: Audio playback speed toggle -> Switches speed between 1.0x, 1.5x, and 2.0x without pitch shift.
  3. `TEST-T1-TCG08-003` [Headless GPUI]: Audio scrubber dragging -> Dragging scrubber updates playback position deterministically.
  4. `TEST-T1-TCG08-004` [Headless GPUI]: Playlist sequential playback -> Ending track automatically advances to next audio track in chat.
  5. `TEST-T1-TCG08-005` [Headless GPUI]: Audio player close -> Dismissing player pauses playback and unmounts floating bar.

#### TCG-N09: Video Notes
- **Upstream Anchor**: `Telegram/SourceFiles/history/view/media/history_view_round_video.cpp`
- **Tests**:
  1. `TEST-T1-TCG09-001` [REST]: Upload video note -> Uploads MP4 video note, crops to 1:1 circular aspect ratio.
  2. `TEST-T1-TCG09-002` [Headless GPUI]: Round video bubble render -> Renders circular video frame with circular progress scrubber.
  3. `TEST-T1-TCG09-003` [Headless GPUI]: Video note audio toggle -> Click unmutes audio; second click pauses video.
  4. `TEST-T1-TCG09-004` [Headless GPUI]: Picture-in-Picture mode -> Scrolling chat past video note pops it into floating PIP mini-player.
  5. `TEST-T1-TCG09-005` [WS]: Video note play completion -> Emits read receipt upon reaching 100% video playback.

#### TCG-N10: Media Viewer Lightbox
- **Upstream Anchor**: `Telegram/SourceFiles/media/view/media_view_overlay_widget.cpp`
- **Tests**:
  1. `TEST-T1-TCG10-001` [Headless GPUI]: Open media lightbox -> Clicking image opens fullscreen modal with blurred backdrop.
  2. `TEST-T1-TCG10-002` [Headless GPUI]: Lightbox zoom & pan -> Scroll wheel zooms from 100% to 500%; drag pans zoomed image.
  3. `TEST-T1-TCG10-003` [Headless GPUI]: Carousel navigation -> Left/Right arrow keys navigate sequentially through chat media.
  4. `TEST-T1-TCG10-004` [Headless GPUI]: Lightbox video playback controls -> Play/pause, seek bar, volume slider, and fullscreen toggle.
  5. `TEST-T1-TCG10-005` [Headless GPUI]: Dismiss lightbox -> Pressing `Esc` or clicking background smoothly closes lightbox.

#### TCG-N11: Groups & Supergroups
- **Upstream Anchor**: `Telegram/SourceFiles/data/data_chat.cpp`, `boxes/manage_peer_box.cpp`
- **Tests**:
  1. `TEST-T1-TCG11-001` [WS]: Create basic group chat -> Creates group with initial member list, broadcasts invitations.
  2. `TEST-T1-TCG11-002` [WS]: Upgrade basic group to supergroup -> Migrates group to supergroup, unlocks 200,000 member capacity.
  3. `TEST-T1-TCG11-003` [WS]: Admin permission assignment -> Grants specific admin rights (Ban, Pin, Delete, Manage Topics).
  4. `TEST-T1-TCG11-004` [WS]: Restrict member permissions -> Restricts member from sending media/stickers for specified duration.
  5. `TEST-T1-TCG11-005` [Headless GPUI]: Member list display in Inspector -> Renders member roster with creator/admin badges and online indicators.

#### TCG-N12: Channels & Broadcasts
- **Upstream Anchor**: `Telegram/SourceFiles/data/data_channel.cpp`, `boxes/create_channel_box.cpp`
- **Tests**:
  1. `TEST-T1-TCG12-001` [WS]: Create broadcast channel -> Creates channel where only admins can post messages.
  2. `TEST-T1-TCG12-002` [WS]: Channel subscriber count broadcast -> Updating subscribers broadcasts accurate counter to subscribers.
  3. `TEST-T1-TCG12-003` [WS]: Channel post comment thread -> Posting in channel spawns linked discussion thread in linked group.
  4. `TEST-T1-TCG12-004` [WS]: Channel silent broadcast -> Posts broadcast without triggering push notification audio on subscribers.
  5. `TEST-T1-TCG12-005` [Headless GPUI]: Channel view mode -> Disables composer for non-admin subscribers, displaying "Mute/Unmute" footer.

#### TCG-N13: Forum Topics
- **Upstream Anchor**: `Telegram/SourceFiles/data/data_forum_topic.cpp`, `data/data_forum.cpp`
- **Tests**:
  1. `TEST-T1-TCG13-001` [WS]: Create forum topic -> Creates topic with custom title and emoji icon; returns `topic_id`.
  2. `TEST-T1-TCG13-002` [WS]: Post message inside topic -> Associates message with `topic_id`, updates topic PTS and unread counter.
  3. `TEST-T1-TCG13-003` [WS]: Pin topic inside forum -> Pinned topic stays at top of topic list regardless of message activity.
  4. `TEST-T1-TCG13-004` [WS]: Close topic -> Marks topic closed; prevents non-admins from posting new messages.
  5. `TEST-T1-TCG13-005` [Headless GPUI]: Topic navigation breadcrumbs -> Stage top bar displays parent group title and current topic pill.

#### TCG-N14: Reactions & Emoji
- **Upstream Anchor**: `Telegram/SourceFiles/history/view/reactions/history_view_reactions.cpp`
- **Tests**:
  1. `TEST-T1-TCG14-001` [WS]: Add emoji reaction to message -> Updates reaction counts, emits `ReactionUpdated` update event.
  2. `TEST-T1-TCG14-002` [WS]: Remove emoji reaction -> Decrements count, cleans up pill if count reaches zero.
  3. `TEST-T1-TCG14-003` [Headless GPUI]: Reaction picker popup -> Hovering message displays floating quick reaction bar (👍, ❤️, 🔥, etc.).
  4. `TEST-T1-TCG14-004` [Headless GPUI]: Reaction bubble render -> Message bubble footer renders reaction pills with count and sender highlight.
  5. `TEST-T1-TCG14-005` [WS]: Reaction list inspect -> Clicking reaction pill retrieves list of users who reacted.

#### TCG-N15: Stickers & GIFs
- **Upstream Anchor**: `Telegram/SourceFiles/chat_helpers/stickers_panel.cpp`
- **Tests**:
  1. `TEST-T1-TCG15-001` [REST]: Fetch installed sticker packs -> Returns list of sticker sets with WebP/TGS assets.
  2. `TEST-T1-TCG15-002` [WS]: Send sticker message -> Renders standalone borderless sticker asset in transcript.
  3. `TEST-T1-TCG15-003` [Headless GPUI]: Sticker picker panel -> Tabs across installed packs, provides instant preview on hover.
  4. `TEST-T1-TCG15-004` [Headless GPUI]: GIF search & playback -> Queries Tenor/local GIF store, renders looping animation in stage.
  5. `TEST-T1-TCG15-005` [WS]: Add sticker set to favorites -> Persists sticker pack to user's favorites list.

#### TCG-N16: Polls & Quizzes
- **Upstream Anchor**: `Telegram/SourceFiles/boxes/polls_create_box.cpp`, `data/data_poll.cpp`
- **Tests**:
  1. `TEST-T1-TCG16-001` [WS]: Create single-choice poll -> Creates poll with question and 2-10 options, broadcasts poll item.
  2. `TEST-T1-TCG16-002` [WS]: Cast vote in poll -> Records vote, recalculates option percentage bars, broadcasts updated totals.
  3. `TEST-T1-TCG16-003` [WS]: Create quiz with correct answer -> Records quiz, reveals explanation text upon user vote.
  4. `TEST-T1-TCG16-004` [WS]: Retract vote in poll -> Allows user to change vote if poll permits unvoting.
  5. `TEST-T1-TCG16-005` [WS]: Close poll -> Freezes final results, prevents further voting.

#### TCG-N17: Pinned Messages
- **Upstream Anchor**: `Telegram/SourceFiles/boxes/pin_message_box.cpp`, `history/view/controls/history_view_pinned_bar.cpp`
- **Tests**:
  1. `TEST-T1-TCG17-001` [WS]: Pin message in chat -> Updates chat pinned list, broadcasts pin notification.
  2. `TEST-T1-TCG17-002` [Headless GPUI]: Pinned message top bar -> Stage displays pinned snippet bar; clicking jumps to message.
  3. `TEST-T1-TCG17-003` [Headless GPUI]: Cycle multiple pinned messages -> Clicking pin bar sequentially jumps through multiple pins.
  4. `TEST-T1-TCG17-004` [WS]: Unpin message -> Removes message from pinned list, updates pin bar view.
  5. `TEST-T1-TCG17-005` [WS]: Unpin all messages -> Clears pinned bar completely in one atomic transaction.

#### TCG-N18: Scheduled & Silent Messages
- **Upstream Anchor**: `Telegram/SourceFiles/boxes/scheduled_messages_box.cpp`, `api/api_sending.cpp`
- **Tests**:
  1. `TEST-T1-TCG18-001` [WS]: Schedule message for future time -> Adds message to scheduled queue, returns scheduled ID.
  2. `TEST-T1-TCG18-002` [WS]: Scheduled message dispatcher worker -> When clock reaches target time, message dispatches automatically.
  3. `TEST-T1-TCG18-003` [Headless GPUI]: Scheduled messages management modal -> Lists pending scheduled items, supports reschedule and delete.
  4. `TEST-T1-TCG18-004` [WS]: Send silent message -> Dispatches message with `silent: true`, bypassing push alerts on recipient.
  5. `TEST-T1-TCG18-005` [WS]: Cancel scheduled message -> Removes message from scheduled queue before execution.

#### TCG-N19: Edit & Delete Messages
- **Upstream Anchor**: `Telegram/SourceFiles/boxes/delete_messages_box.cpp`, `history/history_item.cpp`
- **Tests**:
  1. `TEST-T1-TCG19-001` [WS]: Edit message text -> Updates text, sets `edited_at_ms = now()`, emits `EditMessage` event.
  2. `TEST-T1-TCG19-002` [Headless GPUI]: Edited timestamp indicator -> Renders "(edited)" label beside timestamp in bubble.
  3. `TEST-T1-TCG19-003` [WS]: Delete message for self only -> Hides message in actor's local SQLite store without server tombstone.
  4. `TEST-T1-TCG19-004` [WS]: Delete message for everyone -> Soft-deletes message on server, broadcasts tombstone with new PTS.
  5. `TEST-T1-TCG19-005` [WS]: Batch delete messages -> Deletes list of up to 100 message IDs in single atomic PTS transaction.

#### TCG-N20: PTS Sequence Sync Engine
- **Upstream Anchor**: `Telegram/SourceFiles/api/api_updates.cpp`, `storage/storage_account.cpp`
- **Tests**:
  1. `TEST-T1-TCG20-001` [WS]: Monotonic PTS counter generation -> Every mutation strictly increments user PTS by 1.
  2. `TEST-T1-TCG20-002` [WS]: Sequential update stream consumption -> Client applies sequential updates without triggering gap catch-up.
  3. `TEST-T1-TCG20-003` [WS]: `SyncGetDifference` query execution -> Client requests difference from PTS $K$; server returns delta slice.
  4. `TEST-T1-TCG20-004` [WS]: `SyncGetDifference` pagination -> Validates `has_more: true` when delta exceeds limit (e.g. 100 updates).
  5. `TEST-T1-TCG20-005` [WS]: Multi-device PTS consistency -> Actions on Device A propagate with matching PTS sequence to Device B.

#### TCG-N21: Full-Text Search
- **Upstream Anchor**: `Telegram/SourceFiles/dialogs/dialogs_search_from.cpp`, `storage/storage_databases.cpp`
- **Tests**:
  1. `TEST-T1-TCG21-001` [REST]: Tantivy global search query -> Searches across all conversations, returns matches in <5ms.
  2. `TEST-T1-TCG21-002` [REST]: In-chat scoped search -> Filters results strictly by `chat_id`.
  3. `TEST-T1-TCG21-003` [REST]: Faceted search by media type -> Filters messages with attachments (`has:photo`, `has:file`).
  4. `TEST-T1-TCG21-004` [Headless GPUI]: Search snippet highlighting -> Highlights matching keywords in results list using `<mark>`.
  5. `TEST-T1-TCG21-005` [Headless GPUI]: Search result activation -> Clicking search hit jumps directly to message in SumTree stage.

#### TCG-N22: Shared Media Archive
- **Upstream Anchor**: `Telegram/SourceFiles/overview/overview_layout.cpp`
- **Tests**:
  1. `TEST-T1-TCG22-001` [WS]: Query shared photos & videos -> Returns chronological thumbnail gallery of all chat media.
  2. `TEST-T1-TCG22-002` [WS]: Query shared files & documents -> Returns categorized document list with sizes and download buttons.
  3. `TEST-T1-TCG22-003` [WS]: Query shared links -> Extracts and lists OpenGraph link previews shared in chat.
  4. `TEST-T1-TCG22-004` [WS]: Query shared audio & voice notes -> Lists all voice recordings and music files in chat.
  5. `TEST-T1-TCG22-005` [Headless GPUI]: Shared media infinite scroll -> Smoothly paginates 50 items per batch upon reaching scroll boundary.

#### TCG-N23: Context Inspector Drawer
- **Upstream Anchor**: `Telegram/SourceFiles/info/info_wrap_widget.cpp`, `info/profile/info_profile_widget.cpp`
- **Tests**:
  1. `TEST-T1-TCG23-001` [Headless GPUI]: Toggle 320px Inspector drawer -> Clicking top bar info icon smoothly slides drawer open/close.
  2. `TEST-T1-TCG23-002` [Headless GPUI]: Profile section presentation -> Displays avatar, bio, phone/username, and notification settings.
  3. `TEST-T1-TCG23-003` [Headless GPUI]: Member tab management -> Displays member count, search bar, and admin action buttons.
  4. `TEST-T1-TCG23-004` [Headless GPUI]: Tab switching animation -> Seamlessly switches between Profile, Media, Files, and Links tabs.
  5. `TEST-T1-TCG23-005` [Headless GPUI]: Inspector responsiveness -> Resizing main window preserves 320px fixed drawer width.

#### TCG-N24: 1-on-1 VoIP Calls
- **Upstream Anchor**: `Telegram/SourceFiles/calls/calls_call.cpp`, `calls/calls_controller_webrtc.cpp`
- **Tests**:
  1. `TEST-T1-TCG24-001` [WS]: Initiate 1-on-1 call -> Sends `CallSignalFrame::Invite`, caller hears ringing tone, callee sees incoming call dialog.
  2. `TEST-T1-TCG24-002` [WS]: Accept call & SDP handshake -> Callee sends `Accept` with SDP answer; ICE candidate trickle connects call.
  3. `TEST-T1-TCG24-003` [WebRTC]: Two-way audio streaming -> Audio packets exchange over SRTP with <100ms round-trip latency.
  4. `TEST-T1-TCG24-004` [Headless GPUI]: Mute microphone toggle -> Mutes local mic track, displays muted icon to remote peer.
  5. `TEST-T1-TCG24-005` [WS]: Hangup call -> Sends `Hangup` signal, closes WebRTC peer connection, records call duration in chat.

#### TCG-N25: Group Voice & Video Chats
- **Upstream Anchor**: `Telegram/SourceFiles/calls/calls_group_call.cpp`
- **Tests**:
  1. `TEST-T1-TCG25-001` [WS]: Start group voice chat -> Initializes SFU bridge session in group, broadcasts active voice chat banner.
  2. `TEST-T1-TCG25-002` [WS]: Join group call as listener -> Connects to SFU downstream audio feed without publishing local track.
  3. `TEST-T1-TCG25-003` [WS]: Raise hand to speak -> Emits speak request; group admin approves and unmutes participant.
  4. `TEST-T1-TCG25-004` [Headless GPUI]: Active speaker indicator -> Glowing green avatar ring highlights currently speaking participant.
  5. `TEST-T1-TCG25-005` [WS]: End group call by admin -> SFU terminates bridge session, notifies all participants.

#### TCG-N26: Screen Sharing
- **Upstream Anchor**: `Telegram/SourceFiles/calls/calls_controller_webrtc.cpp`
- **Tests**:
  1. `TEST-T1-TCG26-001` [WebRTC]: Start screen sharing -> Captures local desktop display track and renegotiates SDP video stream.
  2. `TEST-T1-TCG26-002` [Headless GPUI]: Screen share view stage -> Renders remote screen video stream with resolution scaling.
  3. `TEST-T1-TCG26-003` [WebRTC]: Dynamic bitrate adaptation -> Adjusts video frame rate and bitrate dynamically on network congestion.
  4. `TEST-T1-TCG26-004` [Headless GPUI]: Fullscreen screen share toggle -> Double-clicking video expands stream to borderless fullscreen.
  5. `TEST-T1-TCG26-005` [WS]: Stop screen sharing -> Gracefully closes video track, falls back to audio-only VoIP layout.

### 5.2 Telegram Desktop Parity Features (`TCG-N27` to `TCG-N52`) & Inherited Capabilities (`CAP-01` to `CAP-06`)

#### TCG-N27: Bot Platform & Keyboards
- **Upstream Anchor**: `Telegram/SourceFiles/inline_bots/inline_bot_layout_item.cpp`, `chat_helpers/bot_keyboard.cpp`
- **Tests**:
  1. `TEST-T1-TCG27-001` [WS]: Render inline button keyboard -> Renders 1-8 column grid attached to bot message.
  2. `TEST-T1-TCG27-002` [WS]: Callback query dispatch -> Clicking inline button sends `CallbackQuery`, shows spinner feedback.
  3. `TEST-T1-TCG27-003` [WS]: URL button click -> Opens external web link in default browser with warning prompt.
  4. `TEST-T1-TCG27-004` [WS]: Bot command autocomplete -> Typing `/` in composer lists bot command descriptions.
  5. `TEST-T1-TCG27-005` [WS]: Bot reply keyboard markup -> Displays custom keyboard replacing standard text input.

#### TCG-N28: Mini Apps Sandbox
- **Upstream Anchor**: `Telegram/SourceFiles/inline_bots/bot_attach_web_view.cpp`, `boxes/web_view_box.cpp`
- **Tests**:
  1. `TEST-T1-TCG28-001` [Headless GPUI]: Launch Mini App modal -> Opens sandboxed web view with custom title bar and close button.
  2. `TEST-T1-TCG28-002` [Headless GPUI]: CSP policy enforcement -> Restricts iframe navigation to declared domains, blocks eval.
  3. `TEST-T1-TCG28-003` [WS]: Mini App bridge event dispatch -> App sends `web_app_open_link`, host dispatches native action.
  4. `TEST-T1-TCG28-004` [Headless GPUI]: Theme palette bridge -> Injects current Telegram color palette (light/dark) into Mini App.
  5. `TEST-T1-TCG28-005` [Headless GPUI]: Mini App close confirmation -> Warns user if unsaved changes exist before closing.

#### TCG-N29: Contacts & Address Book
- **Upstream Anchor**: `Telegram/SourceFiles/boxes/add_contact_box.cpp`, `boxes/contacts_box.cpp`
- **Tests**:
  1. `TEST-T1-TCG29-001` [REST]: Add contact -> Adds user by phone number or username; syncs contact to address book.
  2. `TEST-T1-TCG29-002` [REST]: Delete contact -> Removes user from contact list without deleting chat history.
  3. `TEST-T1-TCG29-003` [Headless GPUI]: Contacts modal list -> Displays alphabetical list with avatar, name, and online status.
  4. `TEST-T1-TCG29-004` [REST]: Search contacts -> Filters contact roster in real time by first name, last name, or username.
  5. `TEST-T1-TCG29-005` [WS]: Contact online status transition -> Broadcasts online/offline status updates to contacts.

#### TCG-N30: User Profile & Status
- **Upstream Anchor**: `Telegram/SourceFiles/info/profile/info_profile_values.cpp`, `data/data_user.cpp`
- **Tests**:
  1. `TEST-T1-TCG30-001` [REST]: Update profile display name -> Updates first name and last name, emits profile update event.
  2. `TEST-T1-TCG30-002` [REST]: Update profile bio -> Sets custom bio up to 70 characters; rejects oversized text.
  3. `TEST-T1-TCG30-003` [REST]: Upload profile avatar -> Uploads square image, crops to circle, generates thumbnail sizes.
  4. `TEST-T1-TCG30-004` [WS]: Update user presence status -> Toggles between "Online" and "Last seen recently/timestamp".
  5. `TEST-T1-TCG30-005` [Headless GPUI]: Render user profile in Drawer -> Accurately displays avatar, name, bio, and username.

#### TCG-N31: Chat Settings & Themes
- **Upstream Anchor**: `Telegram/SourceFiles/settings/settings_chat.cpp`, `ui/chat/chat_theme.cpp`
- **Tests**:
  1. `TEST-T1-TCG31-001` [Headless GPUI]: Switch Day Blue / Night theme -> Updates GPUI styling context palette dynamically.
  2. `TEST-T1-TCG31-002` [Headless GPUI]: Custom font size slider -> Adjusts bubble font size from 12pt to 20pt with immediate re-layout.
  3. `TEST-T1-TCG31-003` [Headless GPUI]: Chat background wallpaper -> Sets solid color or tiled image wallpaper behind transcript.
  4. `TEST-T1-TCG31-004` [Headless GPUI]: Message bubble corner radius -> Configures corner roundness (4px to 16px).
  5. `TEST-T1-TCG31-005` [Headless GPUI]: Auto-night mode toggle -> Automatically switches theme based on system appearance.

#### TCG-N32: Notifications & Badges
- **Upstream Anchor**: `Telegram/SourceFiles/platform/mac/tray_mac.mm`, `window/notifications_manager.cpp`
- **Tests**:
  1. `TEST-T1-TCG32-001` [Headless GPUI]: In-app banner alert -> Spawns sliding banner at top-right for incoming message.
  2. `TEST-T1-TCG32-002` [Headless GPUI]: Dock unread badge counter -> Updates macOS dock tile badge with aggregate unread count.
  3. `TEST-T1-TCG32-003` [Headless GPUI]: Notification sound alert -> Plays system notification audio on incoming message.
  4. `TEST-T1-TCG32-004` [WS]: Mute chat notifications for duration -> Sets `muted_until` timestamp, suppresses alerts.
  5. `TEST-T1-TCG32-005` [WS]: Unmute chat notifications -> Restores audio and banner notifications for chat.

#### TCG-N33: Privacy & Security
- **Upstream Anchor**: `Telegram/SourceFiles/settings/settings_privacy_controllers.cpp`
- **Tests**:
  1. `TEST-T1-TCG33-001` [REST]: Block user -> Adds user to blocklist, rejects calls and messages from blocked user.
  2. `TEST-T1-TCG33-002` [REST]: Unblock user -> Removes user from blocklist, restores messaging capability.
  3. `TEST-T1-TCG33-003` [REST]: Last seen visibility settings -> Configures visibility (Everybody, My Contacts, Nobody).
  4. `TEST-T1-TCG33-004` [REST]: Phone number visibility -> Hides phone number from non-contacts.
  5. `TEST-T1-TCG33-005` [Headless GPUI]: App passcode lock -> Requires PIN to unlock application after timeout.

#### TCG-N34: Storage & Local Cache
- **Upstream Anchor**: `Telegram/SourceFiles/settings/settings_advanced.cpp`, `storage/cache/storage_cache_cleaner.cpp`
- **Tests**:
  1. `TEST-T1-TCG34-001` [REST]: Query disk cache usage -> Calculates disk size for photos, videos, files, and database.
  2. `TEST-T1-TCG34-002` [REST]: Clear temporary media cache -> Removes cached media files without deleting chat history.
  3. `TEST-T1-TCG34-003` [REST]: Set maximum cache size limit -> Enforces LRU cache eviction when limit is reached.
  4. `TEST-T1-TCG34-004` [REST]: SQLite VACUUM maintenance -> Shrinks database file size and defragments storage.
  5. `TEST-T1-TCG34-005` [REST]: Clear cache for specific chat -> Deletes only media associated with specified chat ID.

#### TCG-N35: Proxy & Network Config
- **Upstream Anchor**: `Telegram/SourceFiles/boxes/connection_box.cpp`, `mtproto/connection.cpp`
- **Tests**:
  1. `TEST-T1-TCG35-001` [REST]: Add SOCKS5 proxy configuration -> Persists proxy settings (host, port, credentials).
  2. `TEST-T1-TCG35-002` [REST]: Proxy ping diagnostic test -> Tests proxy connectivity and reports latency in milliseconds.
  3. `TEST-T1-TCG35-003` [WS]: Enable proxy for WebSocket connection -> Routes WS traffic through configured proxy tunnel.
  4. `TEST-T1-TCG35-004` [WS]: Disable proxy fallback -> Reverts directly to primary network connection.
  5. `TEST-T1-TCG35-005` [Headless GPUI]: Connection status indicator -> Displays "Connecting...", "Updating...", or "Connected" icon.

#### TCG-N36: Draft Sync & Fencing
- **Upstream Anchor**: `Telegram/SourceFiles/data/data_drafts.cpp`
- **Tests**:
  1. `TEST-T1-TCG36-001` [WS]: Synchronize draft across devices -> Typing draft on Device A updates draft on Device B.
  2. `TEST-T1-TCG36-002` [Headless GPUI]: Sidebar draft preview -> Shows red "Draft:" prefix in chat list for uncommitted draft.
  3. `TEST-T1-TCG36-003` [WS]: Clear draft on send -> Sending message automatically deletes draft from server and client.
  4. `TEST-T1-TCG36-004` [WS]: Draft fencing on reply dismiss -> Clearing reply target clears reply association in draft.
  5. `TEST-T1-TCG36-005` [WS]: Conflict resolution on simultaneous draft edits -> Server resolves conflicts using latest timestamp.

#### TCG-N37: Archive & Filter Chips
- **Upstream Anchor**: `Telegram/SourceFiles/dialogs/dialogs_filters_bar.cpp`, `data/data_chat_filters.cpp`
- **Tests**:
  1. `TEST-T1-TCG37-001` [WS]: Archive chat -> Moves chat into "Archived Chats" folder, removes from main chat list.
  2. `TEST-T1-TCG37-002` [WS]: Unarchive chat -> Restores chat back to main chat list.
  3. `TEST-T1-TCG37-003` [WS]: Auto-unarchive on unmuted incoming message -> New message pulls chat out of archive if unmuted.
  4. `TEST-T1-TCG37-004` [Headless GPUI]: Filter chips navigation -> Clicking filter chip (e.g. "Unread", "Work") filters visible chats.
  5. `TEST-T1-TCG37-005` [Headless GPUI]: Reorder folder tabs -> Dragging folder tab changes persistent display order.

#### TCG-N38: Stories Strip
- **Upstream Anchor**: `Telegram/SourceFiles/data/data_stories.cpp`, `boxes/story_viewer_box.cpp`
- **Tests**:
  1. `TEST-T1-TCG38-001` [REST]: Post 24-hour story -> Uploads photo/video story with caption and privacy setting.
  2. `TEST-T1-TCG38-002` [Headless GPUI]: Stories top strip presentation -> Displays circular avatars with colored gradient rings.
  3. `TEST-T1-TCG38-003` [Headless GPUI]: Story viewer player -> Plays story with segmented progress bar and auto-advance.
  4. `TEST-T1-TCG38-004` [WS]: Story reaction -> Sends emoji reaction to story, notifying author.
  5. `TEST-T1-TCG38-005` [REST]: Story expiration cleanup -> Automatically archives story after 24 hours.

#### TCG-N39: Secret Chats & E2EE
- **Upstream Anchor**: `Telegram/SourceFiles/data/data_encrypted_service.cpp`, `storage/storage_encrypted.cpp`
- **Tests**:
  1. `TEST-T1-TCG39-001` [WS]: Initiate secret chat -> Performs X25519 Diffie-Hellman key exchange, derives shared secret.
  2. `TEST-T1-TCG39-002` [WS]: E2EE message encryption & decryption -> Encrypts message with XChaCha20-Poly1305, verifies decryption.
  3. `TEST-T1-TCG39-003` [WS]: Forwarding restriction -> Restricts copying and forwarding of secret chat messages.
  4. `TEST-T1-TCG39-004` [Headless GPUI]: Secret chat visual padlock indicator -> Displays green padlock and distinct encryption visual cues.
  5. `TEST-T1-TCG39-005` [WS]: Terminate secret chat -> Discards cryptographic keys, purges local session.

#### TCG-N40: Self-Destruct Timers
- **Upstream Anchor**: `Telegram/SourceFiles/boxes/auto_delete_settings_box.cpp`
- **Tests**:
  1. `TEST-T1-TCG40-001` [WS]: Set chat auto-delete timer -> Configures self-destruct duration (e.g. 24h, 7d).
  2. `TEST-T1-TCG40-002` [WS]: Message TTL expiration -> Messages past TTL are automatically deleted with tombstone update.
  3. `TEST-T1-TCG40-003` [Headless GPUI]: Self-destruct countdown timer indicator -> Shows subtle flame/timer icon on expiring message.
  4. `TEST-T1-TCG40-004` [WS]: Secret chat read-based self-destruct -> Timer starts only when recipient marks message as read.
  5. `TEST-T1-TCG40-005` [WS]: Disable auto-delete timer -> Restores persistent message retention for future messages.

#### TCG-N41: Location & Live Location
- **Upstream Anchor**: `Telegram/SourceFiles/history/view/media/history_view_location.cpp`
- **Tests**:
  1. `TEST-T1-TCG41-001` [WS]: Share static location -> Sends latitude and longitude, renders map preview pin.
  2. `TEST-T1-TCG41-002` [WS]: Start live location sharing -> Broadcasts periodic coordinate updates for declared duration.
  3. `TEST-T1-TCG41-003` [Headless GPUI]: Live location map view -> Renders moving avatar marker on map canvas.
  4. `TEST-T1-TCG41-004` [WS]: Stop live location sharing -> Ends broadcast, freezes final location marker.
  5. `TEST-T1-TCG41-005` [Headless GPUI]: Open location in external maps -> Clicking map pin opens location in Apple Maps / Google Maps.

#### TCG-N42: Web Link Previews
- **Upstream Anchor**: `Telegram/SourceFiles/history/view/media/history_view_web_page.cpp`, `data/data_web_page.cpp`
- **Tests**:
  1. `TEST-T1-TCG42-001` [REST]: OpenGraph metadata extraction -> Fetches URL, extracts title, description, and thumbnail.
  2. `TEST-T1-TCG42-002` [Headless GPUI]: Link preview card rendering -> Renders rich card above or below message bubble.
  3. `TEST-T1-TCG42-003` [Headless GPUI]: Link preview removal toggle -> User can dismiss preview before sending message.
  4. `TEST-T1-TCG42-004` [Headless GPUI]: Instant View (IV) button -> Clicking "Instant View" opens clean article reader modal.
  5. `TEST-T1-TCG42-005` [REST]: YouTube / video preview embed -> Embeds playable video player inside link preview card.

#### TCG-N43: Export Chat History
- **Upstream Anchor**: `Telegram/SourceFiles/export/export_manager.cpp`, `export/export_panel.cpp`
- **Tests**:
  1. `TEST-T1-TCG43-001` [REST]: Export chat to JSON format -> Generates structured JSON archive of all messages and metadata.
  2. `TEST-T1-TCG43-002` [REST]: Export chat to HTML format -> Generates standalone HTML viewer with CSS styles and images.
  3. `TEST-T1-TCG43-003` [REST]: Filter export by date range -> Exports only messages within specified start and end dates.
  4. `TEST-T1-TCG43-004` [REST]: Export media attachments -> Includes downloaded photos, voice notes, and documents in zip archive.
  5. `TEST-T1-TCG43-005` [Headless GPUI]: Export progress modal -> Displays accurate progress percentage and cancellation button.

#### TCG-N44: Language & Localization
- **Upstream Anchor**: `Telegram/SourceFiles/lang/lang_instance.cpp`, `lang/lang_cloud_manager.cpp`
- **Tests**:
  1. `TEST-T1-TCG44-001` [Headless GPUI]: Switch application language to Simplified Chinese -> UI labels update instantly.
  2. `TEST-T1-TCG44-002` [Headless GPUI]: Switch application language to English -> Restores standard English strings.
  3. `TEST-T1-TCG44-003` [REST]: Dynamic cloud string pack download -> Downloads updated translation pack from server.
  4. `TEST-T1-TCG44-004` [Headless GPUI]: Pluralization rules -> Correctly pluralizes strings (e.g. "1 message" vs "5 messages").
  5. `TEST-T1-TCG44-005` [Headless GPUI]: Missing string fallback -> Falls back to English default if translation key is missing.

#### TCG-N45: Keybindings & Shortcuts
- **Upstream Anchor**: `Telegram/SourceFiles/window/window_controller.cpp`
- **Tests**:
  1. `TEST-T1-TCG45-001` [Headless GPUI]: `Cmd+K` global search shortcut -> Focuses search input field immediately.
  2. `TEST-T1-TCG45-002` [Headless GPUI]: `Cmd+1` to `Cmd+9` folder switching -> Jumps directly to corresponding folder tab.
  3. `TEST-T1-TCG45-003` [Headless GPUI]: `Esc` key navigation -> Closes modal lightbox or unselects active conversation.
  4. `TEST-T1-TCG45-004` [Headless GPUI]: `Cmd+Up` edit last message -> Populates composer with last sent message for quick editing.
  5. `TEST-T1-TCG45-005` [Headless GPUI]: `Cmd+Shift+M` mute/unmute toggle -> Instantly toggles mute state of current chat.

#### TCG-N46: Custom Window Frame
- **Upstream Anchor**: `Telegram/SourceFiles/platform/mac/main_window_mac.mm`, `window/window_title.cpp`
- **Tests**:
  1. `TEST-T1-TCG46-001` [Headless GPUI]: macOS traffic lights integration -> Positions close, minimize, and zoom buttons at (16, 16).
  2. `TEST-T1-TCG46-002` [Headless GPUI]: Custom title bar drag area -> Window drag area moves entire application frame.
  3. `TEST-T1-TCG46-003` [Headless GPUI]: Double-click title bar maximize -> Maximizes or restores window geometry.
  4. `TEST-T1-TCG46-004` [Headless GPUI]: Fullscreen window transition -> Hides traffic lights, smoothly adapts layout to display bounds.
  5. `TEST-T1-TCG46-005` [Headless GPUI]: Minimum window size constraint -> Restricts window resize below 800x600 pixels.

#### TCG-N47: Drag-and-Drop & Clipboard
- **Upstream Anchor**: `Telegram/SourceFiles/chat_helpers/tabbed_selector.cpp`, `ui/chat/attach_send_files_way.cpp`
- **Tests**:
  1. `TEST-T1-TCG47-001` [Headless GPUI]: Drag file onto chat stage -> Displays drop overlay with "Send as photo" or "Send as file".
  2. `TEST-T1-TCG47-002` [Headless GPUI]: Paste image from clipboard -> Extracts PNG image from clipboard and mounts upload preview.
  3. `TEST-T1-TCG47-003` [Headless GPUI]: Copy formatted text to clipboard -> Copies text preserving bold, italic, and hyperlink tags.
  4. `TEST-T1-TCG47-004` [Headless GPUI]: Paste formatted HTML into composer -> Converts HTML into Telegram Markdown v2 entities.
  5. `TEST-T1-TCG47-005` [Headless GPUI]: Drag multiple files -> Batches up to 10 files into single grouped attachment prompt.

#### TCG-N48: Auto-Download Rules
- **Upstream Anchor**: `Telegram/SourceFiles/data/data_auto_download.cpp`, `settings/settings_advanced.cpp`
- **Tests**:
  1. `TEST-T1-TCG48-001` [REST]: Auto-download photos enabled -> Automatically downloads incoming photo thumbnail and full image.
  2. `TEST-T1-TCG48-002` [REST]: Auto-download video disabled -> Displays manual play/download button without auto-caching.
  3. `TEST-T1-TCG48-003` [REST]: Auto-download file size threshold -> Downloads files under 10MB, skips files over 10MB.
  4. `TEST-T1-TCG48-004` [REST]: Cellular vs Wi-Fi download profile -> Switches auto-download rules based on network interface.
  5. `TEST-T1-TCG48-005` [REST]: Voice note auto-preload -> Always preloads voice note audio files under 2MB.

#### TCG-N49: Multi-Account Switching
- **Upstream Anchor**: `Telegram/SourceFiles/main/main_account.cpp`, `main/main_domain.cpp`
- **Tests**:
  1. `TEST-T1-TCG49-001` [Headless GPUI]: NavRail multi-account avatar switcher -> Clicking profile avatar reveals account switch menu.
  2. `TEST-T1-TCG49-002` [Headless GPUI]: Add secondary account -> Authenticates second account, persists credentials in secure storage.
  3. `TEST-T1-TCG49-003` [WS]: Switch active account -> Bumps `account_generation`, isolates WebSocket connection and local DB.
  4. `TEST-T1-TCG49-004` [Headless GPUI]: Unread badge on inactive account -> Shows unread indicator dot on secondary account avatar.
  5. `TEST-T1-TCG49-005` [REST]: Log out secondary account -> Deletes account credentials while keeping primary account intact.

#### TCG-N50: Scope Generation Fencing
- **Upstream Anchor**: `Telegram/SourceFiles/main/main_session.cpp`, `storage/storage_account.cpp`
- **Tests**:
  1. `TEST-T1-TCG50-001` [REST]: Account logout generation bump -> Increments `account_scope_generation` by 1.
  2. `TEST-T1-TCG50-002` [WS]: Reject payload from stale generation -> Discards incoming WS message tagged with older generation.
  3. `TEST-T1-TCG50-003` [Headless GPUI]: Abort background tasks on generation change -> Cancels running background fetch workers.
  4. `TEST-T1-TCG50-004` [REST]: Invalidate cached media streams -> Media stream requests with older generation return HTTP 403.
  5. `TEST-T1-TCG50-005` [REST]: Atomic database scope fencing -> Local SQLite queries strictly filter by current `account_id`.

#### TCG-N51: Chunked Media Upload
- **Upstream Anchor**: `Telegram/SourceFiles/storage/file_upload.cpp`, `storage/storage_upload_manager.cpp`
- **Tests**:
  1. `TEST-T1-TCG51-001` [REST]: Initialize chunked upload -> `POST /api/v1/media/upload/init` returns `upload_id` and chunk size (512KB).
  2. `TEST-T1-TCG51-002` [REST]: Append valid chunk -> `PUT chunk 0` stores binary payload, acknowledges received chunk index.
  3. `TEST-T1-TCG51-003` [REST]: Upload multi-chunk file -> Uploads 10 consecutive chunks, assembling 5MB document.
  4. `TEST-T1-TCG51-004` [REST]: Complete upload & SHA-256 verification -> `POST complete` verifies overall checksum, moves file to CAS.
  5. `TEST-T1-TCG51-005` [REST]: Resume interrupted upload -> Queries uploaded chunks, resumes from last acknowledged chunk index.

#### TCG-N52: Range-Request Streaming
- **Upstream Anchor**: `Telegram/SourceFiles/data/data_streaming.cpp`, `storage/file_download.cpp`
- **Tests**:
  1. `TEST-T1-TCG52-001` [REST]: Stream audio range -> `GET /api/v1/media/stream/:id` with `Range: bytes=0-1023` returns HTTP 206.
  2. `TEST-T1-TCG52-002` [REST]: Stream video middle chunk -> `Range: bytes=524288-1048575` returns exact requested slice.
  3. `TEST-T1-TCG52-003` [REST]: Content-Range header validation -> Response includes valid `Content-Range: bytes 0-1023/5242880`.
  4. `TEST-T1-TCG52-004` [REST]: Audio scrubbing instant seek -> Client seeks audio to 3:45, fetches target byte range without latency.
  5. `TEST-T1-TCG52-005` [REST]: Out of bounds range request -> `Range: bytes=9999999-` beyond file size returns HTTP 416 Range Not Satisfiable.

#### CAP-01: AI Multi-Agent ACP
- **Upstream Anchor**: `source/host/src/runner/production_turn_agent_owner.rs`, `source/host/src/runner/subagent_runtime.rs`
- **Tests**:
  1. `TEST-T1-CAP01-001` [WS]: Turn-taking agent execution -> Agent receives user prompt, executes thinking loop, streams response.
  2. `TEST-T1-CAP01-002` [Headless GPUI]: `AgentThinkingBubble` collapsible display -> Renders collapsible reasoning chain above answer.
  3. `TEST-T1-CAP01-003` [WS]: Subagent tree spawning -> Parent agent spawns child worker agent, tracks execution progress.
  4. `TEST-T1-CAP01-004` [Headless GPUI]: Subagent lineage tree in drawer -> Renders execution tree showing parent/child relationships.
  5. `TEST-T1-CAP01-005` [WS]: Agent turn interruption -> User typing new prompt aborts in-flight generation, settles turn checkpoint.

#### CAP-02: Global Task Center & Cron
- **Upstream Anchor**: `source/host/src/runner/background_work.rs`, `source/host/src/automations/automation_schedule.rs`
- **Tests**:
  1. `TEST-T1-CAP02-001` [Headless GPUI]: NavRail "Tasks" workspace (`Cmd+2`) -> Opens 3-column task board (Waiting, In Progress, Done).
  2. `TEST-T1-CAP02-002` [WS]: 7-State task lifecycle FSM -> Task transitions: `Pending` -> `Running` -> `Completed`.
  3. `TEST-T1-CAP02-003` [WS]: 5-Field cron routine execution -> Cron engine evaluates `*/5 * * * *`, fires task precisely on schedule.
  4. `TEST-T1-CAP02-004` [Headless GPUI]: Task approval gate modal -> Displays amber approval banner for sensitive tools with "Allow" button.
  5. `TEST-T1-CAP02-005` [WS]: Task cancellation -> User abort moves running task to `Cancelled` state, cleans up worker container.

#### CAP-03: MCP & Plugin Ecosystem
- **Upstream Anchor**: `source/host/src/extensions/mcp/coordinator_relay.rs`, `source/box-exec-daemon/src/main.rs`
- **Tests**:
  1. `TEST-T1-CAP03-001` [REST]: Discover MCP server tools -> Connects to Stdio/SSE MCP server, parses `tools/list` schema.
  2. `TEST-T1-CAP03-002` [WS]: MCP tool execution relay -> Agent invokes MCP tool, coordinator executes tool and returns structured result.
  3. `TEST-T1-CAP03-003` [Headless GPUI]: `LocalToolPermissionDock` presentation -> Renders anchored banner above composer on tool request.
  4. `TEST-T1-CAP03-004` [REST]: Sandboxed tool container execution -> Executes tool inside `box-exec-daemon` with network restrictions.
  5. `TEST-T1-CAP03-005` [Headless GPUI]: Enable/disable MCP plugin -> Toggle in Extensions settings activates or deactivates tool server.

#### CAP-04: Remote Device Control
- **Upstream Anchor**: `source/host/src/runner/computer_use.rs`, `source/node-agent-coordinator/src/gateway/box_vnc_proxy.rs`
- **Tests**:
  1. `TEST-T1-CAP04-001` [WS]: Acquire exclusive computer control lease -> Grants `ComputerControlLease` with generation counter.
  2. `TEST-T1-CAP04-002` [Headless GPUI]: `ComputerWorkbench` live VNC display -> Renders remote desktop canvas with smooth cursor updates.
  3. `TEST-T1-CAP04-003` [WS]: Inject mouse click & keystrokes -> Translates agent CUA actions into remote OS mouse and keyboard events.
  4. `TEST-T1-CAP04-004` [Headless GPUI]: Emergency takeover kill-switch -> Pressing `Esc` or "Takeover" revokes agent lease in <10ms.
  5. `TEST-T1-CAP04-005` [WS]: Device enrollment pairing -> Scans QR code / submits pairing token, registers new remote workstation.

#### CAP-05: Full-Duplex Talk Mode
- **Upstream Anchor**: `source/host/tests/openclaw_audio_talk_contract.rs` (OCRM-AUDIO-O01..O12), `crates/gpui_telegram_media/`
- **Tests**:
  1. `TEST-T1-CAP05-001` [Headless GPUI]: Launch `TalkModeOrbModal` -> Clicking top-bar phone icon spawns 160px undulating energy orb.
  2. `TEST-T1-CAP05-002` [WebRTC]: Realtime voice streaming & VAD -> Voice Activity Detector detects speech start and silence boundaries.
  3. `TEST-T1-CAP05-003` [WebRTC]: 60ms barge-in interruption -> User speaking during AI response immediately flushes playback buffer.
  4. `TEST-T1-CAP05-004` [Headless GPUI]: Minimize Talk Mode to PIP capsule -> Collapses 160px orb into 48px floating PIP capsule.
  5. `TEST-T1-CAP05-005` [WS]: Talk mode session end & transcript commit -> Closes audio stream, commits complete conversation to SandAgentDb.

#### CAP-06: A2UI Interactive Cards
- **Upstream Anchor**: `source/node-agent-coordinator/src/tdrp/polls_todo.rs`, `native/mahayana-messaging/src/payment.rs`
- **Tests**:
  1. `TEST-T1-CAP06-001` [Headless GPUI]: Render A2UI declarative card -> Renders title, badge, key-value table, and progress bar.
  2. `TEST-T1-CAP06-002` [Headless GPUI]: Interactive Todo card toggle -> Clicking todo checkbox toggles status, syncs update to peer.
  3. `TEST-T1-CAP06-003` [Headless GPUI]: Stars / USD invoice card -> Renders invoice line items, total amount, and secure "Pay" button.
  4. `TEST-T1-CAP06-004` [Headless GPUI]: Inline button action execution -> Clicking card button submits structured payload to backend.
  5. `TEST-T1-CAP06-005` [WS]: Card state mutation sync -> Updating card fields broadcasts delta update, refreshing card UI in place.

## 6. Tier 2: Boundary, Corner & Hostile Network Cases Tests (290 Test Cases)

Tier 2 exercises the system under **hostile conditions, extreme boundary values, network chaos, and protocol violations** across all 58 features (5 boundary test cases per feature = 290 test cases).

### 6.1 Telegram Desktop Parity Boundary Tests (`TCG-N01` to `TCG-N26`)

#### TCG-N01: Auth & Registration Boundaries
- **Tests**:
  1. `TEST-T2-TCG01-001` [REST]: Phone number with invalid country code or 30-digit overflow -> Rejects with HTTP 400 `InvalidPhoneNumber`.
  2. `TEST-T2-TCG01-002` [REST]: 5 consecutive invalid SMS OTP attempts -> Locks account registration for 15 minutes with HTTP 429 `FloodWait`.
  3. `TEST-T2-TCG01-003` [REST]: Malformed WebAuthn assertion payload -> Rejects cryptographically invalid signature with HTTP 401.
  4. `TEST-T2-TCG01-004` [REST]: Username with forbidden symbols or length < 5 chars -> Rejects with HTTP 400 `UsernameInvalid`.
  5. `TEST-T2-TCG01-005` [WS]: Concurrent login attempt with expired session token -> Rejects WS handshake with close code 4401 `Unauthorized`.

#### TCG-N02: Sessions & Security Boundaries
- **Tests**:
  1. `TEST-T2-TCG02-001` [REST]: Terminate session ID belonging to another user -> Rejects with HTTP 403 `ForbiddenScope`.
  2. `TEST-T2-TCG02-002` [WS]: Simultaneous termination of current session while active message in flight -> Drops connection gracefully, rejects message commit.
  3. `TEST-T2-TCG02-003` [REST]: 2FA cloud password submission with 10,000-character payload -> Rejects payload before Argon2id processing, avoiding DoS.
  4. `TEST-T2-TCG02-004` [WS]: Heartbeat timeout (60s without ping) -> Server closes inactive WebSocket, marks device offline.
  5. `TEST-T2-TCG02-005` [REST]: Attempting session termination with expired credentials -> Returns HTTP 401 with immediate local cache wipe.

#### TCG-N03: Dialogs & Folders Boundaries
- **Tests**:
  1. `TEST-T2-TCG03-001` [WS]: Pinning more than 5 dialogs in non-premium account -> Rejects 6th pin with `PinnedDialogsLimitExceeded`.
  2. `TEST-T2-TCG03-002` [WS]: Creating folder with 1,000 chat filters -> Validates folder filter capacity limit (max 100 chats).
  3. `TEST-T2-TCG03-003` [WS]: Chat deleted while user is dragging folder tab -> Resolves gracefully without UI panic or orphaned references.
  4. `TEST-T2-TCG03-004` [Headless GPUI]: Render 500 pinned chats -> SumTree list clamps layout bounds, maintains 60 FPS scrolling.
  5. `TEST-T2-TCG03-005` [WS]: Rapid folder tab reordering during active sync -> Resolves folder order deterministically using LWW (Last-Write-Wins).

#### TCG-N04: Messages & Bubbles Boundaries
- **Tests**:
  1. `TEST-T2-TCG04-001` [WS]: Message text exceeding 4,096 characters -> Rejects with HTTP 400 / WS Error `MessageTooLong`.
  2. `TEST-T2-TCG04-002` [WS]: Duplicate message submission with identical `client_nonce` -> Returns existing message item without duplicate DB write.
  3. `TEST-T2-TCG04-003` [Headless GPUI]: Message containing unbalanced Markdown v2 tags -> Falls back to raw plain text rendering without parser panic.
  4. `TEST-T2-TCG04-004` [Headless GPUI]: Message containing bidirectional text & RTL override characters -> Renders text safely without crashing GPUI text shaper.
  5. `TEST-T2-TCG04-005` [WS]: Message with zero-width spaces or null bytes -> Sanitizes null characters while preserving valid UTF-8 sequences.

#### TCG-N05: Input Composer & IME Boundaries
- **Tests**:
  1. `TEST-T2-TCG05-001` [Headless GPUI]: IME composition abort (user switches focus mid-composition) -> Discards uncommitted pre-edit string cleanly.
  2. `TEST-T2-TCG05-002` [Headless GPUI]: Pasting 10MB text blob into composer -> Truncates to 4,096 characters with user notification toast.
  3. `TEST-T2-TCG05-003` [Headless GPUI]: Rapid `Cmd+Enter` hammering (100 times in 1s) -> Rate limiter transmits single message, suppresses bursts.
  4. `TEST-T2-TCG05-004` [Headless GPUI]: Quoting a message that gets deleted while composer is open -> Clears quote banner gracefully upon tombstone event.
  5. `TEST-T2-TCG05-005` [Headless GPUI]: Autocomplete list opened while chat switches -> Closes autocomplete popup immediately, preventing cross-chat leaks.

#### TCG-N06: Media & Attachments Boundaries
- **Tests**:
  1. `TEST-T2-TCG06-001` [REST]: Upload file exceeding 2GB limit -> Rejects at chunk 0 with HTTP 413 `PayloadTooLarge`.
  2. `TEST-T2-TCG06-002` [REST]: Corrupted image upload (invalid JPEG header) -> Rejects thumbnail extraction, classifies as generic file.
  3. `TEST-T2-TCG06-003` [REST]: Network disconnect during 99% of file upload -> Resume endpoint continues from last uploaded chunk without re-uploading.
  4. `TEST-T2-TCG06-004` [REST]: Upload complete with SHA-256 hash mismatch -> Discards staged file, returns HTTP 422 `ChecksumMismatch`.
  5. `TEST-T2-TCG06-005` [Headless GPUI]: Render 100 attached photos in single album -> Clamps to 10 photos per album grid as per Telegram spec.

#### TCG-N07: Voice Notes & Waveforms Boundaries
- **Tests**:
  1. `TEST-T2-TCG07-001` [REST]: Voice note audio with 0 samples (silence / empty buffer) -> Rejects with `SandTranscribeEmptyAudioError`.
  2. `TEST-T2-TCG07-002` [REST]: Voice note exceeding 60 minutes duration -> Clamps recording or rejects with `AudioDurationLimitExceeded`.
  3. `TEST-T2-TCG07-003` [Headless GPUI]: Audio device disconnected mid-recording -> Saves recorded fragment up to disconnect, marks voice note complete.
  4. `TEST-T2-TCG07-004` [REST]: Corrupted Opus stream -> 5-bit waveform encoder returns zeroed waveform without panic.
  5. `TEST-T2-TCG07-005` [Headless GPUI]: Playback seeking beyond voice note duration -> Clamps playback position to EOF, stops audio engine.

#### TCG-N08: Audio Player Boundaries
- **Tests**:
  1. `TEST-T2-TCG08-001` [Headless GPUI]: Rapid play/pause toggling (50 toggles/sec) -> State machine remains consistent, avoids audio buffer deadlock.
  2. `TEST-T2-TCG08-002` [REST]: Audio file streaming returning HTTP 500 midway -> Player stops, displays retry button without crashing UI.
  3. `TEST-T2-TCG08-003` [Headless GPUI]: Dragging scrubber while file is still buffering -> Buffering indicator overlays scrubber, resumes upon chunk arrival.
  4. `TEST-T2-TCG08-004` [Headless GPUI]: Playlist containing deleted audio message -> Skips deleted track gracefully, plays next available track.
  5. `TEST-T2-TCG08-005` [Headless GPUI]: Switching playback speed while at 99.9% track progress -> Correctly reaches end-of-track event.

#### TCG-N09: Video Notes Boundaries
- **Tests**:
  1. `TEST-T2-TCG09-001` [REST]: Video note upload with non-square aspect ratio -> Automatically center-crops video to 1:1 circular ratio.
  2. `TEST-T2-TCG09-002` [REST]: Video note exceeding 60s limit -> Rejects video note with `VideoNoteDurationLimitExceeded`.
  3. `TEST-T2-TCG09-003` [Headless GPUI]: PIP window dragged outside screen bounds -> Clamps PIP coordinates within visible display rectangle.
  4. `TEST-T2-TCG09-004` [REST]: Video note playback under zero network bandwidth -> Displays buffering spinner, preserves downloaded frames.
  5. `TEST-T2-TCG09-005` [Headless GPUI]: Rapid unmount of round video while playing -> Properly terminates hardware video decoder instance.

#### TCG-N10: Media Viewer Lightbox Boundaries
- **Tests**:
  1. `TEST-T2-TCG10-001` [Headless GPUI]: Zoom scale boundary (attempt zoom to 10,000%) -> Clamps zoom to maximum 500%.
  2. `TEST-T2-TCG10-002` [Headless GPUI]: Navigate past last media item in chat -> Carousel stops at boundary, does not wrap or crash.
  3. `TEST-T2-TCG10-003` [Headless GPUI]: Window resized while lightbox is open -> Recalculates lightbox center and preserves aspect ratio.
  4. `TEST-T2-TCG10-004` [Headless GPUI]: Media item deleted by sender while open in lightbox -> Displays "Media no longer available" banner.
  5. `TEST-T2-TCG10-005` [Headless GPUI]: Rapid `Esc` pressing during lightbox opening transition -> Smoothly closes without layout artifacts.

#### TCG-N11: Groups & Supergroups Boundaries
- **Tests**:
  1. `TEST-T2-TCG11-001` [WS]: Adding user who has restricted group invitations -> Rejects invitation with `UserPrivacyRestricted`.
  2. `TEST-T2-TCG11-002` [WS]: Promoting member to admin without `CanPromoteAdmins` right -> Rejects action with `AdminPrivilegeRequired`.
  3. `TEST-T2-TCG11-003` [WS]: Basic group exceeding 200 members -> Triggers mandatory supergroup migration flow.
  4. `TEST-T2-TCG11-004` [WS]: Simultaneous ban of same member by two different admins -> Idempotent handling; second action succeeds as no-op.
  5. `TEST-T2-TCG11-005` [WS]: Banned user attempting to send message -> Rejects message with `ChatAdminRequired` / `UserBanned`.

#### TCG-N12: Channels & Broadcasts Boundaries
- **Tests**:
  1. `TEST-T2-TCG12-001` [WS]: Non-admin subscriber attempting to post message -> Rejects with `ChatWriteForbidden`.
  2. `TEST-T2-TCG12-002` [WS]: Channel username change to an existing reserved username -> Rejects with `UsernameOccupied`.
  3. `TEST-T2-TCG12-003` [WS]: Broadcast channel deleted with 100,000 subscribers -> Server cascades soft-deletion, broadcasts channel closure.
  4. `TEST-T2-TCG12-004` [Headless GPUI]: Channel with 1,000,000 subscribers -> Formats subscriber counter as "1M subscribers".
  5. `TEST-T2-TCG12-005` [WS]: Rapid message broadcast (1,000 posts/sec) -> Throttles channel broadcasts per rate-limiting spec.

#### TCG-N13: Forum Topics Boundaries
- **Tests**:
  1. `TEST-T2-TCG13-001` [WS]: Create topic in group where `forum_enabled = false` -> Rejects with `ForumDisabled`.
  2. `TEST-T2-TCG13-002` [WS]: Post message to deleted `topic_id` -> Rejects with `TopicNotFound`, message diverted to "General".
  3. `TEST-T2-TCG13-003` [WS]: Topic title with 500 characters -> Truncates or rejects title exceeding 128 characters.
  4. `TEST-T2-TCG13-004` [Headless GPUI]: Render forum with 10,000 active topics -> Virtualized topic list handles rendering with <16ms frame budget.
  5. `TEST-T2-TCG13-005` [WS]: Pinning 6th topic when limit is 5 -> Rejects with `PinnedTopicsLimitExceeded`.

#### TCG-N14: Reactions & Emoji Boundaries
- **Tests**:
  1. `TEST-T2-TCG14-001` [WS]: Adding reaction not in chat allowed reactions list -> Rejects with `ReactionForbidden`.
  2. `TEST-T2-TCG14-002` [WS]: User adding 20 reactions to single message -> Clamps to maximum 3 reactions per message per user.
  3. `TEST-T2-TCG14-003` [WS]: Rapid reaction clicking (10 toggles in 1s) -> Server debounces reaction count updates.
  4. `TEST-T2-TCG14-004` [WS]: Reaction added to deleted message -> Discards reaction update without PTS increment.
  5. `TEST-T2-TCG14-005` [Headless GPUI]: Render message with 50 distinct reaction types -> Layout wraps reaction pills gracefully.

#### TCG-N15: Stickers & GIFs Boundaries
- **Tests**:
  1. `TEST-T2-TCG15-001` [REST]: Corrupted TGS / Lottie sticker file -> Displays static fallback sticker icon, avoids animation engine crash.
  2. `TEST-T2-TCG15-002` [REST]: Install sticker pack containing 500 stickers -> Rejects pack exceeding 200 sticker limit.
  3. `TEST-T2-TCG15-003` [Headless GPUI]: GIF search under zero network connectivity -> Displays "Network unavailable" state with retry.
  4. `TEST-T2-TCG15-004` [Headless GPUI]: Play 50 animated stickers simultaneously in viewport -> Throttles offscreen animations, maintains 60 FPS.
  5. `TEST-T2-TCG15-005` [WS]: Send sticker from uninstalled pack -> Successfully delivers sticker using cached document ID.

#### TCG-N16: Polls & Quizzes Boundaries
- **Tests**:
  1. `TEST-T2-TCG16-001` [WS]: Create poll with only 1 option -> Rejects with `PollOptionsCountInvalid` (minimum 2 options).
  2. `TEST-T2-TCG16-002` [WS]: Create poll with 11 options -> Rejects with `PollOptionsCountInvalid` (maximum 10 options).
  3. `TEST-T2-TCG16-003` [WS]: Vote in closed poll -> Rejects vote submission with `PollClosed`.
  4. `TEST-T2-TCG16-004` [WS]: Quiz option text exceeding 100 characters -> Rejects with `OptionTextTooLong`.
  5. `TEST-T2-TCG16-005` [WS]: Concurrent votes from 1,000 users on same poll -> Atomic SQL update guarantees total vote count accuracy.

#### TCG-N17: Pinned Messages Boundaries
- **Tests**:
  1. `TEST-T2-TCG17-001` [WS]: Pin already pinned message -> Re-orders pinned list without creating duplicate entry.
  2. `TEST-T2-TCG17-002` [WS]: Jump to pinned message that was deleted -> Displays toast "Message was deleted", removes pin from bar.
  3. `TEST-T2-TCG17-003` [WS]: Pinning message without `CanPinMessages` admin permission -> Rejects with `ChatAdminRequired`.
  4. `TEST-T2-TCG17-004` [Headless GPUI]: Rapid cycling through 100 pinned messages -> SumTree stage jumps to exact scroll offsets without drift.
  5. `TEST-T2-TCG17-005` [WS]: Pinning message in secret chat -> Rejects pin action (unsupported in ephemeral secret chats).

#### TCG-N18: Scheduled & Silent Messages Boundaries
- **Tests**:
  1. `TEST-T2-TCG18-001` [WS]: Schedule message for past timestamp -> Dispatches message immediately as regular message.
  2. `TEST-T2-TCG18-002` [WS]: Schedule message more than 365 days in future -> Rejects with `ScheduleDateTooLate`.
  3. `TEST-T2-TCG18-003` [WS]: Server clock drift during scheduled dispatch -> Cron engine triggers within ±100ms tolerance.
  4. `TEST-T2-TCG18-004` [WS]: Recipient deletes chat while scheduled message is pending -> Discards scheduled message upon dispatch.
  5. `TEST-T2-TCG18-005` [WS]: Queueing 101 scheduled messages in single chat -> Rejects with `ScheduledMessagesLimitExceeded` (max 100).

#### TCG-N19: Edit & Delete Messages Boundaries
- **Tests**:
  1. `TEST-T2-TCG19-001` [WS]: Edit message after 48-hour edit window has expired -> Rejects with `MessageEditExpired`.
  2. `TEST-T2-TCG19-002` [WS]: Edit message from another user without admin rights -> Rejects with `MessageAuthorRequired`.
  3. `TEST-T2-TCG19-003` [WS]: Delete message for everyone after 48-hour window in basic group -> Permits delete (supergroup rules apply).
  4. `TEST-T2-TCG19-004` [WS]: Edit message text to empty string -> Rejects edit; message must be deleted instead.
  5. `TEST-T2-TCG19-005` [WS]: Batch delete containing already deleted message IDs -> Idempotent; skips non-existent IDs cleanly.

#### TCG-N20: PTS Sequence Sync Engine Boundaries
- **Tests**:
  1. `TEST-T2-TCG20-001` [WS]: Client receives PTS gap ($\text{PTS}_{\text{recv}} = \text{PTS}_{\text{local}} + 5$) -> Client halts local commit, invokes `SyncGetDifference`.
  2. `TEST-T2-TCG20-002` [WS]: `SyncGetDifference` executed during server SQLite lock contention -> Retries with exponential backoff up to 3 times.
  3. `TEST-T2-TCG20-003` [WS]: Server PTS table truncated (data recovery scenario) -> Detects invalid state, triggers full snapshot reload.
  4. `TEST-T2-TCG20-004` [WS]: 1,000 concurrent update writes across 10 threads -> Monotonic sequence counter strictly sequential with zero duplicates.
  5. `TEST-T2-TCG20-005` [WS]: Client receives stale duplicate update ($\text{PTS}_{\text{recv}} \le \text{PTS}_{\text{local}}$) -> Silently drops update as duplicate.

#### TCG-N21: Full-Text Search Boundaries
- **Tests**:
  1. `TEST-T2-TCG21-001` [REST]: Search query containing Tantivy syntax errors (`AND OR NOT `) -> Sanitizes query, executes safe literal search.
  2. `TEST-T2-TCG21-002` [REST]: Search query with 1,000 terms -> Clamps query to first 32 terms, executes search in <10ms.
  3. `TEST-T2-TCG21-003` [REST]: Search on freshly inserted message before index commit -> Query falls back to SQLite uncommitted message buffer.
  4. `TEST-T2-TCG21-004` [REST]: Search with CJK strings without spaces -> Tantivy CJK tokenizer splits characters, matches substrings.
  5. `TEST-T2-TCG21-005` [REST]: Tantivy RAM index exhaustion -> Flushes segments to disk segment directory, releases RAM.

#### TCG-N22: Shared Media Archive Boundaries
- **Tests**:
  1. `TEST-T2-TCG22-001` [WS]: Query shared media in newly created chat with 0 media -> Returns empty array with `has_more: false`.
  2. `TEST-T2-TCG22-002` [WS]: Shared media query with invalid pagination cursor -> Falls back to returning latest media slice.
  3. `TEST-T2-TCG22-003` [WS]: Shared media item deleted while browsing tab -> Removes item dynamically from active gallery view.
  4. `TEST-T2-TCG22-004` [Headless GPUI]: Scroll shared links tab with 1,000 URLs -> SumTree list layout recycles link preview cards smoothly.
  5. `TEST-T2-TCG22-005` [WS]: Query shared media in restricted group -> Returns HTTP 403 `ChatAdminRequired` if history hidden.

#### TCG-N23: Context Inspector Drawer Boundaries
- **Tests**:
  1. `TEST-T2-TCG23-001` [Headless GPUI]: Rapid opening/closing of Inspector (20 times/sec) -> Drawer animation state machine avoids lockup.
  2. `TEST-T2-TCG23-002` [Headless GPUI]: Main window resized to minimum 800px width with drawer open -> Clamps chat stage width to minimum 200px.
  3. `TEST-T2-TCG23-003` [Headless GPUI]: User avatar failed to load in Inspector -> Renders fallback initials avatar with solid background.
  4. `TEST-T2-TCG23-004` [Headless GPUI]: Active chat changed while Inspector tab is on "Files" -> Re-queries and displays files for new chat.
  5. `TEST-T2-TCG23-005` [Headless GPUI]: Long user bio (4,000 chars) in Inspector -> Auto-wraps bio in scrollable container without clipping.

#### TCG-N24: 1-on-1 VoIP Calls Boundaries
- **Tests**:
  1. `TEST-T2-TCG24-001` [WS]: Incoming call received while already in an active call -> Emits `CallSignalFrame::Busy` to second caller.
  2. `TEST-T2-TCG24-002` [WebRTC]: Callee does not answer within 60s -> Ringing times out, records "Missed call" in conversation.
  3. `TEST-T2-TCG24-003` [WebRTC]: Peer connection drops during call (packet loss 100%) -> State transitions to `Reconnecting`; hangs up after 15s.
  4. `TEST-T2-TCG24-004` [WebRTC]: SDP offer media section limits & video rejection (`OCRM-AUDIO-O10`) -> Strips video tracks if audio-only call.
  5. `TEST-T2-TCG24-005` [WS]: Signaling gap rejection (`OCRM-AUDIO-O02`) -> Drops out-of-order signaling frames with mismatched `signalSeq`.

#### TCG-N25: Group Voice & Video Chats Boundaries
- **Tests**:
  1. `TEST-T2-TCG25-001` [WS]: 50 participants join group call simultaneously -> SFU scales audio mix streams without saturating CPU.
  2. `TEST-T2-TCG25-002` [WS]: Non-admin attempts to mute another participant -> Rejects with `GroupCallAdminRequired`.
  3. `TEST-T2-TCG25-003` [WebRTC]: Audio packet loss of 30% on participant stream -> WebRTC PLC (Packet Loss Concealment) maintains speech intelligibility.
  4. `TEST-T2-TCG25-004` [Headless GPUI]: 10 participants speaking simultaneously -> Clamps active speaker glowing rings to top 3 loudest speakers.
  5. `TEST-T2-TCG25-005` [WS]: SFU process killed abruptly -> Clients detect disconnection, attempt automatic reconnect to backup SFU node.

#### TCG-N26: Screen Sharing Boundaries
- **Tests**:
  1. `TEST-T2-TCG26-001` [WebRTC]: Screen resolution changes mid-stream (e.g. 4K to 1080p) -> Adapts video encoder parameters without tearing.
  2. `TEST-T2-TCG26-002` [WebRTC]: OS screen recording permission denied -> Emits permission error dialog, gracefully cancels video track.
  3. `TEST-T2-TCG26-003` [WebRTC]: Bandwidth collapses to <50 kbps -> Drops frame rate to 1 FPS, preserves audio track priority.
  4. `TEST-T2-TCG26-004` [Headless GPUI]: Remote screen sharing window closed abruptly -> Transitions view back to grid of voice avatars.
  5. `TEST-T2-TCG26-005` [WebRTC]: Simultaneous screen share request from second participant -> Places request in queue or replaces stream based on admin setting.

### 6.2 Telegram Desktop Parity Boundary Tests (`TCG-N27` to `TCG-N52`) & Inherited Capabilities (`CAP-01` to `CAP-06`)

#### TCG-N27: Bot Platform & Keyboards Boundaries
- **Tests**:
  1. `TEST-T2-TCG27-001` [WS]: Malformed callback data (>64 bytes limit) -> Rejects inline button payload with `CallbackDataTooLong`.
  2. `TEST-T2-TCG27-002` [WS]: Inline keyboard grid with 100 rows -> Clamps keyboard rendering to maximum 50 rows per message.
  3. `TEST-T2-TCG27-003` [WS]: Bot crashes during callback query processing -> Client clears button spinner after 5s timeout, displays error toast.
  4. `TEST-T2-TCG27-004` [WS]: Clicking callback button on deleted message -> Rejects query with `MessageNotFound`.
  5. `TEST-T2-TCG27-005` [WS]: Multiple rapid clicks on single-use callback button -> Only first click dispatches; subsequent clicks ignored.

#### TCG-N28: Mini Apps Sandbox Boundaries
- **Tests**:
  1. `TEST-T2-TCG28-001` [Headless GPUI]: Mini App attempts to access local filesystem or cookie jars -> Sandbox security fence blocks call.
  2. `TEST-T2-TCG28-002` [Headless GPUI]: Mini App executes infinite loop / memory bomb -> Terminates WebView process after memory threshold (500MB).
  3. `TEST-T2-TCG28-003` [Headless GPUI]: Mini App attempts popup window opening -> Intercepts window open, enforces modal container isolation.
  4. `TEST-T2-TCG28-004` [WS]: Invalid signature on Mini App initData -> Backend rejects auth verification with HTTP 401 `InvalidInitData`.
  5. `TEST-T2-TCG28-005` [Headless GPUI]: Mini App backgrounded during payment checkout -> Suspends execution state without losing payment token.

#### TCG-N29: Contacts & Address Book Boundaries
- **Tests**:
  1. `TEST-T2-TCG29-001` [REST]: Importing contact list with 50,000 entries -> Batches import in chunks of 500, avoiding database locks.
  2. `TEST-T2-TCG29-002` [REST]: Contact phone number with letters / symbols -> Strips non-numeric characters before canonical phone normalization.
  3. `TEST-T2-TCG29-003` [REST]: Adding self as contact -> Rejects self-contact addition or maps to "Saved Messages".
  4. `TEST-T2-TCG29-004` [REST]: Contact name containing null bytes or script tags -> Escapes HTML / sanitizes name before persistence.
  5. `TEST-T2-TCG29-005` [WS]: Simultaneous contact deletion on two devices -> Idempotent handling; second delete succeeds as no-op.

#### TCG-N30: User Profile & Status Boundaries
- **Tests**:
  1. `TEST-T2-TCG30-001` [REST]: Profile name containing only whitespace -> Rejects with `NameCannotBeEmpty`.
  2. `TEST-T2-TCG30-002` [REST]: Avatar upload with 100MB TIFF file -> Rejects unsupported format and oversized payload.
  3. `TEST-T2-TCG30-003` [WS]: User sends typing action in 100 chats simultaneously -> Rate limiter suppresses flood, permits max 5 actions/sec.
  4. `TEST-T2-TCG30-004` [WS]: Offline status set while device is actively sending messages -> Automatically switches status to online.
  5. `TEST-T2-TCG30-005` [REST]: Bio containing emoji sequences exceeding 70-character limit -> Calculates grapheme cluster count, clamps strictly.

#### TCG-N31: Chat Settings & Themes Boundaries
- **Tests**:
  1. `TEST-T2-TCG31-001` [Headless GPUI]: Loading corrupted custom theme file -> Detects parsing error, falls back to default Day Blue theme.
  2. `TEST-T2-TCG31-002` [Headless GPUI]: Setting background wallpaper to 50MB image -> Resizes wallpaper to display resolution, clamps memory footprint.
  3. `TEST-T2-TCG31-003` [Headless GPUI]: Extreme font size scaling (e.g. 50pt) -> Clamps font size to maximum 24pt, preserving layout integrity.
  4. `TEST-T2-TCG31-004` [Headless GPUI]: System appearance switches rapidly (light/dark/light in 100ms) -> Debounces theme switch without visual glitch.
  5. `TEST-T2-TCG31-005` [Headless GPUI]: Switching theme while modal dialog is animating -> Preserves active modal state, applies new colors.

#### TCG-N32: Notifications & Badges Boundaries
- **Tests**:
  1. `TEST-T2-TCG32-001` [Headless GPUI]: Aggregate unread count reaches 100,000 -> Dock badge displays "999+" without overflowing icon.
  2. `TEST-T2-TCG32-002` [WS]: 500 incoming messages in 1 second -> Groups notifications into single digest banner: "500 new messages".
  3. `TEST-T2-TCG32-003` [Headless GPUI]: Notification audio file missing or corrupted -> Silently falls back to default system beep.
  4. `TEST-T2-TCG32-004` [WS]: Message marked read on Device B -> Immediately dismisses corresponding notification banner on Device A.
  5. `TEST-T2-TCG32-005` [WS]: Mute chat until timestamp in year 2038 -> Preserves 64-bit timestamp, avoids 32-bit integer overflow.

#### TCG-N33: Privacy & Security Boundaries
- **Tests**:
  1. `TEST-T2-TCG33-001` [REST]: Blocklist capacity limit (adding 10,000 blocked users) -> Handles large blocklists with indexed pagination.
  2. `TEST-T2-TCG33-002` [REST]: Blocked user attempts to add current user to a channel -> Server enforces privacy filter, rejects action.
  3. `TEST-T2-TCG33-003` [Headless GPUI]: App passcode entered incorrectly 5 times -> Enforces progressive delay (30s lockout).
  4. `TEST-T2-TCG33-004` [REST]: Setting "Nobody" can see phone number -> Validates phone number is omitted from all user summaries.
  5. `TEST-T2-TCG33-005` [REST]: Removing 2FA cloud password while sessions are active -> Requires re-authenticating with current password first.

#### TCG-N34: Storage & Local Cache Boundaries
- **Tests**:
  1. `TEST-T2-TCG34-001` [REST]: Disk space reaches 99% capacity during media download -> Halts download, displays "Disk space low" alert.
  2. `TEST-T2-TCG34-002` [REST]: SQLite database file corruption on startup -> Attempts recovery via WAL rollback; falls back to re-syncing from server.
  3. `TEST-T2-TCG34-003` [REST]: Eviction of locked media file -> Retries deletion on next cache sweep without panicking.
  4. `TEST-T2-TCG34-004` [REST]: Simultaneous VACUUM while active write transaction runs -> SQLite WAL mode coordinates concurrency without failure.
  5. `TEST-T2-TCG34-005` [REST]: Clearing cache while audio file is currently streaming -> Delays deletion of actively played file until playback stops.

#### TCG-N35: Proxy & Network Config Boundaries
- **Tests**:
  1. `TEST-T2-TCG35-001` [REST]: SOCKS5 proxy server unresponsive (black hole) -> Connection times out in 5s, attempts direct connection fallback.
  2. `TEST-T2-TCG35-002` [REST]: Proxy authentication failure (bad credentials) -> Prompts user for updated credentials without hanging connection.
  3. `TEST-T2-TCG35-003` [WS]: Proxy disconnects during active WebSocket sync -> Client switches to next configured proxy or direct route.
  4. `TEST-T2-TCG35-004` [REST]: Proxy host configured as invalid IP/domain -> Validates configuration syntax, marks proxy status "Unreachable".
  5. `TEST-T2-TCG35-005` [REST]: High-latency proxy (RTT > 2000ms) -> Adjusts keep-alive timeout windows dynamically.

#### TCG-N36: Draft Sync & Fencing Boundaries
- **Tests**:
  1. `TEST-T2-TCG36-001` [WS]: Rapid draft sync between 2 devices (typing simultaneously) -> Resolves via monotonic timestamp without clobbering.
  2. `TEST-T2-TCG36-002` [WS]: Draft length exceeding 4,096 characters -> Clamps draft text to maximum message length.
  3. `TEST-T2-TCG36-003` [WS]: Draft synchronized for chat that gets deleted -> Discards draft silently upon chat deletion event.
  4. `TEST-T2-TCG36-004` [Headless GPUI]: Application crashed while draft was unsaved -> Local SQLite uncommitted draft recovered on restart.
  5. `TEST-T2-TCG36-005` [WS]: Draft containing attachment reference that failed upload -> Strips invalid attachment pointer, preserves draft text.

#### TCG-N37: Archive & Filter Chips Boundaries
- **Tests**:
  1. `TEST-T2-TCG37-001` [WS]: Archiving 5,000 chats in batch -> Completes archive migration in single transaction without freezing UI.
  2. `TEST-T2-TCG37-002` [Headless GPUI]: Creating 20 custom folder tabs -> Tabs bar supports horizontal mouse wheel scrolling with fade edges.
  3. `TEST-T2-TCG37-003` [WS]: Chat filter rule matching 0 chats -> Displays clean "No chats found in this folder" empty state.
  4. `TEST-T2-TCG37-004` [WS]: Muted chat in archive receiving 1,000 messages -> Remains in archive, does not pop out to main list.
  5. `TEST-T2-TCG37-005` [Headless GPUI]: Deleting currently active folder tab -> Falls back smoothly to "All Chats" default folder tab.

#### TCG-N38: Stories Strip Boundaries
- **Tests**:
  1. `TEST-T2-TCG38-001` [REST]: Uploading story with 1GB video -> Rejects story payload exceeding 50MB video limit.
  2. `TEST-T2-TCG38-002` [REST]: Viewing story from blocked user -> Filter excludes stories from blocked contacts.
  3. `TEST-T2-TCG38-003` [Headless GPUI]: Story viewer player running on exhausted battery -> Throttles background animations.
  4. `TEST-T2-TCG38-004` [WS]: Story deleted by author while peer is halfway through playback -> Closes player gracefully, advances to next contact.
  5. `TEST-T2-TCG38-005` [REST]: Story caption containing 5,000 characters -> Truncates caption to maximum 200 characters.

#### TCG-N39: Secret Chats & E2EE Boundaries
- **Tests**:
  1. `TEST-T2-TCG39-001` [WS]: Secret chat message received on secondary device -> Secret chats are device-specific; secondary device rejects message.
  2. `TEST-T2-TCG39-002` [WS]: Cryptographic key exchange timeout (peer offline for 30 days) -> Expires pending handshake, marks chat aborted.
  3. `TEST-T2-TCG39-003` [WS]: Replay attack with captured encrypted packet -> Sequence counter verification rejects duplicate ciphertext.
  4. `TEST-T2-TCG39-004` [Headless GPUI]: Screen capture attempt on secret chat window -> macOS window flags set to conceal content from screenshots.
  5. `TEST-T2-TCG39-005` [WS]: Secret chat peer re-installs app -> Previous keys invalidated; prompt notifies user "Encryption keys changed".

#### TCG-N40: Self-Destruct Timers Boundaries
- **Tests**:
  1. `TEST-T2-TCG40-001` [WS]: Message TTL set to 1 second -> Message renders, starts timer, deletes from client and server at exactly 1.0s.
  2. `TEST-T2-TCG40-002` [WS]: Recipient device offline when self-destruct timer expires -> Deletes message immediately upon client reconnect.
  3. `TEST-T2-TCG40-003` [WS]: Attempting to edit message with active self-destruct timer -> Rejects edit if remaining TTL < 5s.
  4. `TEST-T2-TCG40-004` [WS]: Self-destruct message screenshot taken by peer -> Emits service notification "Alice took a screenshot!".
  5. `TEST-T2-TCG40-005` [WS]: Media download in progress when self-destruct fires -> Aborts download, scrubs temporary file fragments from disk.

#### TCG-N41: Location & Live Location Boundaries
- **Tests**:
  1. `TEST-T2-TCG41-001` [WS]: Location coordinates with NaN or extreme values (lat > 90) -> Rejects invalid coordinates with `LocationInvalid`.
  2. `TEST-T2-TCG41-002` [WS]: Live location update stream under zero GPS signal -> Sends last known accuracy indicator, pauses updates.
  3. `TEST-T2-TCG41-003` [Headless GPUI]: Map tile loading failure (HTTP 502) -> Displays clean vector grid fallback with location pin.
  4. `TEST-T2-TCG41-004` [WS]: Live location duration expires (e.g. 8 hours) -> Automatically transitions status to static historical pin.
  5. `TEST-T2-TCG41-005` [WS]: 50 participants sharing live location simultaneously in group -> Aggregates proximity markers to prevent map lag.

#### TCG-N42: Web Link Previews Boundaries
- **Tests**:
  1. `TEST-T2-TCG42-001` [REST]: Target URL server returns HTTP 404/500 -> Skips preview generation, displays plain text link.
  2. `TEST-T2-TCG42-002` [REST]: SSRF attack attempt (link to `http://169.254.169.254` or `http://127.0.0.1`) -> Blocks link preview crawler.
  3. `TEST-T2-TCG42-003` [REST]: Web page with 50MB HTML document -> Stream parser reads first 64KB for OpenGraph tags, terminates connection.
  4. `TEST-T2-TCG42-004` [REST]: URL redirection loop (301 redirect loop) -> Aborts crawler after 3 hops, prevents infinite loop.
  5. `TEST-T2-TCG42-005` [Headless GPUI]: Link preview image with invalid dimensions (0x0 or 10000x1) -> Sanitizes dimensions, avoids layout overflow.

#### TCG-N43: Export Chat History Boundaries
- **Tests**:
  1. `TEST-T2-TCG43-001` [REST]: Exporting chat with 500,000 messages and 100GB media -> Streams output archive chunk by chunk without memory exhaustion.
  2. `TEST-T2-TCG43-002` [REST]: Disk full during export archiving -> Cleans up partial archive, displays descriptive error toast.
  3. `TEST-T2-TCG43-003` [REST]: User cancels export midway -> Aborts worker thread, cleans up staging directory immediately.
  4. `TEST-T2-TCG43-004` [REST]: Exporting secret chat history -> Rejects export request; secret chats cannot be exported.
  5. `TEST-T2-TCG43-005` [REST]: Export chat containing corrupted attachment -> Logs warning in manifest, continues exporting remaining messages.

#### TCG-N44: Language & Localization Boundaries
- **Tests**:
  1. `TEST-T2-TCG44-001` [Headless GPUI]: Translation string with mismatched format placeholders (`%1$@`) -> Falls back to raw template without panicking.
  2. `TEST-T2-TCG44-002` [Headless GPUI]: Switching to RTL language (Arabic/Hebrew) -> Flips UI layout mirror, aligns bubbles to opposite edges.
  3. `TEST-T2-TCG44-003` [REST]: Cloud language pack download interrupted by network drop -> Retains existing language pack, retries in background.
  4. `TEST-T2-TCG44-004` [Headless GPUI]: Overly long localized string (German translations 3x English length) -> Text wraps or elides without clipping buttons.
  5. `TEST-T2-TCG44-005` [Headless GPUI]: Unsupported system locale detected -> Defaults to `en-US` cleanly.

#### TCG-N45: Keybindings & Shortcuts Boundaries
- **Tests**:
  1. `TEST-T2-TCG45-001` [Headless GPUI]: Shortcut pressed while modal alert dialog is open -> Blocks navigation shortcuts, allows only `Esc`/`Enter`.
  2. `TEST-T2-TCG45-002` [Headless GPUI]: User types `Cmd+K` repeatedly (50 times in 1s) -> Search drawer remains stably open without flickering.
  3. `TEST-T2-TCG45-003` [Headless GPUI]: Non-standard keyboard layout (Dvorak/AZERTY) -> Translates virtual keycodes according to active layout.
  4. `TEST-T2-TCG45-004` [Headless GPUI]: `Cmd+Up` edit shortcut with empty chat history -> No-op; composer remains unaffected.
  5. `TEST-T2-TCG45-005` [Headless GPUI]: Key combination conflicts with system global shortcuts -> Respects macOS OS-level shortcut interception.

#### TCG-N46: Custom Window Frame Boundaries
- **Tests**:
  1. `TEST-T2-TCG46-001` [Headless GPUI]: Window resized below 800x600 -> Enforces minimum window constraint, clamps resize bounds.
  2. `TEST-T2-TCG46-002` [Headless GPUI]: Window dragged between monitors with different scale factors (1x and 2x Retina) -> Re-scales glyph atlas seamlessly.
  3. `TEST-T2-TCG46-003` [Headless GPUI]: Metal rendering context lost (GPU sleep/wake) -> Re-creates pipeline surfaces without crashing application.
  4. `TEST-T2-TCG46-004` [Headless GPUI]: Fullscreen toggle while modal dialog is open -> Repositions modal in center of fullscreen viewport.
  5. `TEST-T2-TCG46-005` [Headless GPUI]: Window title bar double-clicked in non-resizable state -> Ignores maximize action safely.

#### TCG-N47: Drag-and-Drop & Clipboard Boundaries
- **Tests**:
  1. `TEST-T2-TCG47-001` [Headless GPUI]: Dropping non-file object (e.g. invalid URI) -> Discards drag event, removes drop overlay.
  2. `TEST-T2-TCG47-002` [Headless GPUI]: Pasting clipboard data with corrupted image headers -> Detects invalid image bytes, handles as raw data.
  3. `TEST-T2-TCG47-003` [Headless GPUI]: Dropping 1,000 files simultaneously -> Clamps batch to first 10 files, alerts user of limit.
  4. `TEST-T2-TCG47-004` [Headless GPUI]: Dragging file over disabled composer (e.g. read-only channel) -> Drop cursor shows prohibited indicator.
  5. `TEST-T2-TCG47-005` [Headless GPUI]: Clipboard copy of 50MB formatted message transcript -> Copies safely without freezing main event thread.

#### TCG-N48: Auto-Download Rules Boundaries
- **Tests**:
  1. `TEST-T2-TCG48-001` [REST]: Network interface switches from Wi-Fi to Cellular while 500MB download active -> Suspends download per rules.
  2. `TEST-T2-TCG48-002` [REST]: Low data mode enabled in OS settings -> Overrides app rules, halts all auto-downloads.
  3. `TEST-T2-TCG48-003` [REST]: Incoming media from untrusted non-contact -> Enforces restrictive download thresholds.
  4. `TEST-T2-TCG48-004` [REST]: Media file size header altered by proxy -> Re-evaluates actual size during streaming, halts if threshold exceeded.
  5. `TEST-T2-TCG48-005` [REST]: Auto-download queue reaches 100 pending files -> Throttles queue, processes 3 concurrent downloads max.

#### TCG-N49: Multi-Account Switching Boundaries
- **Tests**:
  1. `TEST-T2-TCG49-001` [WS]: Switching to Account B while Account A is receiving 1,000 messages -> Isolates state; Account B shows only its own updates.
  2. `TEST-T2-TCG49-002` [REST]: Adding 4th account when maximum account limit is 3 -> Rejects addition with `MaxAccountsReached`.
  3. `TEST-T2-TCG49-003` [Headless GPUI]: Rapid account switching (10 times in 2s) -> Scope generation fencing cleanly cancels pending queries.
  4. `TEST-T2-TCG49-004` [REST]: Account credentials revoked remotely while inactive -> Shows exclamation badge, prompts re-login upon selection.
  5. `TEST-T2-TCG49-005` [REST]: Switching accounts with uncommitted local draft -> Persists Account A draft in Account A DB, loads Account B draft.

#### TCG-N50: Scope Generation Fencing Boundaries
- **Tests**:
  1. `TEST-T2-TCG50-001` [WS]: Stale WebSocket frame arrives with `generation = 1` while active generation is 2 -> Frame rejected without processing.
  2. `TEST-T2-TCG50-002` [REST]: Background image download completes for old generation -> Discards downloaded payload, does not write to cache.
  3. `TEST-T2-TCG50-003` [WS]: Multiple concurrent account generation bumps -> Monotonic generation counter increments atomically.
  4. `TEST-T2-TCG50-004` [Headless GPUI]: Active modal dialog open during generation bump -> Closes modal immediately, clears sensitive inputs.
  5. `TEST-T2-TCG50-005` [REST]: SQLite query executed with mismatched generation -> Database pool wrapper traps error, prevents data leak.

#### TCG-N51: Chunked Media Upload Boundaries
- **Tests**:
  1. `TEST-T2-TCG51-001` [REST]: Uploading chunk out of order (chunk 5 before chunk 4) -> Rejects out-of-order chunk with HTTP 400 `ChunkOutOfOrder`.
  2. `TEST-T2-TCG51-002` [REST]: Chunk size smaller than 512KB for non-final chunk -> Rejects with HTTP 400 `InvalidChunkSize`.
  3. `TEST-T2-TCG51-003` [REST]: Staged upload session idle for >24 hours -> Staging cleaner purges partial chunks, returns HTTP 404 on resume.
  4. `TEST-T2-TCG51-004` [REST]: Complete upload called when missing chunk 3 of 10 -> Returns HTTP 400 with list of missing chunk indexes.
  5. `TEST-T2-TCG51-005` [REST]: Re-uploading already acknowledged chunk (network retry) -> Idempotent acknowledgment; returns HTTP 200.

#### TCG-N52: Range-Request Streaming Boundaries
- **Tests**:
  1. `TEST-T2-TCG52-001` [REST]: Malformed Range header (`Range: bytes=abc-def`) -> Ignores invalid range, returns complete file with HTTP 200.
  2. `TEST-T2-TCG52-002` [REST]: Reversed range request (`Range: bytes=500-200`) -> Returns HTTP 416 `Range Not Satisfiable`.
  3. `TEST-T2-TCG52-003` [REST]: Range request specifying start beyond EOF -> Returns HTTP 416 with valid `Content-Range: bytes */size`.
  4. `TEST-T2-TCG52-004` [REST]: Multiple ranges header (`Range: bytes=0-100, 200-300`) -> Returns multipart range response with valid boundaries.
  5. `TEST-T2-TCG52-005` [REST]: Client aborts TCP stream mid-chunk -> Server terminates file handle read stream cleanly without leaking FD.

#### CAP-01: AI Multi-Agent ACP Boundaries
- **Tests**:
  1. `TEST-T2-CAP01-001` [WS]: Agent enters recursive execution loop -> ACP runner enforces maximum turn depth (10 turns), aborts loop.
  2. `TEST-T2-CAP01-002` [WS]: LLM provider returns HTTP 429 Rate Limit -> ACP runner waits for exponential backoff, notifies user of delay.
  3. `TEST-T2-CAP01-003` [WS]: Subagent worker process killed abruptly -> Parent agent catches exit signal, records failure in lineage tree.
  4. `TEST-T2-CAP01-004` [Headless GPUI]: Streaming 100,000 tokens of reasoning text -> SumTree virtualized thinking bubble scrolls smoothly.
  5. `TEST-T2-CAP01-005` [WS]: Stale turn checkpoint mutation rejection -> Rejects mutations to previously settled agent turn records.

#### CAP-02: Global Task Center & Cron Boundaries
- **Tests**:
  1. `TEST-T2-CAP02-001` [WS]: Task in `WaitingAuth` times out after 10 minutes -> Automatically transitions task to `Cancelled` state.
  2. `TEST-T2-CAP02-002` [WS]: Worker connection lost while task is `Running` -> Transitions to `WaitingConnection`; recovers upon reconnect.
  3. `TEST-T2-CAP02-003` [WS]: Invalid cron expression syntax (`* * * * * * *`) -> Rejects cron registration with descriptive parse error.
  4. `TEST-T2-CAP02-004` [WS]: Cron job execution overlaps with previous running instance -> Skips overlapping run or queues per policy.
  5. `TEST-T2-CAP02-005` [WS]: Terminal task state mutation invariant -> Rejects status change attempts on `Completed` or `Cancelled` tasks.

#### CAP-03: MCP & Plugin Ecosystem Boundaries
- **Tests**:
  1. `TEST-T2-CAP03-001` [REST]: MCP tool call with malformed JSON arguments -> Rejects tool execution, returns structured schema error to agent.
  2. `TEST-T2-CAP03-002` [WS]: MCP tool hangs for >30 seconds -> Tool runner aborts container execution with timeout error.
  3. `TEST-T2-CAP03-003` [Headless GPUI]: User denies tool permission in `LocalToolPermissionDock` -> Returns permission denied error to agent.
  4. `TEST-T2-CAP03-004` [REST]: MCP server crashes unexpectedly -> Marks server status "Error", disables tool routing.
  5. `TEST-T2-CAP03-005` [REST]: Tool returns 100MB output payload -> Clamps tool output to 1MB, truncates with warning indicator.

#### CAP-04: Remote Device Control Boundaries
- **Tests**:
  1. `TEST-T2-CAP04-001` [WS]: Agent attempts CUA action without active control lease -> Rejects action with `ControlLeaseRequired`.
  2. `TEST-T2-CAP04-002` [WS]: Control lease expires while agent action is executing -> Aborts action immediately, releases control lock.
  3. `TEST-T2-CAP04-003` [Headless GPUI]: Emergency `Esc` takeover during high-frequency mouse drag -> Instantly restores local human control (<10ms).
  4. `TEST-T2-CAP04-004` [WebRTC]: VNC proxy frame stream drops to 0 FPS -> Displays "Reconnecting to remote desktop..." warning banner.
  5. `TEST-T2-CAP04-005` [WS]: Competing lease acquisition from two different agents -> Grants lease strictly to first agent, rejects second.

#### CAP-05: Full-Duplex Talk Mode Boundaries (OCRM Oracles)
- **Tests**:
  1. `TEST-T2-CAP05-001` [WebRTC]: Realtime audio queue overflow limits (`OCRM-AUDIO-O05`) -> Drops oldest chunks when queue exceeds 320 chunks / 1 MiB.
  2. `TEST-T2-CAP05-002` [WebRTC]: SharedArrayBuffer atomic 32-bit close fence semantics (`OCRM-AUDIO-O06`) -> Enforces atomic close barrier.
  3. `TEST-T2-CAP05-003` [WebRTC]: Exact speech loop prevention (`OCRM-AUDIO-O07`) -> Detects acoustic feedback loop, suppresses echo.
  4. `TEST-T2-CAP05-004` [WebRTC]: Edge TTS CJK dominant voice fallback (`OCRM-AUDIO-O12`) -> Switches voice model when CJK text exceeds 30%.
  5. `TEST-T2-CAP05-005` [WebRTC]: Turn-context tracker silent turn pruning & TTL (`OCRM-AUDIO-O09`) -> Prunes silent turns older than TTL window.

#### CAP-06: A2UI Interactive Cards Boundaries
- **Tests**:
  1. `TEST-T2-CAP06-001` [Headless GPUI]: Card schema containing unknown component type -> Renders known components, displays warning badge for unknown.
  2. `TEST-T2-CAP06-002` [Headless GPUI]: Rapid double-click on payment checkout button -> Debounces button, initiates single payment transaction.
  3. `TEST-T2-CAP06-003` [WS]: Card state update for non-existent card ID -> Rejects update without modifying local state.
  4. `TEST-T2-CAP06-004` [Headless GPUI]: Card text with 10,000 characters -> Auto-collapses long content behind "Show more" expander.
  5. `TEST-T2-CAP06-005` [WS]: Out-of-order card delta updates -> Applies updates strictly according to card version number.

## 7. Tier 3: Cross-Feature Interaction Tests (58 Test Cases)

Tier 3 exercises **multi-feature pairwise and tri-feature integrations** across the entire application ecosystem, validating that complex concurrent workflows, shared state machines, and synchronized UI projections do not suffer from deadlocks, race conditions, or state contamination.

| Test ID | Primary Features Combined | Cross-Feature Interaction Scenario | Execution Channel | Authoritative Verification Criteria |
|---|---|---|---|---|
| `TEST-T3-XF-001` | `TCG-N01` + `TCG-N50` | Auth logout while background file upload is active | REST + WS | Generation bump invalidates session; upload worker halts cleanly, discarding orphaned chunks. |
| `TEST-T3-XF-002` | `TCG-N02` + `TCG-N49` | Remote session termination of secondary account | REST + Headless GPUI | Terminating Account B remotely from Account A updates Account B avatar badge to unauthorized exclamation. |
| `TEST-T3-XF-003` | `TCG-N03` + `TCG-N37` | Moving pinned chat into custom folder filter | WS + Headless GPUI | Pinned chat retains pin priority within newly assigned folder tab; ChatSidebar updates immediately. |
| `TEST-T3-XF-004` | `TCG-N04` + `TCG-N21` | Message sent and edited during active Tantivy search query | WS + REST | Tantivy async indexer commits edit within 500ms; subsequent search query returns updated snippet text. |
| `TEST-T3-XF-005` | `TCG-N05` + `TCG-N36` | Composer draft synchronized across 2 devices during active typing | WS + Headless GPUI | Device A typing updates draft on Device B within 300ms; Device B sending clears draft on Device A. |
| `TEST-T3-XF-006` | `TCG-N06` + `TCG-N51` | 50MB video attachment uploaded via 512KB chunk pipeline | REST + Headless GPUI | UI displays accurate progress bar (0% -> 100%); verifies SHA-256 CAS file and spawns preview bubble. |
| `TEST-T3-XF-007` | `TCG-N07` + `TCG-N08` | Voice note recorded, sent, and played in floating audio bar | Headless GPUI + REST | Hold-to-record generates 5-bit waveform; clicking play unrolls floating audio player with scrubber. |
| `TEST-T3-XF-008` | `TCG-N09` + `TCG-N10` | Video note played and expanded into lightbox viewer | Headless GPUI | Circular round video plays in bubble; double-clicking opens lightbox maintaining circular mask. |
| `TEST-T3-XF-009` | `TCG-N11` + `TCG-N13` | Supergroup converted to forum with topic hierarchy | WS + Headless GPUI | Enables forum topics; creates "General" and custom topics; ChatSidebar reflects topic tree. |
| `TEST-T3-XF-010` | `TCG-N12` + `TCG-N18` | Scheduled silent broadcast in public channel | WS + Cron | Message dispatches at scheduled epoch without audio push alerts; subscriber counters update. |
| `TEST-T3-XF-011` | `TCG-N14` + `TCG-N20` | Reaction added while client catches up missing PTS gap | WS | Client detects gap, executes `SyncGetDifference`; merges reaction update into historical message state. |
| `TEST-T3-XF-012` | `TCG-N15` + `TCG-N47` | Dragging animated sticker asset onto chat stage | Headless GPUI | Drag-and-drop detects sticker MIME, sends as borderless animated sticker without photo compression. |
| `TEST-T3-XF-013` | `TCG-N16` + `CAP-06` | Interactive poll embedded inside A2UI task card | Headless GPUI + WS | Voting in card updates both poll percentage bars and task card completion status atomically. |
| `TEST-T3-XF-014` | `TCG-N17` + `TCG-N13` | Pinning message inside specific forum topic | WS + Headless GPUI | Message pinned to topic banner only; parent group main pinned bar remains untouched. |
| `TEST-T3-XF-015` | `TCG-N18` + `TCG-N32` | Scheduled message fires while app is in background | WS + Headless GPUI | Triggers macOS dock unread badge increment and system notification banner upon dispatch. |
| `TEST-T3-XF-016` | `TCG-N19` + `TCG-N22` | Deleting message containing photo from shared media | WS + REST | Message deleted from stage; corresponding photo thumbnail disappears from Shared Media Archive tab. |
| `TEST-T3-XF-017` | `TCG-N20` + `TCG-N35` | PTS sequence sync during simulated 10s network drop | NetworkChaos + WS | Disconnects WS; server generates 10 updates; on reconnect client fetches delta via `SyncGetDifference`. |
| `TEST-T3-XF-018` | `TCG-N21` + `TCG-N52` | Search result video preview streamed via HTTP 206 range | REST | Clicking video search result streams middle video segment instantly via `Range: bytes` headers. |
| `TEST-T3-XF-019` | `TCG-N22` + `TCG-N23` | Shared Media tab browsing inside 320px Context Inspector | Headless GPUI | Inspector drawer switches to "Media" tab; infinite scroll virtual list loads 100 photo thumbnails. |
| `TEST-T3-XF-020` | `TCG-N23` + `TCG-N29` | Opening user profile in Inspector from Contacts modal | Headless GPUI | Selecting contact in Contacts roster slides open 320px Inspector with populated profile attributes. |
| `TEST-T3-XF-021` | `TCG-N24` + `TCG-N26` | 1-on-1 voice call upgraded to screen sharing stream | WebRTC + WS | Audio call negotiates SDP re-offer adding H.264 video track; remote view switches to screen canvas. |
| `TEST-T3-XF-022` | `TCG-N25` + `TCG-N07` | Group voice chat with active waveform speaker pulses | WebRTC + Headless GPUI | 5-bit waveform meter drives dynamic pulsing avatar ring for the active speaking participant. |
| `TEST-T3-XF-023` | `TCG-N26` + `CAP-04` | Remote computer CUA session view inside screen sharing stage | WebRTC + Headless GPUI | Displays remote VNC canvas inside call stage; yellow banner indicates active agent control lease. |
| `TEST-T3-XF-024` | `TCG-N27` + `TCG-N28` | Bot inline keyboard button opens sandboxed Mini App | Headless GPUI + WS | Clicking bot button opens `MiniAppWindow` with validated `initData` and sandboxed CSP context. |
| `TEST-T3-XF-025` | `TCG-N28` + `CAP-06` | Mini App triggers Telegram Stars checkout modal | Headless GPUI + REST | Mini App invokes `openInvoice`; renders native Stars payment sheet; updates card upon payment. |
| `TEST-T3-XF-026` | `TCG-N29` + `TCG-N30` | Contact profile updated on server reflects in chat header | WS + Headless GPUI | Server updates contact display name; active chat stage title bar updates immediately. |
| `TEST-T3-XF-027` | `TCG-N30` + `TCG-N38` | User profile avatar updated reflected in top Stories strip | REST + Headless GPUI | New avatar renders with gradient active story ring in top horizontal stories strip. |
| `TEST-T3-XF-028` | `TCG-N31` + `TCG-N46` | Night theme switched in custom window frame | Headless GPUI | Metal title bar, traffic light backgrounds, and chat stage transition to dark palette in single frame. |
| `TEST-T3-XF-029` | `TCG-N32` + `TCG-N45` | Shortcut `Cmd+K` opens search, dismissing unread badge | Headless GPUI | Pressing `Cmd+K` jumps to unread conversation; unread badge clears from sidebar and dock tile. |
| `TEST-T3-XF-030` | `TCG-N33` + `TCG-N30` | Privacy set to "Nobody" sees last seen timestamp | REST + WS | Peer views user profile; status string displays "Last seen recently" instead of exact timestamp. |
| `TEST-T3-XF-031` | `TCG-N34` + `TCG-N48` | Auto-download fills disk cache up to configured limit | REST | Cache engine reaches configured 5GB limit; triggers LRU eviction of oldest cached media chunks. |
| `TEST-T3-XF-032` | `TCG-N35` + `TCG-N02` | SOCKS5 proxy enabled during active WebSocket session | NetworkChaos + WS | Closes direct WS connection; reconnects via SOCKS5 proxy; completes handshake within 2s. |
| `TEST-T3-XF-033` | `TCG-N36` + `TCG-N02` | Switching devices while uncommitted draft is in composer | WS + Headless GPUI | Device A types draft and closes app; Device B opens same chat; draft text is pre-populated. |
| `TEST-T3-XF-034` | `TCG-N37` + `TCG-N03` | Archived chat received unmuted message | WS + Headless GPUI | Chat pops out of archive back into main chat list; unread badge counter increments. |
| `TEST-T3-XF-035` | `TCG-N38` + `TCG-N14` | Emoji reaction sent to active 24h story | WS + Headless GPUI | Story player displays animated reaction burst; story author receives notification in chat. |
| `TEST-T3-XF-036` | `TCG-N39` + `TCG-N40` | Secret chat message with 5s self-destruct timer | WS + Headless GPUI | E2EE message decrypts; read receipt starts 5s timer; burns and removes message at 5.0s. |
| `TEST-T3-XF-037` | `TCG-N40` + `TCG-N10` | Photo viewed in lightbox while self-destruct timer expires | Headless GPUI + WS | Self-destruct timer hits zero; closes lightbox automatically, displaying "Media expired". |
| `TEST-T3-XF-038` | `TCG-N41` + `CAP-06` | Live location stream rendered inside A2UI delivery card | Headless GPUI + WS | Dynamic coordinates update driver icon on interactive delivery map card in real time. |
| `TEST-T3-XF-039` | `TCG-N42` + `TCG-N10` | Web link preview article opened in Instant View modal | Headless GPUI | Clicking "Instant View" button on link preview renders clean reader view with formatted images. |
| `TEST-T3-XF-040` | `TCG-N43` + `TCG-N51` | Export chat history includes all chunked media attachments | REST | Export archive downloads all photos and voice notes, assembling complete HTML/JSON archive. |
| `TEST-T3-XF-041` | `TCG-N44` + `TCG-N05` | Language switched to Chinese while CJK IME is active | Headless GPUI | Switching language dynamically does not disrupt active Pinyin IME composition pre-edit text. |
| `TEST-T3-XF-042` | `TCG-N45` + `TCG-N04` | `Cmd+Up` shortcut triggers inline message edit mode | Headless GPUI | Composer converts into edit mode with "Edit Message" banner and pre-filled previous message text. |
| `TEST-T3-XF-043` | `TCG-N46` + `TCG-N10` | Entering fullscreen lightbox conceals custom window title bar | Headless GPUI | Fullscreen transition hides Metal traffic lights; exiting restores custom window frame. |
| `TEST-T3-XF-044` | `TCG-N47` + `TCG-N51` | Dropping 100MB file onto composer initiates chunked upload | Headless GPUI + REST | Drag-and-drop initiates `POST /media/upload/init`, slicing file into 512KB chunks automatically. |
| `TEST-T3-XF-045` | `TCG-N48` + `TCG-N52` | Range streaming starts before auto-download completes | REST | Audio playback begins immediately via HTTP 206 range request while background download finishes. |
| `TEST-T3-XF-046` | `TCG-N49` + `TCG-N50` | Switching account from Account 1 to Account 2 | Headless GPUI + WS | Bumps `account_generation`; closes WS 1; unmounts Account 1 database; opens Account 2 view. |
| `TEST-T3-XF-047` | `TCG-N50` + `CAP-02` | Background cron task running during account switch | WS | Scope fencing isolates cron worker; running task terminates or pauses until account is re-selected. |
| `TEST-T3-XF-048` | `TCG-N51` + `TCG-N07` | Chunked voice note upload extracts 5-bit waveform on server | REST | Final upload chunk triggers server audio decoder, generating 100-sample 5-bit waveform payload. |
| `TEST-T3-XF-049` | `TCG-N52` + `TCG-N09` | Scrubbing round video note streams via HTTP 206 ranges | REST + Headless GPUI | Dragging circular scrubber seeks video instantly, requesting precise byte ranges without buffering. |
| `TEST-T3-XF-050` | `CAP-01` + `CAP-02` | AI agent turn creates background task in Task Center | WS + Headless GPUI | Agent tool call creates 7-state task; task appears in NavRail Task Center board (`Cmd+2`). |
| `TEST-T3-XF-051` | `CAP-01` + `CAP-03` | Agent turn requests MCP tool requiring user approval | WS + Headless GPUI | Spawns `LocalToolPermissionDock` above composer; user approves; tool executes in sandbox. |
| `TEST-T3-XF-052` | `CAP-01` + `CAP-04` | Agent executes CUA computer use tool on remote node | WS + Headless GPUI | Agent acquires control lease; Workbench canvas displays remote screen with injected cursor moves. |
| `TEST-T3-XF-053` | `CAP-02` + `CAP-06` | Task Center approval card rendered in conversation stage | Headless GPUI | Amber approval banner embedded in chat; clicking "Approve Once" transitions task to `Running`. |
| `TEST-T3-XF-054` | `CAP-02` + `TCG-N12` | 5-Field cron routine posts automated update to channel | Cron + WS | Cron schedule `0 9 * * 1` triggers agent worker; posts weekly summary report to broadcast channel. |
| `TEST-T3-XF-055` | `CAP-03` + `TCG-N05` | Dynamic permission dock expands above CanonicalComposer | Headless GPUI | Dock animates smoothly into view, shifting composer input downwards without breaking focus. |
| `TEST-T3-XF-056` | `CAP-04` + `TCG-N45` | Pressing `Esc` triggers emergency takeover of remote PC | Headless GPUI + WS | `Esc` keystroke releases control lease instantly (<10ms), returning input to human operator. |
| `TEST-T3-XF-057` | `CAP-05` + `TCG-N07` | Talk Mode voice turn recorded and saved as voice note | WebRTC + REST | User speaks in Talk Mode; speech transcoded and saved in conversation history as 5-bit voice bubble. |
| `TEST-T3-XF-058` | `CAP-05` + `TCG-N25` + `CAP-01` | AI agent joins group voice chat with full-duplex Talk Mode | WebRTC + WS | Agent connects to SFU audio mix; listens via VAD; speaks into group stage with barge-in support. |

## 8. Tier 4: Real-World Application Scenarios (29 Comprehensive E2E Workflows)

Tier 4 validates **complete end-to-end operational workflows** simulating realistic user behavior across multiple devices, networks, and advanced capabilities from initialization to teardown.

### TEST-T4-E2E-001: User Onboarding, Passkey Auth, Profile Setup & First Direct Message
- **Workflow Steps**:
  1. User launches freshly installed GPUI Telegram client; headless app initializes SQLite memory database and renders Intro screen.
  2. User enters phone number `+15550199999`; server issues registration challenge.
  3. User completes FIDO2 Passkey registration; server stores public key and issues Ed25519 session token.
  4. Client updates user profile: display name "Alice Vance", bio "Distributed Systems Researcher", and uploads 512x512 avatar JPEG.
  5. Alice searches for `@bob_smith` via Tantivy search, opens direct chat, and types "Hello Bob, welcome to GPUI Telegram!".
  6. Message dispatches via WebSocket; server assigns PTS 1; Bob's client receives `NewMessage` event and renders incoming bubble.
- **Verification**: Database contains valid user record, avatar thumbnail exists in CAS storage, message delivery tick transitions from single to double tick upon Bob's read receipt.

### TEST-T4-E2E-002: Multi-Device Sync under Hostile Cellular Network
- **Workflow Steps**:
  1. Alice logs into Desktop Client A and Mobile Client B simultaneously.
  2. Network Chaos Controller injects 300ms latency, 10% packet drop, and periodic 3s socket disconnects on Client B.
  3. Client A sends 20 messages and edits 3 messages in rapid succession.
  4. Client B reconnects, detects PTS gap ($\text{PTS}_{\text{recv}} = 23, \text{PTS}_{\text{local}} = 0$), and triggers `SyncGetDifference`.
  5. Server returns delta payload with 20 new messages and 3 edited messages.
  6. Client B merges delta into local SQLite store and repaints SumTree transcript.
- **Verification**: Client B transcript strictly matches Client A transcript with zero missing, duplicate, or out-of-order messages.

### TEST-T4-E2E-003: Group Lifecycle: Creation, Admin Promotion & Supergroup Migration
- **Workflow Steps**:
  1. Alice creates basic group "Engineering Core" and invites Bob and Charlie.
  2. Alice promotes Bob to Admin with `CanPinMessages` and `CanDeleteMessages` privileges.
  3. Bob pins message "Project kick-off at 10:00 AM"; pinned banner displays in group stage.
  4. Automated invite bot adds 198 members, exceeding 200 member capacity.
  5. System triggers automatic migration to Supergroup; upgrades message history permissions and assigns permanent channel ID.
  6. Supergroup member search verifies instant retrieval across all 201 members.
- **Verification**: Supergroup flags enabled in SQLite, permissions matrix intact, pinned message survives migration.

### TEST-T4-E2E-004: Forum Topic Hierarchy & Threaded Discussions
- **Workflow Steps**:
  1. Admin enables Forum Topics on supergroup "Open Source Community".
  2. Admin creates topics: "General", "Rust Architecture", and "Release Notes".
  3. Members post 50 messages inside "Rust Architecture"; topic unread badge reflects activity without polluting "General".
  4. Admin pins "Rust Architecture Guidelines" inside the topic; pinned bar anchors to top of topic stage.
  5. Admin closes topic "Release Notes"; non-admin composer disables with "This topic is closed" banner.
- **Verification**: Messages queryable strictly by `topic_id`; stage breadcrumb displays correct parent/topic navigation path.

### TEST-T4-E2E-005: Public Channel Broadcast & Linked Discussion Comments
- **Workflow Steps**:
  1. Author creates public channel `@rust_weekly` and links discussion group "Rust Weekly Chat".
  2. Author schedules announcement for 5 seconds in future with silent broadcast flag.
  3. Cron engine dispatches post; 1,000 channel subscribers receive silent notification.
  4. Subscriber clicks "Leave a comment"; stage opens linked discussion thread with quoted broadcast post.
  5. Subscriber posts comment; comment appears in linked group and updates channel post comment counter.
- **Verification**: Channel subscriber counter accurate, discussion thread messages properly linked via `thread_root_message_id`.

### TEST-T4-E2E-006: 4K Video Publishing via Resumable Chunked Pipeline & HTTP 206 Streaming
- **Workflow Steps**:
  1. User attaches 250MB 4K MP4 video in chat stage.
  2. Client initiates chunk upload (`POST /api/v1/media/upload/init`); server allocates upload session.
  3. Client uploads chunks 0 to 240 (512KB each); at chunk 241, Network Chaos Controller drops connection.
  4. Client detects disconnect, waits 1s, queries upload status, and resumes from chunk 241.
  5. Final chunk uploaded; `POST /complete` verifies SHA-256 CAS hash; server generates video thumbnail and duration metadata.
  6. Recipient clicks play; video streams immediately via HTTP 206 Partial Content range requests without waiting for full download.
- **Verification**: Streamed video scrubber seeks instantly to any timestamp; uploaded file bit-identical to source file.

### TEST-T4-E2E-007: Voice Note Recording, 5-Bit Waveform & 2.0x Audio Playback
- **Workflow Steps**:
  1. User holds mic button in CanonicalComposer, slides up to lock hands-free recording.
  2. User speaks for 15 seconds; live audio meter animates input level.
  3. User clicks send; client encodes audio to Opus, computes 100-sample 5-bit waveform vector, and uploads voice note.
  4. Server processes upload and executes background Whisper Speech-to-Text transcription.
  5. Voice message bubble appears in stage with quantized waveform canvas and inline STT text snippet.
  6. Recipient clicks play, toggles speed to 2.0x; audio plays back cleanly at double speed without pitch distortion.
- **Verification**: Waveform samples conform strictly to 5-bit range (0..31); playback scrubber tracks audio position synchronously.

### TEST-T4-E2E-008: Round Video Note Recording, PIP Floating Mode & Read Receipts
- **Workflow Steps**:
  1. User toggles composer mic icon to video camera icon, holds to record video note.
  2. Client records circular 1:1 camera stream for 10 seconds, uploads round video note.
  3. Round video bubble renders in chat with circular progress ring.
  4. User scrolls chat up 500px; video note detaches and floats in bottom-right Picture-in-Picture mini-window.
  5. Video completes 100% playback; client transmits read receipt, marking message read.
- **Verification**: Video decoder releases hardware context on dismissal; PIP window bounds clamp within application frame.

### TEST-T4-E2E-009: Fullscreen Media Lightbox Navigation & Zoom Traversal
- **Workflow Steps**:
  1. Chat contains album of 10 high-resolution photos.
  2. User clicks photo #3; fullscreen lightbox opens with smooth zoom-in transition and blurred backdrop.
  3. User scrolls mouse wheel; image zooms to 300%; user clicks and drags to pan across zoomed image.
  4. User double-clicks to reset zoom; presses Right Arrow key; lightbox smoothly slides to photo #4.
  5. User presses `Esc`; lightbox smoothly closes, restoring focus to chat stage.
- **Verification**: Zoom scale clamped within 100% to 500%; keyboard shortcuts strictly trapped by lightbox modal.

### TEST-T4-E2E-010: Interactive Poll & Quiz Creation with Realtime Vote Convergence
- **Workflow Steps**:
  1. Creator opens Attach Menu -> "Create Poll"; configures single-choice quiz with 4 options and correct answer #2.
  2. Quiz broadcasts to 100 group members.
  3. 50 members vote option #1, 40 vote option #2, 10 vote option #3.
  4. Live percentage bars animate dynamically on all client stages as votes arrive.
  5. Member who selected option #2 receives celebration confetti animation and explanation text.
  6. Creator closes quiz; voting locks, and final result snapshot commits to database.
- **Verification**: Sum of votes strictly equals 100; percentage bars total 100%; closed quiz rejects subsequent votes.

### TEST-T4-E2E-011: Scheduled Message Queueing & Silent Push Execution
- **Workflow Steps**:
  1. User composes message, right-clicks send button -> "Schedule message"; sets time to $T + 10\text{s}$.
  2. Message enters local scheduled queue; scheduled icon displays in composer.
  3. User schedules second message with "Send without sound" flag.
  4. Clock reaches $T + 10\text{s}$; server cron worker dispatches first message with standard alert.
  5. Second message dispatches silently; recipient receives message in transcript without audio beep.
- **Verification**: Scheduled queue empties upon execution; delivery timestamps match target epoch within ±100ms.

### TEST-T4-E2E-012: Markdown v2 Formatting, Inline Editing & Cross-Device Deletion
- **Workflow Steps**:
  1. User sends message containing bold, italic, code block, and spoiler tags.
  2. Client renders rich formatted bubble with hidden spoiler text; clicking spoiler reveals content.
  3. User presses `Cmd+Up` to edit message; corrects typo and appends text; sends edit update.
  4. Bubble updates in real time with "(edited)" label beside timestamp.
  5. User right-clicks message -> "Delete for everyone"; server soft-deletes record and emits tombstone PTS update.
  6. Recipient client removes bubble from stage with smooth collapse animation.
- **Verification**: Soft-deleted record retains tombstone with zero content leakage; unread counters decrement appropriately.

### TEST-T4-E2E-013: Tantivy Full-Text Search with Faceted Media Filtering
- **Workflow Steps**:
  1. System contains 50,000 historical messages across 100 chats.
  2. User presses `Cmd+K` to open Universal Search bar; enters query `quantum algorithm has:document`.
  3. Tantivy FTS engine filters results in 3.2ms; returns matching messages containing PDF documents.
  4. Results list displays highlighted search snippet with `<mark>` tags around "quantum algorithm".
  5. User presses Enter on top result; chat stage navigates to historical message, highlights bubble temporarily.
- **Verification**: Query execution latency < 10ms; search navigation restores correct chat context and scroll offset.

### TEST-T4-E2E-014: 1-on-1 End-to-End Encrypted VoIP Audio Call
- **Workflow Steps**:
  1. Alice initiates audio call to Bob; client generates WebRTC SDP offer with SRTP encryption.
  2. Bob receives incoming call dialog with ringing tone; clicks "Accept".
  3. Bob generates SDP answer; signaling hub exchanges ICE candidates over WebSocket.
  4. Peer-to-peer WebRTC audio stream connects; round-trip latency measured at 45ms.
  5. Alice clicks "Mute"; local audio track mutes, Bob's interface displays muted icon.
  6. Alice hangs up; call ends, and call duration card commits to chat transcript.
- **Verification**: Zero packet leaks outside SRTP session; audio hardware context clean shutdown.

### TEST-T4-E2E-015: Multi-Party Group Voice & Video Stage with SFU Bridge
- **Workflow Steps**:
  1. Group admin launches Group Video Stage; server allocates SFU bridge instance.
  2. 10 participants join the call; 3 participants turn on webcams.
  3. SFU receives 3 video tracks and 10 audio tracks, forwarding composite streams to participants.
  4. Participant Charlie speaks; 5-bit waveform meter detects speech, lights up Charlie's avatar ring.
  5. Participant Dave raises hand; admin receives notification and grants speaking permission.
  6. Admin ends group call; SFU terminates bridge session and disconnects WebRTC channels.
- **Verification**: Memory and CPU utilization remain stable; SFU bridge resources completely deallocated.

### TEST-T4-E2E-016: Ultra-Low Latency Screen Sharing with Dynamic Bitrate Adaptation
- **Workflow Steps**:
  1. During active call, user clicks "Share Screen"; selects primary 4K monitor.
  2. Screen capture pipeline streams 60 FPS video track to WebRTC peer connection.
  3. Remote peer views screen share in dedicated canvas view; text remains sharp and readable.
  4. Network Chaos Controller throttles bandwidth from 20 Mbps to 2 Mbps.
  5. WebRTC congestion controller adapts resolution from 4K to 1080p, reducing frame rate to maintain stream stability.
  6. User clicks "Stop Sharing"; stream reverts back to standard voice call layout.
- **Verification**: Screen sharing stream recovers smoothly upon network restoration without pipeline crashes.

### TEST-T4-E2E-017: Bot Platform & Inline Keyboards with Callback Query Flow
- **Workflow Steps**:
  1. User adds `@booking_bot` to chat and sends `/book_flight`.
  2. Bot responds with message containing 2x3 inline button keyboard (Dates, Destinations, Confirm).
  3. User clicks "Destinations"; button displays loading spinner while callback query dispatches.
  4. Bot server receives callback, updates message in place with list of cities.
  5. User selects "Tokyo"; bot updates message with confirmed booking receipt card.
- **Verification**: Callback queries resolve within 200ms; inline button state transitions smoothly from loading to updated.

### TEST-T4-E2E-018: Sandboxed Mini App with Telegram Stars Payment Checkout
- **Workflow Steps**:
  1. User clicks bot attachment menu -> launches "Cloud Commerce" Mini App.
  2. GPUI desktop creates isolated `MiniAppWindow` with strict CSP fencing and custom title bar.
  3. Mini App requests user theme colors; host injects active Day/Night palette variables.
  4. User clicks "Buy Item for 100 Stars"; Mini App triggers `openInvoice`.
  5. Host renders native Telegram Stars payment modal; user confirms payment.
  6. Backend deducts 100 Stars from user wallet, credits merchant, and sends receipt message to chat.
- **Verification**: Mini App cannot access external DOM or local disk; transaction commits atomically.

### TEST-T4-E2E-019: End-to-End Encrypted Secret Chat with 5-Second Self-Destruct
- **Workflow Steps**:
  1. Alice initiates Secret Chat with Bob; clients execute X25519 Diffie-Hellman key exchange.
  2. Shared key derived using HKDF-SHA256; Secret Chat initialized with green padlock icon.
  3. Alice sets self-destruct timer to 5 seconds.
  4. Alice sends confidential message; message encrypted with XChaCha20-Poly1305.
  5. Bob receives and marks message read; 5-second countdown timer starts visually in bubble.
  6. Timer hits 0; message ciphertext and plaintext wiped from both client databases and memory.
- **Verification**: Database audit confirms zero remaining plaintext or keys for expired secret message.

### TEST-T4-E2E-020: Realtime Live Location Sharing & Interactive Map Tracking
- **Workflow Steps**:
  1. User clicks Attach -> "Share Live Location"; selects 1-hour broadcast duration.
  2. Client acquires GPS mock coordinates, transmitting updates every 5 seconds.
  3. Recipient chat renders interactive map card with pulsing avatar marker.
  4. Simulated GPS updates show user moving along path; map marker animates along route.
  5. User clicks "Stop Sharing"; live broadcast terminates, freezing final path on static map preview.
- **Verification**: GPS coordinate updates stop immediately upon termination; location stream cleans up cleanly.

### TEST-T4-E2E-021: Complete Chat Data Portability: HTML & JSON Export
- **Workflow Steps**:
  1. User opens chat settings -> "Export Chat History".
  2. Configures export: Date range "Last 30 days", includes photos, videos, voice notes, and stickers.
  3. Export worker streams messages from SQLite WAL, copies attachments from CAS storage, and builds zip bundle.
  4. Client unpacks archive; verifies `messages.json` matches SQLite database schema.
  5. Client opens `index.html` in browser; verifies all images, voice players, and bubbles render offline.
- **Verification**: Exported message count bit-for-bit matches database; zero broken media links in HTML archive.

### TEST-T4-E2E-022: Multi-Account Hot Switching with Scope Fencing Protection
- **Workflow Steps**:
  1. Client has Account 1 (Personal) and Account 2 (Work) logged in.
  2. User is viewing Account 1; initiates large 500MB file download in background.
  3. User clicks Left NavRail avatar switcher, switches to Account 2.
  4. Scope Generation Harness increments `account_scope_generation`; isolates WebSocket stream and DB pool.
  5. Account 1 download worker detects generation bump, pauses cleanly without corrupting Account 2 state.
  6. User switches back to Account 1; download resumes from last saved offset.
- **Verification**: Zero cross-account data leaks; database connection pools remain completely isolated.

### TEST-T4-E2E-023: AI Multi-Agent ACP: Lineage Spawning & Thinking Bubble Stream
- **Workflow Steps**:
  1. User sends complex research prompt to `@research_agent` in chat.
  2. Agent begins turn; stage renders collapsible shimmering `AgentThinkingBubble`.
  3. Agent spawns two child subagents: "WebScraper" and "DataAnalyzer".
  4. Context Inspector drawer displays live `SubagentLineageTree` showing worker statuses.
  5. Subagents complete tasks, return artifacts to parent agent; parent summarizes final answer.
  6. Final answer renders with elapsed thinking time pill ("Thought for 4.8s").
- **Verification**: Subagent processes terminate cleanly; execution transcript preserved in SandAgentDb.

### TEST-T4-E2E-024: Global Task Center: 7-State FSM & 5-Field Cron Routine
- **Workflow Steps**:
  1. User opens Global Task Center workspace via Left NavRail (`Cmd+2`).
  2. User creates automated routine with cron `0 8 * * 1-5` (weekdays at 8:00 AM) to summarize team tasks.
  3. Cron engine schedules job; displays next 7-day run timeline.
  4. User triggers manual run; task enters `Pending` -> transitions to `Running`.
  5. Worker requires access to production database; task enters `WaitingAuth` and displays Amber Approval Card.
  6. User clicks "Approve Once"; task transitions back to `Running` -> finishes in `Completed` state.
- **Verification**: Task FSM transitions strictly conform to 7-state lifecycle rules; logs archived in SQLite.

### TEST-T4-E2E-025: MCP Tool Discovery, Dynamic Permission Dock & Sandboxed Run
- **Workflow Steps**:
  1. User installs MCP server "Postgres Inspector" in Extensions settings.
  2. Agent receives prompt: "Show table schema for users table".
  3. Agent selects MCP tool `db_inspect_schema`; triggers tool execution request.
  4. Host detects sensitive local resource access; slides up `LocalToolPermissionDock` above composer.
  5. Dock displays tool name, parameters, and risk level; user reviews and clicks "Allow".
  6. Tool executes inside sandboxed `box-exec-daemon`; returns schema table card into chat stage.
- **Verification**: Tool execution denied if user clicks "Deny"; dock unmounts smoothly without composer blur.

### TEST-T4-E2E-026: Autonomous Remote Computer CUA Control with Emergency Esc Takeover
- **Workflow Steps**:
  1. User prompts agent: "Log into staging server and restart nginx service".
  2. Agent requests remote control lease; host issues `ComputerControlLease` with generation counter.
  3. Conversation opens `ComputerWorkbench` view, displaying live remote VNC desktop stream.
  4. Agent injects simulated mouse movements and keyboard commands; yellow active control banner displays.
  5. Agent begins typing unexpected command; human operator immediately presses `Esc` key.
  6. Emergency kill-switch releases lease in 6ms; remote input locks agent out; operator takes manual control.
- **Verification**: Kill-switch latency strictly < 10ms; agent receives lease revocation error and halts.

### TEST-T4-E2E-027: Full-Duplex Talk Mode with 60ms Barge-In Interruption
- **Workflow Steps**:
  1. User clicks phone/waveform icon in top header; 160px dynamic energy orb modal opens.
  2. Orb enters `Listening` state; user speaks prompt: "Explain the raft consensus protocol".
  3. VAD detects silence boundary (>500ms); orb transitions to `Thinking` then `Speaking`.
  4. AI begins speaking response through speaker; energy orb undulates with audio spectrum.
  5. User speaks "Wait, stop!" during AI speech; VAD triggers barge-in detection in 42ms.
  6. Client immediately flushes audio output buffer, aborts TTS stream, and transitions orb back to `Listening`.
- **Verification**: Interruption latency < 60ms; SandAgentDb transcript truncates AI turn to exact uttered syllables.

### TEST-T4-E2E-028: Collaborative A2UI Workboard with Todos & Stars Settlement
- **Workflow Steps**:
  1. Project manager agent posts A2UI collaborative task workboard card with 5 interactive checklist items.
  2. Team members tick off items #1, #2, and #3; checklist progress bar updates from 0% to 60%.
  3. Agent marks workboard complete, generates linked Stars invoice card for bounty payment.
  4. Team lead clicks "Pay Bounty (500 Stars)"; native Stars checkout executes.
  5. Invoice card updates state to "Paid"; invoice receipt recorded in wallet ledger.
- **Verification**: Workboard state synchronized across all clients via atomic card delta events.

### TEST-T4-E2E-029: Complete Disaster Recovery, WAL Replay & PTS Gap Re-Sync
- **Workflow Steps**:
  1. Client is receiving burst of 100 incoming messages and active media uploads.
  2. Test harness simulates abrupt process termination (`kill -9` on client process).
  3. Client relaunches; SQLite database detects recovery mode, rolls forward uncommitted WAL entries.
  4. Client reconnects to WebSocket gateway; compares local PTS with server PTS.
  5. Detects 45 missed updates during downtime; issues `SyncGetDifference(pts = 105, limit = 100)`.
  6. Server returns missed updates; client applies updates, rebuilding SumTree transcript state.
- **Verification**: Zero database corruption; final transcript bit-identical to server state; all unread badges accurate.

## 9. Automated 3,441 Source-File Ledger Closure Verification Test (MANDATE #4)

In strict accordance with **Mandate #4**, the project mandates that 100% of the **3,441 source and `.style` files** under upstream Telegram Desktop (`/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/`) are enumerated, categorized, and mapped in the Source-File Parity Ledger without omission.

To guarantee that this ledger remains mathematically sound, tamper-proof, and fully realized in code, an automated ledger closure verification test is established as an **uncompromising prerequisite gate for E2E testing and production release**.

### 9.1 Verification Tool: `scripts/verify_ledger_closure.py`
The verification script is located at `scripts/verify_ledger_closure.py` and is invoked during CI pre-flight checks:

```bash
# Verify 100% closure of all 3,441 source files against parity ledger
python3 scripts/verify_ledger_closure.py \
    --upstream-dir /Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles \
    --ledger-json docs/specs/parity-ledger.json \
    --target-workspace . \
    --strict
```

### 9.2 Verification Rules & Invariants
The automated script executes 5 strict mathematical verification stages:

```
+----------------------------------------------------------------------------------------------------+
| STAGE 1: Source Tree Inventory Check                                                               |
| - Scans all files in tdesktop/Telegram/SourceFiles/                                                |
| - Verifies exact file count matches inventory (3,441 files total)                                  |
+----------------------------------------------------------------------------------------------------+
                                                  |
                                                  v
+----------------------------------------------------------------------------------------------------+
| STAGE 2: Ledger Exhaustiveness & Zero-Omission Audit                                              |
| - Compares filesystem paths against ledger `rows[].source_path` entries                            |
| - Asserts unmapped_count == 0; any omitted file triggers immediate exit code 1                     |
+----------------------------------------------------------------------------------------------------+
                                                  |
                                                  v
+----------------------------------------------------------------------------------------------------+
| STAGE 3: Responsibility & Symbol Traceability Check                                                |
| - Asserts every row contains non-empty `core_responsibilities` and `source_symbols`                |
| - Validates target binding (`target_crate_path` in `crates/` or `inherited/`)                       |
+----------------------------------------------------------------------------------------------------+
                                                  |
                                                  v
+----------------------------------------------------------------------------------------------------+
| STAGE 4: Automated Test Linkage Verification                                                       |
| - Asserts every mapped row links to at least one valid Test ID in Tiers 1-4 or baseline contracts |
| - Verifies that the referenced test case exists in test harness source code                        |
+----------------------------------------------------------------------------------------------------+
                                                  |
                                                  v
+----------------------------------------------------------------------------------------------------+
| STAGE 5: UI vs Domain Architectural Disposition Audit                                              |
| - Confirms UI files (.style, widgets, views) map to `gpui_telegram_desktop`                        |
| - Confirms non-UI files (api, mtproto, storage, data) map to server, proto, core, or inherited     |
+----------------------------------------------------------------------------------------------------+
```

### 9.3 Ledger Closure Metric Schema
The verification script emits a structured audit report (`docs/specs/ledger_closure_report.json`):

```json
{
  "timestamp_utc": "2026-11-06T07:35:00Z",
  "audit_version": "1.0.0",
  "upstream_root": "Telegram/SourceFiles",
  "total_source_files_discovered": 3441,
  "total_ledger_entries": 3441,
  "unmapped_files_count": 0,
  "closure_percentage": 100.0,
  "categorization": {
    "dialogs_and_chat_list": 218,
    "history_and_sumtree_stage": 542,
    "chat_helpers_and_composer": 310,
    "media_and_streaming": 384,
    "calls_and_webrtc": 182,
    "boxes_and_modals": 412,
    "settings_and_privacy": 296,
    "data_and_storage": 468,
    "window_and_core": 340,
    "ui_and_styles": 289
  },
  "target_disposition": {
    "gpui_telegram_desktop": 1845,
    "gpui_telegram_server": 612,
    "gpui_telegram_proto_and_core": 384,
    "gpui_telegram_media": 142,
    "gpui_telegram_webrtc": 98,
    "inherited_crates": 360
  },
  "test_linkage_closure_percentage": 100.0,
  "verdict": "PASSED"
}
```

---

## 10. Pass/Fail Semantics, CI Gates & `TEST_READY.md` Publication Protocol

### 10.1 Pass/Fail Gating Thresholds
To ensure uncompromising product quality, the release pipeline implements **zero-tolerance gating**:

| Verification Track | Total Tests / Assets | Minimum Pass Requirement | Gating Threshold |
|---|---|---|---|
| **Source Ledger Closure (Mandate #4)** | 3,441 Source Files | 3,441 / 3,441 Mapped & Verified | **100.0% (Zero omissions)** |
| **Inherited Contract Baseline (Mandate #1)** | 465 Test Suites | 465 / 465 Passing | **100.0% (Zero regressions)** |
| **Tier 1: Feature Coverage** | 290 Test Cases | 290 / 290 Passing | **100.0%** |
| **Tier 2: Boundary & Corner Cases** | 290 Test Cases | 290 / 290 Passing | **100.0%** |
| **Tier 3: Cross-Feature Interactions** | 58 Test Cases | 58 / 58 Passing | **100.0%** |
| **Tier 4: Real-World Scenarios** | 29 Workflows | 29 / 29 Passing | **100.0%** |
| **Total Test Suite** | **>= 667 Tests + 465 Baseline** | **1,132 / 1,132 Passing** | **100.0% Strict Pass** |

- **No Unflagged Test Skips**: Any test annotated with `#[ignore]` without an active tracking issue and orchestrator sign-off triggers immediate CI failure.
- **Flakiness Policy**: Any test that exhibits non-deterministic pass/fail across 3 consecutive runs is flagged as a blocking defect. Tests must utilize deterministic virtual time (`gpui::TestAppContext::advance_clock`) and explicit synchronization barriers rather than wall-clock delays.

---

### 10.2 CI Test Pipeline Execution Flow
The automated CI pipeline (`scripts/run_e2e_tests.sh`) executes in four sequential stages:

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "=== STAGE 1: Verifying 100% Source-File Parity Ledger Closure (3,441 Files) ==="
python3 scripts/verify_ledger_closure.py \
    --upstream-dir /Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles \
    --ledger-json docs/specs/parity-ledger.json \
    --strict

echo "=== STAGE 2: Running Inherited Baseline Contract Tests (465 Suites) ==="
cargo test -p mahayana-messaging
cargo test -p mahayana_host_runtime

echo "=== STAGE 3: Running GPUI Telegram 4-Tier Test Suite (667 Tests) ==="
cargo test -p gpui_telegram_test_harness --test tier1_feature_coverage
cargo test -p gpui_telegram_test_harness --test tier2_boundary_corner
cargo test -p gpui_telegram_test_harness --test tier3_cross_feature
cargo test -p gpui_telegram_test_harness --test tier4_real_world_e2e

echo "=== STAGE 4: Generating TEST_READY.md Certification Artifact ==="
python3 scripts/generate_test_ready_report.py
```

---

### 10.3 Publication Protocol for `TEST_READY.md`
Upon successful execution of all stages, the Orchestrator aggregates the results and publishes the official release certification document at `.agents/teamwork/TEST_READY.md`.

#### Mandatory Template for `TEST_READY.md`:
```markdown
# TEST_READY CERTIFICATION REPORT

## Release Metadata
- **Target Release**: GPUI Telegram 1.0.0
- **Certification Date**: [UTC Timestamp]
- **Target Architecture**: GPUI 0.2.2 Desktop + Independent Rust Server
- **Sign-Off Authority**: Teamwork QA & Test Writer Track

## Verification Results Summary
- **MANDATE #4 Source Ledger Closure**: 3,441 / 3,441 files verified (100.0%) - PASSED
- **MANDATE #1 Inherited Baseline Tests**: 465 / 465 suites passing (100.0%) - PASSED
- **Tier 1 (Feature Coverage)**: 290 / 290 tests passing (100.0%) - PASSED
- **Tier 2 (Boundary & Corner)**: 290 / 290 tests passing (100.0%) - PASSED
- **Tier 3 (Cross-Feature Combinations)**: 58 / 58 tests passing (100.0%) - PASSED
- **Tier 4 (Real-World E2E Scenarios)**: 29 / 29 workflows passing (100.0%) - PASSED
- **Aggregate Pass Rate**: 1,132 / 1,132 (100.0%)

## Formal Sign-Off
All 58 features, 3,441 upstream source files, and inherited production runtimes have been completely verified against authoritative behavioral contracts. The project is certified as TEST_READY for final release packaging.
```
