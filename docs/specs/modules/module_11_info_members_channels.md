# Module 11 (info & settings) — Slice 2: Info Members & Channels Specification

## 1. Executive Summary & Architectural Role

Module 11 Slice 2 implements the member lists, channel subscriber rosters, community join requests, profile tabs host, and header top-bar presentation for the 320px collapsible right info drawer in pure Rust and GPUI. It bridges core peer relationship models with interactive participant management and multi-tab profile layouts.

Key architectural responsibilities include:
- **Member Rosters & Role Rights Management**: Group and channel member listing, search query filtering, and role-based categorization (`Rights::Normal`, `Rights::Admin`, `Rights::Creator`) with interactive badges (`lng_owner_badge`, `lng_admin_badge`), custom title/rank pills, tag editing (`lng_context_add_my_tag`), and kick/removal triggers (`kTagElement = 1`, `kRemoveElement = 2`, `kEnableSearchMembersAfterCount = 20`).
- **Mutual Common Groups & Discovery**: Common chat discovery between contacts via MTProto pagination (`messages.getCommonChats`, `kCommonGroupsPerPage = 40`, `kCommonGroupsSearchAfter = 20`, `preloadGroupId`).
- **Community Join Requests & Moderation**: Admin request review panel with batch approval/rejection (`requestsAddAllButton`, `requestsDeclineAllButton`, `kPerPage = 100`, `kAcceptButton = 1`, `kRejectButton = 2`, `kUserLink = 3`, `kHiddenPill = 4`), in-flight row dimming (`kPendingRowOpacity = 0.4`), and temporary undo toasts (`kUndoToastDuration = 3000ms`).
- **Profile Tabstrip & Multi-Tab Host Framework**: Horizontal scrollable tabstrip (`TabsStrip`, `StripTab`, `Button`) with inertial wheel scrolling, active underline indicator with cubic ease, vertical scroll tracking (`kScrollDuration = 150ms`, `kVerticalScrollGrace = 250ms`, `kVerticalScrollRest = 1000ms`, `kF = 0.5`), and tab skeleton loaders (`TabsHost`, `MediaTabContent`).
- **Collapsible Profile Section Stack**: Dynamic accordion container (`SectionStack`, `RowType::Block`, `RowType::PlainSeparator`, `RowType::TextSeparator`) with zero-duplicate separator elimination via `ComputeVisibility(kinds, intrinsic)` (`blockVisibleAbove && blockVisibleBelow`).
- **Flexible Profile Top Bar & Action Buttons**: Animated header banner (`InfoProfileTopBar`) supporting avatar parallax, story ring status (`kStoryOutlineFadeEnd = 0.4`, `kStoryOutlineFadeRange = 0.6`), star gift badge glares (`kWaitBeforeGiftBadge = 1000ms`, `kGiftBadgeGlares = 3`, `kMaxPinnedToTopGifts = 6`), contrast ratio enforcement (`kMinContrast = 5.5`), and animated morphing icon buttons (`TopBarActionButton`, `kIconFadeStart = 0.4`, `kIconFadeRange = 0.6`).

---

## 2. Upstream C++ 100% Reading Manifest (All 42 Upstream Files)

| Upstream Source Path | Lines | Key C++ Classes / Symbols & Logic Extracted | Dynamic GPUI Target Path | Target Symbol |
| :--- | :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/info/members/info_members_widget.h` | 73 | `Info::Members::Memento`, `Info::Members::Widget`, `PeerListState`, `MembersState` (`list`, `search`), `createWidget`, `showInternal`. | `crates/gpui_telegram_desktop/src/views/drawer/members/info_members_widget.rs` | `InfoMembersWidgetMembers` |
| `Telegram/SourceFiles/info/members/info_members_widget.cpp` | 110 | `info_members_widget.cpp:L20-L75`: `Memento::createWidget`, `Widget::showInternal`, `Widget::setInternalState`, `Widget::title` returning `tr::lng_profile_participants_section`, `Widget::saveState` and `restoreState`. | `crates/gpui_telegram_desktop/src/views/drawer/members/info_members_widget.rs` | `InfoMembersWidgetMembers` |
| `Telegram/SourceFiles/info/profile/info_profile_members.h` | 149 | `Info::Profile::MembersState`, `Info::Profile::Members`: `scrollToRequests`, `applySearchQuery`, `setGroupByRole`, `groupByRoleValue`, `rowsVisibleValue`, `desiredHeight`, `onlineCountValue`, `fullCountValue`. | `crates/gpui_telegram_desktop/src/views/drawer/profile/info_profile_members.rs` | `InfoProfileMembersProfile` |
| `Telegram/SourceFiles/info/profile/info_profile_members.cpp` | 497 | `info_profile_members.cpp:L40`: `kEnableSearchMembersAfterCount = 20`; `info_profile_members.cpp:L50-L160`: search field integration, header visibility, add member action (`tr::lng_channel_add_members`, `tr::lng_manage_peer_members`). | `crates/gpui_telegram_desktop/src/views/drawer/profile/info_profile_members.rs` | `InfoProfileMembersProfile` |
| `Telegram/SourceFiles/info/profile/info_profile_members_controllers.h` | 147 | `MemberListRow`, `MemberListRow::Rights` (`Normal`, `Admin`, `Creator`), `MemberListRow::Type`, `MemberListRow::TagMode`, `kTagElement = 1`, `kRemoveElement = 2`, `elementGeometry`, `paintPill`. | `crates/gpui_telegram_desktop/src/views/drawer/profile/info_profile_members_controllers.rs` | `InfoProfileMembersControllersProfile` |
| `Telegram/SourceFiles/info/profile/info_profile_members_controllers.cpp` | 553 | `info_profile_members_controllers.cpp:L35-L70`: `setType` with `lng_owner_badge`, `lng_admin_badge`, `lng_context_add_my_tag`, `lng_profile_kick`; `tagSize` with `st::memberTagPillPadding`, `max(w, h)`; `elementsPaint` hit testing. | `crates/gpui_telegram_desktop/src/views/drawer/profile/info_profile_members_controllers.rs` | `InfoProfileMembersControllersProfile` |
| `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_members.h` | 20 | Forward declaration `PeerData`, `Info::Profile::MembersTabAdapter`. | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/adapters/info_profile_tab_members.rs` | `InfoProfileTabMembersProfileTabsAdapters` |
| `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_members.cpp` | 151 | `info_profile_tab_members.cpp:L21`: `kEnableSearchMembersAfterCount = 20`; `MembersTabState`, `MembersTabAdapter`: child `Members` instantiation, height and scroll bindings, online count forwarding. | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/adapters/info_profile_tab_members.rs` | `InfoProfileTabMembersProfileTabsAdapters` |
| `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_chats.h` | 23 | Forward declaration `Main::Session`, `Info::Profile::ChatsTabAdapter`. | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/adapters/info_profile_tab_chats.rs` | `InfoProfileTabChatsProfileTabsAdapters` |
| `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_chats.cpp` | 218 | `info_profile_tab_chats.cpp:L28-L29`: `kSearchRequestDelay = crl::time(900)`, `kSearchPreloadHeight = 500`; `ChatsTabAdapter`: search query debounce, preload distance, chat row selection. | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/adapters/info_profile_tab_chats.rs` | `InfoProfileTabChatsProfileTabsAdapters` |
| `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_peer_lists.h` | 23 | Forward declaration `UserData`, `Info::Profile::WidgetTabAdapter`, `GiftsTabAdapter`. | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/adapters/info_profile_tab_peer_lists.rs` | `InfoProfileTabPeerListsProfileTabsAdapters` |
| `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_peer_lists.cpp` | 244 | `info_profile_tab_peer_lists.cpp:L30-L135`: `WidgetTabAdapter` generic peer list container; `GiftsTabAdapter`: star gift tab with filter by ribbon and pinned gift rendering. | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/adapters/info_profile_tab_peer_lists.rs` | `InfoProfileTabPeerListsProfileTabsAdapters` |
| `Telegram/SourceFiles/info/common_groups/info_common_groups_inner_widget.h` | 85 | `Info::CommonGroups::Memento`, `InnerWidget`: `scrollToRequests`, `desiredHeight`, `saveState`, `restoreState`. | `crates/gpui_telegram_desktop/src/views/drawer/common_groups/info_common_groups_inner_widget.rs` | `InfoCommonGroupsInnerWidgetCommonGroups` |
| `Telegram/SourceFiles/info/common_groups/info_common_groups_inner_widget.cpp` | 313 | `info_common_groups_inner_widget.cpp:L27-L28`: `kCommonGroupsPerPage = 40`, `kCommonGroupsSearchAfter = 20`; `ListController`: `messages.getCommonChats` max_id pagination, `SavedState` with `preloadGroupId`. | `crates/gpui_telegram_desktop/src/views/drawer/common_groups/info_common_groups_inner_widget.rs` | `InfoCommonGroupsInnerWidgetCommonGroups` |
| `Telegram/SourceFiles/info/common_groups/info_common_groups_widget.h` | 77 | `Info::CommonGroups::Widget`: `showInternal`, `setInternalState`, `title` returning `tr::lng_profile_common_groups_section`. | `crates/gpui_telegram_desktop/src/views/drawer/common_groups/info_common_groups_widget.rs` | `InfoCommonGroupsWidgetCommonGroups` |
| `Telegram/SourceFiles/info/common_groups/info_common_groups_widget.cpp` | 111 | `info_common_groups_widget.cpp:L20-L80`: `createWidget`, title producer binding, search field query routing to `searchQueryChanged`, state save/restore. | `crates/gpui_telegram_desktop/src/views/drawer/common_groups/info_common_groups_widget.rs` | `InfoCommonGroupsWidgetCommonGroups` |
| `Telegram/SourceFiles/info/community/info_community_widget.h` | 71 | `Info::Community::Memento`, `Info::Community::Widget`: community details, linked channel/supergroup binding, member counts. | `crates/gpui_telegram_desktop/src/views/drawer/community/info_community_widget.rs` | `InfoCommunityWidgetCommunity` |
| `Telegram/SourceFiles/info/community/info_community_widget.cpp` | 315 | `info_community_widget.cpp:L31-L120`: `InnerWidget`: `chatsStatusValue` formatting, flexible top bar integration, back navigation toggles. | `crates/gpui_telegram_desktop/src/views/drawer/community/info_community_widget.rs` | `InfoCommunityWidgetCommunity` |
| `Telegram/SourceFiles/info/community_requests/info_community_requests_widget.h` | 75 | `Info::CommunityRequests::Memento`, `Info::CommunityRequests::Widget`: join request management for private channels and communities. | `crates/gpui_telegram_desktop/src/views/drawer/community_requests/info_community_requests_widget.rs` | `InfoCommunityRequestsWidgetCommunityRequests` |
| `Telegram/SourceFiles/info/community_requests/info_community_requests_widget.cpp` | 1539 | `info_community_requests_widget.cpp:L58-L64`: `kPerPage = 100`, `kAcceptButton = 1`, `kRejectButton = 2`, `kUserLink = 3`, `kHiddenPill = 4`, `kUndoToastDuration = 3000ms`, `kPendingRowOpacity = 0.4`; `MakeUndoButton`, batch accept/dismiss. | `crates/gpui_telegram_desktop/src/views/drawer/community_requests/info_community_requests_widget.rs` | `InfoCommunityRequestsWidgetCommunityRequests` |
| `Telegram/SourceFiles/info/community_requests/info_community_requests_widget.style` | 44 | CSS style sheet: `communityRequestsTitlePadding`, `requestSuggestBadgeSize: 16px`, `requestMembersTop: 11px`, `requestsAddAllButton: 32px`, `requestsDeclineAllButton: 32px`. | `crates/gpui_telegram_desktop/src/theme/info/info_community_requests_widget_style.rs` | `InfoCommunityRequestsWidgetCommunityRequestsStyleSheet` |
| `Telegram/SourceFiles/info/requests_list/info_requests_list_widget.h` | 67 | `Info::RequestsList::Memento`, `Info::RequestsList::Widget`: `showInternal`, `setInternalState`, `title` returning `tr::lng_profile_join_requests_section`. | `crates/gpui_telegram_desktop/src/views/drawer/requests_list/info_requests_list_widget.rs` | `InfoRequestsListWidgetRequestsList` |
| `Telegram/SourceFiles/info/requests_list/info_requests_list_widget.cpp` | 271 | `info_requests_list_widget.cpp:L23-L96`: `InnerWidget`, `RequestsBoxController` integration, search query subscription forwarding to `content()->searchQueryChanged`, state persistence. | `crates/gpui_telegram_desktop/src/views/drawer/requests_list/info_requests_list_widget.rs` | `InfoRequestsListWidgetRequestsList` |
| `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_sub_controller.h` | 46 | `MediaSubController` forward declaration, `Info::Profile::SubControllerTabAdapter`. | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/adapters/info_profile_tab_sub_controller.rs` | `InfoProfileTabSubControllerProfileTabsAdapters` |
| `Telegram/SourceFiles/info/profile/tabs/adapters/info_profile_tab_sub_controller.cpp` | 74 | `info_profile_tab_sub_controller.cpp:L18-L65`: `SubControllerTabAdapter`: delegate sub-controller creation, media overview forwarding, lifecycle binding. | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/adapters/info_profile_tab_sub_controller.rs` | `InfoProfileTabSubControllerProfileTabsAdapters` |
| `Telegram/SourceFiles/info/profile/tabs/info_profile_tab_content.h` | 85 | `enum class ProfileTab`, `struct MediaTabDescriptor`, `struct MediaTabContext`, `class MediaTabContent`, `class MediaTabState`. | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/info_profile_tab_content.rs` | `InfoProfileTabContentProfileTabs` |
| `Telegram/SourceFiles/info/profile/tabs/info_profile_tab_skeleton.h` | 23 | Forward declaration `Ui::RpWidget`, `Info::Profile::MakeTabSkeleton`. | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/info_profile_tab_skeleton.rs` | `InfoProfileTabSkeletonProfileTabs` |
| `Telegram/SourceFiles/info/profile/tabs/info_profile_tab_skeleton.cpp` | 155 | `info_profile_tab_skeleton.cpp:L25-L140`: `MakeTabSkeleton`, `struct State`, shimmer gradient animation, placeholder rows count and dimensions. | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/info_profile_tab_skeleton.rs` | `InfoProfileTabSkeletonProfileTabs` |
| `Telegram/SourceFiles/info/profile/tabs/info_profile_tab_top_bar_bindings.h` | 31 | `struct MenuCallback`, `struct TabTopBarBindings`: title producer, action button descriptor, menu population callback. | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/info_profile_tab_top_bar_bindings.rs` | `InfoProfileTabTopBarBindingsProfileTabs` |
| `Telegram/SourceFiles/info/profile/tabs/info_profile_tabs_host.h` | 151 | `kOptionProfileMediaTabs`, `UseProfileMediaTabs`, `struct TabsState`, `class TabsHost`: `activeTabValue`, `stripWidget`, `trackVerticalScroll`. | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/info_profile_tabs_host.rs` | `InfoProfileTabsHostProfileTabs` |
| `Telegram/SourceFiles/info/profile/tabs/info_profile_tabs_host.cpp` | 898 | `info_profile_tabs_host.cpp:L45-L250`: active tab switching with horizontal slide animation, scroll position memoization per tab, search bar activation. | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/info_profile_tabs_host.rs` | `InfoProfileTabsHostProfileTabs` |
| `Telegram/SourceFiles/info/profile/tabs/info_profile_tabs_strip.h` | 110 | `struct StripTab`, `class TabsStrip`: `setTabs`, `setActiveTab`, `trackVerticalScroll`, `Button` (`geometry`, `text`, `ripple`). | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/info_profile_tabs_strip.rs` | `InfoProfileTabsStripProfileTabs` |
| `Telegram/SourceFiles/info/profile/tabs/info_profile_tabs_strip.cpp` | 533 | `info_profile_tabs_strip.cpp:L25-L27`: `kScrollDuration = 150ms`, `kVerticalScrollGrace = 250ms`, `kVerticalScrollRest = 1000ms`, `kF = 0.5`; active indicator interpolation, wheel event physics. | `crates/gpui_telegram_desktop/src/views/drawer/profile/tabs/info_profile_tabs_strip.rs` | `InfoProfileTabsStripProfileTabs` |
| `Telegram/SourceFiles/info/profile/info_profile_section_stack.h` | 63 | `struct Section`, `class SectionStack`, `enum class RowType` (`Block`, `PlainSeparator`, `TextSeparator`), `ComputeVisibility`. | `crates/gpui_telegram_desktop/src/views/drawer/profile/info_profile_section_stack.rs` | `InfoProfileSectionStackProfile` |
| `Telegram/SourceFiles/info/profile/info_profile_section_stack.cpp` | 248 | `info_profile_section_stack.cpp:L151-L176`: `ComputeVisibility` algorithm (`blockVisibleAbove && blockVisibleBelow`); `finalize`: animated slide wraps with `st::infoSlideDuration`. | `crates/gpui_telegram_desktop/src/views/drawer/profile/info_profile_section_stack.rs` | `InfoProfileSectionStackProfile` |
| `Telegram/SourceFiles/info/profile/info_profile_top_bar.h` | 397 | `class TopBar`: parallax avatar geometry, story outline segments, star gifts glare tooltip, topic badge view, color profile palettes. | `crates/gpui_telegram_desktop/src/views/drawer/profile/info_profile_top_bar.rs` | `InfoProfileTopBarProfile` |
| `Telegram/SourceFiles/info/profile/info_profile_top_bar.cpp` | 4005 | `info_profile_top_bar.cpp:L154-L159`: `kWaitBeforeGiftBadge = 1000ms`, `kGiftBadgeGlares = 3`, `kMinPatternRadius = 8`, `kStoryOutlineFadeEnd = 0.4`, `kStoryOutlineFadeRange = 0.6`, `kSwapMoveAmplitude = 0.3`; `L660`: `kMinContrast = 5.5`; `L3502`: `kMaxPinnedToTopGifts = 6`. | `crates/gpui_telegram_desktop/src/views/drawer/profile/info_profile_top_bar.rs` | `InfoProfileTopBarProfile` |
| `Telegram/SourceFiles/info/profile/info_profile_top_bar.style` | 139 | CSS style sheet: `infoProfileTopBarPhotoSize: 80px`, `infoProfileTopBarTitleTop: 113px`, `infoProfileTopBarStatusTop: 134px`, `infoProfileTopBarGiftSize: 20px`. | `crates/gpui_telegram_desktop/src/theme/info/info_profile_top_bar_style.rs` | `InfoProfileTopBarProfileStyleSheet` |
| `Telegram/SourceFiles/info/profile/info_profile_top_bar_action_button.h` | 77 | `struct TopBarActionButtonStyle`, `class TopBarActionButton`: icon/lottie action buttons, toggle state transitions. | `crates/gpui_telegram_desktop/src/views/drawer/profile/info_profile_top_bar_action_button.rs` | `InfoProfileTopBarActionButtonProfile` |
| `Telegram/SourceFiles/info/profile/info_profile_top_bar_action_button.cpp` | 221 | `info_profile_top_bar_action_button.cpp:L21-L22`: `kIconFadeStart = 0.4`, `kIconFadeRange = 0.6`; icon fade transition, ripple mask, state toggle animation. | `crates/gpui_telegram_desktop/src/views/drawer/profile/info_profile_top_bar_action_button.rs` | `InfoProfileTopBarActionButtonProfile` |
| `Telegram/SourceFiles/info/profile/info_profile_text.h` | 45 | `struct TextWithLabel`, `CreateTextWithLabel`: bio/phone/username layout with copyable text and secondary descriptor label. | `crates/gpui_telegram_desktop/src/views/drawer/profile/info_profile_text.rs` | `InfoProfileTextProfile` |
| `Telegram/SourceFiles/info/profile/info_profile_text.cpp` | 72 | `info_profile_text.cpp:L22-L69`: `CreateTextWithLabel` slide wrap instantiation with `st::infoSlideDuration`, empty string hide filter, selectable text layout. | `crates/gpui_telegram_desktop/src/views/drawer/profile/info_profile_text.rs` | `InfoProfileTextProfile` |

---

## 3. Dynamic Rust / GPUI Architecture & Module Mapping

Under Mandate #17, the 42 C++ upstream files are mapped to authentic, cohesive Rust modules in `crates/gpui_telegram_desktop/src/views/drawer/` and theme directories:

```
crates/gpui_telegram_desktop/src/
├── theme/info/
│   ├── info_community_requests_widget_style.rs     # InfoCommunityRequestsWidgetCommunityRequestsStyleSheet
│   └── info_profile_top_bar_style.rs               # InfoProfileTopBarProfileStyleSheet
└── views/drawer/
    ├── common_groups/
    │   ├── info_common_groups_inner_widget.rs      # InfoCommonGroupsInnerWidgetCommonGroups: common groups list & MTProto paging
    │   └── info_common_groups_widget.rs            # InfoCommonGroupsWidgetCommonGroups: outer container & search bar
    ├── community/
    │   └── info_community_widget.rs                # InfoCommunityWidgetCommunity: linked channels & supergroup card
    ├── community_requests/
    │   └── info_community_requests_widget.rs       # InfoCommunityRequestsWidgetCommunityRequests: join requests list & batch actions
    ├── members/
    │   └── info_members_widget.rs                  # InfoMembersWidgetMembers: group & channel participants container
    ├── profile/
    │   ├── info_profile_members.rs                 # InfoProfileMembersProfile: member list view & search
    │   ├── info_profile_members_controllers.rs     # InfoProfileMembersControllersProfile: MemberListRow, rights & pill tags
    │   ├── info_profile_section_stack.rs           # InfoProfileSectionStackProfile: SectionStack visibility algorithm
    │   ├── info_profile_text.rs                    # InfoProfileTextProfile: text with label & copy actions
    │   ├── info_profile_top_bar.rs                 # InfoProfileTopBarProfile: parallax header, stories, badges, glares
    │   ├── info_profile_top_bar_action_button.rs   # InfoProfileTopBarActionButtonProfile: morphing top bar icon buttons
    │   └── tabs/
    │       ├── info_profile_tab_content.rs         # InfoProfileTabContentProfileTabs: tab content trait & descriptors
    │       ├── info_profile_tab_skeleton.rs        # InfoProfileTabSkeletonProfileTabs: placeholder shimmer loaders
    │       ├── info_profile_tab_top_bar_bindings.rs# InfoProfileTabTopBarBindingsProfileTabs: tab-to-topbar bridge
    │       ├── info_profile_tabs_host.rs           # InfoProfileTabsHostProfileTabs: multi-tab slide host & cache
    │       ├── info_profile_tabs_strip.rs          # InfoProfileTabsStripProfileTabs: horizontal scroll tabstrip
    │       └── adapters/
    │           ├── info_profile_tab_chats.rs       # InfoProfileTabChatsProfileTabsAdapters: chats & channels tab adapter
    │           ├── info_profile_tab_members.rs     # InfoProfileTabMembersProfileTabsAdapters: members tab adapter
    │           ├── info_profile_tab_peer_lists.rs  # InfoProfileTabPeerListsProfileTabsAdapters: peer list & gifts adapter
    │           └── info_profile_tab_sub_controller.rs # InfoProfileTabSubControllerProfileTabsAdapters: sub-controller bridge
    └── requests_list/
        └── info_requests_list_widget.rs            # InfoRequestsListWidgetRequestsList: channel join requests router
```


## 4. Deep Technical Analysis & Implementation Details

### 4.1 Member List Roles, Badges & Rights (`info_profile_members_controllers.cpp:L35-L120`)
- **Rights Enumeration**: Participants are classified via `enum class Rights { Normal, Admin, Creator }` (`info_profile_members_controllers.h:L30-L34`).
- **Tag Mode State Machine**:
  - `TagMode::AddTag`: If `canAddTag = true`, displays interactive prompt `lng_context_add_my_tag` (`info_profile_members_controllers.cpp:L40-L41`).
  - `TagMode::AdminPill`: If `rank` is non-empty and rights are Admin/Creator, or if `rights == Creator` without custom rank, displays `lng_owner_badge` (`info_profile_members_controllers.cpp:L48-L50`). If `rights == Admin`, displays `lng_admin_badge` (`info_profile_members_controllers.cpp:L51-L53`).
  - `TagMode::NormalText`: For regular members with a custom title or description (`info_profile_members_controllers.cpp:L46`).
- **Pill Geometry & Hit Elements**:
  - `kTagElement = 1`, `kRemoveElement = 2` (`info_profile_members_controllers.h:L46-L47`).
  - Tag pill size calculation: `st::memberTagPillPadding`, height is `padding.top + font.height + padding.bottom`, width is `max(padding.left + textWidth + padding.right, height)` to enforce pill capsule rounding (`info_profile_members_controllers.cpp:L95-L100`).
  - Kick action button: Displays `lng_profile_kick` if `canRemove = true` (`info_profile_members_controllers.cpp:L61-L69`).
  - Search trigger: `kEnableSearchMembersAfterCount = 20` (`info_profile_members.cpp:L40`). When total participants <= 20, the search bar is hidden to conserve vertical drawer space.

### 4.2 Mutual Common Groups Discovery & Paging (`info_common_groups_inner_widget.cpp:L25-L60`)
- **Pagination Constants**: `kCommonGroupsPerPage = 40`, `kCommonGroupsSearchAfter = 20` (`info_common_groups_inner_widget.cpp:L27-L28`).
- **State Preservation**: `SavedState` tracks `preloadGroupId` (last group ID fetched for cursor pagination), `allLoaded: bool`, and `wasLoading: bool` (`info_common_groups_inner_widget.cpp:L54-L58`).
- **Search Routing**: Search query debounce updates filtered common groups in memory or initiates server-side filtering via `searchQueryChanged`.

### 4.3 Community Join Requests & Moderation (`info_community_requests_widget.cpp:L56-L115`)
- **Constants**:
  - `kPerPage = 100` (`info_community_requests_widget.cpp:L58`).
  - Elements: `kAcceptButton = 1`, `kRejectButton = 2`, `kUserLink = 3`, `kHiddenPill = 4` (`info_community_requests_widget.cpp:L59-L62`).
  - Undo Duration: `kUndoToastDuration = 3000ms` (`info_community_requests_widget.cpp:L63`).
  - Row In-Flight Dimming: `kPendingRowOpacity = 0.4` (`info_community_requests_widget.cpp:L64`).
- **Undo Toast Controller**: Triggering accept or dismiss executes an optimistic local removal while scheduling a 3-second countdown undo bar (`MakeUndoButton`, `st::toastUndoFont`). If the user clicks Undo before 3000ms, the request is restored; otherwise, the MTProto call (`messages.hideChatJoinRequest`) is committed.

### 4.4 Profile Section Stack Visibility Algorithm (`info_profile_section_stack.cpp:L150-L176`)
- **Row Types**: `enum class RowType { Block, PlainSeparator, TextSeparator }` (`info_profile_section_stack.h:L40-L44`).
- **Zero-Redundant Separator Algorithm**:
  ```cpp
  // Plain separator at index i is visible ONLY if:
  // 1. There is an intrinsically visible Block above it before any other separator.
  // 2. There is an intrinsically visible Block below it before any other separator.
  result[i] = blockVisibleAbove(i) && blockVisibleBelow(i);
  ```
  This mathematically eliminates leading separators, trailing separators, and consecutive double separators when dynamic sections hide themselves.

### 4.5 Profile Tabs Strip & Host Layout (`info_profile_tabs_strip.cpp:L25-L30`, `info_profile_tabs_host.cpp:L45-L250`)
- **Animation Timers**:
  - `kScrollDuration = 150ms`: Active tab underline slide duration (`info_profile_tabs_strip.cpp:L25`).
  - `kVerticalScrollGrace = 250ms`, `kVerticalScrollRest = 1000ms`: Scroll rest dwell timings (`info_profile_tabs_strip.cpp:L26-L27`).
  - `kF = 0.5`: Fractional interpolation factor for momentum scroll settling (`info_profile_tabs_strip.cpp:L515`).
- **Tab Memoization (`TabsState`)**: `base::flat_map<QString, int> scrollTops` preserves exact scroll offsets per tab when switching between Members, Media, Files, Links, and Gifts.

### 4.6 Flexible Profile Top Bar & Action Buttons (`info_profile_top_bar.cpp:L154-L160`, `L660`, `L3502`)
- **Header Constants**:
  - Story Outline Fade: `kStoryOutlineFadeEnd = 0.4`, `kStoryOutlineFadeRange = 0.6` (`info_profile_top_bar.cpp:L157-L158`).
  - Star Gift Badges: `kWaitBeforeGiftBadge = 1000ms`, `kGiftBadgeGlares = 3`, `kMaxPinnedToTopGifts = 6` (`info_profile_top_bar.cpp:L154-L155, L3502`).
  - Minimum Text Contrast: `kMinContrast = 5.5` (`info_profile_top_bar.cpp:L660`).
  - Action Button Morphing: `kIconFadeStart = 0.4`, `kIconFadeRange = 0.6` (`info_profile_top_bar_action_button.cpp:L21-L22`).

---

## 5. Subsystem Test Plan (Coverage for >= 12 Subsystem Tests)

The test suite in `crates/gpui_telegram_desktop/tests/info_members_channels_subsystem_tests.rs` validates all 23 target components:
1. `test_member_rights_pill_and_tag_modes`: Validates `Rights::Creator` -> `lng_owner_badge`, `Rights::Admin` -> `lng_admin_badge`, and `canAddTag` -> `lng_context_add_my_tag`.
2. `test_member_pill_size_capsule_bounding`: Validates `st::memberTagPillPadding` and `max(width, height)` capsule rounding.
3. `test_member_search_visibility_threshold`: Validates `kEnableSearchMembersAfterCount = 20` threshold for search bar activation.
4. `test_common_groups_pagination_and_saved_state`: Validates `kCommonGroupsPerPage = 40` and `SavedState` cursor loading.
5. `test_community_requests_batch_actions_and_constants`: Validates `kPerPage = 100`, `kAcceptButton = 1`, `kRejectButton = 2`, `kPendingRowOpacity = 0.4`.
6. `test_community_requests_undo_toast_timer`: Validates `kUndoToastDuration = 3000ms` window and undo restore behavior.
7. `test_section_stack_visibility_algorithm`: Validates `ComputeVisibility` logic preventing duplicate/edge separators.
8. `test_tabs_strip_active_indicator_physics`: Validates `kScrollDuration = 150ms` and `kF = 0.5` active bar positioning.
9. `test_tabs_host_scroll_state_memoization`: Validates scroll position retention per tab across tab switches.
10. `test_top_bar_action_button_fade_curves`: Validates `kIconFadeStart = 0.4` and `kIconFadeRange = 0.6` morphing interpolation.
11. `test_top_bar_contrast_enforcement`: Validates `kMinContrast = 5.5` against custom background themes.
12. `test_top_bar_star_gift_glare_constants`: Validates `kWaitBeforeGiftBadge = 1000ms`, `kGiftBadgeGlares = 3`, and `kMaxPinnedToTopGifts = 6`.
