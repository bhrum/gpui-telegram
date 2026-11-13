# GPUI Telegram System Topology & Monorepo Architecture Specification

**Document ID**: `DOC-ARCHITECTURE-SYSTEM-TOPOLOGY`  
**Revision**: `3.0.0` (Milestone 1, Explorer Gen 3)  
**Date**: `2026-11-06`  
**Status**: Authoritative Monorepo Architecture Baseline  
**Target Repository**: `/Users/bhrum/teamwork_projects/gpui_telegram`  
**Target Path**: `docs/architecture.md`  

---

## 1. High-Level Architecture & Tiered Boundaries

```
+---------------------------------------------------------------------------------------------------+
|                                     GPUI Desktop Client (Rust)                                    |
|   NavRail (52px)  |  ChatSidebar (288px)  |  ConversationStage (SumTree)  |  Composer / Inspector |
|   - UniformList       - Folder Tabs          - 11 Rich Card Types             - IME Safe Input    |
|   - Task Center       - Search / Filter      - Voice 5-Bit Waveform           - Context Drawer    |
|   - Devices / MCP     - Unread Badges        - Virtualized Transcript         - Voice Gesture Bar |
+-------------------------------------------------+-------------------------------------------------+
                                                  |
                 +--------------------------------+-------------------------------+
                 | (Direct In-Memory / FFI Bridge)|                               |
                 v                                |                               |
+-----------------------------------------------+ |                               |
|        Inherited Production Domain Core       | |                               |
| - mahayana-host-runtime (ACP, Tasks, CUA)     | |                               |
| - fabushi-messaging-core (Store, Encryption)  | |                               |
| - mahayana-node-agent-coordinator (TDRP, VNC) | |                               |
| - SandAgentDb (SQLite WAL, Key-Value Store)   | |                               |
+-----------------------------------------------+ |                               |
                                                  |                               |
                   HTTP/REST (Chunked / Ranges)   | WebSocket (RPC & PTS Sync)   | WebRTC (VoIP)
                                                  v                               |
+---------------------------------------------------------------------------------+-----------------+
|                               gpui_telegram_server (Axum + Tokio)                                 |
|                                                                                                   |
|  +-----------------------------+  +-----------------------------+  +---------------------------+  |
|  | Auth & Session Management   |  | Sequence Sync Engine (PTS)  |  | Gateway & WebSocket Hub   |  |
|  | - WebAuthn / Passkeys       |  | - Monotonic Counter per Chat|  | - RPC Dispatcher          |  |
|  | - Argon2id / Ed25519 Tokens |  | - SyncGetDifference Query   |  | - 30s Heartbeat Ping/Pong |  |
|  | - Generation Fencing        |  | - Gap Recovery & Snapshot   |  | - Ephemeral Status Events |  |
|  +-----------------------------+  +-----------------------------+  +---------------------------+  |
|                                                                                                   |
|  +-----------------------------+  +-----------------------------+  +---------------------------+  |
|  | Messaging Pipeline          |  | Chunked Media & Streaming   |  | Tantivy Search Engine     |  |
|  | - Atomic Write Transaction  |  | - Resumable Upload Chunks   |  | - Pure-Rust Lucene        |  |
|  | - client_nonce Deduplication|  | - HTTP 206 Partial Content  |  | - Sub-ms Message Queries  |  |
|  | - Soft Deletes & Tombstones |  | - Automatic 5-bit Waveforms |  | - CJK Snippet Highlights  |  |
|  +-----------------------------+  +-----------------------------+  +---------------------------+  |
|                                                                                                   |
|  +-----------------------------+  +-----------------------------+  +---------------------------+  |
|  | WebRTC Signaling & SFU      |  | AI Orchestration Engine     |  | Global Task Center Daemon |  |
|  | - SDP Offer / Answer        |  | - ACP Runner (Subagent Tree)|  | - 7-State Task FSM       |  |
|  | - ICE Trickle Relay         |  | - Streaming Reasoning Pulse |  | - 5-Field Cron Routines   |  |
|  | - Built-in STUN / TURN      |  | - Tool Calling Sandbox      |  | - Auto-Review Gatekeeper  |  |
|  +-----------------------------+  +-----------------------------+  +---------------------------+  |
|                                                                                                   |
|  +---------------------------------------------------------------------------------------------+  |
|  |             Persistence Layer: SQLite (WAL Mode) / PostgreSQL Pool & Media CAS Disk           |  |
|  +---------------------------------------------------------------------------------------------+  |
+---------------------------------------------------------------------------------------------------+
```

## 2. 9-Crate Dependency DAG & Anti-Monolith Fine-Grained Layout

### 2.1 Dependency Directed Acyclic Graph (DAG)

```
                       gpui_telegram_core
                         /             \
                        v               v
              gpui_telegram_proto   gpui_telegram_media
                  /     |      \          |
                 v      |       v         v
         gpui_telegram_webrtc  gpui_telegram_agent <---+ [inherited crates]
                 |      |       |         |
                 +------+-------+---------+
                        |
               +--------+--------+
               |                 |
               v                 v
     gpui_telegram_server  gpui_telegram_client <-----+ [inherited crates]
                                 |
                                 v
                       gpui_telegram_desktop (GPUI 0.2.2)
                                 |
                                 v
                       gpui_telegram_test_harness
```

### 2.2 Fine-Grained Directory Layout (Enforcing Mandate #5 Anti-Monolith Architecture)

To eliminate any risk of cramming modules into monolithic files, each crate strictly adopts fine-grained directory decomposition mirroring upstream `tdesktop/Telegram/SourceFiles/`:

```
crates/
├── gpui_telegram_core/                  # Pure Domain Types & Invariants
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── error.rs                     # TelegramError hierarchy
│       ├── types/                       # Primitive domain models
│       │   ├── mod.rs
│       │   ├── ids.rs                   # UserId, ChatId, MessageId, TopicId
│       │   ├── flags.rs                 # MessageFlags, ChatFlags, PrivacyFlags
│       │   └── presence.rs              # UserStatus, TypingStatus
│       └── auth/                        # Cryptographic token definitions
│           ├── mod.rs
│           ├── session_token.rs
│           └── generation_fence.rs
│
├── gpui_telegram_proto/                 # Wire Formats & Sync Contracts
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── sync/                        # Sequence sync schemas
│       │   ├── mod.rs
│       │   ├── pts.rs                   # PtsUpdate, SyncGetDifference
│       │   └── events.rs                # UpdateEvent enum
│       ├── message/                     # Message item & content models
│       │   ├── mod.rs
│       │   ├── item.rs                  # MessageItem
│       │   ├── content.rs               # MessageContent (Text, Media, Poll, Card)
│       │   └── tombstone.rs             # Delete tombstone records
│       ├── ws/                          # WebSocket framing
│       │   ├── mod.rs
│       │   └── frames.rs                # WsClientFrame, WsServerFrame
│       └── cards/                       # Interactive card schemas
│           ├── mod.rs
│           ├── poll.rs
│           ├── todo.rs
│           └── invoice.rs
│
├── gpui_telegram_media/                 # Audio, Video & Waveform Processing
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── chunked_upload.rs            # Chunk verification, staging
│       ├── range_stream.rs              # HTTP 206 partial response adapter
│       ├── waveform/                    # Telegram 5-bit waveform codec
│       │   ├── mod.rs
│       │   ├── encoder.rs               # 100-sample 5-bit compressor
│       │   └── decoder.rs               # Floating-point bar unpacker
│       └── thumbnails/                  # WebP / JPEG thumbnail extractor
│           ├── mod.rs
│           └── generator.rs
│
├── gpui_telegram_webrtc/                # VoIP, Video & Screen Signaler
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── signaling/                   # Signaling frames
│       │   ├── mod.rs
│       │   ├── frames.rs                # Offer, Answer, IceCandidate
│       │   └── session.rs               # CallSession state
│       ├── ice/                         # STUN / TURN credentials
│       │   ├── mod.rs
│       │   └── turn_token.rs
│       └── sfu/                         # Group stage media router
│           ├── mod.rs
│           └── bridge.rs
│
├── gpui_telegram_agent/                 # ACP, Tasks, MCP & Talk Integration
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── acp/                         # AI Multi-Agent ACP runner
│       │   ├── mod.rs
│       │   ├── runner.rs                # Orchestrates mahayana-host-runtime
│       │   └── lineage.rs               # Subagent lineage tree
│       ├── tasks/                       # 7-State Task FSM & Cron
│       │   ├── mod.rs
│       │   ├── fsm.rs                   # TaskStateMachine
│       │   ├── cron.rs                  # 5-field cron routine evaluator
│       │   └── approval.rs              # Sensitive tool approval gate
│       ├── mcp/                         # Tool registry & permission dock
│       │   ├── mod.rs
│       │   ├── registry.rs
│       │   └── permission_dock.rs
│       ├── computer/                    # Remote Computer Control & CUA
│       │   ├── mod.rs
│       │   ├── lease.rs                 # Exclusive control lease
│       │   └── cua_executor.rs
│       └── talk/                        # Full-duplex Talk Mode
│           ├── mod.rs
│           ├── coordinator.rs
│           └── vad_bargein.rs           # 60ms barge-in detector
│
├── gpui_telegram_server/                # Axum + Tokio Independent Server
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── server.rs                    # Axum router & Tower setup
│       ├── config.rs                    # Server runtime config
│       ├── auth/                        # Auth & session handlers
│       │   ├── mod.rs
│       │   ├── session_manager.rs
│       │   └── passkey_service.rs
│       ├── sync/                        # PTS sync engine
│       │   ├── mod.rs
│       │   └── pts_engine.rs            # Atomic sequence updater
│       ├── messaging/                   # Atomic message pipeline
│       │   ├── mod.rs
│       │   ├── pipeline.rs
│       │   └── dedup.rs                 # client_nonce cache
│       ├── search/                      # Tantivy full-text index
│       │   ├── mod.rs
│       │   ├── schema.rs
│       │   └── tantivy_indexer.rs
│       ├── media/                       # Storage & 206 range service
│       │   ├── mod.rs
│       │   └── storage_service.rs
│       ├── ws/                          # WebSocket hub
│       │   ├── mod.rs
│       │   └── gateway_hub.rs
│       └── webrtc/                      # Call signaling hub
│           ├── mod.rs
│           └── signaling_hub.rs
│
├── gpui_telegram_client/                # Client Networking & Local Cache
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── client.rs                    # High-level client facade
│       ├── sync/                        # Client sync coordinator
│       │   ├── mod.rs
│       │   └── coordinator.rs
│       ├── cache/                       # Local SQLite caching
│       │   ├── mod.rs
│       │   └── local_store.rs
│       └── net/                         # WebSocket & HTTP transport
│           ├── mod.rs
│           ├── ws_channel.rs
│           └── http_client.rs
│
├── gpui_telegram_desktop/               # 1:1 GPUI Desktop Application
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                      # App initialization & Metal window
│       ├── app.rs                       # GPUI AppContext wiring
│       ├── theme/                       # Telegram themes (Day Blue, Night)
│       │   ├── mod.rs
│       │   └── palette.rs
│       ├── components/                  # Fine-grained reusable widgets
│       │   ├── mod.rs
│       │   ├── avatar.rs
│       │   ├── badge.rs
│       │   ├── dialog_row.rs            # Chat sidebar row
│       │   ├── thinking_bubble.rs       # Agent reasoning bubble
│       │   ├── voice_bubble_player.rs   # 5-bit waveform player
│       │   ├── inline_keyboard.rs       # 1-8 column inline buttons
│       │   ├── talk_mode_orb.rs         # 160px Canvas energy orb
│       │   ├── tool_permission_dock.rs  # MCP approval dock
│       │   ├── vnc_canvas.rs            # Remote desktop canvas
│       │   └── cards/                   # Rich interactive cards
│       │       ├── mod.rs
│       │       ├── poll_card.rs
│       │       ├── todo_card.rs
│       │       └── invoice_card.rs
│       └── views/                       # Main application views & stages
│           ├── mod.rs
│           ├── shell.rs                 # 4-pane AppShell
│           ├── nav_rail.rs              # 52px Primary NavRail
│           ├── chat_sidebar.rs          # 288px ChatSidebar with folders
│           ├── conversation_stage.rs    # SumTree message transcript
│           ├── canonical_composer.rs    # IME-safe composer dock
│           ├── context_inspector.rs     # 320px Right drawer
│           ├── global_tasks_workspace.rs# 3-column task board
│           ├── computer_workbench.rs    # Remote PC modal
│           ├── media_viewer.rs          # Lightbox modal
│           ├── call_overlay.rs          # WebRTC call UI
│           ├── boxes/                   # Fine-grained dialog boxes
│           │   ├── mod.rs
│           │   ├── share_box.rs
│           │   ├── url_auth_box.rs
│           │   ├── delete_box.rs
│           │   ├── edit_peer_box.rs
│           │   └── forward_box.rs
│           └── settings/                # Fine-grained settings views
│               ├── mod.rs
│               ├── profile_settings.rs
│               ├── privacy_settings.rs
│               ├── storage_settings.rs
│               └── network_settings.rs
│
└── gpui_telegram_test_harness/          # 4-Tier Automated Test Framework
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── tier1_unit/                  # Unit test drivers
        ├── tier2_integration/           # Server integration tests
        ├── tier3_headless/              # GPUI headless test drivers
        ├── tier4_e2e/                   # Full system E2E tests
        └── fixtures/                    # Mock data & network fixtures
```

## 3. Core Subsystem Deep Dives

### 3.1 Sequence Sync Engine (The PTS/QTS Model)
Telegram's consistency model guarantees zero dropped messages through monotonic Presentation Sequences (PTS):
1. **Atomic Sequence Generation**:
   Every state alteration (new message, edit, deletion tombstone, reaction, pin) increments `pts` inside SQLite using an immediate transaction:
   ```sql
   BEGIN IMMEDIATE;
   UPDATE chat_sequence SET pts = pts + 1 WHERE chat_id = ? RETURNING pts;
   INSERT INTO pts_updates (chat_id, pts, event_type, payload, created_at_ms)
   VALUES (?, ?, ?, ?, ?);
   COMMIT;
   ```
2. **Gap Detection & Reconciliation (`SyncGetDifference`)**:
   - The client stores `last_seen_pts`.
   - On reconnect or after receiving an out-of-order event ($pts > last\_seen\_pts + 1$), the client sends `SyncGetDifferenceRequest { pts: last_seen_pts, limit: 100 }`.
   - The server queries `pts_updates` for $pts > last\_seen\_pts$.
   - If the sequence gap exceeds the rolling log buffer (e.g., > 5,000 updates), the server responds with `snapshot_required: true`, prompting the client to invalidate local state and fetch an authoritative full state snapshot.

### 3.2 Tantivy Full-Text Search Engine
- **Schema**:
  Fields include: `message_id` (STRING | STORED), `chat_id` (STRING | STORED | FAST), `sender_id` (STRING | FAST), `date` (I64 | FAST | STORED), `content` (TEXT with CJK tokenizer | STORED), `media_type` (STRING | FAST).
- **Sub-Millisecond Querying**:
  Asynchronous writer thread batches indexing every 500ms using a crossbeam channel. Queries execute with sub-millisecond latency across hundreds of thousands of messages. Highlights return matching byte ranges formatted as GPUI `TextRun` styled tokens.

### 3.3 Resumable Chunked Media Storage & HTTP 206 Streaming
- **Upload Protocol**:
  Client initiates with `upload/init` (declaring total size, SHA-256, chunk size 512KB-1MB). Chunks are uploaded with sequential `chunk_index`. Upon completion, the server verifies the overall SHA-256 hash and atomically moves the staged file into a Content-Addressed Storage (CAS) path `/data/media/{hash[0..2]}/{hash}`.
- **Waveform Synthesis**:
  On audio completion, `gpui_telegram_media::waveform::encoder` processes the audio samples, computing 100 root-mean-square amplitude peaks, quantized to 5-bit unsigned integers ($0 \le v \le 31$), producing a compact 63-byte binary payload stored directly in the message record.
- **HTTP 206 Range Streaming**:
  The media streaming endpoint parses incoming `Range: bytes=start-end` headers and returns chunked partial streams enabling instantaneous seeking in the GPUI audio scrubber and video player.

### 3.4 WebRTC Signaling & SFU Media Gateway
- **Signaling Exchange**:
  WebSocket carries bidirectional `CallSignalFrame`: `Invite`, `Ringing`, `Accept`, `Offer`, `Answer`, `IceCandidate`, `Hangup`.
- **Integrated STUN/TURN**:
  Embedded STUN packet responder handles UDP binding requests on port 3478. Ephemeral TURN authentication tokens are generated for Symmetric NAT traversal.
- **SFU Group Bridge**:
  For group voice and video stages, the server acts as a Selective Forwarding Unit, receiving individual publisher streams and multiplexing them to subscribers without cross-subscriber mesh retransmission.

### 3.5 Existing Capabilities Subsystem Integration
1. **AI Multi-Agent ACP Runner**:
   Wrapped by `gpui_telegram_agent::acp::runner`. Spawns turns against `mahayana-host-runtime`, streaming thinking tokens to `AgentThinkingBubble` with pulse animations and intermediate tool step receipts.
2. **Global Task Center & 5-Field Cron Engine**:
   Orchestrates background tasks through the 7-state lifecycle FSM (`Pending` $\rightarrow$ `Running` $\rightarrow$ `WaitingAuth` / `WaitingConnection` $\rightarrow$ `Completed` / `Failed` / `Cancelled`). Cron triggers follow standard 5-field syntax with next-run previews and automatic execution receipts sent to linked conversation stages.
3. **Remote Computer Control & CUA**:
   `crates/gpui_telegram_agent::computer::lease` manages exclusive control leases (`ComputerControlLease`). When active, an amber warning banner appears in the conversation, the remote desktop canvas renders live frames, and pressing `Esc` triggers immediate human takeover with $<10$ms latency.
4. **Full-Duplex Talk Mode & Barge-in**:
   `crates/gpui_telegram_agent::talk::vad_bargein` executes voice activity detection on input PCM audio. If user speech is detected while the AI is speaking ($<60$ms threshold), an immediate `barge_in_abort` signal flushes playback buffers and truncates previous turn history to the exact millisecond.

