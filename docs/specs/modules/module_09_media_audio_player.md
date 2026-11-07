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
