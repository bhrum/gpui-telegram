//! Challenger Module 12_2 Adversarial Stress Test Suite
//! Subsystem: i18n & CLDR, String Lexer, WebAuthn & CableCore Passkeys
//! Feature ID: TCG-N44, TCG-N01

use gpui_telegram_desktop::i18n::lang_file_parser::{LangFileParser, K_LANG_FILE_LIMIT};
use gpui_telegram_desktop::i18n::lang_tag::*;
use gpui_telegram_desktop::i18n::lang_text_entity::*;
use gpui_telegram_desktop::views::auth::cable::{
    Cable, CableOutcome, LoginRequest, RegisterRequest,
};
use gpui_telegram_desktop::views::auth::cable_box::CableSheet;
use gpui_telegram_desktop::views::auth::cable_ceremony::CableCeremony;
use gpui_telegram_desktop::views::auth::cable_core::{
    CableCore, ADVERT_SIZE, COMPRESSED_PUBLIC_KEY_SIZE, HANDSHAKE_RESPONSE_SIZE,
    NONCE_SIZE, P256_X962_LENGTH, QR_SECRET_SIZE, ROUTING_ID_SIZE,
};
use gpui_telegram_desktop::views::auth::cable_scanner_linux::CableScannerLinux;
use gpui_telegram_desktop::views::auth::cable_scanner_mac::CableScannerMac;
use gpui_telegram_desktop::views::auth::cable_scanner_win::CableScannerWin;
use gpui_telegram_desktop::views::auth::webauthn_common::WebauthnCommon;

#[test]
fn test_adversarial_cldr_plurals_20_plus_languages() {
    // 1. Arabic (ar) - 6 CLDR categories: zero, one, two, few, many, other
    assert_eq!(LangTag::plural("ar", 0.0).key_shift, K_SHIFT_ZERO, "ar 0 -> zero");
    assert_eq!(LangTag::plural("ar", 1.0).key_shift, K_SHIFT_ONE, "ar 1 -> one");
    assert_eq!(LangTag::plural("ar", 2.0).key_shift, K_SHIFT_TWO, "ar 2 -> two");

    // Arabic few: mod100 in 3..=10
    for n in [3.0, 4.0, 7.0, 10.0, 103.0, 108.0, 110.0, 905.0] {
        assert_eq!(LangTag::plural("ar", n).key_shift, K_SHIFT_FEW, "ar {} -> few", n);
    }

    // Arabic many: mod100 in 11..=99
    for n in [11.0, 15.0, 25.0, 99.0, 111.0, 150.0, 199.0, 925.0] {
        assert_eq!(LangTag::plural("ar", n).key_shift, K_SHIFT_MANY, "ar {} -> many", n);
    }

    // Arabic other: 100, 101, 102, 200, 201, 202, 1000
    for n in [100.0, 101.0, 102.0, 200.0, 201.0, 202.0, 1000.0, 1_000_000.0] {
        assert_eq!(LangTag::plural("ar", n).key_shift, K_SHIFT_OTHER, "ar {} -> other", n);
    }

    // 2. Russian (ru), Ukrainian (uk), Belarusian (be) - Slavic rules with boundary stress
    for lang in ["ru", "uk", "be"] {
        // One: mod10 == 1 && mod100 != 11
        for n in [1.0, 21.0, 31.0, 51.0, 101.0, 121.0, 1001.0] {
            assert_eq!(LangTag::plural(lang, n).key_shift, K_SHIFT_ONE, "{} {} -> one", lang, n);
        }

        // Few: mod10 in 2..=4 && mod100 not in 12..=14
        for n in [2.0, 3.0, 4.0, 22.0, 23.0, 24.0, 32.0, 102.0, 124.0, 1004.0] {
            assert_eq!(LangTag::plural(lang, n).key_shift, K_SHIFT_FEW, "{} {} -> few", lang, n);
        }

        // Many: mod10 == 0 || mod10 in 5..=9 || mod100 in 11..=14
        for n in [0.0, 5.0, 6.0, 9.0, 10.0, 15.0, 20.0, 25.0, 30.0, 100.0] {
            assert_eq!(LangTag::plural(lang, n).key_shift, K_SHIFT_MANY, "{} {} -> many", lang, n);
        }

        // Critical Boundary Challenge: 11..14 (Many) vs 21..24 (One/Few)
        assert_eq!(LangTag::plural(lang, 11.0).key_shift, K_SHIFT_MANY, "{} 11 -> many", lang);
        assert_eq!(LangTag::plural(lang, 12.0).key_shift, K_SHIFT_MANY, "{} 12 -> many", lang);
        assert_eq!(LangTag::plural(lang, 13.0).key_shift, K_SHIFT_MANY, "{} 13 -> many", lang);
        assert_eq!(LangTag::plural(lang, 14.0).key_shift, K_SHIFT_MANY, "{} 14 -> many", lang);
        assert_eq!(LangTag::plural(lang, 111.0).key_shift, K_SHIFT_MANY, "{} 111 -> many", lang);
        assert_eq!(LangTag::plural(lang, 112.0).key_shift, K_SHIFT_MANY, "{} 112 -> many", lang);
        assert_eq!(LangTag::plural(lang, 114.0).key_shift, K_SHIFT_MANY, "{} 114 -> many", lang);

        assert_eq!(LangTag::plural(lang, 21.0).key_shift, K_SHIFT_ONE, "{} 21 -> one", lang);
        assert_eq!(LangTag::plural(lang, 22.0).key_shift, K_SHIFT_FEW, "{} 22 -> few", lang);
        assert_eq!(LangTag::plural(lang, 23.0).key_shift, K_SHIFT_FEW, "{} 23 -> few", lang);
        assert_eq!(LangTag::plural(lang, 24.0).key_shift, K_SHIFT_FEW, "{} 24 -> few", lang);

        // Decimals in Slavic always collapse to other
        assert_eq!(LangTag::plural(lang, 1.5).key_shift, K_SHIFT_OTHER, "{} 1.5 -> other", lang);
        assert_eq!(LangTag::plural(lang, 0.5).key_shift, K_SHIFT_OTHER, "{} 0.5 -> other", lang);
        assert_eq!(LangTag::plural(lang, 2.75).key_shift, K_SHIFT_OTHER, "{} 2.75 -> other", lang);
    }

    // 3. Polish (pl) - Slavic variant
    assert_eq!(LangTag::plural("pl", 1.0).key_shift, K_SHIFT_ONE, "pl 1 -> one");
    for n in [2.0, 3.0, 4.0, 22.0, 23.0, 24.0, 102.0, 124.0] {
        assert_eq!(LangTag::plural("pl", n).key_shift, K_SHIFT_FEW, "pl {} -> few", n);
    }
    for n in [0.0, 5.0, 11.0, 12.0, 14.0, 15.0, 20.0, 21.0, 25.0, 112.0] {
        assert_eq!(LangTag::plural("pl", n).key_shift, K_SHIFT_MANY, "pl {} -> many", n);
    }
    assert_eq!(LangTag::plural("pl", 1.5).key_shift, K_SHIFT_OTHER, "pl 1.5 -> other");

    // 4. French (fr) & Brazilian Portuguese (pt_br) - 0 and 1 are singular
    for lang in ["fr", "pt_br"] {
        assert_eq!(LangTag::plural(lang, 0.0).key_shift, K_SHIFT_ONE, "{} 0 -> one", lang);
        assert_eq!(LangTag::plural(lang, 1.0).key_shift, K_SHIFT_ONE, "{} 1 -> one", lang);
        assert_eq!(LangTag::plural(lang, 2.0).key_shift, K_SHIFT_OTHER, "{} 2 -> other", lang);
        assert_eq!(LangTag::plural(lang, 5.0).key_shift, K_SHIFT_OTHER, "{} 5 -> other", lang);
    }

    // 5. Stress test across 20+ diverse language codes and regional subtags
    let additional_languages = [
        "en", "de", "es", "it", "nl", "sv", "no", "da", "fi", "el",
        "pt", "ro", "cs", "hu", "tr", "id", "ms", "vi", "th", "hi",
        "he", "en-US", "es-419", "de-AT", "ru-RU"
    ];
    for &lang in &additional_languages {
        let res_one = LangTag::plural(lang, 1.0);
        let res_zero = LangTag::plural(lang, 0.0);
        let res_many = LangTag::plural(lang, 100.0);

        // Valid shift category within [0, 5]
        assert!(res_one.key_shift <= K_SHIFT_OTHER);
        assert!(res_zero.key_shift <= K_SHIFT_OTHER);
        assert!(res_many.key_shift <= K_SHIFT_OTHER);
        assert!(!res_one.replacement.is_empty());
    }
}


#[test]
fn test_adversarial_number_formatting_extremes() {
    // Standard and compact boundaries
    assert_eq!(LangTag::format_count_to_short(0, false).string, "0");
    assert!(!LangTag::format_count_to_short(0, false).shortened);
    assert_eq!(LangTag::format_count_to_short(999, false).string, "999");
    assert!(!LangTag::format_count_to_short(999, false).shortened);

    assert_eq!(LangTag::format_count_to_short(1000, false).string, "1K");
    assert!(LangTag::format_count_to_short(1000, false).shortened);
    assert_eq!(LangTag::format_count_to_short(1500, false).string, "1.5K");
    assert_eq!(LangTag::format_count_to_short(2_000_000, false).string, "2M");
    assert_eq!(LangTag::format_count_to_short(3_500_000_000, false).string, "3.5B");

    // Extreme high quantities (> 10^12 trillions and quadrillions)
    let one_trillion = 1_000_000_000_000i64;
    let res_trillion = LangTag::format_count_to_short(one_trillion, false);
    assert_eq!(res_trillion.string, "1000B");
    assert!(res_trillion.shortened);

    let five_point_four_trillion = 5_400_000_000_000i64;
    let res_multi = LangTag::format_count_to_short(five_point_four_trillion, false);
    assert_eq!(res_multi.string, "5400B");
    assert!(res_multi.shortened);

    // Negative quantities
    assert_eq!(LangTag::format_count_to_short(-850, false).string, "-850");
    assert!(!LangTag::format_count_to_short(-850, false).shortened);
    assert_eq!(LangTag::format_count_to_short(-1500, false).string, "-1.5K");
    assert!(LangTag::format_count_to_short(-1500, false).shortened);
    assert_eq!(LangTag::format_count_to_short(-2_000_000, false).string, "-2M");
    assert_eq!(LangTag::format_count_to_short(-3_500_000_000, false).string, "-3.5B");

    // only_k mode override
    assert_eq!(LangTag::format_count_to_short(2_500_000, true).string, "2500K");
    assert_eq!(LangTag::format_count_to_short(1_000_000_000, true).string, "1000000K");

    // Decimal comma formatting with extremes
    assert_eq!(LangTag::format_count_decimal(0), "0");
    assert_eq!(LangTag::format_count_decimal(7), "7");
    assert_eq!(LangTag::format_count_decimal(99), "99");
    assert_eq!(LangTag::format_count_decimal(999), "999");
    assert_eq!(LangTag::format_count_decimal(1000), "1,000");
    assert_eq!(LangTag::format_count_decimal(1234567), "1,234,567");
    assert_eq!(LangTag::format_count_decimal(-9876543), "-9,876,543");
    assert_eq!(LangTag::format_count_decimal(one_trillion), "1,000,000,000,000");
    assert_eq!(LangTag::format_count_decimal(-9_876_543_210_123i64), "-9,876,543,210,123");
}

#[test]
fn test_adversarial_string_parser_and_comments() {
    // 1. Complex and nested comment patterns
    let raw = r#"
        // Header single line comment
        /* Multiline block comment
           spanning across multiple lines
           with code: "fake_key" = "fake_val";
        */
        // Another comment right before key
        "lng_title" = "Telegram GPUI";
        /* Consecutive */ /* Block */ /* Comments */
        "lng_quote_in_comment" = "Value 1"; // Comment with quotes: "hello"
        "lng_escaped_quote" = "Hello \"World\"\nSecond line\tTabbed\\Backslash";
        "lng_slashes_in_value" = "https://telegram.org/faq#support";
        "lng_comments_inside_string" = "Contains // not a comment and /* also not */";
        "lng_count#one" = "1 member";
        "lng_count#other" = "{count} members";
    "#;

    let stripped = LangFileParser::strip_comments(raw);
    assert!(!stripped.contains("Header single line comment"));
    assert!(!stripped.contains("spanning across multiple lines"));
    assert!(!stripped.contains("fake_val"));
    assert!(stripped.contains("lng_title"));
    assert!(stripped.contains("Contains // not a comment and /* also not */"));

    let mut parser = LangFileParser::new();
    assert!(parser.parse_content(raw), "Parser errors: {}", parser.errors());
    let found = parser.found();

    assert_eq!(found.get("lng_title").map(|s| s.as_str()), Some("Telegram GPUI"));
    assert_eq!(found.get("lng_quote_in_comment").map(|s| s.as_str()), Some("Value 1"));
    assert_eq!(
        found.get("lng_escaped_quote").map(|s| s.as_str()),
        Some("Hello \"World\"\nSecond line\tTabbed\\Backslash")
    );
    assert_eq!(
        found.get("lng_slashes_in_value").map(|s| s.as_str()),
        Some("https://telegram.org/faq#support")
    );
    assert_eq!(
        found.get("lng_comments_inside_string").map(|s| s.as_str()),
        Some("Contains // not a comment and /* also not */")
    );
    assert_eq!(found.get("lng_count#one").map(|s| s.as_str()), Some("1 member"));
    assert_eq!(found.get("lng_count#other").map(|s| s.as_str()), Some("{count} members"));

    // 2. Non-BMP UTF-16 surrogate pairs & Emojis in strings file
    let emoji_content = r#"
        "lng_emoji_smile" = "Happy 😀 and thumbs up 👍";
        "lng_flags" = "USA 🇺🇸 and Pirate \u{1F3F4}\u{200D}\u{2620}\u{FE0F}";
    "#;
    let mut emoji_parser = LangFileParser::new();
    assert!(emoji_parser.parse_content(emoji_content));
    assert!(emoji_parser.found().get("lng_emoji_smile").unwrap().contains("😀"));
    assert!(emoji_parser.found().get("lng_flags").unwrap().contains("🇺🇸"));

    // 3. Error detection: exceeding 1MB limit
    let huge_content = "a".repeat(K_LANG_FILE_LIMIT + 1);
    let mut limit_parser = LangFileParser::new();
    assert!(!limit_parser.parse_content(&huge_content));
    assert!(limit_parser.errors().contains("Lang file exceeds maximum size limit"));

    // 4. Malformed syntax detection
    let mut malformed_1 = LangFileParser::new();
    assert!(!malformed_1.parse_content(r#""unterminated_value" = "no closing quote;#));

    let mut malformed_2 = LangFileParser::new();
    assert!(!malformed_2.parse_content(r#""missing_equals" "val";"#));

    let mut malformed_3 = LangFileParser::new();
    assert!(!malformed_3.parse_content(r#"no_quotes_key = "val";"#));

    // Empty content parses cleanly to empty map
    let mut empty_parser = LangFileParser::new();
    assert!(empty_parser.parse_content("// only comments\n/* block */ \n\t"));
    assert!(empty_parser.found().is_empty());
}


#[test]
fn test_adversarial_entity_replacement_geometry() {
    let tag: u16 = 0x03;
    let template = "Prefix \x20\x03 Suffix and More";
    // "Prefix \x20\x03 " length = 7 + 2 + 1 = 10 bytes
    let original = TextWithEntities {
        text: template.to_string(),
        entities: vec![
            EntityInText {
                entity_type: EntityType::Bold,
                offset: 0,
                length: 6, // "Prefix"
                data: String::new(),
            },
            EntityInText {
                entity_type: EntityType::Underline,
                offset: 10,
                length: 6, // "Suffix"
                data: String::new(),
            },
            EntityInText {
                entity_type: EntityType::StrikeOut,
                offset: 21,
                length: 4, // "More"
                data: String::new(),
            },
        ],
    };

    // Replacement with rich entities and unicode emoji
    let replacement = TextWithEntities {
        text: "INSERTED_LONG_STRING_WITH_EMOJI_🚀".to_string(), // len 36 bytes (32 ASCII + 4-byte UTF-8 emoji)
        entities: vec![EntityInText {
            entity_type: EntityType::Italic,
            offset: 0,
            length: 8,
            data: String::new(),
        }],
    };

    let result = LangTextEntity::replace_tag(original, tag, &replacement);

    // Entity 0 (Prefix) strictly before insertion: offset unchanged (0), length unchanged (6)
    assert_eq!(result.entities[0].offset, 0);
    assert_eq!(result.entities[0].length, 6);
    assert_eq!(result.entities[0].entity_type, EntityType::Bold);

    // Spliced replacement entity (Italic) starts at insertion point (7)
    assert_eq!(result.entities[1].offset, 7);
    assert_eq!(result.entities[1].length, 8);
    assert_eq!(result.entities[1].entity_type, EntityType::Italic);

    // Delta calculation: 36 bytes replacement - 2 bytes tag placeholder = +34 bytes shift
    // Entity 1 (Suffix): original offset 10 -> new offset 10 + 34 = 44
    assert_eq!(result.entities[2].offset, 44);
    assert_eq!(result.entities[2].length, 6);
    assert_eq!(result.entities[2].entity_type, EntityType::Underline);

    // Entity 2 (More): original offset 21 -> new offset 21 + 34 = 55
    assert_eq!(result.entities[3].offset, 55);
    assert_eq!(result.entities[3].length, 4);
    assert_eq!(result.entities[3].entity_type, EntityType::StrikeOut);

    // Replacement tag absent: returns unchanged original
    let no_tag = TextWithEntities::new("Simple text without tag");
    let untouched = LangTextEntity::replace_tag(no_tag.clone(), 0x99, &replacement);
    assert_eq!(untouched.text, no_tag.text);
}

#[test]
fn test_adversarial_cable_core_and_ecdsa_handshake() {
    // 1. Upstream cryptographic parameter constants
    assert_eq!(P256_X962_LENGTH, 65);
    assert_eq!(COMPRESSED_PUBLIC_KEY_SIZE, 33);
    assert_eq!(QR_SECRET_SIZE, 16);
    assert_eq!(ADVERT_SIZE, 20);
    assert_eq!(ROUTING_ID_SIZE, 3);
    assert_eq!(NONCE_SIZE, 10);
    assert_eq!(HANDSHAKE_RESPONSE_SIZE, 81);

    // 2. EID generation and parsing
    let qr_secret = [0x5au8; QR_SECRET_SIZE];
    let routing_id = [0xaa, 0xbb, 0xcc];
    let advert = CableCore::generate_eid(&qr_secret, &routing_id);

    assert_eq!(advert.len(), ADVERT_SIZE);
    assert_eq!(advert[16..19], routing_id);
    assert_eq!(advert[19], 0x02); // caBLE v2 marker byte

    let (parsed_eid, parsed_routing) = CableCore::parse_advert(&advert).expect("Valid advert parsing");
    assert_eq!(parsed_routing[0..3], routing_id);
    assert_eq!(parsed_routing[3], 0x02);
    assert_ne!(parsed_eid, [0u8; 16]);

    // Truncated advert boundary: < 20 bytes must return None
    assert!(CableCore::parse_advert(&advert[..19]).is_none());
    assert!(CableCore::parse_advert(&[]).is_none());
    assert!(CableCore::parse_advert(&[0u8; 5]).is_none());

    // 3. PSK derivation determinism and nonce sensitivity
    let shared_secret = [0x77u8; 32];
    let nonce_a = [0x01u8; NONCE_SIZE];
    let mut nonce_b = nonce_a;
    nonce_b[0] = 0x02;

    let psk_a = CableCore::derive_psk(&shared_secret, &nonce_a);
    let psk_b = CableCore::derive_psk(&shared_secret, &nonce_b);
    assert_ne!(psk_a, psk_b, "Nonce change must alter derived PSK");

    // 4. ECDSA public key point coordinates & handshake verification
    let mut valid_hs = [0u8; HANDSHAKE_RESPONSE_SIZE];
    valid_hs[0] = 0x04; // Uncompressed EC point marker
    assert!(CableCore::verify_handshake(&valid_hs));

    // Adversarial reject: compressed EC points (0x02, 0x03)
    let mut compressed_even = valid_hs;
    compressed_even[0] = 0x02;
    assert!(!CableCore::verify_handshake(&compressed_even));

    let mut compressed_odd = valid_hs;
    compressed_odd[0] = 0x03;
    assert!(!CableCore::verify_handshake(&compressed_odd));

    // Adversarial reject: invalid lead bytes (0x00, 0x05, 0xFF)
    for invalid_byte in [0x00, 0x05, 0x41, 0xFF] {
        let mut corrupted = valid_hs;
        corrupted[0] = invalid_byte;
        assert!(!CableCore::verify_handshake(&corrupted));
    }

    // Adversarial reject: length deviations (80, 82, 65, 0 bytes)
    assert!(!CableCore::verify_handshake(&valid_hs[..80]));
    assert!(!CableCore::verify_handshake(&valid_hs[..65]));
    assert!(!CableCore::verify_handshake(&[]));
    let oversized = vec![0x04u8; 82];
    assert!(!CableCore::verify_handshake(&oversized));
}

#[test]
fn test_adversarial_webauthn_client_data_and_scanners() {
    // 1. WebAuthn Client Data JSON Oracle
    let challenge_raw = b"telegram_session_challenge_987654";
    let client_data_json = format!(
        r#"{{"type":"webauthn.get","challenge":"{}","origin":"https://web.telegram.org","crossOrigin":false}}"#,
        base64_encode(challenge_raw)
    );
    assert!(client_data_json.contains(r#""type":"webauthn.get""#));
    assert!(client_data_json.contains(r#""origin":"https://web.telegram.org""#));

    // Client data hash computation (SHA-256)
    let client_data_hash = sha256_mock(client_data_json.as_bytes());
    assert_eq!(client_data_hash.len(), 32);

    // 2. Cable Registration & Login with adversarial parameters
    let valid_reg = RegisterRequest {
        rp_id: "telegram.org".into(),
        rp_name: "Telegram".into(),
        user_id: vec![1, 2, 3],
        user_name: "alice".into(),
        user_display_name: "Alice".into(),
        client_data_hash: client_data_hash.clone(),
        algorithms: vec![-7], // ES256
        timeout_ms: 60000,
    };
    let reg_res = Cable::register(valid_reg);
    assert_eq!(reg_res.outcome, CableOutcome::Success);
    assert!(!reg_res.credential_id.is_empty());

    // Empty rp_id or client_data_hash must fail
    let invalid_reg_1 = RegisterRequest { rp_id: String::new(), ..Default::default() };
    assert_eq!(Cable::register(invalid_reg_1).outcome, CableOutcome::Failed);

    let invalid_reg_2 = RegisterRequest {
        rp_id: "telegram.org".into(),
        client_data_hash: Vec::new(),
        ..Default::default()
    };
    assert_eq!(Cable::register(invalid_reg_2).outcome, CableOutcome::Failed);

    // Login flow
    let valid_login = LoginRequest {
        rp_id: "telegram.org".into(),
        client_data_hash,
        allow_credential_ids: vec![reg_res.credential_id],
        timeout_ms: 60000,
    };
    let login_res = Cable::login(valid_login);
    assert_eq!(login_res.outcome, CableOutcome::Success);
    assert!(!login_res.signature.is_empty());

    // 3. WebauthnCommon & FIDO2 device detection
    let webauthn = WebauthnCommon::new();
    assert_eq!(webauthn.timeout_ms, 60_000);
    // Security key presence check is platform-aware
    let is_sec_key = webauthn.security_key_present();
    assert_eq!(is_sec_key, cfg!(target_os = "macos") || cfg!(target_os = "windows"));
    assert!(!webauthn.libfido2_device_present());

    // 4. CableCeremony state machine and timeout
    let mut ceremony = CableCeremony::new();
    assert_eq!(ceremony.current_sheet, CableSheet::Qr);
    let qr_payload = ceremony.start("telegram.org");
    assert!(qr_payload.starts_with("FIDO:/telegram.org/"));
    assert!(ceremony.is_active);

    ceremony.advance_sheet(CableSheet::Connecting);
    assert_eq!(ceremony.current_sheet, CableSheet::Connecting);

    // Timeout countdown
    assert!(!ceremony.tick(60_000));
    assert!(ceremony.is_active);
    assert!(ceremony.tick(120_000)); // Total 180s -> triggers timeout
    assert!(!ceremony.is_active);
    assert_eq!(ceremony.current_sheet, CableSheet::Error);

    // 5. Cross-Platform BLE Scanners
    // macOS CoreBluetooth UUID filter
    assert!(CableScannerMac::matches_cbuuid("FDE2"));
    assert!(CableScannerMac::matches_cbuuid("fde2"));
    assert!(CableScannerMac::matches_cbuuid("FFF9"));
    assert!(CableScannerMac::matches_cbuuid("fff9"));
    assert!(!CableScannerMac::matches_cbuuid("180D"));
    assert!(!CableScannerMac::matches_cbuuid(""));

    let valid_service_data = vec![0x11; ADVERT_SIZE];
    assert!(CableScannerMac::extract_advert("FDE2", &valid_service_data).is_some());
    assert!(CableScannerMac::extract_advert("180D", &valid_service_data).is_none());
    assert!(CableScannerMac::extract_advert("FDE2", &[0x11; 10]).is_none());

    // Linux BlueZ D-Bus UUID filter
    assert!(CableScannerLinux::is_cable_uuid("0000fde2-0000-1000-8000-00805f9b34fb"));
    assert!(CableScannerLinux::is_cable_uuid("0000FFF9-0000-1000-8000-00805F9B34FB"));
    assert!(!CableScannerLinux::is_cable_uuid("0000180d-0000-1000-8000-00805f9b34fb"));

    // Windows WinRT 16-bit UUID and Service Data AD Type
    assert!(CableScannerWin::matches_uuid16(0xFDE2));
    assert!(CableScannerWin::matches_uuid16(0xFFF9));
    assert!(!CableScannerWin::matches_uuid16(0x180D));
    assert!(CableScannerWin::parse_advertisement(0x16, 0xFDE2, &valid_service_data).is_some());
    assert!(CableScannerWin::parse_advertisement(0x16, 0x180D, &valid_service_data).is_none());
    assert!(CableScannerWin::parse_advertisement(0x01, 0xFDE2, &valid_service_data).is_none());
}

fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i] as u32;
        let b1 = if i + 1 < data.len() { data[i + 1] as u32 } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(TABLE[((triple >> 18) & 0x3F) as usize] as char);
        result.push(TABLE[((triple >> 12) & 0x3F) as usize] as char);
        if i + 1 < data.len() {
            result.push(TABLE[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if i + 2 < data.len() {
            result.push(TABLE[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        i += 3;
    }
    result
}

fn sha256_mock(data: &[u8]) -> Vec<u8> {
    let mut hash = vec![0u8; 32];
    for (i, &b) in data.iter().enumerate() {
        hash[i % 32] = hash[i % 32].wrapping_add(b).wrapping_mul(31);
    }
    hash
}
