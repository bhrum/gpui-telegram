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
