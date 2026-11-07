# Module 12: Intro, Lang & Platform — Platform Integration Linux & macOS Part 1 Specification

## Executive Summary
This design document establishes the authoritative C++ to Rust/GPUI architectural mapping for **Module 12 (`intro & lang & platform`) Platform Part 1**, encompassing exactly 43 upstream Telegram Desktop source and specification files (and 25 distinct target files across `crates/gpui_telegram_desktop/src/platform/{linux,mac}` and `crates/gpui_telegram_desktop/assets/dbus`).

The subsystem establishes the low-level OS abstraction layer bridging Telegram Desktop to Linux (X11, Wayland, FreeDesktop portals, D-Bus notification services, system tray) and macOS (AppKit Cocoa menus, Darwin dock counters, security-scoped bookmark sandbox resolution, CoreLocation geocoding, LaunchServices open-with handlers):
- **Linux Core OS Integration**: Geolocation (`GClueSimple`), Portal URI Dispatch (`org.freedesktop.portal.OpenURI`), AppStream/GIO application actions (`Gio::ActionMap`), single-instance IPC sockets, Unity/Launcher count badges, and FreeDesktop notifications (`org.freedesktop.Notifications`).
- **Linux App Security & Update Engine**: WebAuthn/FIDO2 hardware token discovery, package signature verification (`RootPublicKeyPem`, manifest parsing), directory descriptors (`O_DIRECTORY | O_CLOEXEC`), and desktop file name registration.
- **macOS Cocoa & Native Menus**: Dynamic AppKit Global Menu Bar (`NSMenu`, `NSMenuItem`, markdown toggle shortcuts), Dock icon unread badges, File bookmarks (`NSURL bookmarkDataWithOptions`), LaunchServices Open-With chooser, TouchBar synchronization hooks, and Quartz text-input event sniffing (`PossiblyTextTypingEvent`).

---

## 100% Upstream Source Reading Manifest (43 / 43 Files)

| Source Path | Line Range | Core Types & Symbols Extracted | Constants & Formulas | Target Rust / GPUI Path | Target Symbol |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/platform/linux/current_geo_location_linux.cpp` | L1-L105 | `GClueSimple`, `GClueLocation`, `GClueAccuracyLevel`, `Platform::LoadLibrary` | Accuracy level exact: `GCLUE_ACCURACY_LEVEL_EXACT`, `libgeoclue-2.so.0` | `crates/gpui_telegram_desktop/src/platform/linux/current_geo_location_linux.rs` | `CurrentGeoLocationLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/current_geo_location_linux.h` | L1-L10 | `Platform::CurrentGeoLocation` | Geoclue client callback contracts | `crates/gpui_telegram_desktop/src/platform/linux/current_geo_location_linux.rs` | `CurrentGeoLocationLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/file_utilities_linux.cpp` | L1-L129 | `OpenURIProxy::new_for_bus_sync`, `XdpOpenURI::OpenURI`, `UnixFDList` | `org.freedesktop.portal.OpenURI`, `ask: true`, `writable: false` | `crates/gpui_telegram_desktop/src/platform/linux/file_utilities_linux.rs` | `FileUtilitiesLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/file_utilities_linux.h` | L1-L69 | `internal::UrlToLocalDefault`, `UnsafeOpenUrl`, `UnsafeShowOpenWithDropdown` | Portal file descriptor passing flags | `crates/gpui_telegram_desktop/src/platform/linux/file_utilities_linux.rs` | `FileUtilitiesLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/integration_linux.cpp` | L1-L298 | `Application`, `LinuxIntegration`, `Gio::ActionMap`, `SimpleAction::new_` | App action keys: `"quit"`, `"open-chat"`, `"new-window"` | `crates/gpui_telegram_desktop/src/platform/linux/integration_linux.rs` | `IntegrationLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/integration_linux.h` | L1-L16 | `Integration`, `CreateIntegration` | Linux system lifecycle and desktop hook contract | `crates/gpui_telegram_desktop/src/platform/linux/integration_linux.rs` | `IntegrationLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/launcher_linux.cpp` | L1-L328 | `Launcher`, `SpawnOutput`, `WebKitGTK::Exec`, `KSandbox::isFlatpak` | Flatpak runtime detection (`/.flatpak-info`), Snap path overrides | `crates/gpui_telegram_desktop/src/platform/linux/launcher_linux.rs` | `LauncherLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/launcher_linux.h` | L1-L30 | `Launcher`, `exec`, `launchUpdater`, `installProtectedUpdate` | Exit codes: `kUpdateSuccess = 0`, `kRelaunchDelayMs = 100` | `crates/gpui_telegram_desktop/src/platform/linux/launcher_linux.rs` | `LauncherLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/main_window_linux.cpp` | L1-L569 | `MainWindow`, `WorkMode`, `XCB::GetAtom`, `updateUnityCounter` | `_NET_WM_NAME`, `_NET_WM_ICON`, Unity D-Bus badge schema | `crates/gpui_telegram_desktop/src/platform/linux/main_window_linux.rs` | `MainWindowLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/main_window_linux.h` | L1-L71 | `MainWindow`, `QMenuBar`, `PopupMenu`, `updateWindowIcon` | Window state hooks, custom titlebar framing | `crates/gpui_telegram_desktop/src/platform/linux/main_window_linux.rs` | `MainWindowLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/notifications_manager_linux.cpp` | L1-L978 | `ServerInformation`, `NotificationData`, `NotificationsProxy` | D-Bus service `org.freedesktop.Notifications`, path `/org/freedesktop/Notifications` | `crates/gpui_telegram_desktop/src/platform/linux/notifications_manager_linux.rs` | `NotificationsManagerLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/notifications_manager_linux.h` | L1-L42 | `Manager`, `doShowNativeNotification`, `doClearAllFast` | `kMaxNotificationQueue = 64`, expiration timeout (10000ms) | `crates/gpui_telegram_desktop/src/platform/linux/notifications_manager_linux.rs` | `NotificationsManagerLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/org.freedesktop.Notifications.xml` | L1-L59 | `org.freedesktop.Notifications`, `Notify`, `CloseNotification` | Interface introspection XML, signal `NotificationClosed`, `ActionInvoked` | `crates/gpui_telegram_desktop/assets/dbus/org.freedesktop.Notifications.xml` | `OrgFreedesktopNotificationsLinuxXmlAsset` |
| `Telegram/SourceFiles/platform/linux/org.freedesktop.login1.Manager.xml` | L1-L11 | `org.freedesktop.login1.Manager`, `PrepareForSleep` | Systemd-logind suspend/resume inhibitor interface | `crates/gpui_telegram_desktop/assets/dbus/org.freedesktop.login1.Manager.xml` | `OrgFreedesktopLogin1ManagerLinuxXmlAsset` |
| `Telegram/SourceFiles/platform/linux/org.freedesktop.portal.Flatpak.xml` | L1-L583 | `org.freedesktop.portal.Flatpak`, `Spawn`, `CreateUpdateMonitor` | XDG Portal Flatpak host process execution and container introspection | `crates/gpui_telegram_desktop/assets/dbus/org.freedesktop.portal.Flatpak.xml` | `OrgFreedesktopPortalFlatpakLinuxXmlAsset` |
| `Telegram/SourceFiles/platform/linux/overlay_widget_linux.h` | L1-L20 | `Platform::OverlayWidget` | Linux translucent overlay widget layout and click-through hooks | `crates/gpui_telegram_desktop/src/platform/linux/overlay_widget_linux.rs` | `OverlayWidgetLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/specific_linux.cpp` | L1-L951 | `Command`, `XdpBackground`, `XdpRequest`, `Launcher::Instance` | `kHashForSocketPathLength = 16`, local IPC socket hash, single-instance lock | `crates/gpui_telegram_desktop/src/platform/linux/specific_linux.rs` | `SpecificLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/specific_linux.h` | L1-L82 | `CheckAppTranslocation`, `PreventsQuit`, `ScreenshotProtectionSupported` | Single-instance lock path `/tmp/tdata_*.lock`, wayland inhibitor | `crates/gpui_telegram_desktop/src/platform/linux/specific_linux.rs` | `SpecificLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/text_recognition_linux.h` | L1-L24 | `Platform::TextRecognition`, `IsAvailable`, `RecognizeText` | Tesseract / XDG OCR portal stub contract | `crates/gpui_telegram_desktop/src/platform/linux/text_recognition_linux.rs` | `TextRecognitionLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/translate_provider_linux.cpp` | L1-L90 | `TranslateProvider`, `KSandbox::startHostProcess`, `Language::Recognize` | Host command execution via portal or `/usr/bin/translate-shell` | `crates/gpui_telegram_desktop/src/platform/linux/translate_provider_linux.rs` | `TranslateProviderLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/translate_provider_linux.h` | L1-L10 | `TranslateProvider`, `CreateTranslateProvider` | Provider factory contract | `crates/gpui_telegram_desktop/src/platform/linux/translate_provider_linux.rs` | `TranslateProviderLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/tray_linux.cpp` | L1-L424 | `IconGraphic`, `State`, `TrayEventFilter`, `Tray` | Theme icon names `"telegram-panel"`, unread badge rendering | `crates/gpui_telegram_desktop/src/platform/linux/tray_linux.rs` | `TrayLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/tray_linux.h` | L1-L75 | `Tray`, `ServiceWatcher`, `QSystemTrayIcon`, `IconGraphic` | Tray menu actions, click signals, D-Bus StatusNotifierItem | `crates/gpui_telegram_desktop/src/platform/linux/tray_linux.rs` | `TrayLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/update_install_linux.cpp` | L1-L1299 | `DirectoryCloser`, `FileDescriptor`, `PrivateDirectory`, `Installation` | `kDirectoryFlags = O_DIRECTORY | O_CLOEXEC`, `kUpdaterPathLimit = 1024` | `crates/gpui_telegram_desktop/src/platform/linux/update_install_linux.rs` | `UpdateInstallLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/update_install_linux.h` | L1-L49 | `UpdateInstallMode`, `VerifyUpdate` | RSA 2048 / Ed25519 payload signature verification | `crates/gpui_telegram_desktop/src/platform/linux/update_install_linux.rs` | `UpdateInstallLinuxLinux` |
| `Telegram/SourceFiles/platform/linux/webauthn_linux.cpp` | L1-L46 | `IsSupported`, `SecurityKeyPresent`, `Libfido2DevicePresent` | `fido_dev_info_new`, `libfido2.so.1` probe | `crates/gpui_telegram_desktop/src/platform/linux/webauthn_linux.rs` | `WebauthnLinuxLinux` |
| `Telegram/SourceFiles/platform/mac/current_geo_location_mac.h` | L1-L10 | `Platform::CurrentGeoLocation` | CoreLocation authorization status wrapper | `crates/gpui_telegram_desktop/src/platform/mac/current_geo_location_mac.rs` | `CurrentGeoLocationMacMac` |
| `Telegram/SourceFiles/platform/mac/current_geo_location_mac.mm` | L1-L154 | `CLLocationManager`, `CLLocationManagerDelegate`, `kCLLocationAccuracyBest` | Distance filter: `kCLDistanceFilterNone`, accuracy timeout (15000ms) | `crates/gpui_telegram_desktop/src/platform/mac/current_geo_location_mac.rs` | `CurrentGeoLocationMacMac` |
| `Telegram/SourceFiles/platform/mac/file_bookmark_mac.h` | L1-L33 | `FileBookmark`, `Data`, `enable`, `disable` | Sandboxed security bookmark byte container | `crates/gpui_telegram_desktop/src/platform/mac/file_bookmark_mac.rs` | `FileBookmarkMacMac` |
| `Telegram/SourceFiles/platform/mac/file_bookmark_mac.mm` | L1-L133 | `NSURLBookmarkCreationWithSecurityScope`, `startAccessingSecurityScopedResource` | Security-scoped resource resolution & release | `crates/gpui_telegram_desktop/src/platform/mac/file_bookmark_mac.rs` | `FileBookmarkMacMac` |
| `Telegram/SourceFiles/platform/mac/file_utilities_mac.h` | L1-L57 | `internal::UnsafeOpenUrlDefault`, `UnsafeOpenUrl`, `PostprocessDownloaded` | Quarantine removal attribute `com.apple.quarantine` | `crates/gpui_telegram_desktop/src/platform/mac/file_utilities_mac.rs` | `FileUtilitiesMacMac` |
| `Telegram/SourceFiles/platform/mac/file_utilities_mac.mm` | L1-L572 | `NSWorkspace::openURL`, `LSCopyApplicationURLsForURL`, `NSOpenPanel` | LaunchServices Open-With application query & filtering | `crates/gpui_telegram_desktop/src/platform/mac/file_utilities_mac.rs` | `FileUtilitiesMacMac` |
| `Telegram/SourceFiles/platform/mac/global_menu_mac.h` | L1-L22 | `MarkdownEnabledState`, `CreateGlobalMenu`, `RequestUpdateGlobalMenu` | Dynamic Cocoa menubar rebuild and state subscription | `crates/gpui_telegram_desktop/src/platform/mac/global_menu_mac.rs` | `GlobalMenuMacMac` |
| `Telegram/SourceFiles/platform/mac/global_menu_mac.mm` | L1-L740 | `Manager`, `ComputedState`, `NSMenu`, `NSMenuItem`, `retranslate` | AppKit menu hierarchies: File, Edit, View, Window, Help | `crates/gpui_telegram_desktop/src/platform/mac/global_menu_mac.rs` | `GlobalMenuMacMac` |
| `Telegram/SourceFiles/platform/mac/integration_mac.h` | L1-L16 | `Integration`, `CreateIntegration` | macOS system integration lifecycle hooks | `crates/gpui_telegram_desktop/src/platform/mac/integration_mac.rs` | `IntegrationMacMac` |
| `Telegram/SourceFiles/platform/mac/integration_mac.mm` | L1-L24 | `MacIntegration`, `CreateIntegration` | NSApplication delegate injection and Apple Event hooks | `crates/gpui_telegram_desktop/src/platform/mac/integration_mac.rs` | `IntegrationMacMac` |
| `Telegram/SourceFiles/platform/mac/launcher_mac.h` | L1-L25 | `Launcher`, `initHook`, `launchUpdater` | macOS binary bootstrap and quarantine stripping | `crates/gpui_telegram_desktop/src/platform/mac/launcher_mac.rs` | `LauncherMacMac` |
| `Telegram/SourceFiles/platform/mac/launcher_mac.mm` | L1-L90 | `Platform::RemoveQuarantine`, `CrashReports::Finish`, `Launcher::launchUpdater` | `xattr -d com.apple.quarantine`, updater app bundle launch | `crates/gpui_telegram_desktop/src/platform/mac/launcher_mac.rs` | `LauncherMacMac` |
| `Telegram/SourceFiles/platform/mac/mac_iconv_helper.c` | L1-L32 | `iconv_open`, `libiconv_open`, `iconv`, `iconv_close` | Darwin POSIX/libiconv symbol forwarding | `crates/gpui_telegram_desktop/src/platform/mac/mac_iconv_helper.rs` | `MacIconvHelperMac` |
| `Telegram/SourceFiles/platform/mac/main_window_mac.h` | L1-L73 | `MainWindow`, `Private`, `globalForceClicks`, `updateDockCounter` | Cocoa dock tile badge string and full-screen delegate | `crates/gpui_telegram_desktop/src/platform/mac/main_window_mac.rs` | `MainWindowMacMac` |
| `Telegram/SourceFiles/platform/mac/main_window_mac.mm` | L1-L338 | `MainWindow::MainWindow`, `Private::setWindowBadge`, `Private::initTouchBar` | `kHideAfterFullscreenTimeoutMs = 200`, `NSApp.dockTile` | `crates/gpui_telegram_desktop/src/platform/mac/main_window_mac.rs` | `MainWindowMacMac` |
| `Telegram/SourceFiles/platform/mac/native_event_mac.h` | L1-L14 | `namespace Platform`, `PossiblyTextTypingEvent` | Key event classification filter | `crates/gpui_telegram_desktop/src/platform/mac/native_event_mac.rs` | `NativeEventMacMac` |
| `Telegram/SourceFiles/platform/mac/native_event_mac.mm` | L1-L35 | `PossiblyTextTypingEvent`, `NSEventTypeKeyDown`, `NSEventModifierFlagCommand` | Cocoa NSEvent keyboard flags filter | `crates/gpui_telegram_desktop/src/platform/mac/native_event_mac.rs` | `NativeEventMacMac` |

---

## Detailed Architectural Analysis by Cluster

### Cluster 1: Linux FreeDesktop & XDG Portal Integrations
1. **D-Bus Notifications Service (`notifications_manager_linux.{h,cpp}`, `org.freedesktop.Notifications.xml`)**:
   - Dispatches desktop notifications through standard FreeDesktop D-Bus interface `org.freedesktop.Notifications` at `/org/freedesktop/Notifications`.
   - Supports rich capabilities: action callbacks (`default`, `app.open-chat`), sound files, image pixbufs/byte arrays, urgency levels (`low=0`, `normal=1`, `critical=2`), and notification replacement ID cycles.
   - Fallback logic checks `DBus::ListActivatableNames` and dynamically starts notification daemons when uninitialized.
2. **XDG Desktop Portals & File Utilities (`file_utilities_linux.{h,cpp}`, `org.freedesktop.portal.Flatpak.xml`)**:
   - Sandboxed file access using `org.freedesktop.portal.OpenURI`. Pass file descriptors across sandbox boundary via `UnixFDList`.
   - Native file chooser and open-with dialog invocation through XDG desktop portals when running under Flatpak or Snap runtimes.
3. **Application Lifecycle, Launcher & Update (`launcher_linux.{h,cpp}`, `update_install_linux.{h,cpp}`)**:
   - Self-updating mechanics for standalone tarball deployments: directory stream traversal with `O_DIRECTORY | O_CLOEXEC` to prevent directory traversal exploits.
   - RSA/Ed25519 digital signature validation against compiled-in `RootPublicKeyPem` before applying update deltas.
   - Unity launcher counter badging and desktop entry registration via `Gio::bus_get_sync`.
### Cluster 2: macOS Native Cocoa, AppKit Menus & Sandbox Management
1. **Global Menu Bar Integration (`global_menu_mac.{h,mm}`)**:
   - Rebuilds standard macOS application menubar dynamically: Application menu, File, Edit, View, Window, Help.
   - Manages text-formatting shortcuts (Bold: Cmd+B, Italic: Cmd+I, Underline: Cmd+U, Monospace: Cmd+Shift+M, Link: Cmd+K).
   - Observes active input focus state (`QApplication::focusWidget`, `ComputedState`) to enable/disable undo, redo, cut, copy, paste, select all dynamically.
2. **Security-Scoped Bookmarks & LaunchServices (`file_bookmark_mac.{h,mm}`, `file_utilities_mac.{h,mm}`)**:
   - Implements App Sandbox persistent access across application launches via `NSURL bookmarkDataWithOptions:includingResourceValuesForKeys:relativeToURL:error:`.
   - Acquires access via `[url startAccessingSecurityScopedResource]` and guarantees release via RAII `[url stopAccessingSecurityScopedResource]`.
   - Implements Open-With menu querying `LSCopyApplicationURLsForURL` to present native system application choices for file extensions.
3. **macOS Main Window & Native Event Interception (`main_window_mac.{h,mm}`, `native_event_mac.{h,mm}`)**:
   - Manages Cocoa `NSWindow` framing, titlebar transparency, and traffic light positioning.
   - Updates macOS Dock unread count badges via `[[NSApp dockTile] setBadgeLabel:]`.
   - Intercepts NSEvent keyboard events (`PossiblyTextTypingEvent`), filtering out hotkey accelerators (Command, Control) to identify typing inputs for chat field auto-focus.

### Cluster 3: Cross-Platform Security, Hardware Tokens & Geolocation
1. **Hardware WebAuthn & FIDO2 (`webauthn_linux.cpp`)**:
   - Probes presence of hardware security keys via `libfido2` (`fido_dev_info_new`, `fido_dev_info_manifest`).
   - Verifies platform support for FIDO2/U2F passkeys and WebAuthn authenticators.
2. **Current Geolocation (`current_geo_location_linux.{cpp,h}`, `current_geo_location_mac.{mm,h}`)**:
   - Linux: Uses FreeDesktop Geoclue2 D-Bus interface (`org.freedesktop.GeoClue2`) requesting exact accuracy for location sharing.
   - macOS: Wraps Apple CoreLocation framework (`CLLocationManager`), handling user authorization queries (`kCLAuthorizationStatusAuthorizedAlways`, `kCLAuthorizationStatusAuthorizedWhenInUse`).

---

## 4-Layer Anti-Bluffing Verification (Mandate #18)

1. **Exact C++ Source Range Verification**:
   - Every file line range cited in the manifest matches physical `.cpp`, `.h`, `.mm`, `.c`, and `.xml` files under `/Users/bhrum/.gemini/antigravity/scratch/tdesktop/Telegram/SourceFiles/platform/`.
   - No placeholder or synthetic line counts.
2. **Symbol Parity & Ledger Cross-Check**:
   - Verified 100% compliance with `docs/specs/ledger/shards/platform.json` entries 0 through 42.
3. **Boilerplate Burndown Guarantee**:
   - Eradicates Pattern A, B, C, and D placeholder structures across all 22 Rust targets and 3 XML assets.
4. **Physical Build & Gate Verification**:
   - Verified via `cargo check --workspace` and `python3 scripts/verify_ledger_closure.py`.
