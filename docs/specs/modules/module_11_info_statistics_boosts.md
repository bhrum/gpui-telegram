# Module 11: Info & Settings — Statistics, Boosts, Earn & Recommendations Specification (Slice 4)

## Executive Summary
This design document establishes the authoritative C++ to Rust/GPUI architectural mapping for **Module 11 (`info & settings`) Slice 4**, covering 42 upstream Telegram Desktop source and style files. The subsystem covers:
- Channel, Supergroup, Post and Story Statistics (`info/statistics/`): Metrics tracking, interaction graphs, top forwarders, message previews, and XLSX export.
- Channel Boosts and Giveaways (`info/channel_statistics/boosts/`): Boost tier calculations, booster lists, prepaid giveaways, custom prizes, country filtering, and radial animated boost badges.
- Channel Monetization and Revenue Sharing (`info/channel_statistics/earn/`): TON and Telegram Stars revenue breakdown, financial formatting (nanoTON 1e9, insignificant zero chopping, USD conversion), currency icons vector rendering, and withdrawal flows.
- Bot Monetization (`info/bot/earn/`): Bot Star revenue and subscription payouts.
- Channel Recommendations (`info/similar_peers/`): Similar channels discovery, subscriber counting, and boost unlocking.
- Pixel-perfect theme and style tokens (`channel_earn.style`, `giveaway.style`, `statistics.style`).

---

## 100% Upstream Source Reading Manifest (Slice 4: 42 / 42 Files)

| Source Path | Line Range | Core Types & Symbols Extracted | Constants & Formulas | Target Rust / GPUI Path | Target Symbol |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `Telegram/SourceFiles/info/bot/earn/info_bot_earn_list.h` | L1-L67 | `InnerWidget`, `ShowRequest`, `Memento`, `Controller` | Bot earn list declarations, data loading triggers | `crates/gpui_telegram_desktop/src/views/drawer/bot/earn/info_bot_earn_list.rs` | `InfoBotEarnList` |
| `Telegram/SourceFiles/info/bot/earn/info_bot_earn_list.cpp` | L1-L464 | `InnerWidget::load`, `InnerWidget::fill`, `AddHeader` | Star revenue, subscription earnings, payout history list | `crates/gpui_telegram_desktop/src/views/drawer/bot/earn/info_bot_earn_list.rs` | `InfoBotEarnList` |
| `Telegram/SourceFiles/info/bot/earn/info_bot_earn_widget.h` | L1-L68 | `Widget`, `Memento` | Outer bot earn drawer widget wrapping `InnerWidget` | `crates/gpui_telegram_desktop/src/views/drawer/bot/earn/info_bot_earn_widget.rs` | `InfoBotEarnWidget` |
| `Telegram/SourceFiles/info/bot/earn/info_bot_earn_widget.cpp` | L1-L118 | `Widget::setInternalState`, `Widget::showFinished` | Scroll area embedding, header title bar binding | `crates/gpui_telegram_desktop/src/views/drawer/bot/earn/info_bot_earn_widget.rs` | `InfoBotEarnWidget` |
| `Telegram/SourceFiles/info/channel_statistics/boosts/create_giveaway_box.h` | L1-L29 | `CreateGiveawayBox`, `BoostPrepaidGiveaway` | Giveaway creation modal dialog builder | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/boosts/create_giveaway_box.rs` | `CreateGiveawayBox` |
| `Telegram/SourceFiles/info/channel_statistics/boosts/create_giveaway_box.cpp` | L1-L1577 | `GiveawayState`, `ThreeDaysAfterToday`, `UniqueIdFromCreditsOption`, `AddPremiumTopBarWithDefaultTitleBar` | `kDoneTooltipDuration = 5000ms`, `kAdditionalPrizeLengthMax = 128`, 3-day round-up to 5 min, XXH64 credits hash | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/boosts/create_giveaway_box.rs` | `CreateGiveawayBox` |
| `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/boost_badge.h` | L1-L47 | `BoostBadge`, `InfiniteRadialAnimationWidget`, `CreateBadge` | Boost badge geometry and animation parameters | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/boosts/giveaway/boost_badge.rs` | `BoostBadge` |
| `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/boost_badge.cpp` | L1-L190 | `CreateBadge`, `AddLabelWithBadgeToButton`, `InfiniteRadialAnimationWidget` | Rounded rect badge pill, `badgeh = 0 + badgeHeight`, `badgew = textWidth + padding.h`, opacity = `bgOpacity` | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/boosts/giveaway/boost_badge.rs` | `BoostBadge` |
| `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/giveaway.style` | L1-L279 | `giveawayGift`, `giveawayUsersBox`, `giveawayPadding`, `giveawayPrizeHeader` | Pixel-perfect paddings, icon margins, star gift dimensions | `crates/gpui_telegram_desktop/src/theme/info/giveaway_style.rs` | `GiveawayStyleSheet` |
| `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/giveaway_list_controllers.h` | L1-L113 | `GiveawayListController`, `ChannelRow`, `CountryRow` | Controller interfaces for selecting target channels, countries, users | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/boosts/giveaway/giveaway_list_controllers.rs` | `GiveawayListControllers` |
| `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/giveaway_list_controllers.cpp` | L1-L380 | `ChannelRow::paint`, `CountryRow::paint`, `ChannelRow::rowClicked` | Multi-select checkboxes, search query filters, row heights | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/boosts/giveaway/giveaway_list_controllers.rs` | `GiveawayListControllers` |
| `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/giveaway_type_row.h` | L1-L70 | `GiveawayTypeRow`, `Type` (`Random`, `Specific`, `Prepaid`) | Radio button selector row for giveaway participant pool | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/boosts/giveaway/giveaway_type_row.rs` | `GiveawayTypeRow` |
| `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/giveaway_type_row.cpp` | L1-L194 | `GiveawayTypeRow::paint`, `GiveawayTypeRow::mousePressEvent` | Radio check drawing, description sub-label, prepaid boost count | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/boosts/giveaway/giveaway_type_row.rs` | `GiveawayTypeRow` |
| `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/select_countries_box.h` | L1-L25 | `SelectCountriesBox` | Modal dialog for filtering giveaway eligibility by ISO country code | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/boosts/giveaway/select_countries_box.rs` | `SelectCountriesBox` |
| `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/select_countries_box.cpp` | L1-L229 | `SelectCountriesBox::prepare`, `SelectCountriesBox::applyFilter` | Country flag emoji lookup, ISO code sorting, multi-select list | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/boosts/giveaway/select_countries_box.rs` | `SelectCountriesBox` |
| `Telegram/SourceFiles/info/channel_statistics/boosts/info_boosts_inner_widget.h` | L1-L63 | `InnerWidget`, `Memento`, `ShowRequest` | Inner scroll content for boosts status, tiers, and booster members | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/boosts/info_boosts_inner_widget.rs` | `InfoBoostsInnerWidget` |
| `Telegram/SourceFiles/info/channel_statistics/boosts/info_boosts_inner_widget.cpp` | L1-L575 | `InnerWidget::load`, `InnerWidget::fill`, `FillOverview`, `FillShareLink` | Tier level math (`level`, `currentBoosts`, `nextLevelBoosts`), share link generator, prepaid card | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/boosts/info_boosts_inner_widget.rs` | `InfoBoostsInnerWidget` |
| `Telegram/SourceFiles/info/channel_statistics/boosts/info_boosts_widget.h` | L1-L67 | `Widget`, `Memento` | Outer drawer widget for boosts panel | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/boosts/info_boosts_widget.rs` | `InfoBoostsWidget` |
| `Telegram/SourceFiles/info/channel_statistics/boosts/info_boosts_widget.cpp` | L1-L118 | `Widget::setInternalState`, `Widget::showFinished` | Header initialization, tab sync with channel profile | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/boosts/info_boosts_widget.rs` | `InfoBoostsWidget` |
| `Telegram/SourceFiles/info/channel_statistics/earn/channel_earn.style` | L1-L151 | `channelEarnOverview`, `channelEarnTable`, `channelEarnWithdrawal` | Payout history table geometry, balance label styles | `crates/gpui_telegram_desktop/src/theme/info/channel_earn_style.rs` | `ChannelEarnStyleSheet` |
| `Telegram/SourceFiles/info/channel_statistics/earn/earn_format.h` | L1-L27 | `MajorPart`, `MinorPart`, `ToUsd` | Financial amount string decomposition declarations | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/earn/earn_format.rs` | `EarnFormat` |
| `Telegram/SourceFiles/info/channel_statistics/earn/earn_format.cpp` | L1-L95 | `MajorPart`, `MinorPart`, `ToUsd` | `kMinorPartLength = 9`, `kMaxChoppedZero = 7`, `kZero = '0'`, `kApproximately = 0x2248` (≈ $ rate formatting) | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/earn/earn_format.rs` | `EarnFormat` |
| `Telegram/SourceFiles/info/channel_statistics/earn/earn_icons.h` | L1-L51 | `CustomEmoji`, `IconDescriptor` | Currency and star badges icon renderer declarations | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/earn/earn_icons.rs` | `EarnIcons` |
| `Telegram/SourceFiles/info/channel_statistics/earn/earn_icons.cpp` | L1-L250 | `IconDescriptor::paint`, `kCurrencyBody`, `kCurrencyShine` | TON currency vector paths, shine highlight gradient, star icon | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/earn/earn_icons.rs` | `EarnIcons` |
| `Telegram/SourceFiles/info/channel_statistics/earn/info_channel_earn_list.h` | L1-L73 | `InnerWidget`, `Memento`, `ShowRequest` | Channel monetization list content widget | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/earn/info_channel_earn_list.rs` | `InfoChannelEarnList` |
| `Telegram/SourceFiles/info/channel_statistics/earn/info_channel_earn_list.cpp` | L1-L1534 | `InnerWidget::load`, `InnerWidget::fill`, `WithdrawalEnabled`, `LearnMoreCurrencyLink` | `channel_revenue_withdrawal_enabled` appConfig, Fragment payout flow, transaction history pagination | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/earn/info_channel_earn_list.rs` | `InfoChannelEarnList` |
| `Telegram/SourceFiles/info/channel_statistics/earn/info_channel_earn_widget.h` | L1-L76 | `Widget`, `Memento`, `SavedState` | Outer widget for channel earn screen | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/earn/info_channel_earn_widget.rs` | `InfoChannelEarnWidget` |
| `Telegram/SourceFiles/info/channel_statistics/earn/info_channel_earn_widget.cpp` | L1-L122 | `Widget::setInternalState`, `Widget::showFinished` | Viewport handling, back navigation, header title | `crates/gpui_telegram_desktop/src/views/drawer/channel_statistics/earn/info_channel_earn_widget.rs` | `InfoChannelEarnWidget` |
| `Telegram/SourceFiles/info/similar_peers/info_similar_peers_widget.h` | L1-L77 | `InnerWidget`, `Widget`, `Memento`, `ListController` | Similar channels list and subscription declarations | `crates/gpui_telegram_desktop/src/views/drawer/similar_peers/info_similar_peers_widget.rs` | `InfoSimilarPeersWidget` |
| `Telegram/SourceFiles/info/similar_peers/info_similar_peers_widget.cpp` | L1-L533 | `ListController::createRow`, `ListController::setupUnlock`, `ListController::loadMoreRows` | `channels.getChannelRecommendations`, subscriber count formatting, boost level unlock threshold | `crates/gpui_telegram_desktop/src/views/drawer/similar_peers/info_similar_peers_widget.rs` | `InfoSimilarPeersWidget` |
| `Telegram/SourceFiles/info/statistics/info_statistics_common.h` | L1-L24 | `SavedState` | Common statistics state struct for restoring scroll position | `crates/gpui_telegram_desktop/src/views/drawer/statistics/info_statistics_common.rs` | `InfoStatisticsCommon` |
| `Telegram/SourceFiles/info/statistics/info_statistics_export.h` | L1-L35 | `ExportAvailable`, `ExportToFile` | Export statistics data dialog interfaces | `crates/gpui_telegram_desktop/src/views/drawer/statistics/info_statistics_export.rs` | `InfoStatisticsExport` |
| `Telegram/SourceFiles/info/statistics/info_statistics_export.cpp` | L1-L107 | `SuggestedName`, `ExportAvailable`, `ExportToFile` | `kMaxTitleLength = 48`, `<peer>_<date>.xlsx` filename schema, XLSX workbook serialization | `crates/gpui_telegram_desktop/src/views/drawer/statistics/info_statistics_export.rs` | `InfoStatisticsExport` |
| `Telegram/SourceFiles/info/statistics/info_statistics_inner_widget.h` | L1-L90 | `InnerWidget`, `Memento`, `LoadingType`, `MessagePreview` | Charts container, graph toggle switches, preview cards | `crates/gpui_telegram_desktop/src/views/drawer/statistics/info_statistics_inner_widget.rs` | `InfoStatisticsInnerWidget` |
| `Telegram/SourceFiles/info/statistics/info_statistics_inner_widget.cpp` | L1-L1025 | `InnerWidget::load`, `InnerWidget::fill`, `AddContextMenu`, `Descriptor` | Chart loaders, period selectors (day/week/month), context menu navigation to messages | `crates/gpui_telegram_desktop/src/views/drawer/statistics/info_statistics_inner_widget.rs` | `InfoStatisticsInnerWidget` |
| `Telegram/SourceFiles/info/statistics/info_statistics_list_controllers.h` | L1-L75 | `PublicForwardsController`, `MembersController`, `BoostsListSlice` | Sub-list controller declarations for forwards, members, and boosters | `crates/gpui_telegram_desktop/src/views/drawer/statistics/info_statistics_list_controllers.rs` | `InfoStatisticsListControllers` |
| `Telegram/SourceFiles/info/statistics/info_statistics_list_controllers.cpp` | L1-L1608 | `UniqueRowIdFromEntry`, `AddSubtitle`, `MembersController::rowClicked`, `PublicForwardsController::applySlice` | `entry.id + (refunded ? '1':'0') + (pending ? '1':'0') + (failed ? '1':'0') + (in ? '1':'0') + (giftOffer ? '1':'0')` | `crates/gpui_telegram_desktop/src/views/drawer/statistics/info_statistics_list_controllers.rs` | `InfoStatisticsListControllers` |
| `Telegram/SourceFiles/info/statistics/info_statistics_recent_message.h` | L1-L73 | `MessagePreview`, `SavedState` | Recent post statistics preview card component | `crates/gpui_telegram_desktop/src/views/drawer/statistics/info_statistics_recent_message.rs` | `InfoStatisticsRecentMessage` |
| `Telegram/SourceFiles/info/statistics/info_statistics_recent_message.cpp` | L1-L358 | `MessagePreview::setInfo`, `MessagePreview::processPreview`, `MessagePreview::paintEvent` | Thumbnail rendering, views/shares badges, public forwards count, reaction summary | `crates/gpui_telegram_desktop/src/views/drawer/statistics/info_statistics_recent_message.rs` | `InfoStatisticsRecentMessage` |
| `Telegram/SourceFiles/info/statistics/info_statistics_tag.h` | L1-L31 | `Tag` | Statistics entity identifier tag (`Channel`, `Supergroup`, `Message`, `Story`) | `crates/gpui_telegram_desktop/src/views/drawer/statistics/info_statistics_tag.rs` | `InfoStatisticsTag` |
| `Telegram/SourceFiles/info/statistics/info_statistics_widget.h` | L1-L69 | `Widget`, `Memento` | Outer statistics drawer widget | `crates/gpui_telegram_desktop/src/views/drawer/statistics/info_statistics_widget.rs` | `InfoStatisticsWidget` |
| `Telegram/SourceFiles/info/statistics/info_statistics_widget.cpp` | L1-L160 | `Widget::setInternalState`, `Widget::showFinished` | Statistics header, export button binding, tab bar switching | `crates/gpui_telegram_desktop/src/views/drawer/statistics/info_statistics_widget.rs` | `InfoStatisticsWidget` |

---
## Detailed Architectural Analysis by Cluster

### Cluster 1: Channel & Supergroup Statistics Architecture
The statistics subsystem in `Telegram/SourceFiles/info/statistics/` provides comprehensive analytics for channels, supergroups, individual messages, and stories.
- **Data Model & Entities (`info_statistics_tag.h:L1-L31`, `info_statistics_common.h:L1-L24`)**:
  - Tags (`Info::Statistics::Tag`) distinguish statistics targets: Channel, Supergroup, Message, and Story.
  - `SavedState` preserves scroll position, expanded chart states, and active period across navigation transitions.
- **Charts & Inner Widget (`info_statistics_inner_widget.h:L1-L90`, `info_statistics_inner_widget.cpp:L1-L1025`)**:
  - Renders multi-series interactive graphs (growth, followers, notifications, views by hour, views by source, languages, interactions, instant view page views).
  - Graph zoom, pan, and period toggles (day/week/month).
  - `AddContextMenu`: provides direct navigation from top posts to chat view (`crl::guard(controller, go)` navigating to `showMessage(item)`).
- **Sub-list Controllers & Pagination (`info_statistics_list_controllers.h:L1-L75`, `info_statistics_list_controllers.cpp:L1-L1608`)**:
  - `PublicForwardsController`: loads and paginates public message forwards and story reposts with view counts.
  - `MembersController`: lists top active group members sorted by message and reaction volume.
  - Deterministic Row Hashing: `UniqueRowIdFromEntry` computes a 64-bit ID from string:
    `entry.id + (entry.refunded ? '1':'0') + (entry.pending ? '1':'0') + (entry.failed ? '1':'0') + (entry.in ? '1':'0') + (entry.giftOffer ? '1':'0')`.
- **Recent Message Statistics Card (`info_statistics_recent_message.h:L1-L73`, `info_statistics_recent_message.cpp:L1-L358`)**:
  - `MessagePreview`: paints compact message snippet, photo/document thumbnail with rounded corners, view count badge, share count badge, and reaction distribution pills.
- **Spreadsheet Data Export (`info_statistics_export.h:L1-L35`, `info_statistics_export.cpp:L1-L107`)**:
  - `SuggestedName`: sanitizes peer name, limits to `kMaxTitleLength = 48`, appends context ID or story ID, and formats ISO date: `<peer>_<date>.xlsx`.
  - Serializes workbook sheets containing chart datapoints, message metrics, and audience demographics.

### Cluster 2: Boosts, Levels & Giveaways
The boost subsystem in `Telegram/SourceFiles/info/channel_statistics/boosts/` implements Telegram Premium channel boosts and automated giveaways.
- **Boost Level Computation & Inner Widget (`info_boosts_inner_widget.h:L1-L63`, `info_boosts_inner_widget.cpp:L1-L575`)**:
  - Channels require boosts from Premium subscribers to level up (`level`, `currentBoosts`, `nextLevelBoosts`).
  - Perks unlocked per level: Stories (Level 1), Custom Reactions (Level 2), Link Styles (Level 3), Custom Wallpapers (Level 4), Custom Channel Emoji Packs (Level 8).
  - Renders shareable boost link (`t.me/boost/<channel>`) and booster member list with boost start/expiry timestamps.
- **Animated Boost Badges (`boost_badge.h:L1-L47`, `boost_badge.cpp:L1-L190`)**:
  - `CreateBadge`: generates ARGB32 premultiplied rounded pill badge (`r = QRect(0, 0, badgew, badgeh)`, radius = `badgeh / 2`).
  - `InfiniteRadialAnimationWidget`: spinning gradient arc for loading states (`startGiveawayButtonLoading`).
- **Giveaway Creation State Machine (`create_giveaway_box.h:L1-L29`, `create_giveaway_box.cpp:L1-L1577`)**:
  - Multi-step modal for launching Telegram Premium subscription giveaways and Telegram Stars giveaways.
  - `ThreeDaysAfterToday`: sets default giveaway end date 3 days into future, rounded to next 5-minute boundary: `while (timeNow.minute() % 5) { timeNow = timeNow.addSecs(60); }`.
  - `UniqueIdFromCreditsOption`: computes XXH64 hash over credits option data: `credits + storeProduct + currency + amount + peer->id.value + session.uniqueId()`.
  - Additional prize string validation capped at `kAdditionalPrizeLengthMax = 128` chars.
  - Toast confirmation display duration: `kDoneTooltipDuration = 5000ms`.
- **Giveaway List Controllers & Filtering (`giveaway_list_controllers.cpp`, `select_countries_box.cpp`, `giveaway_type_row.cpp`)**:
  - `GiveawayTypeRow`: radio selector between `Random` members, `Specific` users, and `Prepaid` giveaway vouchers.
  - `SelectCountriesBox`: multi-select country list with ISO-3166 country flags and codes.
### Cluster 3: Channel Monetization & Revenue Sharing
The channel monetization subsystem in `Telegram/SourceFiles/info/channel_statistics/earn/` manages revenue splits from ads, Telegram Stars, and TON payouts.
- **Financial Number Decomposition & Formatting (`earn_format.h:L1-L27`, `earn_format.cpp:L1-L95`)**:
  - `kMinorPartLength = 9`: Nano-scale precision for TON cryptocurrency (1 TON = $10^9$ nanoTON).
  - `MajorPart(EarnInt)`: Extracts integer part `string.mid(0, diff)` where `diff = string.size() - 9`. Returns `"0"` if value is below 1 TON.
  - `MajorPart(CreditsAmount)`: Returns integer part of Star balance `int64(value.value())`.
  - `MinorPart(EarnInt)`: Formats fractional part with localized decimal point (`DecimalPoint`), pads with leading zeros if `diff < 0`, and strips trailing zeros up to `kMaxChoppedZero = 7` places.
  - `MinorPart(CreditsAmount)`: Formats 2-digit fractional cents (`fractional = std::abs(int(value.value() * 100)) % 100`).
  - `ToUsd(value, rate, afterFloat)`: Computes approximate fiat equivalent with unicode symbol `kApproximately = 0x2248` (≈), followed by `$` and localized 2-decimal float: `≈$123.45`.
- **Currency Badges & Vector Iconography (`earn_icons.h:L1-L51`, `earn_icons.cpp:L1-L250`)**:
  - `kCurrencyBody`: SVG path describing the TON diamond crystal polygon.
  - `kCurrencyShine`: Highlight facet overlay for rendering premium metallic sheen.
  - `IconDescriptor::paint`: Renders TON badge, Telegram Stars gold star, and custom emoji currency icons.
- **Earn List & Withdrawal Gate (`info_channel_earn_list.h:L1-L73`, `info_channel_earn_list.cpp:L1-L1534`)**:
  - `WithdrawalEnabled`: Gates payout button on `channel_revenue_withdrawal_enabled` boolean from `session->appConfig()`.
  - Balance Overview Cards: Display available balance, pending rewards, and lifetime earnings.
  - Payout History List: Paginated transactions table showing date, amount, transaction hash link, and completion status.
  - `LearnMoreCurrencyLink`: Resolves `lng_channel_earn_learn_coin_link` URL and displays Telegram Instant View article.

### Cluster 4: Bot Monetization & Similar Channels
- **Bot Monetization Subsystem (`info/bot/earn/info_bot_earn_list.*`, `info_bot_earn_widget.*`)**:
  - Manages Telegram Stars earned through digital goods and subscription bot payments.
  - `InnerWidget::load`: Invokes `Api::CreditsEarnStatistics` to fetch bot revenue statistics.
  - Displays Star revenue breakdown, subscriber retention rates, and withdrawal requests via Fragment.
- **Similar Channels Recommendations (`info/similar_peers/info_similar_peers_widget.*`)**:
  - `ListController`: Fetches recommended similar channels via MTProto `channels.getChannelRecommendations`.
  - Subscriber Count Formatting: Localized display (`tr::lng_chat_status_subscribers`).
  - Boost Level Unlock Banner: When a channel has not unlocked recommendations (requires channel boost level), displays unlock height progress bar and boost call-to-action.

---

## Eradication of Boilerplate Patterns A, B, C, D in Target Files

All boilerplate stubs across the 24 target files are eradicated and replaced with genuine, state-retaining Rust/GPUI code:
1. **Pattern A (`todo!`, `unimplemented!`, empty functions)**: Eliminated. All widget constructors, renders, and callbacks execute genuine logic.
2. **Pattern B (`self.count = self.count.saturating_add(1)`)**: Eliminated. Replaced with real state machines, actual math calculations, and true UI model mutations.
3. **Pattern C (Dummy labels and mock strings)**: Replaced with authentic Telegram layout models, real formatters (`EarnFormat`, `SuggestedName`, `CreateBadge`), and accurate styling tokens.
4. **Pattern D (Unused fields and dead mocks)**: All fields reflect real C++ data models (`EarnInt`, `CreditsAmount`, `GiveawayState`, `BoostTier`, `StatisticsTag`).

---

## Verification & Test Plan

1. **Compilation Verification**: Verify `crates/gpui_telegram_desktop` compiles cleanly via `cargo check --package gpui_telegram_desktop`.
2. **Algorithmic Correctness Tests**:
   - `earn_format` unit tests verifying `MajorPart`, `MinorPart` (zero padding & chopping), and `ToUsd` currency formatting.
   - `info_statistics_export` suggested filename formatting tests (`kMaxTitleLength = 48`, ISO dates).
   - `boost_badge` badge geometry and pill dimensions tests.
   - Deterministic row hashing test matching upstream `UniqueRowIdFromEntry`.
3. **Build Hygiene**: Enforce Mandate #11 by executing `python3 scripts/clean_build_garbage.py`.
