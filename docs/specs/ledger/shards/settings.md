# Subsystem Shard: `settings`

**Total Source Files**: 121  
**Total Source Lines**: 51,488  
**Primary Feature IDs**: TCG-N01, TCG-N02, TCG-N11, TCG-N14  
**JSON Shard Path**: `docs/specs/ledger/shards/settings.json`  

## Component Inventory & Parity Mapping

| Source Path | Feature ID | Target Symbol | Target Rust / GPUI Path | Lines | Test ID |
|---|---|---|---|---|---|
| `Telegram/SourceFiles/settings/business/settings_away_message.cpp` | `TCG-N31` | `SettingsAwayMessageBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_away_message.rs` | 376 | `tests/test_tcg_n31_settings_away_message.rs` |
| `Telegram/SourceFiles/settings/business/settings_away_message.h` | `TCG-N31` | `SettingsAwayMessageBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_away_message.rs` | 16 | `tests/test_tcg_n31_settings_away_message.rs` |
| `Telegram/SourceFiles/settings/business/settings_chat_intro.cpp` | `TCG-N01` | `SettingsChatIntroBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_chat_intro.rs` | 646 | `tests/test_tcg_n01_settings_chat_intro.rs` |
| `Telegram/SourceFiles/settings/business/settings_chat_intro.h` | `TCG-N01` | `SettingsChatIntroBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_chat_intro.rs` | 16 | `tests/test_tcg_n01_settings_chat_intro.rs` |
| `Telegram/SourceFiles/settings/business/settings_chat_links.cpp` | `TCG-N11` | `SettingsChatLinksBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_chat_links.rs` | 831 | `tests/test_tcg_n11_settings_chat_links.rs` |
| `Telegram/SourceFiles/settings/business/settings_chat_links.h` | `TCG-N11` | `SettingsChatLinksBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_chat_links.rs` | 16 | `tests/test_tcg_n11_settings_chat_links.rs` |
| `Telegram/SourceFiles/settings/business/settings_chatbots.cpp` | `TCG-N27` | `SettingsChatbotsBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_chatbots.rs` | 898 | `tests/test_tcg_n27_settings_chatbots.rs` |
| `Telegram/SourceFiles/settings/business/settings_chatbots.h` | `TCG-N27` | `SettingsChatbotsBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_chatbots.rs` | 16 | `tests/test_tcg_n27_settings_chatbots.rs` |
| `Telegram/SourceFiles/settings/business/settings_greeting.cpp` | `TCG-N31` | `SettingsGreetingBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_greeting.rs` | 293 | `tests/test_tcg_n31_settings_greeting.rs` |
| `Telegram/SourceFiles/settings/business/settings_greeting.h` | `TCG-N31` | `SettingsGreetingBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_greeting.rs` | 16 | `tests/test_tcg_n31_settings_greeting.rs` |
| `Telegram/SourceFiles/settings/business/settings_location.cpp` | `TCG-N41` | `SettingsLocationBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_location.rs` | 300 | `tests/test_tcg_n41_settings_location.rs` |
| `Telegram/SourceFiles/settings/business/settings_location.h` | `TCG-N41` | `SettingsLocationBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_location.rs` | 16 | `tests/test_tcg_n41_settings_location.rs` |
| `Telegram/SourceFiles/settings/business/settings_quick_replies.cpp` | `TCG-N31` | `SettingsQuickRepliesBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_quick_replies.rs` | 236 | `tests/test_tcg_n31_settings_quick_replies.rs` |
| `Telegram/SourceFiles/settings/business/settings_quick_replies.h` | `TCG-N31` | `SettingsQuickRepliesBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_quick_replies.rs` | 25 | `tests/test_tcg_n31_settings_quick_replies.rs` |
| `Telegram/SourceFiles/settings/business/settings_recipients_helper.cpp` | `TCG-N31` | `SettingsRecipientsHelperBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_recipients_helper.rs` | 436 | `tests/test_tcg_n31_settings_recipients_helper.rs` |
| `Telegram/SourceFiles/settings/business/settings_recipients_helper.h` | `TCG-N31` | `SettingsRecipientsHelperBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_recipients_helper.rs` | 76 | `tests/test_tcg_n31_settings_recipients_helper.rs` |
| `Telegram/SourceFiles/settings/business/settings_shortcut_messages.cpp` | `TCG-N45` | `SettingsShortcutMessagesBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_shortcut_messages.rs` | 1595 | `tests/test_tcg_n45_settings_shortcut_messages.rs` |
| `Telegram/SourceFiles/settings/business/settings_shortcut_messages.h` | `TCG-N45` | `SettingsShortcutMessagesBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_shortcut_messages.rs` | 16 | `tests/test_tcg_n45_settings_shortcut_messages.rs` |
| `Telegram/SourceFiles/settings/business/settings_working_hours.cpp` | `TCG-N31` | `SettingsWorkingHoursBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_working_hours.rs` | 726 | `tests/test_tcg_n31_settings_working_hours.rs` |
| `Telegram/SourceFiles/settings/business/settings_working_hours.h` | `TCG-N31` | `SettingsWorkingHoursBusiness` | `crates/gpui_telegram_desktop/src/views/settings/business/settings_working_hours.rs` | 16 | `tests/test_tcg_n31_settings_working_hours.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_common.cpp` | `TCG-N02` | `SettingsCloudPasswordCommonCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_common.rs` | 423 | `tests/test_tcg_n02_settings_cloud_password_common.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_common.h` | `TCG-N02` | `SettingsCloudPasswordCommonCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_common.rs` | 147 | `tests/test_tcg_n02_settings_cloud_password_common.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_email.cpp` | `TCG-N02` | `SettingsCloudPasswordEmailCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_email.rs` | 210 | `tests/test_tcg_n02_settings_cloud_password_email.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_email.h` | `TCG-N02` | `SettingsCloudPasswordEmailCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_email.rs` | 17 | `tests/test_tcg_n02_settings_cloud_password_email.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_email_confirm.cpp` | `TCG-N02` | `SettingsCloudPasswordEmailConfirmCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_email_confirm.rs` | 369 | `tests/test_tcg_n02_settings_cloud_password_email_confirm.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_email_confirm.h` | `TCG-N02` | `SettingsCloudPasswordEmailConfirmCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_email_confirm.rs` | 17 | `tests/test_tcg_n02_settings_cloud_password_email_confirm.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_hint.cpp` | `TCG-N02` | `SettingsCloudPasswordHintCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_hint.rs` | 159 | `tests/test_tcg_n02_settings_cloud_password_hint.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_hint.h` | `TCG-N02` | `SettingsCloudPasswordHintCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_hint.rs` | 17 | `tests/test_tcg_n02_settings_cloud_password_hint.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_input.cpp` | `TCG-N02` | `SettingsCloudPasswordInputCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_input.rs` | 589 | `tests/test_tcg_n02_settings_cloud_password_input.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_input.h` | `TCG-N02` | `SettingsCloudPasswordInputCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_input.rs` | 18 | `tests/test_tcg_n02_settings_cloud_password_input.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_login_email.cpp` | `TCG-N02` | `SettingsCloudPasswordLoginEmailCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_login_email.rs` | 177 | `tests/test_tcg_n02_settings_cloud_password_login_email.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_login_email.h` | `TCG-N02` | `SettingsCloudPasswordLoginEmailCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_login_email.rs` | 17 | `tests/test_tcg_n02_settings_cloud_password_login_email.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_login_email_confirm.cpp` | `TCG-N02` | `SettingsCloudPasswordLoginEmailConfirmCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_login_email_confirm.rs` | 176 | `tests/test_tcg_n02_settings_cloud_password_login_email_confirm.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_login_email_confirm.h` | `TCG-N02` | `SettingsCloudPasswordLoginEmailConfirmCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_login_email_confirm.rs` | 17 | `tests/test_tcg_n02_settings_cloud_password_login_email_confirm.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_manage.cpp` | `TCG-N02` | `SettingsCloudPasswordManageCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_manage.rs` | 253 | `tests/test_tcg_n02_settings_cloud_password_manage.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_manage.h` | `TCG-N02` | `SettingsCloudPasswordManageCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_manage.rs` | 17 | `tests/test_tcg_n02_settings_cloud_password_manage.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_start.cpp` | `TCG-N02` | `SettingsCloudPasswordStartCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_start.rs` | 62 | `tests/test_tcg_n02_settings_cloud_password_start.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_start.h` | `TCG-N02` | `SettingsCloudPasswordStartCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_start.rs` | 17 | `tests/test_tcg_n02_settings_cloud_password_start.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_step.cpp` | `TCG-N02` | `SettingsCloudPasswordStepCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_step.rs` | 121 | `tests/test_tcg_n02_settings_cloud_password_step.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_step.h` | `TCG-N02` | `SettingsCloudPasswordStepCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_step.rs` | 86 | `tests/test_tcg_n02_settings_cloud_password_step.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_validate_icon.cpp` | `TCG-N02` | `SettingsCloudPasswordValidateIconCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_validate_icon.rs` | 80 | `tests/test_tcg_n02_settings_cloud_password_validate_icon.rs` |
| `Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_validate_icon.h` | `TCG-N02` | `SettingsCloudPasswordValidateIconCloudPassword` | `crates/gpui_telegram_desktop/src/views/settings/cloud_password/settings_cloud_password_validate_icon.rs` | 27 | `tests/test_tcg_n02_settings_cloud_password_validate_icon.rs` |
| `Telegram/SourceFiles/settings/detailed_settings_button.cpp` | `TCG-N31` | `DetailedSettingsButton` | `crates/gpui_telegram_desktop/src/views/settings/detailed_settings_button.rs` | 306 | `tests/test_tcg_n31_detailed_settings_button.rs` |
| `Telegram/SourceFiles/settings/detailed_settings_button.h` | `TCG-N31` | `DetailedSettingsButton` | `crates/gpui_telegram_desktop/src/views/settings/detailed_settings_button.rs` | 78 | `tests/test_tcg_n31_detailed_settings_button.rs` |
| `Telegram/SourceFiles/settings/sections/settings_active_sessions.cpp` | `TCG-N02` | `SettingsActiveSessionsSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_active_sessions.rs` | 1364 | `tests/test_tcg_n02_settings_active_sessions.rs` |
| `Telegram/SourceFiles/settings/sections/settings_active_sessions.h` | `TCG-N02` | `SettingsActiveSessionsSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_active_sessions.rs` | 30 | `tests/test_tcg_n02_settings_active_sessions.rs` |
| `Telegram/SourceFiles/settings/sections/settings_active_sessions.style` | `TCG-N02` | `SettingsActiveSessionsSectionsStyleSheet` | `crates/gpui_telegram_desktop/src/theme/settings/settings_active_sessions_style.rs` | 30 | `tests/test_tcg_n02_settings_active_sessions.rs` |
| `Telegram/SourceFiles/settings/sections/settings_advanced.cpp` | `TCG-N31` | `SettingsAdvancedSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_advanced.rs` | 2058 | `tests/test_tcg_n31_settings_advanced.rs` |
| `Telegram/SourceFiles/settings/sections/settings_advanced.h` | `TCG-N31` | `SettingsAdvancedSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_advanced.rs` | 52 | `tests/test_tcg_n31_settings_advanced.rs` |
| `Telegram/SourceFiles/settings/sections/settings_blocked_peers.cpp` | `TCG-N11` | `SettingsBlockedPeersSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_blocked_peers.rs` | 335 | `tests/test_tcg_n11_settings_blocked_peers.rs` |
| `Telegram/SourceFiles/settings/sections/settings_blocked_peers.h` | `TCG-N11` | `SettingsBlockedPeersSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_blocked_peers.rs` | 20 | `tests/test_tcg_n11_settings_blocked_peers.rs` |
| `Telegram/SourceFiles/settings/sections/settings_business.cpp` | `TCG-N31` | `SettingsBusinessSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_business.rs` | 1041 | `tests/test_tcg_n31_settings_business.rs` |
| `Telegram/SourceFiles/settings/sections/settings_business.h` | `TCG-N31` | `SettingsBusinessSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_business.rs` | 32 | `tests/test_tcg_n31_settings_business.rs` |
| `Telegram/SourceFiles/settings/sections/settings_calls.cpp` | `TCG-N24` | `SettingsCallsSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_calls.rs` | 916 | `tests/test_tcg_n24_settings_calls.rs` |
| `Telegram/SourceFiles/settings/sections/settings_calls.h` | `TCG-N24` | `SettingsCallsSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_calls.rs` | 65 | `tests/test_tcg_n24_settings_calls.rs` |
| `Telegram/SourceFiles/settings/sections/settings_chat.cpp` | `TCG-N11` | `SettingsChatSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_chat.rs` | 2578 | `tests/test_tcg_n11_settings_chat.rs` |
| `Telegram/SourceFiles/settings/sections/settings_chat.h` | `TCG-N11` | `SettingsChatSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_chat.rs` | 80 | `tests/test_tcg_n11_settings_chat.rs` |
| `Telegram/SourceFiles/settings/sections/settings_credits.cpp` | `TCG-N31` | `SettingsCreditsSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_credits.rs` | 1172 | `tests/test_tcg_n31_settings_credits.rs` |
| `Telegram/SourceFiles/settings/sections/settings_credits.h` | `TCG-N31` | `SettingsCreditsSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_credits.rs` | 43 | `tests/test_tcg_n31_settings_credits.rs` |
| `Telegram/SourceFiles/settings/sections/settings_folders.cpp` | `TCG-N31` | `SettingsFoldersSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_folders.rs` | 1340 | `tests/test_tcg_n31_settings_folders.rs` |
| `Telegram/SourceFiles/settings/sections/settings_folders.h` | `TCG-N31` | `SettingsFoldersSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_folders.rs` | 20 | `tests/test_tcg_n31_settings_folders.rs` |
| `Telegram/SourceFiles/settings/sections/settings_global_ttl.cpp` | `TCG-N40` | `SettingsGlobalTtlSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_global_ttl.rs` | 584 | `tests/test_tcg_n40_settings_global_ttl.rs` |
| `Telegram/SourceFiles/settings/sections/settings_global_ttl.h` | `TCG-N40` | `SettingsGlobalTtlSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_global_ttl.rs` | 20 | `tests/test_tcg_n40_settings_global_ttl.rs` |
| `Telegram/SourceFiles/settings/sections/settings_information.cpp` | `TCG-N31` | `SettingsInformationSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_information.rs` | 1571 | `tests/test_tcg_n31_settings_information.rs` |
| `Telegram/SourceFiles/settings/sections/settings_information.h` | `TCG-N31` | `SettingsInformationSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_information.rs` | 62 | `tests/test_tcg_n31_settings_information.rs` |
| `Telegram/SourceFiles/settings/sections/settings_local_passcode.cpp` | `TCG-N02` | `SettingsLocalPasscodeSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_local_passcode.rs` | 1714 | `tests/test_tcg_n02_settings_local_passcode.rs` |
| `Telegram/SourceFiles/settings/sections/settings_local_passcode.h` | `TCG-N02` | `SettingsLocalPasscodeSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_local_passcode.rs` | 18 | `tests/test_tcg_n02_settings_local_passcode.rs` |
| `Telegram/SourceFiles/settings/sections/settings_local_storage.cpp` | `TCG-N34` | `SettingsLocalStorageSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_local_storage.rs` | 2118 | `tests/test_tcg_n34_settings_local_storage.rs` |
| `Telegram/SourceFiles/settings/sections/settings_local_storage.h` | `TCG-N34` | `SettingsLocalStorageSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_local_storage.rs` | 154 | `tests/test_tcg_n34_settings_local_storage.rs` |
| `Telegram/SourceFiles/settings/sections/settings_local_storage.style` | `TCG-N34` | `SettingsLocalStorageSectionsStyleSheet` | `crates/gpui_telegram_desktop/src/theme/settings/settings_local_storage_style.rs` | 89 | `tests/test_tcg_n34_settings_local_storage.rs` |
| `Telegram/SourceFiles/settings/sections/settings_main.cpp` | `TCG-N31` | `SettingsMainSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_main.rs` | 1259 | `tests/test_tcg_n31_settings_main.rs` |
| `Telegram/SourceFiles/settings/sections/settings_main.h` | `TCG-N31` | `SettingsMainSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_main.rs` | 46 | `tests/test_tcg_n31_settings_main.rs` |
| `Telegram/SourceFiles/settings/sections/settings_notifications.cpp` | `TCG-N31` | `SettingsNotificationsSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_notifications.rs` | 1656 | `tests/test_tcg_n31_settings_notifications.rs` |
| `Telegram/SourceFiles/settings/sections/settings_notifications.h` | `TCG-N31` | `SettingsNotificationsSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_notifications.rs` | 16 | `tests/test_tcg_n31_settings_notifications.rs` |
| `Telegram/SourceFiles/settings/sections/settings_notifications.style` | `TCG-N31` | `SettingsNotificationsSectionsStyleSheet` | `crates/gpui_telegram_desktop/src/theme/settings/settings_notifications_style.rs` | 18 | `tests/test_tcg_n31_settings_notifications.rs` |
| `Telegram/SourceFiles/settings/sections/settings_notifications_reactions.cpp` | `TCG-N14` | `SettingsNotificationsReactionsSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_notifications_reactions.rs` | 291 | `tests/test_tcg_n14_settings_notifications_reactions.rs` |
| `Telegram/SourceFiles/settings/sections/settings_notifications_reactions.h` | `TCG-N14` | `SettingsNotificationsReactionsSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_notifications_reactions.rs` | 27 | `tests/test_tcg_n14_settings_notifications_reactions.rs` |
| `Telegram/SourceFiles/settings/sections/settings_notifications_type.cpp` | `TCG-N31` | `SettingsNotificationsTypeSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_notifications_type.rs` | 827 | `tests/test_tcg_n31_settings_notifications_type.rs` |
| `Telegram/SourceFiles/settings/sections/settings_notifications_type.h` | `TCG-N31` | `SettingsNotificationsTypeSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_notifications_type.rs` | 47 | `tests/test_tcg_n31_settings_notifications_type.rs` |
| `Telegram/SourceFiles/settings/sections/settings_passkeys.cpp` | `TCG-N31` | `SettingsPasskeysSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_passkeys.rs` | 536 | `tests/test_tcg_n31_settings_passkeys.rs` |
| `Telegram/SourceFiles/settings/sections/settings_passkeys.h` | `TCG-N31` | `SettingsPasskeysSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_passkeys.rs` | 28 | `tests/test_tcg_n31_settings_passkeys.rs` |
| `Telegram/SourceFiles/settings/sections/settings_premium.cpp` | `TCG-N31` | `SettingsPremiumSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_premium.rs` | 2180 | `tests/test_tcg_n31_settings_premium.rs` |
| `Telegram/SourceFiles/settings/sections/settings_premium.h` | `TCG-N31` | `SettingsPremiumSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_premium.rs` | 113 | `tests/test_tcg_n31_settings_premium.rs` |
| `Telegram/SourceFiles/settings/sections/settings_premium.style` | `TCG-N31` | `SettingsPremiumSectionsStyleSheet` | `crates/gpui_telegram_desktop/src/theme/settings/settings_premium_style.rs` | 39 | `tests/test_tcg_n31_settings_premium.rs` |
| `Telegram/SourceFiles/settings/sections/settings_privacy_security.cpp` | `TCG-N33` | `SettingsPrivacySecuritySections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_privacy_security.rs` | 1199 | `tests/test_tcg_n33_settings_privacy_security.rs` |
| `Telegram/SourceFiles/settings/sections/settings_privacy_security.h` | `TCG-N33` | `SettingsPrivacySecuritySections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_privacy_security.rs` | 60 | `tests/test_tcg_n33_settings_privacy_security.rs` |
| `Telegram/SourceFiles/settings/sections/settings_shortcuts.cpp` | `TCG-N45` | `SettingsShortcutsSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_shortcuts.rs` | 868 | `tests/test_tcg_n45_settings_shortcuts.rs` |
| `Telegram/SourceFiles/settings/sections/settings_shortcuts.h` | `TCG-N45` | `SettingsShortcutsSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_shortcuts.rs` | 25 | `tests/test_tcg_n45_settings_shortcuts.rs` |
| `Telegram/SourceFiles/settings/sections/settings_websites.cpp` | `TCG-N31` | `SettingsWebsitesSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_websites.rs` | 862 | `tests/test_tcg_n31_settings_websites.rs` |
| `Telegram/SourceFiles/settings/sections/settings_websites.h` | `TCG-N31` | `SettingsWebsitesSections` | `crates/gpui_telegram_desktop/src/views/settings/sections/settings_websites.rs` | 20 | `tests/test_tcg_n31_settings_websites.rs` |
| `Telegram/SourceFiles/settings/settings.style` | `TCG-N31` | `SettingsStyleSheet` | `crates/gpui_telegram_desktop/src/theme/settings/settings_style.rs` | 772 | `tests/test_tcg_n31_settings.rs` |
| `Telegram/SourceFiles/settings/settings_builder.cpp` | `TCG-N31` | `SettingsBuilder` | `crates/gpui_telegram_desktop/src/views/settings/settings_builder.rs` | 494 | `tests/test_tcg_n31_settings_builder.rs` |
| `Telegram/SourceFiles/settings/settings_builder.h` | `TCG-N31` | `SettingsBuilder` | `crates/gpui_telegram_desktop/src/views/settings/settings_builder.rs` | 268 | `tests/test_tcg_n31_settings_builder.rs` |
| `Telegram/SourceFiles/settings/settings_codes.cpp` | `TCG-N31` | `SettingsCodes` | `crates/gpui_telegram_desktop/src/views/settings/settings_codes.rs` | 327 | `tests/test_tcg_n31_settings_codes.rs` |
| `Telegram/SourceFiles/settings/settings_codes.h` | `TCG-N31` | `SettingsCodes` | `crates/gpui_telegram_desktop/src/views/settings/settings_codes.rs` | 18 | `tests/test_tcg_n31_settings_codes.rs` |
| `Telegram/SourceFiles/settings/settings_common.cpp` | `TCG-N31` | `SettingsCommon` | `crates/gpui_telegram_desktop/src/views/settings/settings_common.rs` | 882 | `tests/test_tcg_n31_settings_common.rs` |
| `Telegram/SourceFiles/settings/settings_common.h` | `TCG-N31` | `SettingsCommon` | `crates/gpui_telegram_desktop/src/views/settings/settings_common.rs` | 360 | `tests/test_tcg_n31_settings_common.rs` |
| `Telegram/SourceFiles/settings/settings_common_session.cpp` | `TCG-N02` | `SettingsCommonSession` | `crates/gpui_telegram_desktop/src/views/settings/settings_common_session.rs` | 24 | `tests/test_tcg_n02_settings_common_session.rs` |
| `Telegram/SourceFiles/settings/settings_common_session.h` | `TCG-N02` | `SettingsCommonSession` | `crates/gpui_telegram_desktop/src/views/settings/settings_common_session.rs` | 84 | `tests/test_tcg_n02_settings_common_session.rs` |
| `Telegram/SourceFiles/settings/settings_credits_graphics.cpp` | `TCG-N31` | `SettingsCreditsGraphics` | `crates/gpui_telegram_desktop/src/views/settings/settings_credits_graphics.rs` | 3928 | `tests/test_tcg_n31_settings_credits_graphics.rs` |
| `Telegram/SourceFiles/settings/settings_credits_graphics.h` | `TCG-N31` | `SettingsCreditsGraphics` | `crates/gpui_telegram_desktop/src/views/settings/settings_credits_graphics.rs` | 345 | `tests/test_tcg_n31_settings_credits_graphics.rs` |
| `Telegram/SourceFiles/settings/settings_experimental.cpp` | `TCG-N31` | `SettingsExperimental` | `crates/gpui_telegram_desktop/src/views/settings/settings_experimental.rs` | 640 | `tests/test_tcg_n31_settings_experimental.rs` |
| `Telegram/SourceFiles/settings/settings_experimental.h` | `TCG-N31` | `SettingsExperimental` | `crates/gpui_telegram_desktop/src/views/settings/settings_experimental.rs` | 44 | `tests/test_tcg_n31_settings_experimental.rs` |
| `Telegram/SourceFiles/settings/settings_faq_suggestions.cpp` | `TCG-N31` | `SettingsFaqSuggestions` | `crates/gpui_telegram_desktop/src/views/settings/settings_faq_suggestions.rs` | 161 | `tests/test_tcg_n31_settings_faq_suggestions.rs` |
| `Telegram/SourceFiles/settings/settings_faq_suggestions.h` | `TCG-N31` | `SettingsFaqSuggestions` | `crates/gpui_telegram_desktop/src/views/settings/settings_faq_suggestions.rs` | 43 | `tests/test_tcg_n31_settings_faq_suggestions.rs` |
| `Telegram/SourceFiles/settings/settings_intro.cpp` | `TCG-N01` | `SettingsIntro` | `crates/gpui_telegram_desktop/src/views/settings/settings_intro.rs` | 562 | `tests/test_tcg_n01_settings_intro.rs` |
| `Telegram/SourceFiles/settings/settings_intro.h` | `TCG-N01` | `SettingsIntro` | `crates/gpui_telegram_desktop/src/views/settings/settings_intro.rs` | 55 | `tests/test_tcg_n01_settings_intro.rs` |
| `Telegram/SourceFiles/settings/settings_key_navigation.cpp` | `TCG-N45` | `SettingsKeyNavigation` | `crates/gpui_telegram_desktop/src/views/settings/settings_key_navigation.rs` | 407 | `tests/test_tcg_n45_settings_key_navigation.rs` |
| `Telegram/SourceFiles/settings/settings_key_navigation.h` | `TCG-N45` | `SettingsKeyNavigation` | `crates/gpui_telegram_desktop/src/views/settings/settings_key_navigation.rs` | 60 | `tests/test_tcg_n45_settings_key_navigation.rs` |
| `Telegram/SourceFiles/settings/settings_notifications_common.h` | `TCG-N31` | `SettingsNotificationsCommon` | `crates/gpui_telegram_desktop/src/views/settings/settings_notifications_common.rs` | 125 | `tests/test_tcg_n31_settings_notifications_common.rs` |
| `Telegram/SourceFiles/settings/settings_power_saving.cpp` | `TCG-N31` | `SettingsPowerSaving` | `crates/gpui_telegram_desktop/src/views/settings/settings_power_saving.rs` | 192 | `tests/test_tcg_n31_settings_power_saving.rs` |
| `Telegram/SourceFiles/settings/settings_power_saving.h` | `TCG-N31` | `SettingsPowerSaving` | `crates/gpui_telegram_desktop/src/views/settings/settings_power_saving.rs` | 33 | `tests/test_tcg_n31_settings_power_saving.rs` |
| `Telegram/SourceFiles/settings/settings_privacy_controllers.cpp` | `TCG-N33` | `SettingsPrivacyControllers` | `crates/gpui_telegram_desktop/src/views/settings/settings_privacy_controllers.rs` | 1854 | `tests/test_tcg_n33_settings_privacy_controllers.rs` |
| `Telegram/SourceFiles/settings/settings_privacy_controllers.h` | `TCG-N33` | `SettingsPrivacyControllers` | `crates/gpui_telegram_desktop/src/views/settings/settings_privacy_controllers.rs` | 409 | `tests/test_tcg_n33_settings_privacy_controllers.rs` |
| `Telegram/SourceFiles/settings/settings_recent_searches.cpp` | `TCG-N21` | `SettingsRecentSearches` | `crates/gpui_telegram_desktop/src/views/settings/settings_recent_searches.rs` | 109 | `tests/test_tcg_n21_settings_recent_searches.rs` |
| `Telegram/SourceFiles/settings/settings_recent_searches.h` | `TCG-N21` | `SettingsRecentSearches` | `crates/gpui_telegram_desktop/src/views/settings/settings_recent_searches.rs` | 36 | `tests/test_tcg_n21_settings_recent_searches.rs` |
| `Telegram/SourceFiles/settings/settings_scale_preview.cpp` | `TCG-N31` | `SettingsScalePreview` | `crates/gpui_telegram_desktop/src/views/settings/settings_scale_preview.rs` | 812 | `tests/test_tcg_n31_settings_scale_preview.rs` |
| `Telegram/SourceFiles/settings/settings_scale_preview.h` | `TCG-N31` | `SettingsScalePreview` | `crates/gpui_telegram_desktop/src/views/settings/settings_scale_preview.rs` | 30 | `tests/test_tcg_n31_settings_scale_preview.rs` |
| `Telegram/SourceFiles/settings/settings_search.cpp` | `TCG-N21` | `SettingsSearch` | `crates/gpui_telegram_desktop/src/views/settings/settings_search.rs` | 717 | `tests/test_tcg_n21_settings_search.rs` |
| `Telegram/SourceFiles/settings/settings_search.h` | `TCG-N21` | `SettingsSearch` | `crates/gpui_telegram_desktop/src/views/settings/settings_search.rs` | 93 | `tests/test_tcg_n21_settings_search.rs` |
| `Telegram/SourceFiles/settings/settings_type.h` | `TCG-N31` | `SettingsType` | `crates/gpui_telegram_desktop/src/views/settings/settings_type.rs` | 15 | `tests/test_tcg_n31_settings_type.rs` |

## Deep Source Reading Evidence

### `Telegram/SourceFiles/settings/business/settings_away_message.cpp`
- **Classes & Structs**: AwayMessage, AwayScheduleSelectorDescriptor, Type
- **Core Methods & Signals**: unixtime::serialize, Ui::AddSubsectionTitle, tr::lng_away_schedule_always, tr::lng_away_schedule_outside, tr::lng_away_schedule_custom, tr::lng_settings_save
- **Target**: `SettingsAwayMessageBusiness` in `crates/gpui_telegram_desktop/src/views/settings/business/settings_away_message.rs`

### `Telegram/SourceFiles/settings/business/settings_away_message.h`
- **Classes & Structs**: namespace Settings
- **Target**: `SettingsAwayMessageBusiness` in `crates/gpui_telegram_desktop/src/views/settings/business/settings_away_message.rs`

### `Telegram/SourceFiles/settings/business/settings_chat_intro.cpp`
- **Classes & Structs**: PreviewDelegate, PreviewWrap, StickerPanel, Descriptor, CustomChosen, ChatIntro, State, Selector, Mode
- **Core Methods & Signals**: Ui::AddLengthLimitLabel, ChatHelpers::LottiePlayerFromDocument, tr::lng_chat_intro_random_sticker, Settings::CreateButtonWithIcon, PreviewDelegate::PreviewDelegate, PreviewDelegate::elementAnimationsPaused
- **Target**: `SettingsChatIntroBusiness` in `crates/gpui_telegram_desktop/src/views/settings/business/settings_chat_intro.rs`

### `Telegram/SourceFiles/settings/business/settings_chat_intro.h`
- **Classes & Structs**: namespace Settings
- **Target**: `SettingsChatIntroBusiness` in `crates/gpui_telegram_desktop/src/views/settings/business/settings_chat_intro.rs`

### `Telegram/SourceFiles/settings/business/settings_chat_links.cpp`
- **Classes & Structs**: ChatLinks, ChatLinkAction, Type, Row, RowDelegate, LinksController, LinksList, ChatLinkData, Selector
- **Core Methods & Signals**: tr::lng_chat_links_clicks, tr::lng_chat_links_no_clicks, Row::Row, Row::updateStatus, Core::TextContext, Row::update
- **Target**: `SettingsChatLinksBusiness` in `crates/gpui_telegram_desktop/src/views/settings/business/settings_chat_links.rs`

### `Telegram/SourceFiles/settings/business/settings_chat_links.h`
- **Classes & Structs**: namespace Settings
- **Target**: `SettingsChatLinksBusiness` in `crates/gpui_telegram_desktop/src/views/settings/business/settings_chat_links.rs`

### `Telegram/SourceFiles/settings/business/settings_chatbots.cpp`
- **Classes & Structs**: LookupState, BotSearchState, PreviewActionKind, Chatbots, PreviewController, PreviewRow, State, ParsedUsernameQuery, BotSearchRequest, Flag
- **Core Methods & Signals**: Data::ChatbotsPermissions, PreviewRow::PreviewRow, tr::lng_chatbots_add, PreviewRow::addPillSize, PreviewRow::rightActionSize, PreviewRow::rightActionMargins
- **Target**: `SettingsChatbotsBusiness` in `crates/gpui_telegram_desktop/src/views/settings/business/settings_chatbots.rs`

### `Telegram/SourceFiles/settings/business/settings_chatbots.h`
- **Classes & Structs**: namespace Settings
- **Target**: `SettingsChatbotsBusiness` in `crates/gpui_telegram_desktop/src/views/settings/business/settings_chatbots.rs`

### `Telegram/SourceFiles/settings/business/settings_greeting.cpp`
- **Classes & Structs**: Greeting
- **Core Methods & Signals**: Greeting::Greeting, ranges::contains, ranges::sort, views::all, views::transform, tr::lng_days
- **Target**: `SettingsGreetingBusiness` in `crates/gpui_telegram_desktop/src/views/settings/business/settings_greeting.rs`

### `Telegram/SourceFiles/settings/business/settings_greeting.h`
- **Classes & Structs**: namespace Settings
- **Target**: `SettingsGreetingBusiness` in `crates/gpui_telegram_desktop/src/views/settings/business/settings_greeting.rs`

