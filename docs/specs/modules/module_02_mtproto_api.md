# Module 02 Specification: MTProto & API Subsystem (`mtproto & api`)

## 1. Architecture & Subsystem Overview

The MTProto subsystem is the secure transport and cryptographic networking foundation of Telegram Desktop. In the upstream C++ implementation (`Telegram/SourceFiles/mtproto/` and its subdirectories `details/`, `web_proxy/`, and `scheme/`), it encapsulates the entire lifecycle of encrypted client-server interaction:
- **Wire Framing & Obfuscated Transports**: Encapsulating TL-serialized MTProto payloads within TCP (Abridged, Intermediate, Padded Intermediate) and HTTP/HTTPS streams, obfuscated via 64-byte randomized CTR preambles to defeat deep packet inspection (DPI).
- **Cryptographic Key Exchange (Diffie-Hellman & RSA)**: Initiating unencrypted handshake sessions via `req_pq_multi` / `resPQ`, factoring composite modulus $PQ$, verifying server public keys against built-in 2048-bit RSA fingerprints, and computing 2048-bit `AuthKey` secrets alongside ephemeral server salts.
- **Session Multiplexing & Sequential Integrity**: Maintaining client session IDs, monotonically increasing client message IDs ($msg\_id = \lfloor unixtime \rfloor \times 2^{32} + \mu s \times 4$), calculating request sequence numbers (`seq_no`), tracking acknowledgments (`msgs_ack`), container packing (`msg_container`), and automatic resending of unacknowledged RPC queries.
- **DC Options & Multi-Datacenter Routing**: Managing datacenter endpoints (IPv4, IPv6, CDN, test mode), shifted DC indices (`ShiftedDcId`) for download streams, upload pipes, CDN caches, group call voice/video streams, and proxy failovers.
- **RPC Dispatch & Error Handling**: Asynchronous request-response demultiplexing (`rpc_result`, `rpc_error`), handling `FLOOD_WAIT_x`, `FILE_MIGRATE_x`, and transient transport timeouts.

### Subsystem Component Topology

```
┌────────────────────────────────────────────────────────────────────────┐
│                        High-Level API & RPC Caller                     │
│                    (Sender / ConcurrentSender / Facade)                 │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ TL Request / SerializedRequest
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                         MTProto Session Multiplexer                    │
│                           (Session / SessionData)                      │
│  - Message ID Generation               - Container Aggregation         │
│  - Monotonic Sequence Numbers (seq_no)  - ACK Accumulator & Tracking    │
│  - Salt Rotation & Session Resets      - Resend Queue & Deduplication  │
└──────────────────┬─────────────────────────────────┬───────────────────┘
                   │ Plaintext Message               ▲ Decrypted Replies
                   ▼                                 │
┌────────────────────────────────────────────────────────────────────────┐
│                       Cryptographic Engine & AuthKey                   │
│                       (AuthKey / DcKeyCreator / Binder)                │
│  - MTProto 2.0 AES-256-IGE             - MsgKey SHA-256 Verification   │
│  - Diffie-Hellman Key Exchange (DH)    - RSA Signature Checks          │
└──────────────────┬─────────────────────────────────▲───────────────────┘
                   │ Encrypted Packet Payload        │
                   ▼                                 │
┌────────────────────────────────────────────────────────────────────────┐
│                        Connection State Machine                        │
│                 (ConnectionAbstract / Tcp / Http / WebProxy)           │
│  - Obfuscated Preamble (64B CTR)       - Transport Framing             │
│  - TCP Socket Engine                   - HTTP Long-Poll Fallback       │
└──────────────────┬─────────────────────────────────▲───────────────────┘
                   ▼                                 │
              Physical Network / Proxy (SOCKS5 / MTProto Proxy)
```

---

## 2. MTProto Wire Framing & Transport Protocols

MTProto supports multiple transport framing protocols layered above raw TCP or HTTP streams. Each transport format encapsulates MTProto packets (unencrypted handshake packets or AES-256-IGE encrypted payloads).

### 2.1 Transport Framing Modes

1. **Abridged Transport (`0xef`)**:
   - Single-byte header: `0xef` sent immediately upon TCP connection establishment.
   - Per-packet framing:
     - If payload length divided by 4 is $< 0x7f$: 1 byte length ($len / 4$), followed by packet payload.
     - If payload length divided by 4 is $\ge 0x7f$: 1 byte `0x7f` followed by 3 bytes little-endian length ($len / 4$), followed by packet payload.

2. **Intermediate Transport (`0xeeeeebee`)**:
   - 4-byte header: `0xee 0xee 0xee 0xee` sent upon connection.
   - Per-packet framing: 4-byte little-endian length in bytes (including the payload, excluding the 4-byte length prefix), followed by packet payload.

3. **Padded Intermediate Transport (`0xdddddddd`)**:
   - 4-byte header: `0xdd 0xdd 0xdd 0xdd`.
   - Framing matches Intermediate transport, but appends randomized padding (0 to 15 bytes) to obfuscate packet length patterns from network sniffers.

4. **HTTP Long-Poll Transport (`ConnectionHttp`)**:
   - Uses HTTP POST requests targeting `/api` or `/apiw1`.
   - Headers include `Content-Type: application/x-protobuf` or `application/octet-stream`.
   - Body contains raw MTProto frames with 4-byte length prefixes.
   - Employs `http_wait` requests to simulate full-duplex push delivery over unidirectional HTTP links.

5. **WebProxy & MTProto Proxy Transport (`0xeeeeebee` + Obfuscation)**:
   - MTProto Proxy uses 64-byte randomized client preambles encrypted with AES-256-CTR, keyed by the proxy secret and target DC identifier.

### 2.2 Obfuscated Preamble Generation

To evade DPI filtering, TCP connections generate an initial 64-byte randomized buffer:
1. Generate 64 cryptographically secure random bytes.
2. Ensure byte 0 is not `0xef`, and the first 4 bytes do not match `POST`, `GET `, `HEAD`, `0xeeeeebee`, or `0xdddddddd`.
3. Set bytes 56..60 to the selected protocol magic (`0xefefefef` for abridged, `0xeeeeeeee` for intermediate, `0xdddddddd` for padded intermediate).
4. Compute AES-CTR encryption key from bytes 8..40, IV from bytes 40..56.
5. Compute AES-CTR decryption key and IV from reversed slices.
6. Encrypt bytes 56..64 in-place with the encryption key and transmit the entire 64-byte preamble.

```
+-----------------------------------------------------------------------+
| 64-Byte Obfuscation Preamble                                          |
| Bytes 0..7:   Random padding (DPI evasion)                            |
| Bytes 8..39:  Send AES-CTR Key (32 bytes)                             |
| Bytes 40..55: Send AES-CTR IV (16 bytes)                              |
| Bytes 56..59: Protocol Magic (e.g. 0xeeeeeeee or 0xdddddddd) [Encrypted]|
| Bytes 60..61: Target DC ID (int16 little endian) [Encrypted]          |
| Bytes 62..63: Reserved random / flags [Encrypted]                     |
+-----------------------------------------------------------------------+
```

---

## 3. Connection Finite State Machine (FSM)

The connection lifecycle is managed by `AbstractConnection` and specialized by `TcpConnection`, `HttpConnection`, and `WebProxyConnection`.

```
                    ┌─────────────────────────┐
                    │      Disconnected       │
                    └────────────┬────────────┘
                                 │ connectToServer(ip, port, secret)
                                 ▼
                    ┌─────────────────────────┐
                    │       Connecting        │
                    └────────────┬────────────┘
                                 │ Socket Connected & Obfuscation Sent
                                 ▼
                    ┌─────────────────────────┐
                    │       Handshaking       │◄────────┐
                    │     (Fake PQ Probe)     │         │ Re-send PQ
                    └────────────┬────────────┘         │ on Timeout
                                 │ Valid resPQ Received │
                                 ▼                      │
                    ┌─────────────────────────┐         │
             ┌─────►│        Connected        │─────────┘
             │      └──────┬────────────┬─────┘
             │             │            │
    Network  │             │ Heartbeat  │ Network Drop
    Recovered│             │ Timeout    │ or EOF
             │             ▼            ▼
             │      ┌────────────┐┌───────────┐
             └──────┤ Suspended  ││Reconnecting│
                    └────────────┘└───────────┘
```

### 3.1 State Transitions

1. **Disconnected**:
   - Sockets closed, buffers cleared.
   - Entry point: Connection created or explicit `disconnectFromServer()`.

2. **Connecting**:
   - Resolves target IP / port via `DomainResolver` or static DC option.
   - Establishes non-blocking TCP socket or TLS tunnel.
   - Arms `fullConnectTimeout` timer (typically 10-15 seconds).

3. **Handshaking (Fake PQ Probe)**:
   - Generates 64-byte obfuscation preamble and transmits to server.
   - Immediately transmits a lightweight unencrypted `req_pq_multi` probe with a randomized 128-bit nonce.
   - Races multiple parallel test connections (IPv4 TCP, IPv6 TCP, HTTP) across endpoints; the first connection to successfully return a valid `resPQ` reply matching `nonce` is selected as the active transport (`confirmBestConnection`), while lingering test connections are discarded.

4. **Connected**:
   - Bidirectional packet transfer is active.
   - Outgoing packets are formatted, encrypted, framed, and queued for socket transmission.
   - Incoming stream is deframed, verified, and placed into `_receivedQueue`.
   - Periodic keepalive pings (`sendPingByTimer`) are dispatched every 30-60 seconds.

5. **Suspended / Reconnecting**:
   - Triggered upon socket error, EOF, or TCP reset.
   - Triggers exponential backoff retry timer (`_retryTimer`: 1s, 2s, 4s, 8s, up to max 30s).
   - If persistent auth key exists, resumes session state; if session was terminated server-side, initiates session regeneration.

---

## 4. Sessions & Multiplexing

The MTProto session layer maintains cryptographic session continuity across physical socket reconnects.

### 4.1 Session Data Primitives

| Field | Type | Description |
|---|---|---|
| `session_id` | `u64` | 64-bit random identifier generated per client session. Survives socket reconnects, rotated upon explicit session resets or server `bad_server_salt` (code 48). |
| `server_salt` | `u64` | 64-bit salt negotiated during DH key generation or received via `bad_server_salt` / `new_session_created`. Rotated by server approximately every 30 minutes. |
| `seq_no` | `u32` | Packet sequence number within the session. Odd numbers for messages requiring ACK ($2k + 1$), even numbers for non-ACK messages ($2k$). |
| `last_msg_id` | `u64` | Strictly monotonic message timestamp identifier: $\lfloor unixtime \rfloor \times 2^{32} + \mu s \times 4$. Client IDs must be divisible by 4. |

### 4.2 Sequence Number Rule
For every message sent inside a session:
```rust
fn next_seq_no(&mut self, needs_ack: bool) -> u32 {
    if needs_ack {
        let seq = self.messages_counter * 2 + 1;
        self.messages_counter += 1;
        seq
    } else {
        self.messages_counter * 2
    }
}
```

### 4.3 Container Multiplexing (`msg_container`)
When multiple RPC requests, ACKs, or ping messages are ready for transmission, MTProto combines them into a single container message (`0x73f1f8dc`):
- `constructor_id`: `0x73f1f8dc`
- `count`: `int32`
- Followed by `count` consecutive messages:
  - `msg_id`: `int64`
  - `seq_no`: `int32`
  - `bytes`: `int32`
  - `body`: message payload of length `bytes`

Containers cannot be recursively embedded inside other containers.

---

## 5. Cryptographic Handshakes & AuthKey Derivation

MTProto 2.0 uses 2048-bit `AuthKey` secrets derived via the Diffie-Hellman protocol.

### 5.1 DH Key Exchange FSM

```
Client                                                         Server
  │                                                              │
  │─── req_pq_multi(nonce: int128) ─────────────────────────────►│
  │                                                              │
  │◄── resPQ(nonce, server_nonce, pq, fingerprints) ─────────────│
  │                                                              │
  │ [Factor pq = p * q via Pollard's Rho / Trial Division]       │
  │ [Select RSA Public Key matching one of server fingerprints]   │
  │ [Generate new_nonce: int256, compute encrypted data via RSA] │
  │                                                              │
  │─── req_DH_params(nonce, server_nonce, p, q, encrypted_data) ─►│
  │                                                              │
  │◄── server_DH_params_ok(nonce, server_nonce, encrypted_answer)│
  │                                                              │
  │ [Decrypt encrypted_answer with AES-256-IGE using tmp_aes_key]│
  │ [Validate DH parameters: g, dh_prime, g_a]                   │
  │ [Compute random b (2048-bit), g_b = g^b mod dh_prime]        │
  │ [Compute shared key: auth_key = (g_a)^b mod dh_prime]        │
  │                                                              │
  │─── set_client_DH_params(nonce, server_nonce, encrypted_b) ──►│
  │                                                              │
  │◄── dh_gen_ok(nonce, server_nonce, new_nonce_hash1) ──────────│
  │                                                              │
  ▼ [Compute auth_key_id = lower 64 bits of SHA1(auth_key)]      ▼
```

### 5.2 MTProto 2.0 AES-256-IGE Encryption & Key Derivation

For an encrypted packet with `auth_key` (256 bytes):
1. **Plaintext Construction**:
   ```
   plaintext = server_salt (8B) + session_id (8B) + msg_id (8B)
             + seq_no (4B) + length (4B) + message_data (len bytes)
             + padding (12..1024 bytes random, aligned to 16 bytes)
   ```
2. **Message Key Derivation (`msg_key`)**:
   - For client-to-server: $msg\_key = \text{SHA256}(auth\_key[88..120] + plaintext)[0..16]$.
   - For server-to-client: $msg\_key = \text{SHA256}(auth\_key[96..128] + plaintext)[0..16]$.
3. **AES Key & IV Calculation**:
   - $x = 0$ for client sending, $x = 8$ for client receiving.
   - $sha256\_a = \text{SHA256}(msg\_key + auth\_key[x .. x+36])$
   - $sha256\_b = \text{SHA256}(auth\_key[40+x .. 40+x+36] + msg\_key)$
   - $aes\_key = sha256\_a[0..8] + sha256\_b[8..24] + sha256\_a[24..32]$ (32 bytes)
   - $aes\_iv  = sha256\_b[0..8] + sha256\_a[8..24] + sha256\_b[24..32]$ (32 bytes)
4. **Cipher Execution**:
   - Encrypt / Decrypt using AES-256 in Infinite Garble Extension (IGE) mode:
     $c_i = \text{AES\_ECB\_Encrypt}(m_i \oplus c_{i-1}) \oplus m_{i-1}$

---

## 6. DC Options & Dynamic Routing

Telegram operates 5 primary datacenters globally (DC 1 through DC 5), each hosting multiple endpoints with specific capabilities.

### 6.1 Shifted DC ID Schema

To multiplex downloads, uploads, and media streams without head-of-line blocking on a single TCP connection, `ShiftedDcId` applies fixed offsets to bare DC IDs:

```rust
pub const DC_SHIFT: i32 = 10000;

pub fn bare_dc_id(shifted: i32) -> i32 {
    shifted % DC_SHIFT
}

pub fn shift_dc_id(dc_id: i32, shift: i32) -> i32 {
    dc_id + DC_SHIFT * shift
}
```

| Shift Index | Constant | Purpose |
|---|---|---|
| `0x00` | Main DC | Primary interactive user session (updates, dialogs, messages) |
| `0x01` | `kConfigDcShift` | Background configuration & DC options discovery |
| `0x02` | `kLogoutDcShift` | Guest DC session termination |
| `0x03` | `kUpdaterDcShift` | Application autoupdater binary downloads |
| `0x04` | `kExportDcShift` | Account data export sessions |
| `0x06` | `kGroupCallStreamDcShift`| Group voice/video WebRTC stream transport |
| `0x10 .. 0x1F` | `kBaseDownloadDcShift` | Parallelized file download workers (up to 16 streams) |
| `0x20 .. 0x2F` | `kBaseUploadDcShift` | Parallelized file chunk upload workers |

### 6.2 Endpoint Selection Priority
`DcOptions` maintains endpoints filtered by:
1. Active proxy settings (SOCKS5 / MTProto Proxy).
2. IPv6 preference (`use_ipv6` flag).
3. Transport preference (TCP preferred over HTTP; HTTP used only as fallback).
4. Static built-in fallback endpoints for DC 1 through DC 5.

---

## 7. Error Handling, RPC Response Dispatch & ACK Tracking

### 7.1 RPC Error Processing

When a request fails, the server responds with an `rpc_error` packet:
- `error_code`: `int32` (HTTP-aligned: 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, 406 Not Acceptable, 420 Flood, 500 Internal).
- `error_message`: `string` (e.g. `FLOOD_WAIT_14`, `PHONE_CODE_INVALID`, `FILE_MIGRATE_2`).

Helper functions categorize errors:
- `is_flood_error(err)`: Matches `FLOOD_WAIT_(\d+)` or `FLOOD_PREMIUM_WAIT_(\d+)` and extracts backoff delay.
- `is_temporary_error(err)`: True for $code < 0$, $code \ge 500$, or flood errors.
- `is_file_migrate(err)`: Directs the download worker to redirect to the specified target DC ID.

### 7.2 ACK Tracking & Message ID Deduplication

`ReceivedIdsManager` tracks received message IDs:
- Maintains a fixed-capacity circular/sorted buffer of recent IDs (default capacity 400).
- Rejects duplicate packets (`Result::Duplicate`).
- Detects stale/replayed packets older than the lowest recorded ID (`Result::TooOld`).
- Batches outgoing ACKs into `msgs_ack` packets containing up to 8192 message IDs per payload.


---

## 8. API Wrap Architecture

In Telegram Desktop (`Telegram/SourceFiles/apiwrap.h` and `Telegram/SourceFiles/apiwrap.cpp`), `ApiWrap` serves as the central high-level API coordinator. It sits directly above the raw MTProto transport layer and orchestrates the entire fleet of specialized domain API managers.

### 8.1 Subsystem Decomposition and Ownership Model

```
                         ┌───────────────────────────┐
                         │       Session / App       │
                         └─────────────┬─────────────┘
                                       │
                                       ▼
                         ┌───────────────────────────┐
                         │          ApiWrap          │
                         │ (Coordinator & Dispatcher)│
                         └──────┬─────────────┬──────┘
                                │             │
         ┌──────────────────────┼─────────────┼──────────────────────┐
         ▼                      ▼             ▼                      ▼
┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐
│    ApiUpdates    │  │  ApiSending &    │  │ ApiChatFilters & │  │  ApiAuthorizations│
│(PTS Sync / Diff) │  │  ApiEditing      │  │ ApiChatLinks     │  │  & CloudPassword │
└──────────────────┘  └──────────────────┘  └──────────────────┘  └──────────────────┘
         │                      │             │                      │
         └──────────────────────┼─────────────┼──────────────────────┘
                                │             │
                                ▼             ▼
                         ┌───────────────────────────┐
                         │   MTProto RPC Sender      │
                         │   & Request Manager       │
                         └───────────────────────────┘
```

The 58 sub-APIs in `gpui_telegram_proto::api` map 1:1 with the upstream C++ subsystem managers:
- **Session & Identity**: `ApiAuthorizations`, `ApiCloudPassword`, `ApiConfirmPhone`, `ApiSelfDestruct`, `ApiWebsites`.
- **Updates & State Synchronization**: `ApiUpdates`, `ApiFilterUpdates`, `ApiReadMetrics`, `ApiUnreadThings`, `ApiViews`.
- **Messaging & Search**: `ApiSending`, `ApiSendProgress`, `ApiEditing`, `ApiMessagesSearch`, `ApiMessagesSearchMerged`, `ApiSingleMessageSearch`, `ApiTextEntities`.
- **Peer Management & Privacy**: `ApiPeerSearch`, `ApiPeerColors`, `ApiPeerPhoto`, `ApiUserNames`, `ApiUserPrivacy`, `ApiGlobalPrivacy`, `ApiSensitiveContent`, `ApiBlockedPeers`.
- **Groups, Channels & Invites**: `ApiChatParticipants`, `ApiChatInvite`, `ApiChatLinks`, `ApiInviteLinks`, `ApiChatFilters`, `ApiChatFiltersRemoveManager`, `ApiCommunities`.
- **Media, Stickers & Reactions**: `ApiMedia`, `ApiTogglingMedia`, `ApiAttachedStickers`, `ApiStickersCreator`, `ApiReactionsNotifySettings`, `ApiWhoReacted`, `ApiPolls`.
- **Monetization & Analytics**: `ApiPremium`, `ApiPremiumOption`, `ApiCredits`, `ApiCreditsHistoryEntry`, `ApiEarn`, `ApiStatistics`, `ApiStatisticsSender`, `ApiStatisticsDataDeserialize`.
- **Interactive Extensions & Bots**: `ApiBot`, `ApiComposeWithAi`, `ApiTodoLists`, `ApiRichTasks`, `ApiTranscribes`, `ApiRingtones`, `ApiShareThemes`, `ApiSuggestPost`, `ApiReport`, `ApiHash`, `ApiCommon`.

### 8.2 Architectural Principles in Rust / GPUI

1. **Explicit Strongly-Typed Identifiers**: Every API struct operates with concrete identifier types (`UserId`, `ChatId`, `ChannelId`, `MsgId`, `PeerId`, `TopicId`) from `gpui_telegram_core::types`, preventing integer ambiguity errors.
2. **Deterministic State Transitions**: State modifications produce explicit event variants rather than hidden side-effects, allowing GPUI view models to re-render reactively.
3. **Zero Allocation Fallbacks**: Query methods and cache checks avoid heap cloning whenever borrowed references suffice.
4. **Transport Decoupling**: API managers construct abstract RPC envelopes that can serialize either to MTProto binary frames or TDRP self-hosted payloads without changing domain models.

---

## 9. RPC Request Lifecycles & Handlers

### 9.1 Request Lifecycle State Machine

Each outbound RPC request transitions through a strict finite state machine:

```
  ┌───────────┐
  │  Created  │ ── (enqueue) ──► ┌───────────┐
  └───────────┘                  │  Queued   │
                                 └─────┬─────┘
                                       │ (sent to wire)
                                       ▼
  ┌───────────┐                  ┌───────────┐
  │ Cancelled │ ◄── (cancel) ─── │ InFlight  │
  └───────────┘                  └─────┬─────┘
                                       │
                    ┌──────────────────┴──────────────────┐
                    ▼                                     ▼
             ┌─────────────┐                       ┌─────────────┐
             │  Completed  │                       │   Failed    │
             └─────────────┘                       └──────┬──────┘
                                                          │ (retry / backoff)
                                                          ▼
                                                   ┌─────────────┐
                                                   │   Retrying  │
                                                   └─────────────┘
```

1. **Created**: Request payload initialized with unique 64-bit `req_id`, target DC, and timeout duration.
2. **Queued**: Placed into the priority dispatch queue; higher priority requests (interactive user actions like typing, reading, sending) preempt background sync requests.
3. **InFlight**: Serialized into an MTProto container, assigned an incremental sequence number, and transmitted. ACK timer starts.
4. **Completed**: Server returns `rpc_result`; payload is deserialized into domain struct and delivered to caller channel/callback.
5. **Failed / Retrying**: On network timeout, `FLOOD_WAIT`, or temporary server error (`500 INTERNAL`), the request either re-enqueues with exponential backoff or fails permanently.

### 9.2 Request Priority Tiers

| Priority Tier | RPC Methods | Timeout | Max Retries |
|---|---|---|---|
| **Critical Interactive** | `messages.sendMessage`, `messages.sendMedia`, `messages.editMessage` | 15s | 5 |
| **High Interactive** | `messages.readHistory`, `messages.sendReaction`, `account.updateStatus` | 10s | 3 |
| **Normal Navigation** | `messages.getHistory`, `contacts.search`, `messages.getDiscussionMessage` | 20s | 3 |
| **Background Sync** | `updates.getDifference`, `updates.getChannelDifference`, `help.getConfig` | 45s | 10 |
| **Bulk Transfer** | `upload.getFile`, `upload.saveFilePart`, `upload.saveBigFilePart` | 60s | 8 |

---

## 10. Updates Dispatcher & PTS Sequence Alignment

The `ApiUpdates` subsystem implements the updates alignment algorithm specified in upstream `Telegram/SourceFiles/api/api_updates.cpp` and `Telegram/SourceFiles/data/data_pts_waiter.h`.

### 10.1 PTS (Persistent Tracking Sequence) Invariants

Updates modifying messages or user state carry a sequence tracker `pts` and change count `pts_count`:
- Let $P_{\text{local}}$ be the client's current PTS.
- Let $P_{\text{update}}$ be the incoming update's PTS.
- Let $C_{\text{update}}$ be the incoming update's PTS count.
- Invariant: $P_{\text{prev}} = P_{\text{update}} - C_{\text{update}}$.
- If $P_{\text{prev}} == P_{\text{local}}$: Contiguous sequence. Apply update immediately and set $P_{\text{local}} = P_{\text{update}}$.
- If $P_{\text{prev}} < P_{\text{local}}$: Old/duplicate update. Discard update (or apply if idempotent) without advancing $P_{\text{local}}$.
- If $P_{\text{prev}} > P_{\text{local}}$: **Sequence Gap Detected**. Buffer update in `PtsWaiter` and schedule `updates.getDifference`.

### 10.2 Channel Range Difference Alignment

Channels and supergroups have independent, isolated PTS sequences (`pts` per `channel_id`). When a channel update arrives with a PTS gap:
1. `ApiUpdates` checks if a difference request is already pending for that channel.
2. If not, it dispatches `updates.getChannelDifference(channel, filter, pts, limit)`.
3. If the server responds with `updates.channelDifferenceEmpty`, channel PTS is caught up.
4. If the server responds with `updates.channelDifferenceTooLong`, local message history is invalidated and reloaded from the latest messages.

```rust
pub enum DifferenceSyncOutcome {
    UpToDate { pts: u64 },
    AppliedUpdates { pts: u64, messages_count: usize },
    GapDetected { missing_range: std::ops::RangeInclusive<u64> },
    HistoryResetRequired { new_pts: u64 },
}
```

---

## 11. Reaction & Sticker RPC Endpoints

### 11.1 Message Reactions Subsystem

Reaction operations are coordinated through `ApiReactionsNotifySettings` and `ApiWhoReacted`:
- **Send Reaction**: Dispatches `messages.sendReaction(peer, msg_id, reaction, big)` where `reaction` can be:
  - Standard Unicode emoji (e.g. `👍`, `❤️`, `🔥`).
  - Telegram Custom Emoji Document ID (`u64`).
  - Paid reaction (`stars` count for Telegram Stars reactions).
- **Who Reacted Cursor Pagination**: `messages.getMessageReactionsList` with offset cursor, filtering by specific reaction emoji or returning all reactions.
- **Reaction Notifications**: Gated by `ApiReactionsNotifySettings`, controlling whether reactions trigger system notifications, sound, or preview banners.

### 11.2 Custom Sticker Creation & Media Pipelines

`ApiStickersCreator` and `ApiAttachedStickers` manage sticker set authoring:
- **Validation**:
  - Static stickers: PNG or WEBP, exact dimension $512 \times 512$ or one side 512 and other $<512$, max file size 512 KB.
  - Animated stickers: TGS format (Lottie vector profile), max 64 KB, 60 FPS, max duration 3 seconds.
  - Video stickers: WEBP / VP9 WebM with alpha channel, max 256 KB, max duration 3 seconds.
- **RPC Lifecycle**:
  1. `upload.saveFilePart` uploads sticker asset chunks.
  2. `stickers.createStickerSet` initializes set metadata (title, short_name, emojis).
  3. `stickers.addStickerToSet` appends individual stickers with associated emoji triggers.

---

## 12. Chat Participant & Admin APIs

### 12.1 Bitfield Rights Enforcement

Upstream Telegram uses 32-bit bitmasks for chat permissions (`ChatAdminRights` and `ChatBannedRights`), fully modeled in `ApiChatParticipants`:

```rust
pub struct ChatAdminRights {
    pub change_info: bool,
    pub post_messages: bool,
    pub edit_messages: bool,
    pub delete_messages: bool,
    pub ban_users: bool,
    pub invite_users: bool,
    pub pin_messages: bool,
    pub add_admins: bool,
    pub anonymous: bool,
    pub manage_call: bool,
    pub manage_topics: bool,
    pub post_stories: bool,
    pub edit_stories: bool,
    pub delete_stories: bool,
}

pub struct ChatBannedRights {
    pub view_messages: bool,
    pub send_messages: bool,
    pub send_media: bool,
    pub send_stickers: bool,
    pub send_gifs: bool,
    pub send_games: bool,
    pub send_inline: bool,
    pub embed_links: bool,
    pub send_polls: bool,
    pub change_info: bool,
    pub invite_users: bool,
    pub pin_messages: bool,
    pub manage_topics: bool,
    pub send_photos: bool,
    pub send_videos: bool,
    pub send_roundvideos: bool,
    pub send_audios: bool,
    pub send_voices: bool,
    pub send_docs: bool,
    pub send_plain: bool,
}
```

### 12.2 Invite Links & Chat Folders

- **Invite Links**: Managed by `ApiChatLinks` and `ApiInviteLinks`, supporting expiration timestamps, max usage limits, admin approval queues, and paid recurring subscription pricing.
- **Chat Filters**: Managed by `ApiChatFilters` and `ApiChatFiltersRemoveManager`, supporting include/exclude peer sets, folder icon identifiers, pinned chat lists, and shareable filter invite links (`tg://addlist?chatlist=...`).

---

## 13. Error Mapping & Rate Limiting

### 13.1 Leaky-Bucket Rate Limiter

To comply with Telegram server rate limits without triggering `420 FLOOD_WAIT`, `ApiCommon::RateLimiter` enforces client-side smoothing:
- Outbound messages: max 30 messages/second globally; max 1 message/second per private peer.
- Channel posts: max 20 posts/minute.
- Reactions: max 3 reaction toggles per second per chat.

```rust
pub struct RateLimiter {
    pub capacity: f64,
    pub tokens: f64,
    pub refill_rate_per_sec: f64,
    pub last_refill: i64,
}
```

### 13.2 FloodWait Resolution

When an RPC returns `FLOOD_WAIT_X`:
1. The error message is parsed using `parse_flood_wait_seconds(error_message)` -> `Some(X)`.
2. The affected method endpoint enters a suspended state for $X$ seconds.
3. Callers with UI feedback display countdown dialogs or throttle progress indicators.
4. Non-interactive requests automatically reschedule after the backoff window.

---

## 14. Self-Hosted TDRP Server Integration Contracts

Under the project mandate, the client must seamlessly operate either against official Telegram MTProto datacenters or against a self-hosted Rust server (`gpui_telegram_server` via TDRP - Telegram Desktop Rust Protocol).

### 14.1 Protocol Equivalence Matrix

| Telegram MTProto RPC | Self-Hosted TDRP Message | Payload Format |
|---|---|---|
| `messages.sendMessage` | `TdrpSendMessage` | Protobuf / JSON over WebSocket TLS |
| `messages.getHistory` | `TdrpGetHistory` | Fast range slice query over SQLite |
| `updates.getDifference` | `TdrpSyncGetDifference` | High-efficiency PTS delta stream |
| `messages.sendReaction` | `TdrpReactionEvent` | Real-time broadcast fanout |
| `channels.getParticipants` | `TdrpGetParticipants` | Paginated member metadata |

### 14.2 Seamless Switching via Protocol Adapter

`ApiWrap` supports a transparent `TransportKind`:
- `TransportKind::Mtproto`: Serializes to binary TL frames, routes via MTProto session multiplexer to DC endpoints.
- `TransportKind::Tdrp`: Serializes to `WsClientFrame` / TDRP command structures, routes via persistent WebSocket connection to `gpui_telegram_server`.

Both backends emit uniform `UpdateEvent` and `PtsUpdate` streams into `gpui_telegram_proto::sync`, ensuring that the GPUI desktop interface operates identically regardless of whether connected to cloud Telegram or the local private server.

