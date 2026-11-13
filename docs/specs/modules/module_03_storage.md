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
