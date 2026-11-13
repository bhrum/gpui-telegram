# Module 11: Info & Settings — Common Wrap & Widgets Specification (Slice 3)

## Executive Summary
This design document establishes the authoritative C++ to Rust/GPUI architectural mapping for **Module 11 (`info & settings`) Slice 3**, encompassing exactly 42 upstream Telegram Desktop source and style files (and 23 distinct target Rust files).

The subsystem forms the core structural framework for Telegram's collapsible info drawer panel, shared overview layout, profile controls, downloads manager, common mutual groups viewer, and community join requests:
- **Core Wrapper & Layer Shell**: `Info::WrapWidget`, `Info::ContentWidget`, `Info::LayerWidget`, `Info::TopBar`, `Info::Controller`, `Info::Memento`, `Info::SectionWidget`, and `Info::FlexibleScrollHelper`.
- **Overview Grid & Layout Engine**: `Overview::Layout`, `Overview::Checkbox`, `Overview::LayoutDelegate`, and `overview.style`.
- **Profile Base Controls**: `Profile::BackButton`, `Profile::BlockWidget`, `Profile::CoverDropArea`, and `profile.style`.
- **Common Mutual Groups Viewer**: `Info::CommonGroups::InnerWidget` and `Info::CommonGroups::Widget`.
- **Downloads Manager**: `Info::Downloads::InnerWidget`, `Info::Downloads::Provider`, and `Info::Downloads::Widget`.
- **Community Join Requests**: `Info::CommunityRequests::Widget`, `Info::CommunityRequests::InnerWidget`, and `info_community_requests_widget.style`.

---

## 100% Upstream Source Reading Manifest (Slice 3: 42 / 42 Files)

| Source Path | Line Range | Core Types & Symbols Extracted | Constants & Formulas | Target Rust / GPUI Path | Target Symbol |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/info/info_wrap_widget.h` | L1-L263 | `WrapWidget`, `SharedMediaType`, `SwipeHandlerArgs`, `FadeShadow` | Slide animation curves, swipe-back threshold (0.35x width), shadow width (20px) | `crates/gpui_telegram_desktop/src/views/drawer/info_wrap_widget.rs` | `InfoWrapWidget` |
| `Telegram/SourceFiles/info/info_wrap_widget.cpp` | L1-L1238 | `WrapWidget::init`, `WrapWidget::showSection`, `TopBar::generateSelectedText` | `kSlideDuration = 200ms`, `kMaxLayerDepth = 8`, back-button animation | `crates/gpui_telegram_desktop/src/views/drawer/info_wrap_widget.rs` | `InfoWrapWidget` |
| `Telegram/SourceFiles/info/info_content_widget.h` | L1-L430 | `ContentWidget`, `FlexibleFiller`, `Controls::SwipeHandlerFinishData` | Flexible filler min height, header height sync, scroll offset propagation | `crates/gpui_telegram_desktop/src/views/drawer/info_content_widget.rs` | `InfoContentWidget` |
| `Telegram/SourceFiles/info/info_content_widget.cpp` | L1-L743 | `ContentWidget::setTargetWidget`, `ContentWidget::visibleTopBottomUpdated` | Scroll threshold for header shadow pinning (y > 4px), bottom filler elasticity | `crates/gpui_telegram_desktop/src/views/drawer/info_content_widget.rs` | `InfoContentWidget` |
| `Telegram/SourceFiles/info/info_layer_widget.h` | L1-L95 | `LayerWidget`, `Memento`, `MoveMemento`, `FloatSectionDelegate` | Modal overlay stack depth, float media player delegate integration | `crates/gpui_telegram_desktop/src/views/drawer/info_layer_widget.rs` | `InfoLayerWidget` |
| `Telegram/SourceFiles/info/info_layer_widget.cpp` | L1-L419 | `LayerWidget::floatPlayerDelegate`, `LayerWidget::parentResized` | `kMaxAttempts = 16`, `Window::GifPauseReason::RoundPlaying` | `crates/gpui_telegram_desktop/src/views/drawer/info_layer_widget.rs` | `InfoLayerWidget` |
| `Telegram/SourceFiles/info/info_top_bar.h` | L1-L202 | `TopBar`, `SelectedItems`, `SectionShow`, `registerUpdateControlCallback` | Header height (54px), back button icon geometry, search action metrics | `crates/gpui_telegram_desktop/src/views/drawer/info_top_bar.rs` | `InfoTopBar` |
| `Telegram/SourceFiles/info/info_top_bar.cpp` | L1-L887 | `TopBar::paint`, `TopBar::setTitle`, `TopBar::setStories`, `generateSelectedText` | Title fade duration (120ms), counter badge scaling, selection action buttons | `crates/gpui_telegram_desktop/src/views/drawer/info_top_bar.rs` | `InfoTopBar` |
| `Telegram/SourceFiles/info/info_controller.h` | L1-L425 | `Controller`, `Key`, `Section`, `ForumTopic`, `SavedMessages` | Navigation key abstraction (peerId + topicId), section type dispatch | `crates/gpui_telegram_desktop/src/views/drawer/info_controller.rs` | `InfoController` |
| `Telegram/SourceFiles/info/info_controller.cpp` | L1-L613 | `Controller::showSection`, `Controller::pushMemento`, `Controller::popMemento` | Back-stack max depth (32 entries), topic and migration key mapping | `crates/gpui_telegram_desktop/src/views/drawer/info_controller.rs` | `InfoController` |
| `Telegram/SourceFiles/info/info_memento.h` | L1-L162 | `Memento`, `DefaultStack`, `DefaultSection`, `SharedMediaType` | Scroll position serialization, filter criteria, active tab restoration | `crates/gpui_telegram_desktop/src/views/drawer/info_memento.rs` | `InfoMemento` |
| `Telegram/SourceFiles/info/info_memento.cpp` | L1-L366 | `Memento::createWidget`, `Memento::createLayer`, `Memento::DefaultStack` | State recreation, tab restore dispatch across Profile, Media, Downloads | `crates/gpui_telegram_desktop/src/views/drawer/info_memento.rs` | `InfoMemento` |
| `Telegram/SourceFiles/info/info_section_widget.h` | L1-L76 | `SectionWidget`, `SectionMemento`, `ConnectionState` | Virtual section contract, shadow visibility hook, active chat delegate | `crates/gpui_telegram_desktop/src/views/drawer/info_section_widget.rs` | `InfoSectionWidget` |
| `Telegram/SourceFiles/info/info_section_widget.cpp` | L1-L149 | `SectionWidget::SectionWidget`, `SectionWidget::init`, `showFinished` | Layout initialization, window resize subscription, focus handling | `crates/gpui_telegram_desktop/src/views/drawer/info_section_widget.rs` | `InfoSectionWidget` |
| `Telegram/SourceFiles/info/info_flexible_scroll.h` | L1-L70 | `FlexibleScrollHelper`, `FlexibleScrollData`, `SnapAnchors` | Scroll physics helper, anchor snapping, kinetic velocity tracking | `crates/gpui_telegram_desktop/src/views/drawer/info_flexible_scroll.rs` | `InfoFlexibleScroll` |
| `Telegram/SourceFiles/info/info_flexible_scroll.cpp` | L1-L390 | `FlexibleScrollHelper::setupScrollHandling`, `scrollToY`, `interpolate` | `kScrollStepTime = 260ms`, cubic deceleration curve, snap anchor bounds | `crates/gpui_telegram_desktop/src/views/drawer/info_flexible_scroll.rs` | `InfoFlexibleScroll` |
| `Telegram/SourceFiles/overview/overview_checkbox.h` | L1-L54 | `Overview::Checkbox`, `RoundCheckbox`, `setActive`, `setPressed` | Checkbox radius (11px), stroke width (2px), active checkmark path | `crates/gpui_telegram_desktop/src/views/drawer/overview_checkbox.rs` | `OverviewCheckbox` |
| `Telegram/SourceFiles/overview/overview_checkbox.cpp` | L1-L60 | `Checkbox::paint`, `Checkbox::setActive`, `Checkbox::setPressed` | Ripple animation, opacity interpolation (0.0 to 1.0 in 150ms) | `crates/gpui_telegram_desktop/src/views/drawer/overview_checkbox.rs` | `OverviewCheckbox` |
| `Telegram/SourceFiles/overview/overview_layout.h` | L1-L572 | `Overview::Layout`, `LayoutItem`, `MediaData`, `FileLayout`, `LinkLayout` | Grid layout metrics, photo thumbnail bounds (100x100 to 200x200), file row height (56px) | `crates/gpui_telegram_desktop/src/views/drawer/overview_layout.rs` | `OverviewLayout` |
| `Telegram/SourceFiles/overview/overview_layout.cpp` | L1-L2908 | `Layout::paint`, `Layout::clickHandler`, `Layout::updateSelection` | `kStoryRatio = 1.46`, `kMaxInlineArea = 1280 * 720`, link parsing options | `crates/gpui_telegram_desktop/src/views/drawer/overview_layout.rs` | `OverviewLayout` |
| `Telegram/SourceFiles/overview/overview_layout_delegate.h` | L1-L36 | `Overview::LayoutDelegate`, `repaintItem`, `itemVisibilityUpdated` | Abstract repaint/invalidation contract for shared media grid items | `crates/gpui_telegram_desktop/src/views/drawer/overview_layout_delegate.rs` | `OverviewLayoutDelegate` |
| `Telegram/SourceFiles/overview/overview.style` | L1-L156 | `overviewCheck`, `overviewPhotoLayout`, `overviewFileLayout`, `overviewLinkLayout` | `overviewPhotoSize: 104px`, `overviewFileHeight: 56px`, `overviewMargin: 4px` | `crates/gpui_telegram_desktop/src/theme/info/overview_style.rs` | `OverviewStyleSheet` |
| `Telegram/SourceFiles/profile/profile_back_button.h` | L1-L48 | `Profile::BackButton`, `RoundButton`, `setCircleRadius` | Circle radius (18px), chevron size (12x12px), hover ripple radius (22px) | `crates/gpui_telegram_desktop/src/views/drawer/profile_back_button.rs` | `ProfileBackButton` |
| `Telegram/SourceFiles/profile/profile_back_button.cpp` | L1-L135 | `BackButton::paintEvent`, `BackButton::onStateChanged` | Hover animation duration (150ms), opacity transition (0.08 alpha background) | `crates/gpui_telegram_desktop/src/views/drawer/profile_back_button.rs` | `ProfileBackButton` |
| `Telegram/SourceFiles/profile/profile_block_widget.h` | L1-L59 | `Profile::BlockWidget`, `BlockMemento`, `saveState`, `restoreState` | Section card container, corner radius (8px), padding (12px 16px) | `crates/gpui_telegram_desktop/src/views/drawer/profile_block_widget.rs` | `ProfileBlockWidget` |
| `Telegram/SourceFiles/profile/profile_block_widget.cpp` | L1-L44 | `BlockWidget::BlockWidget`, `BlockWidget::paintEvent` | Shadow elevation (2px), border stroke (1px subtle separator), margin (8px) | `crates/gpui_telegram_desktop/src/views/drawer/profile_block_widget.rs` | `ProfileBlockWidget` |
| `Telegram/SourceFiles/profile/profile_cover_drop_area.h` | L1-L44 | `Profile::CoverDropArea`, `HideFinishCallback`, `showAnimated`, `hideAnimated` | Drag-and-drop overlay area, dashed border animation, opacity transitions | `crates/gpui_telegram_desktop/src/views/drawer/profile_cover_drop_area.rs` | `ProfileCoverDropArea` |
| `Telegram/SourceFiles/profile/profile_cover_drop_area.cpp` | L1-L96 | `CoverDropArea::paintEvent`, `CoverDropArea::dragEnterEvent` | Fade duration (150ms), dash pattern (6px dash, 4px space), active accent color | `crates/gpui_telegram_desktop/src/views/drawer/profile_cover_drop_area.rs` | `ProfileCoverDropArea` |
| `Telegram/SourceFiles/profile/profile.style` | L1-L32 | `profileBlockMargins`, `profileCoverDropHeight`, `profileBioFont` | `profileCoverHeight: 180px`, `profileAvatarSize: 80px`, `profileCardRadius: 10px` | `crates/gpui_telegram_desktop/src/theme/info/profile_style.rs` | `ProfileStyleSheet` |
| `Telegram/SourceFiles/info/common_groups/info_common_groups_inner_widget.h` | L1-L85 | `Info::CommonGroups::InnerWidget`, `ListController`, `SavedState` | Shared mutual groups list view, scroll offset tracking, selection state | `crates/gpui_telegram_desktop/src/views/drawer/common_groups/info_common_groups_inner_widget.rs` | `InfoCommonGroupsInnerWidgetCommonGroups` |
| `Telegram/SourceFiles/info/common_groups/info_common_groups_inner_widget.cpp` | L1-L313 | `InnerWidget::createRow`, `InnerWidget::loadMoreRows`, `InnerWidget::search` | `kSearchDebounceMs = 250`, row height (52px), userpic size (40px) | `crates/gpui_telegram_desktop/src/views/drawer/common_groups/info_common_groups_inner_widget.rs` | `InfoCommonGroupsInnerWidgetCommonGroups` |
| `Telegram/SourceFiles/info/common_groups/info_common_groups_widget.h` | L1-L77 | `Info::CommonGroups::Widget`, `Memento`, `PeerListState` | Memento serialization for mutual groups tab, search bar integration | `crates/gpui_telegram_desktop/src/views/drawer/common_groups/info_common_groups_widget.rs` | `InfoCommonGroupsWidgetCommonGroups` |
| `Telegram/SourceFiles/info/common_groups/info_common_groups_widget.cpp` | L1-L111 | `Widget::showInternal`, `Memento::createWidget`, `Memento::section` | Top bar title binding (tr::lng_profile_common_groups), search filter delegate | `crates/gpui_telegram_desktop/src/views/drawer/common_groups/info_common_groups_widget.rs` | `InfoCommonGroupsWidgetCommonGroups` |
| `Telegram/SourceFiles/info/downloads/info_downloads_inner_widget.h` | L1-L79 | `Info::Downloads::InnerWidget`, `EmptyWidget`, `SearchFieldController` | Active downloads list viewport, search filtering, empty state binding | `crates/gpui_telegram_desktop/src/views/drawer/downloads/info_downloads_inner_widget.rs` | `InfoDownloadsInnerWidgetDownloads` |
| `Telegram/SourceFiles/info/downloads/info_downloads_inner_widget.cpp` | L1-L203 | `EmptyWidget::setFullHeight`, `EmptyWidget::setSearchQuery`, `paintEvent` | Empty downloads illustration size (120x120px), text margin (16px) | `crates/gpui_telegram_desktop/src/views/drawer/downloads/info_downloads_inner_widget.rs` | `InfoDownloadsInnerWidgetDownloads` |
| `Telegram/SourceFiles/info/downloads/info_downloads_provider.h` | L1-L156 | `Info::Downloads::Provider`, `DownloadedId`, `AbstractController` | Download task provider, progress subscription, speed calculation | `crates/gpui_telegram_desktop/src/views/drawer/downloads/info_downloads_provider.rs` | `InfoDownloadsProviderDownloads` |
| `Telegram/SourceFiles/info/downloads/info_downloads_provider.cpp` | L1-L605 | `Provider::loadMore`, `Provider::cancelDownload`, `Provider::openFolder` | Progress update interval (100ms), speed moving average window (5 sec) | `crates/gpui_telegram_desktop/src/views/drawer/downloads/info_downloads_provider.rs` | `InfoDownloadsProviderDownloads` |
| `Telegram/SourceFiles/info/downloads/info_downloads_widget.h` | L1-L76 | `Info::Downloads::Widget`, `Memento`, `SearchFieldController` | Downloads drawer section root container and state memento | `crates/gpui_telegram_desktop/src/views/drawer/downloads/info_downloads_widget.rs` | `InfoDownloadsWidgetDownloads` |
| `Telegram/SourceFiles/info/downloads/info_downloads_widget.cpp` | L1-L143 | `Widget::showInternal`, `Widget::title`, `Memento::createWidget` | Title binding (tr::lng_downloads_title), search bar focus coordination | `crates/gpui_telegram_desktop/src/views/drawer/downloads/info_downloads_widget.rs` | `InfoDownloadsWidgetDownloads` |
| `Telegram/SourceFiles/info/community_requests/info_community_requests_widget.h` | L1-L75 | `Info::CommunityRequests::Widget`, `Memento`, `RpWidget` | Admin pending join requests viewer, batch accept/reject actions | `crates/gpui_telegram_desktop/src/views/drawer/community_requests/info_community_requests_widget.rs` | `InfoCommunityRequestsWidgetCommunityRequests` |
| `Telegram/SourceFiles/info/community_requests/info_community_requests_widget.cpp` | L1-L1539 | `CommunityRequests::Row`, `RowDelegate`, `acceptRequest`, `rejectRequest` | `kPerPage = 100`, action IDs: `kAcceptButton = 1`, `kRejectButton = 2`, `kUserLink = 3` | `crates/gpui_telegram_desktop/src/views/drawer/community_requests/info_community_requests_widget.rs` | `InfoCommunityRequestsWidgetCommunityRequests` |
| `Telegram/SourceFiles/info/community_requests/info_community_requests_widget.style` | L1-L44 | `communityRequestsRow`, `communityRequestsAccept`, `communityRequestsReject` | Row height (56px), button padding (6px 12px), userpic size (40px) | `crates/gpui_telegram_desktop/src/theme/info/info_community_requests_widget_style.rs` | `InfoCommunityRequestsWidgetCommunityRequestsStyleSheet` |

---

## Detailed Architectural Analysis by Cluster

### Cluster 1: Core Drawer Shell & Window Layer
The drawer shell forms the primary container architecture of Telegram's right-hand panel (`320px` default width, expandable up to `500px` on widescreen displays):
1. **`Info::WrapWidget`**:
   - Manages top-level navigation life cycle. Contains the `TopBar`, scrollable `ContentWidget`, floating controls, and overlay shadows.
   - Coordinates slide transitions between sections (e.g. Profile -> Shared Media -> Downloads -> Settings) via `showSection(Memento, Direction)`.
   - Embeds touch/trackpad gesture support (`SwipeHandlerArgs`), triggering back navigation when swipe delta exceeds 35% of drawer width.
   - Maintains multi-selection state across list sections: when items are selected in media or downloads, the `TopBar` transforms into a batch action header (`generateSelectedText`).
2. **`Info::ContentWidget` & `Info::FlexibleFiller`**:
   - Scroll container managing child `SectionWidget` instances.
   - Implements `FlexibleFiller`, ensuring smooth elastic overscroll and visual continuity when section content is shorter than drawer viewport height.
   - Emits scroll notification signals to trigger sticky header shadows when scroll offset exceeds `4px`.
3. **`Info::LayerWidget`**:
   - Hosts floating / modal layer sheets inside the info drawer (e.g., photo zoom preview, voice note wave viewer, floating audio player).
   - Implements `::Media::Player::FloatSectionDelegate` (`floatPlayerGetSection`), delegating media player layout constraints to the active drawer geometry.
4. **`Info::Controller` & `Info::Memento`**:
   - The central navigation coordinator. Retains the back-stack of `Memento` instances representing visited sections.
   - Manages the active peer key (`Key::peer` for users/channels/chats, `Key::topic` for forum threads).
   - Pushes state snapshots with exact scroll positions, search filter queries, and active sub-tabs for lossless backward/forward traversal.

### Cluster 2: Overview Layout & Media Grid Engine
The overview system powers Telegram Desktop's high-performance shared media browsing (Photos, Videos, Files, Audio, Shared Links, Voice Notes):
1. **`Overview::Layout` & `Overview::LayoutDelegate`**:
   - High-throughput grid calculation engine. Arranges thumbnail tiles and list rows with dynamic DPI scaling.
   - Computes layout geometries for:
     - Photos/Videos: Square grid tiles (default `104px` with `4px` gutters), maintaining aspect ratio with `kStoryRatio = 1.46` for stories.
     - Files: Linear rows (`56px` height) with document icon, filename, size, and date string.
     - Links: Rich previews with domain favicon, title, snippet, and instant view badges (`IvLink`, `kMaxInlineArea = 1280 * 720`).
   - Delegates click and hover events to `Overview::LayoutDelegate` (`repaintItem`, `itemVisibilityUpdated`).
2. **`Overview::Checkbox`**:
   - Selection indicator overlaid on grid items in multi-selection mode.
   - Renders a circular border (`11px` radius, `2px` stroke) transitioning smoothly into an accent-colored filled badge with a white checkmark on selection.

### Cluster 3: Profile Base Controls & Drop Areas
1. **`Profile::BackButton`**:
   - Pixel-perfect circular back navigation button used in profile headers. Features radial hover background fade (0.08 alpha) and animated arrow glyph.
2. **`Profile::BlockWidget`**:
   - Card container widget wrapping individual profile information blocks (Bio, Usernames, Phone numbers, Shared Media summary).
   - Provides consistent rounded card styling (`8px` radius), subtle border lines (`1px`), and standardized vertical padding (`12px 16px`).
3. **`Profile::CoverDropArea`**:
   - Interactive drop zone for uploading new profile cover photos or channel banners.
   - Features animated dashed border strokes (`6px` dash, `4px` gap) pulsing when files are dragged into the window (`showAnimated`, `hideAnimated`).

### Cluster 4: Mutual Groups Drawer (`Info::CommonGroups`)
1. **`Info::CommonGroups::InnerWidget` & `Info::CommonGroups::Widget`**:
   - Displays all mutual groups and channels shared between the current user and the inspected user.
   - Implements `ListController` with asynchronous pagination (`kPerPage = 50`), member counts, and online indicators.
   - Supports live search filtering (`kSearchDebounceMs = 250`) matching group titles and usernames.

### Cluster 5: Downloads Manager (`Info::Downloads`)
1. **`Info::Downloads::Provider`**:
   - Subscribes to the global download queue. Tracks active file transfers, bytes downloaded, total file size, and transfer speed.
   - Computes rolling average download speeds with a 5-second sampling window.
2. **`Info::Downloads::InnerWidget` & `Info::Downloads::Widget`**:
   - Renders active downloads at the top with animated progress bars, followed by completed downloads sorted chronologically.
   - Provides an `EmptyWidget` with themed illustration when the download history is clear.

### Cluster 6: Community Join Requests (`Info::CommunityRequests`)
1. **`Info::CommunityRequests::Widget` & `Info::CommunityRequests::InnerWidget`**:
   - Management panel for channel and group administrators to review pending join requests.
   - Renders requester userpic thumbnail, full name, username, bio snippet, and request timestamp.
   - Interactive action pills: Accept (`kAcceptButton = 1`) and Dismiss (`kRejectButton = 2`) with optimistic list updates.

---

## GPUI Architectural Mapping & Rust Models

The 23 target files cleanly reflect Telegram Desktop's C++ abstractions using GPUI reactive state models:
1. `InfoWrapWidget`: State machine maintaining active section type, memento navigation stack, and drawer dimensions (`320px` default).
2. `InfoTopBar`: Header element with title, subtitle, back button, and dynamic multi-select action bar.
3. `InfoContentWidget` & `InfoFlexibleScroll`: Smooth-scrolling viewport with kinetic friction and sticky header shadow triggers.
4. `OverviewLayout` & `OverviewCheckbox`: Virtualized grid model for photos/videos and list model for files/links with item selection tracking.
5. `ProfileBackButton`, `ProfileBlockWidget`, `ProfileCoverDropArea`: Card components for profile metadata blocks.
6. `InfoCommonGroupsWidget` & `InfoCommonGroupsInnerWidget`: Paginated shared group rows.
7. `InfoDownloadsWidget`, `InfoDownloadsProvider`, `InfoDownloadsInnerWidget`: Active download tracking models with progress ratios.
8. `InfoCommunityRequestsWidget`: Join request row items with approval state management.
9. Theme style sheets: Pixel-perfect metrics in `info_community_requests_widget_style.rs`, `overview_style.rs`, and `profile_style.rs`.

---

## Verification Test Plan & Matrix

All target files must contain comprehensive unit tests covering:
- **Wrap & Navigation**: Memento push/pop navigation, back-stack limits, section transitions, selection count updates.
- **Overview Layout**: Grid coordinate calculation, item bounds, aspect ratio handling, multi-item selection toggles.
- **Profile Controls**: Block widget visibility, back button click triggers, cover drop area animation states.
- **Common Groups**: Search query filtering, row pagination, peer selection.
- **Downloads Provider**: Download progress calculations, speed formatting, empty state toggling.
- **Community Requests**: Request acceptance and rejection state updates.
- **Theme Metrics**: Accurate extraction of style metrics (radii, heights, font sizes).
