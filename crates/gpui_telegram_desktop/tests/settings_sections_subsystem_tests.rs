//! Settings Sections Subsystem Tests
//! Covers: SettingsMainSections, SettingsInformationSections, SettingsCreditsSections,
//! SettingsPremiumSections, SettingsShortcutsSections, SettingsCallsSections,
//! SettingsLocalPasscodeSections, SettingsPasskeysSections, SettingsWebsitesSections

use gpui_telegram_desktop::views::settings::sections::*;
use gpui_telegram_desktop::views::settings::sections::settings_local_passcode::AutoLockTimeout;

#[test]
fn test_settings_main_section() {
    let mut main = SettingsMainSections::new();
    assert_eq!(main.profile.name, "Pavel Durov");
    assert_eq!(main.profile.phone, "+1 234 567 8900");
    assert!(main.profile.is_verified);

    main.copy_phone();
    assert_eq!(main.toast_message.as_deref(), Some("Phone copied to clipboard"));
}

#[test]
fn test_settings_information_section_bio_validation() {
    let mut info = SettingsInformationSections::new();
    assert_eq!(info.first_name, "Pavel");
    assert_eq!(info.accounts.len(), 2);

    assert!(info.update_bio("Short bio text"));
    assert_eq!(info.bio, "Short bio text");

    let long_bio = "a".repeat(150);
    assert!(!info.update_bio(&long_bio));
}

#[test]
fn test_settings_credits_section_economy() {
    let mut credits = SettingsCreditsSections::new();
    assert_eq!(credits.balance_stars, 1500);

    credits.add_stars(500);
    assert_eq!(credits.balance_stars, 2000);
    assert!(credits.can_withdraw());

    credits.balance_stars = 500;
    assert!(!credits.can_withdraw());
}

#[test]
fn test_settings_premium_section_discounts() {
    let mut premium = SettingsPremiumSections::new();
    assert_eq!(premium.selected_months, 12);
    assert_eq!(premium.tiers.len(), 3);

    premium.select_tier(6);
    assert_eq!(premium.selected_months, 6);
}

#[test]
fn test_settings_shortcuts_section_registry() {
    let shortcuts = SettingsShortcutsSections::new();
    assert_eq!(shortcuts.shortcuts.len(), 6);
    assert_eq!(shortcuts.lng_shortcuts_lock(), "Lock Telegram");
}

#[test]
fn test_settings_calls_section_media_devices() {
    let mut calls = SettingsCallsSections::new();
    assert_eq!(calls.calls_id(), "settings_calls");
    assert_eq!(calls.input_device, "Default Microphone");
    assert_eq!(calls.output_device, "Default Output Device");

    calls.choose_capture_device("USB Audio Mic");
    assert_eq!(calls.input_device, "USB Audio Mic");

    assert!(!calls.mic_test_active);
    calls.toggle_mic_test();
    assert!(calls.mic_test_active);

    calls.update_mic_level(-30.0);
    assert!(calls.mic_level > 0.0 && calls.mic_level < 1.0);
}

#[test]
fn test_settings_local_passcode_section_security() {
    let mut passcode = SettingsLocalPasscodeSections::new();
    assert!(passcode.has_passcode);
    assert_eq!(passcode.auto_lock, AutoLockTimeout::In1Hour);

    assert!(passcode.verify_passcode("correct"));
    assert_eq!(passcode.bad_tries, 0);

    assert!(!passcode.verify_passcode("wrong_pass"));
    assert_eq!(passcode.bad_tries, 1);

    passcode.set_passcode("new_secure_pwd");
    assert!(passcode.verify_passcode("new_secure_pwd"));

    passcode.set_auto_lock(AutoLockTimeout::In5Mins);
    assert_eq!(passcode.auto_lock.as_seconds(), 300);

    assert_eq!(passcode.countdown.formatted_text(), "1:00");
    passcode.countdown.tick();
    assert_eq!(passcode.countdown.formatted_text(), "0:59");
}

#[test]
fn test_settings_passkeys_section_management() {
    let mut passkeys = SettingsPasskeysSections::new();
    assert_eq!(passkeys.passkeys_id(), "settings_passkeys");
    assert_eq!(passkeys.passkeys.len(), 2);

    let created = passkeys.add_passkey("iPad Pro", None);
    assert_eq!(passkeys.passkeys.len(), 3);
    assert_eq!(created.name, "iPad Pro");

    assert!(passkeys.rename_passkey(created.id, "iPad Pro M4"));
    assert!(passkeys.delete_passkey(created.id));
    assert_eq!(passkeys.passkeys.len(), 2);
}

#[test]
fn test_settings_websites_section_authorizations() {
    let mut websites = SettingsWebsitesSections::new();
    assert_eq!(websites.websites_id(), "settings_websites");
    assert_eq!(websites.websites.len(), 2);

    assert!(websites.disconnect(501));
    assert_eq!(websites.websites.len(), 1);

    let cleared = websites.disconnect_all();
    assert_eq!(cleared, 1);
    assert_eq!(websites.websites.len(), 0);
}
