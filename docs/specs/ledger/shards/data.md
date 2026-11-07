# Subsystem Shard: `data`

**Total Source Files**: 226  
**Total Source Lines**: 83,733  
**Primary Feature IDs**: CAP-06, TCG-N02, TCG-N04, TCG-N11  
**JSON Shard Path**: `docs/specs/ledger/shards/data.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/data/business/data_business_chatbots.cpp` | `TCG-N27` | `DataBusinessChatbotsBusiness` | `crates/gpui_telegram_client/src/data/business/data_business_chatbots.rs` | 217 | `tests/test_tcg_n27_data_business_chatbots.rs` |
| `Telegram/SourceFiles/data/business/data_business_chatbots.h` | `TCG-N27` | `DataBusinessChatbotsBusiness` | `crates/gpui_telegram_client/src/data/business/data_business_chatbots.rs` | 76 | `tests/test_tcg_n27_data_business_chatbots.rs` |
| `Telegram/SourceFiles/data/business/data_business_common.cpp` | `TCG-N04` | `DataBusinessCommonBusiness` | `crates/gpui_telegram_client/src/data/business/data_business_common.rs` | 398 | `tests/test_tcg_n04_data_business_common.rs` |
| `Telegram/SourceFiles/data/business/data_business_common.h` | `TCG-N04` | `DataBusinessCommonBusiness` | `crates/gpui_telegram_client/src/data/business/data_business_common.rs` | 302 | `tests/test_tcg_n04_data_business_common.rs` |
| `Telegram/SourceFiles/data/business/data_business_info.cpp` | `TCG-N04` | `DataBusinessInfoBusiness` | `crates/gpui_telegram_client/src/data/business/data_business_info.rs` | 318 | `tests/test_tcg_n04_data_business_info.rs` |
| `Telegram/SourceFiles/data/business/data_business_info.h` | `TCG-N04` | `DataBusinessInfoBusiness` | `crates/gpui_telegram_client/src/data/business/data_business_info.rs` | 64 | `tests/test_tcg_n04_data_business_info.rs` |
| `Telegram/SourceFiles/data/business/data_shortcut_messages.cpp` | `TCG-N45` | `DataShortcutMessagesBusiness` | `crates/gpui_telegram_client/src/data/business/data_shortcut_messages.rs` | 791 | `tests/test_tcg_n45_data_shortcut_messages.rs` |
| `Telegram/SourceFiles/data/business/data_shortcut_messages.h` | `TCG-N45` | `DataShortcutMessagesBusiness` | `crates/gpui_telegram_client/src/data/business/data_shortcut_messages.rs` | 154 | `tests/test_tcg_n45_data_shortcut_messages.rs` |
| `Telegram/SourceFiles/data/components/credits.cpp` | `TCG-N04` | `CreditsComponents` | `crates/gpui_telegram_client/src/data/components/credits.rs` | 272 | `tests/test_tcg_n04_credits.rs` |
| `Telegram/SourceFiles/data/components/credits.h` | `TCG-N04` | `CreditsComponents` | `crates/gpui_telegram_client/src/data/components/credits.rs` | 83 | `tests/test_tcg_n04_credits.rs` |
| `Telegram/SourceFiles/data/components/ephemeral_messages.cpp` | `TCG-N04` | `EphemeralMessagesComponents` | `crates/gpui_telegram_client/src/data/components/ephemeral_messages.rs` | 1157 | `tests/test_tcg_n04_ephemeral_messages.rs` |
| `Telegram/SourceFiles/data/components/ephemeral_messages.h` | `TCG-N04` | `EphemeralMessagesComponents` | `crates/gpui_telegram_client/src/data/components/ephemeral_messages.rs` | 171 | `tests/test_tcg_n04_ephemeral_messages.rs` |
| `Telegram/SourceFiles/data/components/factchecks.cpp` | `TCG-N04` | `FactchecksComponents` | `crates/gpui_telegram_client/src/data/components/factchecks.rs` | 218 | `tests/test_tcg_n04_factchecks.rs` |
| `Telegram/SourceFiles/data/components/factchecks.h` | `TCG-N04` | `FactchecksComponents` | `crates/gpui_telegram_client/src/data/components/factchecks.rs` | 70 | `tests/test_tcg_n04_factchecks.rs` |
| `Telegram/SourceFiles/data/components/gift_auctions.cpp` | `TCG-N15` | `GiftAuctionsComponents` | `crates/gpui_telegram_client/src/data/components/gift_auctions.rs` | 559 | `tests/test_tcg_n15_gift_auctions.rs` |
| `Telegram/SourceFiles/data/components/gift_auctions.h` | `TCG-N15` | `GiftAuctionsComponents` | `crates/gpui_telegram_client/src/data/components/gift_auctions.rs` | 166 | `tests/test_tcg_n15_gift_auctions.rs` |
| `Telegram/SourceFiles/data/components/location_pickers.cpp` | `TCG-N41` | `LocationPickersComponents` | `crates/gpui_telegram_client/src/data/components/location_pickers.rs` | 44 | `tests/test_tcg_n41_location_pickers.rs` |
| `Telegram/SourceFiles/data/components/location_pickers.h` | `TCG-N41` | `LocationPickersComponents` | `crates/gpui_telegram_client/src/data/components/location_pickers.rs` | 39 | `tests/test_tcg_n41_location_pickers.rs` |
| `Telegram/SourceFiles/data/components/passkeys.cpp` | `TCG-N04` | `PasskeysComponents` | `crates/gpui_telegram_client/src/data/components/passkeys.rs` | 210 | `tests/test_tcg_n04_passkeys.rs` |
| `Telegram/SourceFiles/data/components/passkeys.h` | `TCG-N04` | `PasskeysComponents` | `crates/gpui_telegram_client/src/data/components/passkeys.rs` | 81 | `tests/test_tcg_n04_passkeys.rs` |
| `Telegram/SourceFiles/data/components/promo_suggestions.cpp` | `TCG-N04` | `PromoSuggestionsComponents` | `crates/gpui_telegram_client/src/data/components/promo_suggestions.rs` | 373 | `tests/test_tcg_n04_promo_suggestions.rs` |
| `Telegram/SourceFiles/data/components/promo_suggestions.h` | `TCG-N04` | `PromoSuggestionsComponents` | `crates/gpui_telegram_client/src/data/components/promo_suggestions.rs` | 108 | `tests/test_tcg_n04_promo_suggestions.rs` |
| `Telegram/SourceFiles/data/components/recent_inline_bots.cpp` | `TCG-N27` | `RecentInlineBotsComponents` | `crates/gpui_telegram_client/src/data/components/recent_inline_bots.rs` | 69 | `tests/test_tcg_n27_recent_inline_bots.rs` |
| `Telegram/SourceFiles/data/components/recent_inline_bots.h` | `TCG-N27` | `RecentInlineBotsComponents` | `crates/gpui_telegram_client/src/data/components/recent_inline_bots.rs` | 36 | `tests/test_tcg_n27_recent_inline_bots.rs` |
| `Telegram/SourceFiles/data/components/recent_money_recipients.cpp` | `TCG-N04` | `RecentMoneyRecipientsComponents` | `crates/gpui_telegram_client/src/data/components/recent_money_recipients.rs` | 192 | `tests/test_tcg_n04_recent_money_recipients.rs` |
| `Telegram/SourceFiles/data/components/recent_money_recipients.h` | `TCG-N04` | `RecentMoneyRecipientsComponents` | `crates/gpui_telegram_client/src/data/components/recent_money_recipients.rs` | 40 | `tests/test_tcg_n04_recent_money_recipients.rs` |
| `Telegram/SourceFiles/data/components/recent_peers.cpp` | `TCG-N11` | `RecentPeersComponents` | `crates/gpui_telegram_client/src/data/components/recent_peers.rs` | 183 | `tests/test_tcg_n11_recent_peers.rs` |
| `Telegram/SourceFiles/data/components/recent_peers.h` | `TCG-N11` | `RecentPeersComponents` | `crates/gpui_telegram_client/src/data/components/recent_peers.rs` | 58 | `tests/test_tcg_n11_recent_peers.rs` |
| `Telegram/SourceFiles/data/components/recent_shared_media_gifts.cpp` | `TCG-N15` | `RecentSharedMediaGiftsComponents` | `crates/gpui_telegram_client/src/data/components/recent_shared_media_gifts.rs` | 271 | `tests/test_tcg_n15_recent_shared_media_gifts.rs` |
| `Telegram/SourceFiles/data/components/recent_shared_media_gifts.h` | `TCG-N15` | `RecentSharedMediaGiftsComponents` | `crates/gpui_telegram_client/src/data/components/recent_shared_media_gifts.rs` | 73 | `tests/test_tcg_n15_recent_shared_media_gifts.rs` |
| `Telegram/SourceFiles/data/components/scheduled_messages.cpp` | `TCG-N18` | `ScheduledMessagesComponents` | `crates/gpui_telegram_client/src/data/components/scheduled_messages.rs` | 670 | `tests/test_tcg_n18_scheduled_messages.rs` |
| `Telegram/SourceFiles/data/components/scheduled_messages.h` | `TCG-N18` | `ScheduledMessagesComponents` | `crates/gpui_telegram_client/src/data/components/scheduled_messages.rs` | 100 | `tests/test_tcg_n18_scheduled_messages.rs` |
| `Telegram/SourceFiles/data/components/sponsored_messages.cpp` | `TCG-N04` | `SponsoredMessagesComponents` | `crates/gpui_telegram_client/src/data/components/sponsored_messages.rs` | 914 | `tests/test_tcg_n04_sponsored_messages.rs` |
| `Telegram/SourceFiles/data/components/sponsored_messages.h` | `TCG-N04` | `SponsoredMessagesComponents` | `crates/gpui_telegram_client/src/data/components/sponsored_messages.rs` | 257 | `tests/test_tcg_n04_sponsored_messages.rs` |
| `Telegram/SourceFiles/data/components/top_peers.cpp` | `TCG-N11` | `TopPeersComponents` | `crates/gpui_telegram_client/src/data/components/top_peers.rs` | 363 | `tests/test_tcg_n11_top_peers.rs` |
| `Telegram/SourceFiles/data/components/top_peers.h` | `TCG-N11` | `TopPeersComponents` | `crates/gpui_telegram_client/src/data/components/top_peers.rs` | 64 | `tests/test_tcg_n11_top_peers.rs` |
| `Telegram/SourceFiles/data/components/welcome_messages.cpp` | `TCG-N04` | `WelcomeMessagesComponents` | `crates/gpui_telegram_client/src/data/components/welcome_messages.rs` | 748 | `tests/test_tcg_n04_welcome_messages.rs` |
| `Telegram/SourceFiles/data/components/welcome_messages.h` | `TCG-N04` | `WelcomeMessagesComponents` | `crates/gpui_telegram_client/src/data/components/welcome_messages.rs` | 125 | `tests/test_tcg_n04_welcome_messages.rs` |
| `Telegram/SourceFiles/data/data_abstract_sparse_ids.h` | `TCG-N04` | `DataAbstractSparseIds` | `crates/gpui_telegram_client/src/data/data_abstract_sparse_ids.rs` | 83 | `tests/test_tcg_n04_data_abstract_sparse_ids.rs` |
| `Telegram/SourceFiles/data/data_abstract_structure.cpp` | `TCG-N04` | `DataAbstractStructure` | `crates/gpui_telegram_client/src/data/data_abstract_structure.rs` | 38 | `tests/test_tcg_n04_data_abstract_structure.rs` |
| `Telegram/SourceFiles/data/data_abstract_structure.h` | `TCG-N04` | `DataAbstractStructure` | `crates/gpui_telegram_client/src/data/data_abstract_structure.rs` | 70 | `tests/test_tcg_n04_data_abstract_structure.rs` |
| `Telegram/SourceFiles/data/data_ai_compose_tones.cpp` | `TCG-N04` | `DataAiComposeTones` | `crates/gpui_telegram_client/src/data/data_ai_compose_tones.rs` | 375 | `tests/test_tcg_n04_data_ai_compose_tones.rs` |
| `Telegram/SourceFiles/data/data_ai_compose_tones.h` | `TCG-N04` | `DataAiComposeTones` | `crates/gpui_telegram_client/src/data/data_ai_compose_tones.rs` | 118 | `tests/test_tcg_n04_data_ai_compose_tones.rs` |
| `Telegram/SourceFiles/data/data_audio_msg_id.cpp` | `TCG-N04` | `DataAudioMsgId` | `crates/gpui_telegram_client/src/data/data_audio_msg_id.rs` | 103 | `tests/test_tcg_n04_data_audio_msg_id.rs` |
| `Telegram/SourceFiles/data/data_audio_msg_id.h` | `TCG-N04` | `DataAudioMsgId` | `crates/gpui_telegram_client/src/data/data_audio_msg_id.rs` | 50 | `tests/test_tcg_n04_data_audio_msg_id.rs` |
| `Telegram/SourceFiles/data/data_authorization.h` | `TCG-N04` | `DataAuthorization` | `crates/gpui_telegram_client/src/data/data_authorization.rs` | 20 | `tests/test_tcg_n04_data_authorization.rs` |
| `Telegram/SourceFiles/data/data_auto_download.cpp` | `TCG-N04` | `DataAutoDownload` | `crates/gpui_telegram_client/src/data/data_auto_download.rs` | 492 | `tests/test_tcg_n04_data_auto_download.rs` |
| `Telegram/SourceFiles/data/data_auto_download.h` | `TCG-N04` | `DataAutoDownload` | `crates/gpui_telegram_client/src/data/data_auto_download.rs` | 149 | `tests/test_tcg_n04_data_auto_download.rs` |
| `Telegram/SourceFiles/data/data_birthday.cpp` | `TCG-N04` | `DataBirthday` | `crates/gpui_telegram_client/src/data/data_birthday.rs` | 132 | `tests/test_tcg_n04_data_birthday.rs` |
| `Telegram/SourceFiles/data/data_birthday.h` | `TCG-N04` | `DataBirthday` | `crates/gpui_telegram_client/src/data/data_birthday.rs` | 47 | `tests/test_tcg_n04_data_birthday.rs` |
| `Telegram/SourceFiles/data/data_boosts.h` | `TCG-N04` | `DataBoosts` | `crates/gpui_telegram_client/src/data/data_boosts.rs` | 73 | `tests/test_tcg_n04_data_boosts.rs` |
| `Telegram/SourceFiles/data/data_bot_app.cpp` | `TCG-N27` | `DataBotApp` | `crates/gpui_telegram_client/src/data/data_bot_app.rs` | 13 | `tests/test_tcg_n27_data_bot_app.rs` |
| `Telegram/SourceFiles/data/data_bot_app.h` | `TCG-N27` | `DataBotApp` | `crates/gpui_telegram_client/src/data/data_bot_app.rs` | 27 | `tests/test_tcg_n27_data_bot_app.rs` |
| `Telegram/SourceFiles/data/data_changes.cpp` | `TCG-N04` | `DataChanges` | `crates/gpui_telegram_client/src/data/data_changes.rs` | 352 | `tests/test_tcg_n04_data_changes.rs` |
| `Telegram/SourceFiles/data/data_changes.h` | `TCG-N04` | `DataChanges` | `crates/gpui_telegram_client/src/data/data_changes.rs` | 467 | `tests/test_tcg_n04_data_changes.rs` |
| `Telegram/SourceFiles/data/data_channel.cpp` | `TCG-N12` | `DataChannel` | `crates/gpui_telegram_client/src/data/data_channel.rs` | 1620 | `tests/test_tcg_n12_data_channel.rs` |
| `Telegram/SourceFiles/data/data_channel.h` | `TCG-N12` | `DataChannel` | `crates/gpui_telegram_client/src/data/data_channel.rs` | 677 | `tests/test_tcg_n12_data_channel.rs` |
| `Telegram/SourceFiles/data/data_channel_admins.cpp` | `TCG-N12` | `DataChannelAdmins` | `crates/gpui_telegram_client/src/data/data_channel_admins.rs` | 112 | `tests/test_tcg_n12_data_channel_admins.rs` |
| `Telegram/SourceFiles/data/data_channel_admins.h` | `TCG-N12` | `DataChannelAdmins` | `crates/gpui_telegram_client/src/data/data_channel_admins.rs` | 44 | `tests/test_tcg_n12_data_channel_admins.rs` |
| `Telegram/SourceFiles/data/data_channel_earn.h` | `TCG-N12` | `DataChannelEarn` | `crates/gpui_telegram_client/src/data/data_channel_earn.rs` | 42 | `tests/test_tcg_n12_data_channel_earn.rs` |
| `Telegram/SourceFiles/data/data_chat.cpp` | `TCG-N11` | `DataChat` | `crates/gpui_telegram_client/src/data/data_chat.rs` | 638 | `tests/test_tcg_n11_data_chat.rs` |
| `Telegram/SourceFiles/data/data_chat.h` | `TCG-N11` | `DataChat` | `crates/gpui_telegram_client/src/data/data_chat.rs` | 236 | `tests/test_tcg_n11_data_chat.rs` |
| `Telegram/SourceFiles/data/data_chat_filters.cpp` | `TCG-N37` | `DataChatFilters` | `crates/gpui_telegram_client/src/data/data_chat_filters.rs` | 1169 | `tests/test_tcg_n37_data_chat_filters.rs` |
| `Telegram/SourceFiles/data/data_chat_filters.h` | `TCG-N37` | `DataChatFilters` | `crates/gpui_telegram_client/src/data/data_chat_filters.rs` | 275 | `tests/test_tcg_n37_data_chat_filters.rs` |
| `Telegram/SourceFiles/data/data_chat_participant_status.cpp` | `TCG-N11` | `DataChatParticipantStatus` | `crates/gpui_telegram_client/src/data/data_chat_participant_status.rs` | 646 | `tests/test_tcg_n11_data_chat_participant_status.rs` |
| `Telegram/SourceFiles/data/data_chat_participant_status.h` | `TCG-N11` | `DataChatParticipantStatus` | `crates/gpui_telegram_client/src/data/data_chat_participant_status.rs` | 287 | `tests/test_tcg_n11_data_chat_participant_status.rs` |
| `Telegram/SourceFiles/data/data_cloud_file.cpp` | `TCG-N04` | `DataCloudFile` | `crates/gpui_telegram_client/src/data/data_cloud_file.rs` | 379 | `tests/test_tcg_n04_data_cloud_file.rs` |
| `Telegram/SourceFiles/data/data_cloud_file.h` | `TCG-N04` | `DataCloudFile` | `crates/gpui_telegram_client/src/data/data_cloud_file.rs` | 119 | `tests/test_tcg_n04_data_cloud_file.rs` |
| `Telegram/SourceFiles/data/data_cloud_themes.cpp` | `TCG-N31` | `DataCloudThemes` | `crates/gpui_telegram_client/src/data/data_cloud_themes.rs` | 899 | `tests/test_tcg_n31_data_cloud_themes.rs` |
| `Telegram/SourceFiles/data/data_cloud_themes.h` | `TCG-N31` | `DataCloudThemes` | `crates/gpui_telegram_client/src/data/data_cloud_themes.rs` | 177 | `tests/test_tcg_n31_data_cloud_themes.rs` |
| `Telegram/SourceFiles/data/data_community.cpp` | `TCG-N04` | `DataCommunity` | `crates/gpui_telegram_client/src/data/data_community.rs` | 417 | `tests/test_tcg_n04_data_community.rs` |
| `Telegram/SourceFiles/data/data_community.h` | `TCG-N04` | `DataCommunity` | `crates/gpui_telegram_client/src/data/data_community.rs` | 116 | `tests/test_tcg_n04_data_community.rs` |
| `Telegram/SourceFiles/data/data_compose_stash.h` | `TCG-N04` | `DataComposeStash` | `crates/gpui_telegram_client/src/data/data_compose_stash.rs` | 29 | `tests/test_tcg_n04_data_compose_stash.rs` |
| `Telegram/SourceFiles/data/data_credits.h` | `TCG-N04` | `DataCredits` | `crates/gpui_telegram_client/src/data/data_credits.rs` | 170 | `tests/test_tcg_n04_data_credits.rs` |
| `Telegram/SourceFiles/data/data_credits_earn.h` | `TCG-N04` | `DataCreditsEarn` | `crates/gpui_telegram_client/src/data/data_credits_earn.rs` | 34 | `tests/test_tcg_n04_data_credits_earn.rs` |
| `Telegram/SourceFiles/data/data_document.cpp` | `TCG-N04` | `DataDocument` | `crates/gpui_telegram_client/src/data/data_document.rs` | 2046 | `tests/test_tcg_n04_data_document.rs` |
| `Telegram/SourceFiles/data/data_document.h` | `TCG-N04` | `DataDocument` | `crates/gpui_telegram_client/src/data/data_document.rs` | 447 | `tests/test_tcg_n04_data_document.rs` |
| `Telegram/SourceFiles/data/data_document_media.cpp` | `TCG-N04` | `DataDocumentMedia` | `crates/gpui_telegram_client/src/data/data_document_media.rs` | 587 | `tests/test_tcg_n04_data_document_media.rs` |
| `Telegram/SourceFiles/data/data_document_media.h` | `TCG-N04` | `DataDocumentMedia` | `crates/gpui_telegram_client/src/data/data_document_media.rs` | 126 | `tests/test_tcg_n04_data_document_media.rs` |
| `Telegram/SourceFiles/data/data_document_resolver.cpp` | `TCG-N04` | `DataDocumentResolver` | `crates/gpui_telegram_client/src/data/data_document_resolver.rs` | 310 | `tests/test_tcg_n04_data_document_resolver.rs` |
| `Telegram/SourceFiles/data/data_document_resolver.h` | `TCG-N04` | `DataDocumentResolver` | `crates/gpui_telegram_client/src/data/data_document_resolver.rs` | 71 | `tests/test_tcg_n04_data_document_resolver.rs` |
| `Telegram/SourceFiles/data/data_download_manager.cpp` | `TCG-N04` | `DataDownloadManager` | `crates/gpui_telegram_client/src/data/data_download_manager.rs` | 1421 | `tests/test_tcg_n04_data_download_manager.rs` |
| `Telegram/SourceFiles/data/data_download_manager.h` | `TCG-N04` | `DataDownloadManager` | `crates/gpui_telegram_client/src/data/data_download_manager.rs` | 238 | `tests/test_tcg_n04_data_download_manager.rs` |
| `Telegram/SourceFiles/data/data_drafts.cpp` | `TCG-N36` | `DataDrafts` | `crates/gpui_telegram_client/src/data/data_drafts.rs` | 221 | `tests/test_tcg_n36_data_drafts.rs` |
| `Telegram/SourceFiles/data/data_drafts.h` | `TCG-N36` | `DataDrafts` | `crates/gpui_telegram_client/src/data/data_drafts.rs` | 267 | `tests/test_tcg_n36_data_drafts.rs` |
| `Telegram/SourceFiles/data/data_emoji_statuses.cpp` | `TCG-N15` | `DataEmojiStatuses` | `crates/gpui_telegram_client/src/data/data_emoji_statuses.rs` | 572 | `tests/test_tcg_n15_data_emoji_statuses.rs` |
| `Telegram/SourceFiles/data/data_emoji_statuses.h` | `TCG-N15` | `DataEmojiStatuses` | `crates/gpui_telegram_client/src/data/data_emoji_statuses.rs` | 179 | `tests/test_tcg_n15_data_emoji_statuses.rs` |
| `Telegram/SourceFiles/data/data_file_click_handler.cpp` | `TCG-N04` | `DataFileClickHandler` | `crates/gpui_telegram_client/src/data/data_file_click_handler.rs` | 249 | `tests/test_tcg_n04_data_file_click_handler.rs` |
| `Telegram/SourceFiles/data/data_file_click_handler.h` | `TCG-N04` | `DataFileClickHandler` | `crates/gpui_telegram_client/src/data/data_file_click_handler.rs` | 189 | `tests/test_tcg_n04_data_file_click_handler.rs` |
| `Telegram/SourceFiles/data/data_file_origin.cpp` | `TCG-N04` | `DataFileOrigin` | `crates/gpui_telegram_client/src/data/data_file_origin.rs` | 381 | `tests/test_tcg_n04_data_file_origin.rs` |
| `Telegram/SourceFiles/data/data_file_origin.h` | `TCG-N04` | `DataFileOrigin` | `crates/gpui_telegram_client/src/data/data_file_origin.rs` | 250 | `tests/test_tcg_n04_data_file_origin.rs` |
| `Telegram/SourceFiles/data/data_flags.h` | `TCG-N04` | `DataFlags` | `crates/gpui_telegram_client/src/data/data_flags.rs` | 83 | `tests/test_tcg_n04_data_flags.rs` |
| `Telegram/SourceFiles/data/data_folder.cpp` | `TCG-N04` | `DataFolder` | `crates/gpui_telegram_client/src/data/data_folder.rs` | 426 | `tests/test_tcg_n04_data_folder.rs` |
| `Telegram/SourceFiles/data/data_folder.h` | `TCG-N04` | `DataFolder` | `crates/gpui_telegram_client/src/data/data_folder.rs` | 116 | `tests/test_tcg_n04_data_folder.rs` |
| `Telegram/SourceFiles/data/data_forum.cpp` | `TCG-N13` | `DataForum` | `crates/gpui_telegram_client/src/data/data_forum.rs` | 659 | `tests/test_tcg_n13_data_forum.rs` |
| `Telegram/SourceFiles/data/data_forum.h` | `TCG-N13` | `DataForum` | `crates/gpui_telegram_client/src/data/data_forum.rs` | 153 | `tests/test_tcg_n13_data_forum.rs` |
| `Telegram/SourceFiles/data/data_forum_icons.cpp` | `TCG-N13` | `DataForumIcons` | `crates/gpui_telegram_client/src/data/data_forum_icons.rs` | 119 | `tests/test_tcg_n13_data_forum_icons.rs` |
| `Telegram/SourceFiles/data/data_forum_icons.h` | `TCG-N13` | `DataForumIcons` | `crates/gpui_telegram_client/src/data/data_forum_icons.rs` | 63 | `tests/test_tcg_n13_data_forum_icons.rs` |
| `Telegram/SourceFiles/data/data_forum_topic.cpp` | `TCG-N13` | `DataForumTopic` | `crates/gpui_telegram_client/src/data/data_forum_topic.rs` | 1000 | `tests/test_tcg_n13_data_forum_topic.rs` |
| `Telegram/SourceFiles/data/data_forum_topic.h` | `TCG-N13` | `DataForumTopic` | `crates/gpui_telegram_client/src/data/data_forum_topic.rs` | 254 | `tests/test_tcg_n13_data_forum_topic.rs` |
| `Telegram/SourceFiles/data/data_game.cpp` | `TCG-N04` | `DataGame` | `crates/gpui_telegram_client/src/data/data_game.rs` | 13 | `tests/test_tcg_n04_data_game.rs` |
| `Telegram/SourceFiles/data/data_game.h` | `TCG-N04` | `DataGame` | `crates/gpui_telegram_client/src/data/data_game.rs` | 24 | `tests/test_tcg_n04_data_game.rs` |
| `Telegram/SourceFiles/data/data_group_call.cpp` | `TCG-N25` | `DataGroupCall` | `crates/gpui_telegram_client/src/data/data_group_call.rs` | 1159 | `tests/test_tcg_n25_data_group_call.rs` |
| `Telegram/SourceFiles/data/data_group_call.h` | `TCG-N25` | `DataGroupCall` | `crates/gpui_telegram_client/src/data/data_group_call.rs` | 328 | `tests/test_tcg_n25_data_group_call.rs` |
| `Telegram/SourceFiles/data/data_groups.cpp` | `TCG-N11` | `DataGroups` | `crates/gpui_telegram_client/src/data/data_groups.rs` | 177 | `tests/test_tcg_n11_data_groups.rs` |
| `Telegram/SourceFiles/data/data_groups.h` | `TCG-N11` | `DataGroups` | `crates/gpui_telegram_client/src/data/data_groups.rs` | 47 | `tests/test_tcg_n11_data_groups.rs` |
| `Telegram/SourceFiles/data/data_histories.cpp` | `TCG-N38` | `DataHistories` | `crates/gpui_telegram_client/src/data/data_histories.rs` | 1341 | `tests/test_tcg_n38_data_histories.rs` |
| `Telegram/SourceFiles/data/data_histories.h` | `TCG-N38` | `DataHistories` | `crates/gpui_telegram_client/src/data/data_histories.rs` | 265 | `tests/test_tcg_n38_data_histories.rs` |
| `Telegram/SourceFiles/data/data_history_messages.cpp` | `TCG-N04` | `DataHistoryMessages` | `crates/gpui_telegram_client/src/data/data_history_messages.rs` | 517 | `tests/test_tcg_n04_data_history_messages.rs` |
| `Telegram/SourceFiles/data/data_history_messages.h` | `TCG-N04` | `DataHistoryMessages` | `crates/gpui_telegram_client/src/data/data_history_messages.rs` | 71 | `tests/test_tcg_n04_data_history_messages.rs` |
| `Telegram/SourceFiles/data/data_lastseen_status.h` | `TCG-N04` | `DataLastseenStatus` | `crates/gpui_telegram_client/src/data/data_lastseen_status.rs` | 132 | `tests/test_tcg_n04_data_lastseen_status.rs` |
| `Telegram/SourceFiles/data/data_location.cpp` | `TCG-N41` | `DataLocation` | `crates/gpui_telegram_client/src/data/data_location.rs` | 86 | `tests/test_tcg_n41_data_location.rs` |
| `Telegram/SourceFiles/data/data_location.h` | `TCG-N41` | `DataLocation` | `crates/gpui_telegram_client/src/data/data_location.rs` | 84 | `tests/test_tcg_n41_data_location.rs` |
| `Telegram/SourceFiles/data/data_media_preload.cpp` | `TCG-N04` | `DataMediaPreload` | `crates/gpui_telegram_client/src/data/data_media_preload.rs` | 209 | `tests/test_tcg_n04_data_media_preload.rs` |
| `Telegram/SourceFiles/data/data_media_preload.h` | `TCG-N04` | `DataMediaPreload` | `crates/gpui_telegram_client/src/data/data_media_preload.rs` | 83 | `tests/test_tcg_n04_data_media_preload.rs` |
| `Telegram/SourceFiles/data/data_media_rotation.cpp` | `TCG-N04` | `DataMediaRotation` | `crates/gpui_telegram_client/src/data/data_media_rotation.rs` | 49 | `tests/test_tcg_n04_data_media_rotation.rs` |
| `Telegram/SourceFiles/data/data_media_rotation.h` | `TCG-N04` | `DataMediaRotation` | `crates/gpui_telegram_client/src/data/data_media_rotation.rs` | 29 | `tests/test_tcg_n04_data_media_rotation.rs` |
| `Telegram/SourceFiles/data/data_media_types.cpp` | `TCG-N04` | `DataMediaTypes` | `crates/gpui_telegram_client/src/data/data_media_types.rs` | 3229 | `tests/test_tcg_n04_data_media_types.rs` |
| `Telegram/SourceFiles/data/data_media_types.h` | `TCG-N04` | `DataMediaTypes` | `crates/gpui_telegram_client/src/data/data_media_types.rs` | 924 | `tests/test_tcg_n04_data_media_types.rs` |
| `Telegram/SourceFiles/data/data_message_reaction_id.cpp` | `TCG-N14` | `DataMessageReactionId` | `crates/gpui_telegram_client/src/data/data_message_reaction_id.rs` | 105 | `tests/test_tcg_n14_data_message_reaction_id.rs` |
| `Telegram/SourceFiles/data/data_message_reaction_id.h` | `TCG-N14` | `DataMessageReactionId` | `crates/gpui_telegram_client/src/data/data_message_reaction_id.rs` | 95 | `tests/test_tcg_n14_data_message_reaction_id.rs` |
| `Telegram/SourceFiles/data/data_message_reactions.cpp` | `TCG-N14` | `DataMessageReactions` | `crates/gpui_telegram_client/src/data/data_message_reactions.rs` | 2510 | `tests/test_tcg_n14_data_message_reactions.rs` |
| `Telegram/SourceFiles/data/data_message_reactions.h` | `TCG-N14` | `DataMessageReactions` | `crates/gpui_telegram_client/src/data/data_message_reactions.rs` | 462 | `tests/test_tcg_n14_data_message_reactions.rs` |
| `Telegram/SourceFiles/data/data_messages.cpp` | `TCG-N04` | `DataMessages` | `crates/gpui_telegram_client/src/data/data_messages.rs` | 531 | `tests/test_tcg_n04_data_messages.rs` |
| `Telegram/SourceFiles/data/data_messages.h` | `TCG-N04` | `DataMessages` | `crates/gpui_telegram_client/src/data/data_messages.rs` | 243 | `tests/test_tcg_n04_data_messages.rs` |
| `Telegram/SourceFiles/data/data_msg_id.h` | `TCG-N04` | `DataMsgId` | `crates/gpui_telegram_client/src/data/data_msg_id.rs` | 299 | `tests/test_tcg_n04_data_msg_id.rs` |
| `Telegram/SourceFiles/data/data_passkey_deserialize.cpp` | `TCG-N04` | `DataPasskeyDeserialize` | `crates/gpui_telegram_client/src/data/data_passkey_deserialize.rs` | 121 | `tests/test_tcg_n04_data_passkey_deserialize.rs` |
| `Telegram/SourceFiles/data/data_passkey_deserialize.h` | `TCG-N04` | `DataPasskeyDeserialize` | `crates/gpui_telegram_client/src/data/data_passkey_deserialize.rs` | 62 | `tests/test_tcg_n04_data_passkey_deserialize.rs` |
| `Telegram/SourceFiles/data/data_peer.cpp` | `TCG-N11` | `DataPeer` | `crates/gpui_telegram_client/src/data/data_peer.rs` | 2344 | `tests/test_tcg_n11_data_peer.rs` |
| `Telegram/SourceFiles/data/data_peer.h` | `TCG-N11` | `DataPeer` | `crates/gpui_telegram_client/src/data/data_peer.rs` | 708 | `tests/test_tcg_n11_data_peer.rs` |
| `Telegram/SourceFiles/data/data_peer_bot_command.cpp` | `TCG-N27` | `DataPeerBotCommand` | `crates/gpui_telegram_client/src/data/data_peer_bot_command.rs` | 22 | `tests/test_tcg_n27_data_peer_bot_command.rs` |
| `Telegram/SourceFiles/data/data_peer_bot_command.h` | `TCG-N27` | `DataPeerBotCommand` | `crates/gpui_telegram_client/src/data/data_peer_bot_command.rs` | 24 | `tests/test_tcg_n27_data_peer_bot_command.rs` |
| `Telegram/SourceFiles/data/data_peer_bot_commands.cpp` | `TCG-N27` | `DataPeerBotCommands` | `crates/gpui_telegram_client/src/data/data_peer_bot_commands.rs` | 53 | `tests/test_tcg_n27_data_peer_bot_commands.rs` |
| `Telegram/SourceFiles/data/data_peer_bot_commands.h` | `TCG-N27` | `DataPeerBotCommands` | `crates/gpui_telegram_client/src/data/data_peer_bot_commands.rs` | 38 | `tests/test_tcg_n27_data_peer_bot_commands.rs` |
| `Telegram/SourceFiles/data/data_peer_colors.h` | `TCG-N11` | `DataPeerColors` | `crates/gpui_telegram_client/src/data/data_peer_colors.rs` | 23 | `tests/test_tcg_n11_data_peer_colors.rs` |
| `Telegram/SourceFiles/data/data_peer_common.h` | `TCG-N11` | `DataPeerCommon` | `crates/gpui_telegram_client/src/data/data_peer_common.rs` | 34 | `tests/test_tcg_n11_data_peer_common.rs` |
| `Telegram/SourceFiles/data/data_peer_id.cpp` | `TCG-N11` | `DataPeerId` | `crates/gpui_telegram_client/src/data/data_peer_id.rs` | 58 | `tests/test_tcg_n11_data_peer_id.rs` |
| `Telegram/SourceFiles/data/data_peer_id.h` | `TCG-N11` | `DataPeerId` | `crates/gpui_telegram_client/src/data/data_peer_id.rs` | 255 | `tests/test_tcg_n11_data_peer_id.rs` |
| `Telegram/SourceFiles/data/data_peer_share_theme.h` | `TCG-N31` | `DataPeerShareTheme` | `crates/gpui_telegram_client/src/data/data_peer_share_theme.rs` | 48 | `tests/test_tcg_n31_data_peer_share_theme.rs` |
| `Telegram/SourceFiles/data/data_peer_values.cpp` | `TCG-N11` | `DataPeerValues` | `crates/gpui_telegram_client/src/data/data_peer_values.rs` | 694 | `tests/test_tcg_n11_data_peer_values.rs` |
| `Telegram/SourceFiles/data/data_peer_values.h` | `TCG-N11` | `DataPeerValues` | `crates/gpui_telegram_client/src/data/data_peer_values.rs` | 190 | `tests/test_tcg_n11_data_peer_values.rs` |
| `Telegram/SourceFiles/data/data_photo.cpp` | `TCG-N04` | `DataPhoto` | `crates/gpui_telegram_client/src/data/data_photo.rs` | 614 | `tests/test_tcg_n04_data_photo.rs` |
| `Telegram/SourceFiles/data/data_photo.h` | `TCG-N04` | `DataPhoto` | `crates/gpui_telegram_client/src/data/data_photo.rs` | 203 | `tests/test_tcg_n04_data_photo.rs` |
| `Telegram/SourceFiles/data/data_photo_media.cpp` | `TCG-N04` | `DataPhotoMedia` | `crates/gpui_telegram_client/src/data/data_photo_media.rs` | 245 | `tests/test_tcg_n04_data_photo_media.rs` |
| `Telegram/SourceFiles/data/data_photo_media.h` | `TCG-N04` | `DataPhotoMedia` | `crates/gpui_telegram_client/src/data/data_photo_media.rs` | 73 | `tests/test_tcg_n04_data_photo_media.rs` |
| `Telegram/SourceFiles/data/data_poll.cpp` | `TCG-N16` | `DataPoll` | `crates/gpui_telegram_client/src/data/data_poll.rs` | 764 | `tests/test_tcg_n16_data_poll.rs` |
| `Telegram/SourceFiles/data/data_poll.h` | `TCG-N16` | `DataPoll` | `crates/gpui_telegram_client/src/data/data_poll.rs` | 187 | `tests/test_tcg_n16_data_poll.rs` |
| `Telegram/SourceFiles/data/data_poll_messages.cpp` | `TCG-N16` | `DataPollMessages` | `crates/gpui_telegram_client/src/data/data_poll_messages.rs` | 69 | `tests/test_tcg_n16_data_poll_messages.rs` |
| `Telegram/SourceFiles/data/data_poll_messages.h` | `TCG-N16` | `DataPollMessages` | `crates/gpui_telegram_client/src/data/data_poll_messages.rs` | 30 | `tests/test_tcg_n16_data_poll_messages.rs` |
| `Telegram/SourceFiles/data/data_premium_limits.cpp` | `TCG-N04` | `DataPremiumLimits` | `crates/gpui_telegram_client/src/data/data_premium_limits.rs` | 318 | `tests/test_tcg_n04_data_premium_limits.rs` |
| `Telegram/SourceFiles/data/data_premium_limits.h` | `TCG-N04` | `DataPremiumLimits` | `crates/gpui_telegram_client/src/data/data_premium_limits.rs` | 125 | `tests/test_tcg_n04_data_premium_limits.rs` |
| `Telegram/SourceFiles/data/data_premium_subscription_option.h` | `TCG-N04` | `DataPremiumSubscriptionOption` | `crates/gpui_telegram_client/src/data/data_premium_subscription_option.rs` | 25 | `tests/test_tcg_n04_data_premium_subscription_option.rs` |
| `Telegram/SourceFiles/data/data_pts_waiter.cpp` | `TCG-N04` | `DataPtsWaiter` | `crates/gpui_telegram_client/src/data/data_pts_waiter.rs` | 187 | `tests/test_tcg_n04_data_pts_waiter.rs` |
| `Telegram/SourceFiles/data/data_pts_waiter.h` | `TCG-N04` | `DataPtsWaiter` | `crates/gpui_telegram_client/src/data/data_pts_waiter.rs` | 104 | `tests/test_tcg_n04_data_pts_waiter.rs` |
| `Telegram/SourceFiles/data/data_replies_list.cpp` | `TCG-N04` | `DataRepliesList` | `crates/gpui_telegram_client/src/data/data_replies_list.rs` | 1049 | `tests/test_tcg_n04_data_replies_list.rs` |
| `Telegram/SourceFiles/data/data_replies_list.h` | `TCG-N04` | `DataRepliesList` | `crates/gpui_telegram_client/src/data/data_replies_list.rs` | 139 | `tests/test_tcg_n04_data_replies_list.rs` |
| `Telegram/SourceFiles/data/data_reply_preview.cpp` | `TCG-N04` | `DataReplyPreview` | `crates/gpui_telegram_client/src/data/data_reply_preview.rs` | 146 | `tests/test_tcg_n04_data_reply_preview.rs` |
| `Telegram/SourceFiles/data/data_reply_preview.h` | `TCG-N04` | `DataReplyPreview` | `crates/gpui_telegram_client/src/data/data_reply_preview.rs` | 50 | `tests/test_tcg_n04_data_reply_preview.rs` |
| `Telegram/SourceFiles/data/data_report.h` | `TCG-N04` | `DataReport` | `crates/gpui_telegram_client/src/data/data_report.rs` | 24 | `tests/test_tcg_n04_data_report.rs` |
| `Telegram/SourceFiles/data/data_saved_messages.cpp` | `TCG-N04` | `DataSavedMessages` | `crates/gpui_telegram_client/src/data/data_saved_messages.rs` | 657 | `tests/test_tcg_n04_data_saved_messages.rs` |
| `Telegram/SourceFiles/data/data_saved_messages.h` | `TCG-N04` | `DataSavedMessages` | `crates/gpui_telegram_client/src/data/data_saved_messages.rs` | 153 | `tests/test_tcg_n04_data_saved_messages.rs` |
| `Telegram/SourceFiles/data/data_saved_music.cpp` | `TCG-N04` | `DataSavedMusic` | `crates/gpui_telegram_client/src/data/data_saved_music.rs` | 435 | `tests/test_tcg_n04_data_saved_music.rs` |
| `Telegram/SourceFiles/data/data_saved_music.h` | `TCG-N04` | `DataSavedMusic` | `crates/gpui_telegram_client/src/data/data_saved_music.rs` | 89 | `tests/test_tcg_n04_data_saved_music.rs` |
| `Telegram/SourceFiles/data/data_saved_sublist.cpp` | `TCG-N04` | `DataSavedSublist` | `crates/gpui_telegram_client/src/data/data_saved_sublist.rs` | 1324 | `tests/test_tcg_n04_data_saved_sublist.rs` |
| `Telegram/SourceFiles/data/data_saved_sublist.h` | `TCG-N04` | `DataSavedSublist` | `crates/gpui_telegram_client/src/data/data_saved_sublist.rs` | 201 | `tests/test_tcg_n04_data_saved_sublist.rs` |
| `Telegram/SourceFiles/data/data_search_calendar.cpp` | `TCG-N21` | `DataSearchCalendar` | `crates/gpui_telegram_client/src/data/data_search_calendar.rs` | 361 | `tests/test_tcg_n21_data_search_calendar.rs` |
| `Telegram/SourceFiles/data/data_search_calendar.h` | `TCG-N21` | `DataSearchCalendar` | `crates/gpui_telegram_client/src/data/data_search_calendar.rs` | 129 | `tests/test_tcg_n21_data_search_calendar.rs` |
| `Telegram/SourceFiles/data/data_search_controller.cpp` | `TCG-N21` | `DataSearchController` | `crates/gpui_telegram_client/src/data/data_search_controller.rs` | 606 | `tests/test_tcg_n21_data_search_controller.rs` |
| `Telegram/SourceFiles/data/data_search_controller.h` | `TCG-N21` | `DataSearchController` | `crates/gpui_telegram_client/src/data/data_search_controller.rs` | 221 | `tests/test_tcg_n21_data_search_controller.rs` |
| `Telegram/SourceFiles/data/data_send_action.cpp` | `TCG-N04` | `DataSendAction` | `crates/gpui_telegram_client/src/data/data_send_action.rs` | 153 | `tests/test_tcg_n04_data_send_action.rs` |
| `Telegram/SourceFiles/data/data_send_action.h` | `TCG-N04` | `DataSendAction` | `crates/gpui_telegram_client/src/data/data_send_action.rs` | 81 | `tests/test_tcg_n04_data_send_action.rs` |
| `Telegram/SourceFiles/data/data_session.cpp` | `TCG-N02` | `DataSession` | `crates/gpui_telegram_client/src/data/data_session.rs` | 6259 | `tests/test_tcg_n02_data_session.rs` |
| `Telegram/SourceFiles/data/data_session.h` | `TCG-N02` | `DataSession` | `crates/gpui_telegram_client/src/data/data_session.rs` | 1463 | `tests/test_tcg_n02_data_session.rs` |
| `Telegram/SourceFiles/data/data_shared_media.cpp` | `TCG-N04` | `DataSharedMedia` | `crates/gpui_telegram_client/src/data/data_shared_media.rs` | 593 | `tests/test_tcg_n04_data_shared_media.rs` |
| `Telegram/SourceFiles/data/data_shared_media.h` | `TCG-N04` | `DataSharedMedia` | `crates/gpui_telegram_client/src/data/data_shared_media.rs` | 220 | `tests/test_tcg_n04_data_shared_media.rs` |
| `Telegram/SourceFiles/data/data_sparse_ids.cpp` | `TCG-N04` | `DataSparseIds` | `crates/gpui_telegram_client/src/data/data_sparse_ids.rs` | 428 | `tests/test_tcg_n04_data_sparse_ids.rs` |
| `Telegram/SourceFiles/data/data_sparse_ids.h` | `TCG-N04` | `DataSparseIds` | `crates/gpui_telegram_client/src/data/data_sparse_ids.rs` | 203 | `tests/test_tcg_n04_data_sparse_ids.rs` |
| `Telegram/SourceFiles/data/data_star_gift.cpp` | `TCG-N15` | `DataStarGift` | `crates/gpui_telegram_client/src/data/data_star_gift.rs` | 416 | `tests/test_tcg_n15_data_star_gift.rs` |
| `Telegram/SourceFiles/data/data_star_gift.h` | `TCG-N15` | `DataStarGift` | `crates/gpui_telegram_client/src/data/data_star_gift.rs` | 432 | `tests/test_tcg_n15_data_star_gift.rs` |
| `Telegram/SourceFiles/data/data_statistics.h` | `TCG-N12` | `DataStatistics` | `crates/gpui_telegram_client/src/data/data_statistics.rs` | 104 | `tests/test_tcg_n12_data_statistics.rs` |
| `Telegram/SourceFiles/data/data_statistics_chart.cpp` | `TCG-N12` | `DataStatisticsChart` | `crates/gpui_telegram_client/src/data/data_statistics_chart.rs` | 166 | `tests/test_tcg_n12_data_statistics_chart.rs` |
| `Telegram/SourceFiles/data/data_statistics_chart.h` | `TCG-N12` | `DataStatisticsChart` | `crates/gpui_telegram_client/src/data/data_statistics_chart.rs` | 91 | `tests/test_tcg_n12_data_statistics_chart.rs` |
| `Telegram/SourceFiles/data/data_statistics_lists.h` | `TCG-N12` | `DataStatisticsLists` | `crates/gpui_telegram_client/src/data/data_statistics_lists.rs` | 67 | `tests/test_tcg_n12_data_statistics_lists.rs` |
| `Telegram/SourceFiles/data/data_stories.cpp` | `TCG-N38` | `DataStories` | `crates/gpui_telegram_client/src/data/data_stories.rs` | 2607 | `tests/test_tcg_n38_data_stories.rs` |
| `Telegram/SourceFiles/data/data_stories.h` | `TCG-N38` | `DataStories` | `crates/gpui_telegram_client/src/data/data_stories.rs` | 491 | `tests/test_tcg_n38_data_stories.rs` |
| `Telegram/SourceFiles/data/data_stories_ids.cpp` | `TCG-N38` | `DataStoriesIds` | `crates/gpui_telegram_client/src/data/data_stories_ids.rs` | 98 | `tests/test_tcg_n38_data_stories_ids.rs` |
| `Telegram/SourceFiles/data/data_stories_ids.h` | `TCG-N38` | `DataStoriesIds` | `crates/gpui_telegram_client/src/data/data_stories_ids.rs` | 28 | `tests/test_tcg_n38_data_stories_ids.rs` |
| `Telegram/SourceFiles/data/data_story.cpp` | `TCG-N04` | `DataStory` | `crates/gpui_telegram_client/src/data/data_story.rs` | 985 | `tests/test_tcg_n04_data_story.rs` |
| `Telegram/SourceFiles/data/data_story.h` | `TCG-N04` | `DataStory` | `crates/gpui_telegram_client/src/data/data_story.rs` | 333 | `tests/test_tcg_n04_data_story.rs` |
| `Telegram/SourceFiles/data/data_streaming.cpp` | `TCG-N52` | `DataStreaming` | `crates/gpui_telegram_client/src/data/data_streaming.rs` | 253 | `tests/test_tcg_n52_data_streaming.rs` |
| `Telegram/SourceFiles/data/data_streaming.h` | `TCG-N52` | `DataStreaming` | `crates/gpui_telegram_client/src/data/data_streaming.rs` | 102 | `tests/test_tcg_n52_data_streaming.rs` |
| `Telegram/SourceFiles/data/data_subscriptions.h` | `TCG-N04` | `DataSubscriptions` | `crates/gpui_telegram_client/src/data/data_subscriptions.rs` | 40 | `tests/test_tcg_n04_data_subscriptions.rs` |
| `Telegram/SourceFiles/data/data_thread.cpp` | `TCG-N04` | `DataThread` | `crates/gpui_telegram_client/src/data/data_thread.rs` | 248 | `tests/test_tcg_n04_data_thread.rs` |
| `Telegram/SourceFiles/data/data_thread.h` | `TCG-N04` | `DataThread` | `crates/gpui_telegram_client/src/data/data_thread.rs` | 151 | `tests/test_tcg_n04_data_thread.rs` |
| `Telegram/SourceFiles/data/data_todo_list.cpp` | `TCG-N04` | `DataTodoList` | `crates/gpui_telegram_client/src/data/data_todo_list.rs` | 223 | `tests/test_tcg_n04_data_todo_list.rs` |
| `Telegram/SourceFiles/data/data_todo_list.h` | `TCG-N04` | `DataTodoList` | `crates/gpui_telegram_client/src/data/data_todo_list.rs` | 80 | `tests/test_tcg_n04_data_todo_list.rs` |
| `Telegram/SourceFiles/data/data_types.cpp` | `TCG-N04` | `DataTypes` | `crates/gpui_telegram_client/src/data/data_types.rs` | 158 | `tests/test_tcg_n04_data_types.rs` |
| `Telegram/SourceFiles/data/data_types.h` | `TCG-N04` | `DataTypes` | `crates/gpui_telegram_client/src/data/data_types.rs` | 415 | `tests/test_tcg_n04_data_types.rs` |
| `Telegram/SourceFiles/data/data_unread_value.cpp` | `TCG-N04` | `DataUnreadValue` | `crates/gpui_telegram_client/src/data/data_unread_value.rs` | 68 | `tests/test_tcg_n04_data_unread_value.rs` |
| `Telegram/SourceFiles/data/data_unread_value.h` | `TCG-N04` | `DataUnreadValue` | `crates/gpui_telegram_client/src/data/data_unread_value.rs` | 30 | `tests/test_tcg_n04_data_unread_value.rs` |
| `Telegram/SourceFiles/data/data_user.cpp` | `TCG-N04` | `DataUser` | `crates/gpui_telegram_client/src/data/data_user.rs` | 1165 | `tests/test_tcg_n04_data_user.rs` |
| `Telegram/SourceFiles/data/data_user.h` | `TCG-N04` | `DataUser` | `crates/gpui_telegram_client/src/data/data_user.rs` | 391 | `tests/test_tcg_n04_data_user.rs` |
| `Telegram/SourceFiles/data/data_user_names.cpp` | `TCG-N04` | `DataUserNames` | `crates/gpui_telegram_client/src/data/data_user_names.rs` | 62 | `tests/test_tcg_n04_data_user_names.rs` |
| `Telegram/SourceFiles/data/data_user_names.h` | `TCG-N04` | `DataUserNames` | `crates/gpui_telegram_client/src/data/data_user_names.rs` | 38 | `tests/test_tcg_n04_data_user_names.rs` |
| `Telegram/SourceFiles/data/data_user_photos.cpp` | `TCG-N04` | `DataUserPhotos` | `crates/gpui_telegram_client/src/data/data_user_photos.rs` | 257 | `tests/test_tcg_n04_data_user_photos.rs` |
| `Telegram/SourceFiles/data/data_user_photos.h` | `TCG-N04` | `DataUserPhotos` | `crates/gpui_telegram_client/src/data/data_user_photos.rs` | 55 | `tests/test_tcg_n04_data_user_photos.rs` |
| `Telegram/SourceFiles/data/data_user_wallet_card.cpp` | `CAP-06` | `DataUserWalletCard` | `crates/gpui_telegram_client/src/data/data_user_wallet_card.rs` | 128 | `tests/test_cap_06_data_user_wallet_card.rs` |
| `Telegram/SourceFiles/data/data_user_wallet_card.h` | `CAP-06` | `DataUserWalletCard` | `crates/gpui_telegram_client/src/data/data_user_wallet_card.rs` | 51 | `tests/test_cap_06_data_user_wallet_card.rs` |
| `Telegram/SourceFiles/data/data_wall_paper.cpp` | `TCG-N04` | `DataWallPaper` | `crates/gpui_telegram_client/src/data/data_wall_paper.rs` | 819 | `tests/test_tcg_n04_data_wall_paper.rs` |
| `Telegram/SourceFiles/data/data_wall_paper.h` | `TCG-N04` | `DataWallPaper` | `crates/gpui_telegram_client/src/data/data_wall_paper.rs` | 158 | `tests/test_tcg_n04_data_wall_paper.rs` |
| `Telegram/SourceFiles/data/data_web_page.cpp` | `TCG-N04` | `DataWebPage` | `crates/gpui_telegram_client/src/data/data_web_page.rs` | 502 | `tests/test_tcg_n04_data_web_page.rs` |
| `Telegram/SourceFiles/data/data_web_page.h` | `TCG-N04` | `DataWebPage` | `crates/gpui_telegram_client/src/data/data_web_page.rs` | 174 | `tests/test_tcg_n04_data_web_page.rs` |
| `Telegram/SourceFiles/data/notify/data_notify_settings.cpp` | `TCG-N32` | `DataNotifySettingsNotify` | `crates/gpui_telegram_client/src/data/notify/data_notify_settings.rs` | 738 | `tests/test_tcg_n32_data_notify_settings.rs` |
| `Telegram/SourceFiles/data/notify/data_notify_settings.h` | `TCG-N32` | `DataNotifySettingsNotify` | `crates/gpui_telegram_client/src/data/notify/data_notify_settings.rs` | 171 | `tests/test_tcg_n32_data_notify_settings.rs` |
| `Telegram/SourceFiles/data/notify/data_peer_notify_settings.cpp` | `TCG-N32` | `DataPeerNotifySettingsNotify` | `crates/gpui_telegram_client/src/data/notify/data_peer_notify_settings.rs` | 298 | `tests/test_tcg_n32_data_peer_notify_settings.rs` |
| `Telegram/SourceFiles/data/notify/data_peer_notify_settings.h` | `TCG-N32` | `DataPeerNotifySettingsNotify` | `crates/gpui_telegram_client/src/data/notify/data_peer_notify_settings.rs` | 71 | `tests/test_tcg_n32_data_peer_notify_settings.rs` |
| `Telegram/SourceFiles/data/notify/data_peer_notify_volume.cpp` | `TCG-N32` | `DataPeerNotifyVolumeNotify` | `crates/gpui_telegram_client/src/data/notify/data_peer_notify_volume.rs` | 117 | `tests/test_tcg_n32_data_peer_notify_volume.rs` |
| `Telegram/SourceFiles/data/notify/data_peer_notify_volume.h` | `TCG-N32` | `DataPeerNotifyVolumeNotify` | `crates/gpui_telegram_client/src/data/notify/data_peer_notify_volume.rs` | 45 | `tests/test_tcg_n32_data_peer_notify_volume.rs` |
| `Telegram/SourceFiles/data/raw/raw_countries_bounds.cpp` | `TCG-N04` | `RawCountriesBoundsRaw` | `crates/gpui_telegram_client/src/data/raw/raw_countries_bounds.rs` | 193 | `tests/test_tcg_n04_raw_countries_bounds.rs` |
| `Telegram/SourceFiles/data/raw/raw_countries_bounds.h` | `TCG-N04` | `RawCountriesBoundsRaw` | `crates/gpui_telegram_client/src/data/raw/raw_countries_bounds.rs` | 23 | `tests/test_tcg_n04_raw_countries_bounds.rs` |
| `Telegram/SourceFiles/data/stickers/data_custom_emoji.cpp` | `TCG-N15` | `DataCustomEmojiStickers` | `crates/gpui_telegram_client/src/data/stickers/data_custom_emoji.rs` | 1122 | `tests/test_tcg_n15_data_custom_emoji.rs` |
| `Telegram/SourceFiles/data/stickers/data_custom_emoji.h` | `TCG-N15` | `DataCustomEmojiStickers` | `crates/gpui_telegram_client/src/data/stickers/data_custom_emoji.rs` | 214 | `tests/test_tcg_n15_data_custom_emoji.rs` |
| `Telegram/SourceFiles/data/stickers/data_stickers.cpp` | `TCG-N15` | `DataStickersStickers` | `crates/gpui_telegram_client/src/data/stickers/data_stickers.rs` | 1863 | `tests/test_tcg_n15_data_stickers.rs` |
| `Telegram/SourceFiles/data/stickers/data_stickers.h` | `TCG-N15` | `DataStickersStickers` | `crates/gpui_telegram_client/src/data/stickers/data_stickers.rs` | 335 | `tests/test_tcg_n15_data_stickers.rs` |
| `Telegram/SourceFiles/data/stickers/data_stickers_set.cpp` | `TCG-N15` | `DataStickersSetStickers` | `crates/gpui_telegram_client/src/data/stickers/data_stickers_set.rs` | 249 | `tests/test_tcg_n15_data_stickers_set.rs` |
| `Telegram/SourceFiles/data/stickers/data_stickers_set.h` | `TCG-N15` | `DataStickersSetStickers` | `crates/gpui_telegram_client/src/data/stickers/data_stickers_set.rs` | 138 | `tests/test_tcg_n15_data_stickers_set.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/data/business/data_business_chatbots.cpp`
- **Classes & Structs**: Flag, PermissionLabel
- **Core Methods & Signals**: Chatbots::Chatbots, Chatbots::~Chatbots, Chatbots::preload, Chatbots::loaded, Chatbots::current, Chatbots::changes
- **Target**: `DataBusinessChatbotsBusiness` in `crates/gpui_telegram_client/src/data/business/data_business_chatbots.rs`

### `Telegram/SourceFiles/data/business/data_business_chatbots.h`
- **Classes & Structs**: UserData, EditFlagsDescriptor, Session, ChatbotsSettings, Chatbots, SentRequestType, SentRequest
- **Core Methods & Signals**: Chatbots, preload, togglePaused, removeFrom, reload, rpl::changes
- **Target**: `DataBusinessChatbotsBusiness` in `crates/gpui_telegram_client/src/data/business/data_business_chatbots.rs`

### `Telegram/SourceFiles/data/business/data_business_common.cpp`
- **Classes & Structs**: Type, Flag, Right
- **Core Methods & Signals**: ranges::sort, BusinessRecipients::MakeValid, views::transform, views::all, WorkingIntervals::normalized, ranges::remove_if
- **Target**: `DataBusinessCommonBusiness` in `crates/gpui_telegram_client/src/data/business/data_business_common.rs`

### `Telegram/SourceFiles/data/business/data_business_common.h`
- **Classes & Structs**: UserData, Session, BusinessChatType, BusinessChats, BusinessRecipients, BusinessRecipientsType, ChatbotsPermission, Timezone, Timezones, WorkingInterval, WorkingIntervals, WorkingHours
- **Core Methods & Signals**: is_flag_type, bool
- **Target**: `DataBusinessCommonBusiness` in `crates/gpui_telegram_client/src/data/business/data_business_common.rs`

### `Telegram/SourceFiles/data/business/data_business_info.cpp`
- **Classes & Structs**: Flag
- **Core Methods & Signals**: views::transform, BusinessInfo::BusinessInfo, BusinessInfo::~BusinessInfo, BusinessInfo::saveWorkingHours, BusinessInfo::saveChatIntro, MTPDinputBusinessIntro::Flag
- **Target**: `DataBusinessInfoBusiness` in `crates/gpui_telegram_client/src/data/business/data_business_info.rs`

### `Telegram/SourceFiles/data/business/data_business_info.h`
- **Classes & Structs**: Session, BusinessInfo
- **Core Methods & Signals**: BusinessInfo, preload, applyAwaySettings, applyGreetingSettings, preloadTimezones, rpl::awaySettingsChanged
- **Target**: `DataBusinessInfoBusiness` in `crates/gpui_telegram_client/src/data/business/data_business_info.rs`

### `Telegram/SourceFiles/data/business/data_shortcut_messages.h`
- **Classes & Structs**: History, Session, MessagesSlice, Shortcut, ShortcutIdChange, Shortcuts, ShortcutMessages, List, Request, OwnedItem
- **Core Methods & Signals**: ShortcutMessages, apply, appendSending, removeSending, preloadShortcuts, removeShortcut
- **Target**: `DataShortcutMessagesBusiness` in `crates/gpui_telegram_client/src/data/business/data_shortcut_messages.rs`

### `Telegram/SourceFiles/data/components/credits.h`
- **Classes & Structs**: Session, Credits
- **Core Methods & Signals**: Credits, load, tonLoad, apply, applyCurrency, lock
- **Target**: `CreditsComponents` in `crates/gpui_telegram_client/src/data/components/credits.rs`

### `Telegram/SourceFiles/data/components/ephemeral_messages.cpp`
- **Classes & Structs**: ParsedCommand, Flag
- **Core Methods & Signals**: Api::EntitiesFromMTP, HistoryItem::CreateMedia, Iv::ParseRichPage, EphemeralMessages::EphemeralMessages, EphemeralMessages::~EphemeralMessages, EphemeralMessages::clear
- **Target**: `EphemeralMessagesComponents` in `crates/gpui_telegram_client/src/data/components/ephemeral_messages.rs`

### `Telegram/SourceFiles/data/components/ephemeral_messages.h`
- **Classes & Structs**: History, HistoryItem, PeerData, UserData, MessageToSend, SendAction, Session, EphemeralMessages, Entry, List
- **Core Methods & Signals**: EphemeralMessages, clear, apply, send, noteCallbackTopic, revertAnchored
- **Target**: `EphemeralMessagesComponents` in `crates/gpui_telegram_client/src/data/components/ephemeral_messages.rs`

