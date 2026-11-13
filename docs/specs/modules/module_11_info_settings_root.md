# Module 11 (info & settings) — Slice 6: Info Settings, Root/Bot Widgets & Style Specifications

## 1. Executive Summary & Architectural Role

Module 11 Slice 6 delivers the structural backbone and specialized panels of Telegram Desktop's right-hand Drawer panel in pure Rust and GPUI:
- **Settings Section Container**: `InfoSettingsWidgetSettings` hosts user preferences, profile editing, and sub-sections (`info/settings/info_settings_widget.cpp:L42-L160`), orchestrating flexible top bar scrolling, sticky headers, and stack navigation (`showOther`, `showBackFromStack`, `removeFromStack`).
- **Bot Stars & Earn Management**: `InfoBotEarnListBotEarn` and `InfoBotEarnWidgetBotEarn` present Telegram Stars revenue charts, USD rate conversion (`kMinorLength = 3`), balance breakdowns, and withdrawal triggers (`info/bot/earn/info_bot_earn_list.cpp:L118-L260`).
- **Bot StarRef Affiliate Program**: `InfoBotStarrefCommonBotStarref`, `InfoBotStarrefJoinWidgetBotStarref`, and `InfoBotStarrefSetupWidgetBotStarref` enable bot affiliate commissions (default `kCommissionDefault = 200` bp = 2.0%), program durations (`kDurationDefault = 12` months, `kDurationForeverValue = 999`), and peer list pagination (`kPerPage = 50`).
- **Drawer Root Infrastructure**: Core architectural primitives `InfoContentWidget`, `InfoController`, `InfoFlexibleScroll` (`kScrollStepTime = 260ms`, `CountSnapAnchors`), `InfoLayerWidget` (`MinimalSupportedWidth`, `kMaxAttempts = 16`), `InfoMemento`, `InfoSectionWidget`, `InfoTopBar` (54px standard, 56px layer), and `InfoWrapWidget` (`Wrap::Layer`, `Wrap::Side`, `Wrap::Narrow`).
- **Media Overview & Profile Components**: `OverviewCheckbox`, `OverviewLayout`, `OverviewLayoutDelegate` (Photo, Gif, Video, Voice, Document, Link items), `ProfileBackButton`, `ProfileBlockWidget`, and `ProfileCoverDropArea`.
- **Comprehensive Theme & Style Tokens**: 10 `.style` stylesheets providing exact pixel metrics, icon positions, animations, and typography tokens.

---

## 2. Upstream C++ 100% Reading Manifest (All 42 Upstream Files)

| # | Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path | Target Symbol |
|---|:---|---:|:---|:---|:---|
| 1 | `Telegram/SourceFiles/info/bot/earn/info_bot_earn_list.h` | 67 | `InnerWidget`, `Memento`, `ShowRequest`, `IconCurrency`, `_state: CreditsEarnStatistics`, `load()`, `fill()`, `fillHistory()` | `crates/gpui_telegram_desktop/src/views/drawer/bot/earn/info_bot_earn_list.rs` | `InfoBotEarnListBotEarn` |
| 2 | `Telegram/SourceFiles/info/bot/earn/info_bot_earn_list.cpp` | 464 | `info_bot_earn_list.cpp:L123`: `kMinorLength = 3`; `L140-L160`: `revenueGraph.chart` StackBar; `L182-L203`: `ToUsd(v, multiplier, 3)`; `L232-L262`: `AddWithdrawalWidget`; `L324-L346`: `defaultTabsSlider` | `crates/gpui_telegram_desktop/src/views/drawer/bot/earn/info_bot_earn_list.rs` | `InfoBotEarnListBotEarn` |
| 3 | `Telegram/SourceFiles/info/bot/earn/info_bot_earn_widget.h` | 68 | `Memento`, `Widget`, `Make()`, `SavedState = Data::CreditsEarnStatistics`, `showInternal()`, `desiredShadowVisibility()` | `crates/gpui_telegram_desktop/src/views/drawer/bot/earn/info_bot_earn_widget.rs` | `InfoBotEarnWidgetBotEarn` |
| 4 | `Telegram/SourceFiles/info/bot/earn/info_bot_earn_widget.cpp` | 118 | `info_bot_earn_widget.cpp:L42-L72`: `Widget` lifecycle, `title` -> `lng_bot_earn_title`, `desiredShadowVisibility() = true`, `SendPendingMoveResizeEvents` | `crates/gpui_telegram_desktop/src/views/drawer/bot/earn/info_bot_earn_widget.rs` | `InfoBotEarnWidgetBotEarn` |
| 5 | `Telegram/SourceFiles/info/bot/starref/info_bot_starref_common.h` | 112 | `ConnectedBotState`, `ConnectedBot`, `ConnectedBots`, `FormatCommission`, `FormatProgramDuration`, `AddViewListButton`, `MakePeerBubbleButton`, `Parse` | `crates/gpui_telegram_desktop/src/views/drawer/bot/starref/info_bot_starref_common.rs` | `InfoBotStarrefCommonBotStarref` |
| 6 | `Telegram/SourceFiles/info/bot/starref/info_bot_starref_common.cpp` | 1043 | `info_bot_starref_common.cpp:L56-L74`: `ConnectStarRef`; `L89-L95`: `starrefLinkThumbOuter`, `starrefLinkThumbInner`; `L315-L350`: duration formatting; `L610-L640`: `MakePeerBubbleButton` | `crates/gpui_telegram_desktop/src/views/drawer/bot/starref/info_bot_starref_common.rs` | `InfoBotStarrefCommonBotStarref` |
| 7 | `Telegram/SourceFiles/info/bot/starref/info_bot_starref_join_widget.h` | 89 | `Memento`, `Widget`, `Allowed()`, `Make()`, `ProgramsListBox()`, `setupTop()`, `enableBackButton()` | `crates/gpui_telegram_desktop/src/views/drawer/bot/starref/info_bot_starref_join_widget.rs` | `InfoBotStarrefJoinWidgetBotStarref` |
| 8 | `Telegram/SourceFiles/info/bot/starref/info_bot_starref_join_widget.cpp` | 1147 | `info_bot_starref_join_widget.cpp:L60`: `kPerPage = 50`; `L62-L72`: `JoinType` (Joined, Suggested, Existing), `SuggestedSort` (Profitability, Revenue, Date); `ListController` | `crates/gpui_telegram_desktop/src/views/drawer/bot/starref/info_bot_starref_join_widget.rs` | `InfoBotStarrefJoinWidgetBotStarref` |
| 9 | `Telegram/SourceFiles/info/bot/starref/info_bot_starref_setup_widget.h` | 86 | `State`, `InnerWidget`, `Memento`, `Widget`, `Allowed()`, `Make()`, `setupTop()`, `setupBottom()` | `crates/gpui_telegram_desktop/src/views/drawer/bot/starref/info_bot_starref_setup_widget.rs` | `InfoBotStarrefSetupWidgetBotStarref` |
| 10 | `Telegram/SourceFiles/info/bot/starref/info_bot_starref_setup_widget.cpp` | 1086 | `info_bot_starref_setup_widget.cpp:L44-L48`: `kDurationForeverValue = 999`, `kCommissionDefault = 200`, `kDurationDefault = 12`, `kDisabledFade = 0.3`; `ValueForCommission`, `ValueForDurationMonths` | `crates/gpui_telegram_desktop/src/views/drawer/bot/starref/info_bot_starref_setup_widget.rs` | `InfoBotStarrefSetupWidgetBotStarref` |
| 11 | `Telegram/SourceFiles/info/settings/info_settings_widget.h` | 125 | `SectionCustomTopBarData`, `Memento`, `Widget`, `Type = Section::SettingsType`, `FlexibleScrollData`, `_removesFromStack` | `crates/gpui_telegram_desktop/src/views/drawer/settings/info_settings_widget.rs` | `InfoSettingsWidgetSettings` |
| 12 | `Telegram/SourceFiles/info/settings/info_settings_widget.cpp` | 284 | `info_settings_widget.cpp:L48-L65`: `setupFlexibleInnerWidget`, pinned headers; `L78-L86`: `removeFromStack`; `L121-L159`: flexible top bar slide & wheel filter; `L214-L219`: `desiredShadowVisibility` | `crates/gpui_telegram_desktop/src/views/drawer/settings/info_settings_widget.rs` | `InfoSettingsWidgetSettings` |
| 13 | `Telegram/SourceFiles/info/info_content_widget.h` | 430 | `ContentWidget`, `ContentMemento`, `Key`, `SwipeInterceptor`, `FlexibleScrollData`, `selectedListValue`, `floatPlayerHandleWheelEvent` | `crates/gpui_telegram_desktop/src/views/drawer/info_content_widget.rs` | `InfoContentWidget` |
| 14 | `Telegram/SourceFiles/info/info_content_widget.cpp` | 743 | `info_content_widget.cpp:L95-L160`: `applyScrollTopRestore`, swipe back gestures, scroll delta tracking, search field toggling; `L220-L260`: `scrollTopSave`/`scrollTopRestore` | `crates/gpui_telegram_desktop/src/views/drawer/info_content_widget.rs` | `InfoContentWidget` |
| 15 | `Telegram/SourceFiles/info/info_controller.h` | 425 | `Key`, `Section` (22 types: Profile, Media, Settings, BotEarn, BotStarRef, etc.), `Wrap` (Layer, Side, Narrow, Search), `Controller` | `crates/gpui_telegram_desktop/src/views/drawer/info_controller.rs` | `InfoController` |
| 16 | `Telegram/SourceFiles/info/info_controller.cpp` | 613 | `info_controller.cpp:L50-L120`: `Key` variant resolution, `showSection`, `showSettings`, `removeFromStack`, `takeStepData` | `crates/gpui_telegram_desktop/src/views/drawer/info_controller.rs` | `InfoController` |
| 17 | `Telegram/SourceFiles/info/info_flexible_scroll.h` | 70 | `kClassicProfileScroll`, `UseClassicProfileScroll()`, `SetupFlexibleRegularScroll`, `FlexibleScrollData`, `FlexibleScrollHelper` | `crates/gpui_telegram_desktop/src/views/drawer/info_flexible_scroll.rs` | `InfoFlexibleScroll` |
| 18 | `Telegram/SourceFiles/info/info_flexible_scroll.cpp` | 390 | `info_flexible_scroll.cpp:L30`: `kScrollStepTime = 260ms`; `L37-L48`: `CountSnapAnchors` (`st::infoProfileTopBarHeightMax`, `Step1`, `Step2`); `L58-L92`: `applyTop` clamp min/max | `crates/gpui_telegram_desktop/src/views/drawer/info_flexible_scroll.rs` | `InfoFlexibleScroll` |
| 19 | `Telegram/SourceFiles/info/info_layer_widget.h` | 95 | `LayerWidget`, `FloatDelegate`, `MinimalSupportedWidth()`, `takeToThirdSection()`, `parentResized()`, `setupHeightConsumers()` | `crates/gpui_telegram_desktop/src/views/drawer/info_layer_widget.rs` | `InfoLayerWidget` |
| 20 | `Telegram/SourceFiles/info/info_layer_widget.cpp` | 419 | `info_layer_widget.cpp:L207-L211`: width clamp `st::infoDesiredWidth`; `L270-L271`: `MinimalSupportedWidth` = `st::infoMinimalWidth + 2*st::infoMinimalLayerMargin`; `L277`: `kMaxAttempts = 16` | `crates/gpui_telegram_desktop/src/views/drawer/info_layer_widget.rs` | `InfoLayerWidget` |
| 21 | `Telegram/SourceFiles/info/info_memento.h` | 162 | `Memento`, `MoveMemento`, `Default()`, `DefaultSection()`, `DefaultStack()`, `takeStack()`, `stackSize()` | `crates/gpui_telegram_desktop/src/views/drawer/info_memento.rs` | `InfoMemento` |
| 22 | `Telegram/SourceFiles/info/info_memento.cpp` | 366 | `info_memento.cpp:L45-L95`: `DefaultSection` resolution for user/chat/channel; `L120-L180`: `createWidget` and `createLayer` instantiation | `crates/gpui_telegram_desktop/src/views/drawer/info_memento.rs` | `InfoMemento` |
| 23 | `Telegram/SourceFiles/info/info_section_widget.h` | 76 | `SectionWidget`, `Wrap`, `moveContentToLayer()`, `activeChat()`, `grabForShowAnimation()`, `showBackInternal()` | `crates/gpui_telegram_desktop/src/views/drawer/info_section_widget.rs` | `InfoSectionWidget` |
| 24 | `Telegram/SourceFiles/info/info_section_widget.cpp` | 149 | `info_section_widget.cpp:L30-L75`: third-column section container, `ConnectionState` banner, top bar surrogate painting, forward/back stack slides | `crates/gpui_telegram_desktop/src/views/drawer/info_section_widget.rs` | `InfoSectionWidget` |
| 25 | `Telegram/SourceFiles/info/info_top_bar.h` | 202 | `TopBar`, `TitleDescriptor`, `backRequest`, `storyClicks`, `selectionActionRequests`, `createSearchView`, `setSelectedItems` | `crates/gpui_telegram_desktop/src/views/drawer/info_top_bar.rs` | `InfoTopBar` |
| 26 | `Telegram/SourceFiles/info/info_top_bar.cpp` | 887 | `info_top_bar.cpp:L50-L120`: selection action buttons (forward, delete, pin stories); `L170-L240`: search view layout & focus; `L380-L450`: title & subtitle geometry animation | `crates/gpui_telegram_desktop/src/views/drawer/info_top_bar.rs` | `InfoTopBar` |
| 27 | `Telegram/SourceFiles/info/info_wrap_widget.h` | 263 | `WrapWidget`, `Wrap` (Layer, Narrow, Side, Search, StoryAlbumEdit), `SelectedItem`, `SelectedItems`, `SelectionAction` | `crates/gpui_telegram_desktop/src/views/drawer/info_wrap_widget.rs` | `InfoWrapWidget` |
| 28 | `Telegram/SourceFiles/info/info_wrap_widget.cpp` | 1238 | `info_wrap_widget.cpp:L90-L170`: `_historyStack` state transitions, forward/back stack navigation; `L210-L280`: `updateGeometry`, `desiredHeightForContent`, top bar surrogate | `crates/gpui_telegram_desktop/src/views/drawer/info_wrap_widget.rs` | `InfoWrapWidget` |
| 29 | `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/giveaway.style` | 279 | `giveawayTypeListItem: 52px`, `giveawayGiftCodeTopHeight: 195px`, `giveawayGiftCodeStartButton: 42px`, `giveawayGiftCodeCover: 92x90px star`, `darkGiftCodeBox`, `darkGiftPalette` | `crates/gpui_telegram_desktop/src/theme/info/giveaway_style.rs` | `GiveawayChannelStatisticsBoostsGiveawayStyleSheet` |
| 30 | `Telegram/SourceFiles/info/channel_statistics/earn/channel_earn.style` | 151 | `channelEarnOverviewTitleSkip: 11px`, `channelEarnOverviewMajorLabel: 15px semibold`, `channelEarnBalanceMajorLabel: 22px semibold`, `botEarnInputField: 100px`, `botEarnButtonLock` | `crates/gpui_telegram_desktop/src/theme/info/channel_earn_style.rs` | `ChannelEarnChannelStatisticsEarnStyleSheet` |
| 31 | `Telegram/SourceFiles/info/community_requests/info_community_requests_widget.style` | 44 | `communityRequestsTitlePadding`, `requestSuggestBadgeSize: 16px`, `requestMembersTop: 11px`, `requestsAddAllButton: 32px`, `requestsDeclineAllButton: 32px` | `crates/gpui_telegram_desktop/src/theme/info/info_community_requests_widget_style.rs` | `InfoCommunityRequestsWidgetCommunityRequestsStyleSheet` |
| 32 | `Telegram/SourceFiles/info/info.style` | 1382 | `infoTopBarHeight: 54px`, `infoLayerTopBarHeight: 56px`, `infoMinimalWidth: 324px`, `infoDesiredWidth: 392px`, `infoMinimalLayerMargin: 48px`, `infoProfileTopBarHeightMax: 236px`, `infoProfileCover: 108px` | `crates/gpui_telegram_desktop/src/theme/info/info_style.rs` | `InfoStyleSheet` |
| 33 | `Telegram/SourceFiles/info/profile/info_levels.style` | 81 | `levelStyle: 9px semibold`, `levelMargin: margins(4, 3, 2, 2)`, `levelBase: point(0, 6)`, `level1..level90` 26x26 shape inner icons, `levelNegativeBubble: 28x28` | `crates/gpui_telegram_desktop/src/theme/info/info_levels_style.rs` | `InfoLevelsProfileStyleSheet` |
| 34 | `Telegram/SourceFiles/info/profile/info_profile_actions.style` | 69 | `infoProfileLabeledPadding: margins(23, 9, 20, 7)`, `infoPersonalChannelPadding: margins(79, 9, 20, 7)`, `infoOpenAppMargin: margins(16, 12, 16, 12)`, `infoHoursOuterMargin` | `crates/gpui_telegram_desktop/src/theme/info/info_profile_actions_style.rs` | `InfoProfileActionsProfileStyleSheet` |
| 35 | `Telegram/SourceFiles/info/profile/info_profile_top_bar.style` | 139 | `infoProfileTopBarTitleTop: 113px`, `infoProfileTopBarStatusTop: 134px`, `infoProfileTopBarPhotoTop: 24px`, `infoProfileTopBarPhotoSize: 80px`, `infoProfileBalancePillHeight: 16px` | `crates/gpui_telegram_desktop/src/theme/info/info_profile_top_bar_style.rs` | `InfoProfileTopBarProfileStyleSheet` |
| 36 | `Telegram/SourceFiles/info/userpic/info_userpic_builder.style` | 68 | `userpicBuilderEmojiBubblePaletteWidth: 356px`, `userpicBuilderEmojiAccentColorSize: 30px`, `userpicBuilderEmojiLayerMinHeight: 496px`, `userpicBuilderEmojiSelectorMinHeight: 216px` | `crates/gpui_telegram_desktop/src/theme/info/info_userpic_builder_style.rs` | `InfoUserpicBuilderUserpicStyleSheet` |
| 37 | `Telegram/SourceFiles/overview/overview.style` | 156 | `overviewCheckPressedSize: 0.8`, `overviewCheck: 29px`, `overviewCheckSkip: 5px`, `overviewPhotoMinSize: 90px`, `overviewFileLayout: maxWidth 520px`, `linksPhotoSize: 46px` | `crates/gpui_telegram_desktop/src/theme/info/overview_style.rs` | `OverviewStyleSheet` |
| 38 | `Telegram/SourceFiles/profile/profile.style` | 32 | `profileDropAreaPadding: margins(25, 3, 25, 20)`, `profileDropAreaTitleFont: 24px`, `profileDropAreaSubtitleFont: 16px`, `profileBlockMarginTop: 14px`, `profileBlockTitleHeight: 24px` | `crates/gpui_telegram_desktop/src/theme/info/profile_style.rs` | `ProfileStyleSheet` |
| 39 | `Telegram/SourceFiles/overview/overview_checkbox.h` | 54 | `Checkbox`: `_updateCallback`, `_check: Ui::RoundCheckbox`, `_pression: Animations::Simple`, `setActive`, `setPressed`, `setChecked` | `crates/gpui_telegram_desktop/src/views/drawer/overview_checkbox.rs` | `OverviewCheckbox` |
| 40 | `Telegram/SourceFiles/overview/overview_checkbox.cpp` | 60 | `overview_checkbox.cpp:L20-L40`: `paint` checkbox at position with outerWidth; `L42-L58`: `setChecked` with `anim::type::normal`, startAnimationression | `crates/gpui_telegram_desktop/src/views/drawer/overview_checkbox.rs` | `OverviewCheckbox` |
| 41 | `Telegram/SourceFiles/overview/overview_layout.h` | 572 | `ItemBase`, `RadialProgressItem`, `StatusText`, `Photo`, `Gif`, `Video`, `Voice`, `Document`, `Link`, `MediaOptions`, `DocumentFields` | `crates/gpui_telegram_desktop/src/views/drawer/overview_layout.rs` | `OverviewLayout` |
| 42 | `Telegram/SourceFiles/overview/overview_layout.cpp` | 2908 | `overview_layout.cpp:L90-L150`: `StatusText::update` bytes/time strings; `L450-L520`: `Photo::paint` with spoiler; `L1100-L1200`: `Document::paint` file icon/thumb; `L1900-L2000`: `Link::paint` | `crates/gpui_telegram_desktop/src/views/drawer/overview_layout.rs` | `OverviewLayout` |

---

### Companion Overview & Profile Root Extensions (Zero-Omission Ledger Guarantee)

| # | Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path | Target Symbol |
|---|:---|---:|:---|:---|:---|
| 43 | `Telegram/SourceFiles/overview/overview_layout_delegate.h` | 36 | `Delegate`: `registerHeavyItem`, `unregisterHeavyItem`, `repaintItem`, `itemVisible`, `openPhoto`, `openDocument`, `hiddenMark` | `crates/gpui_telegram_desktop/src/views/drawer/overview_layout_delegate.rs` | `OverviewLayoutDelegate` |
| 44 | `Telegram/SourceFiles/profile/profile_back_button.h` | 48 | `BackButton`: `setText`, `setSubtext`, `setWidget`, `setOpacity`, `_unreadBadgeLifetime`, `_cachedWidth` | `crates/gpui_telegram_desktop/src/views/drawer/profile_back_button.rs` | `ProfileBackButton` |
| 45 | `Telegram/SourceFiles/profile/profile_back_button.cpp` | 135 | `profile_back_button.cpp:L30-L70`: text and subtext layout, elision width calculation, unread counter badge subscription | `crates/gpui_telegram_desktop/src/views/drawer/profile_back_button.rs` | `ProfileBackButton` |
| 46 | `Telegram/SourceFiles/profile/profile_block_widget.h` | 59 | `BlockWidget`: `_peer`, `_title`, `contentTop()`, `resizeGetHeight()`, `contentSizeUpdated()`, `paintTitle()` | `crates/gpui_telegram_desktop/src/views/drawer/profile_block_widget.rs` | `ProfileBlockWidget` |
| 47 | `Telegram/SourceFiles/profile/profile_block_widget.cpp` | 44 | `profile_block_widget.cpp:L18-L40`: block container with header title, margin `st::profileBlockMarginTop`, `st::profileBlockTitleHeight` | `crates/gpui_telegram_desktop/src/views/drawer/profile_block_widget.rs` | `ProfileBlockWidget` |
| 48 | `Telegram/SourceFiles/profile/profile_cover_drop_area.h` | 44 | `CoverDropArea`: `_title`, `_subtitle`, `_a_appearance`, `showAnimated`, `hideAnimated`, `HideFinishCallback` | `crates/gpui_telegram_desktop/src/views/drawer/profile_cover_drop_area.rs` | `ProfileCoverDropArea` |
| 49 | `Telegram/SourceFiles/profile/profile_cover_drop_area.cpp` | 96 | `profile_cover_drop_area.cpp:L25-L60`: drag-and-drop cover drop zone, dash border `profileDropAreaBorderWidth = 3px`, fade-in animation `200ms` | `crates/gpui_telegram_desktop/src/views/drawer/profile_cover_drop_area.rs` | `ProfileCoverDropArea` |

---

## 3. Dynamic Rust / GPUI Architecture & Module Mapping

Under Mandate #17, the 42 upstream files (and companion root extensions) map to clean, zero-boilerplate Rust modules in `crates/gpui_telegram_desktop/`:

```
crates/gpui_telegram_desktop/src/
├── theme/info/
│   ├── channel_earn_style.rs                 # ChannelEarnChannelStatisticsEarnStyleSheet
│   ├── giveaway_style.rs                     # GiveawayChannelStatisticsBoostsGiveawayStyleSheet
│   ├── info_community_requests_widget_style.rs # InfoCommunityRequestsWidgetCommunityRequestsStyleSheet
│   ├── info_levels_style.rs                  # InfoLevelsProfileStyleSheet
│   ├── info_profile_actions_style.rs         # InfoProfileActionsProfileStyleSheet
│   ├── info_profile_top_bar_style.rs         # InfoProfileTopBarProfileStyleSheet
│   ├── info_style.rs                         # InfoStyleSheet
│   ├── info_userpic_builder_style.rs         # InfoUserpicBuilderUserpicStyleSheet
│   ├── overview_style.rs                     # OverviewStyleSheet
│   └── profile_style.rs                      # ProfileStyleSheet
└── views/drawer/
    ├── info_content_widget.rs                # InfoContentWidget: Base drawer widget & memento interface
    ├── info_controller.rs                    # InfoController: Key, Section, and Wrap navigation coordinator
    ├── info_flexible_scroll.rs               # InfoFlexibleScroll: Elastic cover scroll & snap physics
    ├── info_layer_widget.rs                  # InfoLayerWidget: Modal floating layer presentation
    ├── info_memento.rs                       # InfoMemento: Stack preservation and state serialization
    ├── info_section_widget.rs                # InfoSectionWidget: Third-column embedded section container
    ├── info_top_bar.rs                       # InfoTopBar: Standard & Layer action title bar
    ├── info_wrap_widget.rs                   # InfoWrapWidget: Main drawer shell & history stack
    ├── overview_checkbox.rs                  # OverviewCheckbox: Media batch selection checkbox
    ├── overview_layout.rs                    # OverviewLayout: Virtual photo/video/doc item renderer
    ├── overview_layout_delegate.rs           # OverviewLayoutDelegate: Heavy item lifecycle delegate
    ├── profile_back_button.rs                # ProfileBackButton: Animated titlebar back button with unread count
    ├── profile_block_widget.rs               # ProfileBlockWidget: Card section block with uppercase header
    ├── profile_cover_drop_area.rs            # ProfileCoverDropArea: Drag & drop cover upload target
    ├── bot/
    │   ├── earn/
    │   │   ├── info_bot_earn_list.rs         # InfoBotEarnListBotEarn: Stars revenue charts & withdrawal
    │   │   └── info_bot_earn_widget.rs       # InfoBotEarnWidgetBotEarn: Stars earn drawer section
    │   └── starref/
    │       ├── info_bot_starref_common.rs    # InfoBotStarrefCommonBotStarref: Affiliate formatting & bubbles
    │       ├── info_bot_starref_join_widget.rs # InfoBotStarrefJoinWidgetBotStarref: StarRef program browser
    │       └── info_bot_starref_setup_widget.rs# InfoBotStarrefSetupWidgetBotStarref: Program commission creator
    └── settings/
        └── info_settings_widget.rs           # InfoSettingsWidgetSettings: Settings drawer section
```

---

## 4. Deep Technical Analysis & Implementation Details

### 4.1 Info Settings Widget & Flexible Navigation (`info_settings_widget.cpp:L42-L160`)
- **Container Structure**: `InfoSettingsWidgetSettings` wraps sub-section content (`::Settings::AbstractSection`) inside `setupFlexibleInnerWidget`. It delegates top/bottom pinning (`_pinnedToTop`, `_pinnedToBottom`) and binds outside clicks (`closeByOutsideClick`).
- **Flexible Top Bar Motion**: When `hasFlexibleTopBar()` is active, the height delta `heightDiff = _pinnedToTop->maximumHeight() - _pinnedToTop->minimumHeight()` dynamically expands `contentHeightValue`. As scroll position `top` advances, `_inner` offsets up to `std::min(0, heightDiff - top)` while the pinned bar compresses to `std::max(current + minHeight, 0)`.
- **Navigation Stack Routing**: The widget listens to `_inner->sectionShowOther()` to push target `Section::SettingsType` settings screens, `sectionShowBack()` to pop frames, and streams `_removesFromStack` to prune obsolete route segments from history.
- **Shadow Invariants**: Unlike regular drawer sections which keep shadows permanently active (`rpl::single(true)`), `Settings::MainId` and `Settings::InformationId` calculate dynamic shadow visibility matching `ContentWidget::desiredShadowVisibility()`.

### 4.2 Bot StarRef Affiliate Program & Commission Rules (`info_bot_starref_setup_widget.cpp:L44-L100`)
- **Default Constraints**:
  - `kCommissionDefault = 200` (representing 2.00% commission = 200 basis points).
  - `kDurationDefault = 12` (12 months standard affiliate duration).
  - `kDurationForeverValue = 999` (eternal program duration indicator).
  - `kDisabledFade = 0.3` (opacity scalar for disabled controls).
- **Commission Calculations**: `ValueForCommission(state)` returns `state.program.commission` if set, falling back to `kCommissionDefault`. `ValueForDurationMonths(state)` returns existing duration, or `kDurationForeverValue` if modifying an active program, or `kDurationDefault` for new programs.
- **Affiliate Program Pagination**: `ListController` in `info_bot_starref_join_widget.cpp` batches active/suggested programs with `kPerPage = 50`, supporting three sorting regimes: `Profitability` (revenue per thousand impressions), `Revenue` (absolute Stars volume), and `Date` (newest first).

### 4.3 Bot Earn Statistics, Balance Overview & Revenue Graph (`info_bot_earn_list.cpp:L118-L260`)
- **Stars Revenue Conversion**: `data.usdRate` acts as the exchange multiplier from Telegram Stars to USD. Fractional amounts format with `kMinorLength = 3` decimal digits via `ToUsd(v, multiplier, 3)`.
- **Three-Tier Balance Breakdown**:
  1. `availableBalance`: Ready for withdrawal or ads purchases, displayed with star glyph and live USD conversion.
  2. `currentBalance`: Unvested pending rewards undergoing lockup.
  3. `overallRevenue`: Lifetime accumulated Stars revenue.
- **Chart Representation**: When `revenueGraph.chart` is present, it renders as a `Statistic::ChartViewType::StackBar` with currency rate scaling applied, separated by `Ui::AddDivider`.
- **Withdrawal Controls**: `::Settings::AddWithdrawalWidget` activates the Telegram Fragment withdrawal flow when `isWithdrawalEnabled` is true and next withdrawal date has elapsed.
### 4.4 Info Content Widget & Memento Lifecycle (`info_content_widget.cpp:L95-L260`)
- **Base Widget Hierarchy**: `ContentWidget` serves as the abstract root for all drawer pages. It owns `_scroll: Ui::ScrollArea`, manages `_scrollTopSkip` and `_scrollBottomSkip`, and synchronizes swipe back gestures (`replaceSwipeHandler`, `SwipeInterceptor`).
- **Flexible Scrolling Physics**: `setupFlexibleRegularScroll` configures elastic cover scrolling with animation time step `kScrollStepTime = 260ms` (`info_flexible_scroll.cpp:L30`). It counts snap anchors using `st::infoProfileTopBarHeightMax`, `Step1`, and `Step2` to prevent awkward halfway-expanded profile cover positions.
- **State Preservation**: `ContentMemento` serializes the scroll position (`scrollTopSave()`), active query in the search bar (`searchFieldQuery()`), and section-specific state (`sectionState()`) across navigation stack pushes and pops.

### 4.5 Info Wrap Widget, Layer Widget & Top Bar Hierarchy (`info_wrap_widget.cpp`, `info_layer_widget.cpp`, `info_top_bar.cpp`)
- **Presentation Modes (`Wrap`)**:
  - `Wrap::Layer`: Centered floating modal overlay with `st::boxRadius` rounded corners and backdrop dismissal.
  - `Wrap::Side`: Right-side collapsible 320–392px drawer panel.
  - `Wrap::Narrow`: Compact split-view panel for compressed window widths.
  - `Wrap::Search`: Full-height search results view.
  - `Wrap::StoryAlbumEdit`: Story album curation layout.
- **Layer Sizing Clamps**:
  - `MinimalSupportedWidth()` = `st::infoMinimalWidth (324px) + 2 * st::infoMinimalLayerMargin (48px) = 420px`.
  - Max width is clamped to `st::infoDesiredWidth = 392px`.
  - Multi-pass layout loop guarantees geometry convergence within `kMaxAttempts = 16`.
- **Top Bar Dynamics**: Standard top bar measures 54px (`infoTopBarHeight`), whereas the modal layer top bar measures 56px (`infoLayerTopBarHeight`). Selection mode activates dynamic actions: forward, delete, toggle story pin in profile, and cancel selection.

### 4.6 Overview Layout, Virtual Item Rendering & Checkbox Selection (`overview_layout.cpp:L90-L2000`)
- **Virtual Media Items**:
  - `Photo`: Thumbnail preview with spoiler blur overlay, aspect ratio calculation, and click-to-open lightbox handler.
  - `Video`: Cover photo with radial download progress, formatted duration label, and spoiler support.
  - `Voice`: 100-bar waveform visualization with played progress tracking and playlist integration.
  - `Document`: File extension badges (`overviewFileExtFont = 18px semibold`), generic 4-color categorization, and drag-and-drop preview export.
  - `Link`: OpenGraph preview title, letter initial avatar fallback, and multi-link description text.
- **Selection Checkbox**: `OverviewCheckbox` operates with outer size `29px`, scale down to `overviewCheckPressedSize = 0.8` on press, and `check` glyph icon offset `point(4px, 8px)`.

### 4.7 Profile Root Widgets (`profile_back_button.cpp`, `profile_block_widget.cpp`, `profile_cover_drop_area.cpp`)
- **Back Button**: Animates title, subtitle, and unread message counter badge with smooth alpha transition and elision caching.
- **Block Widget**: Groups related profile rows under an uppercase header label with `st::profileBlockMarginTop = 14px` and `st::profileBlockTitleHeight = 24px`.
- **Cover Drop Area**: Presents a dashed drop zone with `profileDropAreaBorderWidth = 3px` and 200ms fade-in animation for changing profile covers.

### 4.8 Complete Style Tokens and Metrics Summary
- `infoTopBarHeight = 54px`, `infoLayerTopBarHeight = 56px`.
- `infoMinimalWidth = 324px`, `infoDesiredWidth = 392px`, `infoMinimalLayerMargin = 48px`.
- `infoProfileCover.height = 108px`, `infoProfilePhotoSize = 72px`.
- `infoProfileTopBarHeightMax = 236px` (`180px + 56px`).
- `overviewFileLayout.maxWidth = 520px`, `linksPhotoSize = 46px`.
- `giveawayGiftCodeStartButton.height = 42px`, `giveawayGiftCodeTopHeight = 195px`.

---

## 5. Verification & Acceptance Protocol

1. **Gate 6.6 Zero-Omission Source Verification**: Run `python3 scripts/verify_ledger_closure.py` to confirm all 42 assigned upstream sources are present in the design doc.
2. **Boilerplate Burndown Verification**: Inspect all target files to confirm 0 instances of Patterns A, B, C, and D.
3. **Compilation & Test Suite**:
   ```bash
   cargo check --package gpui_telegram_desktop
   python3 scripts/clean_build_garbage.py
   ```
4. **Mandate #20 Compliance**: All file write operations must not exceed 200 lines per call.
