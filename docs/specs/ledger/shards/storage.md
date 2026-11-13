# Subsystem Shard: `storage`

**Total Source Files**: 47  
**Total Source Lines**: 21,081  
**Primary Feature IDs**: TCG-N11, TCG-N34, TCG-N37, TCG-N51  
**JSON Shard Path**: `docs/specs/ledger/shards/storage.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/storage/details/storage_file_utilities.cpp` | `TCG-N34` | `StorageFileUtilitiesDetails` | `crates/gpui_telegram_client/src/storage/details/storage_file_utilities.rs` | 883 | `tests/test_tcg_n34_storage_file_utilities.rs` |
| `Telegram/SourceFiles/storage/details/storage_file_utilities.h` | `TCG-N34` | `StorageFileUtilitiesDetails` | `crates/gpui_telegram_client/src/storage/details/storage_file_utilities.rs` | 139 | `tests/test_tcg_n34_storage_file_utilities.rs` |
| `Telegram/SourceFiles/storage/details/storage_settings_scheme.cpp` | `TCG-N34` | `StorageSettingsSchemeDetails` | `crates/gpui_telegram_client/src/storage/details/storage_settings_scheme.rs` | 1202 | `tests/test_tcg_n34_storage_settings_scheme.rs` |
| `Telegram/SourceFiles/storage/details/storage_settings_scheme.h` | `TCG-N34` | `StorageSettingsSchemeDetails` | `crates/gpui_telegram_client/src/storage/details/storage_settings_scheme.rs` | 178 | `tests/test_tcg_n34_storage_settings_scheme.rs` |
| `Telegram/SourceFiles/storage/download_manager_mtproto.cpp` | `TCG-N34` | `DownloadManagerMtproto` | `crates/gpui_telegram_client/src/storage/download_manager_mtproto.rs` | 1045 | `tests/test_tcg_n34_download_manager_mtproto.rs` |
| `Telegram/SourceFiles/storage/download_manager_mtproto.h` | `TCG-N34` | `DownloadManagerMtproto` | `crates/gpui_telegram_client/src/storage/download_manager_mtproto.rs` | 288 | `tests/test_tcg_n34_download_manager_mtproto.rs` |
| `Telegram/SourceFiles/storage/file_download.cpp` | `TCG-N34` | `FileDownload` | `crates/gpui_telegram_client/src/storage/file_download.rs` | 565 | `tests/test_tcg_n34_file_download.rs` |
| `Telegram/SourceFiles/storage/file_download.h` | `TCG-N34` | `FileDownload` | `crates/gpui_telegram_client/src/storage/file_download.rs` | 206 | `tests/test_tcg_n34_file_download.rs` |
| `Telegram/SourceFiles/storage/file_download_mtproto.cpp` | `TCG-N34` | `FileDownloadMtproto` | `crates/gpui_telegram_client/src/storage/file_download_mtproto.rs` | 221 | `tests/test_tcg_n34_file_download_mtproto.rs` |
| `Telegram/SourceFiles/storage/file_download_mtproto.h` | `TCG-N34` | `FileDownloadMtproto` | `crates/gpui_telegram_client/src/storage/file_download_mtproto.rs` | 74 | `tests/test_tcg_n34_file_download_mtproto.rs` |
| `Telegram/SourceFiles/storage/file_download_web.cpp` | `TCG-N34` | `FileDownloadWeb` | `crates/gpui_telegram_client/src/storage/file_download_web.rs` | 652 | `tests/test_tcg_n34_file_download_web.rs` |
| `Telegram/SourceFiles/storage/file_download_web.h` | `TCG-N34` | `FileDownloadWeb` | `crates/gpui_telegram_client/src/storage/file_download_web.rs` | 63 | `tests/test_tcg_n34_file_download_web.rs` |
| `Telegram/SourceFiles/storage/file_upload.cpp` | `TCG-N51` | `FileUpload` | `crates/gpui_telegram_client/src/storage/file_upload.rs` | 1281 | `tests/test_tcg_n51_file_upload.rs` |
| `Telegram/SourceFiles/storage/file_upload.h` | `TCG-N51` | `FileUpload` | `crates/gpui_telegram_client/src/storage/file_upload.rs` | 228 | `tests/test_tcg_n51_file_upload.rs` |
| `Telegram/SourceFiles/storage/localimageloader.cpp` | `TCG-N34` | `Localimageloader` | `crates/gpui_telegram_client/src/storage/localimageloader.rs` | 1311 | `tests/test_tcg_n34_localimageloader.rs` |
| `Telegram/SourceFiles/storage/localimageloader.h` | `TCG-N34` | `Localimageloader` | `crates/gpui_telegram_client/src/storage/localimageloader.rs` | 352 | `tests/test_tcg_n34_localimageloader.rs` |
| `Telegram/SourceFiles/storage/localstorage.cpp` | `TCG-N34` | `Localstorage` | `crates/gpui_telegram_client/src/storage/localstorage.rs` | 1439 | `tests/test_tcg_n34_localstorage.rs` |
| `Telegram/SourceFiles/storage/localstorage.h` | `TCG-N34` | `Localstorage` | `crates/gpui_telegram_client/src/storage/localstorage.rs` | 104 | `tests/test_tcg_n34_localstorage.rs` |
| `Telegram/SourceFiles/storage/serialize_common.cpp` | `TCG-N34` | `SerializeCommon` | `crates/gpui_telegram_client/src/storage/serialize_common.rs` | 48 | `tests/test_tcg_n34_serialize_common.rs` |
| `Telegram/SourceFiles/storage/serialize_common.h` | `TCG-N34` | `SerializeCommon` | `crates/gpui_telegram_client/src/storage/serialize_common.rs` | 175 | `tests/test_tcg_n34_serialize_common.rs` |
| `Telegram/SourceFiles/storage/serialize_document.cpp` | `TCG-N34` | `SerializeDocument` | `crates/gpui_telegram_client/src/storage/serialize_document.rs` | 308 | `tests/test_tcg_n34_serialize_document.rs` |
| `Telegram/SourceFiles/storage/serialize_document.h` | `TCG-N34` | `SerializeDocument` | `crates/gpui_telegram_client/src/storage/serialize_document.rs` | 48 | `tests/test_tcg_n34_serialize_document.rs` |
| `Telegram/SourceFiles/storage/serialize_peer.cpp` | `TCG-N11` | `SerializePeer` | `crates/gpui_telegram_client/src/storage/serialize_peer.rs` | 457 | `tests/test_tcg_n11_serialize_peer.rs` |
| `Telegram/SourceFiles/storage/serialize_peer.h` | `TCG-N11` | `SerializePeer` | `crates/gpui_telegram_client/src/storage/serialize_peer.rs` | 33 | `tests/test_tcg_n11_serialize_peer.rs` |
| `Telegram/SourceFiles/storage/storage_account.cpp` | `TCG-N34` | `StorageAccount` | `crates/gpui_telegram_client/src/storage/storage_account.rs` | 4036 | `tests/test_tcg_n34_storage_account.rs` |
| `Telegram/SourceFiles/storage/storage_account.h` | `TCG-N34` | `StorageAccount` | `crates/gpui_telegram_client/src/storage/storage_account.rs` | 425 | `tests/test_tcg_n34_storage_account.rs` |
| `Telegram/SourceFiles/storage/storage_cloud_blob.cpp` | `TCG-N34` | `StorageCloudBlob` | `crates/gpui_telegram_client/src/storage/storage_cloud_blob.rs` | 156 | `tests/test_tcg_n34_storage_cloud_blob.rs` |
| `Telegram/SourceFiles/storage/storage_cloud_blob.h` | `TCG-N34` | `StorageCloudBlob` | `crates/gpui_telegram_client/src/storage/storage_cloud_blob.rs` | 115 | `tests/test_tcg_n34_storage_cloud_blob.rs` |
| `Telegram/SourceFiles/storage/storage_domain.cpp` | `TCG-N34` | `StorageDomain` | `crates/gpui_telegram_client/src/storage/storage_domain.rs` | 1140 | `tests/test_tcg_n34_storage_domain.rs` |
| `Telegram/SourceFiles/storage/storage_domain.h` | `TCG-N34` | `StorageDomain` | `crates/gpui_telegram_client/src/storage/storage_domain.rs` | 231 | `tests/test_tcg_n34_storage_domain.rs` |
| `Telegram/SourceFiles/storage/storage_facade.cpp` | `TCG-N34` | `StorageFacade` | `crates/gpui_telegram_client/src/storage/storage_facade.rs` | 237 | `tests/test_tcg_n34_storage_facade.rs` |
| `Telegram/SourceFiles/storage/storage_facade.h` | `TCG-N34` | `StorageFacade` | `crates/gpui_telegram_client/src/storage/storage_facade.rs` | 83 | `tests/test_tcg_n34_storage_facade.rs` |
| `Telegram/SourceFiles/storage/storage_file_lock.h` | `TCG-N34` | `StorageFileLock` | `crates/gpui_telegram_client/src/storage/storage_file_lock.rs` | 38 | `tests/test_tcg_n34_storage_file_lock.rs` |
| `Telegram/SourceFiles/storage/storage_file_lock_posix.cpp` | `TCG-N34` | `StorageFileLockPosix` | `crates/gpui_telegram_client/src/storage/storage_file_lock_posix.rs` | 124 | `tests/test_tcg_n34_storage_file_lock_posix.rs` |
| `Telegram/SourceFiles/storage/storage_file_lock_win.cpp` | `TCG-N34` | `StorageFileLockWin` | `crates/gpui_telegram_client/src/storage/storage_file_lock_win.rs` | 143 | `tests/test_tcg_n34_storage_file_lock_win.rs` |
| `Telegram/SourceFiles/storage/storage_folder_archive.cpp` | `TCG-N37` | `StorageFolderArchive` | `crates/gpui_telegram_client/src/storage/storage_folder_archive.rs` | 531 | `tests/test_tcg_n37_storage_folder_archive.rs` |
| `Telegram/SourceFiles/storage/storage_folder_archive.h` | `TCG-N37` | `StorageFolderArchive` | `crates/gpui_telegram_client/src/storage/storage_folder_archive.rs` | 62 | `tests/test_tcg_n37_storage_folder_archive.rs` |
| `Telegram/SourceFiles/storage/storage_media_prepare.cpp` | `TCG-N34` | `StorageMediaPrepare` | `crates/gpui_telegram_client/src/storage/storage_media_prepare.rs` | 675 | `tests/test_tcg_n34_storage_media_prepare.rs` |
| `Telegram/SourceFiles/storage/storage_media_prepare.h` | `TCG-N34` | `StorageMediaPrepare` | `crates/gpui_telegram_client/src/storage/storage_media_prepare.rs` | 121 | `tests/test_tcg_n34_storage_media_prepare.rs` |
| `Telegram/SourceFiles/storage/storage_shared_media.cpp` | `TCG-N34` | `StorageSharedMedia` | `crates/gpui_telegram_client/src/storage/storage_shared_media.rs` | 241 | `tests/test_tcg_n34_storage_shared_media.rs` |
| `Telegram/SourceFiles/storage/storage_shared_media.h` | `TCG-N34` | `StorageSharedMedia` | `crates/gpui_telegram_client/src/storage/storage_shared_media.rs` | 315 | `tests/test_tcg_n34_storage_shared_media.rs` |
| `Telegram/SourceFiles/storage/storage_sparse_ids_list.cpp` | `TCG-N34` | `StorageSparseIdsList` | `crates/gpui_telegram_client/src/storage/storage_sparse_ids_list.rs` | 312 | `tests/test_tcg_n34_storage_sparse_ids_list.rs` |
| `Telegram/SourceFiles/storage/storage_sparse_ids_list.h` | `TCG-N34` | `StorageSparseIdsList` | `crates/gpui_telegram_client/src/storage/storage_sparse_ids_list.rs` | 113 | `tests/test_tcg_n34_storage_sparse_ids_list.rs` |
| `Telegram/SourceFiles/storage/storage_user_photos.cpp` | `TCG-N34` | `StorageUserPhotos` | `crates/gpui_telegram_client/src/storage/storage_user_photos.rs` | 223 | `tests/test_tcg_n34_storage_user_photos.rs` |
| `Telegram/SourceFiles/storage/storage_user_photos.h` | `TCG-N34` | `StorageUserPhotos` | `crates/gpui_telegram_client/src/storage/storage_user_photos.rs` | 212 | `tests/test_tcg_n34_storage_user_photos.rs` |
| `Telegram/SourceFiles/storage/streamed_file_downloader.cpp` | `TCG-N52` | `StreamedFileDownloader` | `crates/gpui_telegram_client/src/storage/streamed_file_downloader.rs` | 172 | `tests/test_tcg_n52_streamed_file_downloader.rs` |
| `Telegram/SourceFiles/storage/streamed_file_downloader.h` | `TCG-N52` | `StreamedFileDownloader` | `crates/gpui_telegram_client/src/storage/streamed_file_downloader.rs` | 76 | `tests/test_tcg_n52_streamed_file_downloader.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/storage/details/storage_file_utilities.cpp`
- **Classes & Structs**: WriteEntry, WriteManager, AsyncWriteManager
- **Core Methods & Signals**: WriteManager::WriteManager, WriteManager::write, ranges::find, WriteManager::writeSync, WriteManager::writeNow, QFile::remove
- **Target**: `StorageFileUtilitiesDetails` in `crates/gpui_telegram_client/src/storage/details/storage_file_utilities.rs`

### `Telegram/SourceFiles/storage/details/storage_file_utilities.h`
- **Classes & Structs**: PasscodeKdf, FileReadDescriptor, EncryptedDescriptor, FileWriteDescriptor
- **Core Methods & Signals**: ClearKey, EncryptedDescriptor, finish, FileWriteDescriptor, writeData, writeEncrypted
- **Target**: `StorageFileUtilitiesDetails` in `crates/gpui_telegram_client/src/storage/details/storage_file_utilities.rs`

### `Telegram/SourceFiles/storage/details/storage_settings_scheme.cpp`
- **Classes & Structs**: WorkMode, Type, SendSettings, Notify
- **Core Methods & Signals**: Flags::from_raw, Core::App, Database::Settings, PowerSaving::Set, MTP::ProxyData, Logs::b
- **Target**: `StorageSettingsSchemeDetails` in `crates/gpui_telegram_client/src/storage/details/storage_settings_scheme.rs`

### `Telegram/SourceFiles/storage/details/storage_settings_scheme.h`
- **Classes & Structs**: AuthKey, ReadSettingsContext
- **Core Methods & Signals**: MTP::DcOptions, ApplyReadFallbackConfig
- **Target**: `StorageSettingsSchemeDetails` in `crates/gpui_telegram_client/src/storage/details/storage_settings_scheme.rs`

### `Telegram/SourceFiles/storage/download_manager_mtproto.cpp`
- **Classes & Structs**: Redirect, Flag
- **Core Methods & Signals**: Queue::enqueue, ranges::find_if, ranges::find, Queue::remove, ranges::remove, Queue::resetGeneration
- **Target**: `DownloadManagerMtproto` in `crates/gpui_telegram_client/src/storage/download_manager_mtproto.rs`

### `Telegram/SourceFiles/storage/download_manager_mtproto.h`
- **Classes & Structs**: ApiWrap, Error, DownloadMtprotoTask, DownloadManagerMtproto, Queue, Enqueued, DcSessionBalanceData, DcBalanceData, Location, RequestData, CdnFileHash, CheckCdnHashResult
- **Core Methods & Signals**: DownloadManagerMtproto, enqueue, remove, notifyTaskFinished, changeRequestedAmount, requestSucceeded
- **Target**: `DownloadManagerMtproto` in `crates/gpui_telegram_client/src/storage/download_manager_mtproto.rs`

### `Telegram/SourceFiles/storage/file_download.cpp`
- **Classes & Structs**: FromMemoryLoader
- **Core Methods & Signals**: QByteArray::fromRawData, FromMemoryLoader::FromMemoryLoader, FromMemoryLoader::cacheKey, FromMemoryLoader::fileLocationKey, FromMemoryLoader::cancelHook, FromMemoryLoader::startLoading
- **Target**: `FileDownload` in `crates/gpui_telegram_client/src/storage/file_download.rs`

### `Telegram/SourceFiles/storage/file_download.h`
- **Classes & Structs**: FileOrigin, Session, Key, StorageImageSaved, FileLoader, FailureReason, Error, LocalStatus
- **Core Methods & Signals**: ~FileLoader, finishWithBytes, setFileName, permitLoadFromCloud, increaseLoadSize, start
- **Target**: `FileDownload` in `crates/gpui_telegram_client/src/storage/file_download.rs`

### `Telegram/SourceFiles/storage/file_download_mtproto.h`
- **Classes & Structs**: mtpFileLoader
- **Core Methods & Signals**: mtpFileLoader, fileOrigin, objId, cacheKey, fileLocationKey, startLoading
- **Target**: `FileDownloadMtproto` in `crates/gpui_telegram_client/src/storage/file_download_mtproto.rs`

### `Telegram/SourceFiles/storage/file_download_web.cpp`
- **Classes & Structs**: ProcessResult, Error, Progress, UpdateForLoader, WebLoadManager, Enqueued, Sent, Update
- **Core Methods & Signals**: WebLoadManager::WebLoadManager, QObject::connect, WebLoadManager::handleNetworkErrors, WebLoadManager::~WebLoadManager, WebLoadManager::updates, WebLoadManager::enqueue
- **Target**: `FileDownloadWeb` in `crates/gpui_telegram_client/src/storage/file_download_web.rs`

