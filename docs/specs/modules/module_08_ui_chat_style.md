# Module 08 (ui & boxes) — Slice 5: Chat Styling, Badges & UI Controls Design Spec

## 1. Subsystem Architecture Overview

Module 08 Slice 5 governs the chat visual styling, bubble geometry, unread badge counters, userpic presentation, and conversational UI controls within the Telegram GPUI desktop client.
This specification derives directly from upstream Telegram Desktop C++ sources in:
- `Telegram/SourceFiles/ui/chat/` (69 files: chat style, theme, bubble margins, group call bars, attach preview pipeline)
- `Telegram/SourceFiles/ui/controls/` (94 files: action buttons, download progress bar, jump down badge, toast animations, ratings)
- `Telegram/SourceFiles/ui/` root styling helpers (23 files: cached round corner masks, unread counter formatting, userpic stacking, filter icons)

### Architectural Invariants & GPUI Translation
1. **Corner Rounding & Clipping Cache** (`cached_round_corners.rs`): Upstream `ui/cached_round_corners.cpp:L30-L53` caches 4 corner masks with 3x radius canvas. In GPUI, high-DPI scaling and corner radius paths (`CornersPixmaps`) are precomputed for `BoxCorners`, `MenuCorners`, `DateCorners`, `OverviewVideoCorners`, `ForwardCorners`, and document bubbles.
2. **Badge Capsule Metrics** (`unread_badge.rs`, `unread_badge_paint.rs`): Upstream `unread_badge_paint.cpp:L97-L134` constructs a capsule with two half-circles and a variable-width horizontal bar (`bar = width - 2 * (size / 2)`). It computes 6 palette states via `index = (muted ? 0x03 : 0x00) + (active ? 0x02 : (selected ? 0x01 : 0x00))`.
3. **Counter Formatting** (`unread_counter_format.rs`): Upstream `unread_counter_format.cpp:L10-L37` implements exact string representations: narrow overflow thresholds at `> 999` (`"99+"` with mentions/reactions) and `> 999999` (`"99999+"`), plus compact K/M/B metric prefixes.
4. **Userpic Stacking Geometry** (`userpic_view.rs`): Upstream `userpic_view.cpp:L18-L32` defines precise geometric layout for community card stacking: `kPeek = 0.3`, `kCover = 0.5`, `kPivotY = 0.75`, `kCard1Scale = 0.82`, `kCard1Angle = -9.0°`, `kCard1Radius = 0.22`, `kCard1Opacity = 0.5`, `kCard2Scale = 0.70`, `kCard2Angle = -16.0°`, `kCard2Radius = 0.16`, `kCard2Opacity = 0.3`, and `kGap = 0.02`. `ForumUserpicRadiusMultiplier()` returns `0.3`.
5. **Chat Style Radius & Metrics** (`chat_style.rs`): Upstream `chat_style.cpp:L94-L117` derives service bubble radius: `HistoryServiceMsgRadius() = minMessageHeight / 2`, `HistoryServiceMsgInvertedRadius() = minRowHeight - radius`, and `HistoryServiceMsgInvertedShrink() = (invertedRadius * 2) / 3`. Quote outline opacities: `kDefaultBgOpacity = 0.12`, `kDefaultOutline1Opacity = 0.90`, `kDefaultOutline2Opacity = 0.30`, `kDefaultOutline3Opacity = 0.60`.
6. **Group Call Bar Time Formatting** (`group_call_bar.rs`): Upstream `group_call_bar.cpp:L67-L106` formats countdowns: `left >= 86400s` as days, `hours > 0` as `HH:MM:SS`, otherwise `MM:SS`, prefixing `\u{2212}` (minus sign) when scheduled time is late.
7. **Attach Preparation & Albums** (`attach_prepare.rs`): Upstream `attach_prepare.cpp:L31-L33` enforces `kMaxAlbumCount = 10` items and `kStandardPhotoSideLimit = 1280` px standard photo constraint.

## 2. Upstream C++ Source Files Reading Manifest

### 2.1 Root UI Helpers & Styling (`Telegram/SourceFiles/ui/`)
- `Telegram/SourceFiles/ui/cached_round_corners.h` (82 lines): `CachedRoundCorners` enum (Box, Menu, Date, Overview, Forward, Doc1..4), `CornersPixmaps`, `CachedCornerRadius` tags (Small, ThumbSmall, ThumbLarge, BubbleSmall, BubbleLarge).
- `Telegram/SourceFiles/ui/cached_round_corners.cpp` (285 lines, `L30-L53, L113-L184, L190-L215`): Precomputed corner mask rasterization (`PrepareCorners` on 3r x (3r + s) canvas), 9-slice corner filling (`FillRoundRect`), and shadow offsets (`FillRoundShadow`). Target: `crates/gpui_telegram_desktop/src/components/ui/cached_round_corners.rs` (`CachedRoundCorners`).
- `Telegram/SourceFiles/ui/unread_badge.h` (119 lines): `UnreadBadge` widget interface, `BotVerifyDetails`, `PeerBadge::Descriptor`, `TextBadgeType` (Scam, Fake, Direct).
- `Telegram/SourceFiles/ui/unread_badge.cpp` (491 lines, `L27-L29, L130-L162, L173-L232`): `kPlayStatusLimit = 2`, `kBotVerifiedScale = 0.88`, badge baseline calculation `((size - font.height) / 2) + font.ascent`, scam/fake text badge drawing. Target: `crates/gpui_telegram_desktop/src/components/ui/unread_badge.rs` (`UnreadBadge`).
- `Telegram/SourceFiles/ui/unread_badge_paint.h` (51 lines): `UnreadBadgeSize` (Dialogs, MainMenu, HistoryToDown, StickersPanel, ReactionInDialogs, PollInDialogs, kCount = 8), `UnreadBadgeStyle`.
- `Telegram/SourceFiles/ui/unread_badge_paint.cpp` (193 lines, `L89-L134`): `ComputeUnreadBadgeText` truncation, capsule mask colorization with 6 state indices `(muted ? 0x03 : 0x00) + (active ? 0x02 : (selected ? 0x01 : 0x00))`. Target: `crates/gpui_telegram_desktop/src/components/ui/unread_badge_paint.rs` (`UnreadBadgePaint`).
- `Telegram/SourceFiles/ui/unread_counter_format.h` (15 lines): Signatures for `FormatUnreadCounter` and `FormatUnreadCounterShort`.
- `Telegram/SourceFiles/ui/unread_counter_format.cpp` (37 lines, `L10-L37`): Narrow threshold `99+` (>999 with mention) and `99999+` (>999999), metric scaling `K`, `M`, `B`. Target: `crates/gpui_telegram_desktop/src/components/ui/unread_counter_format.rs` (`UnreadCounterFormat`).
- `Telegram/SourceFiles/ui/userpic_view.h` (67 lines): `PeerUserpicView`, `PeerUserpicShape` (Auto, Circle, Forum, Monoforum), `CommunityUserpicEffect`.
- `Telegram/SourceFiles/ui/userpic_view.cpp` (196 lines, `L18-L32, L34-L37, L38-L128`): Community card fan rotation: `kPeek = 0.3`, `kCover = 0.5`, `kPivotY = 0.75`, `kCard1Scale = 0.82`, `kCard1Angle = -9.0°`, `kCard1Radius = 0.22`, `kCard2Scale = 0.70`, `kCard2Angle = -16.0°`, `kCard2Radius = 0.16`, `ForumUserpicRadiusMultiplier() = 0.3`. Target: `crates/gpui_telegram_desktop/src/components/ui/userpic_view.rs` (`UserpicView`).
- `Telegram/SourceFiles/ui/empty_userpic.h` (158 lines): `EmptyUserpic` procedural fallback avatar generator with color indices and initial monogram.
- `Telegram/SourceFiles/ui/empty_userpic.cpp` (728 lines, `L50-L120`): Monogram font measurement, gradient generation from peer ID hash. Target: `crates/gpui_telegram_desktop/src/components/ui/empty_userpic.rs` (`EmptyUserpic`).
- `Telegram/SourceFiles/ui/filter_icon_panel.h` (95 lines): `FilterIconPanel`, popup animation, `FilterIconChosen`.
- `Telegram/SourceFiles/ui/filter_icon_panel.cpp` (477 lines, `L25-L67, L108-L125`): `kHideTimeoutMs = 300`, `kIconsPerRow = 6`, array of 30 icons (Cat, Book, Money, Game, etc.). Target: `crates/gpui_telegram_desktop/src/components/ui/filter_icon_panel.rs` (`FilterIconPanel`).
- `Telegram/SourceFiles/ui/filter_icons.h` (81 lines): Filter icon enumeration and icon paint routines.
- `Telegram/SourceFiles/ui/filter_icons.cpp` (341 lines, `L40-L110`): SVG icon rendering with custom palette tinting. Target: `crates/gpui_telegram_desktop/src/components/ui/filter_icons.rs` (`FilterIcons`).
- `Telegram/SourceFiles/ui/filter_icons.style` (132 lines): Style metrics for folder icons. Target: `crates/gpui_telegram_desktop/src/theme/ui/filter_icons_style.rs`.
- `Telegram/SourceFiles/ui/color_contrast.h` (28 lines): Contrast ratio calculation helpers.
- `Telegram/SourceFiles/ui/color_contrast.cpp` (140 lines, `L20-L80`): WCAG luminance calculation for readability. Target: `crates/gpui_telegram_desktop/src/components/ui/color_contrast.rs` (`ColorContrast`).
- `Telegram/SourceFiles/ui/color_int_conversion.h` (24 lines): RGB int bit-packing converters.
- `Telegram/SourceFiles/ui/color_int_conversion.cpp` (65 lines, `L10-L40`): Hex string and integer conversions. Target: `crates/gpui_telegram_desktop/src/components/ui/color_int_conversion.rs` (`ColorIntConversion`).
- `Telegram/SourceFiles/ui/color_indices.style` (45 lines): Peer color palette indices. Target: `crates/gpui_telegram_desktop/src/theme/ui/color_indices_style.rs`.
- `Telegram/SourceFiles/ui/menu_icons.style` (60 lines): Context menu icon styles. Target: `crates/gpui_telegram_desktop/src/theme/ui/menu_icons_style.rs`.
- `Telegram/SourceFiles/ui/td_common.style` (70 lines): Common desktop styles. Target: `crates/gpui_telegram_desktop/src/theme/ui/td_common_style.rs`.

### 2.2 Chat Visual Styling & Bars (`Telegram/SourceFiles/ui/chat/`)
- `Telegram/SourceFiles/ui/chat/chat.style` (1120 lines): Comprehensive styles for chat bubbles, history items, media blocks, service messages. Target: `crates/gpui_telegram_desktop/src/theme/ui/chat_style_tokens.rs`.
- `Telegram/SourceFiles/ui/chat/chat_style.h` (831 lines): Core definitions for `ChatStyle`, `MessageStyle`, `MessageImageStyle`, `ChatPaintContext`.
- `Telegram/SourceFiles/ui/chat/chat_style.cpp` (1632 lines, `L56-L70, L94-L117, L119-L141`): Quote styling opacities (`kDefaultBgOpacity = 0.12`, `kDefaultOutline1Opacity = 0.9`, etc.), service message bubble radius, inverted radius, and outline palette indices. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/chat_style.rs` (`ChatStyleChat`).
- `Telegram/SourceFiles/ui/chat/chat_style.style` (410 lines): Visual layout definitions for chat styling. Target: `crates/gpui_telegram_desktop/src/theme/ui/chat_style_metrics.rs`.
- `Telegram/SourceFiles/ui/chat/chat_style_radius.h` (25 lines): Radius calculation helper definitions.
- `Telegram/SourceFiles/ui/chat/chat_style_radius.cpp` (55 lines, `L15-L45`): Message bubble corner radius computation with responsive scale. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/chat_style_radius.rs` (`ChatStyleRadiusChat`).
- `Telegram/SourceFiles/ui/chat/chat_theme.h` (185 lines): `ChatTheme` struct with background image, pattern, palette overrides.
- `Telegram/SourceFiles/ui/chat/chat_theme.cpp` (890 lines, `L40-L120`): Wallpaper background tiling, parallax blur, and theme transitions. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/chat_theme.rs` (`ChatThemeChat`).
- `Telegram/SourceFiles/ui/chat/chat_theme_readability.h` (35 lines): Theme readability checks for dark/light contrast.
- `Telegram/SourceFiles/ui/chat/chat_theme_readability.cpp` (380 lines, `L30-L90`): Luminance validation and bubble contrast adjustment. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/chat_theme_readability.rs` (`ChatThemeReadabilityChat`).
- `Telegram/SourceFiles/ui/chat/chats_filter_tag.h` (30 lines): Filter tag pill drawing interface.
- `Telegram/SourceFiles/ui/chat/chats_filter_tag.cpp` (215 lines, `L20-L75`): Folder pill tag geometry and color badge render. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/chats_filter_tag.rs` (`ChatsFilterTagChat`).
- `Telegram/SourceFiles/ui/chat/choose_send_as.h` (45 lines): Send-as peer chooser dropdown.
- `Telegram/SourceFiles/ui/chat/choose_send_as.cpp` (245 lines, `L30-L95`): Dropdown panel for switching author identity. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/choose_send_as.rs` (`ChooseSendAsChat`).
- `Telegram/SourceFiles/ui/chat/choose_theme_controller.h` (65 lines): Theme preview and selection sheet controller.
- `Telegram/SourceFiles/ui/chat/choose_theme_controller.cpp` (590 lines, `L50-L140`): Carousel theme switcher with live wallpaper rendering. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/choose_theme_controller.rs` (`ChooseThemeControllerChat`).
- `Telegram/SourceFiles/ui/chat/forward_options_box.h` (25 lines): Forward options dialog interface.
- `Telegram/SourceFiles/ui/chat/forward_options_box.cpp` (55 lines, `L15-L48`): Forwarding modal with hide-author / hide-caption options. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/forward_options_box.rs` (`ForwardOptionsBoxChat`).
- `Telegram/SourceFiles/ui/chat/group_call_bar.h` (121 lines): `GroupCallBarContent`, `GroupCallScheduledLeft`, `GroupCallBar`.
- `Telegram/SourceFiles/ui/chat/group_call_bar.cpp` (461 lines, `L67-L106, L110-L160`): Real-time countdown timer formatting (`days`, `HH:MM:SS`, `MM:SS`, late `\u{2212}` minus sign), slide-in animation. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/group_call_bar.rs` (`GroupCallBarChat`).
- `Telegram/SourceFiles/ui/chat/group_call_userpics.h` (50 lines): Overlapping avatar cluster for voice call bars.
- `Telegram/SourceFiles/ui/chat/group_call_userpics.cpp` (310 lines, `L30-L95`): Animated blob waves and speaker pulse glow. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/group_call_userpics.rs` (`GroupCallUserpicsChat`).
- `Telegram/SourceFiles/ui/chat/message_bar.h` (75 lines): Persistent pinned / replied banner bar above composer.
- `Telegram/SourceFiles/ui/chat/message_bar.cpp` (460 lines, `L40-L120`): Slide-wrap container with dismiss button and reply thumbnail. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/message_bar.rs` (`MessageBarChat`).
- `Telegram/SourceFiles/ui/chat/message_bubble.h` (110 lines): Bubble outline paths, corners, and tail geometry.
- `Telegram/SourceFiles/ui/chat/message_bubble.cpp` (410 lines, `L30-L115`): Bubble path rasterization with incoming/outgoing tails. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/message_bubble.rs` (`MessageBubbleChat`).
- `Telegram/SourceFiles/ui/chat/more_chats_bar.h` (45 lines): Forum / folder "More Chats" banner.
- `Telegram/SourceFiles/ui/chat/more_chats_bar.cpp` (145 lines, `L20-L65`): Multi-channel unread indicator bar. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/more_chats_bar.rs` (`MoreChatsBarChat`).
- `Telegram/SourceFiles/ui/chat/pinned_bar.h` (55 lines): Pinned message index bar with sequential browsing.
- `Telegram/SourceFiles/ui/chat/pinned_bar.cpp` (195 lines, `L25-L80`): Multiple pinned message switcher with smooth fade. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/pinned_bar.rs` (`PinnedBarChat`).
- `Telegram/SourceFiles/ui/chat/requests_bar.h` (50 lines): Channel join request banner bar.
- `Telegram/SourceFiles/ui/chat/requests_bar.cpp` (170 lines, `L25-L70`): Pending subscriber requests notification bar. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/requests_bar.rs` (`RequestsBarChat`).
- `Telegram/SourceFiles/ui/chat/sponsored_message_bar.h` (25 lines): Sponsored promotional message bar.
- `Telegram/SourceFiles/ui/chat/sponsored_message_bar.cpp` (290 lines, `L35-L95`): Ad tag banner with report / hide controls. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/sponsored_message_bar.rs` (`SponsoredMessageBarChat`).
- `Telegram/SourceFiles/ui/chat/torn_edge.h` (25 lines): Jagged paper edge separator for IV and long text snippets.
- `Telegram/SourceFiles/ui/chat/torn_edge.cpp` (60 lines, `L15-L45`): Procedural tooth path generator. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/torn_edge.rs` (`TornEdgeChat`).
- `Telegram/SourceFiles/ui/chat/unsupported_notice.h` (30 lines): Future message protocol fallback notice banner.
- `Telegram/SourceFiles/ui/chat/unsupported_notice.cpp` (135 lines, `L20-L60`): Fallback warning box with update link. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/unsupported_notice.rs` (`UnsupportedNoticeChat`).

### 2.3 Attach Preview & Preparation (`Telegram/SourceFiles/ui/chat/attach/`)
- `Telegram/SourceFiles/ui/chat/attach/attach_prepare.h` (223 lines): `PreparedFile`, `PreparedList`, `PreparedGroup`, `PreparedBundle`, `AlbumType`.
- `Telegram/SourceFiles/ui/chat/attach/attach_prepare.cpp` (688 lines, `L31-L33, L53-L104, L106-L135`): `kMaxAlbumCount = 10`, `kStandardPhotoSideLimit = 1280`, `HighQualityBadgeImage ("HD")`, `AnimatedBadgeImage ("GIF")`, album partitioning (`DivideByGroups`). Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_prepare.rs` (`AttachPrepareChatAttach`).
- `Telegram/SourceFiles/ui/chat/attach/attach_controls.h` (54 lines): Attachment control buttons (rotate, crop, edit, delete).
- `Telegram/SourceFiles/ui/chat/attach/attach_controls.cpp` (137 lines, `L20-L75`): Quick edit action strip for media attachments. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_controls.rs` (`AttachControlsChatAttach`).
- `Telegram/SourceFiles/ui/chat/attach/attach_album_preview.h` (142 lines): Multi-item media album grid layout previewer.
- `Telegram/SourceFiles/ui/chat/attach/attach_album_preview.cpp` (715 lines, `L40-L160`): Drag-and-drop reordering and collage grid calculation. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_album_preview.rs` (`AttachAlbumPreviewChatAttach`).
- `Telegram/SourceFiles/ui/chat/attach/attach_album_thumbnail.h` (155 lines): Individual thumbnail item inside album preview.
- `Telegram/SourceFiles/ui/chat/attach/attach_album_thumbnail.cpp` (770 lines, `L35-L120`): Thumbnail crop, spoiler mask, and TTL badge overlay. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_album_thumbnail.rs` (`AttachAlbumThumbnailChatAttach`).
- `Telegram/SourceFiles/ui/chat/attach/attach_abstract_single_preview.h` (24 lines): Abstract base class for single attachment previews. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_abstract_single_preview.rs` (`AttachAbstractSinglePreviewChatAttach`).
- `Telegram/SourceFiles/ui/chat/attach/attach_abstract_single_file_preview.h` (105 lines): Base class for document attachment preview.
- `Telegram/SourceFiles/ui/chat/attach/attach_abstract_single_file_preview.cpp` (390 lines, `L30-L110`): Document metadata extraction and progress indicator. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_abstract_single_file_preview.rs` (`AttachAbstractSingleFilePreviewChatAttach`).
- `Telegram/SourceFiles/ui/chat/attach/attach_abstract_single_media_preview.h` (104 lines): Base class for image/video attachment preview.
- `Telegram/SourceFiles/ui/chat/attach/attach_abstract_single_media_preview.cpp` (358 lines, `L35-L120`): Aspect ratio scaling and thumbnail generation. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_abstract_single_media_preview.rs` (`AttachAbstractSingleMediaPreviewChatAttach`).
- `Telegram/SourceFiles/ui/chat/attach/attach_single_file_preview.h` (41 lines): Single document attachment view.
- `Telegram/SourceFiles/ui/chat/attach/attach_single_file_preview.cpp` (125 lines, `L20-L70`): File icon and file name layout. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_single_file_preview.rs` (`AttachSingleFilePreviewChatAttach`).
- `Telegram/SourceFiles/ui/chat/attach/attach_single_media_preview.h` (45 lines): Single photo/video attachment view.
- `Telegram/SourceFiles/ui/chat/attach/attach_single_media_preview.cpp` (165 lines, `L25-L80`): Large media thumbnail with dimension constraints. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_single_media_preview.rs` (`AttachSingleMediaPreviewChatAttach`).
- `Telegram/SourceFiles/ui/chat/attach/attach_item_single_file_preview.h` (41 lines): Item-level document wrapper.
- `Telegram/SourceFiles/ui/chat/attach/attach_item_single_file_preview.cpp` (115 lines, `L20-L65`): Single document bubble geometry. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_item_single_file_preview.rs` (`AttachItemSingleFilePreviewChatAttach`).
- `Telegram/SourceFiles/ui/chat/attach/attach_item_single_media_preview.h` (71 lines): Item-level photo/video wrapper.
- `Telegram/SourceFiles/ui/chat/attach/attach_item_single_media_preview.cpp` (238 lines, `L30-L90`): Media bubble geometry with interactive actions. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_item_single_media_preview.rs` (`AttachItemSingleMediaPreviewChatAttach`).
- `Telegram/SourceFiles/ui/chat/attach/attach_send_files_way.h` (75 lines): File compression choice (`SendFilesWay`: AsPhotos, AsFiles, etc.).
- `Telegram/SourceFiles/ui/chat/attach/attach_send_files_way.cpp` (65 lines, `L15-L50`): Option toggles for album grouping and photo compression. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_send_files_way.rs` (`AttachSendFilesWayChatAttach`).
- `Telegram/SourceFiles/ui/chat/attach/attach_extensions.h` (14 lines): Extension checking utility.
- `Telegram/SourceFiles/ui/chat/attach/attach_extensions.cpp` (30 lines, `L10-L28`): MIME type and file extension matching. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_extensions.rs` (`AttachExtensionsChatAttach`).
- `Telegram/SourceFiles/ui/chat/attach/attach_bot_downloads.h` (47 lines): WebApp file download bar.
- `Telegram/SourceFiles/ui/chat/attach/attach_bot_downloads.cpp` (268 lines, `L30-L90`): WebApp document download progress. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_bot_downloads.rs` (`AttachBotDownloadsChatAttach`).
- `Telegram/SourceFiles/ui/chat/attach/attach_bot_webview.h` (393 lines): Bot Mini App full-screen and sheet webview host.
- `Telegram/SourceFiles/ui/chat/attach/attach_bot_webview.cpp` (3951 lines, `L100-L250`): Webview bridge, theme synchronization, fullscreen gestures. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_bot_webview.rs` (`AttachBotWebviewChatAttach`).
- `Telegram/SourceFiles/ui/chat/attach/attach_bot_webview_linux_shell.h` (96 lines): Linux webview fallback host interface.
- `Telegram/SourceFiles/ui/chat/attach/attach_bot_webview_linux_shell.cpp` (239 lines, `L30-L100`): Linux webkit2gtk process containment. Target: `crates/gpui_telegram_desktop/src/components/ui/chat/attach/attach_bot_webview_linux_shell.rs` (`AttachBotWebviewLinuxShellChatAttach`).

### 2.4 Interactive Controls & Widgets (`Telegram/SourceFiles/ui/controls/`)
- `Telegram/SourceFiles/ui/controls/button_busy.h` (41 lines): Button busy spinner overlay.
- `Telegram/SourceFiles/ui/controls/button_busy.cpp` (79 lines, `L15-L50`): Indeterminate progress spinner on primary buttons. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/button_busy.rs` (`ButtonBusyControls`).
- `Telegram/SourceFiles/ui/controls/button_context_menu.h` (35 lines): Context menu binding helper for UI buttons.
- `Telegram/SourceFiles/ui/controls/button_context_menu.cpp` (55 lines, `L15-L45`): Right-click and long-press menu triggering. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/button_context_menu.rs` (`ButtonContextMenuControls`).
- `Telegram/SourceFiles/ui/controls/button_labels.h` (35 lines): Multi-line button label helpers.
- `Telegram/SourceFiles/ui/controls/button_labels.cpp` (65 lines, `L15-L50`): Title and subtitle alignment within action buttons. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/button_labels.rs` (`ButtonLabelsControls`).
- `Telegram/SourceFiles/ui/controls/call_button.h` (65 lines): Call control button with circular ripple and icon swap.
- `Telegram/SourceFiles/ui/controls/call_button.cpp` (140 lines, `L25-L80`): End call, accept, and video switch buttons. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/call_button.rs` (`CallButtonControls`).
- `Telegram/SourceFiles/ui/controls/call_mute_button.h` (50 lines): Microphone toggle button with volume levels.
- `Telegram/SourceFiles/ui/controls/call_mute_button.cpp` (175 lines, `L30-L85`): Dynamic speaking wave glow around mute button. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/call_mute_button.rs` (`CallMuteButtonControls`).
- `Telegram/SourceFiles/ui/controls/chat_service_checkbox.h` (30 lines): Styled checkbox for message service dialogues.
- `Telegram/SourceFiles/ui/controls/chat_service_checkbox.cpp` (75 lines, `L15-L55`): Service checkbox with custom checkmark animation. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/chat_service_checkbox.rs` (`ChatServiceCheckboxControls`).
- `Telegram/SourceFiles/ui/controls/compose_ai_button_factory.h` (30 lines): AI assistant quick-prompt button generator.
- `Telegram/SourceFiles/ui/controls/compose_ai_button_factory.cpp` (85 lines, `L20-L65`): Gradient-bordered action pill for AI rewrites. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/compose_ai_button_factory.rs` (`ComposeAiButtonFactoryControls`).
- `Telegram/SourceFiles/ui/controls/custom_emoji_toast_icon.h` (35 lines): Custom emoji icon renderer for toast popups.
- `Telegram/SourceFiles/ui/controls/custom_emoji_toast_icon.cpp` (95 lines, `L20-L70`): Toast icon layout with custom sticker cache. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/custom_emoji_toast_icon.rs` (`CustomEmojiToastIconControls`).
- `Telegram/SourceFiles/ui/controls/delete_message_context_action.h` (35 lines): Action handler for message deletion confirmations.
- `Telegram/SourceFiles/ui/controls/delete_message_context_action.cpp` (110 lines, `L25-L80`): Multi-select delete action prompt. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/delete_message_context_action.rs` (`DeleteMessageContextActionControls`).
- `Telegram/SourceFiles/ui/controls/download_bar.h` (78 lines): `DownloadBarProgress`, `DownloadBarContent`, `DownloadBar`.
- `Telegram/SourceFiles/ui/controls/download_bar.cpp` (313 lines, `L21-L30, L67-L93, L95-L128`): Circular radial animation, thumbnail cache key matching, and finished state fade. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/download_bar.rs` (`DownloadBarControls`).
- `Telegram/SourceFiles/ui/controls/dynamic_images_strip.h` (60 lines): Horizontal scrollable strip of dynamic image previews.
- `Telegram/SourceFiles/ui/controls/dynamic_images_strip.cpp` (280 lines, `L30-L100`): Kinetic scrolling and item selection callback. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/dynamic_images_strip.rs` (`DynamicImagesStripControls`).
- `Telegram/SourceFiles/ui/controls/emoji_button.h` (50 lines): Emoji picker toggle button next to composer input.
- `Telegram/SourceFiles/ui/controls/emoji_button.cpp` (160 lines, `L25-L85`): Hover and active state icons with color transitions. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/emoji_button.rs` (`EmojiButtonControls`).
- `Telegram/SourceFiles/ui/controls/emoji_button_factory.h` (30 lines): Factory helpers for composer accessory buttons.
- `Telegram/SourceFiles/ui/controls/emoji_button_factory.cpp` (75 lines, `L15-L55`): Standard size and icon instantiator. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/emoji_button_factory.rs` (`EmojiButtonFactoryControls`).
- `Telegram/SourceFiles/ui/controls/feature_list.h` (45 lines): Feature bullet list view with check icons.
- `Telegram/SourceFiles/ui/controls/feature_list.cpp` (140 lines, `L25-L75`): Premium / boost feature item layout with custom bullets. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/feature_list.rs` (`FeatureListControls`).
- `Telegram/SourceFiles/ui/controls/filter_link_header.h` (35 lines): Filter share link header box.
- `Telegram/SourceFiles/ui/controls/filter_link_header.cpp` (115 lines, `L20-L65`): Folder name and chat count header. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/filter_link_header.rs` (`FilterLinkHeaderControls`).
- `Telegram/SourceFiles/ui/controls/filter_link_header.style` (45 lines): Style metrics for filter link headers. Target: `crates/gpui_telegram_desktop/src/theme/ui/filter_link_header_style.rs`.
- `Telegram/SourceFiles/ui/controls/invite_link_buttons.h` (40 lines): Copy / share action buttons for invite links.
- `Telegram/SourceFiles/ui/controls/invite_link_buttons.cpp` (130 lines, `L20-L75`): Secondary button group with ripple and toast trigger. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/invite_link_buttons.rs` (`InviteLinkButtonsControls`).
- `Telegram/SourceFiles/ui/controls/invite_link_label.h` (35 lines): Invite link URL pill with click-to-copy.
- `Telegram/SourceFiles/ui/controls/invite_link_label.cpp` (95 lines, `L20-L60`): Truncated link presentation and hover background. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/invite_link_label.rs` (`InviteLinkLabelControls`).
- `Telegram/SourceFiles/ui/controls/jump_down_button.h` (37 lines): Scroll to bottom floating action button with unread count.
- `Telegram/SourceFiles/ui/controls/jump_down_button.cpp` (68 lines, `L27-L35, L48-L58`): Ellipse ripple mask, bottom badge paint (`UnreadBadgeSize::HistoryToDown`, allowDigits = 4). Target: `crates/gpui_telegram_desktop/src/components/ui/controls/jump_down_button.rs` (`JumpDownButtonControls`).
- `Telegram/SourceFiles/ui/controls/keys_legend.h` (30 lines): Keyboard shortcut hints legend.
- `Telegram/SourceFiles/ui/controls/keys_legend.cpp` (85 lines, `L15-L50`): Shortcut key tag rendering. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/keys_legend.rs` (`KeysLegendControls`).
- `Telegram/SourceFiles/ui/controls/labeled_emoji_tabs.h` (55 lines): Emoji / Sticker / GIF tab strip with text badges.
- `Telegram/SourceFiles/ui/controls/labeled_emoji_tabs.cpp` (220 lines, `L30-L90`): Sliding indicator and tab activation. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/labeled_emoji_tabs.rs` (`LabeledEmojiTabsControls`).
- `Telegram/SourceFiles/ui/controls/location_picker.h` (50 lines): Map location selection control.
- `Telegram/SourceFiles/ui/controls/location_picker.cpp` (210 lines, `L30-L95`): Pin crosshairs and coordinate resolution. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/location_picker.rs` (`LocationPickerControls`).
- `Telegram/SourceFiles/ui/controls/peer_list_dummy.h` (25 lines): Dummy layout stub for peer list placeholders.
- `Telegram/SourceFiles/ui/controls/peer_list_dummy.cpp` (45 lines, `L10-L35`): Shimmering skeleton rows for contact lists. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/peer_list_dummy.rs` (`PeerListDummyControls`).

- `Telegram/SourceFiles/ui/controls/popup_selector.h` (45 lines): Floating popup selector for options.
- `Telegram/SourceFiles/ui/controls/popup_selector.cpp` (160 lines, `L25-L75`): Anchored menu popup with arrow indicator. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/popup_selector.rs` (`PopupSelectorControls`).
- `Telegram/SourceFiles/ui/controls/round_video_recorder.h` (85 lines): Circular video note camera recorder widget.
- `Telegram/SourceFiles/ui/controls/round_video_recorder.cpp` (510 lines, `L40-L150`): Camera viewfinder, circular clipping mask, recording progress ring. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/round_video_recorder.rs` (`RoundVideoRecorderControls`).
- `Telegram/SourceFiles/ui/controls/round_video_recorder_data.h` (35 lines): Data models for video note recording state. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/round_video_recorder_data.rs` (`RoundVideoRecorderDataControls`).
- `Telegram/SourceFiles/ui/controls/send_as_button.h` (40 lines): Switch send identity button beside composer.
- `Telegram/SourceFiles/ui/controls/send_as_button.cpp` (71 lines, `L22-L37, L39-L68`): Avatar thumbnail with active cross-fade animation. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/send_as_button.rs` (`SendAsButtonControls`).
- `Telegram/SourceFiles/ui/controls/send_button.h` (157 lines): `SendButton::Type` (Send, Stop, Schedule, Save, Record, Round, Cancel, Slowmode, EditPrice), `RippleShape`.
- `Telegram/SourceFiles/ui/controls/send_button.cpp` (713 lines, `L24-L28, L66-L83, L91-L111`): `kWideScale = 5`, `kForbiddenOpacity = 0.5`, slowmode delay timer text (`MM:SS`), voice/round transition icon animations. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/send_button.rs` (`SendButtonControls`).
- `Telegram/SourceFiles/ui/controls/silent_toggle.h` (52 lines): Channel silent post bell toggle button.
- `Telegram/SourceFiles/ui/controls/silent_toggle.cpp` (119 lines, `L20-L38, L40-L54, L82-L86`): Checked state toggle, tooltip positioning, broadcast accessibility role. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/silent_toggle.rs` (`SilentToggleControls`).
- `Telegram/SourceFiles/ui/controls/stars_rating.h` (80 lines): Stars rating interactive widget and popup dialog.
- `Telegram/SourceFiles/ui/controls/stars_rating.cpp` (552 lines, `L40-L60, L62-L80, L95-L132`): Rating level limits, number abbreviations (<10K decimal, <10M 'K', >=10M 'M'), custom text/shape colors. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/stars_rating.rs` (`StarsRatingControls`).
- `Telegram/SourceFiles/ui/controls/sub_tabs.h` (45 lines): Nested tab strip component.
- `Telegram/SourceFiles/ui/controls/sub_tabs.cpp` (175 lines, `L25-L80`): Tab item switching and pill underline animation. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/sub_tabs.rs` (`SubTabsControls`).
- `Telegram/SourceFiles/ui/controls/subsection_tabs_slider.h` (65 lines): Sliding tabs slider for profile subsections.
- `Telegram/SourceFiles/ui/controls/subsection_tabs_slider.cpp` (290 lines, `L30-L110`): Smooth slide transitions between subsection tabs. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/subsection_tabs_slider.rs` (`SubsectionTabsSliderControls`).
- `Telegram/SourceFiles/ui/controls/subsection_tabs_slider_reorder.h` (40 lines): Reorderable tab header strip.
- `Telegram/SourceFiles/ui/controls/subsection_tabs_slider_reorder.cpp` (180 lines, `L25-L85`): Drag-to-reorder tabs interaction. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/subsection_tabs_slider_reorder.rs` (`SubsectionTabsSliderReorderControls`).
- `Telegram/SourceFiles/ui/controls/swipe_handler.h` (55 lines): Touch / trackpad horizontal message swipe action handler.
- `Telegram/SourceFiles/ui/controls/swipe_handler.cpp` (240 lines, `L30-L100`): Reply gesture thresholding, rubber-band resistance curve, haptic tick trigger. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/swipe_handler.rs` (`SwipeHandlerControls`).
- `Telegram/SourceFiles/ui/controls/swipe_handler_data.h` (45 lines): Data models for swipe coordinates and progress. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/swipe_handler_data.rs` (`SwipeHandlerDataControls`).
- `Telegram/SourceFiles/ui/controls/tabbed_search.h` (162 lines): `TabbedSearch`, `SearchWithGroups`, `EmojiGroup`.
- `Telegram/SourceFiles/ui/controls/tabbed_search.cpp` (908 lines, `L31-L33, L90-L95`): `kDebounceTimeout = 400ms`, `kCategoryIconSizeOverride = 22px`, debounced emoji search query producer. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/tabbed_search.rs` (`TabbedSearchControls`).
- `Telegram/SourceFiles/ui/controls/table_rows.h` (99 lines): Table layout rows for gifting / transactions.
- `Telegram/SourceFiles/ui/controls/table_rows.cpp` (380 lines, `L30-L110`): Key-value pair alignment, peer item integration, tooltip anchors. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/table_rows.rs` (`TableRowsControls`).
- `Telegram/SourceFiles/ui/controls/text_copied_toast.h` (42 lines): Animated toast popup when copying chat text.
- `Telegram/SourceFiles/ui/controls/text_copied_toast.cpp` (185 lines, `L25-L27, L77-L108, L110-L182`): `kExpandDuration = 300ms`, `kExpandWidthShare = 0.6`, cubic ease-out expand, fade-in label, auto-dismiss timer. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/text_copied_toast.rs` (`TextCopiedToastControls`).
- `Telegram/SourceFiles/ui/controls/title_sub_widget.h` (25 lines): Auxiliary widget slot inside top bar header.
- `Telegram/SourceFiles/ui/controls/title_sub_widget.cpp` (111 lines, `L20-L65`): Dynamic title bar widget container. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/title_sub_widget.rs` (`TitleSubWidgetControls`).
- `Telegram/SourceFiles/ui/controls/ton_common.h` (131 lines): TON cryptocurrency UI helpers and wallet badge styles.
- `Telegram/SourceFiles/ui/controls/ton_common.cpp` (460 lines, `L40-L140`): TON diamond badge formatting, address shortening, nanotons conversions. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/ton_common.rs` (`TonCommonControls`).
- `Telegram/SourceFiles/ui/controls/ttl_media.h` (54 lines): Self-destructing media countdown overlay.
- `Telegram/SourceFiles/ui/controls/ttl_media.cpp` (218 lines, `L30-L90`): Pie timer sweep animation over vanishing photos/videos. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/ttl_media.rs` (`TtlMediaControls`).
- `Telegram/SourceFiles/ui/controls/userpic_button.h` (236 lines): Interactive circular avatar button with edit badge.
- `Telegram/SourceFiles/ui/controls/userpic_button.cpp` (1338 lines, `L80-L220`): Live camera capture overlay, photo cropping, progress indicator. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/userpic_button.rs` (`UserpicButtonControls`).
- `Telegram/SourceFiles/ui/controls/warning_tooltip.h` (58 lines): Floating caution tooltip bubble.
- `Telegram/SourceFiles/ui/controls/warning_tooltip.cpp` (112 lines, `L22-L24, L33-L94`): `kDefaultWarningTooltipDuration = 2000ms`, pointAt anchor alignment, animated show/hide. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/warning_tooltip.rs` (`WarningTooltipControls`).
- `Telegram/SourceFiles/ui/controls/who_reacted_context_action.h` (184 lines): Detailed message reaction user list dropdown.
- `Telegram/SourceFiles/ui/controls/who_reacted_context_action.cpp` (1437 lines, `L100-L250`): Reaction emoji tabs, user avatars, custom animation playback. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/who_reacted_context_action.rs` (`WhoReactedContextActionControls`).
- `Telegram/SourceFiles/ui/controls/window_outdated_bar.h` (20 lines): Deprecated operating system banner.
- `Telegram/SourceFiles/ui/controls/window_outdated_bar.cpp` (180 lines, `L23-L26, L64-L70, L105-L134`): `kMinimalSkip = 7`, `kSoonSkip = 30`, `kNowSkip = 90`, YYYYMMDD state storage, dismissal suppression. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/window_outdated_bar.rs` (`WindowOutdatedBarControls`).
- `Telegram/SourceFiles/ui/controls/window_outdated_bar_dummy.cpp` (18 lines): Non-supported platform empty stub. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/window_outdated_bar_dummy.rs` (`WindowOutdatedBarDummyControls`).
- `Telegram/SourceFiles/ui/controls/window_screen_reader_bar.h` (19 lines): Accessibility screen reader status bar.
- `Telegram/SourceFiles/ui/controls/window_screen_reader_bar.cpp` (218 lines, `L25-L85`): ARIA announcement banner for visually impaired users. Target: `crates/gpui_telegram_desktop/src/components/ui/controls/window_screen_reader_bar.rs` (`WindowScreenReaderBarControls`).
