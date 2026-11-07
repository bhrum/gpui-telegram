//! Automated Unit Tests for Settings Privacy, Security, Notifications & Chat (Module 11 Slice 8)

use gpui_telegram_desktop::views::settings::cloud_password::*;
use gpui_telegram_desktop::views::settings::sections::*;
use gpui_telegram_desktop::views::settings::*;
use gpui_telegram_desktop::theme::settings::SettingsNotificationsSectionsStyleSheet;

#[test]
fn test_cloud_password_common() {
    let mut cp = SettingsCloudPasswordCommonCloudPassword::new();
    assert_eq!(cp.current_stage, CloudPasswordStage::Start);
    cp.set_stage(CloudPasswordStage::SetPassword);
    assert_eq!(cp.current_stage, CloudPasswordStage::SetPassword);

    cp.update_hint("First pet");
    assert_eq!(cp.fields.hint, "First pet");

    cp.update_email("test@example.com");
    assert_eq!(cp.fields.email, "test@example.com");
    assert!(cp.fields.has_recovery);

    cp.set_error("Weak password");
    assert_eq!(cp.error_text.as_deref(), Some("Weak password"));
    cp.clear_error();
    assert!(cp.error_text.is_none());

    cp.reset_fields();
    assert!(cp.fields.email.is_empty());
}

#[test]
fn test_cloud_password_email_and_confirm() {
    let mut email_view = SettingsCloudPasswordEmailCloudPassword::new();
    email_view.set_email("invalid-email");
    assert!(!email_view.is_valid);
    assert!(!email_view.submit_email());
    assert!(email_view.error_message.is_some());

    email_view.set_email("user@example.com");
    assert!(email_view.is_valid);
    assert!(email_view.submit_email());
    assert!(email_view.is_submitting);

    email_view.trigger_skip();
    assert!(email_view.skip_warning_shown);

    let mut confirm = SettingsCloudPasswordEmailConfirmCloudPassword::new();
    confirm.set_target_email("test@example.com");
    assert_eq!(confirm.target_email, "test@example.com");
    assert_eq!(confirm.countdown_seconds, 60);

    for digit in ['1', '2', '3', '4', '5'] {
        assert!(!confirm.input_digit(digit));
    }
    assert!(confirm.input_digit('6'));
    assert_eq!(confirm.entered_digits, "123456");

    confirm.backspace();
    assert_eq!(confirm.entered_digits, "12345");
    confirm.clear_code();
    assert!(confirm.entered_digits.is_empty());
}

#[test]
fn test_cloud_password_hint_and_input() {
    let mut hint = SettingsCloudPasswordHintCloudPassword::new();
    hint.set_password_probe("secret123");
    hint.set_hint("secret123");
    assert!(!hint.validate());
    assert!(hint.error_message.is_some());

    hint.set_hint("My favorite car");
    assert!(hint.validate());
    assert!(hint.error_message.is_none());
    assert_eq!(hint.chars_remaining(), 64 - "My favorite car".len());

    let mut input = SettingsCloudPasswordInputCloudPassword::new();
    input.set_password("abc");
    assert_eq!(input.strength, PasswordStrength::TooShort);

    input.set_password("simplepw");
    assert_eq!(input.strength, PasswordStrength::Weak);

    input.set_password("StrongP@ssw0rd!123");
    assert_eq!(input.strength, PasswordStrength::Strong);

    input.set_confirm_password("mismatch");
    assert!(!input.check_match());
    assert!(input.error_message.is_some());

    input.set_confirm_password("StrongP@ssw0rd!123");
    assert!(input.check_match());
    assert!(input.error_message.is_none());
}

#[test]
fn test_cloud_password_wizard_steps_and_badges() {
    let mut step = SettingsCloudPasswordStepCloudPassword::new(CloudPasswordStepId::Start);
    assert_eq!(step.step_index, 0);
    assert_eq!(step.progress_pct(), 0.0);

    step.advance_step();
    assert_eq!(step.current_step, CloudPasswordStepId::EnterPassword);
    assert_eq!(step.step_index, 1);

    assert!(step.go_back());
    assert_eq!(step.current_step, CloudPasswordStepId::Start);

    let mut badge = SettingsCloudPasswordValidateIconCloudPassword::new(ValidationBadgeState::Idle);
    assert!(!badge.is_valid());
    badge.set_checking();
    assert_eq!(badge.state, ValidationBadgeState::Checking);
    badge.set_valid();
    assert!(badge.is_valid());
    badge.set_invalid("Code expired");
    assert_eq!(badge.state, ValidationBadgeState::Invalid);
}

#[test]
fn test_cloud_password_manage_and_start() {
    let mut manage = SettingsCloudPasswordManageCloudPassword::new(true, "Pet name");
    assert!(manage.has_password());
    manage.change_password();
    assert_eq!(manage.pending_action, Some(ManageAction::ChangePassword));

    manage.request_disable_password();
    assert!(manage.confirm_disable_dialog_open);
    assert!(manage.confirm_disable_password());
    assert!(!manage.confirm_disable_dialog_open);

    let mut start = SettingsCloudPasswordStartCloudPassword::new();
    assert!(!start.is_in_progress());
    start.start_setup();
    assert!(start.is_in_progress());
    start.reset();
    assert!(!start.is_in_progress());
}


#[test]
fn test_privacy_controllers_and_rules() {
    let mut ctrl = SettingsPrivacyControllers::new();
    let phone_rule = ctrl.get_rule(PrivacyKey::PhoneNumber);
    assert_eq!(phone_rule.option, PrivacyOption::Contacts);

    assert!(ctrl.is_allowed(PrivacyKey::PhoneNumber, 100, true));
    assert!(!ctrl.is_allowed(PrivacyKey::PhoneNumber, 101, false));

    ctrl.add_always_allow(PrivacyKey::PhoneNumber, 101);
    assert!(ctrl.is_allowed(PrivacyKey::PhoneNumber, 101, false));

    ctrl.add_never_allow(PrivacyKey::PhoneNumber, 100);
    assert!(!ctrl.is_allowed(PrivacyKey::PhoneNumber, 100, true));
}

#[test]
fn test_privacy_security_and_blocked_peers() {
    let mut sec = SettingsPrivacySecuritySections::new();
    assert_eq!(sec.ttl_months, 6);
    assert_eq!(sec.ttl_description(), "6 months");

    sec.set_ttl_months(12);
    assert_eq!(sec.ttl_description(), "1 year");

    sec.set_blocked_count(42);
    assert_eq!(sec.blocked_count, 42);

    let mut blocked = SettingsBlockedPeersSections::new();
    blocked.blocked_list.clear();
    assert!(blocked.is_empty());
    blocked.block_peer(500, "Spammer", "2026-11-01");
    assert!(!blocked.is_empty());
    assert_eq!(blocked.filtered_peers().len(), 1);

    blocked.set_search_query("nonexistent");
    assert_eq!(blocked.filtered_peers().len(), 0);

    assert!(blocked.unblock_peer(500));
    assert!(blocked.is_empty());
}

#[test]
fn test_chat_settings_customization() {
    let mut chat = SettingsChatSections::new();
    assert_eq!(chat.font_size, 13);
    assert_eq!(chat.theme_preset, ChatThemePreset::Day);
    assert!(chat.send_on_enter);

    chat.set_font_size(8);
    assert_eq!(chat.font_size, 12);

    chat.set_font_size(36);
    assert_eq!(chat.font_size, 30);

    chat.set_theme_preset(ChatThemePreset::Night);
    assert_eq!(chat.theme_name(), "Night");

    chat.toggle_send_on_enter();
    assert!(!chat.send_on_enter);
}

#[test]
fn test_notifications_settings_and_stylesheet() {
    let mut notif = SettingsNotificationsSections::new();
    assert!(notif.private_chats_enabled);
    assert!(notif.groups_enabled);
    assert!(!notif.channels_enabled);

    notif.toggle_channels();
    assert!(notif.channels_enabled);

    notif.set_badge_count_type(BadgeCountType::CountChats);
    assert_eq!(notif.badge_count_type, BadgeCountType::CountChats);

    assert_eq!(SettingsNotificationsSectionsStyleSheet::notifications_box_monitor(), 16.0);
    assert_eq!(SettingsNotificationsSectionsStyleSheet::notifications_box_screen_top(), 10.0);
    assert_eq!(SettingsNotificationsSectionsStyleSheet::notifications_box_screen_size(), (160.0, 100.0));
    assert_eq!(SettingsNotificationsSectionsStyleSheet::notification_sample_opacity(), 0.8);

    let style = SettingsNotificationsSectionsStyleSheet::new();
    assert_eq!(style.box_monitor, 16.0);
    assert_eq!(style.sample_opacity, 0.8);
}

#[test]
fn test_notifications_reactions_and_types() {
    let mut react = SettingsNotificationsReactionsSections::new();
    assert!(react.notify_reactions);
    assert_eq!(react.source, ReactionsNotifySource::All);

    react.set_source(ReactionsNotifySource::Contacts);
    assert_eq!(react.source, ReactionsNotifySource::Contacts);

    let mut ntype = SettingsNotificationsTypeSections::new("Channels");
    assert_eq!(ntype.chat_type_name, "Channels");
    ntype.add_exception(300, "News Channel", true, None);
    assert_eq!(ntype.exceptions.len(), 1);
    assert!(ntype.exceptions[0].is_muted);

    assert!(ntype.remove_exception(300));
    assert_eq!(ntype.exceptions.len(), 0);

    let mut common = SettingsNotificationsCommon::new();
    assert_eq!(common.corner, ScreenCorner::BottomRight);
    common.set_corner(ScreenCorner::TopLeft);
    assert_eq!(common.corner, ScreenCorner::TopLeft);

    common.set_count(100);
    assert_eq!(common.notifications_count, K_MAX_NOTIFICATIONS_COUNT);
}
