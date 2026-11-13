# Subsystem Shard: `dialogs`

**Total Source Files**: 83  
**Total Source Lines**: 36,218  
**Primary Feature IDs**: TCG-N03, TCG-N11, TCG-N13, TCG-N15  
**JSON Shard Path**: `docs/specs/ledger/shards/dialogs.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/dialogs/dialogs.style` | `TCG-N03` | `DialogsStyleSheet` | `crates/gpui_telegram_desktop/src/theme/dialogs/dialogs_style.rs` | 970 | `tests/test_tcg_n03_dialogs.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_common.h` | `TCG-N03` | `DialogsCommon` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_common.rs` | 170 | `tests/test_tcg_n03_dialogs_common.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_community_chats_list.cpp` | `TCG-N11` | `DialogsCommunityChatsList` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_community_chats_list.rs` | 263 | `tests/test_tcg_n11_dialogs_community_chats_list.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_community_chats_list.h` | `TCG-N11` | `DialogsCommunityChatsList` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_community_chats_list.rs` | 77 | `tests/test_tcg_n11_dialogs_community_chats_list.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_community_requestable_list.cpp` | `TCG-N03` | `DialogsCommunityRequestableList` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_community_requestable_list.rs` | 158 | `tests/test_tcg_n03_dialogs_community_requestable_list.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_community_requestable_list.h` | `TCG-N03` | `DialogsCommunityRequestableList` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_community_requestable_list.rs` | 44 | `tests/test_tcg_n03_dialogs_community_requestable_list.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_community_rows_view.cpp` | `TCG-N03` | `DialogsCommunityRowsView` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_community_rows_view.rs` | 111 | `tests/test_tcg_n03_dialogs_community_rows_view.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_community_rows_view.h` | `TCG-N03` | `DialogsCommunityRowsView` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_community_rows_view.rs` | 55 | `tests/test_tcg_n03_dialogs_community_rows_view.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_entry.cpp` | `TCG-N03` | `DialogsEntry` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_entry.rs` | 531 | `tests/test_tcg_n03_dialogs_entry.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_entry.h` | `TCG-N03` | `DialogsEntry` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_entry.rs` | 239 | `tests/test_tcg_n03_dialogs_entry.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_indexed_list.cpp` | `TCG-N03` | `DialogsIndexedList` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_indexed_list.rs` | 284 | `tests/test_tcg_n03_dialogs_indexed_list.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_indexed_list.h` | `TCG-N03` | `DialogsIndexedList` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_indexed_list.rs` | 103 | `tests/test_tcg_n03_dialogs_indexed_list.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_inner_widget.cpp` | `TCG-N03` | `DialogsInnerWidget` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_inner_widget.rs` | 7344 | `tests/test_tcg_n03_dialogs_inner_widget.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_inner_widget.h` | `TCG-N03` | `DialogsInnerWidget` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_inner_widget.rs` | 835 | `tests/test_tcg_n03_dialogs_inner_widget.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_inner_widget_accessibility.cpp` | `TCG-N03` | `DialogsInnerWidgetAccessibility` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_inner_widget_accessibility.rs` | 666 | `tests/test_tcg_n03_dialogs_inner_widget_accessibility.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_inner_widget_accessibility.h` | `TCG-N03` | `DialogsInnerWidgetAccessibility` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_inner_widget_accessibility.rs` | 70 | `tests/test_tcg_n03_dialogs_inner_widget_accessibility.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_key.cpp` | `TCG-N03` | `DialogsKey` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_key.rs` | 115 | `tests/test_tcg_n03_dialogs_key.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_key.h` | `TCG-N03` | `DialogsKey` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_key.rs` | 169 | `tests/test_tcg_n03_dialogs_key.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_list.cpp` | `TCG-N03` | `DialogsList` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_list.rs` | 269 | `tests/test_tcg_n03_dialogs_list.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_list.h` | `TCG-N03` | `DialogsList` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_list.rs` | 105 | `tests/test_tcg_n03_dialogs_list.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_main_list.cpp` | `TCG-N03` | `DialogsMainList` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_main_list.rs` | 241 | `tests/test_tcg_n03_dialogs_main_list.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_main_list.h` | `TCG-N03` | `DialogsMainList` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_main_list.rs` | 88 | `tests/test_tcg_n03_dialogs_main_list.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_pinned_list.cpp` | `TCG-N17` | `DialogsPinnedList` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_pinned_list.rs` | 178 | `tests/test_tcg_n17_dialogs_pinned_list.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_pinned_list.h` | `TCG-N17` | `DialogsPinnedList` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_pinned_list.rs` | 63 | `tests/test_tcg_n17_dialogs_pinned_list.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_quick_action.cpp` | `TCG-N03` | `DialogsQuickAction` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_quick_action.rs` | 319 | `tests/test_tcg_n03_dialogs_quick_action.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_quick_action.h` | `TCG-N03` | `DialogsQuickAction` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_quick_action.rs` | 57 | `tests/test_tcg_n03_dialogs_quick_action.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_row.cpp` | `TCG-N03` | `DialogsRow` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_row.rs` | 1001 | `tests/test_tcg_n03_dialogs_row.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_row.h` | `TCG-N03` | `DialogsRow` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_row.rs` | 275 | `tests/test_tcg_n03_dialogs_row.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_search_from_controllers.cpp` | `TCG-N21` | `DialogsSearchFromControllers` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_search_from_controllers.rs` | 83 | `tests/test_tcg_n21_dialogs_search_from_controllers.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_search_from_controllers.h` | `TCG-N21` | `DialogsSearchFromControllers` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_search_from_controllers.rs` | 34 | `tests/test_tcg_n21_dialogs_search_from_controllers.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_search_posts.cpp` | `TCG-N21` | `DialogsSearchPosts` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_search_posts.rs` | 347 | `tests/test_tcg_n21_dialogs_search_posts.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_search_posts.h` | `TCG-N21` | `DialogsSearchPosts` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_search_posts.rs` | 80 | `tests/test_tcg_n21_dialogs_search_posts.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_search_tags.cpp` | `TCG-N21` | `DialogsSearchTags` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_search_tags.rs` | 451 | `tests/test_tcg_n21_dialogs_search_tags.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_search_tags.h` | `TCG-N21` | `DialogsSearchTags` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_search_tags.rs` | 92 | `tests/test_tcg_n21_dialogs_search_tags.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_three_state_icon.h` | `TCG-N03` | `DialogsThreeStateIcon` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_three_state_icon.rs` | 21 | `tests/test_tcg_n03_dialogs_three_state_icon.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_top_bar_suggestion.cpp` | `TCG-N03` | `DialogsTopBarSuggestion` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_top_bar_suggestion.rs` | 268 | `tests/test_tcg_n03_dialogs_top_bar_suggestion.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_top_bar_suggestion.h` | `TCG-N03` | `DialogsTopBarSuggestion` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_top_bar_suggestion.rs` | 37 | `tests/test_tcg_n03_dialogs_top_bar_suggestion.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_top_bar_suggestion_test.cpp` | `TCG-N03` | `DialogsTopBarSuggestionTest` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_top_bar_suggestion_test.rs` | 138 | `tests/test_tcg_n03_dialogs_top_bar_suggestion_test.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_widget.cpp` | `TCG-N03` | `DialogsWidget` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_widget.rs` | 5198 | `tests/test_tcg_n03_dialogs_widget.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_widget.h` | `TCG-N03` | `DialogsWidget` | `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_widget.rs` | 468 | `tests/test_tcg_n03_dialogs_widget.rs` |
| `Telegram/SourceFiles/dialogs/dialogs_widget.style` | `TCG-N03` | `DialogsWidgetStyleSheet` | `crates/gpui_telegram_desktop/src/theme/dialogs/dialogs_widget_style.rs` | 66 | `tests/test_tcg_n03_dialogs_widget.rs` |
| `Telegram/SourceFiles/dialogs/suggestions/suggestion.cpp` | `TCG-N03` | `SuggestionSuggestions` | `crates/gpui_telegram_desktop/src/views/sidebar/suggestions/suggestion.rs` | 26 | `tests/test_tcg_n03_suggestion.rs` |
| `Telegram/SourceFiles/dialogs/suggestions/suggestion.h` | `TCG-N03` | `SuggestionSuggestions` | `crates/gpui_telegram_desktop/src/views/sidebar/suggestions/suggestion.rs` | 75 | `tests/test_tcg_n03_suggestion.rs` |
| `Telegram/SourceFiles/dialogs/suggestions/suggestion_birthday_contacts.cpp` | `TCG-N03` | `SuggestionBirthdayContactsSuggestions` | `crates/gpui_telegram_desktop/src/views/sidebar/suggestions/suggestion_birthday_contacts.rs` | 151 | `tests/test_tcg_n03_suggestion_birthday_contacts.rs` |
| `Telegram/SourceFiles/dialogs/suggestions/suggestion_birthday_setup.cpp` | `TCG-N03` | `SuggestionBirthdaySetupSuggestions` | `crates/gpui_telegram_desktop/src/views/sidebar/suggestions/suggestion_birthday_setup.rs` | 86 | `tests/test_tcg_n03_suggestion_birthday_setup.rs` |
| `Telegram/SourceFiles/dialogs/suggestions/suggestion_custom_promo.cpp` | `TCG-N03` | `SuggestionCustomPromoSuggestions` | `crates/gpui_telegram_desktop/src/views/sidebar/suggestions/suggestion_custom_promo.rs` | 66 | `tests/test_tcg_n03_suggestion_custom_promo.rs` |
| `Telegram/SourceFiles/dialogs/suggestions/suggestion_gift_auctions.cpp` | `TCG-N15` | `SuggestionGiftAuctionsSuggestions` | `crates/gpui_telegram_desktop/src/views/sidebar/suggestions/suggestion_gift_auctions.rs` | 75 | `tests/test_tcg_n15_suggestion_gift_auctions.rs` |
| `Telegram/SourceFiles/dialogs/suggestions/suggestion_low_credits_subs.cpp` | `TCG-N03` | `SuggestionLowCreditsSubsSuggestions` | `crates/gpui_telegram_desktop/src/views/sidebar/suggestions/suggestion_low_credits_subs.rs` | 130 | `tests/test_tcg_n03_suggestion_low_credits_subs.rs` |
| `Telegram/SourceFiles/dialogs/suggestions/suggestion_premium_grace.cpp` | `TCG-N03` | `SuggestionPremiumGraceSuggestions` | `crates/gpui_telegram_desktop/src/views/sidebar/suggestions/suggestion_premium_grace.rs` | 70 | `tests/test_tcg_n03_suggestion_premium_grace.rs` |
| `Telegram/SourceFiles/dialogs/suggestions/suggestion_premium_offer.cpp` | `TCG-N03` | `SuggestionPremiumOfferSuggestions` | `crates/gpui_telegram_desktop/src/views/sidebar/suggestions/suggestion_premium_offer.rs` | 113 | `tests/test_tcg_n03_suggestion_premium_offer.rs` |
| `Telegram/SourceFiles/dialogs/suggestions/suggestion_unreviewed_auth.cpp` | `TCG-N03` | `SuggestionUnreviewedAuthSuggestions` | `crates/gpui_telegram_desktop/src/views/sidebar/suggestions/suggestion_unreviewed_auth.rs` | 144 | `tests/test_tcg_n03_suggestion_unreviewed_auth.rs` |
| `Telegram/SourceFiles/dialogs/suggestions/suggestion_userpic_setup.cpp` | `TCG-N03` | `SuggestionUserpicSetupSuggestions` | `crates/gpui_telegram_desktop/src/views/sidebar/suggestions/suggestion_userpic_setup.rs` | 102 | `tests/test_tcg_n03_suggestion_userpic_setup.rs` |
| `Telegram/SourceFiles/dialogs/ui/chat_search_empty.cpp` | `TCG-N21` | `ChatSearchEmptyUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/chat_search_empty.rs` | 83 | `tests/test_tcg_n21_chat_search_empty.rs` |
| `Telegram/SourceFiles/dialogs/ui/chat_search_empty.h` | `TCG-N21` | `ChatSearchEmptyUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/chat_search_empty.rs` | 44 | `tests/test_tcg_n21_chat_search_empty.rs` |
| `Telegram/SourceFiles/dialogs/ui/chat_search_in.cpp` | `TCG-N21` | `ChatSearchInUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/chat_search_in.rs` | 477 | `tests/test_tcg_n21_chat_search_in.rs` |
| `Telegram/SourceFiles/dialogs/ui/chat_search_in.h` | `TCG-N21` | `ChatSearchInUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/chat_search_in.rs` | 109 | `tests/test_tcg_n21_chat_search_in.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_layout.cpp` | `TCG-N03` | `DialogsLayoutUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_layout.rs` | 1458 | `tests/test_tcg_n03_dialogs_layout.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_layout.h` | `TCG-N03` | `DialogsLayoutUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_layout.rs` | 121 | `tests/test_tcg_n03_dialogs_layout.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_layout.style` | `TCG-N03` | `DialogsLayoutUiStyleSheet` | `crates/gpui_telegram_desktop/src/theme/dialogs/dialogs_layout_style.rs` | 112 | `tests/test_tcg_n03_dialogs_layout.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_message_view.cpp` | `TCG-N03` | `DialogsMessageViewUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_message_view.rs` | 640 | `tests/test_tcg_n03_dialogs_message_view.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_message_view.h` | `TCG-N03` | `DialogsMessageViewUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_message_view.rs` | 118 | `tests/test_tcg_n03_dialogs_message_view.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_pill.cpp` | `TCG-N03` | `DialogsPillUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_pill.rs` | 80 | `tests/test_tcg_n03_dialogs_pill.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_pill.h` | `TCG-N03` | `DialogsPillUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_pill.rs` | 30 | `tests/test_tcg_n03_dialogs_pill.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_quick_action.h` | `TCG-N03` | `DialogsQuickActionUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_quick_action.rs` | 23 | `tests/test_tcg_n03_dialogs_quick_action.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_quick_action_context.h` | `TCG-N03` | `DialogsQuickActionContextUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_quick_action_context.rs` | 48 | `tests/test_tcg_n03_dialogs_quick_action_context.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_stories_content.cpp` | `TCG-N38` | `DialogsStoriesContentUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_stories_content.rs` | 281 | `tests/test_tcg_n38_dialogs_stories_content.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_stories_content.h` | `TCG-N38` | `DialogsStoriesContentUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_stories_content.rs` | 38 | `tests/test_tcg_n38_dialogs_stories_content.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_stories_list.cpp` | `TCG-N38` | `DialogsStoriesListUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_stories_list.rs` | 1346 | `tests/test_tcg_n38_dialogs_stories_list.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_stories_list.h` | `TCG-N38` | `DialogsStoriesListUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_stories_list.rs` | 235 | `tests/test_tcg_n38_dialogs_stories_list.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_suggestions.cpp` | `TCG-N03` | `DialogsSuggestionsUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_suggestions.rs` | 3885 | `tests/test_tcg_n03_dialogs_suggestions.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_suggestions.h` | `TCG-N03` | `DialogsSuggestionsUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_suggestions.rs` | 395 | `tests/test_tcg_n03_dialogs_suggestions.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_top_bar_suggestion_content.cpp` | `TCG-N03` | `DialogsTopBarSuggestionContentUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_top_bar_suggestion_content.rs` | 844 | `tests/test_tcg_n03_dialogs_top_bar_suggestion_content.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_top_bar_suggestion_content.h` | `TCG-N03` | `DialogsTopBarSuggestionContentUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_top_bar_suggestion_content.rs` | 176 | `tests/test_tcg_n03_dialogs_top_bar_suggestion_content.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_topics_view.cpp` | `TCG-N13` | `DialogsTopicsViewUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_topics_view.rs` | 442 | `tests/test_tcg_n13_dialogs_topics_view.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_topics_view.h` | `TCG-N13` | `DialogsTopicsViewUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_topics_view.rs` | 130 | `tests/test_tcg_n13_dialogs_topics_view.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_video_userpic.cpp` | `TCG-N03` | `DialogsVideoUserpicUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_video_userpic.rs` | 175 | `tests/test_tcg_n03_dialogs_video_userpic.rs` |
| `Telegram/SourceFiles/dialogs/ui/dialogs_video_userpic.h` | `TCG-N03` | `DialogsVideoUserpicUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/dialogs_video_userpic.rs` | 82 | `tests/test_tcg_n03_dialogs_video_userpic.rs` |
| `Telegram/SourceFiles/dialogs/ui/posts_search_intro.cpp` | `TCG-N21` | `PostsSearchIntroUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/posts_search_intro.rs` | 252 | `tests/test_tcg_n21_posts_search_intro.rs` |
| `Telegram/SourceFiles/dialogs/ui/posts_search_intro.h` | `TCG-N21` | `PostsSearchIntroUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/posts_search_intro.rs` | 59 | `tests/test_tcg_n21_posts_search_intro.rs` |
| `Telegram/SourceFiles/dialogs/ui/restore_windows_offer.cpp` | `TCG-N03` | `RestoreWindowsOfferUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/restore_windows_offer.rs` | 209 | `tests/test_tcg_n03_restore_windows_offer.rs` |
| `Telegram/SourceFiles/dialogs/ui/restore_windows_offer.h` | `TCG-N03` | `RestoreWindowsOfferUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/restore_windows_offer.rs` | 60 | `tests/test_tcg_n03_restore_windows_offer.rs` |
| `Telegram/SourceFiles/dialogs/ui/top_peers_strip.cpp` | `TCG-N11` | `TopPeersStripUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/top_peers_strip.rs` | 994 | `tests/test_tcg_n11_top_peers_strip.rs` |
| `Telegram/SourceFiles/dialogs/ui/top_peers_strip.h` | `TCG-N11` | `TopPeersStripUi` | `crates/gpui_telegram_desktop/src/views/sidebar/ui/top_peers_strip.rs` | 151 | `tests/test_tcg_n11_top_peers_strip.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/dialogs/dialogs.style`
- **Core Methods & Signals**: font, RippleAnimation, point, margins, PeerListItem, PeerList
- **Style Tokens**: height: pixels, padding: margins, photoSize: pixels, nameLeft: pixels, nameTop: pixels, textLeft: pixels
- **Target**: `DialogsStyleSheet` in `crates/gpui_telegram_desktop/src/theme/dialogs/dialogs_style.rs`

### `Telegram/SourceFiles/dialogs/dialogs_common.h`
- **Classes & Structs**: DialogRightButton, RippleAnimation, Row, SortMode, PositionChange, UnreadState, BadgesState, CountInBadge, IncludeInBadge, RowsByLetter, RightButton
- **Core Methods & Signals**: bool
- **Target**: `DialogsCommon` in `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_common.rs`

### `Telegram/SourceFiles/dialogs/dialogs_community_chats_list.cpp`
- **Classes & Structs**: Event, Flag
- **Core Methods & Signals**: Row::ComputeSt, CommunityChatsList::CommunityChatsList, CommunityChatsList::~CommunityChatsList, CommunityChatsList::rebuild, Data::CommunityChatJoined, ranges::stable_sort
- **Target**: `DialogsCommunityChatsList` in `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_community_chats_list.rs`

### `Telegram/SourceFiles/dialogs/dialogs_community_chats_list.h`
- **Classes & Structs**: History, PeerData, DialogRow, CommunityInfo, SessionController, Row, CommunityChatsKind, CommunityChatsList
- **Core Methods & Signals**: CommunityChatsList, resizeGetHeight, paintEvent, mouseMoveEvent, mousePressEvent, mouseReleaseEvent
- **Target**: `DialogsCommunityChatsList` in `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_community_chats_list.rs`

### `Telegram/SourceFiles/dialogs/dialogs_community_requestable_list.cpp`
- **Classes & Structs**: ChatsController
- **Core Methods & Signals**: ChatsController::ChatsController, ChatsController::session, ChatsController::prepare, ChatsController::rowClicked, CommunityRequestableList::CommunityRequestableList, Data::CommunityChatJoined
- **Target**: `DialogsCommunityRequestableList` in `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_community_requestable_list.rs`

### `Telegram/SourceFiles/dialogs/dialogs_community_requestable_list.h`
- **Classes & Structs**: PeerListContent, CommunityInfo, SessionController, CommunityRequestableList
- **Core Methods & Signals**: CommunityRequestableList, resizeGetHeight, rpl::countValue
- **Target**: `DialogsCommunityRequestableList` in `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_community_requestable_list.rs`

### `Telegram/SourceFiles/dialogs/dialogs_community_rows_view.h`
- **Classes & Structs**: History, Painter, Row, CommunityRowsView
- **Core Methods & Signals**: CommunityRowsView, clear, add, finalize, recountHeights
- **Target**: `DialogsCommunityRowsView` in `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_community_rows_view.rs`

### `Telegram/SourceFiles/dialogs/dialogs_entry.h`
- **Classes & Structs**: HistoryItem, History, UserData, Session, Forum, Folder, ForumTopic, SavedSublist, SavedMessages, Thread, PeerUserpicView, PaintContext
- **Core Methods & Signals**: Entry::unreadStateChangeNotifier, gsl::finally, ~Entry, adjustByPosInChatList, addToChatList, setColorIndexForFilterId
- **Target**: `DialogsEntry` in `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_entry.rs`

### `Telegram/SourceFiles/dialogs/dialogs_indexed_list.h`
- **Classes & Structs**: History, RowsByLetter, Row, IndexedList, iterator, const_iterator
- **Core Methods & Signals**: IndexedList, addToEnd, adjustByDate, moveToTop, freeze, unfreeze
- **Target**: `DialogsIndexedList` in `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_indexed_list.rs`

### `Telegram/SourceFiles/dialogs/dialogs_inner_widget.cpp`
- **Classes & Structs**: InnerWidget, Snapshot, Kind, Event, UpdateFlag, Result, Command
- **Core Methods & Signals**: tr::lng_search_tab_by_hashtag, tr::lng_dlg_search_for_messages, tr::lng_search_tab_no_results, tr::lng_search_tab_no_results_text, tr::link, tr::lng_search_tab_no_results_retry
- **Target**: `DialogsInnerWidget` in `crates/gpui_telegram_desktop/src/views/sidebar/dialogs_inner_widget.rs`

