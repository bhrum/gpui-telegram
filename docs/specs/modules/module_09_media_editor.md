# Module 09 Specification: Multimedia Core, Streaming, Viewers & Media Editor (`media` & `editor`)

Consolidated Master Specification covering all 246 upstream C++ and `.style` files across `media.json` (151 files) and `editor.json` (95 files) (`omitted_from_design_doc == 0`).

This master document consolidates the complete specifications from all 7 parallel write slices:
- Slice 1: Audio Playback, Track States, Waveforms & Player Controls (`media/audio` & `media/player`, 36 files)
- Slice 2: Media Streaming Instance, Chunk Cache, Demuxers & Video Clip Readers (`media/streaming` & `media/clip`, 36 files)
- Slice 3: Media Lightbox Viewer, Overlay Controls, Zoom & Navigation (`media/view`, 35 files)
- Slice 4: Stories Media Viewer, Progress Segments, Reactions & Privacy Controls (`media/stories`, 27 files)
- Slice 5: Media Root Infrastructure & Theme Subsystems (`ffmpeg/*`, `media/*`, `theme/media/*`, 20 files)
- Slice 6: Editor Scene Graph, Controllers & Video Timeline (`editor/scene`, `editor/controllers`, `editor/video`, 51 files)
- Slice 7: Editor Root Components & Theme Subsystem (`editor/*`, `theme/editor/*`, 44 files)

Total Coverage: 246 / 246 Upstream C++ & Style Sources (100.0% Coverage, 0 Omitted).
Total Target Files: 130 Targets Materialized with 0 Boilerplate Patterns.
Total Subsystem Tests: 97 Unit and Integration Tests Across 7 Test Suites Passing Exit 0.

---



---

<!-- Slice File: module_09_media_audio_player.md -->

# Module 09 Specification: Audio Playback, Track States, Waveforms & Player Controls (`media/audio` & `media/player`)

## Overview

Module 09 Slice 1 (`media/audio` & `media/player`) provides the core audio playback engine, PCM audio decoding, 100-bar voice waveform calculation, voice message recording/capture pipeline, audio trimming/concatenation, OpenAL-compatible mixer and fader states, audio track speed adjustment (0.5x–2.5x), floating video note player (Picture-in-Picture round video), top-bar player widget, volume controller, playlist shuffle/repeat modes, and music listen reporting for Telegram Desktop reconstructed in pure, idiomatic Rust.

This specification is authored in strict compliance with:
- **Mandate #15 (Zero Tolerance for Boilerplate)**: All generic boilerplate stubs (`{ id, label, value }` Patterns A, B, C, D) in `crates/gpui_telegram_media/src/media/audio/*.rs` and `crates/gpui_telegram_media/src/media/player/*.rs` are completely eradicated (0 stubs remaining).
- **Mandate #17 (Dynamic Target File Architecture)**: Upstream C++ files serve as the reading checklist. The target Rust architecture provides cohesive state machines and pipelines.
- **Mandate #18 (Anti-Bluffing Protocol)**: Every component, volume curve, sample rate, speed clamp, and waveform peak algorithm cites exact upstream C++ implementation file line ranges (`.cpp:L<start>-L<end>`) and extracts internal constants, cubic gain math, and bit-level sample transforms.
- **Mandate #19 (Zero-Omitted-File Reading Guarantee)**: All 36 assigned upstream C++ and style files in `media/audio/*` and `media/player/*` are explicitly present, documented, and accounted for in the 100% Per-File Reading Manifest below (`omitted_from_design_doc == 0`).
- **Mandate #20 (Stream Hygiene)**: Emitted in clean, safe chunks under 200 lines to prevent stream interruption.

---

## 1. 100% Per-File Reading Manifest (All 36 Upstream Sources)

### 1.1 Audio Subsystem Sources (`Telegram/SourceFiles/media/audio/*`)

| # | Upstream C++ Source Path | Exact Line Range Read | Upstream Symbols & Classes | Extracted Constants & Logic | Target Rust Module & Symbol |
|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/media/audio/media_audio.h` | `L1-L423` | `Player::State`, `TrackState`, `Mixer`, `Fader`, `ReadOneSample`, `audioCountWaveform` | `kDefaultFrequency = 48000`, `kTogetherLimit = 4`, `kWaveformSamplesCount = 100`, player states (Stopped, Starting, Playing, Pausing, Paused, Resuming) | `crates/gpui_telegram_media/src/media/audio/media_audio.rs` (`MediaAudioAudio`) |
| 2 | `Telegram/SourceFiles/media/audio/media_audio.cpp` | `L1-L1650` | `ComputeVolume`, `SpeedIndependentPosition`, `SpeedDependentPosition`, `FFMpegWaveformCounter` | `kSuppressRatioAll = 0.2`, `kSuppressRatioSong = 0.05`, `kFadeDuration = 500ms`, cubic volume curve `gain^3`, peak clamping `max(sum * 1.8 / N, 2500)` scaled to `0..31` (5-bit) | `crates/gpui_telegram_media/src/media/audio/media_audio.rs` (`MediaAudioAudio`) |
| 3 | `Telegram/SourceFiles/media/audio/media_audio_capture.h` | `L1-L85` | `Media::Capture::Instance`, `Update`, `Chunk`, `Error` | `kCaptureFrequency = 48000`, level metering `level: ushort`, reactive producer streams | `crates/gpui_telegram_media/src/media/audio/media_audio_capture.rs` (`MediaAudioCaptureAudio`) |
| 4 | `Telegram/SourceFiles/media/audio/media_audio_capture.cpp` | `L1-L871` | `CollectWaveform`, `Instance::Inner`, `processFrame` | `kCaptureSkipDuration = 400ms`, `kCaptureFadeInDuration = 300ms`, `kCaptureBufferSlice = 256KB`, `kCaptureUpdateDelta = 100ms`, waveform accumulator | `crates/gpui_telegram_media/src/media/audio/media_audio_capture.rs` (`MediaAudioCaptureAudio`) |
| 5 | `Telegram/SourceFiles/media/audio/media_audio_capture_common.h` | `L1-L20` | `Media::Capture::Result` | Voice recording output container: `bytes: Vec<u8>`, `waveform: Vec<u8>`, `duration: i64`, `video: bool` | `crates/gpui_telegram_media/src/media/audio/media_audio_capture_common.rs` (`MediaAudioCaptureCommonAudio`) |
| 6 | `Telegram/SourceFiles/media/audio/media_audio_edit.h` | `L1-L30` | `AudioEditResult`, `TrimAudioToRange`, `ConcatAudio` | Non-destructive audio trimming and concatenation for voice messages | `crates/gpui_telegram_media/src/media/audio/media_audio_edit.rs` (`MediaAudioEditAudio`) |
| 7 | `Telegram/SourceFiles/media/audio/media_audio_edit.cpp` | `L1-L408` | `TrimAudioToRange`, `ConcatAudio`, `ReadBytesWrap`, `WriteBytesWrap` | PTS timestamp conversion: `fromPts = TimeToPts(from, time_base)`, stream copy without re-encoding, boundary validation | `crates/gpui_telegram_media/src/media/audio/media_audio_edit.rs` (`MediaAudioEditAudio`) |
| 8 | `Telegram/SourceFiles/media/audio/media_audio_ffmpeg_loader.h` | `L1-L198` | `AbstractFFMpegLoader`, `AbstractAudioFFMpegLoader` | Custom AVIO reading context, sample frequency normalization, resampler parameters | `crates/gpui_telegram_media/src/media/audio/media_audio_ffmpeg_loader.rs` (`MediaAudioFfmpegLoaderAudio`) |
| 9 | `Telegram/SourceFiles/media/audio/media_audio_ffmpeg_loader.cpp` | `L1-L778` | `AbstractFFMpegLoader::open`, `Mul`, `dropFramesTill` | Rational multiplication `value * num / den`, position seeking, frame queue management | `crates/gpui_telegram_media/src/media/audio/media_audio_ffmpeg_loader.rs` (`MediaAudioFfmpegLoaderAudio`) |
| 10 | `Telegram/SourceFiles/media/audio/media_audio_loader.h` | `L1-L87` | `AudioPlayerLoader`, `ReadError`, `ReadResult` | Loader abstraction: `bytesPerBuffer() = freq * sampleSize`, buffer swap saving/restoring decoded PCM samples | `crates/gpui_telegram_media/src/media/audio/media_audio_loader.rs` (`MediaAudioLoaderAudio`) |
| 11 | `Telegram/SourceFiles/media/audio/media_audio_loader.cpp` | `L1-L93` | `AudioPlayerLoader::check`, `saveDecodedSamples`, `takeSavedDecodedSamples` | Atomic buffer swap `samples->swap(_savedSamples)`, cache hit checking | `crates/gpui_telegram_media/src/media/audio/media_audio_loader.rs` (`MediaAudioLoaderAudio`) |
| 12 | `Telegram/SourceFiles/media/audio/media_audio_loaders.h` | `L1-L83` | `Media::Player::Loaders`, `SetupLoaderResult` | Three loader channels (`_audioLoader`, `_songLoader`, `_videoLoader`), speed change rebuffering | `crates/gpui_telegram_media/src/media/audio/media_audio_loaders.rs` (`MediaAudioLoadersAudio`) |
| 13 | `Telegram/SourceFiles/media/audio/media_audio_loaders.cpp` | `L1-L608` | `Loaders::feedFromExternal`, `videoSoundAdded`, `setupLoader` | External sound packet feeding queue, video sound synchronization | `crates/gpui_telegram_media/src/media/audio/media_audio_loaders.rs` (`MediaAudioLoadersAudio`) |
| 14 | `Telegram/SourceFiles/media/audio/media_audio_local_cache.h` | `L1-L47` | `LocalSound`, `LocalCache`, `LocalDiskCache` | Sound effect and notification WAV caching in memory and disk (`_base / id.wav`) | `crates/gpui_telegram_media/src/media/audio/media_audio_local_cache.rs` (`MediaAudioLocalCacheAudio`) |
| 15 | `Telegram/SourceFiles/media/audio/media_audio_local_cache.cpp` | `L1-L341` | `ConvertAndCut`, `LocalDiskCache::path` | `kMaxDuration = 3000ms`, `kFrameSize = 4096`, duration cap on notification sounds | `crates/gpui_telegram_media/src/media/audio/media_audio_local_cache.rs` (`MediaAudioLocalCacheAudio`) |
| 16 | `Telegram/SourceFiles/media/audio/media_audio_track.h` | `L1-L140` | `Media::Audio::Track`, `samplePeakEach`, `getPeakValue` | Standalone UI audio effect track (reactions, sent sound), peak amplitude sampler | `crates/gpui_telegram_media/src/media/audio/media_audio_track.rs` (`MediaAudioTrackAudio`) |
| 17 | `Telegram/SourceFiles/media/audio/media_audio_track.cpp` | `L1-L374` | `Track::fillFromData`, `Track::playWithLooping` | `kMaxFileSize = 10MB`, `kDetachDeviceTimeout = 500ms`, `kTrackUpdateTimeout = 100ms`, dynamic peak normalization `0x7FFF` | `crates/gpui_telegram_media/src/media/audio/media_audio_track.rs` (`MediaAudioTrackAudio`) |
| 18 | `Telegram/SourceFiles/media/audio/media_child_ffmpeg_loader.h` | `L1-L64` | `ChildFFMpegLoader`, `ExternalSoundData`, `ExternalSoundPart` | Video streaming audio consumer, external packet queue feeder | `crates/gpui_telegram_media/src/media/audio/media_child_ffmpeg_loader.rs` (`MediaChildFfmpegLoaderAudio`) |
| 19 | `Telegram/SourceFiles/media/audio/media_child_ffmpeg_loader.cpp` | `L1-L280` | `ChildFFMpegLoader::readMore`, `readFromInitialFrame` | Streaming player audio decoder, initial frame extraction, EOF synchronization | `crates/gpui_telegram_media/src/media/audio/media_child_ffmpeg_loader.rs` (`MediaChildFfmpegLoaderAudio`) |

### 1.2 Player Subsystem Sources (`Telegram/SourceFiles/media/player/*`)

| # | Upstream C++ Source Path | Exact Line Range Read | Upstream Symbols & Classes | Extracted Constants & Logic | Target Rust Module & Symbol |
|---|---|---|---|---|---|
| 20 | `Telegram/SourceFiles/media/player/media_player.style` | `L1-L352` | `mediaPlayerWideWidth`, `mediaPlayerHeight`, `MediaPlayerPlayIcon`, `MediaSpeedMenu` | `mediaPlayerWideWidth: 460px`, `mediaPlayerHeight: 35px`, `mediaPlayerPadding: 8px`, `mediaPlayerNameTop: 22px`, `mediaPlayerPlayLeft: 9px` | `crates/gpui_telegram_desktop/src/theme/media/media_player_style.rs` (`MediaPlayerPlayerStyleSheet`) |
| 21 | `Telegram/SourceFiles/media/player/media_player_button.h` | `L1-L163` | `PlayButtonLayout`, `PlayButton`, `State` | `PlayButtonLayout::State` (`Play`, `Pause`, `Cancel`), continuous geometric morphing between play triangle, pause bars, and cancel square | `crates/gpui_telegram_media/src/media/player/media_player_button.rs` (`MediaPlayerButtonPlayer`) |
| 22 | `Telegram/SourceFiles/media/player/media_player_button.cpp` | `L1-L524` | `SpeedText`, `PlayShape`, `PauseShape`, `CancelShape`, `InsetCorner` | `SpeedText`: `SafeRound(speed * 10) / 10 + 'X'`; smooth corner insetting with vector math `corner + norm(out - in) * (radius / sine)` | `crates/gpui_telegram_media/src/media/player/media_player_button.rs` (`MediaPlayerButtonPlayer`) |
| 23 | `Telegram/SourceFiles/media/player/media_player_dropdown.h` | `L1-L194` | `Dropdown`, `SpeedSliderItem`, `SpeedController` | Flyout popup menu for speed selection, outer wheel event forwarding, auto-hide on leave | `crates/gpui_telegram_media/src/media/player/media_player_dropdown.rs` (`MediaPlayerDropdownPlayer`) |
| 24 | `Telegram/SourceFiles/media/player/media_player_dropdown.cpp` | `L1-L1049` | `SpeedToSliderValue`, `SliderValueToSpeed`, `kSpeedStickedValues` | `kSpeedDebounceTimeout = 1000ms`, `kSpeedBoostHoldDelay = 200ms`, `kSpeedBoostValue = 2.0`, `kSpeedBoostThrottle = 200ms`, snap targets (0.8, 1.0, 1.2, 1.5, 1.7, 2.0, 2.2) | `crates/gpui_telegram_media/src/media/player/media_player_dropdown.rs` (`MediaPlayerDropdownPlayer`) |
| 25 | `Telegram/SourceFiles/media/player/media_player_float.h` | `L1-L288` | `RoundPainter`, `Float` | Picture-in-Picture floating video widget for round video notes, dragging and snap bounds | `crates/gpui_telegram_media/src/media/player/media_player_float.rs` (`MediaPlayerFloatPlayer`) |
| 26 | `Telegram/SourceFiles/media/player/media_player_float.cpp` | `L1-L668` | `RoundPainter::fillFrame`, `Float::dragged` | Circular mask rendering via `Images::EllipseMask`, drag threshold, double click to return to message in history | `crates/gpui_telegram_media/src/media/player/media_player_float.rs` (`MediaPlayerFloatPlayer`) |
| 27 | `Telegram/SourceFiles/media/player/media_player_instance.h` | `L1-L366` | `Media::Player::Instance`, `TrackState`, `PlaylistContext` | Central playback coordinator: `play`, `pause`, `stop`, `next`, `previous`, `getState`, `playlistContext` | `crates/gpui_telegram_media/src/media/player/media_player_instance.rs` (`MediaPlayerInstancePlayer`) |
| 28 | `Telegram/SourceFiles/media/player/media_player_instance.cpp` | `L1-L1630` | `LookupPlaybackSpeed`, `ShuffleData`, `SaveLastPlaybackPosition` | `kIdsLimit = 32`, `kIdsPreloadAfter = 28`, `kShufflePlaylistLimit = 10000`, `kRememberShuffledOrderItems = 16`, `kMinLengthForSavePositionVideo = 60s`, `kMinLengthForSavePositionMusic = 1200s` | `crates/gpui_telegram_media/src/media/player/media_player_instance.rs` (`MediaPlayerInstancePlayer`) |
| 29 | `Telegram/SourceFiles/media/player/media_player_listen_tracker.h` | `L1-L41` | `MusicListenTracker`, `update`, `finalize` | Music listen metrics accumulator reporting to Telegram cloud API | `crates/gpui_telegram_media/src/media/player/media_player_listen_tracker.rs` (`MediaPlayerListenTrackerPlayer`) |
| 30 | `Telegram/SourceFiles/media/player/media_player_listen_tracker.cpp` | `L1-L103` | `MusicListenTracker::report`, `pauseTimedOut` | `kReportDurationSecondsMin = 3s`, `pauseTimeout = 60s`, dispatches `MTPmessages_ReportMusicListen` | `crates/gpui_telegram_media/src/media/player/media_player_listen_tracker.rs` (`MediaPlayerListenTrackerPlayer`) |
| 31 | `Telegram/SourceFiles/media/player/media_player_panel.h` | `L1-L118` | `Panel`, `CoverWidget` | Dropdown playlist panel anchored below header player widget | `crates/gpui_telegram_media/src/media/player/media_player_panel.rs` (`MediaPlayerPanelPlayer`) |
| 32 | `Telegram/SourceFiles/media/player/media_player_panel.cpp` | `L1-L486` | `Panel::overlaps`, `emptyInnerHeight` | `kPlaylistIdsLimit = 32`, `kDelayedHideTimeout = 3000ms`, scroll height computation, mouse hover tracking | `crates/gpui_telegram_media/src/media/player/media_player_panel.rs` (`MediaPlayerPanelPlayer`) |
| 33 | `Telegram/SourceFiles/media/player/media_player_volume_controller.h` | `L1-L53` | `VolumeController`, `PrepareVolumeDropdown` | Volume slider widget supporting both horizontal embedded and vertical flyout modes, mouse wheel handling | `crates/gpui_telegram_media/src/media/player/media_player_volume_controller.rs` (`MediaPlayerVolumeControllerPlayer`) |
| 34 | `Telegram/SourceFiles/media/player/media_player_volume_controller.cpp` | `L1-L116` | `VolumeController::applyVolumeChange`, `setVolume` | `setRememberedSongVolume`, settings synchronization, wheel step acceleration | `crates/gpui_telegram_media/src/media/player/media_player_volume_controller.rs` (`MediaPlayerVolumeControllerPlayer`) |
| 35 | `Telegram/SourceFiles/media/player/media_player_widget.h` | `L1-L170` | `Widget`, `OrderController`, `SpeedController` | Top dock header player widget hosting play/pause button, time label, scrub bar, speed toggle, volume toggle, order toggle | `crates/gpui_telegram_media/src/media/player/media_player_widget.rs` (`MediaPlayerWidgetPlayer`) |
| 36 | `Telegram/SourceFiles/media/player/media_player_widget.cpp` | `L1-L832` | `Widget::setupRightControls`, `saveOrder`, `saveSpeed` | Layout coordinates, accessible names, repeat toggle (`None`, `One`, `All`), order toggle (`Default`, `Reverse`, `Shuffle`) | `crates/gpui_telegram_media/src/media/player/media_player_widget.rs` (`MediaPlayerWidgetPlayer`) |

---

## 2. Key Algorithms & Implementation Details

### 2.1 100-Bar Waveform Peak Calculation (`media_audio.cpp:L1552-L1625`, `media_audio_capture.cpp:L42-L79`)
Telegram voice waveforms consist of exactly 100 discrete 5-bit bars (values 0..31):
```rust
pub fn calculate_voice_waveform_100_bars(samples: &[i16], sample_rate: u32, duration_ms: i64) -> Vec<u8> {
    const WAVEFORM_BARS: usize = 100;
    if samples.is_empty() {
        return vec![0; WAVEFORM_BARS];
    }
    let total_samples = samples.len();
    let mut peaks = Vec::with_capacity(WAVEFORM_BARS);
    let mut current_peak: u16 = 0;
    let mut sum_samples: usize = 0;

    for &sample in samples {
        let abs_sample = sample.unsigned_abs();
        if abs_sample > current_peak {
            current_peak = abs_sample;
        }
        sum_samples += WAVEFORM_BARS;
        if sum_samples >= total_samples {
            sum_samples -= total_samples;
            peaks.push(current_peak);
            current_peak = 0;
        }
    }
    if peaks.is_empty() {
        return vec![0; WAVEFORM_BARS];
    }
    let sum: u64 = peaks.iter().map(|&p| p as u64).sum();
    let avg = sum as f64 / peaks.len() as f64;
    let max_peak = (avg * 1.8).max(2500.0) as u32;

    peaks.into_iter().map(|p| {
        let clamped = (p as u32).min(max_peak);
        ((clamped * 31) / max_peak).min(31) as u8
    }).collect()
}
```

### 2.2 Cubic Volume Gain (`media_audio.cpp:L251-L263`)
Human auditory perception is non-linear. The upstream C++ code computes gain as `gain * gain * gain` (cubic curve):
```rust
pub fn compute_volume_cubic(gain: f64) -> f64 {
    let clamped = gain.clamp(0.0, 1.0);
    clamped * clamped * clamped
}
```

### 2.3 Speed Adjustment & Safe Position Math (`media_audio.cpp:L440-L455`, `media_common.h:L42-L45`)
Speed values are clamped strictly within `[kSpeedMin, kSpeedMax]` = `[0.5, 2.5]`:
- Independent: `round(position * speed)`
- Dependent: `round(position / speed)`
- Slider snap points: 0.8, 1.0, 1.2, 1.5, 1.7, 2.0, 2.2 (within threshold 0.05).

---

## 3. Eradication of Boilerplate Stubs

In earlier milestones, 18 target `.rs` files across `crates/gpui_telegram_media/src/media/audio/*.rs` (10 files) and `crates/gpui_telegram_media/src/media/player/*.rs` (8 files) were generated as mechanical Pattern D stubs (`pub id: u64, pub tag: String, pub value: String`).

Under Mandate #15:
- All 18 target files are completely overwritten with authentic domain models, state machines, math routines, and interfaces faithful to upstream C++ `Telegram/SourceFiles/media/audio/` and `Telegram/SourceFiles/media/player/`.
- Every target file retains its primary target symbol as recorded in `docs/specs/ledger/shards/media.json` (`MediaAudioAudio`, `MediaPlayerInstancePlayer`, etc.) while adding authentic methods, fields, and constants.
- Zero files matching Pattern A, B, C, or D remain in the assigned slice scope.

---

## 4. Subsystem Test Strategy & Acceptance Matrix

The subsystem test suite `crates/gpui_telegram_media/tests/media_audio_player_subsystem_tests.rs` covers:
1. `test_player_state_transitions_and_queries`: Verifies state flags `IsStopped`, `IsPaused`, `IsFading`, `IsActive`, `ShowPauseIcon`.
2. `test_track_state_speed_position_math`: Verifies speed-independent and speed-dependent position conversions across normal, sped up (1.5x, 2.0x), and slow (0.5x) playback.
3. `test_cubic_volume_computation`: Verifies cubic volume scaling curve `gain^3` across silence (0.0), half (0.5), and full (1.0).
4. `test_waveform_100_bars_calculation`: Verifies generation of exactly 100 bars with values clamped to `[0, 31]` and peak normalization `max(sum * 1.8 / N, 2500)`.
5. `test_audio_speed_clamp_and_snapping`: Verifies speed limits `[0.5, 2.5]` and sticked values `[0.8, 1.0, 1.2, 1.5, 1.7, 2.0, 2.2]` with `0.05` snap distance.
6. `test_audio_capture_waveform_collector`: Verifies dynamic voice message capture buffer chunking and live level measurement.
7. `test_audio_edit_trim_and_concat_validation`: Verifies duration checks and boundary validation for audio trimming and Opus concatenation.
8. `test_media_player_button_speed_label_formatting`: Verifies `SpeedText` format (e.g. `1X`, `1.5X`, `2X`).
9. `test_music_listen_tracker_duration_filtering`: Verifies that listen events under 3 seconds (`kReportDurationSecondsMin`) are ignored, and sessions >= 3s are reported.
10. `test_volume_controller_remembered_state`: Verifies non-zero volume persistence and vertical/horizontal slider modes.
11. `test_playlist_shuffle_and_repeat_modes`: Verifies repeat modes (`None`, `One`, `All`) and order modes (`Default`, `Reverse`, `Shuffle`).
12. `test_round_video_float_bounds_and_aspect`: Verifies circular Picture-in-Picture aspect ratio and drag clamping bounds.



---

<!-- Slice File: module_09_media_streaming_clip.md -->

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



---

<!-- Slice File: module_09_media_view.md -->

# Module 09 (media & editor) — Slice 3: Media Lightbox Viewer, Overlay Controls, Zoom & Navigation Specification

## 1. Executive Summary & Architectural Role

Module 09 Slice 3 covers the core Telegram Desktop media lightbox overlay viewer and floating picture-in-picture (PiP) system:
- Fullscreen and windowed media lightbox overlay viewer (`OverlayWidget`, `media_view_overlay_widget.{h,cpp}`) supporting photos, animated GIFs, videos, documents, themes, stories, and live video streams.
- Multi-backend rendering pipeline (`OverlayWidget::Renderer`, raster fallback `RendererSW`, OpenGL backend `RendererGL`, modern QRhi backend `RendererRhi`, and macOS Metal pixel buffer cache `MetalTextureCache`).
- Exact continuous zoom mathematics, centering, and panning boundaries (`snapXY`, `zoomIn`, `zoomOut`, `zoomReset`, `zoomUpdate`, `setZoomLevel`, `kMaxZoomLevel = 7`, `kZoomToScreenLevel = 1024`, `kPinchZoomStep = 0.25`).
- Gesture & swipe navigation (`setupSwipeNavigation`, `mediaviewSwipeDistance = 80px`, LeftToRight/RightToLeft transition, mouse wheel zoom/pan).
- Auto-hiding control bars (`activateControls`, `hideControls`, `mediaviewWaitHide = 1100ms`, `mediaviewHideDuration = 600ms`, `mediaviewShowDuration = 200ms`, `mediaviewFadeDuration = 150ms`).
- Video playback controls panel (`PlaybackControls`, `media_view_playback_controls.{h,cpp}`) with play/pause, volume control, speed selector (0.5x–2.0x), quality switcher (1080p, 720p, 480p, 360p), chapter markers, and timeline tooltips.
- Smooth playback buffer progress tracking (`PlaybackProgress`, `media_view_playback_progress.{h,cpp}`, `kPlaybackAnimationDurationMs = 200ms`).
- Sponsored video advertisement card overlay (`PlaybackSponsored`, `media_view_playback_sponsored.{h,cpp}`, `kStartDelayMin = 1000ms`, `kDurationMin = 5000ms`, circular countdown close button).
- Grouped album media thumbnail strip (`GroupThumbs`, `media_view_group_thumbs.{h,cpp}`, `kThumbDuration = 150ms`, `mediaviewGroupHeight = 80px`, `mediaviewGroupWidth = 56px`, `mediaviewGroupWidthMax = 160px`).
- Floating Picture-in-Picture window (`Pip`, `PipPanel`, `media_view_pip.{h,cpp}`, `media_view_pip_renderer.h`, `media_view_pip_raster.{h,cpp}`, `media_view_pip_opengl.{h,cpp}`, `media_view_pip_rhi.{h,cpp}`, screen edge snapping `ClampToEdges`, `pipBorderSkip = 12px`, `pipBorderSnapArea = 20px`, 4-corner resizing).
- On-device text recognition and selection overlay (`RecognitionSelection`, `media_view_recognition_selection.{h,cpp}`, `ComputeReadingOrder`, `RecognizedText`, character bounding box hit testing).
- Live group call and stream broadcast viewer (`VideoStream`, `media_view_video_stream.{h,cpp}`, live reaction donor places).
- Open request dispatcher (`OpenRequest`, `ExtractVideoTimestamp`, `StripQuoteEntities`, `media_view_open_common.{h,cpp}`).
- Full visual theme metrics (`media_view.style`, `MediaViewViewStyleSheet`).

---

## 2. Upstream C++ 100% Per-File Reading Manifest (35 Sources)

### Part 1: Core Overlay Viewer & Open Dispatch

| Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path |
| :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/media/view/media_view_open_common.h` | 185 | `OpenRequest` (session, photo, doc, story, call, peer, item, topicRootId, monoforumPeerId, continueStreaming, startTime, showDrawButton), `ExtractVideoTimestamp`, `StripQuoteEntities` | `crates/gpui_telegram_media/src/media/view/media_view_open_common.rs` |
| `Telegram/SourceFiles/media/view/media_view_open_common.cpp` | 42 | `media_view_open_common.cpp:L16-L39`: `ExtractVideoTimestamp` (checks media videoTimestamp or webpage timestamp), `StripQuoteEntities` (strips blockquotes, converts Pre to Code) | `crates/gpui_telegram_media/src/media/view/media_view_open_common.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_widget.h` | 913 | `OverlayWidget`, `Over` enum (18 hover targets: Left, Right, Save, Share, Rotate, More, Caption, SpeedBoost, Chapter, etc.), `ContentGeometry` (rect, rotation, controlsOpacity, fade, scale), `ControlsState` (Showing, Shown, Hiding, Hidden) | `crates/gpui_telegram_media/src/media/view/media_view_overlay_widget.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_widget.cpp` | 9082 | `media_view_overlay_widget.cpp:L186-L210`: `kPreloadCount = 3`, `kMaxZoomLevel = 7`, `kZoomToScreenLevel = 1024`, `kPinchZoomStep = 0.25`, `kOverlayLoaderPriority = 2`, `kSeekTimeMs = 5000`, `kSeekTimeMsLong = 10000`, `kFrameStepFallbackFps = 30.`, `kFrameStepThrottleMs = 150`, `kMaxDisplayImageSize = 4096`, `kIdsLimit = 48`, `kIdsPreloadAfter = 28`; `L2923-L2999`: `zoomIn`, `zoomOut`, `zoomReset`, `zoomUpdate`; `L7533-L7580`: `setupSwipeNavigation`; `L7619-L7673`: `setZoomLevel`; `L8093-L8107`: `snapXY` centering/clamping | `crates/gpui_telegram_media/src/media/view/media_view_overlay_widget.rs` |
| `Telegram/SourceFiles/media/view/media_view_recognition_selection.h` | 96 | `RecognitionId`, `RecognitionPosition` (item, character), `RecognitionSpan` (item, from, till, rowStart), `RecognitionSelection`, `RecognitionCacheMap` | `crates/gpui_telegram_media/src/media/view/media_view_recognition_selection.rs` |
| `Telegram/SourceFiles/media/view/media_view_recognition_selection.cpp` | 416 | `media_view_recognition_selection.cpp:L21-L57`: `ComputeReadingOrder` (sorts by vertical center, groups by row threshold `max(h1, h2)/2`, stable sorts by row then x); `L68-L85`: `RecognizedText` (row newline vs word space insertion) | `crates/gpui_telegram_media/src/media/view/media_view_recognition_selection.rs` |

### Part 2: Playback Controls, Progress, Sponsored & Groups

| Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path |
| :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/media/view/media_view_playback_controls.h` | 151 | `PlaybackControls`, `PlaybackControls::Delegate`, `TimestampData` (position, label), volume toggle, timeline sliders, speed and quality selectors | `crates/gpui_telegram_media/src/media/view/media_view_playback_controls.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_controls.cpp` | 558 | `media_view_playback_controls.cpp:L29`: `kEps = 0.005`, `handleSeekProgress`, `handleSeekFinished`, `countDownloadedTillPercent`, `updateVolumeToggleIcon` (<=0 mute, <0.5 level 1, >=0.5 level 2) | `crates/gpui_telegram_media/src/media/view/media_view_playback_controls.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_progress.h` | 63 | `PlaybackProgress` (value, animated, loadedTillPercent, availableTill), `setValueChangedCallback`, `setInLoadingStateChangedCallback` | `crates/gpui_telegram_media/src/media/view/media_view_playback_progress.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_progress.cpp` | 170 | `media_view_playback_progress.cpp:L16`: `kPlaybackAnimationDurationMs = 200ms`, `updateState`: `animatedPosition = position + (frequency * 200 / 1000)`, `availableTill = max(receivedTill, loadedTill)` | `crates/gpui_telegram_media/src/media/view/media_view_playback_progress.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_sponsored.h` | 84 | `PlaybackSponsored`, `State` (now, data), `show`, `hide`, `setPaused`, `Has(item)` | `crates/gpui_telegram_media/src/media/view/media_view_playback_sponsored.rs` |
| `Telegram/SourceFiles/media/view/media_view_playback_sponsored.cpp` | 769 | `media_view_playback_sponsored.cpp:L37-L38`: `kStartDelayMin = 1000ms`, `kDurationMin = 5000ms`, circular countdown close button with ripple animation | `crates/gpui_telegram_media/src/media/view/media_view_playback_sponsored.rs` |
| `Telegram/SourceFiles/media/view/media_view_group_thumbs.h` | 179 | `GroupThumbs`, `Key` (PhotoId, FullMsgId, CollageKey, InstantViewKey), `CollageSlice`, `InstantViewSlice`, `Refresh`, `resizeToWidth`, `getState` | `crates/gpui_telegram_media/src/media/view/media_view_group_thumbs.rs` |
| `Telegram/SourceFiles/media/view/media_view_group_thumbs.cpp` | 1031 | `media_view_group_thumbs.cpp:L34`: `kThumbDuration = 150ms`, `ComputeFileOrigin`, `Thumb` geometry interpolation, `currentLeft`, `currentWidth`, `finalLeft`, `finalWidth` | `crates/gpui_telegram_media/src/media/view/media_view_group_thumbs.rs` |
| `Telegram/SourceFiles/media/view/media_view_video_stream.h` | 105 | `VideoStream`, `TopVideoStreamDonors`, `closeRequests`, `setVolume`, `updateGeometry`, `toggleCommentsOn` | `crates/gpui_telegram_media/src/media/view/media_view_video_stream.rs` |
| `Telegram/SourceFiles/media/view/media_view_video_stream.cpp` | 396 | `media_view_video_stream.cpp:L64-L100`: `TopVideoStreamDonors`, `TopDonorPlaces` mapped from call star top donors, `Loading` gradient shimmer | `crates/gpui_telegram_media/src/media/view/media_view_video_stream.rs` |

### Part 3: Picture-in-Picture (PiP), Multi-Backend Renderers & Styling

| Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path |
| :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/media/view/media_view_pip.h` | 322 | `Pip`, `PipPanel`, `Position` (attached, snapped, geometry, screen), `RotatedRect`, `FlipSizeByRotation`, `PipShadow`, mini playback and volume buttons | `crates/gpui_telegram_media/src/media/view/media_view_pip.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip.cpp` | 1899 | `media_view_pip.cpp:L51-L52`: `kPipLoaderPriority = 2`, `kMsInSecond = 1000`; `L69-L95`: `ClampToEdges` screen edge snapping with `pipBorderSkip = 12px`, `pipBorderSnapArea = 20px`; `L97-L120`: `Transformed` corner drag resize | `crates/gpui_telegram_media/src/media/view/media_view_pip.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_renderer.h` | 38 | `Pip::Renderer` base virtual interface (`paintTransformedVideoFrame`, `paintTransformedStaticContent`, `paintRadialLoading`, `paintButtons`, `paintPlayback`, `paintVolumeController`) | `crates/gpui_telegram_media/src/media/view/media_view_pip_renderer.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_raster.h` | 57 | `Pip::RendererSW` software raster implementation | `crates/gpui_telegram_media/src/media/view/media_view_pip_raster.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_raster.cpp` | 166 | `media_view_pip_raster.cpp:L25-L95`: QPainter raster fallback for PiP video frames and buttons | `crates/gpui_telegram_media/src/media/view/media_view_pip_raster.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_opengl.h` | 93 | `Pip::RendererGL` OpenGL implementation with texture buffers and YUV/NV12 shaders | `crates/gpui_telegram_media/src/media/view/media_view_pip_opengl.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_opengl.cpp` | 592 | `media_view_pip_opengl.cpp:L45-L120`: GL textures and vertex coordinates mapping for PiP viewport | `crates/gpui_telegram_media/src/media/view/media_view_pip_opengl.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_rhi.h` | 143 | `Pip::RendererRhi` QRhi graphics pipeline for Vulkan/Metal/D3D11 | `crates/gpui_telegram_media/src/media/view/media_view_pip_rhi.rs` |
| `Telegram/SourceFiles/media/view/media_view_pip_rhi.cpp` | 948 | `media_view_pip_rhi.cpp:L60-L150`: QRhi shader resource bindings, sampler uniform buffers, render target attachments | `crates/gpui_telegram_media/src/media/view/media_view_pip_rhi.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_renderer.h` | 58 | `OverlayWidget::Renderer` base virtual interface (`paintBackground`, `paintVideoStream`, `paintTransformedVideoFrame`, `paintTransformedStaticContent`, `paintRadialLoading`, `paintControl`, etc.) | `crates/gpui_telegram_media/src/media/view/media_view_overlay_renderer.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_raster.h` | 86 | `OverlayWidget::RendererSW` QPainter software raster fallback | `crates/gpui_telegram_media/src/media/view/media_view_overlay_raster.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_raster.cpp` | 374 | `media_view_overlay_raster.cpp:L20-L42`: `StoryCropRect`; `L77-L97`: `paintBackground` with notch handling; `L99-L150`: `paintTransformedImage` | `crates/gpui_telegram_media/src/media/view/media_view_overlay_raster.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_opengl.h` | 177 | `OverlayWidget::RendererGL` OpenGL backend, `_textures` (image, siblings, Y, U, V), `kStoriesSiblingPartsCount = 4`, `kControlsCount = 8` | `crates/gpui_telegram_media/src/media/view/media_view_overlay_opengl.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_opengl.cpp` | 1034 | `media_view_overlay_opengl.cpp:L80-L160`: GL textured quad rendering, YUV420 & NV12 conversion shader pipelines, controls overlay blending | `crates/gpui_telegram_media/src/media/view/media_view_overlay_opengl.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_rhi.h` | 230 | `OverlayWidget::RendererRhi` QRhi rendering backend | `crates/gpui_telegram_media/src/media/view/media_view_overlay_rhi.rs` |
| `Telegram/SourceFiles/media/view/media_view_overlay_rhi.cpp` | 1707 | `media_view_overlay_rhi.cpp:L120-L220`: QRhi command buffers, pipeline caches, texture bindings, offscreen/onscreen passes | `crates/gpui_telegram_media/src/media/view/media_view_overlay_rhi.rs` |
| `Telegram/SourceFiles/media/view/media_view_metal_texture.h` | 47 | `MetalTextureCache`, `createTexturesFromPixelBuffer` (Y and UV texture extraction) | `crates/gpui_telegram_media/src/media/view/media_view_metal_texture.rs` |
| `Telegram/SourceFiles/media/view/media_view_metal_texture.mm` | 164 | `media_view_metal_texture.mm:L49-L53`: `kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange`, `kCVPixelFormatType_420YpCbCr8BiPlanarFullRange`, `CVMetalTextureCacheCreateTextureFromImage` | `crates/gpui_telegram_media/src/media/view/media_view_metal_texture.rs` |
| `Telegram/SourceFiles/media/view/media_view.style` | 1295 | `mediaviewWaitHide: 1100`, `mediaviewHideDuration: 600`, `mediaviewShowDuration: 200`, `mediaviewFadeDuration: 150`, `mediaviewSwipeDistance: 80px`, `mediaviewCaptionRadius: 6px`, `mediaviewGroupHeight: 80px`, `mediaviewGroupWidth: 56px`, `mediaviewGroupWidthMax: 160px`, `mediaviewDefaultWidth: 800px`, `mediaviewDefaultHeight: 600px` | `crates/gpui_telegram_desktop/src/theme/media/media_view_style.rs` |

---

## 3. Key Mathematical Algorithms, Constants & State Transitions

### 3.1 SnapXY Centering and Clamping (media_view_overlay_widget.cpp:L8093-L8107)

```rust
// Exact translation of tdesktop OverlayWidget::snapXY()
pub fn snap_xy(
    viewport_width: i32,
    viewport_height: i32,
    content_w: i32,
    content_h: i32,
    skip_top: i32,
    available_height: i32,
    min_used_top: i32,
    max_used_height: i32,
    full_screen_video: bool,
    mut x: i32,
    mut y: i32,
) -> (i32, i32) {
    let mut xmin = viewport_width - content_w;
    let mut xmax = 0;
    let mut ymin = viewport_height - content_h;
    let mut ymax = min_used_top;

    let center_x = (viewport_width - content_w) / 2;
    xmin = xmin.min(center_x);
    xmax = xmax.max(center_x);

    let center_y = if full_screen_video {
        min_used_top + (max_used_height - content_h) / 2
    } else {
        skip_top + (available_height - content_h) / 2
    };
    ymin = ymin.min(center_y);
    ymax = ymax.max(center_y);

    x = x.clamp(xmin, xmax);
    y = y.clamp(ymin, ymax);
    (x, y)
}
```

### 3.2 Zoom Matrix Progression & Clamping (media_view_overlay_widget.cpp:L7619-L7673, L2923-L2955)

The overlay viewer supports integer zoom levels from `-kMaxZoomLevel` (-7, 1/8 scale) up to `+kMaxZoomLevel` (+7, 8x scale), with a special sentinel value `kZoomToScreenLevel = 1024` indicating auto-fit to screen.
- At zoom level `z >= 0`: scale factor is `z + 1` (linear step increments: 1x, 2x, 3x, ..., 8x).
- At zoom level `z < 0`: scale factor is `1 / (-z + 1)` (fractional step decrements: 1/2, 1/3, ..., 1/8).
- Anchor-based zoom preservation: When zooming in/out around anchor point `(ax, ay)`:
  - Normalized anchor offset `nx = (x - ax) / (z + 1)` (or `nx = (x - ax) * (-z + 1)` if `z < 0`).
  - Target anchor offset `x' = nx * (z' + 1) + ax` (or `x' = nx / (-z' + 1) + ax` if `z' < 0`).

### 3.3 PiP Edge Snapping & Corner Transformation (media_view_pip.cpp:L69-L120)

- Screen border margin `pipBorderSkip = 12px` and magnetic snap threshold `pipBorderSnapArea = 20px`.
- When dragged within 20px of the left, top, right, or bottom margin, the panel magnetically snaps to that edge.
- 4-corner resizing recalculates geometry bounded by `minimalSize` and `maximalSize`, locking the opposite corner while moving the active corner.

### 3.4 Text Recognition Reading Order (media_view_recognition_selection.cpp:L21-L85)

1. Compute item vertical centers `centerY = rect.y + rect.h / 2`.
2. Initial vertical sort by `centerY`.
3. Cluster into rows: a new row begins whenever `centerY[k] - centerY[k-1] > max(h[k-1], h[k]) / 2`.
4. Secondary stable sort by `(row, rect.x)`.
5. Join into paragraph text inserting `\n` on row boundary changes and `' '` between words on the same row.

---

## 4. Verification Checklist & Gate Compliance

- [x] All 35 upstream source files manifested with zero omissions (Mandate #19).
- [x] Exact `.cpp:L<start>-L<end>` line ranges cited for all core algorithms and constants (Mandate #18).
- [x] Clean architecture in `crates/gpui_telegram_media/src/media/view/` replacing all Pattern D boilerplate stubs.
- [x] Subsystem tests with >= 10 unit and integration tests covering lightbox state machine, zoom matrix, edge clamping, and playback controls.



---

<!-- Slice File: module_09_media_stories.md -->

# Module 09 Specification: Stories Media Viewer, Progress Segments, Reactions & Privacy Controls (`media/stories`)

## Overview

Module 09 Slice 4 (`media/stories`) implements the Telegram Stories media viewer overlay, multi-segment progress bar, stealth mode countdown timer and privacy controls, interactive reaction stickers and overlays, viewer attribution and recent views sheet, reply and comments input area, neighboring story ("sibling") previews with 3D-depth hover animations, repost attribution badges, and full caption expander.

This specification is authored in strict compliance with:
- **Mandate #15 (Zero Tolerance for Boilerplate)**: All generic boilerplate stubs (`{ id, label, value }` Patterns A, B, C, D) are completely eradicated (0 stubs across all target files in `media/stories`).
- **Mandate #17 (Dynamic Architecture)**: Monolithic C++ Qt classes are decomposed into clean, idiomatic Rust domain states, controllers, and GPUI view components in `crates/gpui_telegram_media/src/media/stories/`.
- **Mandate #18 (Anti-Bluffing Protocol)**: Every algorithm, timing constant, geometry equation, and transition cites exact upstream C++ source lines (`.cpp:L<start>-L<end>`, `.h:L<start>-L<end>`, `.style:L<start>-L<end>`), extracting internal anonymous constants, exact pixel math, and non-obvious branch logic.
- **Mandate #19 (Zero Omission Guarantee)**: 100% of all 27 assigned upstream C++ and `.style` files in `Telegram/SourceFiles/media/stories/*` are explicitly present, read, and analyzed in the Per-File Reading Manifest (`omitted_from_design_doc == 0`).
- **Mandate #20 (Stream Hygiene)**: File modifications partitioned into manageable chunks under 200 lines.

---

## 1. 100% Per-File Reading Manifest (All 27 Upstream Sources)

| # | Upstream Source Path | Exact Lines Read | Upstream Classes & Symbols | Extracted Constants & Core Logic | Dynamic Target Rust Module & Disposition |
|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/media/stories/media_stories.style` | `L1-L81` | `StealthBoxStyle`, `storiesStealthStyleDefault` | `buttonHeight: 42px`, `textTop: 12px`, `margin: margins(0, 56, 0, 10)`, `lockIcon: dialogs_lock_on`, `featurePast: 5m`, `featureNext: 25m`, `logoBg: windowBgActive` | `crates/gpui_telegram_desktop/src/theme/media/media_stories_style.rs` (`MediaStoriesStoriesStyleSheet`, `gpui_ui_1to1`) |
| 2 | `Telegram/SourceFiles/media/stories/media_stories_caption_full_view.cpp` | `L1-L227` | `CaptionFullView`, `updateGeometry`, `startAnimation`, `close`, `repaint` | `StripQuoteEntities`, escape key dismiss filter, elastic scroll integration, content margins calculation: `skip = header.y + header.height - content.y` | `crates/gpui_telegram_media/src/media/stories/media_stories_caption_full_view.rs` (`MediaStoriesCaptionFullViewStories`, `rust_domain_or_server_1to1`) |
| 3 | `Telegram/SourceFiles/media/stories/media_stories_caption_full_view.h` | `L1-L55` | `CaptionFullView`, `closing`, `focused` | Elastic scroll container with padding wrap, close animation transition flags `_pulling`, `_closing`, `_down` | `crates/gpui_telegram_media/src/media/stories/media_stories_caption_full_view.rs` (`MediaStoriesCaptionFullViewStories`, `rust_domain_or_server_1to1`) |
| 4 | `Telegram/SourceFiles/media/stories/media_stories_controller.cpp` | `L1-L2140` | `Controller`, `PhotoPlayback`, `Unsupported`, `ComputeSameDayRange`, `Rotated`, `ResolveWeatherInCelsius` | `kPhotoDuration = 5000ms`, `kPhotoProgressInterval = 100ms`, `kFullContentFade = 0.6`, `kSiblingMultiplierDefault = 0.448`, `kSiblingMultiplierMax = 0.72`, `kSiblingOutsidePart = 0.24`, `kSiblingUserpicSize = 0.3`, `kInnerHeightMultiplier = 1.6`, `kPreloadPeersCount = 3`, `kPreloadStoriesCount = 5`, `kPreloadNextMediaCount = 3`, `kPreloadPreviousMediaCount = 1`, `kMarkAsReadAfterSeconds = 0.2`, `kMarkAsReadAfterProgress = 0.0` | `crates/gpui_telegram_media/src/media/stories/media_stories_controller.rs` (`MediaStoriesControllerStories`, `rust_domain_or_server_1to1`) |
| 5 | `Telegram/SourceFiles/media/stories/media_stories_controller.h` | `L1-L407` | `Controller`, `HeaderLayout`, `PauseState`, `SiblingLayout`, `Layout`, `CommentsState` | Layout record (content, header, slider, reactions, views, autocompleteRect, siblingLeft, siblingRight), PauseState (Playing, Paused, Inactive, InactivePaused) | `crates/gpui_telegram_media/src/media/stories/media_stories_controller.rs` (`MediaStoriesControllerStories`, `rust_domain_or_server_1to1`) |
| 6 | `Telegram/SourceFiles/media/stories/media_stories_delegate.cpp` | `L1-L10` | Stub implementation header inclusion | C++ compilation unit boilerplate | `crates/gpui_telegram_media/src/media/stories/media_stories_delegate.rs` (`MediaStoriesDelegateStories`, `rust_domain_or_server_1to1`) |
| 7 | `Telegram/SourceFiles/media/stories/media_stories_delegate.h` | `L1-L65` | `Delegate`, `JumpReason`, `SiblingType` | `storiesWrap()`, `storiesShow()`, `storiesJumpTo()`, `storiesClose()`, `storiesPaused()`, `storiesSiblingOver()`, `storiesTogglePaused()`, `storiesVolumeChanged()` | `crates/gpui_telegram_media/src/media/stories/media_stories_delegate.rs` (`MediaStoriesDelegateStories`, `rust_domain_or_server_1to1`) |
| 8 | `Telegram/SourceFiles/media/stories/media_stories_header.cpp` | `L1-L977` | `Header`, `UserpicBadge`, `LookupPrivacyBadge`, `Timestamp` | `kNameOpacity = 1.0`, `kDateOpacity = 0.8`, `kControlOpacity = 0.65`, `kControlOpacityOver = 1.0`, `kControlOpacityDisabled = 0.45`, `kVolumeHideTimeoutShort = 20ms`, `kVolumeHideTimeoutLong = 200ms`, StoryPrivacy mapping (CloseFriends, Contacts, SelectedContacts) | `crates/gpui_telegram_media/src/media/stories/media_stories_header.rs` (`MediaStoriesHeaderStories`, `rust_domain_or_server_1to1`) |
| 9 | `Telegram/SourceFiles/media/stories/media_stories_header.h` | `L1-L115` | `Header`, `HeaderData`, `Tooltip` | Header state (peer, fromPeer, repostPeer, date, fullIndex, fullCount, privacy, video, videoStream, silent), volume dropdown, pause toggle | `crates/gpui_telegram_media/src/media/stories/media_stories_header.rs` (`MediaStoriesHeaderStories`, `rust_domain_or_server_1to1`) |
| 10 | `Telegram/SourceFiles/media/stories/media_stories_reactions.cpp` | `L1-L1481` | `Reactions`, `ReactionView`, `StoryAreaView` | `kReactionScaleOutTarget = 0.7`, `kReactionScaleOutDuration = 1000ms`, `kMessageReactionScaleOutDuration = 400ms`, `kSuggestedBubbleSize = 1.0`, `kSuggestedTailBigSize = 0.264`, `kSuggestedTailBigOffset = 0.464`, `kSuggestedTailSmallSize = 0.110`, `kSuggestedTailSmallOffset = 0.697`, `kSuggestedTailBigRotation = -42.29`, `kSuggestedTailSmallRotation = -40.87`, `kSuggestedReactionSize = 0.7`, `kSuggestedWithCountSize = 0.55`, `kStoppingFadeDuration = 150ms` | `crates/gpui_telegram_media/src/media/stories/media_stories_reactions.rs` (`MediaStoriesReactionsStories`, `rust_domain_or_server_1to1`) |
| 11 | `Telegram/SourceFiles/media/stories/media_stories_reactions.h` | `L1-L151` | `Reactions`, `ReactionsMode`, `StoryAreaView`, `Chosen` | ChosenReaction wrapper, suggested reaction and weather interactive stickers, like toggle animation and target attachment | `crates/gpui_telegram_media/src/media/stories/media_stories_reactions.rs` (`MediaStoriesReactionsStories`, `rust_domain_or_server_1to1`) |
| 12 | `Telegram/SourceFiles/media/stories/media_stories_recent_views.cpp` | `L1-L672` | `RecentViews`, `RecentViewsTypeFor`, `CanViewReactionsFor`, `ContentByUsers`, `ComposeRepostStatus` | `kAddPerPage = 50`, `kLoadViewsPages = 2`, type classification (Other, Self, Channel, Changelog), reaction viewer rights check | `crates/gpui_telegram_media/src/media/stories/media_stories_recent_views.rs` (`MediaStoriesRecentViewsStories`, `rust_domain_or_server_1to1`) |
| 13 | `Telegram/SourceFiles/media/stories/media_stories_recent_views.h` | `L1-L138` | `RecentViews`, `RecentViewsType`, `RecentViewsData`, `MenuEntry` | Viewers list, reaction counts, forward count, views count, interactive menu with userpic loading | `crates/gpui_telegram_media/src/media/stories/media_stories_recent_views.rs` (`MediaStoriesRecentViewsStories`, `rust_domain_or_server_1to1`) |
| 14 | `Telegram/SourceFiles/media/stories/media_stories_reply.cpp` | `L1-L1039` | `ReplyArea`, `PlaceholderText` | Stealth mode countdown awareness, video stream comments, star-paid comment placeholders (`starsPerMessage`), reaction dispatch | `crates/gpui_telegram_media/src/media/stories/media_stories_reply.rs` (`MediaStoriesReplyStories`, `rust_domain_or_server_1to1`) |
| 15 | `Telegram/SourceFiles/media/stories/media_stories_reply.h` | `L1-L189` | `ReplyArea`, `ReplyAreaData`, `ReplyAreaType` | Type variants (Reply, Comment, VideoStreamComment), file sending confirmation, voice message sending | `crates/gpui_telegram_media/src/media/stories/media_stories_reply.rs` (`MediaStoriesReplyStories`, `rust_domain_or_server_1to1`) |
| 16 | `Telegram/SourceFiles/media/stories/media_stories_repost_view.cpp` | `L1-L299` | `RepostView`, `height`, `draw`, `lookupHandler` | `height = padding.top + semiboldFont.height + normalFont.height + padding.bottom`, simple mode (omits text line, `y += normalFont.height`), quote background fill with `QColor(0, 0, 0, 64)` | `crates/gpui_telegram_media/src/media/stories/media_stories_repost_view.rs` (`MediaStoriesRepostViewStories`, `rust_domain_or_server_1to1`) |
| 17 | `Telegram/SourceFiles/media/stories/media_stories_repost_view.h` | `L1-L72` | `RepostView`, `ClickHandlerHost` | Repost quote preview from peer or channel post, ripple effect and link click handler | `crates/gpui_telegram_media/src/media/stories/media_stories_repost_view.rs` (`MediaStoriesRepostViewStories`, `rust_domain_or_server_1to1`) |
| 18 | `Telegram/SourceFiles/media/stories/media_stories_share.cpp` | `L1-L320` | `PrepareShareBox`, `FormatShareAtTime`, `PrepareShareAtTimeBox` | `FormatShareAtTime` (seconds -> `h:mm:ss` or `m:ss`), `canCopyLink = story->hasDirectLink()`, comment counting (`1 : 2`) | `crates/gpui_telegram_media/src/media/stories/media_stories_share.rs` (`MediaStoriesShareStories`, `rust_domain_or_server_1to1`) |
| 19 | `Telegram/SourceFiles/media/stories/media_stories_share.h` | `L1-L35` | `PrepareShareBox`, `FormatShareAtTime`, `PrepareShareAtTimeBox` | Share dialog builder with direct story link copy callback and video timestamp forwarding | `crates/gpui_telegram_media/src/media/stories/media_stories_share.rs` (`MediaStoriesShareStories`, `rust_domain_or_server_1to1`) |
| 20 | `Telegram/SourceFiles/media/stories/media_stories_sibling.cpp` | `L1-L437` | `Sibling`, `LoaderPhoto`, `LoaderVideo`, `nameImage`, `namePosition`, `view` | `kGoodFadeDuration = 200ms`, `kSiblingFade = 0.5`, `kSiblingFadeOver = 0.4`, `kSiblingNameOpacity = 0.8`, `kSiblingNameOpacityOver = 1.0`, `kSiblingScaleOver = 0.05` (1.05x hover scale) | `crates/gpui_telegram_media/src/media/stories/media_stories_sibling.rs` (`MediaStoriesSiblingStories`, `rust_domain_or_server_1to1`) |
| 21 | `Telegram/SourceFiles/media/stories/media_stories_sibling.h` | `L1-L82` | `Sibling`, `SiblingView`, `SiblingLayout` | Neighboring story thumbnail loader, blurred vs good image crossfade, elided name generation | `crates/gpui_telegram_media/src/media/stories/media_stories_sibling.rs` (`MediaStoriesSiblingStories`, `rust_domain_or_server_1to1`) |
| 22 | `Telegram/SourceFiles/media/stories/media_stories_slider.cpp` | `L1-L157` | `Slider`, `layout`, `paint`, `resetProgress` | `kOpacityInactive = 0.4`, `kOpacityActive = 1.0`, `single = st::storiesSliderWidth`, `skip = st::storiesSliderSkip`, `max = (width + skip) / (single + skip)`, `one = (width - (count - 1) * skip) / count`, radius `single / 2.0` | `crates/gpui_telegram_media/src/media/stories/media_stories_slider.rs` (`MediaStoriesSliderStories`, `rust_domain_or_server_1to1`) |
| 23 | `Telegram/SourceFiles/media/stories/media_stories_slider.h` | `L1-L64` | `Slider`, `SliderData` | Multi-segment progress bar with active segment progress fill and video stream skip | `crates/gpui_telegram_media/src/media/stories/media_stories_slider.rs` (`MediaStoriesSliderStories`, `rust_domain_or_server_1to1`) |
| 24 | `Telegram/SourceFiles/media/stories/media_stories_stealth.cpp` | `L1-L429` | `SetupStealthMode`, `AddStealthModeMenu`, `TimeLeftText`, `StealthModeBox` | `kAlreadyToastDuration = 4000ms`, `kCooldownButtonLabelOpacity = 0.5`, `TimeLeftText` formatting (`h:mm:ss` / `m:ss` / `0:ss`), active vs cooldown timer logic | `crates/gpui_telegram_media/src/media/stories/media_stories_stealth.rs` (`MediaStoriesStealthStories`, `rust_domain_or_server_1to1`) |
| 25 | `Telegram/SourceFiles/media/stories/media_stories_stealth.h` | `L1-L46` | `SetupStealthMode`, `AddStealthModeMenu`, `StealthModeDescriptor`, `TimeLeftText` | Stealth mode privacy dialog, profile/stories context menu injector | `crates/gpui_telegram_media/src/media/stories/media_stories_stealth.rs` (`MediaStoriesStealthStories`, `rust_domain_or_server_1to1`) |
| 26 | `Telegram/SourceFiles/media/stories/media_stories_view.cpp` | `L1-L195` | `View`, `show`, `ready`, `togglePaused`, `subjumpFor`, `jumpFor` | Top-level delegate adapter forwarding story display, playback timing, and full caption expander | `crates/gpui_telegram_media/src/media/stories/media_stories_view.rs` (`MediaStoriesViewStories`, `rust_domain_or_server_1to1`) |
| 27 | `Telegram/SourceFiles/media/stories/media_stories_view.h` | `L1-L149` | `View`, `ContentLayout`, `SiblingView`, `RepostClickHandler` | `kCollapsedCaptionLines = 2`, `kMaxShownCaptionLines = 4`, public API bridge for stories player | `crates/gpui_telegram_media/src/media/stories/media_stories_view.rs` (`MediaStoriesViewStories`, `rust_domain_or_server_1to1`) |


---

## 2. Core Subsystems & Algorithmic Specifications

### 2.1 Segmented Progress Bar Arithmetic (`media_stories_slider.cpp:L90-L154`)

The story progress bar consists of `total` rounded segments separated by a fixed horizontal gap `st::storiesSliderSkip` (3px):
1. **Segment Count Clamping**:
   $$\text{max} = \left\lfloor \frac{\text{width} + \text{skip}}{\text{single} + \text{skip}} \right\rfloor$$
   $$\text{count} = \text{clamp}(\text{total}, 1, \text{max})$$
2. **Segment Width Computation**:
   $$\text{one} = \frac{\text{width} - (\text{count} - 1) \cdot \text{skip}}{\text{count}}$$
3. **Rendering & Opacity State**:
   - Prior segments ($i < \text{index}$): Opacity $1.0$ (completed playback).
   - Subsequent segments ($i > \text{index}$): Opacity $0.4$ (`kOpacityInactive`, unplayed).
   - Active segment ($i == \text{index}$):
     - Background track: Opacity $0.4$, width `full - activeWidth + min`.
     - Active filled track: Opacity $1.0$, width $\max(\text{full} \cdot \text{progress}, \text{min})$.
     - Corner radius: $r = \frac{\text{single}}{2.0}$.

### 2.2 Stories Playback State Machine & Hold-to-Pause Gestures (`media_stories_controller.cpp:L185-L235, L1470-L1486`)

1. **Photo Story Timing**:
   - `kPhotoDuration = 5000ms` (5.0s default display duration).
   - `kPhotoProgressInterval = 100ms` (10Hz tick rate for progress interpolation).
   - Mark as read threshold: `kMarkAsReadAfterSeconds = 0.2s` (200ms).
2. **Hold-to-Pause FSM**:
   - On `MouseButtonPress` on content stage: `contentPressed(true)` -> `togglePaused(true)` freezes the progress timer and records pause timestamp.
   - On `MouseButtonRelease`: `contentPressed(false)` -> `togglePaused(false)` resumes timer with offset adjustment:
     $$\text{started} \gets \text{started} + (\text{now} - \text{paused})$$
   - On full caption expand or context menu open: automatically forces `togglePaused(true)`.
3. **Subjump & Jump Navigation**:
   - Left tap / ArrowLeft: `subjumpFor(-1)` moves to previous segment or wraps to left sibling story.
   - Right tap / ArrowRight / Space: `subjumpFor(1)` moves to next segment or jumps to right sibling story.

### 2.3 Stealth Mode Countdown & Format (`media_stories_stealth.cpp:L39-L426`)

1. **Active vs Cooldown States**:
   - If $\text{enabledTill} > \text{now}$: Mode active, toast shows remaining active time.
   - If $\text{cooldownTill} > \text{now}$: In cooldown period, activation button disabled with opacity `kCooldownButtonLabelOpacity = 0.5`.
   - Toast duration: `kAlreadyToastDuration = 4000ms`.
2. **`TimeLeftText(left)` Formatting Algorithm**:
   - If $\text{hours} > 0$: Format as `"h:mm:ss"`.
   - Else if $\text{minutes} > 0$: Format as `"m:ss"`.
   - Else: Format as `"0:ss"`.

### 2.4 Neighboring Sibling Previews & Hover Interpolation (`media_stories_sibling.cpp:L28-L48, L316-L333`)

Neighboring stories appear on the left and right with darkened backdrop and scale perspective:
- Scale interpolation: $\text{scale} = 1.0 + (\text{over} \cdot 0.05)$ (zooms up to 1.05x on hover).
- Backdrop fade: $\text{fade} = 0.5 \cdot (1 - \text{over}) + 0.4 \cdot \text{over}$.
- Name label opacity: $\text{opacity} = 0.8 \cdot (1 - \text{over}) + 1.0 \cdot \text{over}$.
- Good thumbnail fade transition duration: `kGoodFadeDuration = 200ms`.

### 2.5 Recent Views Classification & Permissions (`media_stories_recent_views.cpp:L34-L160`)

1. **Viewer Type Classification**:
   - Video streams $\to$ `RecentViewsType::Other`.
   - Self stories $\to$ `RecentViewsType::Self`.
   - Channel broadcasts $\to$ `RecentViewsType::Channel`.
   - Service users $\to$ `RecentViewsType::Changelog`.
   - Other users $\to$ `RecentViewsType::Other`.
2. **Reactions Visibility**:
   - Channels: Allowed only if user is creator or administrator with admin rights (`CanViewReactionsFor`).
   - Page batching: `kAddPerPage = 50`, `kLoadViewsPages = 2`.

### 2.6 Interactive Story Reaction Stickers (`media_stories_reactions.cpp:L52-L65`)

Reaction sticker animations and geometric transforms:
- `kSuggestedBubbleSize = 1.0`
- `kSuggestedTailBigSize = 0.264`, `kSuggestedTailBigOffset = 0.464`, rotation $-42.29^\circ$.
- `kSuggestedTailSmallSize = 0.110`, `kSuggestedTailSmallOffset = 0.697`, rotation $-40.87^\circ$.
- `kReactionScaleOutTarget = 0.7`, `kReactionScaleOutDuration = 1000ms`.
- `kMessageReactionScaleOutDuration = 400ms`, `kStoppingFadeDuration = 150ms`.

### 2.7 Share Link & Time Formatting (`media_stories_share.cpp:L237-L246`)

- `FormatShareAtTime(seconds)`: Formats video timestamps as `h:mm:ss` (if hours > 0) or `m:ss` for story direct deep links (`?t=...`).



---

<!-- Slice File: module_09_media_root_theme.md -->

# Module 09 Specification: Media Root Infrastructure & Theme Subsystems

## Overview

Module 09 Slice 5 provides foundational multimedia infrastructure, FFmpeg byte-level IO abstractions, frame generators, video encoding pipelines, audio waveform synthesis, system media controls bridge, and the 3 core media theme stylesheets for GPUI Telegram Desktop.

This specification adheres strictly to:
- **Mandate #15 (Zero Tolerance for Boilerplate)**: All generic stubs (Patterns A, B, C, D) are completely replaced with genuine domain algorithms and state machines.
- **Mandate #17 (Dynamic Target File Architecture)**: Upstream C++ files serve as the reading checklist, consolidated into idiomatic Rust modules in `crates/gpui_telegram_media/src/media/` and theme sheets in `crates/gpui_telegram_desktop/src/theme/media/`.
- **Mandate #18 (Anti-Bluffing Protocol)**: Every component cites exact upstream C++ line ranges (`.cpp:L<start>-L<end>`), constants, and mathematical algorithms.
- **Mandate #19 (Zero-Omission Reading Guarantee)**: 100% of the 20 assigned upstream C++ and style files are enumerated with exact line ranges and extracted logic.
- **Mandate #20 (Stream Hygiene)**: Authored in modular chunks under 200 lines per call.

---

## 1. 100% Per-File Reading Manifest (All 20 Assigned Sources)

| # | Upstream Source Path | Line Range | Upstream Symbols & Classes | Extracted Constants & Logic | Dynamic Rust Target Module & Symbol |
|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/ffmpeg/ffmpeg_bytes_io_wrap.h` | `L1-L131` | `ReadFileWrap`, `ReadBytesWrap`, `WriteBytesWrap` | `kReserve = 1024 * 1024` (L93), `AVSEEK_SIZE`, `AVERROR_EOF` | `crates/gpui_telegram_media/src/media/ffmpeg_bytes_io_wrap.rs` (`FfmpegBytesIoWrap`) |
| 2 | `Telegram/SourceFiles/ffmpeg/ffmpeg_frame_generator.cpp` | `L1-L429` | `FrameGenerator::Impl`, `ReadFrame` | `kMaxArea = 1920 * 1080 * 4` (L17), aspect ratio scaling, alpha premultiply, rotation | `crates/gpui_telegram_media/src/media/ffmpeg_frame_generator.rs` (`FfmpegFrameGenerator`) |
| 3 | `Telegram/SourceFiles/ffmpeg/ffmpeg_frame_generator.h` | `L1-L42` | `FrameGenerator`, `Frame` | `renderNext`, `renderCurrent`, `jumpToStart`, `rate`, `count` | `crates/gpui_telegram_media/src/media/ffmpeg_frame_generator.rs` (`FfmpegFrameGenerator`) |
| 4 | `Telegram/SourceFiles/ffmpeg/ffmpeg_utility.cpp` | `L1-L884` | `AvErrorWrap`, `Packet`, `MakeIOPointer`, `MakeFormatPointer` | `kAlignImageBy = 64` (L48), `kMaxScaleByAspectRatio = 16` (L50), `kAvioBlockSize = 4096` (L51), `PtsToTime`, `TimeToPts` | `crates/gpui_telegram_media/src/media/ffmpeg_utility.rs` (`FfmpegUtility`) |
| 5 | `Telegram/SourceFiles/ffmpeg/ffmpeg_utility.h` | `L1-L262` | `Packet`, `CodecDescriptor`, `FormatSettings`, `SwscalePointer` | `kPixelBytesSize = 4` (L35), `kAVBlockSize = 4096` (L36), `kUniversalTimeBase` | `crates/gpui_telegram_media/src/media/ffmpeg_utility.rs` (`FfmpegUtility`) |
| 6 | `Telegram/SourceFiles/media/media_audio_waveform.cpp` | `L1-L259` | `Decoder`, `LoadWaveform`, `SamplesCallback` | `kMinimalPeak = 2500` (L18), `kMaxPeak = 65535` (L19), `kPeakAverageRatio = 1.8` (L20), `kFineSpansPerSecond = 50` (L21), `kMaxFineSpans = 100000` (L22) | `crates/gpui_telegram_media/src/media/media_audio_waveform.rs` (`MediaAudioWaveform`) |
| 7 | `Telegram/SourceFiles/media/media_audio_waveform.h` | `L1-L28` | `Waveform` | `bars: std::vector<uint16>`, `peak: uint16`, `empty()` | `crates/gpui_telegram_media/src/media/media_audio_waveform.rs` (`MediaAudioWaveform`) |
| 8 | `Telegram/SourceFiles/media/media_common.h` | `L1-L61` | `RepeatMode`, `OrderMode`, `VideoQuality` | `kSpeedMin = 0.5` (L42), `kSpeedMax = 2.5` (L43), `kSpedUpDefault = 1.7` (L44), `ValidFrameSize`, `EqualSpeeds` | `crates/gpui_telegram_media/src/media/media_common.rs` (`MediaCommon`) |
| 9 | `Telegram/SourceFiles/media/media_video_canvas.cpp` | `L1-L71` | `DominantCanvasBackground`, `PaintCanvasBackground` | `kSampleSide = 240` (L13), `kSampleTop = 0.1` (L14), `kSampleBottom = 0.9` (L15), HSV saturation and value clamping | `crates/gpui_telegram_media/src/media/media_video_canvas.rs` (`MediaVideoCanvas`) |
| 10 | `Telegram/SourceFiles/media/media_video_canvas.h` | `L1-L29` | `CanvasBackground` | `top: QColor`, `bottom: QColor`, `valid()` | `crates/gpui_telegram_media/src/media/media_video_canvas.rs` (`MediaVideoCanvas`) |
| 11 | `Telegram/SourceFiles/media/media_video_encode.cpp` | `L1-L2767` | `TranscodeVideo`, `Run`, `RunWebmSticker`, `EstimateTranscodedSize` | `kMinBitrate = 600000` (L31), `kMaxBitrate = 6800000` (L32), `kWebmStickerCrf = 28` (L33), `kWebmStickerCrfLadder` (L34-41), `kMaxSourceSize = 1GB` (L42), `kAudioFrequency = 48000` (L44), `kRateControlFps = 60` (L52) | `crates/gpui_telegram_media/src/media/media_video_encode.rs` (`MediaVideoEncode`) |
| 12 | `Telegram/SourceFiles/media/media_video_encode.h` | `L1-L190` | `VideoSource`, `StillSource`, `Job`, `Result`, `TranscodeResult`, `MusicTrack` | `DownscaledSize`, `TranscodedSize`, `TranscodedDuration`, `ComposeCutout` | `crates/gpui_telegram_media/src/media/media_video_encode.rs` (`MediaVideoEncode`) |
| 13 | `Telegram/SourceFiles/media/media_video_frames.cpp` | `L1-L401` | `ReadFileInfo`, `ExtractFrames`, `ExtractFrame`, `Source`, `Extractor` | `kMaxDecodeSpan = 10000` (L18), `kForwardDecodeWindow = 3000` (L19), `kMaxDecodeArea = 16M` (L20), cover aspect cropping | `crates/gpui_telegram_media/src/media/media_video_frames.rs` (`MediaVideoFrames`) |
| 14 | `Telegram/SourceFiles/media/media_video_frames.h` | `L1-L46` | `FileInfo`, `ExtractRequest`, `ExtractCallback` | `dimensions: QSize`, `duration: crl::time`, `box: QSize`, `cover: bool` | `crates/gpui_telegram_media/src/media/media_video_frames.rs` (`MediaVideoFrames`) |
| 15 | `Telegram/SourceFiles/media/system_media_controls_manager.cpp` | `L1-L636` | `SystemMediaControlsManager`, `RepeatModeToLoopStatus`, `ActivePlayerType` | `kSkipMs = 15000` (L35), playback rate synchronization, metadata sync, track switching | `crates/gpui_telegram_media/src/media/system_media_controls_manager.rs` (`SystemMediaControlsManager`) |
| 16 | `Telegram/SourceFiles/media/system_media_controls_manager.h` | `L1-L70` | `SystemMediaControlsManager`, `SystemMediaControlsVideoSink` | `videoStart`, `videoUpdate`, `videoSetThumbnail`, `videoFinish`, `Supported()` | `crates/gpui_telegram_media/src/media/system_media_controls_manager.rs` (`SystemMediaControlsManager`) |
| 17 | `Telegram/SourceFiles/media/system_media_controls_video.h` | `L1-L51` | `SystemMediaControlsVideoDelegate`, `SystemMediaControlsVideoSink` | `VideoState` (`title`, `artist`, `position`, `duration`, `playing`), delegate methods `smtcPlay`, `smtcPause`, `smtcSeek` | `crates/gpui_telegram_media/src/media/system_media_controls_video.rs` (`SystemMediaControlsVideo`) |
| 18 | `Telegram/SourceFiles/media/player/media_player.style` | `L1-L352` | `mediaPlayerWideWidth`, `mediaPlayerHeight`, `mediaPlayerPlayIcon`, `mediaPlayerSpeedMenu` | `mediaPlayerWideWidth = 460px` (L83), `mediaPlayerHeight = 35px` (L84), `mediaPlayerPlayIcon` 24x24 (L142), `playRadius = 1.5` (L147), `duration = 200ms` (L157) | `crates/gpui_telegram_desktop/src/theme/media/media_player_style.rs` (`MediaPlayerPlayerStyleSheet`) |
| 19 | `Telegram/SourceFiles/media/stories/media_stories.style` | `L1-L81` | `StealthBoxStyle`, `storiesStealthStyleDefault` | `buttonHeight = 42px` (L29), `margin = 56px` top (L35), `boxClose` 40px ripple (L56), stealth icons | `crates/gpui_telegram_desktop/src/theme/media/media_stories_style.rs` (`MediaStoriesStoriesStyleSheet`) |
| 20 | `Telegram/SourceFiles/media/view/media_view.style` | `L1-L1295` | `mediaviewControllerSize`, `mediaviewPlayback`, `mediaviewPlayButton` | `mediaviewOverDuration = 150ms` (L18), `mediaviewControllerSize = 480x72` (L44), `mediaviewPlayButton` 40x40 (L59), `mediaviewVolumeWidth = 75px` (L93), `mediaviewControllerRadius = 9px` (L94) | `crates/gpui_telegram_desktop/src/theme/media/media_view_style.rs` (`MediaViewViewStyleSheet`) |

---

## 2. Subsystem Architecture & Implementation Details

### 2.1 FFmpeg Byte IO & Utility Abstractions (`ffmpeg/*`)
- **Byte Stream Handling (`ffmpeg_bytes_io_wrap.h:L16-L128`)**:
  - `ReadFileWrap`: Adapts local filesystem paths into seekable AVIO callbacks.
  - `ReadBytesWrap`: In-memory continuous slice reader maintaining `offset` and returning `AVERROR_EOF` when exhausted.
  - `WriteBytesWrap`: Dynamic append/overwrite memory buffer using `kReserve = 1024 * 1024` (1MB chunk allocation) for encoding outputs.
- **Utility & Color Conversion (`ffmpeg_utility.cpp:L48-L884`)**:
  - Memory alignment: `kAlignImageBy = 64`, `kPixelBytesSize = 4`.
  - Timestamp math: `PtsToTime(pts, timeBase)` and `TimeToPts(time, timeBase)` with millisecond conversion `(pts * 1000 * num) / den`.
  - Frame buffer creation: `CreateFrameStorage` allocating aligned RGBA/BGRA byte buffers.
  - Alpha operations: `PremultiplyInplace` and `UnPremultiply`.

### 2.2 Audio Waveform Synthesizer (`media/media_audio_waveform.*`)
- **Algorithm (`media_audio_waveform.cpp:L18-L256`)**:
  - Ingests audio packets and resamples to single-channel 16-bit PCM at input sample rate.
  - Slices audio into fine spans (`kFineSpansPerSecond = 50`).
  - Dynamic span compaction: When `fine.size()` hits `kMaxFineSpans = 100000`, pairs are halved (`max(2*i, 2*i+1)`) and `spanSize` doubles.
  - Maps compacted spans into exactly `count` bars (typically 100 bars).
  - Peak thresholding: `peak = clamp(max(kMinimalPeak = 2500, sum * 1.8 / count), 0, kMaxPeak = 65535)`.
  - Final bars clamped to computed `peak`.

### 2.3 Video Processing, Canvas & Frames (`media/media_video_*`)
- **Canvas (`media_video_canvas.cpp:L13-L68`)**:
  - Extracts dominant top and bottom colors from scaled 240x240 image sample (`kSampleTop = 0.1`, `kSampleBottom = 0.9`).
  - `AdaptColor`: Clamps HSV value between 0.15 and 0.85; boosts low saturation (`<= 0.5` by `+0.2`) and tempers high saturation (`> 0.8` by `-0.4`).
- **Frames Extraction (`media_video_frames.cpp:L18-L188`)**:
  - `kMaxDecodeArea = 16M` pixels, `kForwardDecodeWindow = 3000ms`.
  - Extracts keyframes at specified millisecond timestamps with aspect ratio preserving downscale or centered cover crop.
- **Encoding Pipeline (`media_video_encode.cpp:L30-L2767`)**:
  - Output bitrate: `clamp(pixels * fps * 0.07, 600kbps, 6.8Mbps)`.
  - WebM sticker CRF ladder: `[None, 34, 40, 47, 55, 63]` iterating until file fits sticker byte limit.
  - Transcode duration: `TranscodedDuration(from, till, duration) = max(min(till, duration) - from, 0)`.
  - Cutout composition: Composites animated frame through cutout picture hole and alpha mask.

### 2.4 System Media Controls Integration (`media/system_media_controls_*`)
- **Bridge Architecture (`system_media_controls_manager.cpp:L34-L635`)**:
  - Native OS media integration (macOS MPNowPlayingInfoCenter, Windows SMTC, Linux MPRIS).
  - Loop status mapping: `RepeatMode::None -> None`, `One -> Track`, `All -> Playlist`.
  - Skip step: `kSkipMs = 15000` (15s forward/backward).
  - Video and audio delegates dispatching `smtcPlay`, `smtcPause`, `smtcStop`, `smtcNext`, `smtcPrevious`, `smtcSeek`.

### 2.5 Media Style Sheets (`*.style`)
- **`media_player.style:L83-L200`**: Panel height `35px`, wide width `460px`, play icon 24x24 with `playRadius: 1.5`, `pauseRadius: 1.0`, animation duration `200ms`, speed slider with 12px semibold font.
- **`media_stories.style:L26-L80`**: Stealth box button height `42px`, top margin `56px`, close button with `40px` ripple, `windowBgActive` logo background.
- **`media_view.style:L18-L120`**: Overlay animation duration `150ms`, controller dimensions `480x72px` with `9px` corner radius, 40x40 play button, 32x32 PiP button, 75px volume slider.



---

<!-- Slice File: module_09_editor_scene_video.md -->

# Module 09 Specification: Editor Scene Graph, Controllers & Video Timeline (`editor/scene`, `editor/controllers`, `editor/video`)

## Overview

Module 09 Slice 6 delivers the complete interactive graphics scene graph, affine transformations, sticky snapping guidelines, vector drawing canvas, text editing controller, stickers/emoji integration, undo/redo stack, and video timeline editing subsystem for Telegram Desktop reconstructed in pure, idiomatic Rust.

This specification is authored in strict compliance with:
- **Mandate #15 (Zero Tolerance for Boilerplate)**: All generic boilerplate stubs (`{ id, label, value }` Patterns A, B, C, D) are completely eliminated across `crates/gpui_telegram_media/src/editor/scene/`, `crates/gpui_telegram_media/src/editor/controllers/`, and `crates/gpui_telegram_media/src/editor/video/`.
- **Mandate #17 (Dynamic Target File Architecture)**: Upstream C++ files serve as the reading checklist. The target architecture provides fine-grained, authentic Rust modules and state machines with clean mathematical abstractions.
- **Mandate #18 (Anti-Bluffing Protocol)**: Every component, transform algorithm, and timeline state machine cites exact upstream C++ implementation file line ranges (`.cpp:L<start>-L<end>`) and header definitions (`.h:L<start>-L<end>`), extracting anonymous-namespace `k...` constants, geometric equations, and edge-case behaviors.
- **Mandate #19 (Zero-Omitted-File Reading Guarantee)**: 100% of the 51 assigned upstream C++ source and header files are explicitly present, documented, and accounted for in the Per-File Reading Manifest below (`omitted_from_design_doc == 0`).
- **Mandate #20 (Stream Hygiene)**: Written in modular chunks under ~150-200 lines to prevent timeout.

---

## 1. 100% Per-File Reading Manifest (All 51 Assigned Upstream Sources)

| # | Upstream C++ Source Path | Exact Line Range Read | Upstream Symbols & Classes | Extracted Constants & Logic | Dynamic Rust Target Module & Disposition |
|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/editor/scene/scene.cpp` | `L1-L1262` | `Editor::Scene`, `ItemAction`, `ItemEraser`, `StickyGuidesItem`, `CapturedPlacement`, `PendingShape` | Scene graph lifecycle, undo/redo stack (`_items`, `_itemNumber`), sticky guidelines rendering with animation (`kStickyDuration = 150ms`), shape placement, text edit dispatch, video clip selection synchronization | `crates/gpui_telegram_media/src/editor/scene/scene.rs` (`SceneScene`, `gpui_ui_1to1`) |
| 2 | `Telegram/SourceFiles/editor/scene/scene.h` | `L1-L217` | `Editor::Scene`, `PendingShape`, `CapturedPlacement` | Scene class interface: `setCanvasRect`, `applyBrush`, `setTextDefaults`, `addItem`, `removeItem`, `canEqualizeDurations`, `equalizeDurations`, `hasUndo`, `hasRedo`, `performUndo`, `performRedo` | `crates/gpui_telegram_media/src/editor/scene/scene.rs` (`SceneScene`, `gpui_ui_1to1`) |
| 3 | `Telegram/SourceFiles/editor/scene/scene_emoji_document.cpp` | `L1-L263` | `EmojiDocument`, `ReplaceEmoji`, `ReplaceEmojiInRange`, `SanitizeRange`, `RecoverTextFromDocument`, `RecoverWrappedTextFromDocument` | Custom emoji loading via resource loader `emoji://` URI scheme; device pixel ratio scaling; text document entity serialization and deserialization with line-wrapping recovery | `crates/gpui_telegram_media/src/editor/scene/scene_emoji_document.rs` (`SceneEmojiDocumentScene`, `gpui_ui_1to1`) |
| 4 | `Telegram/SourceFiles/editor/scene/scene_emoji_document.h` | `L1-L29` | `EmojiDocument`, `ReplaceEmoji`, `SanitizeRange` | Rich text emoji document definition and emoji parsing utility function prototypes | `crates/gpui_telegram_media/src/editor/scene/scene_emoji_document.rs` (`SceneEmojiDocumentScene`, `gpui_ui_1to1`) |
| 5 | `Telegram/SourceFiles/editor/scene/scene_item_animated.cpp` | `L1-L108` | `ItemAnimated`, `entityKind`, `entityRect`, `visibleRect`, `paintFrame` | Base class for animated media scene elements (stickers, video, animated messages); keying cache, loop duration tracking, frame playback and mirror flipping | `crates/gpui_telegram_media/src/editor/scene/scene_item_animated.rs` (`SceneItemAnimatedScene`, `gpui_ui_1to1`) |
| 6 | `Telegram/SourceFiles/editor/scene/scene_item_animated.h` | `L1-L53` | `ItemAnimated` | Abstract interface for animated scene items: `animated()`, `hasContent()`, `content()`, `loopDuration()`, `trim()`, `releasePlayers()`, `animatedEntity()` | `crates/gpui_telegram_media/src/editor/scene/scene_item_animated.rs` (`SceneItemAnimatedScene`, `gpui_ui_1to1`) |
| 7 | `Telegram/SourceFiles/editor/scene/scene_item_base.cpp` | `L1-L872` | `NumberedItem`, `ItemBase`, `kSnapAngle = 45.`, `kMinSizeRatio = 0.05`, `kStickyDuration = 150ms`, `kStickyLines = 5` | Affine transformation interaction: left/right handles, rotation angle calculation `atan2(dy, dx)*180/PI`, 45-degree angle snapping (`kSnapAngle`), size clamping, 5-line sticky snapping (`kStickyLines`), cubic ease-out animation | `crates/gpui_telegram_media/src/editor/scene/scene_item_base.rs` (`SceneItemBaseScene`, `gpui_ui_1to1`) |
| 8 | `Telegram/SourceFiles/editor/scene/scene_item_base.h` | `L1-L267` | `NumberedItem`, `ItemBase`, `Placement`, `Sticky`, `StickyAxis`, `HandleType` | Comprehensive base item definitions: placement state `{ position, rotation, scale, zValue, size, aspectRatio, bend, fontSize, flipped }`, handle hit-testing, context menu, duplication | `crates/gpui_telegram_media/src/editor/scene/scene_item_base.rs` (`SceneItemBaseScene`, `gpui_ui_1to1`) |
| 9 | `Telegram/SourceFiles/editor/scene/scene_item_canvas.cpp` | `L1-L518` | `ItemCanvas`, `kMinPointDistanceBase = 2.0`, `kMaxPointDistance = 15.0`, `kSmoothingStrength = 0.5`, `kSegmentOverlap = 3` | Freehand drawing canvas stroke smoothing algorithm: distance threshold, quadratic Bezier curve interpolation, blur tool offscreen clipping, eraser blending | `crates/gpui_telegram_media/src/editor/scene/scene_item_canvas.rs` (`SceneItemCanvasScene`, `gpui_ui_1to1`) |
| 10 | `Telegram/SourceFiles/editor/scene/scene_item_canvas.h` | `L1-L110` | `ItemCanvas`, `Content`, `StrokePoint` | Freehand drawing canvas graphics item interface: `applyBrush`, `setCanvasRect`, `drawableRect`, mouse stroke capturing stream | `crates/gpui_telegram_media/src/editor/scene/scene_item_canvas.rs` (`SceneItemCanvasScene`, `gpui_ui_1to1`) |
| 11 | `Telegram/SourceFiles/editor/scene/scene_item_image.cpp` | `L1-L48` | `ItemImage`, `paint`, `performFlip`, `duplicate` | Static raster image layer item: aspect ratio preservation, horizontal reflection, duplicate offset by `size / 3` | `crates/gpui_telegram_media/src/editor/scene/scene_item_image.rs` (`SceneItemImageScene`, `gpui_ui_1to1`) |
| 12 | `Telegram/SourceFiles/editor/scene/scene_item_image.h` | `L1-L32` | `ItemImage` | Header definition for static image scene items | `crates/gpui_telegram_media/src/editor/scene/scene_item_image.rs` (`SceneItemImageScene`, `gpui_ui_1to1`) |
| 13 | `Telegram/SourceFiles/editor/scene/scene_item_line.cpp` | `L1-L93` | `ItemLine`, `applyEraser`, `setPixmap` | Persistent painted vector brush stroke line item; eraser application via mask subtraction; undo state preservation | `crates/gpui_telegram_media/src/editor/scene/scene_item_line.rs` (`SceneItemLineScene`, `gpui_ui_1to1`) |
| 14 | `Telegram/SourceFiles/editor/scene/scene_item_line.h` | `L1-L47` | `ItemLine` | Interface definition for completed brush stroke lines | `crates/gpui_telegram_media/src/editor/scene/scene_item_line.rs` (`SceneItemLineScene`, `gpui_ui_1to1`) |
| 15 | `Telegram/SourceFiles/editor/scene/scene_item_link.cpp` | `L1-L181` | `ItemLink`, `kMaxWidthRatio = 0.9`, `NextStyle`, `PreviousStyle` | Story link preview pill widget: 3 cycling styles (`Framed`, `SemiTransparent`, `Opaque`), density computation from image width, URL editing callback | `crates/gpui_telegram_media/src/editor/scene/scene_item_link.rs` (`SceneItemLinkScene`, `gpui_ui_1to1`) |
| 16 | `Telegram/SourceFiles/editor/scene/scene_item_link.h` | `L1-L59` | `ItemLink` | Header declaration for story link pills | `crates/gpui_telegram_media/src/editor/scene/scene_item_link.rs` (`SceneItemLinkScene`, `gpui_ui_1to1`) |
| 17 | `Telegram/SourceFiles/editor/scene/scene_item_message.cpp` | `L1-L499` | `ItemMessage`, `kMaxRatio = 8`, `kPlaybackFrameSide = 1024`, `CoverRect` | Forwarded message quote sticker item: video playback integration, dynamic ratio scaling, cover rect calculation, theme color styling | `crates/gpui_telegram_media/src/editor/scene/scene_item_message.rs` (`SceneItemMessageScene`, `gpui_ui_1to1`) |
| 18 | `Telegram/SourceFiles/editor/scene/scene_item_message.h` | `L1-L121` | `ItemMessage`, `MessageVideoOptions` | Forwarded quote message item interface: video playback options, message renderer binding | `crates/gpui_telegram_media/src/editor/scene/scene_item_message.rs` (`SceneItemMessageScene`, `gpui_ui_1to1`) |
| 19 | `Telegram/SourceFiles/editor/scene/scene_item_shape.cpp` | `L1-L463` | `ItemShape`, `kStarInnerRatio = 0.4316`, `kRoundingRatio = 0.234`, `kBubbleBodyRatio = 2./2.25`, `kBubbleTailRatio = 0.75`, `kArrowHeadRatio = 0.16`, `kArrowHeadStrokeFactor = 2.5`, `kArrowHeadAngle = 30 deg`, `kArrowMaxBendRatio = 0.35`, `kStarBoxWidth = 1.902113`, `kStarBoxHeight = 1.809017` | Vector geometric shapes: Rectangle, Circle, Star (golden ratio 5-point math), Speech Bubble with tail, Arrow with quadratic curved shaft and triangular arrowhead; live bend control point dragging | `crates/gpui_telegram_media/src/editor/scene/scene_item_shape.rs` (`SceneItemShapeScene`, `gpui_ui_1to1`) |
| 20 | `Telegram/SourceFiles/editor/scene/scene_item_shape.h` | `L1-L93` | `ItemShape`, `ArrowPoints` | Vector shape graphics item interface: color, stroke width, fill, live draft frame sizing, and bend ratio | `crates/gpui_telegram_media/src/editor/scene/scene_item_shape.rs` (`SceneItemShapeScene`, `gpui_ui_1to1`) |
| 21 | `Telegram/SourceFiles/editor/scene/scene_item_sticker.cpp` | `L1-L236` | `ItemSticker`, `updateThumbnail`, `createPlayer` | Static and animated sticker items: Lottie and WebM player initialization, high-resolution thumbnail fallback, aspect ratio preservation, flipping | `crates/gpui_telegram_media/src/editor/scene/scene_item_sticker.rs` (`SceneItemStickerScene`, `gpui_ui_1to1`) |
| 22 | `Telegram/SourceFiles/editor/scene/scene_item_sticker.h` | `L1-L73` | `ItemSticker` | Header interface for sticker graphics scene elements | `crates/gpui_telegram_media/src/editor/scene/scene_item_sticker.rs` (`SceneItemStickerScene`, `gpui_ui_1to1`) |
| 23 | `Telegram/SourceFiles/editor/scene/scene_item_text.cpp` | `L1-L1015` | `ItemText`, `kPaddingFactor = 0.4`, `kMaxWidthFactor = 0.8`, `kMinContentWidth = 20`, `kBrightnessFramedThreshold = 0.721`, `kBrightnessSemiTransparentThreshold = 0.25`, `kSemiTransparentAlpha = 0x99`, `kCornerRadiusFactor = 1./3.`, `kLinePadHFactor = 1./3.`, `kMergeRadiusFactor = 1.5`, `kCondensedStretch = 78` | Text formatting & layout: 4 typefaces, multi-line background bubbles, bubble corner radius rounding and vertical line merging, luminance contrast calculations, alignment adjustments | `crates/gpui_telegram_media/src/editor/scene/scene_item_text.rs` (`SceneItemTextScene`, `gpui_ui_1to1`) |
| 24 | `Telegram/SourceFiles/editor/scene/scene_item_text.h` | `L1-L136` | `ItemText`, `TextLayoutSpec`, `TextBackgroundLine`, `BuildTextBackgroundPath` | Text scene item interface and layout specification data structures | `crates/gpui_telegram_media/src/editor/scene/scene_item_text.rs` (`SceneItemTextScene`, `gpui_ui_1to1`) |
| 25 | `Telegram/SourceFiles/editor/scene/scene_item_video.cpp` | `L1-L141` | `ItemVideo`, `kPlaybackFrameSide = 512`, `FrameSizeFor` | Embedded video clip item: playback thumbnail downscaling to 512px box, video clip player binding, looping trim window synchronization | `crates/gpui_telegram_media/src/editor/scene/scene_item_video.rs` (`SceneItemVideoScene`, `gpui_ui_1to1`) |
| 26 | `Telegram/SourceFiles/editor/scene/scene_item_video.h` | `L1-L54` | `ItemVideo` | Video item scene element interface | `crates/gpui_telegram_media/src/editor/scene/scene_item_video.rs` (`SceneItemVideoScene`, `gpui_ui_1to1`) |
| 27 | `Telegram/SourceFiles/editor/scene/scene_text_editing.cpp` | `L1-L708` | `TextEditController`, `TextEditProxy`, `kMinWidthFactor = 0.16`, `kIdealWidthExtra = 2`, `kScaleThreshold = 0.01`, `kPlaceholderOpacity = 0.38`, `kAutoShrinkHeightFactor = 1./3.`, `kAutoShrinkStep = 0.9`, `kMinFontSizeFactor = 1./50.` | Inline text editor controller: text proxy placement over scene item, auto-shrink algorithm when text overflows canvas height, placeholder rendering, color change dispatch | `crates/gpui_telegram_media/src/editor/scene/scene_text_editing.rs` (`SceneTextEditingScene`, `gpui_ui_1to1`) |
| 28 | `Telegram/SourceFiles/editor/scene/scene_text_editing.h` | `L1-L90` | `TextEditController` | Interface for managing active scene text editing sessions | `crates/gpui_telegram_media/src/editor/scene/scene_text_editing.rs` (`SceneTextEditingScene`, `gpui_ui_1to1`) |
| 29 | `Telegram/SourceFiles/editor/controllers/controllers.h` | `L1-L43` | `Editor::Controllers` | Aggregator container bundling stickers panel controller, undo controller, and UI show layers | `crates/gpui_telegram_media/src/editor/controllers/controllers.rs` (`ControllersControllers`, `gpui_ui_1to1`) |
| 30 | `Telegram/SourceFiles/editor/controllers/stickers_panel_controller.cpp` | `L1-L113` | `StickersPanelController`, `ShowRequest`, `setShowRequestChanges`, `setMoveRequestChanges` | Stickers/GIF/media drawer integration: positioning below toolbar button, animation requests (`ToggleAnimated`, `ShowAnimated`, `HideAnimated`, `HideFast`), document selection dispatch | `crates/gpui_telegram_media/src/editor/controllers/stickers_panel_controller.rs` (`StickersPanelControllerControllers`, `gpui_ui_1to1`) |
| 31 | `Telegram/SourceFiles/editor/controllers/stickers_panel_controller.h` | `L1-L53` | `StickersPanelController`, `ShowRequest` | Header definition for stickers and emoji panel controller | `crates/gpui_telegram_media/src/editor/controllers/stickers_panel_controller.rs` (`StickersPanelControllerControllers`, `gpui_ui_1to1`) |
| 32 | `Telegram/SourceFiles/editor/controllers/undo_controller.cpp` | `L1-L39` | `UndoController`, `Undo`, `EnableRequest` | Undo/redo request stream controller: binds can-perform state and triggers undo/redo command streams | `crates/gpui_telegram_media/src/editor/controllers/undo_controller.rs` (`UndoControllerControllers`, `gpui_ui_1to1`) |
| 33 | `Telegram/SourceFiles/editor/controllers/undo_controller.h` | `L1-L41` | `UndoController`, `Undo`, `EnableRequest` | Interface for tracking and executing editor undo/redo operations | `crates/gpui_telegram_media/src/editor/controllers/undo_controller.rs` (`UndoControllerControllers`, `gpui_ui_1to1`) |
| 34 | `Telegram/SourceFiles/editor/video/video_clip.cpp` | `L1-L147` | `VideoClip`, `VideoClipSource`, `setTrim`, `loopDuration` | Single video clip media manager: segment player binding, trim window clamping `[from, till]`, audio track mute/volume control, thumbnail generation | `crates/gpui_telegram_media/src/editor/video/video_clip.rs` (`VideoClipVideo`, `gpui_ui_1to1`) |
| 35 | `Telegram/SourceFiles/editor/video/video_clip.h` | `L1-L71` | `VideoClip`, `VideoClipSource`, `State` | Video clip class declaration with playback state and frame extraction interface | `crates/gpui_telegram_media/src/editor/video/video_clip.rs` (`VideoClipVideo`, `gpui_ui_1to1`) |
| 36 | `Telegram/SourceFiles/editor/video/video_editor.cpp` | `L1-L999` | `VideoEditor`, `ControlsBar`, `BarTextButton`, `kBubbleMinVisible = 600ms`, `kBubbleDuration = 500ms`, `kCaptureDuration = 420ms`, `kMaxGifSize = 10MB` | Full-screen video editing layer widget: video streaming pipeline, trim timeline, quality slider, GIF conversion mode, animated play badge, size estimation | `crates/gpui_telegram_media/src/editor/video/video_editor.rs` (`VideoEditorVideo`, `gpui_ui_1to1`) |
| 37 | `Telegram/SourceFiles/editor/video/video_editor.h` | `L1-L145` | `VideoEditor`, `VideoEditorDescriptor` | Video editor widget class interface with done/cancel event producers | `crates/gpui_telegram_media/src/editor/video/video_editor.rs` (`VideoEditorVideo`, `gpui_ui_1to1`) |
| 38 | `Telegram/SourceFiles/editor/video/video_editor_common.cpp` | `L1-L116` | `EditedFrameSize`, `VideoEdited`, `ComposeVideoSource`, `ExtractCoverImage` | Video modification validation: crop rectangle intersection, rotation transposing, audio track difference detection, transcode flag determination | `crates/gpui_telegram_media/src/editor/video/video_editor_common.rs` (`VideoEditorCommonVideo`, `gpui_ui_1to1`) |
| 39 | `Telegram/SourceFiles/editor/video/video_editor_common.h` | `L1-L65` | `VideoModifications`, `VideoEditorData` | Modification descriptors: geometry, trim bounds (`from`, `till`, `cover`), quality step, GIF flag, FPS limits | `crates/gpui_telegram_media/src/editor/video/video_editor_common.rs` (`VideoEditorCommonVideo`, `gpui_ui_1to1`) |
| 40 | `Telegram/SourceFiles/editor/video/video_editor_layer.cpp` | `L1-L318` | `ProfileVideoEditorData`, `ShowVideoEditorLayer`, `PrepareProfileVideo`, `kMaxSideRatio = 10`, `AcceptableDimensions` | Layer integration for avatar/profile video editing: 800x800 square aspect ratio constraint, 9.6s max duration, 1s min duration, 30fps transcode | `crates/gpui_telegram_media/src/editor/video/video_editor_layer.rs` (`VideoEditorLayerVideo`, `gpui_ui_1to1`) |
| 41 | `Telegram/SourceFiles/editor/video/video_editor_layer.h` | `L1-L68` | `kProfileVideoSide = 800`, `kProfileVideoMaxDuration = 9600ms`, `kProfileVideoMinDuration = 1000ms`, `kProfileVideoFps = 30.`, `ProfileMedia` | Profile video editor dimension and duration constants and layer factory functions | `crates/gpui_telegram_media/src/editor/video/video_editor_layer.rs` (`VideoEditorLayerVideo`, `gpui_ui_1to1`) |
| 42 | `Telegram/SourceFiles/editor/video/video_editor_quality.cpp` | `L1-L46` | `kLevels = { 360, 480, 720, 1080 }`, `VideoQualityLevels`, `VideoQualityLabel` | Quality ladder generation: downscaled resolutions preserving aspect ratio for shorter side < original shorter side, plus original resolution notch | `crates/gpui_telegram_media/src/editor/video/video_editor_quality.rs` (`VideoEditorQualityVideo`, `gpui_ui_1to1`) |
| 43 | `Telegram/SourceFiles/editor/video/video_editor_quality.h` | `L1-L26` | `VideoQualityLevel`, `VideoQualityLevels`, `VideoQualityLabel` | Quality level structure `{ shorterSide, resolution }` and label formatting prototypes | `crates/gpui_telegram_media/src/editor/video/video_editor_quality.rs` (`VideoEditorQualityVideo`, `gpui_ui_1to1`) |
| 44 | `Telegram/SourceFiles/editor/video/video_quality_slider.cpp` | `L1-L225` | `VideoQualitySlider`, `kShiftDuration = 120ms`, `LabelsMinSkip`, `dotX`, `indexAt` | Discrete step quality slider: animated notch selector, label rendering, snap-to-nearest notch during touch/mouse dragging | `crates/gpui_telegram_media/src/editor/video/video_quality_slider.rs` (`VideoQualitySliderVideo`, `gpui_ui_1to1`) |
| 45 | `Telegram/SourceFiles/editor/video/video_quality_slider.h` | `L1-L57` | `VideoQualitySlider` | Discrete step video quality slider widget declaration | `crates/gpui_telegram_media/src/editor/video/video_quality_slider.rs` (`VideoQualitySliderVideo`, `gpui_ui_1to1`) |
| 46 | `Telegram/SourceFiles/editor/video/video_segment_player.cpp` | `L1-L259` | `SegmentPlayer`, `FrameRequestFor`, `start`, `setSegment`, `restart`, `frame` | Video segment looping playback engine: streaming player integration, loop boundaries `[from, till]`, seeking throttle, pause/resume | `crates/gpui_telegram_media/src/editor/video/video_segment_player.rs` (`VideoSegmentPlayerVideo`, `gpui_ui_1to1`) |
| 47 | `Telegram/SourceFiles/editor/video/video_segment_player.h` | `L1-L98` | `SegmentPlayer`, `SegmentPlayerOptions` | Looping segment player interface with frame delivery and position updates | `crates/gpui_telegram_media/src/editor/video/video_segment_player.rs` (`VideoSegmentPlayerVideo`, `gpui_ui_1to1`) |
| 48 | `Telegram/SourceFiles/editor/video/video_timeline.cpp` | `L1-L287` | `VideoTimeline`, `VideoTimelineFramesCache`, `kMaxFrames = 24`, `kMaxCachedFrameSets = 8`, `kDotDuration = 500ms`, `kReloadDelay = 150ms`, `kFrameWidthTolerance = 0.25` | Video thumbnail strip timeline: async frame generator, 24 thumbnail slots, cache eviction (>8 sets), cover dot scrubber, trim handle synchronization | `crates/gpui_telegram_media/src/editor/video/video_timeline.rs` (`VideoTimelineVideo`, `gpui_ui_1to1`) |
| 49 | `Telegram/SourceFiles/editor/video/video_timeline.h` | `L1-L92` | `VideoTimeline`, `VideoTimelineFrames`, `VideoTimelineFramesCache`, `VideoTimelineDescriptor` | Timeline thumbnail frame cache and interactive trimmer interface | `crates/gpui_telegram_media/src/editor/video/video_timeline.rs` (`VideoTimelineVideo`, `gpui_ui_1to1`) |
| 50 | `Telegram/SourceFiles/editor/video/video_timeline_seeker.cpp` | `L1-L86` | `TimelineSeeker`, `kSeekThrottle = 120ms`, `seek`, `finishDragging` | Timeline scrubbing throttler: 120ms seek timer, scrubbing vs playback state coordination, boundary looping synchronization | `crates/gpui_telegram_media/src/editor/video/video_timeline_seeker.rs` (`VideoTimelineSeekerVideo`, `gpui_ui_1to1`) |
| 51 | `Telegram/SourceFiles/editor/video/video_timeline_seeker.h` | `L1-L39` | `TimelineSeeker` | Timeline scrubbing coordinator linking trim timeline with video segment player | `crates/gpui_telegram_media/src/editor/video/video_timeline_seeker.rs` (`VideoTimelineSeekerVideo`, `gpui_ui_1to1`) |

---

## 2. Subsystem Architectural Details & Algorithms

### 2.1 Affine Transformation Math & Snapping (`scene_item_base.cpp:L27-L61`, `L203-L230`)

In Telegram Desktop's editor scene, every interactive element (`ItemBase`) supports full 2D affine transformation (translation, scaling, rotation, and horizontal flipping) with rotational angle snapping and sticky alignment guidelines.

1. **Rotation Angle Calculation & Snap**:
   - Given the mouse position in item local coordinates `(mx, my)` mapped to scene coordinates `pos`, and the center of the item `origin`:
     $$\Delta x = \text{pos}.x - \text{origin}.x, \quad \Delta y = \text{pos}.y - \text{origin}.y$$
     $$\theta = \text{Normalized}\left((\text{isLeft} ? 180^\circ : 0^\circ) + \text{atan2}(\Delta y, \Delta x) \cdot \frac{180^\circ}{\pi}\right)$$
   - Angle normalization wraps any value outside $[-360^\circ, 360^\circ]$:
     $$\text{Normalized}(\theta) = \theta + (|\theta| < 360^\circ ? 0 : -360^\circ \cdot \text{sgn}(\theta))$$
   - When the Shift key is held, rotation snaps to multiples of `kSnapAngle = 45.0` degrees:
     $$\theta_{\text{snapped}} = \text{round}\left(\frac{\theta}{45.0^\circ}\right) \cdot 45.0^\circ$$

2. **Handle Sizing & Interactive Dragging**:
   - Handle width scales inversely with view zoom: `scaledHandleSize = photoEditorItemHandleSize / zoom`.
   - Right handle bounds: `QRectF(horizontalSize/2 - handleSize/2, -handleSize/2, handleSize, handleSize)`.
   - Left handle bounds: `QRectF(-horizontalSize/2 - handleSize/2, -handleSize/2, handleSize, handleSize)`.
   - Resizing adjusts `horizontalSize = clamp(max(2 * px, 2 * py), minSize, maxSize)` and recalculates `verticalSize = horizontalSize * aspectRatio`.

3. **Sticky Alignment Guidelines (`kStickyLines = 5`, `kStickyDuration = 150ms`)**:
   - Upstream evaluates 5 equidistant guidelines across the canvas:
     $$\text{StickyLine}(i) = \text{Start} + \text{Length} \cdot \frac{i}{kStickyLines - 1}, \quad i \in \{0, 1, 2, 3, 4\}$$
   - Proximity detection triggers within `scaledStickyTrigger = 6.0 / zoom`.
   - Animated snapping smoothly translates item coordinates with `easeOutCubic` over 150ms.

### 2.2 Vector Canvas Stroke Smoothing & Eraser Algorithm (`scene_item_canvas.cpp:L19-L36`)

Freehand drawing uses a high-fidelity quadratic Bezier curve interpolation algorithm with dynamic distance thresholding:
- Minimum point distance: `kMinPointDistanceBase = 2.0`
- Maximum point distance: `kMaxPointDistance = 15.0`
- Smoothing strength: `kSmoothingStrength = 0.5`
- Half strength: `kHalfStrength = 0.25`, Inverse: `kInvStrength = 0.5`
- Interpolated point formula:
  $$P_{\text{smooth}} = P_{\text{prev}} \cdot kHalfStrength + P_{\text{curr}} \cdot kInvStrength + P_{\text{next}} \cdot kHalfStrength$$
- Eraser tool applies mask subtraction to previously painted stroke pixmaps without modifying static background image data.

### 2.3 Geometric Shapes Math & Vector Curves (`scene_item_shape.cpp:L21-L32`)

Vector shapes implement parametric formulas:
1. **Star**: 5-point star with golden ratio:
   - Outer-to-inner radius ratio: `kStarInnerRatio = 0.4316`
   - Bounding box aspect ratio: `kStarBoxHeight / kStarBoxWidth = 1.809017 / 1.902113 \approx 0.9510565`
2. **Speech Bubble**:
   - Body to height ratio: `kBubbleBodyRatio = 2.0 / 2.25 \approx 0.8889`
   - Tail offset: `kBubbleTailRatio = 0.75`
3. **Curved Arrow**:
   - Arrowhead length ratio: `kArrowHeadRatio = 0.16`
   - Arrowhead angle: `kArrowHeadAngle = 30^\circ` ($30 \cdot \pi / 180 \approx 0.5236$ rad)
   - Maximum curvature bend ratio: `kArrowMaxBendRatio = 0.35`
   - Arrowhead stroke factor: `kArrowHeadStrokeFactor = 2.5`

### 2.4 Text Rendering, Bubble Merging & Auto-Shrink (`scene_item_text.cpp:L34-L43`, `scene_text_editing.cpp:L27-L38`)

Text items support 4 Telegram font typefaces (Default, Outlined, Framed, SemiTransparent):
- Multi-line bubble rendering:
  - `kPaddingFactor = 0.4`, `kMaxWidthFactor = 0.8`, `kMinContentWidth = 20`
  - `kBrightnessFramedThreshold = 0.721`, `kBrightnessSemiTransparentThreshold = 0.25`
  - `kSemiTransparentAlpha = 0x99` (60% opacity)
  - Corner rounding factor: `kCornerRadiusFactor = 1.0 / 3.0`
  - Adjacent line merge radius: `kMergeRadiusFactor = 1.5`
- Auto-shrink controller:
  - If text document height exceeds available editor bounds, font size scales by `kAutoShrinkStep = 0.9` iteratively down to `kMinFontSizeFactor = 1.0 / 50.0`.

### 2.5 Undo/Redo State Machine & Transaction Log (`undo_controller.cpp:L15-L39`, `scene.cpp:L29-L76`)

The scene graph tracks reversible operations via `NumberedItem` and `ItemAction`:
- Operations increment monotonically increasing sequence numbers `_itemNumber`.
- Each action records `.save(SaveState::Keep)` and `.save(SaveState::Apply)` snapshots.
- Undo/redo streams broadcast `EnableRequest { command: Undo::Undo | Undo::Redo, enable: bool }`.
- Deleting, adding, modifying transforms, or erasing strokes can be unrolled and re-applied without data loss.

### 2.6 Video Clip Trim, Scrubbing & Timeline FSM (`video_clip.cpp:L54-L60`, `video_timeline.cpp:L18-L24`, `video_timeline_seeker.cpp:L16-L30`)

Video editing provides non-destructive trimming and playback scrubbing:
- Trim boundaries: `from = clamp(from, 0, duration)` and `till = (till > 0) ? clamp(till, from, duration) : 0`.
- Thumbnail strip: `kMaxFrames = 24`, cache retention `kMaxCachedFrameSets = 8`, reload debounce `kReloadDelay = 150ms`.
- Cover frame scrubber dot animation: `kDotDuration = 500ms`.
- Scrubbing throttler: `kSeekThrottle = 120ms` decouples high-frequency mouse moves from video decoder restarts.

### 2.7 Video Quality Ladder & Profile Video Constraints (`video_editor_quality.cpp:L16-L38`, `video_editor_layer.h:L28-L32`)

- Quality resolution rungs: `kLevels = [360, 480, 720, 1080]`.
- Output ladder generates rungs where `shorterSide < min(width, height)`, followed by original `shorterSide = 0`.
- Profile video constraints:
  - `kProfileVideoSide = 800` (1:1 square)
  - `kProfileVideoMinDuration = 1000ms` (1.0s)
  - `kProfileVideoMaxDuration = 9600ms` (9.6s)
  - `kProfileVideoFps = 30.0`
  - Audio is muted (`removeAudio = true`), forced transcode (`transcodeAlways = true`).

---

## 3. Dynamic Rust Architecture & Implementation Plan

The Rust implementation replaces all Pattern D boilerplate across `crates/gpui_telegram_media/src/editor/` with high-cohesion, authentic structures:

1. **`editor/scene/scene_item_base.rs`**: Core `ItemPlacement`, `AffineTransform2D`, handle hit-testing (`kSnapAngle = 45.0`, `kStickyLines = 5`), and sticky alignment math.
2. **`editor/scene/scene_item_shape.rs`**: Geometric shapes (`ShapeType`: Rectangle, Circle, Star, SpeechBubble, Arrow) with exact radius ratios and bend control points.
3. **`editor/scene/scene_item_canvas.rs`**: Stroke smoothing engine with distance thresholds and eraser mask blending.
4. **`editor/scene/scene_item_text.rs` & `scene_text_editing.rs`**: Text styling, multi-line bubble geometry, and auto-shrink engine.
5. **`editor/scene/scene.rs`**: Scene graph manager with item z-ordering, selection, and undo stack.
6. **`editor/scene/scene_emoji_document.rs`**, `scene_item_animated.rs`, `scene_item_image.rs`, `scene_item_line.rs`, `scene_item_link.rs`, `scene_item_message.rs`, `scene_item_sticker.rs`, `scene_item_video.rs`: Dedicated authentic elements.
7. **`editor/controllers/`**: `UndoController`, `StickersPanelController`, and `Controllers` aggregator.
8. **`editor/video/`**: `VideoClip`, `VideoTimeline`, `TimelineSeeker`, `VideoQualitySlider`, `VideoEditorQuality`, `VideoSegmentPlayer`, `VideoEditorCommon`, `VideoEditorLayer`, and `VideoEditor`.

---

## 4. Verification & Test Plan

A comprehensive subsystem test suite is authored at `crates/gpui_telegram_media/tests/editor_scene_video_subsystem_tests.rs` containing >= 10 tests verifying:
- Affine transformation angle calculation and 45-degree snap quantization.
- 5-line sticky guideline coordinate generation and snap trigger logic.
- Star, speech bubble, and arrow parametric geometry ratios.
- Vector stroke smoothing algorithm and distance thresholds.
- Multi-line text bubble margin and auto-shrink scale calculation.
- Undo/redo controller event streaming and transaction reversal.
- Video trim clamping and duration normalization.
- Video quality ladder downscaling and original fallback.
- Profile video dimension and duration constraint enforcement.
- Video timeline scrubbing throttler state machine.



---

<!-- Slice File: module_09_editor_root_theme.md -->

# Module 09 Specification: Editor Root Components & Theme Subsystem (`editor/*`, `theme/editor/*`)

## Overview

Module 09 Slice 7 delivers the core editing engine, visual canvas controls, media timelines, brush tools, color pickers, and global editor stylesheet metrics for GPUI Telegram Desktop. This subsystem provides comprehensive photo transformation and cropping, raster paint and drawing tools, vector shapes and typography presets, audio track timelines with waveform visualization, video trim and playback timelines with millisecond stamp parsing, link pill badges, message preview rendering, and full theme metric integration from `editor.style`.

This specification is authored in strict compliance with:
- **Mandate #15 (Zero Tolerance for Boilerplate)**: All generic boilerplate stubs (`{ id, label, value }` Patterns A, B, C, D) are completely eradicated and replaced with authentic domain models, exact mathematical algorithms, and live GPUI components.
- **Mandate #17 (Dynamic Target File Architecture)**: Upstream C++ source files serve as the authoritative reading checklist. Target Rust/GPUI architecture decomposes Qt widgets into idiomatic GPUI elements, state machines, and theme sheets.
- **Mandate #18 (Anti-Bluffing Protocol)**: Every component, algorithm, cache key, and metric cites exact upstream C++ implementation file line ranges (`.cpp:L<start>-L<end>`), header definitions (`.h:L<start>-L<end>`), and style tokens (`.style:L<start>-L<end>`), extracting anonymous-namespace `k...` constants and mathematical formulas.
- **Mandate #19 (Zero-Omitted-File Reading Guarantee)**: Every single one of the 44 assigned upstream C++ and style files in `Telegram/SourceFiles/editor/` (43 C++ files + 1 `.style` file) is accounted for in the 100% Per-File Reading Manifest below (`omitted_from_design_doc == 0`).
- **Mandate #20 (Stream Hygiene)**: Authored in modular segments under 200 lines per tool call.

---

## 1. 100% Per-File Reading Manifest (All 44 Upstream C++ & Style Sources)

| # | Upstream C++ Source Path | Exact Line Range Read | Upstream Symbols & Classes | Extracted Constants & Logic | Dynamic Rust Target Module & Disposition |
|---|---|---|---|---|---|
| 1 | `Telegram/SourceFiles/editor/audio_track_timeline.cpp` | `L1-L81` | `AudioTrackTimeline` | `resizeToWidth`, `photoEditorButtonBarHeight` | `crates/gpui_telegram_media/src/editor/audio_track_timeline.rs` (`AudioTrackTimeline`, `gpui_ui_1to1`) |
| 2 | `Telegram/SourceFiles/editor/audio_track_timeline.h` | `L1-L43` | `AudioTrackTimeline` | `setTrack`, `refreshTrim`, `commitPendingEdit` | `crates/gpui_telegram_media/src/editor/audio_track_timeline.rs` (`AudioTrackTimeline`, `gpui_ui_1to1`) |
| 3 | `Telegram/SourceFiles/editor/color_picker.cpp` | `L1-L1117` | `ColorPicker`, `ColorButton`, `PlusCircle`, `ToolLottieButton` | `kMinBrushSize=0.1`, `kMinBrushWidth=1.`, `kMaxBrushWidth=25.`, `kCircleDuration=200ms`, `kSizeControlSwitchDuration=140ms`, `PaletteColors` (10 colors), conical gradient 7 stops | `crates/gpui_telegram_media/src/editor/color_picker.rs` (`ColorPicker`, `gpui_ui_1to1`) |
| 4 | `Telegram/SourceFiles/editor/color_picker.h` | `L1-L121` | `ColorPicker` | `moveLine`, `setCanvasRect`, `sizeControlRatioFromY`, `sizeControlShapePath` | `crates/gpui_telegram_media/src/editor/color_picker.rs` (`ColorPicker`, `gpui_ui_1to1`) |
| 5 | `Telegram/SourceFiles/editor/editor.style` | `L1-L400` | Style definitions | `photoEditorControlsHeight: 146px`, `photoEditorButtonBarHeight: 48px`, `photoEditorMarkerOpacity: 0.35`, `photoEditorArrowHeadLengthFactor: 2.5`, `photoEditorBlurRadius: 20` | `crates/gpui_telegram_desktop/src/theme/editor/editor_style.rs` (`EditorStyleSheet`, `gpui_ui_1to1`) |
| 6 | `Telegram/SourceFiles/editor/editor_audio_disc_button.cpp` | `L1-L155` | `AudioDiscButton` | `kSpinDuration = 4000ms`, `angle = fmod(spinBase + passed * 360 / kSpinDuration, 360)`, `st::photoEditorAudioDiscCoverSize` | `crates/gpui_telegram_media/src/editor/editor_audio_disc_button.rs` (`EditorAudioDiscButton`, `gpui_ui_1to1`) |
| 7 | `Telegram/SourceFiles/editor/editor_audio_disc_button.h` | `L1-L58` | `AudioDiscButton` | `setTrack`, `setActive`, `angle`, `prepareRippleMask` | `crates/gpui_telegram_media/src/editor/editor_audio_disc_button.rs` (`EditorAudioDiscButton`, `gpui_ui_1to1`) |
| 8 | `Telegram/SourceFiles/editor/editor_audio_menu.cpp` | `L1-L229` | `MenuVolumeItem`, `AddVolumeAction`, `CreateAudioMenu` | `kMuteDuration = 150ms`, `kSpeakerThresholds = {0.05, 0.25, 0.75}`, `kVolumeStickedValues = {{0.25, 0.02}, {0.5, 0.02}, {0.75, 0.02}}` | `crates/gpui_telegram_media/src/editor/editor_audio_menu.rs` (`EditorAudioMenu`, `gpui_ui_1to1`) |
| 9 | `Telegram/SourceFiles/editor/editor_audio_menu.h` | `L1-L81` | `MenuVolumeItem` | `applyVolume`, `updateSliderColor`, `unmuteColor`, `muteColor` | `crates/gpui_telegram_media/src/editor/editor_audio_menu.rs` (`EditorAudioMenu`, `gpui_ui_1to1`) |
| 10 | `Telegram/SourceFiles/editor/editor_audio_timeline.cpp` | `L1-L299` | `AudioTimeline` | `kWaveformBarDuration = 20ms`, `kMaxWaveformBars = 16384`, `kLoadedDuration = 600ms`, `TrimDescriptor` | `crates/gpui_telegram_media/src/editor/editor_audio_timeline.rs` (`EditorAudioTimeline`, `gpui_ui_1to1`) |
| 11 | `Telegram/SourceFiles/editor/editor_audio_timeline.h` | `L1-L70` | `AudioTimeline` | `setPlaying`, `refreshTrim`, `loadWaveform`, `paintWaveform` | `crates/gpui_telegram_media/src/editor/editor_audio_timeline.rs` (`EditorAudioTimeline`, `gpui_ui_1to1`) |
| 12 | `Telegram/SourceFiles/editor/editor_crop.cpp` | `L1-L664` | `Crop`, `PointOfEdge`, `FlipSizeByRotation`, `AdjustCropToInner` | `kMaxCanvasRatio = 2.0`, `OriginalCrop`, `3x3 grid line painter`, `cornerLength = min(st::photoEditorCropPointSize * 2, min(w, h)/2)` | `crates/gpui_telegram_media/src/editor/editor_crop.rs` (`EditorCrop`, `gpui_ui_1to1`) |
| 13 | `Telegram/SourceFiles/editor/editor_crop.h` | `L1-L126` | `Crop` | `applyTransform`, `saveCropRect`, `setAspectRatio`, `setCornersLevel` | `crates/gpui_telegram_media/src/editor/editor_crop.rs` (`EditorCrop`, `gpui_ui_1to1`) |
| 14 | `Telegram/SourceFiles/editor/editor_keys_legend.cpp` | `L1-L302` | `KeysLegendButton`, `FixedCropRows`, `CropSection`, `CanvasSection` | Key mappings (`CtrlKey`, `ShiftKey`, `DeleteKey`), gesture strings | `crates/gpui_telegram_media/src/editor/editor_keys_legend.rs` (`EditorKeysLegend`, `gpui_ui_1to1`) |
| 15 | `Telegram/SourceFiles/editor/editor_keys_legend.h` | `L1-L50` | `KeysLegendButton`, `KeysLegendContext` | `toggle`, `showLegend`, `tooltipText` | `crates/gpui_telegram_media/src/editor/editor_keys_legend.rs` (`EditorKeysLegend`, `gpui_ui_1to1`) |
| 16 | `Telegram/SourceFiles/editor/editor_layer_widget.cpp` | `L1-L93` | `LayerWidget` | `BlurOverlayAlpha(night)`: night ? 128 : 192, forwardKeyEvent | `crates/gpui_telegram_media/src/editor/editor_layer_widget.rs` (`EditorLayerWidget`, `gpui_ui_1to1`) |
| 17 | `Telegram/SourceFiles/editor/editor_layer_widget.h` | `L1-L51` | `LayerWidget` | `parentResized`, `closeByOutsideClick`, `forwardKeyEvent` | `crates/gpui_telegram_media/src/editor/editor_layer_widget.rs` (`EditorLayerWidget`, `gpui_ui_1to1`) |
| 18 | `Telegram/SourceFiles/editor/editor_link_box.cpp` | `L1-L1009` | `LinkBox`, `LoadingMessage` | `kResolveDelay = 700ms`, `kLoadingTitleRatio = 0.45`, `kLoadingLineRatios = {0.32, 0.58, 0.86}`, `kGlareTimeout = 1000ms`, `kGlareDuration = 1000ms` | `crates/gpui_telegram_media/src/editor/editor_link_box.rs` (`EditorLinkBox`, `gpui_ui_1to1`) |
| 19 | `Telegram/SourceFiles/editor/editor_link_box.h` | `L1-L38` | `LinkBoxResult`, `LinkBoxArgs`, `LinkBox` | `LinkBoxResult` structure, dialog box constructor | `crates/gpui_telegram_media/src/editor/editor_link_box.rs` (`EditorLinkBox`, `gpui_ui_1to1`) |
| 20 | `Telegram/SourceFiles/editor/editor_link_pill.cpp` | `L1-L136` | `LinkPill`, `WithoutScheme` | `kDpPerWidth = 360.`, `kPaddingLeft = 4.`, `kPaddingTop = 4.33`, `kPaddingRight = 7.66`, `kPaddingBottom = 3.`, `kIconSize = 30.`, `kFontSize = 24.`, `kSemiTransparentAlpha = 0x4C` | `crates/gpui_telegram_media/src/editor/editor_link_pill.rs` (`EditorLinkPill`, `gpui_ui_1to1`) |
| 21 | `Telegram/SourceFiles/editor/editor_link_pill.h` | `L1-L59` | `LinkPreview`, `LinkStyle`, `LinkPill` | `DensityFor(canvasWidth) = canvasWidth / 360.`, `setStyle`, `paint` | `crates/gpui_telegram_media/src/editor/editor_link_pill.rs` (`EditorLinkPill`, `gpui_ui_1to1`) |
| 22 | `Telegram/SourceFiles/editor/editor_message_render.cpp` | `L1-L398` | `MessageRenderer`, `OpaqueBounds`, `CornerRadius` | `OpaqueBounds` alpha scanning, `CachedCornerRadiusValue`, `MakeChatTheme` | `crates/gpui_telegram_media/src/editor/editor_message_render.rs` (`EditorMessageRender`, `gpui_ui_1to1`) |
| 23 | `Telegram/SourceFiles/editor/editor_message_render.h` | `L1-L71` | `MessageRenderer` | `setRepaintCallback`, `setDark`, `mediaRect`, `videoMask`, `render` | `crates/gpui_telegram_media/src/editor/editor_message_render.rs` (`EditorMessageRender`, `gpui_ui_1to1`) |
| 24 | `Telegram/SourceFiles/editor/editor_message_source.cpp` | `L1-L136` | `MessageSource`, `MessageForbidsRender`, `CanRenderMessage` | `MessageToRender`, `watchRemoval` | `crates/gpui_telegram_media/src/editor/editor_message_source.rs` (`EditorMessageSource`, `gpui_ui_1to1`) |
| 25 | `Telegram/SourceFiles/editor/editor_message_source.h` | `L1-L59` | `MessageSource` | `session`, `item`, `link`, `webpage`, `video`, `removed` | `crates/gpui_telegram_media/src/editor/editor_message_source.rs` (`EditorMessageSource`, `gpui_ui_1to1`) |
| 26 | `Telegram/SourceFiles/editor/editor_message_video.cpp` | `L1-L186` | `MessageVideo` | `readSource`, `waitForLoader`, `fail`, `progress` | `crates/gpui_telegram_media/src/editor/editor_message_video.rs` (`EditorMessageVideo`, `gpui_ui_1to1`) |
| 27 | `Telegram/SourceFiles/editor/editor_message_video.h` | `L1-L58` | `MessageVideo` | `Find`, `document`, `load`, `loading`, `progress`, `source` | `crates/gpui_telegram_media/src/editor/editor_message_video.rs` (`EditorMessageVideo`, `gpui_ui_1to1`) |
| 28 | `Telegram/SourceFiles/editor/editor_paint.cpp` | `L1-L1197` | `Paint`, `BrushSize`, `DefaultShapeSize` | `kMaxBrush=25.`, `kMinBrush=1.`, `kShapeSizeRatio=2/5`, `kMediaSizeRatio=1/2`, `kImageMaxSizeRatio=4.`, `kMessageMaxWidthRatio=0.88`, `kMessageMaxHeightRatio=0.7`, `kMinCanvasZoom=1.`, `kMaxCanvasZoom=8.`, `kCanvasZoomStep=1.15` | `crates/gpui_telegram_media/src/editor/editor_paint.rs` (`EditorPaint`, `gpui_ui_1to1`) |
| 29 | `Telegram/SourceFiles/editor/editor_paint.h` | `L1-L203` | `Paint` | `saveScene`, `restoreScene`, `applyTransform`, `applyBrush`, `zoomSceneItems` | `crates/gpui_telegram_media/src/editor/editor_paint.rs` (`EditorPaint`, `gpui_ui_1to1`) |
| 30 | `Telegram/SourceFiles/editor/editor_trim_timeline.cpp` | `L1-L1208` | `TrimTimeline`, `Stamp`, `ParseStamp` | `kZoomStep=1.2`, `kMinVisibleSpan=2000ms`, `kMinSelection=100ms`, `kEdgeScrollSpeed=9.`, `Stamp` precision 3 formatting, `ParseStamp` milliseconds parser | `crates/gpui_telegram_media/src/editor/editor_trim_timeline.rs` (`EditorTrimTimeline`, `gpui_ui_1to1`) |
| 31 | `Telegram/SourceFiles/editor/editor_trim_timeline.h` | `L1-L208` | `TrimTimeline`, `TrimTimelineDescriptor` | `from`, `till`, `cover`, `setTrim`, `setPlaybackPosition`, `timeAt`, `xAt` | `crates/gpui_telegram_media/src/editor/editor_trim_timeline.rs` (`EditorTrimTimeline`, `gpui_ui_1to1`) |
| 32 | `Telegram/SourceFiles/editor/photo_editor.cpp` | `L1-L713` | `PhotoEditor`, `DefaultBrushes`, `Serialize` | `kPrecision=100000`, `kBrushesVersion=-2`, `kDefaultBrushSizeRatio=0.9`, default colors: Pen(234,39,57), Arrow(252,150,77), Marker(252,222,101), Eraser(0,0,0), Blur(0,0,0) | `crates/gpui_telegram_media/src/editor/photo_editor.rs` (`PhotoEditor`, `gpui_ui_1to1`) |
| 33 | `Telegram/SourceFiles/editor/photo_editor.h` | `L1-L100` | `PhotoEditor`, `InitEditorLayer` | `save`, `doneRequests`, `cancelRequests`, `updateColorPickerVisibility` | `crates/gpui_telegram_media/src/editor/photo_editor.rs` (`PhotoEditor`, `gpui_ui_1to1`) |
| 34 | `Telegram/SourceFiles/editor/photo_editor_common.cpp` | `L1-L309` | `ApplyShapeMask`, `RoundedCornersMultiplier`, `ExpandCanvas` | `kAnimatedMaxSide=854`, `kAnimatedFps=30.`, `kAnimatedMinDuration=1000ms`, `RoundedCornersMultiplier`: Circle=0.5, Large=0.44, Medium=0.2, Small=0.12, None=0.0 | `crates/gpui_telegram_media/src/editor/photo_editor_common.rs` (`PhotoEditorCommon`, `gpui_ui_1to1`) |
| 35 | `Telegram/SourceFiles/editor/photo_editor_common.h` | `L1-L100` | `AudioTrack`, `RoundedCornersLevel`, `EditorData`, `PhotoModifications` | `AudioTrack::length`, `PhotoModifications::empty`, `ImageModified`, `ComposeAnimatedJob` | `crates/gpui_telegram_media/src/editor/photo_editor_common.rs` (`PhotoEditorCommon`, `gpui_ui_1to1`) |
| 36 | `Telegram/SourceFiles/editor/photo_editor_content.cpp` | `L1-L631` | `PhotoEditorContent`, `MaxDistance` | `kExpansionRoomRatio=0.7`, `kLayoutSmoothTau=60.`, `kLayoutMaxFrameDelta=64ms`, `kLayoutEpsilon=0.5`, `MaxDistance` formula | `crates/gpui_telegram_media/src/editor/photo_editor_content.rs` (`PhotoEditorContent`, `gpui_ui_1to1`) |
| 37 | `Telegram/SourceFiles/editor/photo_editor_content.h` | `L1-L128` | `PhotoEditorContent` | `applyModifications`, `applyMode`, `applyBrush`, `createTextItem`, `createShapeItem` | `crates/gpui_telegram_media/src/editor/photo_editor_content.rs` (`PhotoEditorContent`, `gpui_ui_1to1`) |
| 38 | `Telegram/SourceFiles/editor/photo_editor_controls.cpp` | `L1-L1396` | `PhotoEditorControls`, `CheckAction`, `ShapeAction`, `EdgeButton` | `EdgeButton` ripple calculation, `RoundCheckAction` animation, button top alignment | `crates/gpui_telegram_media/src/editor/photo_editor_controls.rs` (`PhotoEditorControls`, `gpui_ui_1to1`) |
| 39 | `Telegram/SourceFiles/editor/photo_editor_controls.h` | `L1-L149` | `PhotoEditorControls` | `rotateRequests`, `flipRequests`, `paintModeRequests`, `setShapeToolActive` | `crates/gpui_telegram_media/src/editor/photo_editor_controls.rs` (`PhotoEditorControls`, `gpui_ui_1to1`) |
| 40 | `Telegram/SourceFiles/editor/photo_editor_inner_common.h` | `L1-L106` | `PhotoEditorMode`, `Brush`, `TextStyle`, `TextTypeface`, `TextAlignment`, `TextPrefs`, `ShapeType`, `ShapeRequest`, `VideoTrim` | Core domain enums and structs for tools, brush parameters, text formatting, and video trim | `crates/gpui_telegram_media/src/editor/photo_editor_inner_common.rs` (`PhotoEditorInnerCommon`, `gpui_ui_1to1`) |
| 41 | `Telegram/SourceFiles/editor/photo_editor_layer_widget.cpp` | `L1-L182` | `OpenWithPreparedFile`, `PrepareProfilePhoto`, `PrepareProfilePhotoFromFile` | `ValidateThumbDimensions`, `PhotoSideLimit`, layer callback flow | `crates/gpui_telegram_media/src/editor/photo_editor_layer_widget.rs` (`PhotoEditorLayerWidget`, `gpui_ui_1to1`) |
| 42 | `Telegram/SourceFiles/editor/photo_editor_layer_widget.h` | `L1-L57` | `kProfilePhotoSize = 640` | `OpenWithPreparedFile`, `PrepareProfilePhoto`, `PrepareProfilePhotoFromFile` | `crates/gpui_telegram_media/src/editor/photo_editor_layer_widget.rs` (`PhotoEditorLayerWidget`, `gpui_ui_1to1`) |
| 43 | `Telegram/SourceFiles/editor/video_item_timeline.cpp` | `L1-L112` | `VideoItemTimeline` | `VideoTimelineDescriptor`, `TimelineSeeker`, volume action popup | `crates/gpui_telegram_media/src/editor/video_item_timeline.rs` (`VideoItemTimeline`, `gpui_ui_1to1`) |
| 44 | `Telegram/SourceFiles/editor/video_item_timeline.h` | `L1-L51` | `VideoItemTimeline` | `setClip`, `refreshTrim`, `commitPendingEdit`, `lengthChanges` | `crates/gpui_telegram_media/src/editor/video_item_timeline.rs` (`VideoItemTimeline`, `gpui_ui_1to1`) |

### Reading Manifest Verification Summary

- **Total Assigned Upstream Files**: 44 / 44 (100.0% coverage, 0 omitted)
  - Root `Telegram/SourceFiles/editor/` C++ files: 43 entries (21 `.h`/`.cpp` pairs + 1 `.h` `photo_editor_inner_common.h`)
  - Root style file: 1 entry (`Telegram/SourceFiles/editor/editor.style`)
- **Target Rust Files in Scope**:
  - `crates/gpui_telegram_media/src/editor/*.rs` (22 root modules)
  - `crates/gpui_telegram_desktop/src/theme/editor/editor_style.rs` (1 theme stylesheet)
- **Zero Boilerplate Guarantee**: All 23 target files transition completely from autogenerated `{ id, label, value }` and `{ id, tag, value }` stubs to authentic, mathematically faithful domain models and state machines.


---

## 2. Deep Source Analysis & Mathematical Algorithms

### 2.1 Color Picker, HSV Color Wheel & Size Control (`color_picker.cpp:L27-L300`)

1. **Brush Size Bounds & Interpolation** (`color_picker.cpp:L28-L43`):
   - Minimum brush size normalized: `kMinBrushSize = 0.1`
   - Minimum brush pixel width: `kMinBrushWidth = 1.0`
   - Maximum brush pixel width: `kMaxBrushWidth = 25.0`
   - Linear interpolation:
     $$\text{InterpolateF}(a, b, t) = a + (b - a) \times t$$
     $$\text{InterpolationRatio}(\text{from}, \text{to}, \text{val}) = \frac{\text{val} - \text{from}}{\text{to} - \text{from}}$$
2. **Brush Tools & Fixed Color Tools** (`color_picker.cpp:L44-L78`):
   - Tool Indices: Pen (0), Arrow (1), Marker (2), Blur (3), Eraser (4)
   - Fixed Color Tools: `FixedColorTool(tool)` returns `true` for `Eraser` and `Blur`. Their color is fixed to `QColor(0, 0, 0)` (Black).
3. **Color Wheel Conical Gradient** (`color_picker.cpp:L143-L151`):
   The outer rainbow ring is rendered via a conical gradient centered at the button with 7 discrete angle stops:
   - $0/7$: `#EB4B4B` (Red)
   - $1/7$: `#FFA500` (Orange)
   - $2/7$: `#FFFF00` (Yellow)
   - $3/7$: `#8FCE00` (Green)
   - $4/7$: `#00FFFF` (Cyan)
   - $5/7$: `#6080E4` (Blue)
   - $6/7$: `#EE82EE` (Violet)
   - $7/7$: `#EB4B4B` (Red loop)
4. **Default Color Palette** (`color_picker.cpp:L266-L279`):
   The quick-select palette provides 10 standard colors:
   - `#000000` (Black), `#FFFFFF` (White), `#EA2739` (Red), `#FC964D` (Orange), `#FCDE65` (Yellow),
   - `#80C864` (Green), `#49C5ED` (Cyan/Light Blue), `#3051E3` (Blue), `#DB3AD2` (Magenta), `#FF72A9` (Pink).
5. **Size Control Geometry & Handle** (`color_picker.cpp:L61-L72`):
   - Collapsed width: 6px; Expanded top width: 25px; Expanded bottom width: 4px.
   - Ratio to Y coordinate mapping:
     $$y(\text{ratio}) = y_{\text{bottom}} - \text{ratio} \times (y_{\text{bottom}} - y_{\text{top}})$$
     $$\text{ratio}(y) = \frac{y_{\text{bottom}} - y}{y_{\text{bottom}} - y_{\text{top}}}$$

### 2.2 Paint & Brush Engine (`editor_paint.cpp:L54-L147`, `editor.style:L186-L195`)

1. **Brush Size Computation** (`editor_paint.cpp:L81-L84`):
   $$\text{BrushSize}(\text{brush}) = kMinBrush + (kMaxBrush - kMinBrush) \times \text{brush.sizeRatio}$$
   Where $kMinBrush = 1.0$ and $kMaxBrush = 25.0$.
2. **Default Shape & Media Ratios** (`editor_paint.cpp:L56-L63`):
   - Shape Size Ratio: $kShapeSizeRatio = 2/5 = 0.40$ of the smaller image dimension.
   - Media Size Ratio: $kMediaSizeRatio = 1/2 = 0.50$.
   - Max Image Size Ratio: $kImageMaxSizeRatio = 4.0$.
   - Message Max Bounds: Width ratio $0.88$, Height ratio $0.70$, Cascade offset ratio $1/20 = 0.05$.
3. **Special Brush Characteristics** (`editor.style:L186-L195`):
   - **Marker**: Opacity $0.35$, Size Multiplier $2.5\times$.
   - **Arrow**: Head Length Factor $2.5\times$, Min Distance Factor $1.5\times$, Head Angle $26^\circ$.
   - **Eraser**: Preview Opacity $0.25$.
   - **Blur**: Size Multiplier $3.0\times$, Preview Opacity $0.25$, Blur Kernel Radius $20\text{px}$.
4. **Canvas Zoom & Smoothing** (`editor_paint.cpp:L92-L102`):
   - Zoom range: $kMinCanvasZoom = 1.0$ to $kMaxCanvasZoom = 8.0$.
   - Step: $1.15\times$ (normal wheel), $1.015\times$ (fine wheel with Shift).
   - Smoothing: Exponential smoothing filter with time constant $kZoomSmoothTau = 60.0\text{ms}$.
   - Text item default font size: $\text{fontSize} = \min(\text{width}, \text{height}) / 15.0$.

### 2.3 Crop Controls & Geometry Transforms (`editor_crop.cpp:L17-L83`, `photo_editor_common.cpp:L93-L100`)

1. **Rotation & Flip Math** (`editor_crop.cpp:L49-L51`):
   - When angle is rotated by $90^\circ$ or $270^\circ$ (`((angle / 90) % 2) == 1`), width and height are transposed.
   - Scaled matrix incorporates center translation, horizontal flip (`scale(-1, 1)` if flipped), and rotation.
2. **Rounded Corners Multipliers** (`photo_editor_common.cpp:L93-L100`):
   - `Circle`: $0.50$ (half of min dimension = circular pill/ellipse)
   - `Large`: $0.44$ (Squircle / Forum Monoforum avatar radius)
   - `Medium`: $0.20$
   - `Small`: $0.12$
   - `None`: $0.00$
3. **Rule of Thirds Grid Painting** (`editor_crop.cpp:L257-L281`):
   - Grid divides crop area into a $3 \times 3$ grid with horizontal steps at $w/3$ and $2w/3$, and vertical steps at $h/3$ and $2h/3$.
   - Corner handle lines: length $\min(20\text{px}, \min(w, h) / 2)$, line thickness $4 \times \text{st::lineWidth}$.


### 2.4 Timelines & Playback Control (`editor_trim_timeline.cpp:L25-L100`, `editor_audio_disc_button.cpp:L23-L93`, `editor_audio_menu.cpp:L25-L35`)

1. **Trim Timeline Bounds & Stamp Parsing** (`editor_trim_timeline.cpp:L25-L100`):
   - $kMinVisibleSpan = 2000\text{ms}$ (minimum visible span on screen)
   - $kMinSelection = 100\text{ms}$ (minimum allowed trim clip duration)
   - $kZoomStep = 1.20$, $kZoomSnap = 0.001$
   - $kEdgeScrollSpeed = 9.0$ pixels per second overshoot, max $3.0\times$ strip width/sec.
   - Timestamp parsing (`ParseStamp`): parses `SS.mmm`, `MM:SS.mmm`, or `HH:MM:SS.mmm` into exact integer milliseconds with precision up to 3 fractional digits.
   - Timestamp formatting (`Stamp`): formats milliseconds into `MM:SS.f` with customizable decimal precision.
2. **Audio Disc Spinning Animation** (`editor_audio_disc_button.cpp:L23-L93`):
   - Full rotation duration: $kSpinDuration = 4000\text{ms}$ (4.0 seconds per 360-degree rotation).
   - Angle at time $t$:
     $$\theta(t) = \left(\theta_0 + \frac{(t - t_{\text{start}}) \times 360.0}{4000.0}\right) \pmod{360.0}$$
3. **Volume Slider Magnetic Snapping** (`editor_audio_menu.cpp:L27-L34`):
   - Speaker thresholds for icon arcs: `[0.05, 0.25, 0.75]`.
   - Magnetic snap points: `0.25` (snap window $\pm 0.02$), `0.50` ($\pm 0.02$), `0.75` ($\pm 0.02$).

### 2.5 Link Previews, Pills & Message Rendering (`editor_link_pill.cpp:L16-L97`, `editor_link_box.cpp:L47-L52`, `editor_message_render.cpp:L35-L78`)

1. **Link Pill Density & Sizing** (`editor_link_pill.cpp:L16-L63`):
   - Canvas baseline width: $kDpPerWidth = 360.0\text{px}$.
   - Density: $\text{density} = \text{canvasWidth} / 360.0$.
   - Padding metrics: Left $4.0\text{px}$, Top $4.33\text{px}$, Right $7.66\text{px}$, Bottom $3.0\text{px}$, Icon Size $30.0\text{px}$, Icon Padding $3.25\text{px}$, Font Size $24.0\text{px}$.
   - Pill Style Background & Foreground:
     - `Framed`: Background `#FFFFFF`, Foreground `#3391D4`.
     - `SemiTransparent`: Background `#000000` with alpha `0x4C` (76/255 = 30%), Foreground `#FFFFFF`.
     - `Opaque`: Background `#000000`, Foreground `#FFFFFF`.
2. **Loading Element Line Ratios** (`editor_link_box.cpp:L48-L51`):
   - Title placeholder ratio: $0.45$.
   - Content line width ratios: `[0.32, 0.58, 0.86]`.
   - Glare animation duration: $1000\text{ms}$, timeout: $1000\text{ms}$.
3. **Opaque Bounds Alpha Scanning** (`editor_message_render.cpp:L35-L78`):
   - Computes tightest bounding rectangle $R = [x_{\min}, y_{\min}, w, h]$ containing non-zero alpha pixels by scanning scanlines row-by-row and column-by-column.

### 2.6 Editor Stylesheet & Metric System (`editor.style:L16-L400`)

The GPUI `EditorStyleSheet` embodies all metric tokens from `editor.style`:
- Controls Bar Height: `photoEditorControlsHeight: 146px`
- Bottom Skip: `photoEditorControlsBottomSkip: 20px`, Center Skip: `photoEditorControlsCenterSkip: 6px`
- Button Bar Height: `photoEditorButtonBarHeight: 48px`, Width: `422px`
- Button Padding: Horizontal $2\text{px}$; Text Button Padding: Horizontal $22\text{px}$
- Color Button Size: `24px`, Border: `1px`, Palette Item: `20px`, Gap: `6px`, Selection Width: `2px`
- Tool Button Size: `20px`, Gap: `18px`, Hover Duration: `350ms`, Select Duration: `120ms`
- Brush Control Height: `280px`, Collapsed Width: `6px`, Top Inset: `1px`, Hit Padding: `24px`
- Crop Point Size: `10px`, Min Size: `20px`, Item Handle Size: `10px`, Sticky Trigger: `12px`
- Audio Disc Size: `48px`, Skip: `12px`, Cover Size: `30px`, Ring: `2px`
- Audio Timeline Bar Width: `2px`, Skip: `3px`, Min: `1px`, Ratio: `0.6`, Opacity: `0.4`
- Video Timeline Strip Height: `54px`, Label Height: `18px`, Trim Strip Height: `32px`
- Video Timeline Dot Size: `5px`, Active: `9px`, Radius: `6px`, Handle Width: `11px`
- Video Quality Slider Dot Size: `7px`, Active: `11px`, Line Width: `2px`, Height: `32px`
