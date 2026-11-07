//! Subsystem Integration & Unit Tests for Module 12: Intro & Auth Wizard
//! Covers: CountriesInstance, CountriesManager, IntroStep, IntroPhone, IntroCode,
//! IntroCodeInput, IntroSignup, IntroPasswordCheck, Cable, CableCore, CableScanners

use gpui_telegram_desktop::views::auth::*;

#[test]
fn test_countries_instance_flag_emoji() {
    // Upstream pirate flag override
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("FT"), "\u{1F3F4}\u{200D}\u{2620}\u{FE0F}");

    // Regional indicator conversions
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("US"), "🇺🇸");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("DE"), "🇩🇪");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("GB"), "🇬🇧");

    // Invalid ISO formats
    assert_eq!(CountriesInstance::flag_emoji_by_iso2(""), "");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("USA"), "");
    assert_eq!(CountriesInstance::flag_emoji_by_iso2("12"), "");
}

#[test]
fn test_countries_instance_lookup_and_format() {
    let instance = CountriesInstance::new();

    let us = instance.by_iso2("US").expect("US should exist");
    assert_eq!(us.name, "United States");

    let by_code = instance.by_code("1").expect("+1 should exist");
    assert_eq!(by_code.iso2, "US");

    let iso = instance.country_iso2_by_phone("+14155552671");
    assert_eq!(iso, "US");

    let fmt = instance.format(countries_instance::FormatArgs {
        phone: "+14155552671".into(),
        only_code: false,
    });
    assert_eq!(fmt.code, "1");
    assert_eq!(fmt.formatted, "+1 4155552671");
}

#[test]
fn test_countries_manager_search_and_serialization() {
    let mut manager = CountriesManager::new();

    let search_us = manager.search("United");
    assert!(!search_us.is_empty());
    assert!(search_us.iter().any(|c| c.iso2 == "US"));

    let data = manager.serialize_data();
    assert!(!data.is_empty());

    let mut restored = CountriesManager::new();
    assert!(restored.deserialize_data(&data));
    assert_eq!(restored.hash(), manager.hash());
    assert!(!restored.instance().list().is_empty());
}

#[test]
fn test_intro_code_input_push_pop() {
    let mut input = IntroCodeInput::new(5);
    assert_eq!(input.digits_count, 5);
    assert!(!input.is_complete());

    assert!(input.push_digit('1'));
    assert!(input.push_digit('2'));
    assert!(input.push_digit('3'));
    assert_eq!(input.code(), "123");
    assert!(!input.is_complete());

    assert!(input.push_digit('4'));
    assert!(input.push_digit('5'));
    assert_eq!(input.code(), "12345");
    assert!(input.is_complete());

    // Backspace
    assert!(input.pop_digit());
    assert_eq!(input.code(), "1234");
    assert!(!input.is_complete());

    input.clear();
    assert_eq!(input.code(), "");
}

#[test]
fn test_intro_code_countdown_timer() {
    let mut code_view = IntroCode::new(5, true, 65);
    assert_eq!(code_view.formatted_timeout(), "1:05");

    code_view.tick_timer();
    assert_eq!(code_view.call_timeout_sec, 64);
    assert_eq!(code_view.formatted_timeout(), "1:04");

    code_view.call_timeout_sec = 3661;
    assert_eq!(code_view.formatted_timeout(), "01:01:01");

    code_view.call_timeout_sec = 0;
    assert_eq!(code_view.formatted_timeout(), "");
}

#[test]
fn test_intro_widget_navigation_flow() {
    let mut widget = IntroWidget::new(IntroEnterPoint::Start);
    assert_eq!(widget.current_step, intro_step::IntroStepType::Start);
    assert!(!widget.can_go_back());

    widget.show_step(intro_step::IntroStepType::Qr);
    assert_eq!(widget.current_step, intro_step::IntroStepType::Qr);
    assert!(widget.can_go_back());

    widget.show_step(intro_step::IntroStepType::Phone);
    assert_eq!(widget.current_step, intro_step::IntroStepType::Phone);

    assert!(widget.step_back());
    assert_eq!(widget.current_step, intro_step::IntroStepType::Qr);

    assert!(widget.step_back());
    assert_eq!(widget.current_step, intro_step::IntroStepType::Start);
    assert!(!widget.can_go_back());
}

#[test]
fn test_cable_core_cryptographic_helpers() {
    assert_eq!(cable_core::P256_X962_LENGTH, 65);
    assert_eq!(cable_core::ADVERT_SIZE, 20);
    assert_eq!(cable_core::HANDSHAKE_RESPONSE_SIZE, 81);

    let qr_secret = [0x42u8; cable_core::QR_SECRET_SIZE];
    let routing_id = [0x01, 0x02, 0x03];
    let advert = CableCore::generate_eid(&qr_secret, &routing_id);

    assert_eq!(advert.len(), 20);
    assert_eq!(advert[16..19], routing_id);
    assert_eq!(advert[19], 0x02); // caBLE v2 marker

    let parsed = CableCore::parse_advert(&advert).expect("Valid advert");
    assert_eq!(parsed.1[0..3], routing_id);

    // Handshake verification
    let mut valid_hs = [0u8; cable_core::HANDSHAKE_RESPONSE_SIZE];
    valid_hs[0] = 0x04; // Uncompressed EC point
    assert!(CableCore::verify_handshake(&valid_hs));

    let mut invalid_hs = valid_hs;
    invalid_hs[0] = 0x02; // Compressed point rejected
    assert!(!CableCore::verify_handshake(&invalid_hs));
}

#[test]
fn test_cable_scanner_uuid_filters() {
    // Linux
    assert!(CableScannerLinux::is_cable_uuid(cable_scanner_linux::GOOGLE_CABLE_UUID));
    assert!(CableScannerLinux::is_cable_uuid(cable_scanner_linux::FIDO_CABLE_UUID));
    assert!(!CableScannerLinux::is_cable_uuid("invalid-uuid"));

    // Mac
    assert!(CableScannerMac::matches_cbuuid("FDE2"));
    assert!(CableScannerMac::matches_cbuuid("FFF9"));
    assert!(!CableScannerMac::matches_cbuuid("180D"));

    // Windows
    assert!(CableScannerWin::matches_uuid16(0xFDE2));
    assert!(CableScannerWin::matches_uuid16(0xFFF9));
    assert!(!CableScannerWin::matches_uuid16(0x180D));
}
