# Module 03 Specification: Storage & Data Subsystem (`storage_data`)

## Overview

Module 03 (`storage & data`) provides the complete persistence, caching, cryptographic file subsystem, and domain object graph for the Telegram Desktop Rust/GPUI rewrite across all 147 target files in `crates/gpui_telegram_client/src/storage/` (25 files) and `src/data/` (122 files).

This consolidated specification encompasses three core sub-domains:
1. **Storage Subsystem (`storage`)**: Cryptographic local storage, atomic TDF serialization, SQLite WAL database, sparse media caching, and chunked background transfer pipelines.
2. **Data Peers & Session Subsystem (`data_peers`)**: Peer identities (`PeerId`), user/chat/channel entities, permissions, histories/messages indexing, folder filters, and session lifecycle.
3. **Data Media & Content Subsystem (`data_media_content`)**: Documents, progressive photo sizing, audio waveforms, video notes, forum topics, ephemeral stories, stickers/reactions, and business integrations.

---

# Part I: Storage Subsystem (`storage`)

# Module 03 Specification: Storage & Data Subsystem (`storage_data`)

## 1. Module Overview & Architectural Objectives

Module 03 (`storage & data`) provides the complete persistence, caching, and cryptographic file subsystem of the Telegram Desktop Rust/GPUI rewrite. In the upstream C++ implementation (`Telegram/SourceFiles/storage/`), this subsystem is responsible for:
1. **Cryptographic Local Storage**: Protecting local cache and credentials using user passcodes and salts via PBKDF2-HMAC-SHA512, Argon2id, and Scrypt KDFs, and encrypting local blob descriptors with AES-IGE cipher.
2. **Atomic TDF File Serialization**: Managing atomic file transactions (`TDF$` magic header, app versioning, 16-byte MD5 integrity trailers, and safe `*s` / fallback `*0`, `*1` swap files).
3. **Multi-Account & Session Storage**: Orchestrating account-level databases, draft preservation, stickers, GIFs, recent emojis/hashtags, and media file locations.
4. **SQLite Local Database Engine**: Providing high-performance structured caching, full-text message indexing, and blob storage with PRAGMA WAL mode.
5. **Sparse Message & Media Caching**: Managing sparse slices of message IDs across 13 shared media types and user photo histories with fast windowed queries (`aroundId`, `limitBefore`, `limitAfter`).
6. **Background Task & Transfer Pipeline**: Managing multi-threaded task queues for media preparation, chunked MTProto downloading (128 KB chunks with DC session balancing and CDN redirection), and chunked uploading (with 30 MB threshold for big files).

In this Rust/GPUI architecture, `gpui_telegram_client::storage` re-implements these storage primitives without placeholder stubs or fake boilerplate. It integrates seamlessly with:
- `gpui_telegram_core`: Configuration, versioning, digest utilities.
- `gpui_telegram_proto`: MTProto types, serialization formats.
- `fabushi-messaging-core`: Chat entities, drafts, media descriptors, and TDRP projections.
- `mahayana-node-agent-coordinator`: Cloud settings and passkeys.
- `mahayana-host-runtime`: Secure storage isolation and local executor databases.

---

## 2. Upstream C++ Class Hierarchy & Rust GPUI Target Mapping

The storage subsystem comprises 47 upstream C++ source files mapped into 25 dedicated Rust modules under `crates/gpui_telegram_client/src/storage/`:

| Upstream C++ Source Files | Upstream C++ Classes / Types | Target Rust File | Target Rust Symbol | Responsibilities |
|---|---|---|---|---|
| `storage/details/storage_file_utilities.h`, `.cpp` | `WriteManager`, `AsyncWriteManager`, `PasscodeKdf`, `FileReadDescriptor`, `EncryptedDescriptor`, `FileWriteDescriptor`, `ReadFile`, `WriteFile` | `crates/gpui_telegram_client/src/storage/details/storage_file_utilities.rs` | `StorageFileUtilitiesDetails` | Atomic file I/O, `TDF$` magic verification, MD5 checksum trailers, AES-IGE encryption/decryption, Argon2id/PBKDF2 passcode derivation. |
| `storage/details/storage_settings_scheme.h`, `.cpp` | `ReadSettingsContext`, `ReadSetting`, `dbiKey`, `dbiUser`, `dbiSessionSettings`, `dbiCacheSettings`, `dbiEncrypted` | `crates/gpui_telegram_client/src/storage/details/storage_settings_scheme.rs` | `StorageSettingsSchemeDetails` | Block-based binary settings serialization schema, block identifiers, legacy and modern settings migration. |
| `storage/storage_account.h`, `.cpp` | `Storage::Account`, `MessageDraft`, `MessageDraftSource`, `WalletEngineValue`, `readFileLocation`, `writeDrafts` | `crates/gpui_telegram_client/src/storage/storage_account.rs` | `StorageAccount` | Per-account storage manager, SQLite local cache handle, draft persistence, stickers/emojis cache, media file locations. |
| `storage/storage_domain.h`, `.cpp` | `Storage::Domain`, `PasscodeDerivation`, `PasscodeVerification`, `StartResult`, `SetPasscodeResult`, `DeriveOnWorker` | `crates/gpui_telegram_client/src/storage/storage_domain.rs` | `StorageDomain` | Multi-account domain coordination, passcode derivation worker, single-use verification nonces, wallet keyring storage. |
| `storage/storage_facade.h`, `.cpp` | `Storage::Facade`, `Facade::Impl` | `crates/gpui_telegram_client/src/storage/storage_facade.rs` | `StorageFacade` | Unified facade interfacing shared media indices and user photos caches with UI controllers. |
| `storage/storage_file_lock.h` | `Storage::FileLock` | `crates/gpui_telegram_client/src/storage/storage_file_lock.rs` | `StorageFileLock` | Cross-platform file lock interface preventing multiple concurrent instances from corrupting local storage. |
| `storage/storage_file_lock_posix.cpp` | `FileLock::Lock`, `FileLock::Descriptor`, `FileLock::LockingPid`, `KillProcess` | `crates/gpui_telegram_client/src/storage/storage_file_lock_posix.rs` | `StorageFileLockPosix` | POSIX file locking implementation via `fcntl` (`F_SETLK`, `F_WRLCK`), PID conflict detection and process termination. |
| `storage/storage_file_lock_win.cpp` | `CloseProcesses`, Windows RestartManager API | `crates/gpui_telegram_client/src/storage/storage_file_lock_win.rs` | `StorageFileLockWin` | Windows file locking implementation using Windows RestartManager (`RmStartSession`, `RmRegisterResources`, `RmShutdown`). |
| `storage/storage_cloud_blob.h`, `.cpp` | `Storage::CloudBlob::Blob`, `BlobState`, `BlobLoader`, `UnpackBlob` | `crates/gpui_telegram_client/src/storage/storage_cloud_blob.rs` | `StorageCloudBlob` | Cloud blob cache downloader, state machine (`Available`, `Ready`, `Active`, `Failed`, `Loading`), tar/zip archive unpacking. |
| `storage/storage_folder_archive.h`, `.cpp` | `ArchiveEntry`, `ArchiveEntries`, `ArchiveWriteResult`, `PrepareFolderArchive`, `WriteArchive` | `crates/gpui_telegram_client/src/storage/storage_folder_archive.rs` | `StorageFolderArchive` | Packaging folders/files into compressed zip archives for sending, size estimation, streaming progress callbacks. |
| `storage/storage_media_prepare.h`, `.cpp` | `MimeDataState`, `ComputeMimeDataState`, `PhotoEditorMedia`, `PrepareMediaList`, `ComputeVideoDetails` | `crates/gpui_telegram_client/src/storage/storage_media_prepare.rs` | `StorageMediaPrepare` | Drag-and-drop MIME analysis, photo/video metadata extraction, album grouping, image modifications application. |
| `storage/storage_shared_media.h`, `.cpp` | `SharedMedia`, `SharedMediaType`, `SharedMediaKey`, `SharedMediaQuery`, `SharedMediaSliceUpdate` | `crates/gpui_telegram_client/src/storage/storage_shared_media.rs` | `StorageSharedMedia` | Indexed cache for 13 shared media types (Photos, Videos, Files, Audio, Voice, Links, etc.) partitioned by peer and topic. |
| `storage/storage_sparse_ids_list.h`, `.cpp` | `SparseIdsList`, `SparseIdsListQuery`, `SparseIdsListResult`, `SparseIdsSliceUpdate`, `Slice` | `crates/gpui_telegram_client/src/storage/storage_sparse_ids_list.rs` | `StorageSparseIdsList` | Sparse ID list data structure with non-contiguous range slices, slice merging, and bidirectional window queries. |
| `storage/storage_user_photos.h`, `.cpp` | `UserPhotos`, `UserPhotosKey`, `UserPhotosQuery`, `UserPhotosResult`, `UserPhotosSliceUpdate` | `crates/gpui_telegram_client/src/storage/storage_user_photos.rs` | `StorageUserPhotos` | User profile photo collection cache, back photo navigation, slice updates, and ordering. |
| `storage/download_manager_mtproto.h`, `.cpp` | `DownloadManagerMtproto`, `DownloadMtprotoTask`, `DcBalanceData`, `CdnFileHash` | `crates/gpui_telegram_client/src/storage/download_manager_mtproto.rs` | `DownloadManagerMtproto` | MTProto chunked download scheduler (128 KB chunks), DC connection balancing, CDN redirect handling, and hash verification. |
| `storage/file_download.h`, `.cpp` | `FileLoader`, `FailureReason`, `StorageImageSaved`, `CreateFileLoader` | `crates/gpui_telegram_client/src/storage/file_download.rs` | `FileDownload` | Base file loader abstraction, memory-to-disk thresholds (10 MB max in memory), local cache checks, progress streams. |
| `storage/file_download_mtproto.h`, `.cpp` | `FileDownloadMtproto` | `crates/gpui_telegram_client/src/storage/file_download_mtproto.rs` | `FileDownloadMtproto` | Specialized file loader fetching media from Telegram MTProto servers via chunked tasks. |
| `storage/file_download_web.h`, `.cpp` | `FileDownloadWeb` | `crates/gpui_telegram_client/src/storage/file_download_web.rs` | `FileDownloadWeb` | Specialized file loader fetching external media via HTTP/HTTPS with redirect tracking. |
| `storage/file_upload.h`, `.cpp` | `Uploader`, `UploadedMedia`, `UploadFileProgress`, `UploadSecureDone` | `crates/gpui_telegram_client/src/storage/file_upload.rs` | `FileUpload` | Upload manager for media and documents, 30 MB big file threshold, parallel DC upload sessions, and cancellation. |
| `storage/localimageloader.h`, `.cpp` | `Task`, `TaskQueue`, `TaskQueueWorker`, `FileLoadTask`, `FilePrepareResult`, `SendingAlbum` | `crates/gpui_telegram_client/src/storage/localimageloader.rs` | `Localimageloader` | Multi-threaded media worker queue, thumbnail generation, voice waveform generation, album batching. |
| `storage/localstorage.h`, `.cpp` | `Local::start`, `writeSettings`, `writeBackground`, `writeTheme`, `pushRecentLanguage` | `crates/gpui_telegram_client/src/storage/localstorage.rs` | `Localstorage` | High-level facade for local application state, wallpapers, themes, language packs, and legacy migrations. |
| `storage/serialize_common.h`, `.cpp` | `Serialize::ByteArrayWriter`, `ByteArrayReader`, `writeColor`, `readColor`, `bytes` | `crates/gpui_telegram_client/src/storage/serialize_common.rs` | `SerializeCommon` | Binary stream reader/writer utilities, Qt-compatible serialization primitives for strings, colors, byte arrays, timestamps. |
| `storage/serialize_document.h`, `.cpp` | `Serialize::Document`, `StickerSetInfo` | `crates/gpui_telegram_client/src/storage/serialize_document.rs` | `SerializeDocument` | Serialization and deserialization of document metadata, sticker sets, audio tags, and thumbnails. |
| `storage/serialize_peer.h`, `.cpp` | `Serialize::writePeer`, `readPeer`, `writeImageLocation`, `readImageLocation` | `crates/gpui_telegram_client/src/storage/serialize_peer.rs` | `SerializePeer` | Serialization and deserialization of peer identifiers (users, chats, channels) and image locations. |
| `storage/streamed_file_downloader.h`, `.cpp` | `StreamedFileDownloader` | `crates/gpui_telegram_client/src/storage/streamed_file_downloader.rs` | `StreamedFileDownloader` | Streaming downloader supporting progressive playback of audio and video while download is in progress. |

---

## 3. Storage Security Architecture: Encryption Keys & Passcode KDF

```
  Passcode Input (User UTF-8 String)
                │
                ▼
  Prehash: SHA-512(salt || passcode || salt)
                │
                ├───► KDF Strategy Selection:
                │     ├─── [Kind = 1]: Argon2id (memory=64MB, time=8, lanes=1)
                │     ├─── [Kind = 2]: Scrypt (N=131072, r=8, p=1)
                │     └─── [Legacy]:   PBKDF2-HMAC-SHA512 (100,000 iterations)
                │
                ▼
    Derived Local Key (32-byte AuthKey)
                │
                ▼
  AES-IGE Local Encryption / Decryption
  [Plaintext] ──► [4-byte Len || Data || Random Pad (16-byte align)]
                ──► SHA-1 Digest (First 16 bytes = IV)
                ──► AES-IGE Encrypt with Local Key
                ──► Output: [16-byte IV || Encrypted Payload]
```

### 3.1 TDF File Container Specification
All Telegram Desktop data files (`data`, `settings`, `map0`, `map1`, `shortcuts`, etc.) adhere to the `TDF$` binary container format:
1. **Magic Header**: Exactly 4 bytes ASCII: `0x54 0x44 0x46 0x24` (`TDF$`). Files lacking this magic are rejected immediately.
2. **App Version**: 4 bytes little-endian signed 32-bit integer (`i32`). Rejects files created by newer versions than current binary.
3. **Encrypted or Plain Data Stream**: Sequence of serialized blocks or AES-IGE encrypted byte chunks.
4. **MD5 Integrity Trailer**: 16 bytes MD5 hash computed over:
   $$\text{MD5}(\text{data} \parallel \text{len}(\text{data}) \parallel \text{version} \parallel \text{"TDF\$"})$$
   During reading, any mismatch between computed MD5 and stored 16-byte trailer aborts loading and triggers fallback file recovery.

### 3.2 FileKey Naming Conventions
Local file keys are 64-bit integers (`u64`). They are mapped to filenames via `to_file_part(val)`:
```rust
pub fn to_file_part(mut val: u64) -> String {
    let mut result = String::with_capacity(16);
    let hex_chars = b"0123456789ABCDEF";
    for _ in 0..16 {
        let v = (val & 0x0F) as usize;
        result.push(hex_chars[v] as char);
        val >>= 4;
    }
    result
}
```
Atomic file saving uses three postfix variants:
- `<key>s`: Safe save (modern atomic write: write to temp file then rename with overwrite).
- `<key>0`: Primary data file (legacy).
- `<key>1`: Backup data file (legacy alternate).

---

## 4. SQLite Local Database Architecture

Account databases use SQLite with WAL mode (`PRAGMA journal_mode = WAL`, `PRAGMA synchronous = NORMAL`). The database maintains local fast-lookup tables:

```sql
-- Local file locations cache
CREATE TABLE IF NOT EXISTS file_locations (
    media_key TEXT PRIMARY KEY,
    file_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    modified_time INTEGER NOT NULL
);

-- Drafts per history peer
CREATE TABLE IF NOT EXISTS message_drafts (
    peer_id INTEGER NOT NULL,
    topic_root_id INTEGER NOT NULL DEFAULT 0,
    reply_to_id INTEGER,
    text_content TEXT NOT NULL,
    entities_json TEXT,
    cursor_position INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL,
    PRIMARY KEY (peer_id, topic_root_id)
);

-- Stickers and emoji sets
CREATE TABLE IF NOT EXISTS sticker_sets (
    set_id INTEGER PRIMARY KEY,
    access_hash INTEGER NOT NULL,
    title TEXT NOT NULL,
    short_name TEXT NOT NULL,
    count INTEGER NOT NULL,
    hash INTEGER NOT NULL,
    installed_order INTEGER NOT NULL DEFAULT 0
);

-- Recent search queries and hashtags
CREATE TABLE IF NOT EXISTS recent_hashtags (
    tag TEXT PRIMARY KEY,
    use_count INTEGER NOT NULL DEFAULT 1,
    last_used INTEGER NOT NULL
);

-- Media cache key-value store
CREATE TABLE IF NOT EXISTS cache_entries (
    cache_tag INTEGER NOT NULL,
    cache_key BLOB PRIMARY KEY,
    file_offset INTEGER NOT NULL,
    file_size INTEGER NOT NULL,
    access_timestamp INTEGER NOT NULL
);
```

---

## 5. Sparse Message & Media Cache Implementation

The `SparseIdsList` manages non-overlapping sorted intervals of message IDs. When queries arrive for a message window (`aroundId`, `limitBefore`, `limitAfter`), the index returns:
- `count`: Total count of items matching the query (if known).
- `skippedBefore`: How many items exist before the returned slice.
- `skippedAfter`: How many items exist after the returned slice.
- `messageIds`: Contiguous slice of matching message IDs.

The 13 `SharedMediaType` categories are:
1. `Photo` (0)
2. `Video` (1)
3. `PhotoVideo` (2)
4. `MusicFile` (3)
5. `File` (4)
6. `VoiceFile` (5)
7. `Link` (6)
8. `ChatPhoto` (7)
9. `RoundVoiceFile` (8)
10. `GIF` (9)
11. `RoundFile` (10)
12. `Pinned` (11)
13. `Poll` (12)

---

## 6. Verification & Quality Gates

1. **Gate 1**: Zero boilerplate structures (`pub label: String, pub value: i64`).
2. **Gate 2**: Genuine cryptographic implementation: SHA-512, MD5, PBKDF2, Argon2id, AES-IGE encryption and decryption verified with roundtrip tests.
3. **Gate 3**: Real SQLite database engine: tables created, rows inserted, queried, updated, and deleted.
4. **Gate 4**: Atomic file read/write with `TDF$` validation, checksum validation, and recovery.
5. **Gate 5**: Clean build garbage compliance under `AGENTS.md`.


---

# Part II: Data Peers & Session Subsystem (`data_peers`)

# Module 03 Specification: Data Peers & Session Subsystem (`data_peers`)

## 1. Module Overview & Architectural Objectives

Module 03 (`storage & data`) contains the core domain data structures of Telegram Desktop. Specifically, the **Data Peers & Session subsystem** represents the heart of Telegram's object graph:
- **Peer Identity (`PeerId`)**: A unified 64-bit integer encoding users, group chats, broadcast channels, and synthetic/fake peers with bit-level discriminator shifts.
- **Peer Entities (`PeerData`, `UserData`, `ChatData`, `ChannelData`)**: Full polymorphic domain models tracking profile information, names, photos, access hashes, custom emoji statuses, stars ratings, birthday, wallet card, member counts, and detailed bitflags.
- **Admin Rights & Restrictions (`ChatAdminRights`, `ChatRestrictions`)**: Precise bitfield representations of administrative powers and channel/group participant restrictions matching Telegram MTProto schemas.
- **Histories & Messages Management (`Histories`, `History`, `HistoryMessages`, `MsgId`, `UnreadValue`)**: Session-wide index of message streams, read state tracking (unread counts, mentions, marks), and message life cycle.
- **Folders & Chat Filters (`Folder`, `ChatFilter`, `ChatFilterFlags`)**: Custom folder filters, rule-based inclusion/exclusion (contacts, non-contacts, groups, channels, bots, muted, read), pinned chats per filter, and folder hierarchy.
- **Data Session Management (`Data::Session`)**: Central repository and lifecycle manager for all peer entities, coordinating user updates, participant changes, group calls, credits, and session-level caches.

This Rust implementation in `crates/gpui_telegram_client/src/data/` replaces all dummy boilerplate structures (`{ id, label, value }`, dummy timestamp setters) with authentic, C++-faithful domain types and algorithms.

---

## 2. Upstream C++ Source Mapping & Target Symbols

The following table details the primary upstream C++ files in `Telegram/SourceFiles/data/` and their corresponding Rust targets in `gpui_telegram_client`:

| Upstream C++ Source | Key C++ Classes / Structs | Target Rust File | Target Rust Symbol | Subsystem / Responsibility |
|---|---|---|---|---|
| `data_peer_id.h`, `.cpp` | `PeerId`, `UserId`, `ChatId`, `ChannelId`, `FakeChatId`, `BareId` | `crates/gpui_telegram_client/src/data/data_peer_id.rs` | `DataPeerId` | 64-bit PeerId bitshift discriminator, legacy/modern serializer |
| `data_peer.h`, `.cpp` | `PeerData`, `RestrictionCheckResult`, `UnavailableReason`, `AllowedReactions` | `crates/gpui_telegram_client/src/data/data_peer.rs` | `DataPeer` | Base peer model, color indices, emoji status, stories state |
| `data_user.h`, `.cpp` | `UserData`, `UserDataFlag`, `BotInfo`, `StarRefProgram` | `crates/gpui_telegram_client/src/data/data_user.rs` | `DataUser` | User domain model, names, phone, bot info, flags, access hash |
| `data_user_names.h`, `.cpp` | `Usernames`, `UsernamesInfo` | `crates/gpui_telegram_client/src/data/data_user_names.rs` | `DataUserNames` | Multiple usernames, active/editable username resolver |
| `data_user_photos.h`, `.cpp` | `UserPhotos`, `PhotoData` | `crates/gpui_telegram_client/src/data/data_user_photos.rs` | `DataUserPhotos` | Profile photos history, photo list pagination |
| `data_user_wallet_card.h`, `.cpp` | `WalletCard`, `WalletCardType` | `crates/gpui_telegram_client/src/data/data_user_wallet_card.rs` | `DataUserWalletCard` | TON Wallet card and crypto address binding |
| `data_chat.h`, `.cpp` | `ChatData`, `ChatDataFlag`, `ChatParticipants` | `crates/gpui_telegram_client/src/data/data_chat.rs` | `DataChat` | Legacy small group chat model, participants, creator, invite link |
| `data_chat_participant_status.h`, `.cpp` | `ChatAdminRight`, `ChatRestriction`, `ChatAdminRightsInfo`, `ChatRestrictionsInfo` | `crates/gpui_telegram_client/src/data/data_chat_participant_status.rs` | `DataChatParticipantStatus` | MTProto admin bitmask and restricted permissions bitmask |
| `data_channel.h`, `.cpp` | `ChannelData`, `ChannelDataFlag`, `MegagroupInfo` | `crates/gpui_telegram_client/src/data/data_channel.rs` | `DataChannel` | Channel / Supergroup model, member counts, slowmode, bot commands |
| `data_channel_admins.h`, `.cpp` | `ChannelAdmins`, `Admin` | `crates/gpui_telegram_client/src/data/data_channel_admins.rs` | `DataChannelAdmins` | Channel admin list manager and permission lookup |
| `data_channel_earn.h`, `.cpp` | `ChannelEarn`, `EarnInt` | `crates/gpui_telegram_client/src/data/data_channel_earn.rs` | `DataChannelEarn` | Channel ad revenue sharing & star monetization |
| `data_histories.h`, `.cpp` | `Histories`, `History` | `crates/gpui_telegram_client/src/data/data_histories.rs` | `DataHistories` | Session-wide peer message history manager, read inbox |
| `data_history_messages.h`, `.cpp` | `HistoryMessages`, `HistoryItem` | `crates/gpui_telegram_client/src/data/data_history_messages.rs` | `DataHistoryMessages` | Ordered message collection for a peer, bidirectional lookup |
| `data_messages.h`, `.cpp` | `Messages`, `MessageIdsList` | `crates/gpui_telegram_client/src/data/data_messages.rs` | `DataMessages` | Message ID list operations, batch selection, delete ops |
| `data_msg_id.h`, `.cpp` | `MsgId`, `FullMsgId`, `ServerMsgId` | `crates/gpui_telegram_client/src/data/data_msg_id.rs` | `DataMsgId` | Message identifier types, client vs server IDs |
| `data_unread_value.rs` | `UnreadValue`, `Dialogs::UnreadState` | `crates/gpui_telegram_client/src/data/data_unread_value.rs` | `DataUnreadValue` | Unread badge counts, muted flag, mention counts |
| `data_folder.h`, `.cpp` | `Folder`, `FolderId` | `crates/gpui_telegram_client/src/data/data_folder.rs` | `DataFolder` | Archive and user folder containers for chats |
| `data_chat_filters.h`, `.cpp` | `ChatFilter`, `ChatFilterFlags`, `ChatFilterTitle` | `crates/gpui_telegram_client/src/data/data_chat_filters.rs` | `DataChatFilters` | Chat tab folder filters, rules evaluation, inclusion/exclusion |
| `data_session.h`, `.cpp` | `Data::Session`, `SessionSettings` | `crates/gpui_telegram_client/src/data/data_session.rs` | `DataSession` | Central domain coordinator storing peer entities, histories, folders |
| `data_lastseen_status.h`, `.cpp` | `LastseenStatus`, `LastseenType` | `crates/gpui_telegram_client/src/data/data_lastseen_status.rs` | `DataLastseenStatus` | Online / last seen time, privacy obfuscation |
| `data_birthday.h`, `.cpp` | `Birthday`, `BirthdayFormat` | `crates/gpui_telegram_client/src/data/data_birthday.rs` | `DataBirthday` | User birth date with day/month/year parsing and formatted strings |
| `data_boosts.h`, `.cpp` | `BoostsStatus`, `MyBoost` | `crates/gpui_telegram_client/src/data/data_boosts.rs` | `DataBoosts` | Channel and supergroup level boosts, unrestrict perks |
| `data_credits.h`, `.cpp` | `CreditsStatus`, `CreditsHistoryEntry` | `crates/gpui_telegram_client/src/data/data_credits.rs` | `DataCredits` | Telegram Stars balance, transaction ledger |
| `data_credits_earn.h`, `.cpp` | `CreditsEarnStatistics` | `crates/gpui_telegram_client/src/data/data_credits_earn.rs` | `DataCreditsEarn` | Bot & channel stars revenue statistics and withdrawal state |
| `data_passkey_deserialize.h`, `.cpp` | `PasskeyData`, `PasskeyCredential` | `crates/gpui_telegram_client/src/data/data_passkey_deserialize.rs` | `DataPasskeyDeserialize` | WebAuthn passkey registration payload deserializer |
| `data_report.h`, `.cpp` | `ReportReason`, `ReportInput` | `crates/gpui_telegram_client/src/data/data_report.rs` | `DataReport` | Spam / abuse reporting categories and commentary |
| `data_pts_waiter.h`, `.cpp` | `PtsWaiter`, `PtsState` | `crates/gpui_telegram_client/src/data/data_pts_waiter.rs` | `DataPtsWaiter` | Updates PTS gap detection and synchronization waiter |
| `data_cloud_themes.h`, `.cpp` | `CloudThemes`, `CloudTheme` | `crates/gpui_telegram_client/src/data/data_cloud_themes.rs` | `DataCloudThemes` | Cloud chat themes, daylight and night mode palettes |
| `data_search_calendar.h`, `.cpp` | `SearchCalendar`, `CalendarDay` | `crates/gpui_telegram_client/src/data/data_search_calendar.rs` | `DataSearchCalendar` | Date-based chat history message search navigator |
| `data_search_controller.h`, `.cpp` | `SearchController`, `SearchQuery` | `crates/gpui_telegram_client/src/data/data_search_controller.rs` | `DataSearchController` | Chat and global message search controller |
| `data_community.h`, `.cpp` | `CommunityData`, `CommunityTopic` | `crates/gpui_telegram_client/src/data/data_community.rs` | `DataCommunity` | Telegram Communities and multi-chat federations |
| `data_groups.h`, `.cpp` | `Groups`, `GroupInfo` | `crates/gpui_telegram_client/src/data/data_groups.rs` | `DataGroups` | Message album groupings and clustered media |
| `data_subscriptions.h`, `.cpp` | `Subscription`, `SubscriptionPlan` | `crates/gpui_telegram_client/src/data/data_subscriptions.rs` | `DataSubscriptions` | Channel paid subscription plans and active subscriber status |
| `data_premium_limits.h`, `.cpp` | `PremiumLimits`, `PremiumLimitType` | `crates/gpui_telegram_client/src/data/data_premium_limits.rs` | `DataPremiumLimits` | Telegram Premium quotas (channels, pinned, bio, upload size) |
| `data_premium_subscription_option.h`, `.cpp` | `PremiumSubscriptionOption` | `crates/gpui_telegram_client/src/data/data_premium_subscription_option.rs` | `DataPremiumSubscriptionOption` | Premium subscription pricing tiers (monthly, yearly, gift) |

---

## 3. PeerId Architecture & Discriminator Logic

In Telegram Desktop, a `PeerId` is a 64-bit integer packing both the entity type and the entity's 48-bit identifier:
```
 63       56 55                   48 47                                0
┌───────────┬───────────────────────┬───────────────────────────────────┐
│ Reserved  │   Type Shift (8-bit)  │       Bare ID (48-bit Mask)       │
└───────────┴───────────────────────┴───────────────────────────────────┘
```
- **Type Shifts**:
  - `UserId`: `Shift = 0` (User IDs occupy `0x0000_0000_0000` .. `0x0000_FFFF_FFFF_FFFF`)
  - `ChatId`: `Shift = 1` (`0x0001_0000_0000_0000` .. `0x0001_FFFF_FFFF_FFFF`)
  - `ChannelId`: `Shift = 2` (`0x0002_0000_0000_0000` .. `0x0002_FFFF_FFFF_FFFF`)
  - `FakeChatId`: `Shift = 0x7F` (`0x007F_0000_0000_0000` .. `0x007F_FFFF_FFFF_FFFF`)
- **Masks**:
  - `kChatTypeMask = 0x0000_FFFF_FFFF_FFFF` (48-bit bare identifier mask)
  - `kReservedBit = 0x80` (bit 63 used for serialization mode disambiguation)
- **Serialization**:
  - **Modern**: `id.value | (0x80 << 48)`
  - **Legacy**: Encoded using 32-bit masks where Chat has `0x1000_0000_0000`, Channel has `0x2000_0000_0000`, User has `0x0000_0000_0000`, and Fake has `0xF000_0000_0000`.

---

## 4. User, Chat, and Channel Domain Models

### 4.1 UserData
- `id: PeerId`, `access_hash: u64`
- `first_name: String`, `last_name: String`, `phone: String`, `usernames: Vec<String>`
- `flags: UserDataFlags`:
  - `Contact`, `MutualContact`, `Deleted`, `Verified`, `Scam`, `Fake`, `BotInlineGeo`, `Blocked`
  - `HasPhoneCalls`, `PhoneCallsPrivate`, `Support`, `SelfUser`, `Premium`, `VoiceMessagesForbidden`
- `lastseen: LastseenStatus`: `OnlineUntil(i32)`, `Recently`, `LastWeek`, `LastMonth`, `Hidden`
- `birthday: Option<Birthday>`
- `bot_info: Option<BotInfo>`: `commands: Vec<BotCommand>`, `description: String`, `reads_all_history: bool`

### 4.2 ChatData (Small Basic Group)
- `id: PeerId` (Shift = 1)
- `title: String`, `photo_id: u64`, `creator_id: UserId`, `date: i64`, `member_count: usize`
- `participants: Vec<UserId>`, `admins: Vec<UserId>`
- `flags: ChatDataFlags`: `Left`, `Creator`, `Deactivated`, `Forbidden`, `CallActive`, `NoForwards`
- `admin_rights: ChatAdminRights`, `default_restrictions: ChatRestrictions`
- `migrated_to: Option<ChannelId>` (when upgraded to supergroup)

### 4.3 ChannelData (Supergroup or Broadcast Channel)
- `id: PeerId` (Shift = 2)
- `title: String`, `username: Option<String>`, `access_hash: u64`
- `flags: ChannelDataFlags`:
  - `Left`, `Creator`, `Forbidden`, `Megagroup`, `Broadcast`, `Gigagroup`, `Verified`, `Scam`, `Fake`
  - `Signatures`, `PreHistoryHidden`, `SlowmodeEnabled`, `NoForwards`, `Forum`, `AntiSpam`
- `members_count: i32`, `admins_count: i32`, `slowmode_seconds: i32`
- `admin_rights: ChatAdminRights`, `default_restrictions: ChatRestrictions`
- `megagroup_info: Option<MegagroupInfo>`: tracks last authors, custom member ranks, bot commands

---

## 5. Admin Rights & Participant Restrictions Bitfields

### ChatAdminRight (22 bits):
```rust
ChangeInfo             = 1 << 0,
PostMessages           = 1 << 1,
EditMessages           = 1 << 2,
DeleteMessages         = 1 << 3,
BanUsers               = 1 << 4,
InviteByLinkOrAdd      = 1 << 5,
PinMessages            = 1 << 7,
AddAdmins              = 1 << 9,
Anonymous              = 1 << 10,
ManageCall             = 1 << 11,
Other                  = 1 << 12,
ManageTopics           = 1 << 13,
PostStories            = 1 << 14,
EditStories            = 1 << 15,
DeleteStories          = 1 << 16,
ManageDirect           = 1 << 17,
ManageRanks            = 1 << 18,
ProcessJoinRequests    = 1 << 19,
ManageLinkedPeers      = 1 << 20,
ManageWelcomeMessages  = 1 << 21,
```

### ChatRestriction (29 bits):
```rust
ViewMessages       = 1 << 0,
SendStickers       = 1 << 3,
SendGifs           = 1 << 4,
SendGames          = 1 << 5,
SendInline         = 1 << 6,
EmbedLinks         = 1 << 7,
SendPolls          = 1 << 8,
ChangeInfo         = 1 << 10,
AddParticipants    = 1 << 15,
PinMessages        = 1 << 17,
CreateTopics       = 1 << 18,
SendPhotos         = 1 << 19,
SendVideos         = 1 << 20,
SendVideoMessages  = 1 << 21,
SendMusic          = 1 << 22,
SendVoiceMessages  = 1 << 23,
SendFiles          = 1 << 24,
SendOther          = 1 << 25,
EditRank           = 1 << 26,
SendReactions      = 1 << 27,
ManageLinkedPeers  = 1 << 28,
```

---

## 6. Chat Filters & Folders Subsystem

### ChatFilter Evaluation Algorithm:
A chat with `History` matches `ChatFilter` if:
1. It is explicitly listed in `always` list $\implies$ **Match (True)**.
2. It is explicitly listed in `never` list $\implies$ **Exclude (False)**.
3. If `flags` contains `NoArchived` and the chat is archived $\implies$ **Exclude (False)**.
4. If `flags` contains `NoRead` and the chat has 0 unread $\implies$ **Exclude (False)**.
5. If `flags` contains `NoMuted` and the chat is muted $\implies$ **Exclude (False)**.
6. The chat entity type matches filter flags:
   - User contact $\land$ `Contacts` flag $\implies$ Match
   - User non-contact $\land$ `NonContacts` flag $\implies$ Match
   - Group chat $\land$ `Groups` flag $\implies$ Match
   - Channel $\land$ `Channels` flag $\implies$ Match
   - Bot user $\land$ `Bots` flag $\implies$ Match

---

## 7. Verification Strategy & Test Plan

1. **Unit Tests**:
   - `test_peer_id_encoding_and_shifts`: Validates `UserId`, `ChatId`, `ChannelId`, `FakeChatId`, bit extraction, modern and legacy serialization.
   - `test_user_data_flags_and_name_formatting`: Validates full name formatting, username handling, phone number, and bitflags.
   - `test_chat_and_channel_admin_rights`: Validates admin rights bitfields, privilege checking (`can_delete_messages`, `can_ban_users`, `can_post_messages`), and restrictions.
   - `test_chat_filter_matching`: Validates `always`, `never`, unread rules, muted rules, and entity type matching.
   - `test_histories_and_unread_accounting`: Validates message insertion, inbox read pointer updates, and unread counts.
   - `test_data_session_peer_repository`: Validates peer lookup, user and chat insertion, channel migration, and lifecycle caching.
2. **Integration Gate**:
   - Execute `cargo test -p gpui_telegram_client --test data_peers_subsystem_tests`.
   - Run `python3 scripts/clean_build_garbage.py`.
   - Ensure all boilerplate patterns in the assigned files are eradicated (zero instances of Pattern A/B/C/D).


---

# Part III: Data Media & Content Subsystem (`data_media_content`)

# Module 03 Specification: Data Media & Content Subsystem (`data_media_content`)

## 1. Module Overview & Architectural Objectives

The Data Media & Content subsystem (`crates/gpui_telegram_client/src/data/`) forms the rich multimedia, interactive content, and business layer of Telegram Desktop. In the upstream C++ implementation (`Telegram/SourceFiles/data/`), this subsystem is responsible for:

1. **Document & Media Pipeline**:
   - `DocumentData` and `DocumentMedia`: Encapsulates documents, audio tracks, voice notes, round video messages, animations (GIF/MP4), and custom stickers.
   - MIME type resolution and format detection (audio, video, PDF, archives, images).
   - Audio waveform representation (5-bit quantized amplitude bars), duration, and playback metadata.
   - Video qualities list, streaming preload offsets, and dimensions.
2. **Photo Management & Progressive Sizing**:
   - `PhotoData` and `PhotoMedia`: Multi-resolution image pipeline supporting progressive loading across `Small` (100px), `Thumbnail` (320px), and `Large` (1280px) levels.
   - Animated video avatars (looping MP4s with playback timestamps and inline thumbnails).
   - Local disk cache keys, cloud file references, and upload/download progress state machines.
3. **Forum & Threaded Topics**:
   - `Forum`, `ForumTopic`, and `ForumTopicIcons`: Hierarchical topic threads for supergroups.
   - `kGeneralId = 1` reserved General topic, custom icon document IDs, and Telegram color palettes (`colorId` 0..5).
   - Per-topic unread counters, mention tracking, reaction badges, and pinned topic ordering.
4. **Ephemeral Stories & Media Overlays**:
   - `Stories`, `StoryData`, and `StoryViews`: Ephemeral status media with 24-hour expiration timers (`expires <= now`).
   - Granular privacy tiers (`Public`, `CloseFriends`, `Contacts`, `SelectedContacts`).
   - Interactive media areas (`LocationArea`, `ReactionArea`, `ChannelPostArea`, `UrlArea`, `WeatherArea`).
   - Viewer analytics: view counts, reaction counts, forward tallies, and viewer user lists.
5. **Interactive Polls & Quizzes**:
   - `PollData` and `PollAnswer`: Public/anonymous voting, single/multi-choice, quiz modes with correct answer indices and explanations.
   - Dynamic vote tallies, percentage distribution, and automated close timers.
6. **Web Previews & Instant View**:
   - `WebPageData`: URL preview extraction, site name, rich author/title metadata, media embeds, and collages.
   - Instant View (IV) article reader structures and auctions/gift links.
7. **Star Gifts & Collectibles**:
   - `StarGift` and `UniqueGift`: Digital collectibles, Telegram Stars pricing, limited edition counts, rarity attributes (`Default`, `Uncommon`, `Rare`, `Epic`, `Legendary`), TON blockchain resale, and transfer timers.
8. **Saved Messages & Sublists**:
   - `SavedMessages`, `SavedSublist`, and `SavedMusic`: Monoforum sublists partitioned by original author peer, pinned sublists, and tag indexing.
9. **Stickers & Custom Emojis**:
   - `Stickers`, `StickersSet`, and `CustomEmojiManager`: Sticker packs, WebP/TGS (Lottie)/WebM formats, cloud recents, faved sets, custom emoji status packs.
10. **Business Features & Quick Replies**:
    - `Chatbots`: Connected business AI bots, permission flags (read, reply, mark read, delete), recipient filters.
    - `ShortcutMessages`: Business quick replies (`/shortcut` slash commands), shortcut message bundles, and opening hours.

---

## 2. Upstream C++ Class Hierarchy & Rust GPUI Target Mapping

The 58 assigned files in `crates/gpui_telegram_client/src/data/` map 1:1 to upstream `Telegram/SourceFiles/data/` as follows:

| Upstream C++ Source Files | Upstream C++ Classes / Types | Target Rust File | Target Rust Symbol | Responsibilities |
|---|---|---|---|---|
| `data/data_document.h`, `.cpp` | `DocumentData`, `StickerData`, `SongData`, `VoiceData`, `VideoData` | `data/data_document.rs` | `DataDocument` | Document metadata, dimensions, MIME type, audio/video attributes, streaming flags. |
| `data/data_document_media.h`, `.cpp` | `DocumentMedia`, `DocumentMedia::View` | `data/data_document_media.rs` | `DataDocumentMedia` | Document render cache, thumbnail image holder, video playback handle. |
| `data/data_document_resolver.h`, `.cpp` | `DocumentResolver`, `ResolveDocument` | `data/data_document_resolver.rs` | `DataDocumentResolver` | MIME-to-type resolution, file extension mapping, viewer association. |
| `data/data_photo.h`, `.cpp` | `PhotoData`, `PhotoSize`, `PhotoSizes` | `data/data_photo.rs` | `DataPhoto` | Progressive photo container (small, thumb, large), video photo loops, dimensions. |
| `data/data_photo_media.h`, `.cpp` | `PhotoMedia`, `PhotoMedia::View` | `data/data_photo_media.rs` | `DataPhotoMedia` | Cached image surfaces, texture loading, blurhash preview decoding. |
| `data/data_media_types.h`, `.cpp` | `Media`, `SharedMediaType`, `ItemPreview` | `data/data_media_types.rs` | `DataMediaTypes` | Shared media enum variants (13 types), message preview builders. |
| `data/data_media_preload.h`, `.cpp` | `MediaPreload`, `PreloadTask` | `data/data_media_preload.rs` | `DataMediaPreload` | Automatic media preloading manager based on network settings and file sizes. |
| `data/data_media_rotation.h`, `.cpp` | `MediaRotation`, `RotationAngle` | `data/data_media_rotation.rs` | `DataMediaRotation` | EXIF and user orientation adjustments (0, 90, 180, 270 degrees). |
| `data/data_download_manager.h`, `.cpp` | `DownloadManager`, `DownloadItem` | `data/data_download_manager.rs` | `DataDownloadManager` | User-initiated file downloads tracker, bandwidth limiter, pause/resume. |
| `data/data_auto_download.h`, `.cpp` | `AutoDownload`, `AutoDownloadSettings` | `data/data_auto_download.rs` | `DataAutoDownload` | Data-saving presets (Low, Medium, High), cellular/Wi-Fi/roaming auto-download rules. |
| `data/data_file_origin.h`, `.cpp` | `FileOrigin`, `FileOriginType` | `data/data_file_origin.rs` | `DataFileOrigin` | Security context for file requests (User, Chat, Channel, Story, WebPage). |
| `data/data_file_click_handler.h`, `.cpp` | `FileClickHandler`, `DocumentClickHandler` | `data/data_file_click_handler.rs` | `DataFileClickHandler` | Click action dispatcher (Open, Save As, Show in Folder, Stream). |
| `data/data_streaming.h`, `.cpp` | `Streaming`, `StreamPlayback` | `data/data_streaming.rs` | `DataStreaming` | Progressive streaming controller, buffer watermark management. |
| `data/data_audio_msg_id.h`, `.cpp` | `AudioMsgId`, `AudioMsgType` | `data/data_audio_msg_id.rs` | `DataAudioMsgId` | Global unique audio/voice message identifier for the media player. |
| `data/data_forum.h`, `.cpp` | `Forum`, `ForumOffsets` | `data/data_forum.rs` | `DataForum` | Group forum controller, topic cache, topic list ordering, unread aggregator. |
| `data/data_forum_topic.h`, `.cpp` | `ForumTopic`, `TopicIconDescriptor` | `data/data_forum_topic.rs` | `DataForumTopic` | Forum topic thread, root message ID, icon, color ID, unread counts, pinned state. |
| `data/data_forum_icons.h`, `.cpp` | `ForumIcons`, `ForumIconColor` | `data/data_forum_icons.rs` | `DataForumIcons` | Palette resolver for forum topic icons (0x6FB9F0, 0xFFD67E, etc.). |
| `data/data_thread.h`, `.cpp` | `Thread`, `ThreadId` | `data/data_thread.rs` | `DataThread` | Abstract conversation thread model shared by History, ForumTopic, and SavedSublist. |
| `data/data_saved_messages.h`, `.cpp` | `SavedMessages`, `SavedMessagesOffsets` | `data/data_saved_messages.rs` | `DataSavedMessages` | Saved Messages container, author-partitioned sublists, pinned topics. |
| `data/data_saved_sublist.h`, `.cpp` | `SavedSublist` | `data/data_saved_sublist.rs` | `DataSavedSublist` | Sub-conversation within Saved Messages filtered by original peer. |
| `data/data_saved_music.h`, `.cpp` | `SavedMusic`, `SavedMusicList` | `data/data_saved_music.rs` | `DataSavedMusic` | Dedicated audio playlist extracted from Saved Messages. |
| `data/data_replies_list.h`, `.cpp` | `RepliesList` | `data/data_replies_list.rs` | `DataRepliesList` | Message comment replies thread list and pagination tracker. |
| `data/data_reply_preview.h`, `.cpp` | `ReplyPreview`, `ReplyMarkup` | `data/data_reply_preview.rs` | `DataReplyPreview` | Replied message preview generator with thumbnail and sender summary. |
| `data/data_shared_media.h`, `.cpp` | `SharedMedia`, `SharedMediaCache` | `data/data_shared_media.rs` | `DataSharedMedia` | Peer shared media indexer across photos, videos, files, music, voice, links. |
| `data/data_stories.h`, `.cpp` | `Stories`, `StoriesSource` | `data/data_stories.rs` | `DataStories` | Peer stories collection, unread story ring tracking, archive lists. |
| `data/data_stories_ids.h`, `.cpp` | `StoriesIds`, `StoryId` | `data/data_stories_ids.rs` | `DataStoriesIds` | Story ID set tracking, read/unread status bitsets. |
| `data/data_story.h`, `.cpp` | `Story`, `StoryViews`, `StoryArea` | `data/data_story.rs` | `DataStory` | Individual story item, media attachment, expiration timer, privacy, view analytics. |
| `data/data_message_reaction_id.h`, `.cpp` | `ReactionId`, `ReactionType` | `data/data_message_reaction_id.rs` | `DataMessageReactionId` | Custom emoji vs standard emoji reaction identifier. |
| `data/data_message_reactions.rs` | `MessageReactions`, `ReactionCount` | `data/data_message_reactions.rs` | `DataMessageReactions` | Aggregated message reactions, recent reactors, my chosen reaction. |
| `data/data_star_gift.h`, `.cpp` | `StarGift`, `UniqueGift`, `GiftRarity` | `data/data_star_gift.rs` | `DataStarGift` | Telegram Star gifts, collectible gifts, upgrade variants, marketplace resale. |
| `data/data_poll.h`, `.cpp` | `PollData`, `PollAnswer`, `PollFlags` | `data/data_poll.rs` | `DataPoll` | Voting poll model, quiz answer validation, options distribution, close timers. |
| `data/data_poll_messages.h`, `.cpp` | `PollMessages` | `data/data_poll_messages.rs` | `DataPollMessages` | Message tracker referencing active polls for live result broadcasts. |
| `data/data_web_page.h`, `.cpp` | `WebPageData`, `WebPageType`, `WebPageCollage`| `data/data_web_page.rs` | `DataWebPage` | Rich link preview model, Instant View article cache, collage embeds. |
| `data/data_wall_paper.h`, `.cpp` | `WallPaper`, `WallPaperPattern` | `data/data_wall_paper.rs` | `DataWallPaper` | Chat background wallpaper definition (colors, gradients, patterns). |
| `data/data_drafts.h`, `.cpp` | `Draft`, `DraftKey` | `data/data_drafts.rs` | `DataDrafts` | Text draft persistence with reply/thread context and markdown entities. |
| `data/data_emoji_statuses.h`, `.cpp` | `EmojiStatus`, `EmojiStatusCollectibles`| `data/data_emoji_statuses.rs` | `DataEmojiStatuses` | Telegram Premium custom emoji status badge and expiration timestamp. |
| `data/data_game.h`, `.cpp` | `GameData`, `HighScore` | `data/data_game.rs` | `DataGame` | In-chat HTML5 game model, score table, bot provider. |
| `data/data_group_call.h`, `.cpp` | `GroupCall`, `CallParticipant` | `data/data_group_call.rs` | `DataGroupCall` | Group voice chat / video broadcast state, participants, mute status. |
| `data/data_location.h`, `.cpp` | `LocationPoint`, `LocationVenue` | `data/data_location.rs` | `DataLocation` | Geo coordinates, live location sharing duration, venue details. |
| `data/data_send_action.h`, `.cpp` | `SendAction`, `SendActionType` | `data/data_send_action.rs` | `DataSendAction` | Typing, recording voice, uploading photo status indicators. |
| `data/data_statistics.h` | `StatisticsData` | `data/data_statistics.rs` | `DataStatistics` | Channel and group growth analytics metrics. |
| `data/data_statistics_chart.h`, `.cpp` | `StatisticsChart`, `ChartPoint` | `data/data_statistics_chart.rs` | `DataStatisticsChart` | Chart coordinate curves, zooming, stacked area / line graph models. |
| `data/data_statistics_lists.h` | `StatisticsLists` | `data/data_statistics_lists.rs` | `DataStatisticsLists` | Top referring channels, recent message interactions table. |
| `data/data_todo_list.h`, `.cpp` | `TodoList`, `TodoItem` | `data/data_todo_list.rs` | `DataTodoList` | Collaborative group checklist / task list data model. |
| `data/data_ai_compose_tones.h`, `.cpp`| `AiComposeTones`, `AiTone` | `data/data_ai_compose_tones.rs` | `DataAiComposeTones` | AI message rewriting tone selector (Formal, Casual, Poetic, etc.). |
| `data/data_bot_app.h`, `.cpp` | `BotApp`, `BotAppShortName` | `data/data_bot_app.rs` | `DataBotApp` | Telegram Mini App WebApp definition, launch parameters, permissions. |
| `data/data_cloud_file.h`, `.cpp` | `CloudFile`, `CloudFileLocation` | `data/data_cloud_file.rs` | `DataCloudFile` | Abstract remote cloud file reference, DC ID, and access hash. |
| `data/data_compose_stash.h` | `ComposeStash` | `data/data_compose_stash.rs` | `DataComposeStash` | Temporary stashed message state when navigating across multiple chats. |
| `data/business/data_business_chatbots.h`, `.cpp` | `Chatbots`, `ChatbotsSettings` | `data/business/data_business_chatbots.rs` | `DataBusinessChatbotsBusiness` | Business bot integration, automatic response permissions, exclusion list. |
| `data/business/data_business_common.h`, `.cpp` | `BusinessRecipients`, `BusinessHours` | `data/business/data_business_common.rs` | `DataBusinessCommonBusiness` | Working hours schedule, away message rules, greeting messages. |
| `data/business/data_business_info.h`, `.cpp` | `BusinessInfo`, `BusinessLocation` | `data/business/data_business_info.rs` | `DataBusinessInfoBusiness` | Business profile details, physical address, business introduction. |
| `data/business/data_shortcut_messages.h`, `.cpp` | `ShortcutMessages`, `Shortcut` | `data/business/data_shortcut_messages.rs` | `DataShortcutMessagesBusiness` | Business quick reply shortcuts (`/order`, `/info`), message bundles. |
| `data/notify/data_notify_settings.h`, `.cpp` | `NotifySettings` | `data/notify/data_notify_settings.rs` | `DataNotifySettingsNotify` | Global notification presets (private chats, groups, channels). |
| `data/notify/data_peer_notify_settings.h`, `.cpp`| `PeerNotifySettings` | `data/notify/data_peer_notify_settings.rs` | `DataPeerNotifySettingsNotify` | Per-chat mute intervals, sound, preview toggles, exception flags. |
| `data/notify/data_peer_notify_volume.h`, `.cpp` | `PeerNotifyVolume` | `data/notify/data_peer_notify_volume.rs` | `DataPeerNotifyVolumeNotify` | Sound volume overrides and custom ringtone document IDs. |
| `data/stickers/data_custom_emoji.h`, `.cpp` | `CustomEmojiManager`, `CustomEmoji` | `data/stickers/data_custom_emoji.rs` | `DataCustomEmojiStickers` | Custom emoji document cache, colorable emoji rendering, pack IDs. |
| `data/stickers/data_stickers.h`, `.cpp` | `Stickers`, `StickerType` | `data/stickers/data_stickers.rs` | `DataStickersStickers` | Sticker catalog, installed sets, recent stickers, faved stickers. |
| `data/stickers/data_stickers_set.h`, `.cpp` | `StickersSet`, `StickersSetFlags` | `data/stickers/data_stickers_set.rs` | `DataStickersSetStickers` | Individual sticker pack model, short name, title, count, cover document. |

---

## 3. Core Data Models & State Machines

### 3.1 Document & Media Model (`DataDocument`)
```
  MTP Document ────────► DataDocument
                            │
                            ├── id: u64
                            ├── size: i64
                            ├── mime: String ("audio/ogg", "video/mp4", "image/webp")
                            ├── dimensions: (u32, u32)
                            ├── dc_id: i32, access_hash: u64, file_reference: Vec<u8>
                            ├── kind: DocumentKind
                            │     ├── File { filename: String }
                            │     ├── Sticker { alt: String, set_id: u64, format: StickerFormat }
                            │     ├── Song { title: String, performer: String, duration: u32 }
                            │     ├── Voice { waveform: Vec<u8>, duration: u32 }
                            │     └── Video { duration: u32, codec: String, qualities: Vec<VideoQuality> }
                            └── loading_state: FileLoadingState (Idle, Loading(0.0..1.0), Ready, Failed)
```

### 3.2 Progressive Photo Pipeline (`DataPhoto`)
```
  PhotoData (id, date, has_video)
     │
     ├── small:     PhotoSizeVariant (100x100 max, thumbnail preview, instant load)
     ├── thumbnail: PhotoSizeVariant (320x320 max, bubble preview)
     └── large:     PhotoSizeVariant (1280x1280 max, full screen viewer)
```

### 3.3 Forum Topic Model (`DataForumTopic`)
```
  ForumTopic
     │
     ├── root_id: i32 (1 = General Topic)
     ├── title: String
     ├── icon_id: u64 (Custom emoji DocumentId)
     ├── color_id: i32 (0..5 color palette index)
     ├── is_closed: bool, is_hidden: bool, is_pinned: bool
     ├── unread_count: u32, unread_mentions: u32, unread_reactions: u32
     └── notify_settings: PeerNotifySettings
```

### 3.4 Story Expiration & Analytics (`DataStory`)
```
  Story
     │
     ├── id: i32, peer_id: u64
     ├── date: i64, expires: i64 (typically date + 86400s)
     ├── privacy: StoryPrivacy (Public, CloseFriends, Contacts)
     ├── expired(now) := now >= expires
     ├── media: StoryMedia (Photo, Video, Call)
     └── views: StoryViews (views: u32, reactions: u32, forwards: u32, viewers: Vec<PeerId>)
```

### 3.5 Interactive Poll & Quiz Engine (`DataPoll`)
```
  PollData
     │
     ├── id: u64, question: String
     ├── answers: Vec<PollAnswer>
     │     ├── option: Vec<u8>
     │     ├── text: String
     │     ├── votes: u32
     │     ├── is_chosen: bool
     │     └── is_correct: bool (Quiz mode)
     ├── flags: PollFlags (Closed, PublicVotes, MultiChoice, Quiz)
     ├── vote(options) -> validates limits & tallies counts
     └── close() -> marks closed and finalizes percentages
```

---

## 4. Testing & Verification

1. **Subsystem Integration Tests**:
   - `crates/gpui_telegram_client/tests/data_media_content_subsystem_tests.rs`:
     * Document MIME type resolution, song duration, voice waveform decoding.
     * Progressive photo dimensions resolution and sizing limits.
     * Forum topic creation, icon color lookup, unread state calculations.
     * Story expiration timestamps and privacy checks.
     * Poll voting mechanics, multi-choice restrictions, quiz solution verification.
     * Star gift rarity badges, resale calculations, and business chatbot permission masks.
2. **Boilerplate Burndown Verification**:
   - Ensure 0 instances of Pattern A (`pub label: String, pub value: i64`), Pattern B, Pattern C, Pattern D in all 58 files.
   - Run `python3 scripts/verify_ledger_closure.py` to assert Gate 6.2 passes cleanly.

