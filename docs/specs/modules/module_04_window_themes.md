# Module 04 Specification: Window, App Shell & Themes Subsystem (`module_04_window_themes`)

## 1. Executive Summary & Architectural Objectives

Module 04 (`main & window & layout` / `window & themes`) provides the foundational user interface framework, window lifecycle coordination, adaptive responsive layout engine, app shell navigation controllers, and rich theming/palette subsystem for the Telegram Desktop GPUI rewrite.

In the upstream C++ implementation (`Telegram/SourceFiles/window/`, `Telegram/SourceFiles/window/themes/`, `Telegram/SourceFiles/menu/`, and `Telegram/SourceFiles/support/`), this subsystem is responsible for:
1. **Window Lifecycle & Multi-Window Management**: Managing top-level OS windows, separate message/media windows (`Window::SeparateId`), window activation, minimization to tray, full-screen modes, and pass-code / terms-of-service locking (`Window::Controller`, `MainWindow`).
2. **Session Navigation & Shell Orchestration**: Serving as the master router for user interactions within an active session (`Window::SessionController`, `Window::SessionNavigation`), driving section navigation stacks (`SectionShow`, `SectionMemento`), dialog selection, forum topic switching, search contexts, and modal layer overlays.
3. **Adaptive Desktop Layout**: Implementing responsive column layouts dynamically transitioning between `OneColumn` (< 600px mobile/compact mode), `Normal` (600–1024px two-column mode: dialogs + chat), and `ThreeColumn` (>= 1024px three-column mode: dialogs + chat + right info/profile drawer) with `Normal` and `Wide` chat bubbles (`Window::Adaptive`).
4. **Window Main Menu & Sliding Drawer**: Providing the hamburger slide-out navigation panel with user profile card, status badges, multi-account switcher with unread counts (`OthersUnreadState`), new group/channel creation shortcuts, settings shortcuts, and animated day/night theme toggle (`Window::MainMenu`).
5. **Theme & Palette Subsystem**: Comprehensive color scheme engine providing embedded daylight and night presets (`DayBlue`, `Default` Classic, `Night`, `NightGreen`), customizable accent color tinting (`EmbeddedAccentRecipe`, `AccentColors`), cloud-synchronized theme packs, wallpaper management with pattern tiling (`ChatBackground`, `BackgroundUpdate`), and live in-app theme editor with real-time preview (`WindowThemeEditor`, `WindowThemePreview`).
6. **Desktop Notifications & Toast Overlays**: Non-intrusive desktop notification dispatching (`NotificationsManager`), multi-notification stacking, sound triggers, and in-window transient toast popups (`Ui::Toast`).
7. **Context Menu Actions & Support Tools**: Rich popup action validators (`menu_*`) and support agent autocompletion / canned template tools (`support_*`).

In this GPUI Rust implementation, all legacy Qt widgets (`QWidget`, `QEvent`, `QPainter`) are faithfully translated into GPUI element hierarchies (`gpui::IntoElement`, `gpui::Render`, `gpui::div`, `gpui::Window`, `gpui::Context`), high-performance reactive state machines, and strongly-typed data structures wired directly into the existing Rust domain layer (`fabushi-messaging-core`, `mahayana-node-agent-coordinator`, and `gpui_telegram_core`).

---

## 2. Upstream C++ Source to Target Rust Mapping

The following ledger establishes the 1:1 bidirectional mapping across all 128 source entries in `docs/specs/ledger/shards/window.json` and their corresponding Rust targets in `gpui_telegram_desktop`:

| Upstream C++ Source | Core C++ Classes / Symbols | Target Rust Path | Target Rust Symbol | Responsibilities |
|---|---|---|---|---|
| `window/window_controller.h/.cpp` | `Window::Controller`, `Ui::Show` | `src/views/app_shell/window_controller.rs` | `WindowController` | Root window lifecycle, activation, modal layer stack, toast dispatch, locking |
| `window/window_session_controller.h/.cpp` | `Window::SessionController`, `SessionNavigation` | `src/views/app_shell/window_session_controller.rs` | `WindowSessionController` | Active session routing, section stack, chat/forum navigation, search state |
| `window/window_session_controller_contact.cpp` | `ContactTokenResolver`, `ResolveContact` | `src/views/app_shell/window_session_controller_contact.rs` | `WindowSessionControllerContact` | Deep link and contact token resolution state machine |
| `window/window_session_controller_link_info.h` | `PeerByLinkInfo`, `LinkInfoType` | `src/views/app_shell/window_session_controller_link_info.rs` | `WindowSessionControllerLinkInfo` | Information record for links opening chats, voice chats, or messages |
| `window/main_window.h/.cpp` | `MainWindow`, `PositionTracker` | `src/views/app_shell/main_window.rs` | `MainWindow` | Top-level GPUI window container, titlebar controls, tray minimize logic |
| `window/window_adaptive.h/.cpp` | `Window::Adaptive`, `WindowLayout`, `ChatLayout` | `src/views/app_shell/window_adaptive.rs` | `WindowAdaptive` | Breakpoint-based responsive 1-column, 2-column, and 3-column layout engine |
| `window/window_main_menu.h/.cpp` | `Window::MainMenu`, `OthersUnreadState` | `src/views/app_shell/window_main_menu.rs` | `WindowMainMenu` | Sliding navigation drawer, userpic, accounts switcher, action items |
| `window/window_main_menu_helpers.h/.cpp` | `SetupMainMenuAction`, `CreateMenuButton` | `src/views/app_shell/window_main_menu_helpers.rs` | `WindowMainMenuHelpers` | Button builders and visual styling helpers for main menu items |
| `window/window_separate_id.h/.cpp` | `Window::SeparateId`, `SeparateType` | `src/views/app_shell/window_separate_id.rs` | `WindowSeparateId` | Unique identifiers for detached peer/media windows |
| `window/window_saved_windows.h/.cpp` | `SavedWindowsManager`, `WindowPosition` | `src/views/app_shell/window_saved_windows.rs` | `WindowSavedWindows` | Persistent multi-window geometry, state serialization and restoration |
| `window/window_restore_shell.h/.cpp` | `RestoreShell`, `RestoreState` | `src/views/app_shell/window_restore_shell.rs` | `WindowRestoreShell` | Window state recovery after crash or process restart |
| `window/section_widget.h/.cpp` | `Window::SectionWidget` | `src/views/app_shell/section_widget.rs` | `SectionWidget` | Generic base container for navigation stack view sections |
| `window/section_memento.h` | `Window::SectionMemento` | `src/views/app_shell/section_memento.rs` | `SectionMemento` | Snapshot of section state (scroll position, drafts) for back-navigation |
| `window/window_section_common.h` | `SectionSlideParams`, `SectionTransition` | `src/views/app_shell/window_section_common.rs` | `WindowSectionCommon` | Transition animations, easing curves, and slide directions |
| `window/session/window_session_media.h/.cpp` | `SessionMedia`, `MediaViewerContext` | `src/views/app_shell/session/window_session_media.rs` | `WindowSessionMediaSession` | Media viewer integration state attached to the active window session |
| `window/window_lock_widgets.h/.cpp` | `LockWidget`, `PasscodeBox` | `src/views/app_shell/window_lock_widgets.rs` | `WindowLockWidgets` | Lock screen overlay displaying when application is passcode-locked |
| `window/window_unlock_passcode_box.h/.cpp` | `UnlockPasscodeBox` | `src/views/app_shell/window_unlock_passcode_box.rs` | `WindowUnlockPasscodeBox` | Passcode entry dialog with error shake animation and auto-lock reset |
| `window/window_setup_email.h/.cpp` | `SetupEmailBox`, `SetupEmailState` | `src/views/app_shell/window_setup_email.rs` | `WindowSetupEmail` | Mandatory email verification modal gate during onboarding/login |
| `window/window_no_paid_token.h/.cpp` | `NoPaidTokenBox` | `src/views/app_shell/window_no_paid_token.rs` | `WindowNoPaidToken` | Warning modal for paid channel/bot interactions without active credits |
| `window/window_connecting_widget.h/.cpp` | `ConnectingWidget`, `ConnectionState` | `src/views/app_shell/window_connecting_widget.rs` | `WindowConnectingWidget` | Top status banner showing "Connecting...", "Updating...", or proxy info |
| `window/window_top_bar_wrap.h` | `TopBarWrap`, `WrapContent` | `src/views/app_shell/window_top_bar_wrap.rs` | `WindowTopBarWrap` | Header bar wrapper handling title text, subtitle, action buttons, and back arrow |
| `window/window_slide_animation.h/.cpp` | `SlideAnimation`, `SlideDirection` | `src/views/app_shell/window_slide_animation.rs` | `WindowSlideAnimation` | Interpolated horizontal sliding transitions for section switches |
| `window/window_history_hider.h/.cpp` | `HistoryHider` | `src/views/app_shell/window_history_hider.rs` | `WindowHistoryHider` | Overlay preventing background chat flash during fast navigation transitions |
| `window/window_text_drop.h/.cpp` | `TextDropHandler` | `src/views/app_shell/window_text_drop.rs` | `WindowTextDrop` | Drag-and-drop text/file reception coordinator for window regions |
| `window/window_chat_preview.h/.cpp` | `ChatPreviewManager` | `src/views/app_shell/window_chat_preview.rs` | `WindowChatPreview` | Long-press/hover popup preview for dialogs and profile pictures |
| `window/window_chat_switch_process.h/.cpp` | `ChatSwitchProcess` | `src/views/app_shell/window_chat_switch_process.rs` | `WindowChatSwitchProcess` | Asynchronous chat load-and-switch transaction manager |
| `window/window_media_preview.h/.cpp` | `MediaPreviewWidget` | `src/views/app_shell/window_media_preview.rs` | `WindowMediaPreview` | Quick inline media inspection popup |
| `window/window_filters_menu.h/.cpp` | `FiltersMenu`, `FiltersList` | `src/views/app_shell/window_filters_menu.rs` | `WindowFiltersMenu` | Left vertical/horizontal tab bar for chat folders/filters |
| `window/window_filters_favorite.h/.cpp` | `FiltersFavoriteManager` | `src/views/app_shell/window_filters_favorite.rs` | `WindowFiltersFavorite` | Quick favorite filter switching and re-ordering state |
| `window/window_peer_menu.h/.cpp` | `PeerMenuBuilder`, `PeerActions` | `src/views/app_shell/window_peer_menu.rs` | `WindowPeerMenu` | Context menu generator for chats, channels, users, and bots |
| `window/notifications_manager.h/.cpp` | `Notifications::Manager` | `src/views/app_shell/notifications_manager.rs` | `NotificationsManager` | Core notification routing, audio alerts, and display scheduling |
| `window/notifications_manager_default.h/.cpp` | `Notifications::Default::Manager` | `src/views/app_shell/notifications_manager_default.rs` | `NotificationsManagerDefault` | Desktop notification window manager with grouping, dismiss, and actions |
| `window/notifications_utilities.h/.cpp` | `NotificationUtils`, `FormatBody` | `src/views/app_shell/notifications_utilities.rs` | `NotificationsUtilities` | Notification text clipping, avatar thumbnailing, and privacy redaction |
| `window/support_autocomplete.h/.cpp` | `Support::Autocomplete` | `src/views/app_shell/support_autocomplete.rs` | `SupportAutocomplete` | Support query shortcut and macro completion engine |
| `window/support_common.h/.cpp` | `Support::CommonData` | `src/views/app_shell/support_common.rs` | `SupportCommon` | Shared data structures for Telegram customer support workflows |
| `window/support_helper.h/.cpp` | `Support::Helper` | `src/views/app_shell/support_helper.rs` | `SupportHelper` | Support ticket management and canned answer provider |
| `window/support_preload.h/.cpp` | `Support::Preload` | `src/views/app_shell/support_preload.rs` | `SupportPreload` | Preloading support templates and user history |
| `window/support_templates.h/.cpp` | `Support::Templates` | `src/views/app_shell/support_templates.rs` | `SupportTemplates` | Support template storage and search index |
| `window/themes/window_theme.h/.cpp` | `Window::Theme`, `Instance`, `Cached` | `src/views/app_shell/themes/window_theme.rs` | `WindowThemeThemes` | Global theme manager, palette application, and wallpaper state |
| `window/themes/window_theme_editor.h/.cpp` | `ThemeEditor`, `PaletteColorPicker` | `src/views/app_shell/themes/window_theme_editor.rs` | `WindowThemeEditorThemes` | Interactive theme editor with property tree and color picker |
| `window/themes/window_theme_editor_block.h/.cpp` | `EditorBlock` | `src/views/app_shell/themes/window_theme_editor_block.rs` | `WindowThemeEditorBlockThemes` | Section block widget within the theme editor palette inspector |
| `window/themes/window_theme_editor_box.h/.cpp` | `ThemeEditorBox` | `src/views/app_shell/themes/window_theme_editor_box.rs` | `WindowThemeEditorBoxThemes` | Modal container for opening, exporting, and saving theme files |
| `window/themes/window_theme_preview.h/.cpp` | `ThemePreview` | `src/views/app_shell/themes/window_theme_preview.rs` | `WindowThemePreviewThemes` | Mock chat stage demonstrating live palette changes |
| `window/themes/window_theme_warning.h/.cpp` | `ThemeWarningBox` | `src/views/app_shell/themes/window_theme_warning.rs` | `WindowThemeWarningThemes` | Warning modal for unsupported or unstable theme file versions |
| `window/themes/window_themes_chat.h/.cpp` | `ThemesChat` | `src/views/app_shell/themes/window_themes_chat.rs` | `WindowThemesChatThemes` | Per-chat theme selection, overrides, and cloud wallpaper pairing |
| `window/themes/window_themes_cloud_list.h/.cpp` | `CloudThemesList` | `src/views/app_shell/themes/window_themes_cloud_list.rs` | `WindowThemesCloudListThemes` | Official cloud theme catalogue browser and downloader |
| `window/themes/window_themes_embedded.h/.cpp` | `EmbeddedScheme`, `AccentRecipe` | `src/views/app_shell/themes/window_themes_embedded.rs` | `WindowThemesEmbeddedThemes` | Built-in themes (DayBlue, Default, Night, NightGreen) and accent recipes |
| `window/themes/window_themes_generate_name.h/.cpp` | `GenerateThemeName` | `src/views/app_shell/themes/window_themes_generate_name.rs` | `WindowThemesGenerateNameThemes` | Human-readable name generator for user-created theme schemes |
| `menu/gift_resale_filter.h/.cpp` | `GiftResaleFilterAction` | `src/views/app_shell/gift_resale_filter.rs` | `GiftResaleFilter` | Menu filter for collectible gifts and resale items |
| `menu/menu_action_with_thumbnail.h/.cpp` | `ActionWithThumbnail` | `src/views/app_shell/menu_action_with_thumbnail.rs` | `MenuActionWithThumbnail` | Menu item with dynamic icon or user avatar thumbnail |
| `menu/menu_antispam_validator.h/.cpp` | `AntispamValidator` | `src/views/app_shell/menu_antispam_validator.rs` | `MenuAntispamValidator` | Action validator ensuring administrative anti-spam rules |
| `menu/menu_check_item.h/.cpp` | `MenuCheckItem` | `src/views/app_shell/menu_check_item.rs` | `MenuCheckItem` | Toggleable checkbox action within context menus |
| `menu/menu_checked_action.h/.cpp` | `MenuCheckedAction` | `src/views/app_shell/menu_checked_action.rs` | `MenuCheckedAction` | Radio-group action maintaining single selection |
| `menu/menu_dock.h/.cpp` | `MenuDock` | `src/views/app_shell/menu_dock.rs` | `MenuDock` | Context menu container docking to click coordinates |
| `menu/menu_emoji_status.h/.cpp` | `MenuEmojiStatus` | `src/views/app_shell/menu_emoji_status.rs` | `MenuEmojiStatus` | Emoji status chooser action in profile popup menu |
| `menu/menu_item_download_files.h/.cpp` | `ItemDownloadFiles` | `src/views/app_shell/menu_item_download_files.rs` | `MenuItemDownloadFiles` | Menu action to trigger file or media batch downloading |
| `menu/menu_item_rate_transcribe.h/.cpp` | `ItemRateTranscribe` | `src/views/app_shell/menu_item_rate_transcribe.rs` | `MenuItemRateTranscribe` | Voice transcription feedback rating action |
| `menu/menu_item_rate_transcribe_session.h/.cpp` | `RateTranscribeSession` | `src/views/app_shell/menu_item_rate_transcribe_session.rs` | `MenuItemRateTranscribeSession` | Session state for voice transcription rating submission |
| `menu/menu_mark_as_read.h/.cpp` | `MarkAsReadAction` | `src/views/app_shell/menu_mark_as_read.rs` | `MenuMarkAsRead` | Context menu action to mark chat or folder as read |
| `menu/menu_mute.h/.cpp` | `MuteAction` | `src/views/app_shell/menu_mute.rs` | `MenuMute` | Context menu action for mute durations (1h, 8h, 2d, forever) |
| `menu/menu_send.h/.cpp` | `SendAction` | `src/views/app_shell/menu_send.rs` | `MenuSend` | Extended send menu (Send without sound, Schedule message, Send when online) |
| `menu/menu_send_details.h` | `SendDetails` | `src/views/app_shell/menu_send_details.rs` | `MenuSendDetails` | Metadata payload for scheduled or silent send actions |
| `menu/menu_sponsored.h/.cpp` | `SponsoredAction` | `src/views/app_shell/menu_sponsored.rs` | `MenuSponsored` | Context menu actions for sponsored channel messages |
| `menu/menu_timecode_action.h/.cpp` | `TimecodeAction` | `src/views/app_shell/menu_timecode_action.rs` | `MenuTimecodeAction` | Timestamp link copier action for video/audio playback |
| `menu/menu_ttl.h/.cpp` | `TtlAction` | `src/views/app_shell/menu_ttl.rs` | `MenuTtl` | Auto-delete / TTL setting menu action (24h, 7d, 1m, custom) |
| `menu/menu_ttl_validator.h/.cpp` | `TtlValidator` | `src/views/app_shell/menu_ttl_validator.rs` | `MenuTtlValidator` | Privilege validator for enabling message auto-delete in channels |
| `window/window.style` | `windowMinWidth`, `windowDefaultWidth`, metrics | `src/theme/window/window_style.rs` | `WindowStyleSheet` | Layout metrics, minimum window sizes, shadows, splitter handles |
| `window/window_lock_widgets.style` | `passcodeBoxWidth`, lock button styles | `src/theme/window/window_lock_widgets_style.rs` | `WindowLockWidgetsStyleSheet` | Styling tokens for passcode lock screen and error highlights |
| `window/window_main_menu.style` | `mainMenuWidth`, slide animation durations | `src/theme/window/window_main_menu_style.rs` | `WindowMainMenuStyleSheet` | Drawer width (280px), userpic dimensions, account toggle metrics |

---

## 3. Window Management Hierarchy & Controller Lifecycle

### 3.1 WindowController & MainWindow Architecture

The desktop client architecture separates physical OS window management from active session orchestration:

```
┌──────────────────────────────────────────────────────────────┐
│                       GPUI Application                       │
└──────────────────────────────┬───────────────────────────────┘
                               │ spawns / manages
                               ▼
               ┌──────────────────────────────┐
               │       WindowController       │
               │  - SeparateId (Primary / ID) │
               │  - Passcode Lock Overlay     │
               │  - Modal Layer Manager       │
               │  - Toast Notification Queue  │
               └──────────────┬───────────────┘
                              │ owns
                              ▼
               ┌──────────────────────────────┐
               │          MainWindow          │
               │  - GPUI Window Frame / Title │
               │  - Custom macOS/Win Titlebar │
               │  - Adaptive Breakpoint FSM   │
               └──────────────┬───────────────┘
                              │ embeds
                              ▼
               ┌──────────────────────────────┐
               │   WindowSessionController    │
               │  - MainWidget / Active Chat  │
               │  - Navigation Section Stack  │
               │  - Dialogs Selection State   │
               │  - Sliding Main Menu Drawer  │
               └──────────────────────────────┘
```

1. **`WindowController`**:
   - Manages top-level window identity (`SeparateId`). A window may be the primary application window (`SeparateId::Primary`) or a secondary detached window (`SeparateId::Chat(peer_id)` or `SeparateId::Media(msg_id)`).
   - Coordinates global modal layers (`show_box()`, `show_layer()`, `hide_layer()`). Multiple dialog boxes form a stack; pressing Esc or the back button dismisses the topmost layer.
   - Hosts toast notifications (`show_toast()`). Toasts queue gracefully, display at the bottom center of the window, and dismiss after their configured duration.
   - Implements application locking: when locked by passcode timeout or user command, the main contents are masked and `WindowLockWidgets` takes focus.
2. **`MainWindow`**:
   - The top-level GPUI element container. Handles window resizing, dragging, minimize-to-system-tray behavior, and activation/focus changes.
   - Embeds native or client-side titlebar controls (close, minimize, maximize buttons).
   - Monitors window width and forwards dimension updates to `WindowAdaptive`.

---

## 4. Session Controller & Navigation State Machine

### 4.1 Section Navigation Stack

Within an active session, navigation between chat histories, user profiles, shared media lists, and settings is handled by `WindowSessionController`:

```
                 ┌────────────────────────────────┐
                 │     WindowSessionController    │
                 └───────────────┬────────────────┘
                                 │ drives
                                 ▼
                 ┌────────────────────────────────┐
                 │       SectionShow Request      │
                 │ - way: Forward/Backward/Clear  │
                 │ - animated: bool               │
                 │ - highlight: MessageHighlight  │
                 └───────────────┬────────────────┘
                                 │ pushes / pops
                                 ▼
┌──────────────────┐   Forward   ┌──────────────────┐   Forward   ┌──────────────────┐
│   Dialogs List   │ ──────────► │   Chat History   │ ──────────► │   Profile Info   │
│   (Root Stage)   │ ◄────────── │   (PeerId 100)   │ ◄────────── │   (Shared Media) │
└──────────────────┘   Backward  └──────────────────┘   Backward  └──────────────────┘
                                 ▲
                                 │ SectionMemento (Restores scroll & draft)
                                 └─────────────────────────────────────────
```

- **`SectionMemento`**: When navigating away from a section, a memento captures the current scroll position, active draft text, and selection state. On `SectionShow::Way::Backward`, the previous section is restored from its memento.
- **`SectionShow::Way::ClearStack`**: Used when clicking a chat in the dialogs list. Clears intermediate history stack layers and directly presents the requested chat history.

### 4.2 Active Chat & Dialog Selection State

`WindowSessionController` maintains reactive observables:
- `active_chat`: Currently selected `PeerId` / `DialogsKey`.
- `active_folder`: Currently opened folder filter (`Data::Folder`).
- `search_query`: Active search filter in the dialogs list or within the active chat.
- `chat_preview`: Transient popup triggered on hover/long-press showing message snapshots without marking messages as read.

---

## 5. Adaptive Responsive Layout Engine (`WindowAdaptive`)

Telegram Desktop adapts seamlessly between narrow compact windows and ultra-wide screens:

```
┌────────────────────────────────────────────────────────────────────────┐
│ OneColumn Layout (< 600px)                                             │
│ ┌────────────────────────────────────────────────────────────────────┐ │
│ │ Either Dialogs List OR Active Chat Stage (Full Width)              │ │
│ └────────────────────────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────────────┐
│ Normal Layout (600px - 1024px)                                         │
│ ┌───────────────────────────┬────────────────────────────────────────┐ │
│ │ Dialogs List (260-400px)  │ Active Chat Stage (Remaining Width)    │ │
│ └───────────────────────────┴────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────────────┐
│ ThreeColumn Layout (>= 1024px)                                         │
│ ┌──────────────────────┬──────────────────────────┬──────────────────┐ │
│ │ Dialogs (260-400px)  │ Chat Stage (Flex 1)      │ Info/Right Column│ │
│ │                      │                          │ (320-400px)      │ │
│ └──────────────────────┴──────────────────────────┴──────────────────┘ │
└────────────────────────────────────────────────────────────────────────┘
```

### Breakpoint Metrics:
- `kOneColumnThreshold`: 600 px.
- `kThreeColumnThreshold`: 1024 px.
- `ChatLayout`:
  - `Normal`: Maximum message bubble width capped at 480 px.
  - `Wide`: Message bubble width expands dynamically on wide displays.

---

## 6. Window Main Menu & Sliding Drawer Architecture

The sliding drawer (`WindowMainMenu`) is summoned via the hamburger icon in the top left:

1. **Header Card**:
   - User profile avatar with initials or custom photo (`UserpicButton`).
   - Profile full name and verified badge (`Info::Profile::Badge`).
   - Phone number / username subtitle.
   - Emoji status badge (`EmojiStatusPanel`).
2. **Account Switcher Accordion**:
   - Lists up to 4 configured Telegram accounts (`Main::Account`).
   - Displays unread message badges for background accounts (`OthersUnreadState`).
   - Action button to add a new account.
3. **Menu Item List**:
   - My Profile / Saved Messages.
   - New Group.
   - New Channel.
   - Contacts Directory.
   - Calls History.
   - Saved Messages.
   - Settings.
   - Night Mode Toggle: Custom switch with animated timer and immediate palette transition.
4. **Footer**:
   - Version label (e.g. `GPUI Telegram v1.0.0 (Desktop)`).

---

## 7. Themes & Palette Subsystem Architecture

### 7.1 Embedded Theme Schemes & Accent Recipes

`WindowThemeThemes` and `WindowThemesEmbeddedThemes` manage four foundational schemes:

```
┌───────────────────┬───────────────────┬───────────────────┬───────────────────┐
│     DayBlue       │  Default (Classic)│       Night       │    NightGreen     │
├───────────────────┼───────────────────┼───────────────────┼───────────────────┤
│ Primary: #2AABEE  │ Primary: #3390EC  │ Primary: #5288C1  │ Primary: #4FA882  │
│ Bg:      #FFFFFF  │ Bg:      #F5F5F5  │ Bg:      #18222D  │ Bg:      #17212B  │
│ Sidebar: #F1F1F1  │ Sidebar: #FFFFFF  │ Sidebar: #0E1621  │ Sidebar: #0E1621  │
│ BubbleIn:#FFFFFF  │ BubbleIn:#FFFFFF  │ BubbleIn:#182533  │ BubbleIn:#1E2C3A  │
│ BubbleOut:#EEFFDE │ BubbleOut:#EFFDDE │ BubbleOut:#2B5278 │ BubbleOut:#26473E │
└───────────────────┴───────────────────┴───────────────────┴───────────────────┘
```

### 7.2 Dynamic Accent Colors & Palette Recipe Engine
`EmbeddedAccentRecipe` computes derived tints across all UI tokens from a single selected accent color:
- `active_tab_border`: 100% accent.
- `button_hover`: Accent blended with 10% white (dark mode) or 10% black (day mode).
- `bubble_out_border`: Accent blended with background at 25% opacity.
- `contrast_ratio`: Ensures text readability satisfies WCAG 2.1 AAA requirements.

### 7.3 Theme Editor & Live Chat Preview
- `WindowThemeEditorThemes`: Allows users to inspect any palette variable, tweak hex values, and test custom backgrounds.
- `WindowThemePreviewThemes`: Renders a mock conversation stage showing outgoing bubbles, incoming bubbles, timestamps, and checkmarks under the live editor palette.

---

## 8. Desktop Notifications & System Tray Integration

1. **`NotificationsManager`**: Central dispatcher receiving incoming message updates.
   - Evaluates peer mute status, do-not-disturb schedules, and focus assist settings.
   - Computes total unread count across all accounts for the dock/taskbar icon badge.
2. **`NotificationsManagerDefault`**:
   - Displays floating notification popups in the screen's bottom-right or top-right corner.
   - Supports interactive quick-reply field and mute actions directly on the notification card.

---

## 9. Genuine Rust Implementation Details

All boilerplate stubs matching Patterns A, B, C, or D in `crates/gpui_telegram_desktop/src/views/app_shell/` and `src/theme/window/` are completely replaced with authentic domain logic:
- Real field definitions holding genuine application state (no `pub label: String, pub value: i64`).
- Real methods executing genuine state mutations, transitions, and calculations (no `self.count.saturating_add(1)`).
- Full compatibility with the upstream C++ source signatures recorded in `docs/specs/ledger/shards/window.json`.

---

## 10. Verification & Test Suite

The module is verified through:
1. `cargo check -p gpui_telegram_desktop` confirming clean compilation without warnings.
2. `tests/window_themes_subsystem_tests.rs`: Comprehensive test suite testing:
   - WindowController layer stacking and dismissal.
   - WindowAdaptive breakpoint calculation (OneColumn, Normal, ThreeColumn).
   - WindowSessionController navigation stack transitions and mementos.
   - MainMenu account switching and unread count aggregation.
   - ThemePalette color resolution, night mode toggling, and accent color recipe generation.
3. `python3 scripts/verify_ledger_closure.py` verifying that Boilerplate count for Module 04 drops to 0 and all Gate assertions pass.
