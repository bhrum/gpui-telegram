//! Challenger Module 12.1 Adversarial Test Suite
//! Target Subsystems: Intro Auth Wizard, Countries Subsystem, Platform Abstractions
//! Scope:
//!   - Country search fuzzing, invalid ISO codes, unicode flag conversions, binary serialization
//!   - Rapid auth state transitions, PIN underflow/overflow, countdown timer, SRP verification
//!   - Platform D-Bus notifications timeout/expiry, tray DPR scaling, bookmark resolution, window titles

use gpui_telegram_core::core::core_cloud_password::{CloudPasswordAlgoModPow, CoreCloudPassword};
use gpui_telegram_desktop::platform::linux::notifications_manager_linux::NotificationsManagerLinuxLinux;
use gpui_telegram_desktop::platform::linux::tray_linux::TrayLinuxLinux;
use gpui_telegram_desktop::platform::mac::file_bookmark_mac::FileBookmarkMacMac;
use gpui_telegram_desktop::platform::mac::tray_mac::TrayMacMac;
use gpui_telegram_desktop::platform::mac::window_title_mac::WindowTitleMacMac;
use gpui_telegram_desktop::platform::win::windows_toast_activator::{
    WindowsToastActivatorWin, NOTIFICATION_ACTIVATION_CALLBACK_IID, TOAST_ACTIVATOR_CLSID,
};
use gpui_telegram_desktop::platform::{PlatformFileBookmark, PlatformTray, PlatformWindowTitle};
use gpui_telegram_desktop::views::auth::countries_instance::{CallingCodeInfo, CountryInfo, FormatArgs};
use gpui_telegram_desktop::views::auth::intro_step::IntroStepType;
use gpui_telegram_desktop::views::auth::*;

// ============================================================================
// 1. Countries Subsystem: ISO Codes, Flag Conversions, Phone Parsing Edge Cases
// ============================================================================

#[test]
fn test_adversarial_countries_flag_emoji_invalid_and_exotic_iso() {
    // Upstream pirate flag special case (FT -> Jolly Roger)
    assert_eq!(
        CountriesInstance::flag_emoji_by_iso2("FT"),
        "\u{1F3F4}\u{200D}\u{2620}\u{FE0F}"
    );

    // Standard ISO 3166-1 alpha-2 regional indicators
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("US"), "🇺🇸");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("GB"), "🇬🇧");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("DE"), "🇩🇪");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("FR"), "🇫🇷");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("JP"), "🇯🇵");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("CN"), "🇨🇳");

    // Invalid lengths: empty, 1-char, 3-char, 4-char
    assert_eq!(CountriesInstance::flag_emoji_by_iso2(""), "");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("U"), "");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("USA"), "");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("DEU"), "");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("ABCD"), "");

    // Invalid casing: lowercase, mixed case
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("us"), "");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("De"), "");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("jP"), "");

    // Digits and punctuation
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("12"), "");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("!@"), "");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("U1"), "");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("1S"), "");

    // Exotic Unicode and multi-byte inputs
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("🇺🇸"), "");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("中国"), "");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("\0\0"), "");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("\n\t"), "");
}

#[test]
fn test_adversarial_countries_instance_lookup_and_formatting_edge_cases() {
    let instance = CountriesInstance::new();

    // Default country list checks
    let us = instance.by_iso2("US").expect("US should exist");
    assert_eq!(us.name, "United States");
    assert_eq!(us.alternative_name, "United States of America");
    assert!(!us.is_hidden);

    // Case insensitivity of by_iso2
    assert_eq!(instance.by_iso2("us"), Some(us));
    assert_eq!(instance.by_iso2("Us"), Some(us));
    assert_eq!(instance.by_iso2("uS"), Some(us));

    // Non-existent ISO lookup
    assert!(instance.by_iso2("ZZ").is_none());
    assert!(instance.by_iso2("").is_none());
    assert_eq!(instance.country_name_by_iso2("ZZ"), "");
    assert_eq!(instance.country_name_by_iso2("US"), "United States");

    // by_code with and without leading '+'
    let by_1 = instance.by_code("1").expect("+1 country");
    assert_eq!(by_1.iso2, "US");
    let by_plus_1 = instance.by_code("+1").expect("+1 with plus");
    assert_eq!(by_plus_1.iso2, "US");

    let by_44 = instance.by_code("+44").expect("+44 country");
    assert_eq!(by_44.iso2, "GB");

    // Unknown calling code
    assert!(instance.by_code("9999").is_none());
    assert!(instance.by_code("").is_none());
    assert!(instance.by_code("+").is_none());

    // Formatting: only_code vs full phone formatting
    let fmt_only = instance.format(FormatArgs {
        phone: "+1 650 253 0000".into(),
        only_code: true,
    });
    assert_eq!(fmt_only.code, "1");
    assert_eq!(fmt_only.formatted, "+1");

    let fmt_full = instance.format(FormatArgs {
        phone: "+49 30 123456".into(),
        only_code: false,
    });
    assert_eq!(fmt_full.code, "49");
    assert_eq!(fmt_full.formatted, "+49 30123456");

    // Unknown code formatting fallback
    let fmt_unknown = instance.format(FormatArgs {
        phone: "+999123456".into(),
        only_code: false,
    });
    assert_eq!(fmt_unknown.code, "");
    assert_eq!(fmt_unknown.formatted, "+999123456");

    // extract_phone_code helper
    assert_eq!(instance.extract_phone_code("+86 13800000000"), "86");
    assert_eq!(instance.extract_phone_code("+999123"), "");
}

#[test]
fn test_adversarial_countries_phone_parsing_fuzzing() {
    let instance = CountriesInstance::new();

    // Valid number extractions
    assert_eq!(instance.country_iso2_by_phone("+14155552671"), "US");
    assert_eq!(instance.country_iso2_by_phone("+447911123456"), "GB");
    assert_eq!(instance.country_iso2_by_phone("+491512345678"), "DE");
    assert_eq!(instance.country_iso2_by_phone("+33123456789"), "FR");
    assert_eq!(instance.country_iso2_by_phone("+81312345678"), "JP");
    assert_eq!(instance.country_iso2_by_phone("+8613800138000"), "CN");

    // Phone numbers with complex noise (spaces, parentheses, dashes, dots)
    assert_eq!(instance.country_iso2_by_phone("+1 (415) 555-2671"), "US");
    assert_eq!(instance.country_iso2_by_phone("+44-7911-123456"), "GB");
    assert_eq!(instance.country_iso2_by_phone("+86.138.0013.8000"), "CN");

    // Degenerate / Fuzzed phone inputs
    assert_eq!(instance.country_iso2_by_phone(""), "");
    assert_eq!(instance.country_iso2_by_phone("+"), "");
    assert_eq!(instance.country_iso2_by_phone("++---.."), "");
    assert_eq!(instance.country_iso2_by_phone("not_a_number"), "");
    assert_eq!(instance.country_iso2_by_phone("+00000000"), "");

    // Extremely long string (10,000 characters) fuzzing
    let huge_noise = format!("+1{}", "9".repeat(10_000));
    assert_eq!(instance.country_iso2_by_phone(&huge_noise), "US");
}

// ============================================================================
// 2. Countries Manager: Search Fuzzing, Serialization & Corruption Resilience
// ============================================================================

#[test]
fn test_adversarial_countries_manager_search_fuzzing() {
    let manager = CountriesManager::new();

    // 1. Empty / Whitespace queries return the entire catalog
    assert_eq!(manager.search("").len(), 6);
    assert_eq!(manager.search("   ").len(), 6);
    assert_eq!(manager.search("\t\n").len(), 6);

    // 2. Case insensitive searches
    let q_us_lower = manager.search("united");
    assert_eq!(q_us_lower.len(), 2); // US and GB ("United States", "United Kingdom")
    let q_us_upper = manager.search("UNITED");
    assert_eq!(q_us_upper.len(), 2);

    // 3. Alternative name matches
    let q_alt = manager.search("Deutschland");
    assert_eq!(q_alt.len(), 1);
    assert_eq!(q_alt[0].iso2, "DE");

    let q_prc = manager.search("PRC");
    assert_eq!(q_prc.len(), 1);
    assert_eq!(q_prc[0].iso2, "CN");

    // 4. Calling code queries
    let q_code = manager.search("49");
    assert!(q_code.iter().any(|c| c.iso2 == "DE"));
    let q_code86 = manager.search("86");
    assert!(q_code86.iter().any(|c| c.iso2 == "CN"));

    // 5. Special characters, punctuation, and injection payloads
    assert!(manager.search("!@#$%^&*()").is_empty());
    assert!(manager.search("' OR '1'='1").is_empty());
    assert!(manager.search("<script>alert(1)</script>").is_empty());
    assert!(manager.search("\0\x1f\x7f").is_empty());

    // 6. Huge query string (5,000 characters)
    let huge_query = "A".repeat(5000);
    assert!(manager.search(&huge_query).is_empty());
}

#[test]
fn test_adversarial_countries_manager_serialization_and_corruption() {
    let mut manager = CountriesManager::new();
    let original_hash = 12345;
    let original_list = manager.instance().list().to_vec();
    manager.set_country_list(original_hash, original_list);

    // Normal serialization roundtrip
    let data = manager.serialize_data();
    assert!(data.len() >= 12);

    let mut restored = CountriesManager::new();
    assert!(restored.deserialize_data(&data));
    assert_eq!(restored.hash(), original_hash);
    assert_eq!(restored.instance().list().len(), manager.instance().list().len());

    // Adversarial Corruption Tests:
    // 1. Truncated buffer (< 12 bytes header)
    let mut bad_mgr = CountriesManager::new();
    assert!(!bad_mgr.deserialize_data(&[]));
    assert!(!bad_mgr.deserialize_data(&[0u8; 4]));
    assert!(!bad_mgr.deserialize_data(&[0u8; 11]));

    // 2. Version mismatch
    let mut bad_version_data = data.clone();
    bad_version_data[0..4].copy_from_slice(&999i32.to_le_bytes());
    assert!(!bad_mgr.deserialize_data(&bad_version_data));

    // 3. Huge item count claim with no payload (out-of-bounds safety)
    let mut bad_count_data = data.clone();
    bad_count_data[8..12].copy_from_slice(&1_000_000u32.to_le_bytes());
    // Should parse what is available or safely return
    let _ = bad_mgr.deserialize_data(&bad_count_data);

    // 4. Overlong string length header (potential OOM/overflow attack)
    let mut bad_str_len = data.clone();
    // Offset 12 is first item's name_len
    if bad_str_len.len() >= 16 {
        bad_str_len[12..16].copy_from_slice(&0x7FFFFFFF_u32.to_le_bytes());
        // Should safely break and not crash with OOM
        let _ = bad_mgr.deserialize_data(&bad_str_len);
    }
}

#[test]
fn test_adversarial_countries_manager_custom_country_list() {
    let mut manager = CountriesManager::new();

    // USA alias normalization test
    let custom_list = vec![
        CountryInfo {
            name: "USA".into(),
            iso2: "US".into(),
            alternative_name: "".into(),
            is_hidden: false,
            codes: vec![CallingCodeInfo {
                calling_code: "1".into(),
                prefixes: vec![],
                patterns: vec![],
            }],
        },
        CountryInfo {
            name: "Canada".into(),
            iso2: "CA".into(),
            alternative_name: "".into(),
            is_hidden: false,
            codes: vec![CallingCodeInfo {
                calling_code: "1".into(),
                prefixes: vec![],
                patterns: vec![],
            }],
        },
    ];

    manager.set_country_list(42, custom_list);
    assert_eq!(manager.hash(), 42);
    assert!(!manager.is_loading());

    // Check that USA had alternative_name updated to "United States of America"
    let us_info = manager.instance().by_iso2("US").expect("US in custom list");
    assert_eq!(us_info.alternative_name, "United States of America");

    // Both US and CA share calling code 1
    let by_code1 = manager.instance().by_code("1").expect("code 1 exists");
    assert!(by_code1.iso2 == "US" || by_code1.iso2 == "CA");
}

// ============================================================================
// 3. Auth Wizard & PIN Input Underflow/Overflow Stress Tests
// ============================================================================

#[test]
fn test_adversarial_intro_code_input_bounds_and_overflow() {
    // 1. Extreme digit length clamping (min 1, max 32)
    let input_zero = IntroCodeInput::new(0);
    assert_eq!(input_zero.digits_count, 1);
    assert_eq!(input_zero.digits.len(), 1);

    let mut input_excess = IntroCodeInput::new(1000);
    assert_eq!(input_excess.digits_count, 32);
    assert_eq!(input_excess.digits.len(), 32);

    // 2. set_digits_count clamping and cursor adjustment
    input_excess.set_digits_count(0);
    assert_eq!(input_excess.digits_count, 1);
    input_excess.set_digits_count(500);
    assert_eq!(input_excess.digits_count, 32);

    // 3. Non-digit character rejection
    let mut standard = IntroCodeInput::new(5);
    assert!(!standard.push_digit('a'));
    assert!(!standard.push_digit(' '));
    assert!(!standard.push_digit('\n'));
    assert!(!standard.push_digit('+'));
    assert!(!standard.push_digit('🦀'));
    assert_eq!(standard.code(), "");
    assert_eq!(standard.focused_index, 0);

    // 4. Push exact digits
    assert!(standard.push_digit('1'));
    assert!(standard.push_digit('2'));
    assert!(standard.push_digit('3'));
    assert!(standard.push_digit('4'));
    assert!(standard.push_digit('5'));
    assert_eq!(standard.code(), "12345");
    assert!(standard.is_complete());

    // 5. Overflow handling: pushing beyond length
    // When focused_index is at end (digits_count - 1), pushing replaces the slot
    assert!(standard.push_digit('9'));
    assert_eq!(standard.code(), "12349");
    assert!(standard.is_complete());
}

#[test]
fn test_adversarial_intro_code_input_underflow_and_clearing() {
    let mut input = IntroCodeInput::new(4);
    assert_eq!(input.code(), "");
    assert!(!input.is_complete());

    // 1. Popping on empty input (underflow guard)
    assert!(!input.pop_digit());
    assert!(!input.pop_digit());
    assert_eq!(input.focused_index, 0);

    // 2. Interleaved pushes and pops
    assert!(input.push_digit('8'));
    assert_eq!(input.code(), "8");
    assert!(input.pop_digit());
    assert_eq!(input.code(), "");

    // Push 3 digits
    assert!(input.push_digit('1'));
    assert!(input.push_digit('2'));
    assert!(input.push_digit('3'));
    assert_eq!(input.code(), "123");

    // Pop once -> clears slot 3
    assert!(input.pop_digit());
    assert_eq!(input.code(), "12");

    // Pop second time -> clears slot 2
    assert!(input.pop_digit());
    assert_eq!(input.code(), "1");

    // Clear
    input.clear();
    assert_eq!(input.code(), "");
    assert_eq!(input.focused_index, 0);
    assert!(!input.is_error);

    // 3. set_code formatting filter
    input.set_code("9-8-7-6");
    assert_eq!(input.code(), "9876");
    assert!(input.is_complete());

    // Excess digits beyond capacity overwrite the terminal slot
    input.set_code("9-8-7-6-5");
    assert_eq!(input.code(), "9875"); // slot 3 overwritten by final digit '5'
    assert!(input.is_complete());
}

#[test]
fn test_adversarial_intro_countdown_timer_limits() {
    let mut intro_code = IntroCode::new(5, true, 60);

    // 1. Format edge cases: minute and seconds
    assert_eq!(intro_code.formatted_timeout(), "1:00");
    intro_code.tick_timer();
    assert_eq!(intro_code.formatted_timeout(), "0:59");

    // 2. Hours formatting (> 3600 seconds)
    intro_code.call_timeout_sec = 3605;
    assert_eq!(intro_code.formatted_timeout(), "01:00:05");
    intro_code.call_timeout_sec = 7320;
    assert_eq!(intro_code.formatted_timeout(), "02:02:00");
    intro_code.call_timeout_sec = 86400; // 24 hours
    assert_eq!(intro_code.formatted_timeout(), "24:00:00");

    // 3. Underflow / Zero countdown
    intro_code.call_timeout_sec = 1;
    assert_eq!(intro_code.formatted_timeout(), "0:01");
    intro_code.tick_timer();
    assert_eq!(intro_code.call_timeout_sec, 0);
    assert_eq!(intro_code.formatted_timeout(), "");

    // Additional ticks at 0 should not underflow into negative
    intro_code.tick_timer();
    assert_eq!(intro_code.call_timeout_sec, 0);
    assert_eq!(intro_code.formatted_timeout(), "");

    // Negative initial values return empty string
    intro_code.call_timeout_sec = -50;
    assert_eq!(intro_code.formatted_timeout(), "");

    // 4. Submitting incomplete vs complete code
    intro_code.input.clear();
    assert!(intro_code.submit(false).is_err());
    assert!(intro_code.input.is_error);

    intro_code.input.set_code("12345");
    assert_eq!(intro_code.submit(false), Ok(IntroStepType::Signup));

    intro_code.input.set_code("12345");
    assert_eq!(intro_code.submit(true), Ok(IntroStepType::PasswordCheck));
}

#[test]
fn test_adversarial_intro_widget_deep_state_transitions() {
    let mut widget = IntroWidget::new(IntroEnterPoint::Start);
    assert_eq!(widget.current_step, IntroStepType::Start);
    assert!(!widget.can_go_back());

    // Back-step on start step must return false
    assert!(!widget.step_back());

    // Traverse full wizard path: Start -> Qr -> Phone -> Code -> PasswordCheck -> Email -> Signup
    let sequence = [
        IntroStepType::Qr,
        IntroStepType::Phone,
        IntroStepType::Code,
        IntroStepType::PasswordCheck,
        IntroStepType::Email,
        IntroStepType::Signup,
    ];

    for &step in &sequence {
        widget.show_step(step);
        assert_eq!(widget.current_step, step);
        assert!(widget.can_go_back());
    }

    // Repeated navigation to the same step should not pollute history
    widget.show_step(IntroStepType::Signup);
    assert_eq!(widget.current_step, IntroStepType::Signup);
    assert_eq!(widget.history.len(), 6);

    // Unwind entire navigation stack back to Start
    for _ in 0..6 {
        assert!(widget.step_back());
    }
    assert_eq!(widget.current_step, IntroStepType::Start);
    assert!(!widget.can_go_back());
    assert!(!widget.step_back());

    // Rapid transition stress loop
    for _ in 0..500 {
        widget.show_step(IntroStepType::Qr);
        widget.show_step(IntroStepType::Phone);
        assert!(widget.step_back());
        assert!(widget.step_back());
    }
    assert_eq!(widget.current_step, IntroStepType::Start);
}

// ============================================================================
// 4. IntroPasswordCheck & SRP-6A Cloud Password Key Derivation
// ============================================================================

#[test]
fn test_adversarial_intro_password_check_workflow() {
    let mut check = IntroPasswordCheck::new("My hint");
    assert_eq!(check.hint, "My hint");
    assert!(!check.is_password_visible);
    assert!(!check.is_recovering);
    assert!(!check.is_submitting);

    // Empty password submission fails
    assert!(check.submit().is_err());
    assert!(!check.is_submitting);
    assert!(check.step.data.error_text.is_some());

    // Visibility toggle
    check.set_password("CorrectHorseBatteryStaple");
    assert_eq!(check.password, "CorrectHorseBatteryStaple");
    check.toggle_visibility();
    assert!(check.is_password_visible);
    check.toggle_visibility();
    assert!(!check.is_password_visible);

    // Recovery mode toggling
    check.to_recover();
    assert!(check.is_recovering);
    check.to_password();
    assert!(!check.is_recovering);

    // Non-empty password submission succeeds
    assert!(check.submit().is_ok());
    assert!(check.is_submitting);
    assert!(check.step.data.error_text.is_none());
}

#[test]
fn test_adversarial_srp_cloud_password_hash_computation() {
    let algo = CloudPasswordAlgoModPow {
        salt1: b"salt_primary_entropy_12345".to_vec(),
        salt2: b"salt_secondary_pepper_67890".to_vec(),
        g: 3,
        p: vec![0xFF; 256],
    };

    let password = b"super_secret_password_2026!";

    // 1. Determinism: same input always produces identical 32-byte digest
    let hash1 = CoreCloudPassword::compute_password_hash(&algo, password);
    let hash2 = CoreCloudPassword::compute_password_hash(&algo, password);
    assert_eq!(hash1.len(), 32);
    assert_eq!(hash1, hash2);

    // 2. Sensitivity: differing password produces different digest
    let different_pwd = b"super_secret_password_2026?";
    let hash_diff_pwd = CoreCloudPassword::compute_password_hash(&algo, different_pwd);
    assert_ne!(hash1, hash_diff_pwd);

    // 3. Salt isolation: changing salt1 or salt2 changes the digest
    let mut algo_alt_salt1 = algo.clone();
    algo_alt_salt1.salt1 = b"different_salt_1".to_vec();
    let hash_diff_salt1 = CoreCloudPassword::compute_password_hash(&algo_alt_salt1, password);
    assert_ne!(hash1, hash_diff_salt1);

    let mut algo_alt_salt2 = algo.clone();
    algo_alt_salt2.salt2 = b"different_salt_2".to_vec();
    let hash_diff_salt2 = CoreCloudPassword::compute_password_hash(&algo_alt_salt2, password);
    assert_ne!(hash1, hash_diff_salt2);

    // 4. Edge cases: Empty password and empty salts
    let empty_algo = CloudPasswordAlgoModPow {
        salt1: vec![],
        salt2: vec![],
        g: 2,
        p: vec![],
    };
    let empty_hash = CoreCloudPassword::compute_password_hash(&empty_algo, b"");
    assert_eq!(empty_hash.len(), 32);

    // 5. Extremely large password (10,000 bytes)
    let huge_password = vec![0x42u8; 10_000];
    let huge_hash = CoreCloudPassword::compute_password_hash(&algo, &huge_password);
    assert_eq!(huge_hash.len(), 32);
}

#[test]
fn test_adversarial_intro_phone_and_signup_submission_validation() {
    // 1. IntroPhone validation
    let mut phone = IntroPhone::new();
    assert_eq!(phone.country_code, "1");
    assert_eq!(phone.country_name, "United States");

    // Empty phone number rejected
    assert!(phone.submit().is_err());
    assert!(phone.step.data.error_text.is_some());

    // Enter phone digits
    phone.on_phone_input("415 555 2671");
    assert_eq!(phone.phone_number, "4155552671");
    assert_eq!(phone.full_number(), "+14155552671");

    assert_eq!(phone.submit(), Ok(IntroStepType::Code));
    assert_eq!(phone.step.data.phone, "+14155552671");
    assert_eq!(phone.step.data.country, "United States");

    // Change country
    phone.set_country("DE");
    assert_eq!(phone.country_name, "Germany");
    assert_eq!(phone.country_code, "49");
    assert_eq!(phone.flag_emoji, "🇩🇪");

    // 2. IntroSignup validation
    let mut signup = IntroSignup::new(false);
    assert!(signup.submit().is_err()); // empty first name rejected
    assert!(signup.step.data.error_text.is_some());

    signup.set_first_name("Pavel");
    signup.set_last_name("Durov");
    assert!(signup.submit().is_ok());
    assert!(signup.step.data.error_text.is_none());

    // Invert order toggle (e.g., CJK naming order)
    let inverted = IntroSignup::new(true);
    assert!(inverted.invert_order);
}

// ============================================================================
// 5. Platform Subsystem: D-Bus, Tray DPR, Bookmarks, Window Titles & COM GUIDs
// ============================================================================

#[test]
fn test_adversarial_platform_dbus_notifications_linux() {
    let mut mgr = NotificationsManagerLinuxLinux::new();
    assert!(mgr.is_supported);
    assert!(mgr.is_enabled);

    // Activatable names include FreeDesktop notification daemon and Flatpak portal
    let names = mgr.list_activatable_names();
    assert!(names.contains(&"org.freedesktop.Notifications".to_string()));
    assert!(names.contains(&"org.freedesktop.portal.Desktop".to_string()));

    // 1. Show multiple notifications with different topic IDs
    let id1 = mgr.show_notification("Chat 101", "Hello 1", Some(101));
    let id2 = mgr.show_notification("Chat 101", "Hello 2", Some(101));
    let id3 = mgr.show_notification("Chat 202", "Hello 3", Some(202));
    let id4 = mgr.show_notification("Direct", "Hello 4", None);

    assert_eq!(mgr.active_notifications.len(), 4);
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);
    assert_eq!(id4, 4);

    // 2. Targeted clear by topic (clears 101 only)
    assert!(mgr.do_clear_from_topic(101).is_ok());
    assert_eq!(mgr.active_notifications.len(), 2);
    assert!(!mgr.active_notifications.contains_key(&id1));
    assert!(!mgr.active_notifications.contains_key(&id2));
    assert!(mgr.active_notifications.contains_key(&id3));
    assert!(mgr.active_notifications.contains_key(&id4));

    // 3. Fast clear all
    assert!(mgr.do_clear_all_fast().is_ok());
    assert!(mgr.active_notifications.is_empty());

    // 4. Remote D-Bus error stripping stress
    assert_eq!(
        mgr.strip_remote_error("org.freedesktop.DBus.Error.NoReply: Did not receive a reply"),
        "Did not receive a reply"
    );
    assert_eq!(
        mgr.strip_remote_error("Simple error with no colon prefix"),
        "Simple error with no colon prefix"
    );
    assert_eq!(mgr.strip_remote_error(""), "");
    assert_eq!(
        mgr.strip_remote_error("A: B: C"),
        "B: C"
    );
}

#[test]
fn test_adversarial_platform_tray_dpr_scaling_and_menu_actions() {
    // 1. Linux Tray DPR Scaling
    let mut tray_linux = TrayLinuxLinux::new();
    assert_eq!(tray_linux.dpr, 1.0);
    assert_eq!(tray_linux.dpr_size(), 22.0);

    // Scale DPR for HiDPI displays (150%, 200%, 300%)
    tray_linux.set_dpr(1.5);
    assert_eq!(tray_linux.dpr_size(), 33.0);
    tray_linux.set_dpr(2.0);
    assert_eq!(tray_linux.dpr_size(), 44.0);
    tray_linux.set_dpr(3.0);
    assert_eq!(tray_linux.dpr_size(), 66.0);

    // Clamping sub-1.0 and negative DPR values to 1.0 floor
    tray_linux.set_dpr(0.5);
    assert_eq!(tray_linux.dpr, 1.0);
    assert_eq!(tray_linux.dpr_size(), 22.0);
    tray_linux.set_dpr(-3.0);
    assert_eq!(tray_linux.dpr, 1.0);
    assert_eq!(tray_linux.dpr_size(), 22.0);

    // Muted badge states
    tray_linux.set_unread_count(15, false);
    assert_eq!(tray_linux.unread_count, 15);
    assert!(!tray_linux.is_muted);
    tray_linux.set_unread_count(25, true);
    assert_eq!(tray_linux.unread_count, 25);
    assert!(tray_linux.is_muted);

    // 2. macOS Tray Menu Actions & Separators
    let mut tray_mac = TrayMacMac::new();
    tray_mac.create_icon().unwrap();
    assert!(tray_mac.has_icon);

    tray_mac.add_action("open", "Open Telegram").unwrap();
    tray_mac.add_separator().unwrap();
    tray_mac.add_action("mute", "Mute Notifications").unwrap();
    tray_mac.add_action("quit", "Quit").unwrap();
    assert_eq!(tray_mac.actions.len(), 4);

    // Valid action trigger
    assert!(tray_mac.trigger_action("open").unwrap());
    assert_eq!(tray_mac.last_action_triggered.as_deref(), Some("open"));

    // Unknown action trigger
    assert!(!tray_mac.trigger_action("non_existent").unwrap());

    // Separators cannot be triggered
    assert!(!tray_mac.trigger_action("").unwrap());

    // 3. Cross-platform Tray Abstraction
    let mut plat_tray = PlatformTray::new();
    assert!(plat_tray.is_visible);
    plat_tray.set_unread_count(99);
    assert_eq!(plat_tray.unread_count, 99);
    assert!(plat_tray.tooltip.contains("99"));
    plat_tray.set_unread_count(0);
    assert_eq!(plat_tray.tooltip, "Telegram Desktop");
}

#[test]
fn test_adversarial_platform_file_bookmark_sandbox_resolution() {
    let raw_target = "/Users/telegram/Documents/confidential.pdf";

    // 1. Valid Security-Scoped Bookmark Creation & Enabling
    let bookmark_bytes = PlatformFileBookmark::create_path_bookmark(raw_target);
    assert!(bookmark_bytes.starts_with(PlatformFileBookmark::BOOKMARK_MAGIC));

    let mut bm = PlatformFileBookmark::new(bookmark_bytes.clone());
    assert!(bm.check());
    assert_eq!(bm.resolved_path, raw_target);
    assert_eq!(bm.name("default.pdf"), raw_target);

    assert!(bm.enable().unwrap());
    assert!(bm.is_access_enabled);
    assert!(bm.disable().is_ok());
    assert!(!bm.is_access_enabled);

    // 2. Stale Bookmark Behavior
    bm.is_stale = true;
    assert!(!bm.check());
    assert!(!bm.enable().unwrap());
    assert!(!bm.is_access_enabled);

    // 3. Corrupt / Empty Bookmark Data
    let empty_bm = PlatformFileBookmark::new(vec![]);
    assert!(!empty_bm.check());
    assert_eq!(empty_bm.resolved_path, "");
    assert_eq!(empty_bm.name("fallback.pdf"), "fallback.pdf");

    let corrupt_bm = PlatformFileBookmark::new(vec![0xDE, 0xAD, 0xBE, 0xEF]);
    // Raw bookmark data is present so check() is true, but magic is absent so resolved_path is empty
    assert!(corrupt_bm.check());
    assert_eq!(corrupt_bm.resolved_path, "");
    assert_eq!(corrupt_bm.name("fallback.pdf"), "fallback.pdf");

    // 4. macOS-specific Bookmark helper
    let mut mac_bm = FileBookmarkMacMac::with_file(raw_target, vec![1, 2, 3]);
    assert_eq!(mac_bm.name(), "confidential.pdf");
    assert_eq!(mac_bm.path_bookmark(), raw_target);
    assert!(mac_bm.enable().unwrap());
    assert!(mac_bm.is_accessing);
    assert!(mac_bm.disable().is_ok());
    assert!(!mac_bm.is_accessing);
}

#[test]
fn test_adversarial_platform_window_titles_and_metrics() {
    // 1. Cross-platform window title styling
    let mut plat_win = PlatformWindowTitle::new();
    assert_eq!(plat_win.title, "Telegram");

    // Extreme title strings: long unicode, emojis, control characters
    let exotic_title = "🚀 Telegram (1,000 unread) — 私のチャット — \u{202E}RTL_TEST\u{202C}";
    plat_win.set_title(exotic_title);
    assert_eq!(plat_win.title, exotic_title);

    assert!(plat_win.window_title_aligns_left().is_ok());
    assert!(plat_win.align_left);
    assert!(plat_win.preview_title_height().is_ok());
    assert_eq!(plat_win.get_title_height(), 32.0);

    // 2. macOS Window Title & Traffic Light Button Palette
    let mut mac_title = WindowTitleMacMac::new();
    assert_eq!(mac_title.title_height, WindowTitleMacMac::DEFAULT_TITLE_HEIGHT);
    assert_eq!(mac_title.button_size, WindowTitleMacMac::DEFAULT_BUTTON_SIZE);
    assert_eq!(mac_title.button_skip, WindowTitleMacMac::DEFAULT_BUTTON_SKIP);

    // Standard macOS Traffic Light colors
    let close = mac_title.close_button_colors();
    assert_eq!(close.fill, 0xfc605c);
    assert_eq!(close.border, 0xde403b);

    let min = mac_title.minimize_button_colors();
    assert_eq!(min.fill, 0xfec041);
    assert_eq!(min.border, 0xdd9819);

    let max = mac_title.maximize_button_colors();
    assert_eq!(max.fill, 0x34c84a);
    assert_eq!(max.border, 0x27ae38);

    // Graphite mode palette (all traffic lights turn uniform graphite)
    mac_title.set_graphite_tint(true).unwrap();
    assert!(mac_title.is_graphite);
    let close_graph = mac_title.close_button_colors();
    let min_graph = mac_title.minimize_button_colors();
    let max_graph = mac_title.maximize_button_colors();
    assert_eq!(close_graph.fill, 0x8d8d92);
    assert_eq!(min_graph.fill, 0x8d8d92);
    assert_eq!(max_graph.fill, 0x8d8d92);
}

#[test]
fn test_adversarial_platform_windows_toast_activator_com_invariants() {
    let mut toast = WindowsToastActivatorWin::new();

    // 1. Windows Notification Activation COM GUID contract invariants
    assert_eq!(toast.clsid, TOAST_ACTIVATOR_CLSID);
    assert_eq!(toast.iid, NOTIFICATION_ACTIVATION_CALLBACK_IID);
    assert_eq!(TOAST_ACTIVATOR_CLSID, "F11932D3-6110-4BBC-9B02-B2EC07A1BD19");
    assert_eq!(NOTIFICATION_ACTIVATION_CALLBACK_IID, "53E31837-6600-4A81-9395-75CFFE746F94");

    // 2. COM RefCount Lifecycle & Saturating Underflow Guard
    assert_eq!(toast.ref_count, 1);
    assert!(toast.add_ref().is_ok());
    assert_eq!(toast.ref_count, 2);
    assert!(toast.interlocked_increment().is_ok());
    assert_eq!(toast.ref_count, 3);

    assert!(toast.release().is_ok());
    assert_eq!(toast.ref_count, 2);
    assert!(toast.release().is_ok());
    assert_eq!(toast.ref_count, 1);
    assert!(toast.release().is_ok());
    assert_eq!(toast.ref_count, 0);

    // Release at 0 should saturate at 0 and not underflow into u32::MAX
    assert!(toast.release().is_ok());
    assert_eq!(toast.ref_count, 0);

    // 3. Toast Activation Payload Handling
    toast.activate(
        "org.telegram.TelegramDesktop",
        "action=reply&msg_id=12345",
        Some("user_input"),
        Some("Yes, I agree!"),
    );
    assert!(toast.last_event.is_some());
    let ev = toast.last_event.as_ref().unwrap();
    assert_eq!(ev.app_user_model_id, "org.telegram.TelegramDesktop");
    assert_eq!(ev.invoked_args, "action=reply&msg_id=12345");
    assert_eq!(ev.user_input_key.as_deref(), Some("user_input"));
    assert_eq!(ev.user_input_value.as_deref(), Some("Yes, I agree!"));
}
