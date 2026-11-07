# Specification 03: Rust Server Architecture & Protocol Interface Contracts

**Document ID**: `SPEC-03-SERVER-ARCHITECTURE-CONTRACTS`  
**Revision**: `3.0.0` (Milestone 1, Explorer Gen 3)  
**Date**: `2026-11-06`  
**Status**: Authoritative Technical Specification & Server Blueprint  
**Target Repository**: `/Users/bhrum/teamwork_projects/gpui_telegram`  
**Target Path**: `docs/specs/03-rust-server-architecture-contracts.md`  

---

## 1. Executive Summary & Architectural Mandates

This specification defines the complete, production-grade architecture, subsystem interfaces, database transaction models, and network protocols for the **independent, self-hosted Rust backend server (`gpui_telegram_server`)**.

### 1.1 Strict Compliance with Mandatory Architectural Directives
1. **Full Inheritance of Existing Rust Crates (Mandate #1)**:
   - The server builds directly upon the mature Rust runtime engines in `inherited/`:
     * `native/mahayana-messaging`: Core messaging pipeline, local SQLite storage engine, access control, and signaling.
     * `source/host`: AI Turn-Taking ACP runner (`production_turn_agent_owner.rs`), Global Task Center daemon (`background_work.rs`), and MCP relay.
     * `source/node-agent-coordinator`: WebAuthn passkey authentication, remote computer CUA proxy, and TDRP endpoints.
   - Zero duplicate mock or facade code: The server orchestrates and exposes these battle-tested crates over Axum HTTP/REST, WebSockets, and WebRTC.
2. **Telegram 1:1 Parity & Independent Deployment**:
   - Operates completely independently on user-owned hardware without relying on external Telegram MTProto servers.
   - Provides 100% equivalent behavior for authentication, PTS sequence synchronization, real-time message fanout, chunked media streaming, and voice/video call signaling.
3. **Source-File Ledger Governance (Mandates #2 & #4)**:
   - All server capabilities and wire protocols correspond 1:1 with upstream `tdesktop/` C++ endpoints, OpenClaw gateway contracts, and inherited Rust implementations as mapped in the Source-File Parity Ledger.

## 2. Server System Topology & Tokio/Axum Architecture

```
+---------------------------------------------------------------------------------------+
|                               GPUI Desktop Client (Rust)                              |
|   52px NavRail | 288px ChatSidebar | SumTree Message Stage | CanonicalComposer Dock  |
+------------------------------------+--------------------------------------------------+
                                     |
               HTTP/REST (Media, Range Streaming) | WebSocket (RPC & PTS Sync Frames)
               WebRTC (VoIP, Video, VNC Screen)   |
                                     v
+---------------------------------------------------------------------------------------+
|                         gpui_telegram_server (Axum + Tokio)                           |
|                                                                                       |
|  +----------------------------------+     +----------------------------------------+  |
|  | Presentation & Gateway Tier      |     | Sequence Sync Engine (PTS Model)       |  |
|  | - Axum Router & Tower Middleware |     | - Monotonic PTS Counter per User/Chat  |  |
|  | - WebSocket Hub & 30s Heartbeat  |     | - SyncGetDifference Query Handler      |  |
|  | - Token Auth & Rate Limiter      |     | - Gap Detection & Snapshot Fallback    |  |
|  +----------------------------------+     +----------------------------------------+  |
|                                                                                       |
|  +----------------------------------+     +----------------------------------------+  |
|  | Auth & Session Subsystem         |     | High-Performance Messaging Pipeline    |  |
|  | - WebAuthn / Passkeys (FIDO2)    |     | - Atomic DB Writes & client_nonce Dedup|  |
|  | - Argon2id Password / Ed25519    |     | - Soft-Deletes & Tombstone Broadcast   |  |
|  | - Multi-Device Session Registry  |     | - Forum Topics & Thread Hierarchies    |  |
|  +----------------------------------+     +----------------------------------------+  |
|                                                                                       |
|  +----------------------------------+     +----------------------------------------+  |
|  | Media Storage & Range Streaming  |     | Tantivy Full-Text Search Engine        |  |
|  | - 512KB-1MB Resumable Chunk Sink |     | - Embedded Pure-Rust Lucene Equivalent |  |
|  | - SHA-256 CAS File Store         |     | - Sub-ms Message & Entity Queries      |  |
|  | - HTTP 206 Partial Content Stream|     | - Snippet Extraction & Highlighting    |  |
|  +----------------------------------+     +----------------------------------------+  |
|                                                                                       |
|  +----------------------------------+     +----------------------------------------+  |
|  | WebRTC Signaling & SFU Gateway   |     | Inherited Runtime Engines              |  |
|  | - SDP Offer/Answer & ICE Trickle |     | - native/mahayana-messaging (Engine)   |  |
|  | - Integrated STUN/TURN Token Gen |     | - source/host (ACP Runner, Tasks FSM)  |  |
|  | - Multi-Party SFU Bridge         |     | - source/node-agent-coordinator (TDRP) |  |
|  +----------------------------------+     +----------------------------------------+  |
|                                                                                       |
|  +---------------------------------------------------------------------------------+  |
|  | Persistence Layer: SQLite (WAL Mode) / PostgreSQL Pool & Tantivy Index Files    |  |
|  +---------------------------------------------------------------------------------+  |
+---------------------------------------------------------------------------------------+
```

## 3. Spec 03 Source-File Equivalence Ledger (服务端源码文件对等账本)

| 服务端子系统 | 上游 Telegram Desktop 源码锚点 (`tdesktop/`) | 上游 OpenClaw 源码锚点 (`openclaw/`) | 已继承的成熟 Rust 模块路径 (`fabushi-unified-design-20261105`) | 目标 GPUI 服务端实现路径 (`crates/gpui_telegram_server/`) | 核心结构体 / 契约接口 |
|---|---|---|---|---|---|
| **1. Auth & Sessions (认证与会话)** | `Telegram/SourceFiles/api/api_authorizations.cpp`<br>`Telegram/SourceFiles/core/two_factor_auth.cpp`<br>`Telegram/SourceFiles/webauthn/webauthn_box.cpp`<br>`Telegram/SourceFiles/data/data_session.cpp` | `src/secrets/`<br>`src/pairing/`<br>`extensions/device-pair/` | `source/node-agent-coordinator/src/webauthn.rs`<br>`source/node-agent-coordinator/src/tdrp/settings_cloud_passkeys.rs`<br>`source/host/src/device_pairing_tokens.rs`<br>`source/host/src/device_pairing_storage.rs`<br>`native/mahayana-messaging/src/access.rs`<br>`native/mahayana-messaging/src/device.rs` | `crates/gpui_telegram_core/src/auth.rs`<br>`crates/gpui_telegram_server/src/auth/session_manager.rs`<br>`crates/gpui_telegram_server/src/auth/passkey_verifier.rs` | `SessionTokenPayload`<br>`DeviceSessionRecord`<br>`WebAuthnCredentialStore`<br>`accountScopeGeneration` |
| **2. Sequence Sync Engine (PTS 同步引擎)** | `Telegram/SourceFiles/api/api_updates.cpp`<br>`Telegram/SourceFiles/api/api_updates.h`<br>`Telegram/SourceFiles/storage/storage_account.cpp` | `packages/gateway-protocol/`<br>`crates/openclaw-gateway-client/` | `native/mahayana-messaging/src/service_tdrp_sync.rs`<br>`native/mahayana-messaging/src/engine.rs`<br>`native/mahayana-messaging/src/service_tdrp_queries.rs` | `crates/gpui_telegram_proto/src/sync.rs`<br>`crates/gpui_telegram_server/src/sync/pts_engine.rs`<br>`crates/gpui_telegram_client/src/sync/coordinator.rs` | `SyncGetDifferenceRequest`<br>`SyncGetDifferenceResponse`<br>`UpdateEvent`<br>`PtsSequenceCounter` |
| **3. Messaging Pipeline (消息管线与去重)** | `Telegram/SourceFiles/api/api_sending.cpp`<br>`Telegram/SourceFiles/history/history_item.cpp`<br>`Telegram/SourceFiles/data/data_forum_topic.cpp` | `src/chat/`<br>`src/auto-reply/`<br>`packages/normalization-core/` | `native/mahayana-messaging/src/message.rs`<br>`native/mahayana-messaging/src/conversation.rs`<br>`native/mahayana-messaging/src/engine_tdrp_dialogs.rs`<br>`native/mahayana-messaging/src/engine_tdrp_topics_channels.rs`<br>`source/node-agent-coordinator/src/tdrp/forum_topics.rs` | `crates/gpui_telegram_proto/src/message.rs`<br>`crates/gpui_telegram_server/src/messaging/pipeline.rs`<br>`crates/gpui_telegram_server/src/messaging/dedup.rs` | `MessageItem`<br>`MessageContent`<br>`client_nonce`<br>`TopicDescriptor`<br>`TombstoneRecord` |
| **4. Tantivy Search Engine (全文本搜索)** | `Telegram/SourceFiles/storage/storage_databases.cpp`<br>`Telegram/SourceFiles/dialogs/dialogs_key.cpp` | `packages/memory-host-sdk/`<br>`src/memory/` | `native/mahayana-messaging/src/search.rs`<br>`source/host/src/storage/store_db.rs` | `crates/gpui_telegram_server/src/search/tantivy_indexer.rs`<br>`crates/gpui_telegram_server/src/search/query_service.rs` | `TantivySearchSchema`<br>`IndexCommand`<br>`SearchSnippetHighlight`<br>`FacetedMediaQuery` |
| **5. Chunked Media & Streaming (分块存储与串流)** | `Telegram/SourceFiles/storage/file_upload.cpp`<br>`Telegram/SourceFiles/storage/file_download.cpp`<br>`Telegram/SourceFiles/data/data_streaming.cpp` | `packages/media-core/`<br>`src/media/` | `native/mahayana-messaging/src/blob_store.rs`<br>`native/mahayana-messaging/src/media.rs`<br>`native/mahayana-messaging/src/media_cache.rs`<br>`source/host/src/attachment_paths.rs`<br>`source/host/src/media_mime.rs` | `crates/gpui_telegram_media/src/chunked_upload.rs`<br>`crates/gpui_telegram_server/src/media/storage_service.rs`<br>`crates/gpui_telegram_server/src/media/range_stream.rs` | `ChunkUploadInit`<br>`ChunkUploadAppend`<br>`CasBlobStorage`<br>`Http206RangeResponder` |
| **6. WebSocket Realtime Hub (实时网关与心跳)** | `Telegram/SourceFiles/mtproto/connection.cpp`<br>`Telegram/SourceFiles/mtproto/session.cpp` (替代为 Axum WS) | `src/gateway/`<br>`packages/gateway-protocol/` | `source/host/src/gateway_server.rs`<br>`source/host/src/gateway_protocol.rs`<br>`native/mahayana-messaging/src/gateway.rs` | `crates/gpui_telegram_proto/src/ws.rs`<br>`crates/gpui_telegram_server/src/ws/gateway_hub.rs`<br>`crates/gpui_telegram_server/src/ws/heartbeat.rs` | `WsClientFrame`<br>`WsServerFrame`<br>`HeartbeatPingPong`<br>`ClientSocketRegistry` |
| **7. WebRTC Signaling & SFU (通话信令与媒体网关)** | `Telegram/SourceFiles/calls/calls_call.cpp`<br>`Telegram/SourceFiles/calls/calls_instance.cpp`<br>`Telegram/SourceFiles/calls/calls_group_call.cpp`<br>`Telegram/SourceFiles/calls/calls_controller_webrtc.cpp` | `src/talk/talk-session-controller.ts`<br>`src/meeting-bot/` | `native/mahayana-messaging/src/realtime.rs`<br>`native/mahayana-messaging/src/signaling.rs`<br>`native/mahayana-messaging/src/signaling_server.rs`<br>`source/node-agent-coordinator/src/tdrp/calls_group_calls.rs` | `crates/gpui_telegram_webrtc/src/signaling.rs`<br>`crates/gpui_telegram_server/src/webrtc/signaling_hub.rs`<br>`crates/gpui_telegram_server/src/webrtc/sfu_bridge.rs` | `CallSignalFrame`<br>`SdpNegotiation`<br>`IceCandidateTrickle`<br>`SfuGroupBridge` |

---

## 4. Subsystem Deep-Dive Designs & Interface Contracts

### 4.1 Subsystem 1: Auth & Sessions Design
- **WebAuthn / Passkeys**: Uses `webauthn-rs` library. Generates FIDO2 challenge, registers credential public key, validates assertion signatures.
- **Passwords**: Argon2id (`m=65536, t=3, p=4`), zero plain-text storage.
- **Session Tokens**: Ed25519 signed cryptographically unforgeable session tokens containing:
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct SessionTokenPayload {
      pub session_id: Uuid,
      pub user_id: Uuid,
      pub device_id: String,
      pub client_type: String, // "gpui_desktop_macos"
      pub account_generation: u64,
      pub created_at_ms: i64,
      pub expires_at_ms: i64,
  }
  ```
- **Session Registry**: Tracks active devices, IP addresses, location, and last active timestamps. Supports instantaneous remote revocation.
- **Account Scope Generation Fencing**:
  When an account logs out or switches, `account_scope_generation` is bumped on server and client. All active background tasks, media streams, and WebSockets tied to older generations are rejected immediately.

### 4.2 Subsystem 2: Atomic PTS Sequence Sync Engine
The **Presentation Sequence (PTS)** model is the cornerstone of Telegram's guaranteed delivery without data loss.

```sql
BEGIN IMMEDIATE;
UPDATE user_sequence SET pts = pts + 1 WHERE user_id = ? RETURNING pts;
INSERT INTO pts_updates (user_id, pts, date, update_type, update_payload)
VALUES (?, ?, ?, ?, ?);
COMMIT;
```

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncGetDifferenceRequest {
    pub pts: u64,
    pub limit: u32,
    pub date: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncGetDifferenceResponse {
    pub new_pts: u64,
    pub has_more: bool,
    pub messages: Vec<MessageItem>,
    pub other_updates: Vec<UpdateEvent>,
    pub users: Vec<UserSummary>,
    pub chats: Vec<ChatSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum UpdateEvent {
    NewMessage(MessageItem),
    EditMessage(MessageItem),
    DeleteMessages { chat_id: Uuid, message_ids: Vec<Uuid>, pts: u64 },
    ReadHistoryOutbox { chat_id: Uuid, max_read_id: Uuid, pts: u64 },
    ReadHistoryInbox { chat_id: Uuid, max_read_id: Uuid, pts: u64 },
    ReactionUpdated { chat_id: Uuid, message_id: Uuid, reactions: Vec<ReactionPill>, pts: u64 },
    PinnedDialogsUpdated { folder_id: u32, order: Vec<Uuid>, pts: u64 },
    UserPresence { user_id: Uuid, is_online: bool, last_seen_ms: i64 },
    UserTyping { chat_id: Uuid, user_id: Uuid, action: TypingAction },
    TaskStatusChanged { task_id: Uuid, status: TaskStatus, progress: Option<f32>, pts: u64 },
}
```

### 4.3 Subsystem 3: Messaging Pipeline
- **Atomic Writes**: Messages are inserted into SQLite WAL mode alongside updates log in a single transaction.
- **Deduplication via `client_nonce`**: Every client send includes a random `client_nonce: u64`. The database enforces a `UNIQUE(sender_id, client_nonce)` constraint for 24 hours. Retries return the existing message without duplicate posting.
- **Soft-Deletes & Tombstones**: Deleted messages set `deleted_at_ms = now()`, retain message ID, and emit `UpdateDeleteMessages` with a new `pts`.
- **Forum Topics**: Chats support `topic_id: Option<i64>`. Topics are lightweight virtual threads with unread counters, pinned status, and separate breadcrumbs.

### 4.4 Subsystem 4: Tantivy Full-Text Search Engine
- **Schema**:
  ```rust
  let mut schema_builder = Schema::builder();
  let f_message_id = schema_builder.add_text_field("message_id", STRING | STORED);
  let f_chat_id = schema_builder.add_text_field("chat_id", STRING | STORED);
  let f_sender_id = schema_builder.add_text_field("sender_id", STRING);
  let f_date = schema_builder.add_i64_field("date", INDEXED | FAST | STORED);
  let f_content = schema_builder.add_text_field("content", TEXT | STORED);
  let f_media_type = schema_builder.add_text_field("media_type", STRING | FAST);
  let schema = schema_builder.build();
  ```
- **Indexing**: Async background channel receives index commands from message pipeline. Commits batched every 500ms.
- **Search Capabilities**:
  * Global search across all dialogs.
  * In-chat scoped search (`chat_id = ?`).
  * Faceted media filtering (`has:photo`, `has:document`, `has:voice`, `has:link`).
  * Fast phrase queries and CJK tokenized queries with sub-millisecond latency.
  * Snippet highlighting returning snippet with `<mark>` matched ranges.

### 4.5 Subsystem 5: Chunked Media Storage & HTTP 206 Streaming
- **Chunk Upload Protocol**:
  1. `POST /api/v1/media/upload/init`: Body: `{ filename, size_bytes, mime_type, total_chunks, sha256_hash }`. Returns `upload_id`.
  2. `PUT /api/v1/media/upload/:upload_id/chunk/:chunk_index`: Binary payload (512 KB to 1 MB). Server writes to staging file. If connection breaks, client resumes from last uploaded `chunk_index`.
  3. `POST /api/v1/media/upload/:upload_id/complete`: Verifies overall SHA-256 hash. Moves file to CAS directory (`/data/media/{hash[0..2]}/{hash}`).
  4. Auto-processing on completion:
     - Voice files (`audio/ogg`, `audio/opus`): decodes and generates 100-sample 5-bit waveform binary (`Vec<u8>`).
     - Images: generates 320px WebP thumbnail.
     - Videos: extracts first frame thumbnail and duration/resolution metadata.
- **HTTP 206 Range Streaming**:
  - Endpoint `GET /api/v1/media/stream/:file_id` honors standard `Range: bytes=start-end` headers for instant audio scrubbing and video playback.

### 4.6 Subsystem 6: WebSocket Realtime Hub
- **Endpoint**: `ws://<server>/ws/v1?token=<session_token>`
- **Framing**: Binary (MessagePack or protobuf) and JSON supported.
- **Heartbeat Contract**:
  - Client sends `{"type": "Ping", "seq": 101}` every 30 seconds.
  - Server replies `{"type": "Pong", "seq": 101, "server_time_ms": 1793948000000}`.
  - Socket closed if no message received within 60 seconds.

### 4.7 Subsystem 7: WebRTC Signaling & SFU Media Gateway
- **Signaling Frames**:
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  #[serde(tag = "action", content = "payload")]
  pub enum CallSignalFrame {
      Invite { call_id: Uuid, callee_id: Uuid, has_video: bool },
      Ringing { call_id: Uuid },
      Accept { call_id: Uuid, sdp_answer: String },
      Offer { call_id: Uuid, sdp_offer: String },
      IceCandidate { call_id: Uuid, candidate: String, sdp_mid: Option<String>, sdp_mline_index: Option<u16> },
      Hangup { call_id: Uuid, reason: String },
      ScreenShareStart { call_id: Uuid },
      ScreenShareStop { call_id: Uuid },
  }
  ```
- **Integrated STUN/TURN**: Built-in STUN packet responder on UDP port 3478, and ephemeral TURN token generation for NAT traversal.
- **SFU Group Bridge**: Selective Forwarding Unit topology for group voice and video stages, avoiding $N \times N$ peer mesh saturation.

