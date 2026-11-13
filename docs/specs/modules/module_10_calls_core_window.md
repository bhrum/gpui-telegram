# Module 10: Calls — Core, Controller, Window & Panel Specification (Slice 1)

## Executive Summary
This design document establishes the authoritative C++ to Rust/GPUI architectural mapping for **Module 10 (`calls`) Slice 1**, encompassing 34 upstream Telegram Desktop source and style files. The subsystem covers:
- Core 1-on-1 Voice and Video Call State Machine (`Calls::Call`).
- Media Controller abstractions for WebRTC and TgVoip (`Calls::Controller`, `Calls::WebrtcController`, `Calls::TgVoipController`).
- Cryptographic 4-word Emoji Fingerprint generation and carousel animation (`Calls::ComputeEmojiFingerprint`).
- Dynamic Signal Quality Bar calculation and rendering (`Calls::SignalBars`).
- Floating, draggable Picture-in-Picture Video Bubbles (`Calls::VideoBubble`).
- Remote Video Surface Decoders & Rendering Shaders (`Calls::Panel::Incoming`).
- Call Panel, Floating Top Bar, and Window Lifecycle (`Calls::Panel`, `Calls::TopBar`, `Calls::Window`).
- 5-Star Call Rating and Feedback Flow (`Calls::RateCall`, `Calls::EndCloseButton`).
- Call Audio & Video Device Selection Menu (`Calls::MakeDeviceSelectionMenu`).
- Theme tokens and pixel-perfect geometry definitions (`calls.style`).

---

## 100% Upstream Source Reading Manifest (Slice 1: 34 / 34 Files)

| Source Path | Line Range | Core Types & Symbols Extracted | Constants & Formulas | Target Rust / GPUI Path | Target Symbol |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/calls/calls.style` | L1-L1913 | `CallButton`, `CallSignalBars`, `CallBodyLayout`, `callAnswer`, `callHangup` | `callWidth: 720px`, `callHeight: 540px`, `callWidthMin: 380px`, `callHeightMin: 520px`, `callBottomControlsHeight: 87px`, `photoSize: 160px` | `crates/gpui_telegram_desktop/src/theme/calls/calls_style.rs` | `CallsStyleSheet` |
| `Telegram/SourceFiles/calls/calls_box_controller.h` | L1-L88 | `BoxController`, `ListController`, `ClearCallsBox`, `ShowCallsBox` | Dialog controller declarations for calls history and group call lists | `crates/gpui_telegram_webrtc/src/calls/calls_box_controller.rs` | `CallsBoxController` |
| `Telegram/SourceFiles/calls/calls_box_controller.cpp` | L1-L930 | `BoxController::receivedCalls`, `BoxController::loadMoreRows`, `GroupCallRow` | `kFirstPageCount = 20`, `kPerPageCount = 100`, call log pagination | `crates/gpui_telegram_webrtc/src/calls/calls_box_controller.rs` | `CallsBoxController` |
| `Telegram/SourceFiles/calls/calls_call.h` | L1-L407 | `Call`, `Call::Delegate`, `Call::State`, `RemoteAudioState`, `RemoteBatteryState`, `DhConfig` | 17 call states (`Starting`, `WaitingInit`, `WaitingInitAck`, `Established`, `FailedHangingUp`, `Failed`, `HangingUp`, `MigrationHangingUp`, `Ended`, `EndedByOtherDevice`, `ExchangingKeys`, `Waiting`, `Requesting`, `WaitingIncoming`, `Ringing`, `Busy`, `WaitingUserConfirmation`) | `crates/gpui_telegram_webrtc/src/calls/calls_call.rs` | `CallsCall` |
| `Telegram/SourceFiles/calls/calls_call.cpp` | L1-L1723 | `Call::start`, `Call::handleUpdate`, `Call::answer`, `Call::hangup`, `Call::redial` | `kMinLayer = 65`, `kHangupTimeoutMs = 5000`, `kSha256Size = 32`, `kAuthKeySize = 256`, `kDefaultVersion = "2.4.4"`, `kSoundSampleMs = 100` | `crates/gpui_telegram_webrtc/src/calls/calls_call.rs` | `CallsCall` |
| `Telegram/SourceFiles/calls/calls_controller.h` | L1-L59 | `Controller` virtual interface, `MakeController`, `CollectControllerVersions`, `ControllerMaxLayer` | Device setters, gain control, echo cancellation, traffic stats, signaling channel | `crates/gpui_telegram_webrtc/src/calls/calls_controller.rs` | `CallsController` |
| `Telegram/SourceFiles/calls/calls_controller.cpp` | L1-L53 | `MakeController`, `CollectControllerVersions`, `ControllerMaxLayer` | Runtime dispatch between WebRTC and TgVoip controllers | `crates/gpui_telegram_webrtc/src/calls/calls_controller.rs` | `CallsController` |
| `Telegram/SourceFiles/calls/calls_controller_tgvoip.h` | L1-L99 | `TgVoipController` wrapper for legacy TgVoip instance | `_impl->setMuteMicrophone`, `_impl->getTrafficStats`, `_impl->stop` | `crates/gpui_telegram_webrtc/src/calls/calls_controller_tgvoip.rs` | `CallsControllerTgvoip` |
| `Telegram/SourceFiles/calls/calls_controller_webrtc.h` | L1-L60 | `WebrtcController` wrapping `Webrtc::CallContext` | `_impl->receiveSignalingData`, `_impl->state().changes()`, `_stateUpdatedLifetime` | `crates/gpui_telegram_webrtc/src/calls/calls_controller_webrtc.rs` | `CallsControllerWebrtc` |
| `Telegram/SourceFiles/calls/calls_controller_webrtc.cpp` | L1-L175 | `ConvertEndpoint`, `MakeContextConfig`, `WebrtcController::setOnStateUpdated` | Endpoint conversion (IPv4, IPv6, peerTag 16 bytes), state mapping to `TgVoipState` (`Initializing` -> `WaitInit`, `Reconnecting`, `Connected` -> `Established`, `Failed`) | `crates/gpui_telegram_webrtc/src/calls/calls_controller_webrtc.rs` | `CallsControllerWebrtc` |
| `Telegram/SourceFiles/calls/calls_emoji_fingerprint.h` | L1-L72 | `ComputeEmojiFingerprint`, `FingerprintBadgeState`, `CreateFingerprintAndSignalBars` | Badge carousel state, entry animation position and speed | `crates/gpui_telegram_webrtc/src/calls/calls_emoji_fingerprint.rs` | `CallsEmojiFingerprint` |
| `Telegram/SourceFiles/calls/calls_emoji_fingerprint.cpp` | L1-L838 | `ComputeEmojiIndex`, `EmojiByIndex`, `ComputeEmojiFingerprint` | `kTooltipShowTimeoutMs = 1000`, `kCarouselOneDuration = 100`, `kStartTimeShift = 50`, `kEmojiInFingerprint = 4`, `kEmojiInCarousel = 10`, 333 emoji index table (`Offsets` array) | `crates/gpui_telegram_webrtc/src/calls/calls_emoji_fingerprint.rs` | `CallsEmojiFingerprint` |
| `Telegram/SourceFiles/calls/calls_instance.h` | L1-L238 | `Instance`, `StartOutgoingCallArgs`, `StartGroupCallArgs`, `ConferenceInvites` | Single-instance coordinator for private and group calls, conference invite tracking | `crates/gpui_telegram_webrtc/src/calls/calls_instance.rs` | `CallsInstance` |
| `Telegram/SourceFiles/calls/calls_instance.cpp` | L1-L1240 | `Instance::Delegate`, `Instance::startOutgoingCall`, `Instance::handleUpdate` | `kServerConfigUpdateTimeoutMs = 86400000` (24h), sound triggers (`call_busy`, `call_end`, `call_connect`), permission prompts | `crates/gpui_telegram_webrtc/src/calls/calls_instance.rs` | `CallsInstance` |
| `Telegram/SourceFiles/calls/calls_panel.h` | L1-L258 | `Panel`, `Panel::Incoming`, `AnswerHangupRedialState`, `ConferencePanelMigration` | Desktop call window widget, top bar, userpic, buttons layout | `crates/gpui_telegram_webrtc/src/calls/calls_panel.rs` | `CallsPanel` |
| `Telegram/SourceFiles/calls/calls_panel.cpp` | L1-L1949 | `Panel::initLayout`, `Panel::updateControlsGeometry`, `Panel::paint` | `kHideControlsTimeout = 5000`, `kHideControlsQuickTimeout = 2000`, `kNameFadeDuration = 70`, `BatterySvg` generator | `crates/gpui_telegram_webrtc/src/calls/calls_panel.rs` | `CallsPanel` |
| `Telegram/SourceFiles/calls/calls_panel_background.h` | L1-L77 | `PanelBackground`, `Data::ColorProfileSet`, `renderPattern` | Peer-colored background renderer with custom emoji pattern | `crates/gpui_telegram_webrtc/src/calls/calls_panel_background.rs` | `CallsPanelBackground` |
| `Telegram/SourceFiles/calls/calls_panel_background.cpp` | L1-L237 | `PanelBackground::paint`, `PanelBackground::updateBrush` | `kDarkOverlayOpacity = 0.14`, `kDarkenAboveLightness = 0.3`, radial gradient calculation | `crates/gpui_telegram_webrtc/src/calls/calls_panel_background.rs` | `CallsPanelBackground` |
| `Telegram/SourceFiles/calls/calls_rate_call.h` | L1-L116 | `RateCall`, `EndCloseButton`, `RateCall::Star`, `RateCall::Burst` | 5-star rating overlay, star burst animations, switch to close | `crates/gpui_telegram_webrtc/src/calls/calls_rate_call.rs` | `CallsRateCall` |
| `Telegram/SourceFiles/calls/calls_rate_call.cpp` | L1-L528 | `StarPath`, `StarImage`, `RateCall::commit`, `RateCall::startBurst` | `kStarsCount = 5`, `kBurstMinRating = 4`, `kShowDuration = 250`, `kStarShowDelay = 16`, `kSelectDuration = 250`, `kCardShowScaleFrom = 0.7`, `kStarShowScaleFrom = 0.3`, `kStarPressedScale = 0.8`, `kDarkOpacity = 0.14`, `kRippleOpacity = 0.3`, `kPathSize = 96.0`, `kPathStroke = 6.0`, `kPathMiterLimit = 4.0`, `kCloseTextShadeOpacity = 0.15`, `kCloseRippleOpacity = 0.12`, `kMaxBursts = 5`, `kStarPath` bezier table | `crates/gpui_telegram_webrtc/src/calls/calls_rate_call.rs` | `CallsRateCall` |
| `Telegram/SourceFiles/calls/calls_signal_bars.h` | L1-L37 | `SignalBars` widget declaration | Bar count variable binding to `Call::signalBarCountValue` | `crates/gpui_telegram_webrtc/src/calls/calls_signal_bars.rs` | `CallsSignalBars` |
| `Telegram/SourceFiles/calls/calls_signal_bars.cpp` | L1-L65 | `SignalBars::paintEvent`, `SignalBars::changed` | `kSignalBarCount = 4`, `kSignalBarStarting = -1`, `kSignalBarFinished = -2`, `barHeight = min + (max - min) * (i / 3.0)` | `crates/gpui_telegram_webrtc/src/calls/calls_signal_bars.rs` | `CallsSignalBars` |
| `Telegram/SourceFiles/calls/calls_top_bar.h` | L1-L114 | `TopBar`, `BarState` | In-call floating header bar, duration timer, mute state | `crates/gpui_telegram_webrtc/src/calls/calls_top_bar.rs` | `CallsTopBar` |
| `Telegram/SourceFiles/calls/calls_top_bar.cpp` | L1-L837 | `TopBar::paintEvent`, `LinearBlobs`, `BarStateFromMuteState` | `kUpdateDebugTimeoutMs = 500`, `kMinorBlobAlpha = 76./255.`, `kHideBlobsDuration = 500`, `kBlobLevelDuration = 250`, `kBlobUpdateInterval = 100` | `crates/gpui_telegram_webrtc/src/calls/calls_top_bar.rs` | `CallsTopBar` |
| `Telegram/SourceFiles/calls/calls_userpic.h` | L1-L67 | `Userpic` round photo with animated mute badge | Geometry update, photo reload on download finish | `crates/gpui_telegram_webrtc/src/calls/calls_userpic.rs` | `CallsUserpic` |
| `Telegram/SourceFiles/calls/calls_userpic.cpp` | L1-L231 | `Userpic::paint`, `Userpic::refreshPhoto`, `Userpic::setMuted` | Rounded avatar clipping, mute badge positioning (`mutePosition`, `muteSize`, `muteStroke`) | `crates/gpui_telegram_webrtc/src/calls/calls_userpic.rs` | `CallsUserpic` |
| `Telegram/SourceFiles/calls/calls_video_bubble.h` | L1-L71 | `VideoBubble`, `VideoBubble::DragMode` (`None`, `SnapToCorners`) | Picture-in-picture floating camera preview | `crates/gpui_telegram_webrtc/src/calls/calls_video_bubble.rs` | `CallsVideoBubble` |
| `Telegram/SourceFiles/calls/calls_video_bubble.cpp` | L1-L266 | `VideoBubble::updateGeometry`, `VideoBubble::paint` | Dragging mode, corner snapping logic, aspect ratio preservation, frame mirroring | `crates/gpui_telegram_webrtc/src/calls/calls_video_bubble.rs` | `CallsVideoBubble` |
| `Telegram/SourceFiles/calls/calls_video_incoming.h` | L1-L46 | `Panel::Incoming` video surface wrapper | Selection between GL, SW, and Rhi video renderers | `crates/gpui_telegram_webrtc/src/calls/calls_video_incoming.rs` | `CallsVideoIncoming` |
| `Telegram/SourceFiles/calls/calls_video_incoming.cpp` | L1-L1209 | `RendererGL`, `RendererSW`, `RendererRhi`, `FragmentBottomShadow` | `kBottomShadowAlphaMax = 74`, YUV420 to RGB conversion shaders, controls gradient overlay | `crates/gpui_telegram_webrtc/src/calls/calls_video_incoming.rs` | `CallsVideoIncoming` |
| `Telegram/SourceFiles/calls/calls_window.h` | L1-L117 | `Window`, separate title controls, layer manager | Standalone call window host, power save blocker | `crates/gpui_telegram_webrtc/src/calls/calls_window.rs` | `CallsWindow` |
| `Telegram/SourceFiles/calls/calls_window.cpp` | L1-L295 | `Window::setPinnedOnTop`, `Window::togglePowerSaveBlocker` | System sleep prevention while in call, fullscreen / maximize toggle | `crates/gpui_telegram_webrtc/src/calls/calls_window.rs` | `CallsWindow` |
| `Telegram/SourceFiles/calls/ui/calls_device_menu.h` | L1-L36 | `DeviceSelection`, `MakeDeviceSelectionMenu` | Device menu builders for microphone, speaker, and camera | `crates/gpui_telegram_webrtc/src/calls/ui/calls_device_menu.rs` | `CallsDeviceMenuUi` |
| `Telegram/SourceFiles/calls/ui/calls_device_menu.cpp` | L1-L252 | `Subsection`, `Selector`, `AddDeviceSelection` | Device lists with default fallback, radio button group | `crates/gpui_telegram_webrtc/src/calls/ui/calls_device_menu.rs` | `CallsDeviceMenuUi` |

---

## Detailed Architectural Analysis by Cluster

### Cluster 1: Controller & WebRTC Abstraction
The upstream C++ codebase abstracts VOIP protocol implementations behind the `Calls::Controller` interface (`calls_controller.h:L14-L43`). Telegram supports two backends:
1. `Calls::WebrtcController` (`calls_controller_webrtc.h:L18-L58`, `calls_controller_webrtc.cpp:L67-L175`): The modern WebRTC pipeline using `webrtc::CallContext`.
   - Endpoint Conversion (`calls_controller_webrtc.cpp:L17-L27`): Translates MTProto endpoint records (`TgVoipEndpoint`) into WebRTC connection descriptors, parsing IPv4, IPv6, 16-byte peer tags, and ports.
   - Signaling Relay: Handled via `receiveSignalingData` (`calls_controller_webrtc.cpp:L125-L127`) and `sendSignalingData` callbacks.
   - State Bridging: Translates `Webrtc::CallState` (`Initializing`, `Reconnecting`, `Connected`, `Failed`) to `TgVoipState` (`WaitInit`, `Reconnecting`, `Established`, `Failed`) in `calls_controller_webrtc.cpp:L155-L162`.
2. `Calls::TgVoipController` (`calls_controller_tgvoip.h:L14-L97`): Legacy libtgvoip wrapper maintaining compatibility with older protocol layers.
### Cluster 2: Call State Machine & Lifecycle Management
The primary state machine is defined in `Calls::Call` (`calls_call.h:L70-L290`, `calls_call.cpp:L8-L1723`):
- **17 Discrete Call States**:
  - `Starting`: Initial state when dialing or handling incoming call.
  - `WaitingInit`, `WaitingInitAck`: Protocol handshake and parameter negotiation.
  - `ExchangingKeys`: Diffie-Hellman key exchange (`calls_call.h:L155`, `calls_call.cpp:L49-L52`). `kAuthKeySize = 256` bytes, `kSha256Size = 32` bytes.
  - `Waiting`, `Requesting`, `WaitingIncoming`, `Ringing`: Pre-connection signaling phases.
  - `Established`: Media streams flowing; signal bars active; duration timer running.
  - `HangingUp`, `MigrationHangingUp`: Tear-down phases (`kHangupTimeoutMs = 5000` ms).
  - `Ended`, `EndedByOtherDevice`: Normal termination.
  - `FailedHangingUp`, `Failed`: Connection or media failure.
  - `Busy`: Callee returned busy status.
  - `WaitingUserConfirmation`: Callee confirmation modal.
- **Diffie-Hellman Key Exchange**:
  Uses `DhConfig` (`calls_call.h:L46-L50`) with generator `g` and prime `p`. Once the 256-byte key is computed, SHA-256 fingerprint hash is extracted for visual verification.
- **Instance Coordinator** (`calls_instance.h:L83-L170`, `calls_instance.cpp:L59-L200`):
  Ensures only one active 1-on-1 or group call operates at any time (`currentCall`, `currentGroupCall`). Manages audio cues via `callPlaySound`:
  - `call_busy`: Busy tone
  - `call_end`: Call finished chime
  - `call_connect`: Connected indication

### Cluster 3: Security & Verification: Emoji Fingerprint
Telegram verifies end-to-end encryption through a 4-emoji visual fingerprint (`calls_emoji_fingerprint.h:L20-L71`, `calls_emoji_fingerprint.cpp:L25-L168`):
- **Emoji Dictionary**: 333 predefined emojis indexed in `Data` and `Offsets` arrays (`calls_emoji_fingerprint.cpp:L34-L121`, `kEmojiCount = 333`).
- **Algorithm**:
  - Given a 32-byte SHA-256 hash of the DH encryption key (`calls_emoji_fingerprint.cpp:L157-L168`).
  - Splitted into 4 chunks of 8 bytes (`kPartSize = 8`, `kEmojiInFingerprint = 4`).
  - Each 8-byte chunk is converted to a 64-bit integer (`ComputeEmojiIndex`, `calls_emoji_fingerprint.cpp:L123-L134`):
    `((bytes[0] & 0x7F) << 56) | (bytes[1] << 48) | (bytes[2] << 40) | (bytes[3] << 32) | (bytes[4] << 24) | (bytes[5] << 16) | (bytes[6] << 8) | bytes[7]`.
  - The index is modulo reduced: `index = value % 333`.
  - Maps to the corresponding Unicode emoji character.
- **Animation & Carousel**:
  During key exchange, a slot-machine carousel animation rolls through 10 random emojis per slot (`kCarouselOneDuration = 100` ms, `kStartTimeShift = 50` ms) before locking onto the verified emoji.
### Cluster 4: Windows, Panels & Background Rendering
- **Call Window & Controls** (`calls_window.h:L45-L115`, `calls_window.cpp:L25-L295`):
  Standalone window displaying ongoing call media. Integrates `PowerSaveBlocker` (`calls_window.cpp:L270-L285`) to prevent OS sleep during active calls.
- **Call Panel UI** (`calls_panel.h:L75-L155`, `calls_panel.cpp:L79-L250`):
  Houses peer userpic, animated call status, bottom control dock (`callAnswer`, `callHangup`, `callMicrophoneMute`, `callCameraMute`, `callScreencastOn`). Controls automatically fade out after `kHideControlsTimeout = 5000` ms of inactivity.
- **Peer-Themed Radial Gradient** (`calls_panel_background.h:L26-L75`, `calls_panel_background.cpp:L30-L120`):
  Renders a dynamic radial gradient anchored at the center of the peer's avatar (`calls_panel_background.cpp:L82-L95`). Incorporates `kDarkOverlayOpacity = 0.14` overlay and darkens if peer color lightness exceeds `kDarkenAboveLightness = 0.3`.

### Cluster 5: Visual Components, Bars, Video Bubbles & Devices
- **Signal Quality Bars** (`calls_signal_bars.h:L20-L36`, `calls_signal_bars.cpp:L16-L65`):
  4 rounded bars (`kSignalBarCount = 4`). Bar heights scale linearly between `min` (4px) and `max` (10px):
  `barHeight = min + (max - min) * (i / 3.0)`. Active bars have full opacity 1.0; inactive bars render at `inactiveOpacity = 0.5`.
- **Floating Video Bubble** (`calls_video_bubble.h:L20-L70`, `calls_video_bubble.cpp:L17-L120`):
  Self-view video preview floating over incoming video. Supports `SnapToCorners` drag mode, frame mirroring, and aspect ratio preservation.
- **Remote Video Decoders** (`calls_video_incoming.h:L19-L44`, `calls_video_incoming.cpp:L27-L140`):
  Converts incoming YUV420 video frames to RGB texture surfaces with GL and software fallbacks, applying bottom control shadow vignette (`kBottomShadowAlphaMax = 74`).
- **5-Star Rating & Feedback** (`calls_rate_call.h:L25-L94`, `calls_rate_call.cpp:L20-L120`):
  Presents 5 stars using 25-point cubic bezier path `kStarPath` (`calls_rate_call.cpp:L47-L73`). Ratings >= 4 trigger celebratory burst animations (`kBurstMinRating = 4`).
- **Device Selection Menu** (`calls_device_menu.h:L25-L35`, `calls_device_menu.cpp:L24-L250`):
  Categorizes audio input (microphones), audio output (speakers/headphones), and video capture devices (webcams) with system default selection fallbacks.

### Cluster 6: Styling, Layout Metrics & Animation Parameters
Tokens defined in `calls.style` (`calls.style:L17-L200`):
- Window geometry: `callWidth: 720px`, `callHeight: 540px`, `callWidthMin: 380px`, `callHeightMin: 520px`.
- Bottom controls dock: `callBottomControlsHeight: 87px`.
- Body layout metrics: `photoSize: 160px`, `photoTop: 21px`, `nameTop: 221px`, `statusTop: 254px`, `muteSize: 36px`, `mutePosition: (140px, 135px)`.
- Button dimensions: 44px round icon buttons within 68x79px action zones.
