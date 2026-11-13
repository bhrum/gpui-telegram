# Module 09 (media & editor) — Slice 2: Media Streaming Instance, Chunk Cache, Demuxers & Video Clip Readers Specification

## 1. Executive Summary & Architectural Role

Module 09 Slice 2 implements the core multimedia streaming and clip playback subsystem for Telegram Desktop in pure Rust and GPUI. It is responsible for:
- **Cloud & Local Media Streaming**: Progressive downloading and segmented streaming from MTProto cloud storage or local disk (`LoaderLocal`, `LoaderMtproto`, `Loader`, `LoadedPart`, `PriorityQueue`).
- **Segmented Chunk Caching & Virtual File Demuxer**: 128 KB chunk management, 8 MB slices (`kPartsInSlice = 64`, `kInSlice = 8MB`), LRU caching with dynamic memory eviction, and multi-thread safe custom AVIO seek/read (`Reader`, `File`, `FileDelegate`).
- **Audio & Video Track Synchronization**: Ring-buffered video frames (`kFramesCount = 4`), PTS clock synchronization, dynamic framerate estimation, aspect ratio correction, hardware accelerated NV12/ARGB conversion, and adaptive quality/bitrate switching (`AudioTrack`, `VideoTrack`, `Player`, `Document`, `Instance`).
- **GIF / Animated Video Clip Subsystem**: Fast-start MP4 `moov` atom validation (`CheckStreamingSupport`), looping video playback with triple-buffered frames (`_frames[3]`), lock-free atomic step transitions (`-3..5`), and device-pixel-ratio scaling (`Reader`, `FFMpegReaderImplementation`, `RoundPreview`).

---

## 2. Upstream C++ 100% Reading Manifest (Part 1: Streaming Common, Loaders & Delegates — 10 Files)

| Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path |
| :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/media/streaming/media_streaming_common.h` | 214 | `kTimeUnknown = i64::MIN`, `kDurationMax = i32::MAX`, `kDurationUnavailable = i64::MAX`, `kMaxFrameArea = 3840 * 2160 = 8294400`, `Mode { Both, Audio, Video, Inspection }`, `PlaybackOptions` (speed 0.5..2.0, volume 0.0..1.0, loop, seekable), `TrackState` (position, receivedTill, duration), `VideoInformation`, `AudioInformation`, `Information`, `WaitingForData`, `SpeedEstimate` (bytesPerSecond, unreliable), `Update` variant, `Error { OpenFailed, LoadFailed, InvalidData, NotStreamable }`, `FrameRequest` (empty, goodFor, strict), `FrameFormat { None, ARGB32, YUV420, NV12, NativeTexture }`, `FrameYUV`, `NativeFrame`, `FrameWithInfo`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_common.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_file_delegate.h` | 42 | `FileDelegate`: virtual methods `fileOpenMode()`, `fileReady(headerSize, video, audio)`, `fileError(error)`, `fileWaitingForData()`, `fileFullInCache(fullInCache)`, `fileProcessEndOfFile()`, `fileProcessPackets(packets)`, `fileReadMore()`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_file_delegate.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_loader.h` | 81 | `LoadedPart`: `offset`, `bytes`, `kFailedOffset = -1`, `valid(size)`; `Loader`: `kPartSize = 128 * 1024` (128 KB), `baseCacheKey()`, `size()`, `load()`, `cancel()`, `resetPriorities()`, `setPriority()`, `stop()`, `parts()`, `speedEstimate()`; `PriorityQueue`: `Entry { value, priority }`, `add()`, `remove()`, `front()`, `take()`, `takeInRange()`, `resetPriorities()`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_loader.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_loader.cpp` | 95 | `media_streaming_loader.cpp:L13-L18`: `LoadedPart::valid` validates non-failed offset and part size == 128KB or end of file; `media_streaming_loader.cpp:L19-L29`: `PriorityQueue::Entry` sort comparator (higher priority comes first `a.priority > b.priority`, tiebreaker lower offset `a.value < b.value`); `media_streaming_loader.cpp:L72-L83`: `takeInRange(from, till)`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_loader.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_loader_local.h` | 56 | `LoaderLocal`: inherits `Loader`, wraps `QIODevice`, `_size`, `_parts` event stream, factory helpers `MakeFileLoader(path)` and `MakeBytesLoader(bytes)`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_loader_local.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_loader_local.cpp` | 116 | `media_streaming_loader_local.cpp:L19-L23`: `kMaxFileSize = 8000 * 512 * 1024` (approx 4GB Telegram max file size check); `media_streaming_loader_local.cpp:L45-L60`: `load(offset)` seeks device, reads `kPartSize = 128KB`, fires `_parts` stream on main thread or fails with `kFailedOffset`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_loader_local.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_loader_mtproto.h` | 82 | `LoaderMtproto`: inherits `Loader` & `DownloadMtprotoTask`, `StatsEntry { start, end, offset }`, `_requested` `PriorityQueue`, `_stats` window, `_statsTimer`, attached `StreamedFileDownloader`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_loader_mtproto.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_loader_mtproto.cpp` | 222 | `media_streaming_loader_mtproto.cpp:L19-L21`: `kCheckStatsInterval = 1000ms`, `kInitialStatsWait = 5000ms`; `media_streaming_loader_mtproto.cpp:L173-L218`: `checkStats()` calculates download speed `received * 1000 / duration` clamped to `64 * 1024 * 1024` (64 MB/s), marks `unreliable = (received < 3 * 128KB)`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_loader_mtproto.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_utility.h` | 100 | `TimePoint { trackTime, worldTime }`, `Stream { index, duration, timeBase, codec, decodedFrame, queue, frequency, rotation, aspect, fps }`, `FramePosition(stream)`, `ProcessPacket()`, `ReadNextFrame()`, `GoodForRequest()`, `TransferFrame()`, `ConvertFrame()`, `ExtractYUV()`, `DecideFrameResize()`, `PrepareBlurredBackground()`, `PrepareByRequest()`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_utility.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_utility.cpp` | 457 | `media_streaming_utility.cpp:L19`: `kSkipInvalidDataPackets = 10`; `media_streaming_utility.cpp:L23-L37`: `FramePosition` converts PTS to time and clamps to duration; `media_streaming_utility.cpp:L39-L72`: skips bad packets up to 10; `media_streaming_utility.cpp:L294-L323`: 40x40 blurred background generation; `media_streaming_utility.cpp:L371-L415`: `DecideFrameResize` aspect expansion algorithm. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_utility.rs` |


## 3. Upstream C++ 100% Reading Manifest (Part 2: Previews, Platforms, Document, File & Instance — 8 Files)

| Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path |
| :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/media/streaming/media_streaming_round_preview.h` | 36 | `RoundPreview`: inherits `Ui::DynamicImage`, wraps video message round preview, `_reader` `Clip::ReaderPointer`, `_size`, `_repaint` callback, `image(size)`, `subscribeToUpdates(callback)`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_round_preview.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_round_preview.cpp` | 63 | `media_streaming_round_preview.cpp:L12-L35`: `RoundPreview` creates clip reader with `ImageRoundRadius::Ellipse`, queries current frame with device pixel ratio; `media_streaming_round_preview.cpp:L40-L60`: `clipCallback` handles `Notification::Reinit` and `Notification::Repaint`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_round_preview.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_native_frame_mac.h` | 23 | macOS native video frame decoding: `ConvertNativeFrameToARGB32(const NativeFrame &frame)`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_native_frame_mac.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_native_frame_mac.mm` | 130 | `media_streaming_native_frame_mac.mm:L20-L48`: `PixelBufferLock` RAII guard with `CVPixelBufferLockBaseAddress` and `CVPixelBufferUnlockBaseAddress`; `media_streaming_native_frame_mac.mm:L52-L125`: checks NV12 pixel format `kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange`, extracts Y and UV planes, uses Swscale to convert to BGRA image storage. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_native_frame_mac.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_document.h` | 99 | `QualityDescriptor { sizeInBytes, height }`, `Document`: plays `DocumentData` and `PhotoData`, `Player _player`, `Information _info`, `_instances`, `_otherQualities`, `waitingShown()`, `waitingOpacity()`, `waitingState()`, `switchQualityRequests()`, `checkSwitchToHigherQuality()`, `checkSwitchToLowerQuality()`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_document.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_document.cpp` | 369 | `media_streaming_document.cpp:L28-L34`: `kWaitingFastDuration = 200ms`, `kWaitingShowDuration = 500ms`, `kWaitingShowDelay = 500ms`, `kGoodThumbQuality = 87`, `kSwitchQualityUpPreloadedThreshold = 4000ms`, `kSwitchQualityUpSpeedMultiplier = 1.2`; `media_streaming_document.cpp:L189-L222`: adaptive quality switch to higher resolution when download speed exceeds bitrate * 1.2; `media_streaming_document.cpp:L224-L245`: fallback to lower resolution on buffering. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_document.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_file.h` | 127 | `StartOptions { position, durationOverride, seekable, hwAllow }`, `File`: multi-threaded file packet demuxer, `File::Context` with custom AVIO `Read` & `Seek` callbacks, `_semaphore`, `_queuedPackets`, `_reader`, `_delegate`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_file.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_file.cpp` | 512 | `media_streaming_file.cpp:L18-L33`: `kMaxSingleReadAmount = 8MB`, `kMaxQueuedPackets = 1024`, `UnreliableFormatDuration` for WebM/VP9; `media_streaming_file.cpp:L56-L101`: `File::Context::read` acquires semaphore, fills from reader, handles remote wait; `media_streaming_file.cpp:L358-L382`: `readNextPacket` queues up to 1024 packets then signals player thread. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_file.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_instance.h` | 106 | `Instance`: consumer handle bound to `Document`, `play()`, `pause()`, `resume()`, `stop()`, `stopAudio()`, `setSpeed()`, `setVolume()`, `waitingShown()`, `frame()`, `frameWithInfo()`, `markFrameShown()`, priority control and lock reference counting. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_instance.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_instance.cpp` | 271 | `media_streaming_instance.cpp:L20-L81`: `Instance` constructor/destructor registers with `Document`, manages player lock count; `media_streaming_instance.cpp:L90-L92`: `setPriority` dynamically updates player streaming priority to highest active instance. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_instance.rs` |


## 4. Upstream C++ 100% Reading Manifest (Part 3: Tracks, Player & Reader — 8 Files)

| Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path |
| :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/media/streaming/media_streaming_audio_track.h` | 96 | `AudioTrack`: audio decode pipeline, `_stream`, `_audioId`, `_playPosition`, `_startedPosition`, `_initialSkippingFrame`, `process(packets)`, `pause(time)`, `resume(time)`, `setSpeed(speed)`, `setVolume(volume)`, `waitingForData()`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_audio_track.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_audio_track.cpp` | 254 | `media_streaming_audio_track.cpp:L50-L66`: `process()` enqueues decoded audio packets into sound mixer; `media_streaming_audio_track.cpp:L78-L110`: `tryReadFirstFrame()` skips backward seek frames until target timestamp reached; `media_streaming_audio_track.cpp:L130-L140`: `mixerInit()` configures playback speed and volume. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_audio_track.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_video_track.h` | 189 | `VideoTrack`: video decode pipeline, `VideoTrack::Shared` with 4-frame ring buffer (`kFramesCount = 4`), `kFrameDisplayTimeAlreadyDone`, `Frame { decoded, transferred, original, yuv, nativeFrame, position, display, format, prepared }`, `markFrameDisplayed()`, `addTimelineDelay()`, `markFrameShown()`, `nextFrameDisplayTime()`, `frameWithInfo()`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_video_track.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_video_track.cpp` | 1441 | `media_streaming_video_track.cpp:L27-L29`: `kDisplaySkipped = -1`, `kFinishedPosition = i64::MAX`; `media_streaming_video_track.cpp:L33-L39`: `WorldTimeDelay(trackDelay, speed) = round(trackDelay / speed)`; `media_streaming_video_track.cpp:L41-L94`: `ConvertToARGB32` with Swscale YUV420/NV12 to BGRA; `media_streaming_video_track.cpp:L148-L158`: lock-free atomic `_counter` with modulo 2 read/write alternation for timeline delay. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_video_track.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_reader.h` | 287 | `Reader`: progressive loader chunk coordinator, `Reader::FillState { Success, WaitingCache, WaitingRemote, Failed }`, `kLoadFromRemoteMax = 8`, `PartsMap = flat_map<u32, QByteArray>`, `Slice { parts, flags }`, `Slices { _data, _header, _usedSlices, _headerMode }`, `fill()`, `startStreaming()`, `stopStreaming()`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_reader.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_reader.cpp` | 1443 | `media_streaming_reader.cpp:L20-L31`: `kPartSize = 128KB`, `kPartsInSlice = 64` (8MB slice), `kInSlice = 8MB`, `kMaxPartsInHeader = 64`, `kMaxOnlyInHeader = 80 * 128KB = 10MB`, `kPartsOutsideFirstSliceGood = 8`, `kSlicesInMemory = 2`, `kPreloadPartsAhead = 8` (1MB preload ahead of reading), `kDownloaderRequestsLimit = 4`; `media_streaming_reader.cpp:L75-L144`: binary cache serializer for contiguous and sparse parts. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_reader.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_player.h` | 225 | `Player`: overall media player state machine implementing `FileDelegate`, manages `File`, `AudioTrack`, and `VideoTrack`, `play()`, `pause()`, `resume()`, `stop()`, `stopAudio()`, `buffering()`, `playing()`, `updates()`, `frameWithInfo()`, `loadInAdvanceFor()`, `prepareLegacyState()`. | `crates/gpui_telegram_media/src/media/streaming/media_streaming_player.rs` |
| `Telegram/SourceFiles/media/streaming/media_streaming_player.cpp` | 1044 | `media_streaming_player.cpp:L22-L25`: `kBufferFor = 3000ms`, `kLoadInAdvanceForRemote = 32000ms` (32s remote buffer), `kLoadInAdvanceForLocal = 5000ms` (5s local buffer), `kMsFrequency = 1000`; `media_streaming_player.cpp:L96-L125`: `checkNextFrameRender` timer scheduler, PTS sync, and loop playback timestamp offsetting (`_loopingShift`). | `crates/gpui_telegram_media/src/media/streaming/media_streaming_player.rs` |

---

## 5. Upstream C++ 100% Reading Manifest (Part 4: Clip Subsystem — 8 Files)

| Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path |
| :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/media/clip/media_clip_check_streaming.h` | 23 | Streaming capability inspection: `CheckStreamingSupport(location, data)`. | `crates/gpui_telegram_media/src/media/clip/media_clip_check_streaming.rs` |
| `Telegram/SourceFiles/media/clip/media_clip_check_streaming.cpp` | 98 | `media_clip_check_streaming.cpp:L22-L24`: `kHeaderSize = 8`, `kFindMoovBefore = 128 * 1024` (128 KB); `media_clip_check_streaming.cpp:L39-L94`: parses MP4 box header, searches for `'moov'` atom before 128 KB offset, handles 32-bit and 64-bit box lengths. | `crates/gpui_telegram_media/src/media/clip/media_clip_check_streaming.rs` |
| `Telegram/SourceFiles/media/clip/media_clip_implementation.h` | 76 | `ReaderImplementation`: abstract clip demuxer, `Mode { Silent, Inspecting }`, `ReadResult { Success, Error, EndOfFile }`, `readFramesTill(frameMs, systemMs)`, `frameRealTime()`, `framePresentationTime()`, `renderFrame(to, hasAlpha, index, size)`, `durationMs()`, `start()`. | `crates/gpui_telegram_media/src/media/clip/media_clip_implementation.rs` |
| `Telegram/SourceFiles/media/clip/media_clip_implementation.cpp` | 32 | `media_clip_implementation.cpp:L16-L28`: `initDevice()` selects `QFile` from `_location` or `QBuffer` from memory `_data`. | `crates/gpui_telegram_media/src/media/clip/media_clip_implementation.rs` |
| `Telegram/SourceFiles/media/clip/media_clip_ffmpeg.h` | 121 | `FFMpegReaderImplementation`: inherits `ReaderImplementation`, `kMaxInMemory = 10MB`, `Rotation { None, Degrees90, Degrees180, Degrees270 }`, `readNextFrame()`, `processReadFrame()`, `renderFrame()`, `durationMs()`, `start()`, `isGifv()`, `isWebmSticker()`. | `crates/gpui_telegram_media/src/media/clip/media_clip_ffmpeg.rs` |
| `Telegram/SourceFiles/media/clip/media_clip_ffmpeg.cpp` | 511 | `media_clip_ffmpeg.cpp:L19-L27`: `kSkipInvalidDataPackets = 10`, `kMaxInlineArea = 1280 * 720`, `kMaxSendingArea = 3840 * 2160`; `media_clip_ffmpeg.cpp:L50-L75`: loop playback on `AVERROR_EOF` by seeking stream back to 0, flushing codec buffers, resetting timestamps; `media_clip_ffmpeg.cpp:L125-L146`: `processReadFrame` calculates frame presentation delay `(pts * 1000 * num) / den`. | `crates/gpui_telegram_media/src/media/clip/media_clip_ffmpeg.rs` |
| `Telegram/SourceFiles/media/clip/media_clip_reader.h` | 249 | `Reader`: top-level animated clip engine, `State { Reading, Error, Finished }`, `FrameRequest` (frame, outer, factor, radius, colored, keepAlpha), `kWaitingForDimensionsStep = -3`, `kWaitingForRequestStep = -2`, `kWaitingForFirstFrameStep = -1`, `Notification { Reinit, Repaint }`, `FrameInfo`, `ReaderPointer`, triple-buffer `_frames[3]`, `_step` atomic state machine. | `crates/gpui_telegram_media/src/media/clip/media_clip_reader.rs` |
| `Telegram/SourceFiles/media/clip/media_clip_reader.cpp` | 1066 | `media_clip_reader.cpp:L36-L39`: `kClipThreadsCount = 8`, `kAverageGifSize = 320 * 240`, `kWaitBeforeGifPause = 200ms`; `media_clip_reader.cpp:L40-L125`: `PrepareFrame` resizing, rounded corners, and transparent background letterboxing; `media_clip_reader.cpp:L148-L175`: triple-buffered frame step transitions `show = ((step + 1) / 2) % 3` and `write = ((step + 3) / 2) % 3`. | `crates/gpui_telegram_media/src/media/clip/media_clip_reader.rs` |


## 6. Dynamic Rust / GPUI Architecture & Module Mapping

Under Mandate #17, the 36 C++ upstream files are mapped to idiomatic, safe Rust modules in `crates/gpui_telegram_media/src/media/`:

```
crates/gpui_telegram_media/src/media/
├── streaming/
│   ├── mod.rs                                    # Re-exports streaming types and ledger target symbols
│   ├── media_streaming_common.rs                 # Mode, PlaybackOptions, TrackState, FrameRequest, FrameWithInfo, Error
│   ├── media_streaming_file_delegate.rs          # FileDelegate trait for demuxer callbacks
│   ├── media_streaming_loader.rs                 # LoadedPart, Loader trait, PriorityQueue
│   ├── media_streaming_loader_local.rs           # LoaderLocal (file/memory progressive reader)
│   ├── media_streaming_loader_mtproto.rs         # LoaderMtproto (segmented cloud download & speed estimator)
│   ├── media_streaming_utility.rs                # Frame sizing, aspect ratio, blurred bg, packet skipping
│   ├── media_streaming_round_preview.rs          # RoundPreview video message dynamic renderer
│   ├── media_streaming_native_frame_mac.rs       # macOS NV12 CVPixelBuffer conversion logic
│   ├── media_streaming_document.rs               # Document streaming coordinator, adaptive bitrate switcher
│   ├── media_streaming_file.rs                   # Multi-thread custom AVIO demuxer with semaphore synchronization
│   ├── media_streaming_instance.rs               # Instance consumer handles with priority routing
│   ├── media_streaming_audio_track.rs            # AudioTrack with mixer enqueuing and PTS synchronization
│   ├── media_streaming_video_track.rs            # VideoTrack with 4-frame ring buffer & lock-free counter
│   ├── media_streaming_reader.rs                 # Reader with 128KB chunks, 8MB slices, LRU cache
│   └── media_streaming_player.rs                 # Player top-level state machine (32s remote buffer, 5s local buffer)
└── clip/
    ├── mod.rs                                    # Re-exports clip types and ledger target symbols
    ├── media_clip_check_streaming.rs             # Fast-start MP4 moov atom validation (< 128 KB)
    ├── media_clip_implementation.rs              # ReaderImplementation abstraction
    ├── media_clip_ffmpeg.rs                      # FFMpegReaderImplementation (looping GIF/video reader)
    └── media_clip_reader.rs                      # Reader with triple-buffered frames and -3..5 FSM
```

All 19 target `.rs` files replace the mechanical `{ id, label, value }` template files with authentic, zero-stub Rust structures that fulfill all Ledger Target Symbols (`MediaStreaming*Streaming` and `MediaClip*Clip`).

---

## 7. Mathematical Formulas, Buffering Constants & Thread Synchronization State Machines

### 7.1 Chunking & Slice Calculations
- Chunk Part Size: `kPartSize = 128 * 1024` (131,072 bytes).
- Slices: Each slice holds `kPartsInSlice = 64` parts = 8,388,608 bytes (8 MB).
- Slice Count: `SlicesCount(size) = (size + 8388608 - 1) / 8388608`.
- Maximum Slices Retained in Memory: `kSlicesInMemory = 2` (16 MB maximum RAM footprint for cached file chunks).
- Preload Window: `kPreloadPartsAhead = 8` (1 MB read-ahead before playback position).

### 7.2 Download Speed & Adaptive Bitrate Estimation
- Speed calculation over active windows:
  $$\text{speed} = \operatorname{clamp}\left(\frac{\text{received\_bytes} \times 1000}{\Delta t_{\text{active}}}, 0, 64 \times 1024 \times 1024\right)$$
- Unreliability threshold: Marked `unreliable = true` if `received_bytes < 3 \times 128\text{ KB} = 384\text{ KB}`.
- Adaptive Quality Switch:
  - Upscale: `speed >= descriptor.bitrate * current_speed * 1.2` when `receivedTill >= position + 4000ms`.
  - Downscale: Buffer underrun (`waiting == true`) triggers fallback to highest descriptor where `size < current_size`.

### 7.3 Video & Clip Frame Synchronization
- Time Delay Adjustment:
  $$\text{WorldTimeDelay}(\text{trackDelay}, \text{speed}) = \operatorname{round}\left(\frac{\text{trackDelay}}{\text{speed}}\right)$$
- Triple-Buffer Atomic Step Transition in `Clip::Reader`:
  $$\text{show\_index} = \left(\frac{\text{step} + 1}{2}\right) \bmod 3, \quad \text{write\_index} = \left(\frac{\text{step} + 3}{2}\right) \bmod 3$$

---

## 8. Subsystem Test Plan & Verification Gate

The test suite in `crates/gpui_telegram_media/tests/media_streaming_clip_subsystem_tests.rs` tests the real behavioral invariants of Module 09 Slice 2:
1. `test_priority_queue_ordering_and_range_extraction`: Verifies high priority wins, tiebreaking by lower offset, and `takeInRange`.
2. `test_loaded_part_validation`: Verifies 128KB chunk boundary validation and end-of-file validation.
3. `test_loader_mtproto_speed_estimation`: Tests non-overlapping active download duration and 64 MB/s clamping.
4. `test_reader_slice_index_math`: Asserts slice counts and part boundaries for 8 MB slices.
5. `test_check_streaming_support_moov_atom`: Verifies detection of MP4 `'moov'` atom within 128 KB and rejection of fragmented/remote atoms.
6. `test_clip_reader_triple_buffer_fsm`: Tests atomic step transitions (`-3..5`) and show/write index alternating.
7. `test_adaptive_bitrate_switching_logic`: Tests upscale at 1.2x multiplier and downscale on buffering.
8. `test_frame_resize_and_aspect_expansion`: Tests aspect ratio expansion logic and letterbox calculations.
9. `test_streaming_player_buffering_thresholds`: Verifies 32s remote buffer vs 5s local buffer and 3s resume threshold.
10. `test_playback_options_clamping_and_validation`: Tests speed (0.5..2.0) and volume (0.0..1.0) boundaries.
