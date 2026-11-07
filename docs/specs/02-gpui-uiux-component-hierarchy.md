# Specification 02: GPUI 1:1 UI/UX Component Hierarchy & Design Token Specification

**Document ID**: `SPEC-02-GPUI-COMPONENT-HIERARCHY`  
**Revision**: `3.0.0` (Milestone 1, Explorer Gen 3)  
**Date**: `2026-11-06`  
**Status**: Authoritative Technical Specification & UI/UX Blueprint  
**Target Repository**: `/Users/bhrum/teamwork_projects/gpui_telegram`  
**Target Path**: `docs/specs/02-gpui-uiux-component-hierarchy.md`  

---

## 1. Executive Summary & Design System Principles

This specification defines the complete, production-grade **GPUI 0.2.2 (Rust)** UI/UX component hierarchy, layout geometry, interaction models, and pixel-for-pixel design tokens to reconstruct **Telegram Desktop** with 1:1 fidelity, while seamlessly embedding advanced agentic, task, and hardware control capabilities.

### 1.1 Core Rendering & Layout Principles in GPUI
1. **GPU-Accelerated Native Rendering**:
   - GPUI renders directly via Metal (macOS) / Vulkan (Linux/Windows) with sub-millisecond frame times.
   - Zero DOM overhead, zero Electron/Chromium lag.
2. **Virtualized Viewport Rendering**:
   - Sidebar chat rows use GPUI `uniform_list` (O(1) layout calculation for uniform 62px rows).
   - Conversation stage messages use GPUI `SumTree` virtualized list with variable height caching and binary search index projection.
3. **IME Composition Protection**:
   - The CanonicalComposer protects multi-byte IME inputs (CJK candidates) to prevent premature message dispatch during composition.
4. **Strict Upstream Metric Parity**:
   - Every spacing, padding, radius, font, and color token is extracted directly from upstream `tdesktop/Telegram/SourceFiles/*.style` files.

---

## 2. 4-Pane AppShell Architecture

The primary application shell is structured as a 4-pane responsive layout:

```
+-------------------------------------------------------------------------------------------------------------------------+
| [Traffic Lights]  Window Title / Active Chat Breadcrumbs                                                 [ -  +  x ]     |
+----+-----------------------+--------------------------------------------------------------------+-----------------------+
| N  | [ Search Input      ] | [ Pinned Message Sticky Banner (Optional)                        ] | [ Context Inspector ] |
| a  | +-------------------+ |--------------------------------------------------------------------+ |  (320px Right Drawer) |
| v  | | All | Work | Bots | |                                                                    | |                       |
| R  | +-------------------+ |                      ConversationStage                             | |  - Peer Profile     |
| a  |                       |                     (SumTree Virtual List)                         | |  - Member Roster    |
| i  | ChatSidebar           |                                                                    | |  - Shared Media     |
| l  | (288px)               |   [Incoming Message Bubble]                                        | |  - Task Center      |
|    |                       |                   [Outgoing Message Bubble]                        | |  - MCP Tools Dock   |
| 5  | - DialogRow (62px)    |   [Voice Note Waveform Bubble]                                     | |                       |
| 2  | - DialogRow (62px)    |                   [A2UI Task Card]                                 | |                       |
| p  | - DialogRow (62px)    |   [ACP Thinking Process Stream]                                    | |                       |
| x  | - DialogRow (62px)    |                                                                    | |                       |
|    | - DialogRow (62px)    |--------------------------------------------------------------------+ |                       |
|    |                       | CanonicalComposer (Min 44px, Auto-grow 192px, IME Protected)       | |                       |
|    |                       | [Attach] [   Type a message...                          ] [Mic/Send] | |                       |
+----+-----------------------+--------------------------------------------------------------------+-----------------------+
```

### 2.1 Pane 1: Custom Metal Window Frame & Titlebar
- **Upstream Anchors**: `Telegram/SourceFiles/window/window_main_menu.cpp`, `window/window.style`
- **Dimensions & Geometry**:
  * Titlebar Height: 38px
  * macOS Traffic Lights Margin: Left 14px, Top 12px
  * Window Border Radius: 10px (macOS Sonoma / Sequoia native curvature)
- **GPUI Implementation Structure**:
  * Component: `CustomWindowFrame` (`crates/gpui_telegram_desktop/src/views/app_shell/window_frame.rs`)
  * Draggable Area: Entire header bar excluding window controls, search box, and action buttons via `window.on_mouse_down()` window drag handler.
  * Active Chat Header: Avatar (32px), Peer Name (font 14px bold), Presence / Member subtitle (font 12px muted), Action icons (Audio Call, Video Call, Search, Right Drawer Toggle).

### 2.2 Pane 2: Primary Navigation Rail (PrimaryNavRail)
- **Width**: Exactly **52px** fixed width (Leftmost strip)
- **Background**: `#1c242d` (Night) / `#2b5278` (Day Blue)
- **Items & Alignment**:
  * **Top Section**:
    1. Active Account Avatar (36px circular avatar with online dot badge, click to pop account switcher)
    2. Separator Line (32px width, 1px height, 10% opacity)
    3. Chats Nav Item (Icon: Chat bubbles, badge counter)
    4. Global Task Center Nav Item (`CAP-02`, Icon: Checkbox list, running tasks badge)
    5. MCP & Tools Nav Item (`CAP-03`, Icon: Puzzle / Terminal plug)
    6. Talk Mode Full-Duplex Orb Nav Item (`CAP-05`, Icon: Pulsing sound wave)
    7. Remote Computer Workbench Nav Item (`CAP-04`, Icon: Desktop monitor)
  * **Bottom Section**:
    8. Saved Messages Shortcut (Bookmark icon)
    9. Settings Gear (Open settings modal)
    10. Connection Status Dot (Green: Connected, Orange: Connecting / Updating PTS)
- **GPUI Implementation**:
  * Component: `PrimaryNavRail` (`crates/gpui_telegram_desktop/src/views/app_shell/primary_nav_rail.rs`)
  * Event Routing: Emits `NavSelectionChanged(NavDestination)` to parent `AppShell`.

### 2.3 Pane 3: Chat Sidebar (ChatSidebar)
- **Width**: Default **288px** (resizable between 240px and 360px via mouse drag handle)
- **Upstream Anchors**: `Telegram/SourceFiles/dialogs/dialogs_widget.cpp`, `dialogs/dialogs.style`
- **Sub-Components**:
  1. **Universal Search Header**:
     * Height: 48px
     * Search Input Field: Rounded 16px background, search magnifying glass icon, shortcut placeholder (`Cmd+K`).
     * Menu Button: Hamburger icon for legacy Telegram menu.
  2. **Folder Tabs Strip (Filter Chips)**:
     * Height: 36px
     * Tabs: Horizontal scrollable strip (`All`, `Personal`, `Work`, `Bots`, `Unread`, `+ Add Folder`)
     * Active Tab Indicator: 2px accent underline, active text bold with unread count pill badge (`dialogsUnreadHeight: 19px`).
  3. **Virtualized Dialog List**:
     * Implemented using GPUI `uniform_list` for maximum memory efficiency.
     * Item Height: Exactly **62px** (`dialogsRowHeight: 62px` from upstream `dialogs.style`).
     * Item Layout (`defaultDialogRow` parity):
       - Userpic Avatar: 46px diameter (`photoSize: 46px`) positioned at `point(10px, 8px)`.
       - Title / Peer Name: Left 68px (`nameLeft: 68px`), Top 10px (`nameTop: 10px`), font 14px bold (`nameFont`).
       - Date / Timestamp: Right aligned at 10px right margin, font 12px muted (`dialogsDateFont`).
       - Preview Message Snippet: Left 68px (`textLeft: 68px`), Top 34px (`textTop: 34px`), font 13px regular (`dialogsTextFont`).
       - Delivery Status Icons: Clock (sending), Single check (sent), Double check (read).
       - Unread Count Pill Badge: Height 19px (`dialogsUnreadHeight: 19px`), min-width 19px, padding 5px (`dialogsUnreadPadding: 5px`), background `#40a7e3` (active) / `#6b808d` (muted).
       - Pinned Chat Pin Icon: 12px pin icon replacing unread badge when muted and read.

### 2.4 Pane 4: Main Conversation Stage & CanonicalComposer
- **Stage Layout**: Flex-grow 1, vertical column container.
- **Upstream Anchors**: `Telegram/SourceFiles/history/history.cpp`, `ui/chat/chat.style`
- **Sub-Components**:
  1. **Pinned Message Sticky Banner**:
     * Height: 44px sticky bar at top of stage.
     * Left blue accent bar (2px width), "Pinned Message" title, single-line text preview, dismiss button.
     * Clicking cycles through multiple pinned messages.
  2. **ConversationStage (SumTree Virtual List)**:
     * Variable-height item projection using GPUI `SumTree`.
     * Dynamic scroll-to-bottom anchor lock during incoming streaming.
     * Date separator pills: Centered rounded pill badges (`margins(12px, 3px, 12px, 4px)`).
  3. **CanonicalComposer (Bottom Dock)**:
     * Min Height: 44px, Max Height: 192px (smooth auto-expansion based on text line count).
     * Attachment Button (`+` paperclip): Click triggers media picker popover (Photo, Document, Poll, Task).
     * IME-Protected Text Area: Multi-line text edit buffer with IME composition fence.
     * Emoji & Sticker Trigger: Smile icon revealing animated sticker & emoji drawer.
     * Voice Record / Send Action Button:
       - When text is empty: Microphone icon (Hold-to-record voice note gesture, slide left to cancel, slide up to lock).
       - When text is present: Paper airplane icon (Immediate send on Enter; Cmd+Enter for newline, configurable).

### 2.5 Context Inspector Drawer (Unified Right Panel)
- **Width**: Exactly **320px** fixed width (Toggleable via header button or `Cmd+I`)
- **Upstream Anchors**: `Telegram/SourceFiles/info/info_wrap_widget.cpp`, `info/info.style`
- **Tab Panes**:
  1. **Profile Pane**: Peer avatar (80px), Name, Username, Phone number, Bio, Notifications toggle.
  2. **Members Pane**: Member list with admin badges, search member, add member button.
  3. **Shared Media Pane**: Filtered grid of Photos, Videos, Documents, Audio files, Voice notes, and Links.
  4. **Task Center Pane (`CAP-02`)**: Chat-specific background tasks, cron routines, status timeline.
  5. **MCP Tools Pane (`CAP-03`)**: Registered agent tools, tool invocation logs, permissions grant list.

---

## 3. The 11 Canonical Message Card Types

Every message rendered in `ConversationStage` maps to one of **11 canonical card types**. Each card enforces strict Telegram Desktop visual geometry, bubble curvature rules, and interactive state machines.

### 3.1 Bubble Geometry & Corner Radius Invariants
- **Bubble Radii**:
  * Outer/Leading corner radius: **16px** (`st::bubbleRadiusLarge`)
  * Attached/Consecutive same-author message corner radius: **4px** (`st::bubbleRadiusSmall`)
- **Max Width**: **430px** (`st::msgMaxWidth`)
- **Min Width**: **160px** (`st::msgMinWidth`)
- **Padding**: `margins(11px, 8px, 11px, 8px)` (`st::msgPadding`)
- **Bubble Colors (Day Blue)**:
  * Incoming: `#ffffff` with `#e0e0e0` subtle border / `#f0f2f5` background
  * Outgoing: `#d7f0ff` (Telegram Day Blue sent bubble) with `#40a7e3` accent text highlights
- **Bubble Colors (Night)**:
  * Incoming: `#182533` with `#2b5278` links
  * Outgoing: `#2b5278` with `#5ca7d4` highlights

---

### 3.2 Card 1: Text Message Card (`card_text.rs`)
- **Upstream Anchor**: `Telegram/SourceFiles/history/view/history_view_message.cpp`
- **Visual Structure**:
  * Sender Name (In group chats): Font 13px bold, colored by author index (`historyPeer1..8NameFg`).
  * Reply-To Header (Optional): 2px left vertical quote bar (`historyReplyPadding: margins(11px, 2px, 6px, 2px)`), small thumbnail (32px), author name, single-line text preview.
  * Formatted Message Body: Rich text supporting Bold, Italic, Monospace (`msgInMonoFg`), Strikethrough, Underline, Spoiler (animated noise mask, click to reveal), and Inline Hyperlinks.
  * Metadata Footer: Embedded timestamp (font 12px muted) and delivery tick mark (clock, single check, double check).

### 3.3 Card 2: Photo / Video Message Card (`card_media.rs`)
- **Upstream Anchor**: `Telegram/SourceFiles/history/view/media/history_view_photo.cpp`, `history_view_video.cpp`
- **Metrics**:
  * Max Media Width: **430px** (`st::maxMediaSize`)
  * Min Media Width: **100px** (`st::minPhotoSize`)
  * Corner Radius: 16px outer, 4px inner attached
- **Visual Structure**:
  * Progressive BlurHash placeholder during initial fetch.
  * Aspect-Ratio preserving GPU-decoded texture.
  * Video Duration Badge: Translucent black pill badge bottom-left with play triangle and duration (`03:42`).
  * Caption: Formatted text below media with standard bubble padding.
  * Click Interaction: Opens Media Viewer Lightbox (`TCG-N10`).

### 3.4 Card 3: Voice Note Card (`card_voice.rs`)
- **Upstream Anchor**: `Telegram/SourceFiles/history/view/controls/history_view_voice_record_bar.style`, `data_document.cpp`
- **Metrics**:
  * Waveform Sample Count: Exactly **100 vertical bars**
  * Bar Width: 2px, Bar Gap: 1px (Total waveform width ~300px)
  * Bar Height: Dynamic based on 5-bit amplitude (0..31 scaled to 2px..24px)
  * Play Button: Circular 36px icon (`playPosition: point(2px, 0px)`, `MediaPlayerPlayIcon`)
  * Duration & Timestamp: Left aligned elapsed/total duration (`0:45 / 1:20`), right aligned message timestamp.
- **5-Bit Waveform Codec Parity**:
  * Stored as 63 bytes compact payload via `documentWaveformEncode5bit()`.
  * Playback Progress: Realtime audio playback position turns played bars from muted gray (`#a0a0a0`) to active accent (`#40a7e3`).

### 3.5 Card 4: Document / File Attachment Card (`card_document.rs`)
- **Upstream Anchor**: `Telegram/SourceFiles/history/view/media/history_view_document.cpp`, `ui/chat/chat.style`
- **Metrics**:
  * Thumbnail Box: 44px x 44px square (`st::historyFileThumbSize`) with 4px rounded corners (`st::roundRadiusLarge`).
  * File Extension Badge: Colored by type (`msgFile1Bg` Blue: PDF/DOC; `msgFile2Bg` Green: XLS/CSV; `msgFile3Bg` Red: PPT/ZIP; `msgFile4Bg` Orange: Audio/Code).
- **Visual Structure**:
  * Left: Circular Download/Open icon inside colored square (or animated progress spinner ring).
  * Right: File Name (font 14px semibold, truncated with ellipsis), File Size & Download Status (e.g. `24.5 MB / 100%`).

### 3.6 Card 5: Sticker & Lottie Animated Card (`card_sticker.rs`)
- **Upstream Anchor**: `Telegram/SourceFiles/history/view/media/history_view_sticker.cpp`, `chat_helpers/stickers_list_widget.cpp`
- **Metrics**:
  * Max Sticker Width/Height: **224px** (`st::maxStickerSize`)
  * Bubble Background: Completely transparent (frameless borderless rendering directly on chat wallpaper).
- **Interactive Behavior**:
  * Supports TGS (Lottie vector animation), WebP animated, and WebM video stickers.
  * Hover: Plays animation once or loops continuously based on settings.
  * Click: Opens Sticker Pack info modal with "Add Sticker Pack" action button.

### 3.7 Card 6: Poll & Quiz Card (`card_poll.rs`)
- **Upstream Anchor**: `Telegram/SourceFiles/poll/poll_widget.cpp`, `history/view/media/history_view_poll.cpp`
- **Visual Structure**:
  * Header: Poll Question (font 15px bold), Poll Subtitle ("Anonymous Poll" or "Quiz", total votes counter).
  * Option Rows: Radio button (single-choice) or Checkbox (multi-choice), Option label text, percentage bar background.
  * Post-Vote State: Smooth animated fill of percentage bars, vote count per option, correct answer green check (Quiz mode).
  * Footer: Retract Vote button (if enabled), "View Results" toggle.

### 3.8 Card 7: ToDo & Task Interactive Card (`card_todo.rs` / `CAP-06`)
- **Inherited Anchor**: `native/mahayana-messaging/src/cards/`, `source/host/src/tasks/`
- **Visual Structure**:
  * Top Header: Accent badge `[Task / ToDo]`, Priority indicator (Low: Gray, Medium: Blue, High: Amber, Urgent: Red).
  * Task Title & Description: Markdown rendered goal summary.
  * Checkbox Items: Interactive subtask list with checkboxes (`[x] Step 1`, `[ ] Step 2`).
  * Action Buttons: Inline button bar (`Mark Complete`, `Assign to Agent`, `Delegate`).
  * Status Pill: `PendingApproval`, `Running`, `Completed`, `Failed`.

### 3.9 Card 8: AI ACP Thinking Process Card (`card_thinking.rs` / `CAP-01`)
- **Inherited Anchor**: `source/host/src/acp/`, `source/host/src/runner/`
- **Visual Structure**:
  * Collapsible Header: Shimmering purple/blue gradient icon, "Thinking..." indicator, elapsed timer (`1.4s`), token counter (`420 tokens`).
  * Disclosure Chevron: Click expands/collapses internal reasoning stream.
  * Thought Stream Body: Monospace subtle font (`#9090a0`), real-time markdown streaming of inner thought chain, tool invocation intent blocks.
  * Swarm Lineage Breadcrumbs: Shows subagent hierarchy (e.g. `Orchestrator -> CodeExplorer_1 -> ShellRunner`).

### 3.10 Card 9: Talk Mode Energy Orb Card (`card_talk_mode_orb.rs` / `CAP-05`)
- **Inherited Anchor**: `source/host/src/talk/`, `native/mahayana-messaging/src/realtime.rs`
- **Visual Structure**:
  * Embedded Full-Duplex Session Card: Rendered when a live Talk Mode conversation is docked into a thread.
  * Animated Radial Energy Orb: Multi-layer pulsing GPU shader representing realtime mic audio amplitude and AI speech frequency.
  * Live Transcription Stream: Bi-directional speech-to-text text preview below orb.
  * Control Pill: Mute Microphone toggle, End Session button, Latency metric (`48ms VAD`).

### 3.11 Card 10: Remote Computer Workbench Card (`card_computer_workbench.rs` / `CAP-04`)
- **Inherited Anchor**: `source/node-agent-coordinator/src/vnc/`, `source/box-exec-daemon/`
- **Visual Structure**:
  * Virtual Desktop Frame: 16:9 embedded VNC display canvas streaming remote host frames.
  * Status Bar: Active Lease Badge ("Locked by Autonomous Agent" / "Human Interactivity Mode"), FPS indicator (`60 fps`), Ping (`12 ms`).
  * Control Dock: "Take Control (Lease Request)" button, "Send Ctrl+Alt+Del", "Release Lease".
  * Click Interaction: Focuses canvas for direct keyboard/mouse forwarding to remote computer.

### 3.12 Card 11: WebApp & Payment Card (`card_webapp_payment.rs` / `TCG-N28`)
- **Upstream Anchor**: `Telegram/SourceFiles/payments/ui/payments.style`, `boxes/url_auth_box_content.cpp`
- **Visual Structure**:
  * Card Header: Merchant / MiniApp branding, verified bot badge.
  * Payment Details: Item name, quantity, price breakdown in Stars (`⭐`) or fiat currency.
  * Action Button: Full-width vibrant Pay button (e.g. `Pay 250 ⭐`), secure transaction lock icon.
  * WebApp Launch Button: Opens sandboxed WebApp overlay modal with CSP enforcement.

---

## 4. CanonicalComposer & Voice Gesture Engine

### 4.1 IME Composition State Machine & Safety Fencing
Premature message dispatch during Japanese/Chinese/Korean character selection corrupts user text. The `CanonicalComposer` enforces a strict 3-state IME FSM:

```
[ Idle / Empty ] 
      |
      v (User types keystroke)
[ CompositionStarted (IME Active) ] <----+
      |                                  | (Candidate selection / Arrow keys)
      | (Enter confirms candidate)       |
      v                                  +--+
[ CompositionEnded (IME Committed) ] -------+
      |
      v (Clean Return Keypress without composition)
[ Message Dispatched ]
```

- **Invariant**:
  * If `ime_state == CompositionStarted`: Return/Enter key confirms the candidate selection and **MUST NOT** trigger message send.
  * Only when `ime_state == Idle` or `CompositionEnded` does Return/Enter trigger message dispatch.

### 4.2 Voice Record Gesture State Machine
- **Upstream Anchor**: `Telegram/SourceFiles/history/view/controls/history_view_voice_record_bar.style`, `media_audio_capture.cpp`
- **Exact Upstream Dimensions**:
  * Lock Dimension: `historyRecordLockSize = size(75px, 133px)`
  * Lock Icon: `historyRecordLockIconSize = size(14px, 17px)`
  * Slide Cancel Distance: `historyRecordTextWidthForWrap = 210px`
  * Lock Trigger Offset: 80px vertical drag upwards

```
[ Composer Idle (Mic Icon Visible) ]
               |
               v (Mouse Down / Long Press on Mic)
[ Recording Started ] <-----------------------------------------------------+
   - Start 16kHz audio capture                                              |
   - Display elapsed time (0:01...) and realtime waveform bars              |
   - Show "< Slide to Cancel" text (Left) and Lock Padlock (Up 133px)       |
               |                                                            |
   +-----------+-----------------------------------+                        |
   |                                               |                        |
   v (Drag Left >= 210px)                          v (Drag Up >= 80px)      |
[ Recording Cancelled ]                    [ Recording Locked ]             |
   - Delete temporary audio buffer            - User can release mouse      |
   - Discard waveform                         - Shows Stop, Pause, Send     |
   - Return to Composer Idle                  - Preview waveform scrubber   |
                                                   |                        |
               +-----------------------------------+                        |
               |                                                            |
               v (Click Trash Can)                 v (Click Send Button)    |
         [ Discarded ]                     [ Encoded 5-bit & Dispatched ]---+
```

---

## 5. Design Tokens & Upstream `.style` Metric Parity

### 5.1 Comprehensive Metrics Parity Table

| Metric Token | Upstream `.style` Source | Exact Upstream Value | GPUI Value / Rem Equivalent | Description |
|---|---|---|---|---|
| `navRailWidth` | Custom Shell | `52px` | `px(52.0)` | Left Primary Navigation Rail width |
| `dialogsSidebarWidthDefault`| `dialogs.style` | `288px` | `px(288.0)` | Default Chat Sidebar width |
| `dialogsSidebarWidthMin` | `dialogs.style` | `240px` | `px(240.0)` | Minimum resizable sidebar width |
| `dialogsSidebarWidthMax` | `dialogs.style` | `360px` | `px(360.0)` | Maximum resizable sidebar width |
| `contextInspectorWidth` | `info.style` | `320px` | `px(320.0)` | Right Context Inspector drawer width |
| `dialogsRowHeight` | `dialogs.style:91` | `62px` | `px(62.0)` | Height of each dialog row in uniform_list |
| `taggedDialogRowHeight`| `dialogs.style:115` | `72px` | `px(72.0)` | Height of dialog row with tag pill |
| `forumDialogRowHeight` | `dialogs.style:119` | `80px` | `px(80.0)` | Height of forum topics dialog row |
| `dialogsPhotoSize` | `dialogs.style:98` | `46px` | `px(46.0)` | Userpic avatar diameter in chat row |
| `dialogsRowPadding` | `dialogs.style:97` | `margins(10, 8, 10, 8)`| `px(10.0), px(8.0)` | Padding inside dialog row |
| `dialogsNameLeft` | `dialogs.style:99` | `68px` | `px(68.0)` | Left offset of chat title |
| `dialogsNameTop` | `dialogs.style:100` | `10px` | `px(10.0)` | Top offset of chat title |
| `dialogsTextLeft` | `dialogs.style:101` | `68px` | `px(68.0)` | Left offset of message preview |
| `dialogsTextTop` | `dialogs.style:102` | `34px` | `px(34.0)` | Top offset of message preview |
| `dialogsUnreadHeight` | `dialogs.style:79` | `19px` | `px(19.0)` | Height of unread badge pill |
| `dialogsUnreadPadding`| `dialogs.style:80` | `5px` | `px(5.0)` | Horizontal padding of unread badge |
| `msgMaxWidth` | `chat.style:14` | `430px` | `px(430.0)` | Maximum width of chat bubble |
| `msgMinWidth` | `chat.style:22` | `160px` | `px(160.0)` | Minimum width of chat bubble |
| `msgPadding` | `chat.style:25` | `margins(11, 8, 11, 8)`| `px(11.0), px(8.0)` | Inner padding of chat bubble |
| `bubbleRadiusLarge` | `chat.style:327` | `16px` | `px(16.0)` | Leading/outer bubble corner radius |
| `bubbleRadiusSmall` | `chat.style:326` | `4px` | `px(4.0)` | Attached/grouped bubble corner radius |
| `minPhotoSize` | `chat.style:168` | `100px` | `px(100.0)` | Minimum photo dimension |
| `maxMediaSize` | `chat.style:169` | `430px` | `px(430.0)` | Maximum photo/video dimension |
| `maxStickerSize` | `chat.style:170` | `224px` | `px(224.0)` | Maximum sticker dimension |
| `maxVideoMessageSize` | `chat.style:173` | `240px` | `px(240.0)` | Diameter of round video note |
| `historyRecordLockSize`| `voice_record_bar.style:24` | `size(75, 133)` | `px(75.0), px(133.0)` | Voice gesture padlock popup size |
| `historyRecordLockIconSize`| `voice_record_bar.style:25` | `size(14, 17)` | `px(14.0), px(17.0)` | Padlock icon dimension |
| `historyRecordTextWidthForWrap`| `voice_record_bar.style:20`| `210px` | `px(210.0)` | Distance to slide left to cancel voice |

---

### 5.2 Color Palettes: Day Blue & Night Schemes

Extracted directly from `Telegram/SourceFiles/window/themes/window_themes_embedded.cpp:353-395`:

```rust
pub struct PaletteTokens {
    pub window_bg: gpui::Rgba,
    pub window_fg: gpui::Rgba,
    pub accent: gpui::Rgba,
    pub bubble_in_bg: gpui::Rgba,
    pub bubble_in_fg: gpui::Rgba,
    pub bubble_out_bg: gpui::Rgba,
    pub bubble_out_fg: gpui::Rgba,
    pub dialogs_row_bg: gpui::Rgba,
    pub dialogs_row_hover: gpui::Rgba,
    pub dialogs_row_active: gpui::Rgba,
    pub unread_badge_bg: gpui::Rgba,
    pub unread_badge_fg: gpui::Rgba,
    pub unread_badge_muted: gpui::Rgba,
}

pub const DAY_BLUE_PALETTE: PaletteTokens = PaletteTokens {
    window_bg: gpui::rgb(0xffffff),
    window_fg: gpui::rgb(0x000000),
    accent: gpui::rgb(0x40a7e3),             // Upstream DayBlue accent
    bubble_in_bg: gpui::rgb(0xffffff),       // Upstream DayBlue received bubble
    bubble_in_fg: gpui::rgb(0x000000),
    bubble_out_bg: gpui::rgb(0xd7f0ff),      // Upstream DayBlue sent bubble
    bubble_out_fg: gpui::rgb(0x000000),
    dialogs_row_bg: gpui::rgb(0xffffff),
    dialogs_row_hover: gpui::rgb(0xf4f4f5),
    dialogs_row_active: gpui::rgb(0x40a7e3),  // Active row highlight
    unread_badge_bg: gpui::rgb(0x40a7e3),
    unread_badge_fg: gpui::rgb(0xffffff),
    unread_badge_muted: gpui::rgb(0xb0b0b0),
};

pub const NIGHT_PALETTE: PaletteTokens = PaletteTokens {
    window_bg: gpui::rgb(0x0e1621),          // Telegram dark background
    window_fg: gpui::rgb(0xf5f5f5),
    accent: gpui::rgb(0x5288c1),             // Upstream Night accent
    bubble_in_bg: gpui::rgb(0x182533),       // Upstream Night received bubble
    bubble_in_fg: gpui::rgb(0xf5f5f5),
    bubble_out_bg: gpui::rgb(0x2b5278),      // Upstream Night sent bubble
    bubble_out_fg: gpui::rgb(0xf5f5f5),
    dialogs_row_bg: gpui::rgb(0x0e1621),
    dialogs_row_hover: gpui::rgb(0x17212b),
    dialogs_row_active: gpui::rgb(0x2b5278),
    unread_badge_bg: gpui::rgb(0x5288c1),
    unread_badge_fg: gpui::rgb(0xffffff),
    unread_badge_muted: gpui::rgb(0x6b808d),
};
```

---

## 6. Overlays, Modals & Lightbox Layer

All modals in GPUI Telegram are managed by an authoritative `ModalManager` layer rendering on top of the 4-pane AppShell:

1. **Media Viewer Lightbox (`TCG-N10`)**:
   - Semi-transparent backdrop (`#000000` with 90% opacity).
   - Top action bar: Author info, timestamp, download button, zoom level, close button.
   - Smooth mouse wheel pinch-zoom and panning texture.
2. **VoIP Call Overlay (`TCG-N24` / `TCG-N25`)**:
   - Floating picture-in-picture window or full-screen view.
   - Active speaker animated waveform halo, mute controls, screen share toggle, end call red button.
3. **Settings Modal (`TCG-N31`)**:
   - Left section list: `Edit Profile`, `General`, `Notifications`, `Privacy & Security`, `Chat Settings`, `Folders`, `Advanced`, `Active Sessions`.
   - Right content panel rendering dedicated submodule views (`settings_chat.rs`, `settings_local_storage.rs`, etc.).
4. **MiniApps Sandboxed Frame (`TCG-N28`)**:
   - Fixed aspect ratio or full viewport drawer with custom top close bar and reload icon.

---

## 7. Verification Method & Acceptance Criteria

1. **Pixel Parity Visual Regression Tests**:
   - Headless GPUI snapshot tests verifying `defaultDialogRow` (62px), `photoSize` (46px), and `msgPadding`.
2. **IME Fencing Automated Tests**:
   - Simulates Japanese input composition; verifies that Return during composition does not send message.
3. **Voice Gesture Gesture Tests**:
   - Simulates mouse hold, drag left 210px (cancel), drag up 80px (lock), and verifies state transitions.
4. **Waveform 5-Bit Codec Roundtrip Tests**:
   - Encodes 100 random samples; decodes; asserts `decoded == original` with 0 bit errors.
