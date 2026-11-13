# Subsystem Shard: `api`

**Total Source Files**: 116  
**Total Source Lines**: 28,854  
**Primary Feature IDs**: TCG-N01, TCG-N02, TCG-N11, TCG-N12  
**JSON Shard Path**: `docs/specs/ledger/shards/api.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/api/api_attached_stickers.cpp` | `TCG-N15` | `ApiAttachedStickers` | `crates/gpui_telegram_proto/src/api/api_attached_stickers.rs` | 92 | `tests/test_tcg_n15_api_attached_stickers.rs` |
| `Telegram/SourceFiles/api/api_attached_stickers.h` | `TCG-N15` | `ApiAttachedStickers` | `crates/gpui_telegram_proto/src/api/api_attached_stickers.rs` | 44 | `tests/test_tcg_n15_api_attached_stickers.rs` |
| `Telegram/SourceFiles/api/api_authorizations.cpp` | `TCG-N01` | `ApiAuthorizations` | `crates/gpui_telegram_proto/src/api/api_authorizations.rs` | 405 | `tests/test_tcg_n01_api_authorizations.rs` |
| `Telegram/SourceFiles/api/api_authorizations.h` | `TCG-N01` | `ApiAuthorizations` | `crates/gpui_telegram_proto/src/api/api_authorizations.rs` | 93 | `tests/test_tcg_n01_api_authorizations.rs` |
| `Telegram/SourceFiles/api/api_blocked_peers.cpp` | `TCG-N11` | `ApiBlockedPeers` | `crates/gpui_telegram_proto/src/api/api_blocked_peers.rs` | 226 | `tests/test_tcg_n11_api_blocked_peers.rs` |
| `Telegram/SourceFiles/api/api_blocked_peers.h` | `TCG-N11` | `ApiBlockedPeers` | `crates/gpui_telegram_proto/src/api/api_blocked_peers.rs` | 74 | `tests/test_tcg_n11_api_blocked_peers.rs` |
| `Telegram/SourceFiles/api/api_bot.cpp` | `TCG-N27` | `ApiBot` | `crates/gpui_telegram_proto/src/api/api_bot.rs` | 665 | `tests/test_tcg_n27_api_bot.rs` |
| `Telegram/SourceFiles/api/api_bot.h` | `TCG-N27` | `ApiBot` | `crates/gpui_telegram_proto/src/api/api_bot.rs` | 44 | `tests/test_tcg_n27_api_bot.rs` |
| `Telegram/SourceFiles/api/api_chat_filters.cpp` | `TCG-N37` | `ApiChatFilters` | `crates/gpui_telegram_proto/src/api/api_chat_filters.rs` | 1063 | `tests/test_tcg_n37_api_chat_filters.rs` |
| `Telegram/SourceFiles/api/api_chat_filters.h` | `TCG-N37` | `ApiChatFilters` | `crates/gpui_telegram_proto/src/api/api_chat_filters.rs` | 49 | `tests/test_tcg_n37_api_chat_filters.rs` |
| `Telegram/SourceFiles/api/api_chat_filters_remove_manager.cpp` | `TCG-N37` | `ApiChatFiltersRemoveManager` | `crates/gpui_telegram_proto/src/api/api_chat_filters_remove_manager.rs` | 128 | `tests/test_tcg_n37_api_chat_filters_remove_manager.rs` |
| `Telegram/SourceFiles/api/api_chat_filters_remove_manager.h` | `TCG-N37` | `ApiChatFiltersRemoveManager` | `crates/gpui_telegram_proto/src/api/api_chat_filters_remove_manager.rs` | 35 | `tests/test_tcg_n37_api_chat_filters_remove_manager.rs` |
| `Telegram/SourceFiles/api/api_chat_invite.cpp` | `TCG-N11` | `ApiChatInvite` | `crates/gpui_telegram_proto/src/api/api_chat_invite.rs` | 760 | `tests/test_tcg_n11_api_chat_invite.rs` |
| `Telegram/SourceFiles/api/api_chat_invite.h` | `TCG-N11` | `ApiChatInvite` | `crates/gpui_telegram_proto/src/api/api_chat_invite.rs` | 41 | `tests/test_tcg_n11_api_chat_invite.rs` |
| `Telegram/SourceFiles/api/api_chat_invite.style` | `TCG-N11` | `ApiChatInviteStyleSheet` | `crates/gpui_telegram_desktop/src/theme/api/api_chat_invite_style.rs` | 44 | `tests/test_tcg_n11_api_chat_invite.rs` |
| `Telegram/SourceFiles/api/api_chat_links.cpp` | `TCG-N11` | `ApiChatLinks` | `crates/gpui_telegram_proto/src/api/api_chat_links.rs` | 170 | `tests/test_tcg_n11_api_chat_links.rs` |
| `Telegram/SourceFiles/api/api_chat_links.h` | `TCG-N11` | `ApiChatLinks` | `crates/gpui_telegram_proto/src/api/api_chat_links.rs` | 64 | `tests/test_tcg_n11_api_chat_links.rs` |
| `Telegram/SourceFiles/api/api_chat_participants.cpp` | `TCG-N11` | `ApiChatParticipants` | `crates/gpui_telegram_proto/src/api/api_chat_participants.rs` | 948 | `tests/test_tcg_n11_api_chat_participants.rs` |
| `Telegram/SourceFiles/api/api_chat_participants.h` | `TCG-N11` | `ApiChatParticipants` | `crates/gpui_telegram_proto/src/api/api_chat_participants.rs` | 212 | `tests/test_tcg_n11_api_chat_participants.rs` |
| `Telegram/SourceFiles/api/api_cloud_password.cpp` | `TCG-N02` | `ApiCloudPassword` | `crates/gpui_telegram_proto/src/api/api_cloud_password.rs` | 612 | `tests/test_tcg_n02_api_cloud_password.rs` |
| `Telegram/SourceFiles/api/api_cloud_password.h` | `TCG-N02` | `ApiCloudPassword` | `crates/gpui_telegram_proto/src/api/api_cloud_password.rs` | 88 | `tests/test_tcg_n02_api_cloud_password.rs` |
| `Telegram/SourceFiles/api/api_common.cpp` | `TCG-N01` | `ApiCommon` | `crates/gpui_telegram_proto/src/api/api_common.rs` | 48 | `tests/test_tcg_n01_api_common.rs` |
| `Telegram/SourceFiles/api/api_common.h` | `TCG-N01` | `ApiCommon` | `crates/gpui_telegram_proto/src/api/api_common.rs` | 103 | `tests/test_tcg_n01_api_common.rs` |
| `Telegram/SourceFiles/api/api_communities.cpp` | `TCG-N01` | `ApiCommunities` | `crates/gpui_telegram_proto/src/api/api_communities.rs` | 408 | `tests/test_tcg_n01_api_communities.rs` |
| `Telegram/SourceFiles/api/api_communities.h` | `TCG-N01` | `ApiCommunities` | `crates/gpui_telegram_proto/src/api/api_communities.rs` | 136 | `tests/test_tcg_n01_api_communities.rs` |
| `Telegram/SourceFiles/api/api_compose_with_ai.cpp` | `TCG-N01` | `ApiComposeWithAi` | `crates/gpui_telegram_proto/src/api/api_compose_with_ai.rs` | 308 | `tests/test_tcg_n01_api_compose_with_ai.rs` |
| `Telegram/SourceFiles/api/api_compose_with_ai.h` | `TCG-N01` | `ApiComposeWithAi` | `crates/gpui_telegram_proto/src/api/api_compose_with_ai.rs` | 121 | `tests/test_tcg_n01_api_compose_with_ai.rs` |
| `Telegram/SourceFiles/api/api_confirm_phone.cpp` | `TCG-N01` | `ApiConfirmPhone` | `crates/gpui_telegram_proto/src/api/api_confirm_phone.rs` | 172 | `tests/test_tcg_n01_api_confirm_phone.rs` |
| `Telegram/SourceFiles/api/api_confirm_phone.h` | `TCG-N01` | `ApiConfirmPhone` | `crates/gpui_telegram_proto/src/api/api_confirm_phone.rs` | 36 | `tests/test_tcg_n01_api_confirm_phone.rs` |
| `Telegram/SourceFiles/api/api_credits.cpp` | `TCG-N01` | `ApiCredits` | `crates/gpui_telegram_proto/src/api/api_credits.rs` | 416 | `tests/test_tcg_n01_api_credits.rs` |
| `Telegram/SourceFiles/api/api_credits.h` | `TCG-N01` | `ApiCredits` | `crates/gpui_telegram_proto/src/api/api_credits.rs` | 131 | `tests/test_tcg_n01_api_credits.rs` |
| `Telegram/SourceFiles/api/api_credits_history_entry.cpp` | `TCG-N01` | `ApiCreditsHistoryEntry` | `crates/gpui_telegram_proto/src/api/api_credits_history_entry.rs` | 171 | `tests/test_tcg_n01_api_credits_history_entry.rs` |
| `Telegram/SourceFiles/api/api_credits_history_entry.h` | `TCG-N01` | `ApiCreditsHistoryEntry` | `crates/gpui_telegram_proto/src/api/api_credits_history_entry.rs` | 22 | `tests/test_tcg_n01_api_credits_history_entry.rs` |
| `Telegram/SourceFiles/api/api_earn.cpp` | `TCG-N01` | `ApiEarn` | `crates/gpui_telegram_proto/src/api/api_earn.rs` | 171 | `tests/test_tcg_n01_api_earn.rs` |
| `Telegram/SourceFiles/api/api_earn.h` | `TCG-N01` | `ApiEarn` | `crates/gpui_telegram_proto/src/api/api_earn.rs` | 35 | `tests/test_tcg_n01_api_earn.rs` |
| `Telegram/SourceFiles/api/api_editing.cpp` | `TCG-N01` | `ApiEditing` | `crates/gpui_telegram_proto/src/api/api_editing.rs` | 805 | `tests/test_tcg_n01_api_editing.rs` |
| `Telegram/SourceFiles/api/api_editing.h` | `TCG-N01` | `ApiEditing` | `crates/gpui_telegram_proto/src/api/api_editing.rs` | 76 | `tests/test_tcg_n01_api_editing.rs` |
| `Telegram/SourceFiles/api/api_filter_updates.h` | `TCG-N37` | `ApiFilterUpdates` | `crates/gpui_telegram_proto/src/api/api_filter_updates.rs` | 27 | `tests/test_tcg_n37_api_filter_updates.rs` |
| `Telegram/SourceFiles/api/api_global_privacy.cpp` | `TCG-N33` | `ApiGlobalPrivacy` | `crates/gpui_telegram_proto/src/api/api_global_privacy.rs` | 341 | `tests/test_tcg_n33_api_global_privacy.rs` |
| `Telegram/SourceFiles/api/api_global_privacy.h` | `TCG-N33` | `ApiGlobalPrivacy` | `crates/gpui_telegram_proto/src/api/api_global_privacy.rs` | 111 | `tests/test_tcg_n33_api_global_privacy.rs` |
| `Telegram/SourceFiles/api/api_hash.cpp` | `TCG-N01` | `ApiHash` | `crates/gpui_telegram_proto/src/api/api_hash.rs` | 139 | `tests/test_tcg_n01_api_hash.rs` |
| `Telegram/SourceFiles/api/api_hash.h` | `TCG-N01` | `ApiHash` | `crates/gpui_telegram_proto/src/api/api_hash.rs` | 72 | `tests/test_tcg_n01_api_hash.rs` |
| `Telegram/SourceFiles/api/api_invite_links.cpp` | `TCG-N01` | `ApiInviteLinks` | `crates/gpui_telegram_proto/src/api/api_invite_links.rs` | 807 | `tests/test_tcg_n01_api_invite_links.rs` |
| `Telegram/SourceFiles/api/api_invite_links.h` | `TCG-N01` | `ApiInviteLinks` | `crates/gpui_telegram_proto/src/api/api_invite_links.rs` | 245 | `tests/test_tcg_n01_api_invite_links.rs` |
| `Telegram/SourceFiles/api/api_media.cpp` | `TCG-N01` | `ApiMedia` | `crates/gpui_telegram_proto/src/api/api_media.rs` | 156 | `tests/test_tcg_n01_api_media.rs` |
| `Telegram/SourceFiles/api/api_media.h` | `TCG-N01` | `ApiMedia` | `crates/gpui_telegram_proto/src/api/api_media.rs` | 26 | `tests/test_tcg_n01_api_media.rs` |
| `Telegram/SourceFiles/api/api_messages_search.cpp` | `TCG-N21` | `ApiMessagesSearch` | `crates/gpui_telegram_proto/src/api/api_messages_search.rs` | 226 | `tests/test_tcg_n21_api_messages_search.rs` |
| `Telegram/SourceFiles/api/api_messages_search.h` | `TCG-N21` | `ApiMessagesSearch` | `crates/gpui_telegram_proto/src/api/api_messages_search.rs` | 81 | `tests/test_tcg_n21_api_messages_search.rs` |
| `Telegram/SourceFiles/api/api_messages_search_merged.cpp` | `TCG-N21` | `ApiMessagesSearchMerged` | `crates/gpui_telegram_proto/src/api/api_messages_search_merged.rs` | 120 | `tests/test_tcg_n21_api_messages_search_merged.rs` |
| `Telegram/SourceFiles/api/api_messages_search_merged.h` | `TCG-N21` | `ApiMessagesSearchMerged` | `crates/gpui_telegram_proto/src/api/api_messages_search_merged.rs` | 61 | `tests/test_tcg_n21_api_messages_search_merged.rs` |
| `Telegram/SourceFiles/api/api_peer_colors.cpp` | `TCG-N11` | `ApiPeerColors` | `crates/gpui_telegram_proto/src/api/api_peer_colors.rs` | 287 | `tests/test_tcg_n11_api_peer_colors.rs` |
| `Telegram/SourceFiles/api/api_peer_colors.h` | `TCG-N11` | `ApiPeerColors` | `crates/gpui_telegram_proto/src/api/api_peer_colors.rs` | 83 | `tests/test_tcg_n11_api_peer_colors.rs` |
| `Telegram/SourceFiles/api/api_peer_photo.cpp` | `TCG-N11` | `ApiPeerPhoto` | `crates/gpui_telegram_proto/src/api/api_peer_photo.rs` | 923 | `tests/test_tcg_n11_api_peer_photo.rs` |
| `Telegram/SourceFiles/api/api_peer_photo.h` | `TCG-N11` | `ApiPeerPhoto` | `crates/gpui_telegram_proto/src/api/api_peer_photo.rs` | 177 | `tests/test_tcg_n11_api_peer_photo.rs` |
| `Telegram/SourceFiles/api/api_peer_search.cpp` | `TCG-N21` | `ApiPeerSearch` | `crates/gpui_telegram_proto/src/api/api_peer_search.rs` | 180 | `tests/test_tcg_n21_api_peer_search.rs` |
| `Telegram/SourceFiles/api/api_peer_search.h` | `TCG-N21` | `ApiPeerSearch` | `crates/gpui_telegram_proto/src/api/api_peer_search.rs` | 78 | `tests/test_tcg_n21_api_peer_search.rs` |
| `Telegram/SourceFiles/api/api_polls.cpp` | `TCG-N16` | `ApiPolls` | `crates/gpui_telegram_proto/src/api/api_polls.rs` | 583 | `tests/test_tcg_n16_api_polls.rs` |
| `Telegram/SourceFiles/api/api_polls.h` | `TCG-N16` | `ApiPolls` | `crates/gpui_telegram_proto/src/api/api_polls.rs` | 67 | `tests/test_tcg_n16_api_polls.rs` |
| `Telegram/SourceFiles/api/api_premium.cpp` | `TCG-N01` | `ApiPremium` | `crates/gpui_telegram_proto/src/api/api_premium.rs` | 1184 | `tests/test_tcg_n01_api_premium.rs` |
| `Telegram/SourceFiles/api/api_premium.h` | `TCG-N01` | `ApiPremium` | `crates/gpui_telegram_proto/src/api/api_premium.rs` | 320 | `tests/test_tcg_n01_api_premium.rs` |
| `Telegram/SourceFiles/api/api_premium_option.cpp` | `TCG-N01` | `ApiPremiumOption` | `crates/gpui_telegram_proto/src/api/api_premium_option.rs` | 50 | `tests/test_tcg_n01_api_premium_option.rs` |
| `Telegram/SourceFiles/api/api_premium_option.h` | `TCG-N01` | `ApiPremiumOption` | `crates/gpui_telegram_proto/src/api/api_premium_option.rs` | 67 | `tests/test_tcg_n01_api_premium_option.rs` |
| `Telegram/SourceFiles/api/api_reactions_notify_settings.cpp` | `TCG-N14` | `ApiReactionsNotifySettings` | `crates/gpui_telegram_proto/src/api/api_reactions_notify_settings.rs` | 185 | `tests/test_tcg_n14_api_reactions_notify_settings.rs` |
| `Telegram/SourceFiles/api/api_reactions_notify_settings.h` | `TCG-N14` | `ApiReactionsNotifySettings` | `crates/gpui_telegram_proto/src/api/api_reactions_notify_settings.rs` | 65 | `tests/test_tcg_n14_api_reactions_notify_settings.rs` |
| `Telegram/SourceFiles/api/api_read_metrics.cpp` | `TCG-N01` | `ApiReadMetrics` | `crates/gpui_telegram_proto/src/api/api_read_metrics.rs` | 73 | `tests/test_tcg_n01_api_read_metrics.rs` |
| `Telegram/SourceFiles/api/api_read_metrics.h` | `TCG-N01` | `ApiReadMetrics` | `crates/gpui_telegram_proto/src/api/api_read_metrics.rs` | 45 | `tests/test_tcg_n01_api_read_metrics.rs` |
| `Telegram/SourceFiles/api/api_report.cpp` | `TCG-N01` | `ApiReport` | `crates/gpui_telegram_proto/src/api/api_report.rs` | 239 | `tests/test_tcg_n01_api_report.rs` |
| `Telegram/SourceFiles/api/api_report.h` | `TCG-N01` | `ApiReport` | `crates/gpui_telegram_proto/src/api/api_report.rs` | 81 | `tests/test_tcg_n01_api_report.rs` |
| `Telegram/SourceFiles/api/api_rich_tasks.cpp` | `TCG-N01` | `ApiRichTasks` | `crates/gpui_telegram_proto/src/api/api_rich_tasks.rs` | 135 | `tests/test_tcg_n01_api_rich_tasks.rs` |
| `Telegram/SourceFiles/api/api_rich_tasks.h` | `TCG-N01` | `ApiRichTasks` | `crates/gpui_telegram_proto/src/api/api_rich_tasks.rs` | 58 | `tests/test_tcg_n01_api_rich_tasks.rs` |
| `Telegram/SourceFiles/api/api_ringtones.cpp` | `TCG-N01` | `ApiRingtones` | `crates/gpui_telegram_proto/src/api/api_ringtones.rs` | 205 | `tests/test_tcg_n01_api_ringtones.rs` |
| `Telegram/SourceFiles/api/api_ringtones.h` | `TCG-N01` | `ApiRingtones` | `crates/gpui_telegram_proto/src/api/api_ringtones.rs` | 69 | `tests/test_tcg_n01_api_ringtones.rs` |
| `Telegram/SourceFiles/api/api_self_destruct.cpp` | `TCG-N01` | `ApiSelfDestruct` | `crates/gpui_telegram_proto/src/api/api_self_destruct.rs` | 76 | `tests/test_tcg_n01_api_self_destruct.rs` |
| `Telegram/SourceFiles/api/api_self_destruct.h` | `TCG-N01` | `ApiSelfDestruct` | `crates/gpui_telegram_proto/src/api/api_self_destruct.rs` | 42 | `tests/test_tcg_n01_api_self_destruct.rs` |
| `Telegram/SourceFiles/api/api_send_progress.cpp` | `TCG-N01` | `ApiSendProgress` | `crates/gpui_telegram_proto/src/api/api_send_progress.rs` | 182 | `tests/test_tcg_n01_api_send_progress.rs` |
| `Telegram/SourceFiles/api/api_send_progress.h` | `TCG-N01` | `ApiSendProgress` | `crates/gpui_telegram_proto/src/api/api_send_progress.rs` | 105 | `tests/test_tcg_n01_api_send_progress.rs` |
| `Telegram/SourceFiles/api/api_sending.cpp` | `TCG-N19` | `ApiSending` | `crates/gpui_telegram_proto/src/api/api_sending.rs` | 1339 | `tests/test_tcg_n19_api_sending.rs` |
| `Telegram/SourceFiles/api/api_sending.h` | `TCG-N19` | `ApiSending` | `crates/gpui_telegram_proto/src/api/api_sending.rs` | 70 | `tests/test_tcg_n19_api_sending.rs` |
| `Telegram/SourceFiles/api/api_sensitive_content.cpp` | `TCG-N01` | `ApiSensitiveContent` | `crates/gpui_telegram_proto/src/api/api_sensitive_content.rs` | 121 | `tests/test_tcg_n01_api_sensitive_content.rs` |
| `Telegram/SourceFiles/api/api_sensitive_content.h` | `TCG-N01` | `ApiSensitiveContent` | `crates/gpui_telegram_proto/src/api/api_sensitive_content.rs` | 51 | `tests/test_tcg_n01_api_sensitive_content.rs` |
| `Telegram/SourceFiles/api/api_share_themes.cpp` | `TCG-N31` | `ApiShareThemes` | `crates/gpui_telegram_proto/src/api/api_share_themes.rs` | 233 | `tests/test_tcg_n31_api_share_themes.rs` |
| `Telegram/SourceFiles/api/api_share_themes.h` | `TCG-N31` | `ApiShareThemes` | `crates/gpui_telegram_proto/src/api/api_share_themes.rs` | 60 | `tests/test_tcg_n31_api_share_themes.rs` |
| `Telegram/SourceFiles/api/api_single_message_search.cpp` | `TCG-N21` | `ApiSingleMessageSearch` | `crates/gpui_telegram_proto/src/api/api_single_message_search.rs` | 237 | `tests/test_tcg_n21_api_single_message_search.rs` |
| `Telegram/SourceFiles/api/api_single_message_search.h` | `TCG-N21` | `ApiSingleMessageSearch` | `crates/gpui_telegram_proto/src/api/api_single_message_search.rs` | 76 | `tests/test_tcg_n21_api_single_message_search.rs` |
| `Telegram/SourceFiles/api/api_statistics.cpp` | `TCG-N12` | `ApiStatistics` | `crates/gpui_telegram_proto/src/api/api_statistics.rs` | 822 | `tests/test_tcg_n12_api_statistics.rs` |
| `Telegram/SourceFiles/api/api_statistics.h` | `TCG-N12` | `ApiStatistics` | `crates/gpui_telegram_proto/src/api/api_statistics.rs` | 128 | `tests/test_tcg_n12_api_statistics.rs` |
| `Telegram/SourceFiles/api/api_statistics_data_deserialize.cpp` | `TCG-N12` | `ApiStatisticsDataDeserialize` | `crates/gpui_telegram_proto/src/api/api_statistics_data_deserialize.rs` | 35 | `tests/test_tcg_n12_api_statistics_data_deserialize.rs` |
| `Telegram/SourceFiles/api/api_statistics_data_deserialize.h` | `TCG-N12` | `ApiStatisticsDataDeserialize` | `crates/gpui_telegram_proto/src/api/api_statistics_data_deserialize.rs` | 19 | `tests/test_tcg_n12_api_statistics_data_deserialize.rs` |
| `Telegram/SourceFiles/api/api_statistics_sender.cpp` | `TCG-N12` | `ApiStatisticsSender` | `crates/gpui_telegram_proto/src/api/api_statistics_sender.rs` | 86 | `tests/test_tcg_n12_api_statistics_sender.rs` |
| `Telegram/SourceFiles/api/api_statistics_sender.h` | `TCG-N12` | `ApiStatisticsSender` | `crates/gpui_telegram_proto/src/api/api_statistics_sender.rs` | 58 | `tests/test_tcg_n12_api_statistics_sender.rs` |
| `Telegram/SourceFiles/api/api_stickers_creator.cpp` | `TCG-N15` | `ApiStickersCreator` | `crates/gpui_telegram_proto/src/api/api_stickers_creator.rs` | 601 | `tests/test_tcg_n15_api_stickers_creator.rs` |
| `Telegram/SourceFiles/api/api_stickers_creator.h` | `TCG-N15` | `ApiStickersCreator` | `crates/gpui_telegram_proto/src/api/api_stickers_creator.rs` | 127 | `tests/test_tcg_n15_api_stickers_creator.rs` |
| `Telegram/SourceFiles/api/api_suggest_post.cpp` | `TCG-N01` | `ApiSuggestPost` | `crates/gpui_telegram_proto/src/api/api_suggest_post.rs` | 707 | `tests/test_tcg_n01_api_suggest_post.rs` |
| `Telegram/SourceFiles/api/api_suggest_post.h` | `TCG-N01` | `ApiSuggestPost` | `crates/gpui_telegram_proto/src/api/api_suggest_post.rs` | 29 | `tests/test_tcg_n01_api_suggest_post.rs` |
| `Telegram/SourceFiles/api/api_text_entities.cpp` | `TCG-N01` | `ApiTextEntities` | `crates/gpui_telegram_proto/src/api/api_text_entities.rs` | 459 | `tests/test_tcg_n01_api_text_entities.rs` |
| `Telegram/SourceFiles/api/api_text_entities.h` | `TCG-N01` | `ApiTextEntities` | `crates/gpui_telegram_proto/src/api/api_text_entities.rs` | 36 | `tests/test_tcg_n01_api_text_entities.rs` |
| `Telegram/SourceFiles/api/api_todo_lists.cpp` | `TCG-N01` | `ApiTodoLists` | `crates/gpui_telegram_proto/src/api/api_todo_lists.rs` | 258 | `tests/test_tcg_n01_api_todo_lists.rs` |
| `Telegram/SourceFiles/api/api_todo_lists.h` | `TCG-N01` | `ApiTodoLists` | `crates/gpui_telegram_proto/src/api/api_todo_lists.rs` | 69 | `tests/test_tcg_n01_api_todo_lists.rs` |
| `Telegram/SourceFiles/api/api_toggling_media.cpp` | `TCG-N01` | `ApiTogglingMedia` | `crates/gpui_telegram_proto/src/api/api_toggling_media.rs` | 141 | `tests/test_tcg_n01_api_toggling_media.rs` |
| `Telegram/SourceFiles/api/api_toggling_media.h` | `TCG-N01` | `ApiTogglingMedia` | `crates/gpui_telegram_proto/src/api/api_toggling_media.rs` | 44 | `tests/test_tcg_n01_api_toggling_media.rs` |
| `Telegram/SourceFiles/api/api_transcribes.cpp` | `TCG-N01` | `ApiTranscribes` | `crates/gpui_telegram_proto/src/api/api_transcribes.rs` | 309 | `tests/test_tcg_n01_api_transcribes.rs` |
| `Telegram/SourceFiles/api/api_transcribes.h` | `TCG-N01` | `ApiTranscribes` | `crates/gpui_telegram_proto/src/api/api_transcribes.rs` | 81 | `tests/test_tcg_n01_api_transcribes.rs` |
| `Telegram/SourceFiles/api/api_unread_things.cpp` | `TCG-N01` | `ApiUnreadThings` | `crates/gpui_telegram_proto/src/api/api_unread_things.rs` | 223 | `tests/test_tcg_n01_api_unread_things.rs` |
| `Telegram/SourceFiles/api/api_unread_things.h` | `TCG-N01` | `ApiUnreadThings` | `crates/gpui_telegram_proto/src/api/api_unread_things.rs` | 53 | `tests/test_tcg_n01_api_unread_things.rs` |
| `Telegram/SourceFiles/api/api_updates.cpp` | `TCG-N01` | `ApiUpdates` | `crates/gpui_telegram_proto/src/api/api_updates.rs` | 2969 | `tests/test_tcg_n01_api_updates.rs` |
| `Telegram/SourceFiles/api/api_updates.h` | `TCG-N01` | `ApiUpdates` | `crates/gpui_telegram_proto/src/api/api_updates.rs` | 227 | `tests/test_tcg_n01_api_updates.rs` |
| `Telegram/SourceFiles/api/api_user_names.cpp` | `TCG-N01` | `ApiUserNames` | `crates/gpui_telegram_proto/src/api/api_user_names.rs` | 280 | `tests/test_tcg_n01_api_user_names.rs` |
| `Telegram/SourceFiles/api/api_user_names.h` | `TCG-N01` | `ApiUserNames` | `crates/gpui_telegram_proto/src/api/api_user_names.rs` | 64 | `tests/test_tcg_n01_api_user_names.rs` |
| `Telegram/SourceFiles/api/api_user_privacy.cpp` | `TCG-N33` | `ApiUserPrivacy` | `crates/gpui_telegram_proto/src/api/api_user_privacy.rs` | 345 | `tests/test_tcg_n33_api_user_privacy.rs` |
| `Telegram/SourceFiles/api/api_user_privacy.h` | `TCG-N33` | `ApiUserPrivacy` | `crates/gpui_telegram_proto/src/api/api_user_privacy.rs` | 85 | `tests/test_tcg_n33_api_user_privacy.rs` |
| `Telegram/SourceFiles/api/api_views.cpp` | `TCG-N01` | `ApiViews` | `crates/gpui_telegram_proto/src/api/api_views.rs` | 252 | `tests/test_tcg_n01_api_views.rs` |
| `Telegram/SourceFiles/api/api_views.h` | `TCG-N01` | `ApiViews` | `crates/gpui_telegram_proto/src/api/api_views.rs` | 69 | `tests/test_tcg_n01_api_views.rs` |
| `Telegram/SourceFiles/api/api_websites.cpp` | `TCG-N01` | `ApiWebsites` | `crates/gpui_telegram_proto/src/api/api_websites.rs` | 134 | `tests/test_tcg_n01_api_websites.rs` |
| `Telegram/SourceFiles/api/api_websites.h` | `TCG-N01` | `ApiWebsites` | `crates/gpui_telegram_proto/src/api/api_websites.rs` | 62 | `tests/test_tcg_n01_api_websites.rs` |
| `Telegram/SourceFiles/api/api_who_reacted.cpp` | `TCG-N01` | `ApiWhoReacted` | `crates/gpui_telegram_proto/src/api/api_who_reacted.rs` | 837 | `tests/test_tcg_n01_api_who_reacted.rs` |
| `Telegram/SourceFiles/api/api_who_reacted.h` | `TCG-N01` | `ApiWhoReacted` | `crates/gpui_telegram_proto/src/api/api_who_reacted.rs` | 71 | `tests/test_tcg_n01_api_who_reacted.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/api/api_attached_stickers.h`
- **Classes & Structs**: ApiWrap, DocumentData, PhotoData, SessionController, AttachedStickers
- **Core Methods & Signals**: AttachedStickers, requestAttachedStickerSets, request
- **Target**: `ApiAttachedStickers` in `crates/gpui_telegram_proto/src/api/api_attached_stickers.rs`

### `Telegram/SourceFiles/api/api_authorizations.cpp`
- **Classes & Structs**: Flag
- **Core Methods & Signals**: Authorizations::Entry, QString::number, Core::FormatVersionDisplay, Core::App, QString::fromUtf8, Authorizations::ActiveDateString
- **Target**: `ApiAuthorizations` in `crates/gpui_telegram_proto/src/api/api_authorizations.rs`

### `Telegram/SourceFiles/api/api_authorizations.h`
- **Classes & Structs**: ApiWrap, Authorizations, Entry, List
- **Core Methods & Signals**: Authorizations, reload, cancelCurrentRequest, apply, review, updateTTL
- **Target**: `ApiAuthorizations` in `crates/gpui_telegram_proto/src/api/api_authorizations.rs`

### `Telegram/SourceFiles/api/api_blocked_peers.h`
- **Classes & Structs**: ApiWrap, Session, BlockedPeers, Slice, Item, Request
- **Core Methods & Signals**: BlockedPeers, reload, slice, block, rpl::slice
- **Target**: `ApiBlockedPeers` in `crates/gpui_telegram_proto/src/api/api_blocked_peers.rs`

### `Telegram/SourceFiles/api/api_bot.cpp`
- **Classes & Structs**: ButtonType, Flag
- **Core Methods & Signals**: MTPmessages_GetBotCallbackAnswer::Flags, gsl::finally, Ui::MakeInformBox, UrlClickHandler::Open, QVariant::fromValue, MTPephemeral_GetCallbackAnswer::Flag
- **Target**: `ApiBot` in `crates/gpui_telegram_proto/src/api/api_bot.rs`

### `Telegram/SourceFiles/api/api_bot.h`
- **Classes & Structs**: ClickHandlerContext, HistoryItem, HistoryMessageMarkupButton, SessionController, BotButtonLookup
- **Core Methods & Signals**: SendBotCallbackData, SendBotCallbackDataWithPassword, SwitchInlineBotButtonReceived, ActivateBotButton, ActivateBotCommand, ActivateRichPageBotButton
- **Target**: `ApiBot` in `crates/gpui_telegram_proto/src/api/api_bot.rs`

### `Telegram/SourceFiles/api/api_chat_filters.cpp`
- **Classes & Structs**: ToggleAction, PreviewableRow, ToggleChatsController, State, Type
- **Core Methods & Signals**: PreviewableRow::rightActionSize, PreviewableRow::rightActionMargins, PreviewableRow::rightActionPaint, PreviewableRow::rightActionAddRipple, RippleAnimation::EllipseMask, QCursor::pos
- **Target**: `ApiChatFilters` in `crates/gpui_telegram_proto/src/api/api_chat_filters.rs`

### `Telegram/SourceFiles/api/api_chat_filters.h`
- **Classes & Structs**: Session, SessionController, ChatFilter, ChatFilterTitle
- **Core Methods & Signals**: SaveNewFilterPinned, CheckFilterInvite, ProcessFilterUpdate
- **Target**: `ApiChatFilters` in `crates/gpui_telegram_proto/src/api/api_chat_filters.rs`

### `Telegram/SourceFiles/api/api_chat_filters_remove_manager.h`
- **Classes & Structs**: SessionController, RpWidget, RemoveComplexChatFilter
- **Core Methods & Signals**: RemoveComplexChatFilter, request
- **Target**: `ApiChatFiltersRemoveManager` in `crates/gpui_telegram_proto/src/api/api_chat_filters_remove_manager.rs`

### `Telegram/SourceFiles/api/api_chat_invite.cpp`
- **Classes & Structs**: InviteParticipant, ChatInvite, State, Type
- **Core Methods & Signals**: tr::lng_group_request_sent, tr::lng_group_request_sent_channel, tr::lng_group_invite_no_room, tr::lng_group_invite_bad_link, Ui::AddSkip, Ui::PaintOutlinedColoredCreditsIconCallback
- **Target**: `ApiChatInvite` in `crates/gpui_telegram_proto/src/api/api_chat_invite.rs`

