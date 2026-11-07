# Module 12: Platform — macOS Core & TouchBar Architecture Specification (Part 2)

## Executive Summary
This authoritative specification documents **Module 12 (`platform`) Part 2: macOS Core & TouchBar Architecture**, covering exactly 44 upstream Telegram Desktop C++ / Objective-C++ / Header source files (`Telegram/SourceFiles/platform/mac/*` and `Telegram/SourceFiles/platform/*`) and 24 materialized Rust GPUI targets in `crates/gpui_telegram_desktop/src/platform/`.
The subsystem covers:
- **macOS TouchBar Framework & Contextual Item Controllers**: Root TouchBar dispatch (`RootTouchBar`), Audio player touchbar with playback controls and scrub slider (`TouchBarAudioPlayer`), Media viewer overlay touchbar (`MediaViewTouchBar`), Rich text formatting popover with key event dispatch (`TextFormatPopover`), Interactive pinned chats scrubber with unread badges and online indicators (`PinnedDialogsPanel`), and Sticker/Emoji popover scrubber (`StickerEmojiPopover`).
- **macOS Native Notification Architecture**: Dual-engine notification dispatching across modern macOS 10.14+ `UNUserNotificationCenter` (`UNManager`) with inline text replies and category actions, and legacy `NSUserNotificationCenter` (`Manager`) with background clearing thread loops and local sound cache management.
- **Window Overlays & Native Controls**: Frameless window titlebar controls (`MacOverlayWidgetHelper`), traffic lights positioning (Close, Minimize, Zoom), native window movable state toggling, and native window decoration metrics (`PreviewWindowTitle`, `PreviewWindowSystemButton`).
- **macOS System Integration & Gatekeeper Translocation**: Dock menu management (`SetupDockMenu`), system tray status item (`Tray`), Apple quarantine attribute stripping (`com.apple.quarantine`) and untranslocated app relaunch (`CheckTranslocationState`, `RelaunchUntranslocated`).
- **Hardware Security & Platform Intelligence**: WebAuthn passkey registration/authentication (`ASAuthorizationPlatformPublicKeyCredentialProvider`), Touch ID / Secure Enclave (SEP) hardware vault key wrapping (`TouchIdProtection`), Apple Vision text recognition (`VNRecognizeTextRequest`), and Swift system translation provider bridge (`TranslateProvider`).
- **Cross-Platform Abstraction Interfaces**: System geolocation resolution (`ResolveCurrentExactLocation`, `ResolveLocationAddress`), security-scoped file bookmarks (`FileBookmark`), and safe sandboxed launch/URL dispatching with launch fuses (`UnsafeOpenUrl`, `UnsafeLaunch`).

---

## 100% Upstream Source Reading Manifest (Part 2: 44 / 44 Files)

| Source Path | Line Range | Core Types & Symbols Extracted | Constants & Formulas | Target Rust / GPUI Path | Target Symbol |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/platform/mac/native_event_mac.h` | L1-L15 | `PossiblyTextTypingEvent` | `nodiscard` typing filter | `crates/gpui_telegram_desktop/src/platform/mac/native_event_mac.rs` | `NativeEventMacMac` |
| `Telegram/SourceFiles/platform/mac/native_event_mac.mm` | L1-L36 | `PossiblyTextTypingEvent` | `NSEventModifierFlagDeviceIndependentFlagsMask`, `utf16 >= 32 && utf16 != '?'` | `crates/gpui_telegram_desktop/src/platform/mac/native_event_mac.rs` | `NativeEventMacMac` |
| `Telegram/SourceFiles/platform/mac/notifications_manager_mac.h` | L1-L44 | `Manager`, `Private`, `NotificationInfo` | `doShowNativeNotification`, `doClearFromItem`, `doClearFromSession` | `crates/gpui_telegram_desktop/src/platform/mac/notifications_manager_mac.rs` | `NotificationsManagerMacMac` |
| `Telegram/SourceFiles/platform/mac/notifications_manager_mac.mm` | L1-L742 | `NotificationDelegate`, `Manager::Private`, `ClearTask`, `LocalDiskCache` | `kQuerySettingsEachMs = 1000`, `kCacheExpirationWeeks = 5`, `CFPreferencesGetAppBooleanValue` | `crates/gpui_telegram_desktop/src/platform/mac/notifications_manager_mac.rs` | `NotificationsManagerMacMac` |
| `Telegram/SourceFiles/platform/mac/notifications_manager_mac_un.h` | L1-L48 | `UNManager`, `UseUNManager`, `Private` | `doClearFromTopic`, `doClearFromSublist`, `accountNameSeparator` | `crates/gpui_telegram_desktop/src/platform/mac/notifications_manager_mac_un.rs` | `NotificationsManagerMacUnMac` |
| `Telegram/SourceFiles/platform/mac/notifications_manager_mac_un.mm` | L1-L668 | `UserNotificationsDelegate`, `UNManager::Private`, `NotificationParsed`, `InFlightRequest` | `UNNotificationPresentationOptionBanner \| List \| Sound`, `UNTextInputNotificationResponse` | `crates/gpui_telegram_desktop/src/platform/mac/notifications_manager_mac_un.rs` | `NotificationsManagerMacUnMac` |
| `Telegram/SourceFiles/platform/mac/overlay_widget_mac.h` | L1-L61 | `MacOverlayWidgetHelper`, `TitleControl`, `Data` | `skipTitleHitTest`, `beforeShow`, `afterShow`, `setControlsOpacity`, `topNotchSkipValue` | `crates/gpui_telegram_desktop/src/platform/mac/overlay_widget_mac.rs` | `OverlayWidgetMacMac` |
| `Telegram/SourceFiles/platform/mac/overlay_widget_mac.mm` | L1-L350 | `TitleButton`, `MacOverlayWidgetHelper::Data`, `activate`, `resolveNative` | `Qt::WA_MacAlwaysShowToolWindow`, `setMovable:!over`, traffic light hit testing | `crates/gpui_telegram_desktop/src/platform/mac/overlay_widget_mac.rs` | `OverlayWidgetMacMac` |
| `Telegram/SourceFiles/platform/mac/specific_mac.h` | L1-L77 | `AutostartSupported`, `TrayIconSupported`, `HasTouchBar`, `psAppDataPath`, `psCleanup` | `ActivationWindowId = 1`, `psCheckLocalSocket`, `psDownloadPathBookmark` | `crates/gpui_telegram_desktop/src/platform/mac/specific_mac.rs` | `SpecificMacMac` |
| `Telegram/SourceFiles/platform/mac/specific_mac.mm` | L1-L566 | `TranslocationState`, `CheckTranslocationState`, `RemoveQuarantineRecursively`, `RelaunchUntranslocated` | `com.apple.quarantine`, `SecTranslocateIsTranslocatedURL`, `kUntranslocatedArgument` | `crates/gpui_telegram_desktop/src/platform/mac/specific_mac.rs` | `SpecificMacMac` |
| `Telegram/SourceFiles/platform/mac/specific_mac_p.h` | L1-L29 | `objc_handleMediaKeyEvent`, `objc_start`, `objc_finish`, `objc_documentsPath`, `objc_appDataPath` | Media key events, sandbox download path bookmark APIs | `crates/gpui_telegram_desktop/src/platform/mac/specific_mac_p.rs` | `SpecificMacPMac` |
| `Telegram/SourceFiles/platform/mac/specific_mac_p.mm` | L1-L361 | `SetupDockMenu`, `qVisualize`, `ApplicationDelegate`, `objc_debugShowAlert` | `kIgnoreActivationTimeoutMs = 500`, `Menu::RefreshDockMenu`, AppKit version polling | `crates/gpui_telegram_desktop/src/platform/mac/specific_mac_p.rs` | `SpecificMacPMac` |
| `Telegram/SourceFiles/platform/mac/text_recognition_mac.h` | L1-L8 | `Platform::TextRecognition` | Header asset declaration | `crates/gpui_telegram_desktop/src/platform/mac/text_recognition_mac.rs` | `TextRecognitionMacMac` |
| `Telegram/SourceFiles/platform/mac/text_recognition_mac.mm` | L1-L111 | `IsAvailable`, `RecognizeText`, `VNRecognizeTextRequest`, `VNImageRequestHandler` | `VNRequestTextRecognitionLevelAccurate`, `boundingBoxForRange`, `ConvertScale` | `crates/gpui_telegram_desktop/src/platform/mac/text_recognition_mac.rs` | `TextRecognitionMacMac` |
| `Telegram/SourceFiles/platform/mac/touchbar/items/mac_formatter_item.h` | L1-L17 | `TextFormatPopover` | `NSPopoverTouchBarItem` subclass header | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/items/mac_formatter_item.rs` | `MacFormatterItemMacTouchbarItems` |
| `Telegram/SourceFiles/platform/mac/touchbar/items/mac_formatter_item.mm` | L1-L145 | `TextFormatPopover`, `SendKeyEvent` | `kCommandBold = 0x010` .. `kCommandLink = 0x017`, `WidthFromString * 1.4` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/items/mac_formatter_item.rs` | `MacFormatterItemMacTouchbarItems` |
| `Telegram/SourceFiles/platform/mac/touchbar/items/mac_pinned_chats_item.h` | L1-L21 | `PinnedDialogsPanel` | `NSImageView` subclass for pinned chats strip | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/items/mac_pinned_chats_item.rs` | `MacPinnedChatsItemMacTouchbarItems` |
| `Telegram/SourceFiles/platform/mac/touchbar/items/mac_pinned_chats_item.mm` | L1-L874 | `PinnedDialogsPanel`, `Pin`, `State`, `SavedMessagesUserpic`, `UnreadBadge` | `kCircleDiameter = 30`, `kPinnedButtonsSpace = 30`, `kOnlineCircleSize = 8` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/items/mac_pinned_chats_item.rs` | `MacPinnedChatsItemMacTouchbarItems` |
| `Telegram/SourceFiles/platform/mac/touchbar/items/mac_scrubber_item.h` | L1-L22 | `StickerEmojiPopover` | `NSPopoverTouchBarItem<NSTouchBarDelegate>` header | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/items/mac_scrubber_item.rs` | `MacScrubberItemMacTouchbarItems` |
| `Telegram/SourceFiles/platform/mac/touchbar/items/mac_scrubber_item.mm` | L1-L718 | `StickerEmojiPopover`, `PickerScrubberItem`, `ScrubberItemType` | `kIdealIconSize = 36`, `kSegmentIconSize = 25`, `kSegmentSize = 92`, `kMaxStickerSets = 5` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/items/mac_scrubber_item.rs` | `MacScrubberItemMacTouchbarItems` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_audio.h` | L1-L16 | `TouchBarAudioPlayer` | `closeRequests` producer declaration | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_audio.rs` | `MacTouchbarAudioMacTouchbar` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_audio.mm` | L1-L177 | `TouchBarAudioPlayer`, `Format`, `kCustomizationIdPlayer` | `kSeekBarItemIdentifier`, `kPlayItemIdentifier`, `kClosePlayerItemIdentifier` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_audio.rs` | `MacTouchbarAudioMacTouchbar` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_common.h` | L1-L33 | `TouchBar::kCircleDiameter`, `CustomEnterToCocoaEventLoop`, `WidthFromString` | `kCircleDiameter = 30`, `performSelectorOnMainThread` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_common.rs` | `MacTouchbarCommonMacTouchbar` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_common.mm` | L1-L30 | `WidthFromString`, `CreateNSImageFromStyleIcon` | `ceil([[NSTextField labelWithString:s] frame].size.width) * 1.2` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_common.rs` | `MacTouchbarCommonMacTouchbar` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_controls.h` | L1-L65 | `CreateTouchBarButton`, `CreateTouchBarButtonWithTwoStates`, `CreateTouchBarSlider`, `CreateTouchBarTrackPosition` | Button & slider factory helper declarations | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_controls.rs` | `MacTouchbarControlsMacTouchbar` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_controls.mm` | L1-L278 | `CreateTouchBarButton`, `CreateTouchBarSlider`, `CreateTouchBarTrackPosition` | Slider continuous events, format duration text, custom event loop | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_controls.rs` | `MacTouchbarControlsMacTouchbar` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_main.h` | L1-L29 | `TouchBarMain`, `kPinnedPanelItemIdentifier`, `kPopoverInputItemIdentifier`, `kPopoverPickerItemIdentifier` | TouchBarMain class declaration and item IDs | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_main.rs` | `MacTouchbarMainMacTouchbar` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_main.mm` | L1-L54 | `TouchBarMain::init`, `templateItems` | Template items set: PinnedDialogsPanel, StickerEmojiPopover, TextFormatPopover | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_main.rs` | `MacTouchbarMainMacTouchbar` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_manager.h` | L1-L31 | `RootTouchBar`, `MarkdownEnabledState` | `NSTouchBar<NSTouchBarDelegate>` root manager | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_manager.rs` | `MacTouchbarManagerMacTouchbar` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_manager.mm` | L1-L184 | `RootTouchBar`, `GroupTouchBarItem`, `touchBarSwitches` | `kMainItemIdentifier`, `kAudioItemIdentifier`, passcode lock & audio playback triggers | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_manager.rs` | `MacTouchbarManagerMacTouchbar` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_media_view.h` | L1-L24 | `SetupMediaViewTouchBar` | Media view touchbar setup function declaration | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_media_view.rs` | `MacTouchbarMediaViewMacTouchbar` |
| `Telegram/SourceFiles/platform/mac/touchbar/mac_touchbar_media_view.mm` | L1-L225 | `MediaViewTouchBar`, `Delegate`, `ItemType`, `SetupMediaViewTouchBar` | `kPlayItemIdentifier`, `kRotateItemIdentifier`, `kFullscreenItemIdentifier`, `kPipItemIdentifier` | `crates/gpui_telegram_desktop/src/platform/mac/touchbar/mac_touchbar_media_view.rs` | `MacTouchbarMediaViewMacTouchbar` |
| `Telegram/SourceFiles/platform/mac/translate_provider_mac.h` | L1-L19 | `CreateTranslateProvider`, `IsTranslateProviderAvailable` | System translation provider factory | `crates/gpui_telegram_desktop/src/platform/mac/translate_provider_mac.rs` | `TranslateProviderMacMac` |
| `Telegram/SourceFiles/platform/mac/translate_provider_mac.mm` | L1-L119 | `TranslateProvider`, `ParseErrorCode`, `TranslateProviderMacSwiftTranslate` | `local-language-pack-missing`, Swift 6 C-bridge dispatch | `crates/gpui_telegram_desktop/src/platform/mac/translate_provider_mac.rs` | `TranslateProviderMacMac` |
| `Telegram/SourceFiles/platform/mac/tray_mac.h` | L1-L79 | `Tray`, `NativeIcon`, `HasMonochromeSetting` | Tray interface: createIcon, destroyIcon, updateIcon, addAction, showTrayMessage | `crates/gpui_telegram_desktop/src/platform/mac/tray_mac.rs` | `TrayMacMac` |
| `Telegram/SourceFiles/platform/mac/tray_mac.mm` | L1-L499 | `CommonDelegate`, `NativeIcon`, `Tray::Tray`, `TrayIconBack` | `button.effectiveAppearance` KVO observation, status item menu generation | `crates/gpui_telegram_desktop/src/platform/mac/tray_mac.rs` | `TrayMacMac` |
| `Telegram/SourceFiles/platform/mac/wallet_protection_mac.h` | L1-L17 | `RegisterWalletProtectionProvider` | Secure Enclave wallet protection bootstrap | `crates/gpui_telegram_desktop/src/platform/mac/wallet_protection_mac.rs` | `WalletProtectionMacMac` |
| `Telegram/SourceFiles/platform/mac/wallet_protection_mac.mm` | L1-L524 | `Payload`, `UnwrapOperation`, `TouchIdProtection`, `SerializePayload`, `ParsePayload` | `kTokenOIDAttribute = CFSTR("toid")`, `kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA256AESGCM` | `crates/gpui_telegram_desktop/src/platform/mac/wallet_protection_mac.rs` | `WalletProtectionMacMac` |
| `Telegram/SourceFiles/platform/mac/webauthn_mac.mm` | L1-L380 | `WebAuthnDelegate`, `ResolveAnchorWindow`, `ASAuthorizationPlatformPublicKeyCredentialProvider` | Passkey FIDO2 assertion & registration, clientDataJSON & attestationObject | `crates/gpui_telegram_desktop/src/platform/mac/webauthn_mac.rs` | `WebauthnMacMac` |
| `Telegram/SourceFiles/platform/mac/window_title_mac.mm` | L1-L188 | `PreviewTitleHeight`, `PreviewWindowSystemButton`, `PreviewWindowTitle`, `PreviewWindowFramePaint` | Title height = 22, button size = 12, skip = 8, close `(252,96,92)`, minimize `(254,192,65)`, maximize `(52,200,74)` | `crates/gpui_telegram_desktop/src/platform/mac/window_title_mac.rs` | `WindowTitleMacMac` |
| `Telegram/SourceFiles/platform/platform_current_geo_location.h` | L1-L24 | `ResolveCurrentExactLocation`, `ResolveLocationAddress` | Geolocation request callbacks | `crates/gpui_telegram_desktop/src/platform/platform_current_geo_location.rs` | `PlatformCurrentGeoLocation` |
| `Telegram/SourceFiles/platform/platform_file_bookmark.h` | L1-L44 | `FileBookmark`, `PathBookmark` | macOS security-scoped bookmarks & fallback stubs | `crates/gpui_telegram_desktop/src/platform/platform_file_bookmark.rs` | `PlatformFileBookmark` |
| `Telegram/SourceFiles/platform/platform_file_utilities.h` | L1-L63 | `UrlToLocal`, `UnsafeOpenUrl`, `UnsafeOpenEmailLink`, `UnsafeShowOpenWith`, `UnsafeLaunch`, `InitLastPath` | Core platform file launcher declarations | `crates/gpui_telegram_desktop/src/platform/platform_file_utilities.rs` | `PlatformFileUtilities` |
| `Telegram/SourceFiles/platform/platform_file_utilities.cpp` | L1-L50 | `Test::BlockLaunch`, `Unfused::UnsafeOpenUrl`, `Unfused::UnsafeLaunch` | Launch safety fuses and test block dispatchers | `crates/gpui_telegram_desktop/src/platform/platform_file_utilities.rs` | `PlatformFileUtilities` |

---

## Detailed Architectural Analysis by Cluster

### Cluster 1: TouchBar Framework, Controller & Item Component Lifecycle
Telegram Desktop's macOS TouchBar subsystem provides deep hardware-accelerated context switching and custom AppKit input controls on supported MacBook Pro models:
1. **Root TouchBar Dispatcher (`mac_touchbar_manager.h:L24-L31`, `mac_touchbar_manager.mm:L57-L184`)**:
   - `RootTouchBar` acts as an `NSTouchBar<NSTouchBarDelegate>` container responding to real-time application state changes.
   - Combines 4 asynchronous signals:
     - Active session changes (`domain->activeSessionChanges()`)
     - Passcode lock state (`Core::App().passcodeLockChanges()`)
     - Audio player state (`Media::Player::instance()->startsPlay(AudioMsgId::Type::Song)`)
     - Voice message recording state (`Media::Capture::instance()->startedChanges()`)
   - Dispatch Rule:
     - If the app is locked or voice recording is in progress, TouchBar is blanked (`defaultItemIdentifiers = @[]`).
     - If a music track / podcast (`Song`) is actively playing, TouchBar transitions to `kAudioItemIdentifier` (`TouchBarAudioPlayer`).
     - Otherwise, if a session is active, TouchBar transitions to `kMainItemIdentifier` (`TouchBarMain`).
2. **Audio Player TouchBar (`mac_touchbar_audio.h:L12-L16`, `mac_touchbar_audio.mm:L47-L177`)**:
   - `TouchBarAudioPlayer` provides hardware scrubber controls for background music playback.
   - Identifiers: `telegram.touchbar.play`, `previousItem`, `nextItem`, `seekbar`, `currentPosition`, `closePlayer`.
   - Continuous seeking callback: seeking is previewed while dragging and committed on touch-up (`mediaPlayer->finishSeeking(kSongType, value)`).
3. **Media Viewer TouchBar (`mac_touchbar_media_view.h:L15-L22`, `mac_touchbar_media_view.mm:L42-L225`)**:
   - In fullscreen and windowed media view modes, renders video controls:
     - Play/Pause toggle with dual-state icons (`st::touchBarIconPlayerPause`, `st::touchBarIconPlayerPlay`).
     - Rotate counter-clockwise button (`NSImageNameTouchBarRotateLeftTemplate`).
     - Fullscreen toggle button.
     - Picture-in-Picture (PiP) toggle.
     - Video track timeline scrubber and elapsed/total duration label.
4. **Rich Text Formatting Item (`mac_formatter_item.h:L13-L17`, `mac_formatter_item.mm:L34-L144`)**:
   - `TextFormatPopover` is an `NSPopoverTouchBarItem` wrapping an `NSSegmentedControl` within an `NSScrollView`.
   - Dispatches keyboard shortcuts to the focused `QTextEdit`:
     - Bold: `kCommandBold = 0x010` -> `Ctrl + B`
     - Italic: `kCommandItalic = 0x011` -> `Ctrl + I`
     - Underline: `kCommandUnderline = 0x012` -> `Ctrl + U`
     - Strike-through: `kCommandStrikeOut = 0x013` -> `Ctrl + Shift + X`
     - Blockquote: `kCommandBlockquote = 0x014` -> `Ctrl + Shift + .`
     - Monospace: `kCommandMonospace = 0x015` -> `Ctrl + Shift + M`
     - Clear formatting: `kCommandClear = 0x016` -> `Ctrl + Shift + N`
     - Insert link: `kCommandLink = 0x017` -> `Ctrl + K`
   - Segment width sizing: `WidthFromString(string) * 1.4` with uniform row height `kCircleDiameter = 30px`.
5. **Pinned Chats Strip (`mac_pinned_chats_item.h:L16-L21`, `mac_pinned_chats_item.mm:L44-L874`)**:
   - `PinnedDialogsPanel` renders horizontal avatar bubbles (`kCircleDiameter = 30px`, item spacing `kPinnedButtonsSpace = 30px`).
   - Renders Saved Messages (`SavedMessagesUserpic`), Replies (`RepliesMessagesUserpic`), and User/Group avatars.
   - Real-time badges:
     - Unread count badge overlay (`kUnreadBadgeSize = 15px`).
     - Online green dot indicator (`kOnlineCircleSize = 8px`, border stroke `1.5px`).
   - Touch interactions: single tap switches active chat; press & drag reorders pinned chats or scrolls the strip.
6. **Emoji & Stickers Scrubber (`mac_scrubber_item.h:L17-L22`, `mac_scrubber_item.mm:L58-L718`)**:
   - `StickerEmojiPopover` provides high-speed browsing of recent/favorite stickers and emoji categories.
   - Layout metrics: `kIdealIconSize = 36px`, `kSegmentIconSize = 25px`, `kSegmentSize = 92px`, `kMaxStickerSets = 5`.
   - On tap, checks chat permissions (`Data::CanSendAnyOf(rights)`) and sends sticker/emoji directly to the active chat conversation.

### Cluster 2: Native macOS Notifications (UserNotifications vs NSUserNotificationCenter)
Upstream maintains a dual implementation for macOS system notifications:
1. **Modern UserNotifications Framework (`notifications_manager_mac_un.h:L16-L48`, `notifications_manager_mac_un.mm:L49-L668`)**:
   - Utilizes macOS 10.14+ `UNUserNotificationCenter`.
   - Presentation Options: `UNNotificationPresentationOptionBanner | UNNotificationPresentationOptionList | UNNotificationPresentationOptionSound` (macOS 11+) or `UNNotificationPresentationOptionAlert | Sound` (macOS 10.14-10.15).
   - Structured Payloads: userInfo dictionary packs `session`, `peer`, `topic`, `monoforumpeer`, `msgid`, and `manager`.
   - Interactive Notifications:
     - Inline text reply via `UNTextInputNotificationResponse` (`manager->notificationReplied(id, text)`).
     - Category actions for direct `Reply` (`tr::lng_notification_reply`) and `Mark as Read` (`tr::lng_context_mark_read`).
     - Custom sound generation written to `~/Library/Sounds`.
2. **Legacy NSUserNotificationCenter (`notifications_manager_mac.h:L16-L40`, `notifications_manager_mac.mm:L86-L320`)**:
   - Used for macOS versions prior to 10.14 or when `kOptionMacModernNotifications` is disabled.
   - Dedicated Background Clearing Thread: `_clearingThread` processes asynchronous `ClearTask` variants:
     - `ClearFromItem`: Clears notification for a specific message ID.
     - `ClearFromTopic` / `ClearFromSublist`: Clears notifications within a forum topic or saved sublist.
     - `ClearFromHistory`: Clears all notifications for a specific peer conversation.
     - `ClearFromSession`: Clears all notifications for an account session.
     - `ClearAll`: Purges all delivered notifications.
   - Do Not Disturb (DND) Detection: queries `CFPreferencesGetAppBooleanValue` for `doNotDisturb` in `com.apple.notificationcenterui` every `kQuerySettingsEachMs = 1000ms`.

### Cluster 3: Window Overlays, Traffic Light Geometry & Titlebar Styling
Window control and decoration rendering on macOS require custom AppKit bridging to respect macOS window management and aesthetics:
1. **Overlay Widget & Native Titlebar Controls (`overlay_widget_mac.h:L25-L61`, `overlay_widget_mac.mm:L44-L140`)**:
   - `MacOverlayWidgetHelper` orchestrates window decorations for media viewer and custom overlay windows.
   - Traffic lights controls: Close, Minimize, Zoom/Maximize (`Ui::Platform::TitleControl`).
   - Dynamic window movable state: when cursor hovers over traffic light buttons, `[_data->native setMovable:NO]` is invoked to prevent accidental window dragging while clicking system buttons.
   - Hit testing: `skipTitleHitTest(QPoint position)` returns true if coordinate is inside any system button bounding box.
   - Fullscreen handling: configures `Qt::WA_MacAlwaysShowToolWindow` and `Qt::FramelessWindowHint` appropriately across transitions.
2. **Window Title Geometry & System Decoration Metrics (`window_title_mac.mm:L24-L188`)**:
   - Standard macOS titlebar preview height: `PreviewTitleHeight() = 22px`.
   - Traffic lights button diameter: `12px`, spacing skip: `8px`.
   - Exact macOS color metrics:
     - Close: fill `rgb(252, 96, 92)`, border `rgb(222, 64, 59)`.
     - Minimize: fill `rgb(254, 192, 65)`, border `rgb(221, 152, 25)`.
     - Maximize: fill `rgb(52, 200, 74)`, border `rgb(39, 174, 56)`.
     - Graphite appearance: fill `rgb(141, 141, 146)`, border `rgb(104, 104, 109)`.
   - Typography: uses `.AppleSystemUIFont` (13px bold) fallback to `.SF NS Text` or `Helvetica Neue`.

### Cluster 4: macOS System Integration, Dock Menu, Tray & App Translocation Quarantine Handling
1. **Dock Menu Management (`specific_mac_p.h:L16-L29`, `specific_mac_p.mm:L46-L100`)**:
   - `SetupDockMenu()` establishes `setAsDockMenu()` on macOS dock icon.
   - Dynamically populates accounts list, unread messages count, and mute actions on `aboutToShow`.
2. **Menu Bar Status Item Tray (`tray_mac.h:L25-L79`, `tray_mac.mm:L48-L200`)**:
   - Allocates `NSStatusItem` in system menu bar with variable length (`NSVariableStatusItemLength`).
   - Key-Value Observing (KVO) on `button.effectiveAppearance` automatically invalidates and redraws monochrome / dark mode tray icons when macOS theme changes.
   - Provides unread count badge overlay and actions: Show Telegram, Mute notifications, Quit Telegram.
3. **App Translocation & Quarantine Stripping (`specific_mac.mm:L118-L240`)**:
   - Detects macOS Sierra+ Gatekeeper translocation sandbox via `SecTranslocateIsTranslocatedURL` or path containing `/AppTranslocation/`.
   - Identifies original bundle path using `SecTranslocateCreateOriginalPathForURL` or `statfs.f_mntfromname`.
   - Removes `com.apple.quarantine` extended attribute recursively across all bundle files using `removexattr(path, "com.apple.quarantine", XATTR_NOFOLLOW)`.
   - Relaunches untranslocated instance via `NSWorkspace launchApplicationAtURL` with `--untranslocated` argument.
4. **App Activation & Typing Filtering (`native_event_mac.mm:L14-L35`)**:
   - `PossiblyTextTypingEvent(NSEvent *e)` inspects `NSEventTypeKeyDown` events.
   - Strips non-shift modifiers using `flags & ~NSEventModifierFlagShift`.
   - Tests UTF-16 character code points >= 32 (non-control) and != '?' to identify legitimate typing events without activating keyboard shortcuts.

### Cluster 5: Vision Text Recognition, Swift Translation Bridge & Hardware Security
1. **Apple Vision Text Recognition (`text_recognition_mac.mm:L19-L108`)**:
   - Checks macOS 10.15+ availability (`VNRecognizeTextRequest`).
   - Schedulers accurate text recognition (`VNRequestTextRecognitionLevelAccurate`).
   - Projects normalized Vision bounding boxes `(origin.x, 1.0 - origin.y - height, width, height)` into canvas pixel coordinates with `style::ConvertScale`.
   - Extracts character-level glyph bounding boxes via `boundingBoxForRange` on macOS 11+.
2. **Swift 6 Translation Bridge (`translate_provider_mac.mm:L19-L100`)**:
   - Calls system translation framework via C-bridge `TranslateProviderMacSwiftTranslate`.
   - Parses domain errors such as `local-language-pack-missing` (`Ui::TranslateProviderError::LocalLanguagePackMissing`).
3. **Secure Enclave Touch ID Vault Protection (`wallet_protection_mac.mm:L27-L100`)**:
   - Encrypts Telegram TON wallet vault root key using Secure Enclave hardware key (`kSecAttrTokenOID = CFSTR("toid")`).
   - Cryptographic algorithm: `kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA256AESGCM`.
   - LocalAuthentication biometrics prompt (`LAContext evaluatePolicy:LAPolicyDeviceOwnerAuthenticationWithBiometrics`).
4. **WebAuthn FIDO2 Passkeys (`webauthn_mac.mm:L22-L100`)**:
   - Leverages `ASAuthorizationController` and `ASAuthorizationPlatformPublicKeyCredentialProvider` (macOS 12+).
   - Generates FIDO2 registration (`credentialID`, `rawAttestationObject`, `rawClientDataJSON`) and login assertion signatures for passwordless account authentication.

### Cluster 6: Cross-Platform Core Interfaces
1. **Geolocation Resolution (`platform_current_geo_location.h:L17-L22`)**:
   - `ResolveCurrentExactLocation`: retrieves GPS coordinates (`latitude`, `longitude`, `accuracy`).
   - `ResolveLocationAddress`: reverse geocodes coordinates to street, city, country in user's UI language.
2. **Security-Scoped File Bookmarks (`platform_file_bookmark.h:L16-L40`)**:
   - Encapsulates `NSURL bookmarkDataWithOptions` for persistent file system access in macOS sandbox.
   - Methods: `check()`, `enable()`, `disable()`, `bookmark()`, `name()`.
3. **Safe File Utilities & Launch Fuses (`platform_file_utilities.h:L15-L51`, `platform_file_utilities.cpp:L14-L48`)**:
   - Intercepts external URL, mailto, and file launching via `Test::BlockLaunch` fuses to protect against malicious URI injection.
   - Methods: `UnsafeOpenUrl`, `UnsafeOpenEmailLink`, `UnsafeShowOpenWithDropdown`, `UnsafeShowOpenWith`, `UnsafeLaunch`, `PostprocessDownloaded`.

---

## 4-Layer Anti-Bluffing Proof (Mandate #18)

### Layer 1: Verbatim Upstream Citations & Numerical Constants
- **TouchBar Metrics & IDs**:
  - `mac_touchbar_common.h:L15`: `constexpr auto kCircleDiameter = 30;`
  - `mac_touchbar_common.mm:L17-L19`: `return (int)ceil([[NSTextField labelWithString:s] frame].size.width) * 1.2;`
  - `mac_touchbar_items/mac_pinned_chats_item.mm:L44-L49`: `kPinnedButtonsSpace = 30; kPinnedButtonsLeftSkip = 15; kOnlineCircleSize = 8; kOnlineCircleStrokeWidth = 1.5; kUnreadBadgeSize = 15;`
  - `mac_touchbar_items/mac_scrubber_item.mm:L59-L63`: `kIdealIconSize = 36; kSegmentIconSize = 25; kSegmentSize = 92; kMaxStickerSets = 5;`
  - `mac_touchbar_items/mac_formatter_item.mm:L23-L30`: `kCommandBold = 0x010; kCommandItalic = 0x011; kCommandUnderline = 0x012; kCommandStrikeOut = 0x013; kCommandBlockquote = 0x014; kCommandMonospace = 0x015; kCommandClear = 0x016; kCommandLink = 0x017;`
- **Notification Timeouts & Keys**:
  - `notifications_manager_mac.mm:L38-L40`: `constexpr auto kQuerySettingsEachMs = crl::time(1000); constexpr auto kCacheExpirationWeeks = 5; constexpr auto kCacheExpirationSeconds = kCacheExpirationWeeks * 7 * 24 * 60 * 60;`
  - `notifications_manager_mac.mm:L42-L44`: `kTelegramMarkAsReadText = @"TelegramMarkAsReadText"; kTelegramMarkAsReadTimestamp = @"TelegramMarkAsReadTimestamp";`
  - `notifications_manager_mac_un.mm:L138-L144`: `UNNotificationPresentationOptionBanner | UNNotificationPresentationOptionList | UNNotificationPresentationOptionSound`
- **Window Decorations & System Buttons**:
  - `window_title_mac.mm:L30`: `return 22;` (macOS standard preview title height)
  - `window_title_mac.mm:L34`: `auto buttonSize = 12; auto buttonSkip = 8;`
  - `window_title_mac.mm:L96-L100`: Close: `QColor(252, 96, 92)` / `(222, 64, 59)`; Minimize: `QColor(254, 192, 65)` / `(221, 152, 25)`; Maximize: `QColor(52, 200, 74)` / `(39, 174, 56)`.
- **Translocation & Crypto Tokens**:
  - `specific_mac.mm:L184`: `constexpr auto kAttribute = "com.apple.quarantine";`
  - `wallet_protection_mac.mm:L34-L37`: `kTokenOIDAttribute = CFSTR("toid"); kAlgorithm = kSecKeyAlgorithmECIESEncryptionCofactorVariableIVX963SHA256AESGCM;`

### Layer 2: Reviewer Direct Inspection Anchor Points
1. **TouchBar Layout Engine Verification**:
   - Verify that `RootTouchBar` correctly maps playback state `startsPlay(AudioMsgId::Type::Song)` to `kAudioItemIdentifier` and locks to `@[]` (`mac_touchbar_manager.mm:L108-L118`).
   - Verify that text formatting segmented buttons map directly to control key combinations (`SendKeyEvent` in `mac_formatter_item.mm:L41-L77`).
2. **Notification Dual Engine Routing**:
   - Verify that `UseUNManager()` checks `base::options::lookup<bool>(kOptionMacModernNotifications)` and checks bundle identifier before selecting `UNManager` over legacy `Manager` (`notifications_manager_mac_un.mm:L211-L219`).
   - Verify that `UserNotificationsDelegate` unpacks `session`, `peer`, `topic`, `monoforumpeer`, and `msgid` (`notifications_manager_mac_un.mm:L50-L70`).
3. **App Translocation Quarantine Stripping**:
   - Verify that `RemoveQuarantineRecursively` navigates `QDirIterator::Subdirectories` and invokes `removexattr(path, "com.apple.quarantine", XATTR_NOFOLLOW)` (`specific_mac.mm:L183-L220`).

### Layer 3: Challenger Oracle Equivalence Invariants
- **Typing Event Filter Invariant**: Any key down event with `NSEventModifierFlagShift` and ASCII char >= 32 (e.g. 'A') evaluates to `true`; any event with Command or Alt modifier or control character < 32 evaluates to `false`.
- **TouchBar Formatter Invariant**: Index `0` generates Bold (`0x010`), Index `1` generates Italic (`0x011`), ..., Index `7` generates Link (`0x017`).
- **Notification Task Invariant**: `ClearFromItem` targets single message, `ClearFromHistory` targets conversation, `ClearAll` flushes all pending and delivered notifications.
- **Window Movable Invariant**: Hovering over traffic light buttons sets `movable = false`; moving outside sets `movable = true`.

### Layer 4: Forensic Auditor Traceability Matrix
| Target Module | Upstream Files Citations | Core Responsibilities | Verification Test Anchor |
| :--- | :--- | :--- | :--- |
| `native_event_mac.rs` | `native_event_mac.{h,mm}` | Keyboard event typing filter | Test typing event filter |
| `notifications_manager_mac.rs` | `notifications_manager_mac.{h,mm}` | Legacy notification center & clear tasks | Test clearing tasks & sound cache |
| `notifications_manager_mac_un.rs` | `notifications_manager_mac_un.{h,mm}` | Modern UNUserNotificationCenter | Test UN payloads & category actions |
| `overlay_widget_mac.rs` | `overlay_widget_mac.{h,mm}` | Overlay window traffic light controls | Test traffic lights & movable state |
| `specific_mac.rs` | `specific_mac.{h,mm}` | Translocation quarantine stripping | Test translocation & bundle detection |
| `specific_mac_p.rs` | `specific_mac_p.{h,mm}` | Dock menu & app activation | Test dock menu & paths |
| `text_recognition_mac.rs` | `text_recognition_mac.{h,mm}` | Apple Vision text recognition | Test OCR result parsing & bounding boxes |
| `mac_formatter_item.rs` | `items/mac_formatter_item.{h,mm}` | TouchBar formatting popover | Test formatting command key codes |
| `mac_pinned_chats_item.rs` | `items/mac_pinned_chats_item.{h,mm}` | TouchBar pinned dialogs strip | Test pinned chats layout & badges |
| `mac_scrubber_item.rs` | `items/mac_scrubber_item.{h,mm}` | TouchBar emoji & sticker scrubber | Test scrubber metrics & category items |
| `mac_touchbar_audio.rs` | `mac_touchbar_audio.{h,mm}` | TouchBar audio player controls | Test seekbar & audio playback IDs |
| `mac_touchbar_common.rs` | `mac_touchbar_common.{h,mm}` | TouchBar metrics & style helpers | Test circle diameter & width calculation |
| `mac_touchbar_controls.rs` | `mac_touchbar_controls.{h,mm}` | TouchBar button & slider builders | Test control state changes |
| `mac_touchbar_main.rs` | `mac_touchbar_main.{h,mm}` | TouchBar main item collection | Test template items composition |
| `mac_touchbar_manager.rs` | `mac_touchbar_manager.{h,mm}` | Root TouchBar state switching | Test audio vs main vs locked states |
| `mac_touchbar_media_view.rs` | `mac_touchbar_media_view.{h,mm}` | TouchBar media viewer overlay | Test media view play/rotate/pip items |
| `translate_provider_mac.rs` | `translate_provider_mac.{h,mm}` | macOS system translation bridge | Test translation requests & errors |
| `tray_mac.rs` | `tray_mac.{h,mm}` | Status item menu bar tray | Test tray actions & effectiveAppearance |
| `wallet_protection_mac.rs` | `wallet_protection_mac.{h,mm}` | Secure Enclave Touch ID protection | Test SEP token OID & payload serialization |
| `webauthn_mac.rs` | `webauthn_mac.mm` | WebAuthn passkey assertion & registration | Test passkey assertion & credential parsing |
| `window_title_mac.rs` | `window_title_mac.mm` | macOS titlebar preview & system buttons | Test preview title height & button colors |
| `platform_current_geo_location.rs` | `platform_current_geo_location.h` | Geolocation coordinates & address resolution | Test GPS coordinates & address structure |
| `platform_file_bookmark.rs` | `platform_file_bookmark.h` | Security-scoped file bookmarks | Test bookmark persistence & access enable |
| `platform_file_utilities.rs` | `platform_file_utilities.{h,cpp}` | Safe launch & open with fuses | Test launch fuses & URL verification |

---

## Zero-Omission Reconciliation Checklist
- [x] All 44 assigned upstream files in `platform.json` Part 2 analyzed and documented.
- [x] All 24 target files in `crates/gpui_telegram_desktop/src/platform/` mapped with exact symbols.
- [x] 4-Layer Anti-Bluffing Proof complete with verbatim constants and code lines.
- [x] Zero omitted files: Gate 6 verification ready (`omitted_from_design_doc == 0`).
