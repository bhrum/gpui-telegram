//! Tests for Module 12: Internationalization & Localization Subsystem (`lang`)
//! Upstream: `Telegram/SourceFiles/lang/` (25 source files across `lang.json`)
//! Feature ID: `TCG-N44`

use chrono::NaiveDate;
use gpui_telegram_desktop::i18n::*;
use gpui_telegram_desktop::i18n::lang_tag::*;
use gpui_telegram_desktop::i18n::lang_text_entity::*;
use gpui_telegram_desktop::i18n::lang_instance::*;
use gpui_telegram_desktop::i18n::lang_cloud_manager::*;
use gpui_telegram_desktop::i18n::translate_provider::*;

#[test]
fn test_lang_hardcoded_strings() {
    assert_eq!(LangHardcoded::faved_set_title(), "Favorite stickers");
    assert_eq!(LangHardcoded::server_error(), "Internal server error.");
    assert_eq!(LangHardcoded::clear_path_failed(), "Clear failed :(");
    assert_eq!(
        LangHardcoded::call_error_incompatible("Alice"),
        "Alice's app is using an incompatible protocol. They need to update their app before you can call them."
    );
    assert!(LangHardcoded::passport_corrupted().contains("Passport data was corrupted"));
}

#[test]
fn test_lang_file_parser_strip_comments_and_parse() {
    let raw = r#"
        // Single line comment
        /* Multi
           line comment */
        "lng_test_greeting" = "Hello \"World\"\nWelcome!";
        "lng_user_count" = "Count: 42";
    "#;

    let stripped = LangFileParser::strip_comments(raw);
    assert!(!stripped.contains("Single line comment"));
    assert!(!stripped.contains("Multi"));

    let mut parser = LangFileParser::new();
    assert!(parser.parse_content(raw));
    let found = parser.found();
    assert_eq!(
        found.get("lng_test_greeting").map(|s| s.as_str()),
        Some("Hello \"World\"\nWelcome!")
    );
    assert_eq!(
        found.get("lng_user_count").map(|s| s.as_str()),
        Some("Count: 42")
    );
}

#[test]
fn test_lang_tag_cldr_plurals() {
    // English
    assert_eq!(LangTag::plural("en", 1.0).key_shift, K_SHIFT_ONE);
    assert_eq!(LangTag::plural("en", 0.0).key_shift, K_SHIFT_OTHER);
    assert_eq!(LangTag::plural("en", 5.0).key_shift, K_SHIFT_OTHER);

    // Russian
    assert_eq!(LangTag::plural("ru", 1.0).key_shift, K_SHIFT_ONE);
    assert_eq!(LangTag::plural("ru", 2.0).key_shift, K_SHIFT_FEW);
    assert_eq!(LangTag::plural("ru", 5.0).key_shift, K_SHIFT_MANY);
    assert_eq!(LangTag::plural("ru", 21.0).key_shift, K_SHIFT_ONE);
    assert_eq!(LangTag::plural("ru", 23.0).key_shift, K_SHIFT_FEW);

    // Arabic
    assert_eq!(LangTag::plural("ar", 0.0).key_shift, K_SHIFT_ZERO);
    assert_eq!(LangTag::plural("ar", 1.0).key_shift, K_SHIFT_ONE);
    assert_eq!(LangTag::plural("ar", 2.0).key_shift, K_SHIFT_TWO);
    assert_eq!(LangTag::plural("ar", 5.0).key_shift, K_SHIFT_FEW);
    assert_eq!(LangTag::plural("ar", 50.0).key_shift, K_SHIFT_MANY);
}

#[test]
fn test_lang_tag_number_formatting() {
    assert_eq!(LangTag::format_count_to_short(850, false).string, "850");
    assert_eq!(LangTag::format_count_to_short(1500, false).string, "1.5K");
    assert_eq!(LangTag::format_count_to_short(2_000_000, false).string, "2M");
    assert_eq!(LangTag::format_count_to_short(3_500_000_000, false).string, "3.5B");

    assert_eq!(LangTag::format_count_decimal(1234567), "1,234,567");
    assert_eq!(LangTag::format_count_decimal(-9876543), "-9,876,543");
}

#[test]
fn test_lang_text_entity_replacement_geometry() {
    let tag: u16 = 0x01;
    // Template string with tag \x20\x01
    let template_text = format!("Prefix \x20\x01 Suffix");
    let original = TextWithEntities {
        text: template_text,
        entities: vec![EntityInText {
            entity_type: EntityType::Bold,
            offset: 9, // "Suffix" starts after "Prefix \x20\x01 " (7 + 2 = 9)
            length: 6,
            data: String::new(),
        }],
    };

    let replacement = TextWithEntities {
        text: "REPLACEMENT_LONG_VALUE".to_string(), // len 22
        entities: vec![EntityInText {
            entity_type: EntityType::Italic,
            offset: 0,
            length: 11,
            data: String::new(),
        }],
    };

    let result = LangTextEntity::replace_tag(original, tag, &replacement);
    assert!(result.text.contains("Prefix REPLACEMENT_LONG_VALUE Suffix"));

    // Check entity offsets
    assert_eq!(result.entities.len(), 2);
    // Replacement italic entity starts at 7
    assert_eq!(result.entities[0].offset, 7);
    assert_eq!(result.entities[0].length, 11);
    // Original bold entity shifted: 9 + (22 - 2) = 29
    assert_eq!(result.entities[1].offset, 29);
    assert_eq!(result.entities[1].length, 6);
}

#[test]
fn test_lang_keys_name_ordering_and_dates() {
    assert_eq!(LangKeys::full_name("John", "Doe", "en"), "John Doe");
    assert_eq!(LangKeys::full_name("Taro", "Yamada", "ja"), "Yamada Taro");
    assert_eq!(LangKeys::full_name("Ming", "Li", "zh"), "Li Ming");
    assert_eq!(LangKeys::full_name("", "OnlyLast", "en"), "OnlyLast");
    assert_eq!(LangKeys::full_name("OnlyFirst", "", "en"), "OnlyFirst");

    let current = NaiveDate::from_ymd_opt(2026, 11, 7).unwrap();
    let same_year = NaiveDate::from_ymd_opt(2026, 4, 15).unwrap();
    let diff_year = NaiveDate::from_ymd_opt(2025, 4, 15).unwrap();

    assert_eq!(LangKeys::day_of_month(same_year, current), "Apr 15");
    assert_eq!(LangKeys::day_of_month(diff_year, current), "Apr 15, 2025");
}

#[test]
fn test_lang_numbers_animation() {
    let tag = 0x05;
    let template = format!("Reactions: \x20\x05 count");
    let orig = StringWithNumbers::new(template);
    let repl = LangNumbersAnimation::wrap_count("128");

    let result = LangNumbersAnimation::replace_tag(orig, tag, &repl);
    assert_eq!(result.text, "Reactions: 128 count");
    assert_eq!(result.offset, 11);
    assert_eq!(result.length, 3);
}

#[test]
fn test_lang_translator_os_menus() {
    assert_eq!(
        LangTranslator::translate("QMenuBar", "Quit %1", "Telegram"),
        Some("Quit Telegram".to_string())
    );
    assert_eq!(
        LangTranslator::translate("QCocoaMenuItem", "Copy", "Telegram"),
        Some("Copy".to_string())
    );
    assert_eq!(
        LangTranslator::translate("QLineEdit", "Undo", "Telegram"),
        Some("Undo".to_string())
    );
    assert_eq!(
        LangTranslator::translate("NonExistent", "Cut", "Telegram"),
        None
    );
}

#[test]
fn test_lang_instance_and_cloud_manager() {
    let mut instance = LangInstance::new();
    assert_eq!(instance.id(), "en");
    assert_eq!(instance.version(), 0);

    let mut cloud_mgr = LangCloudManager::new();
    let lang_es = CloudLangPackLanguage {
        lang_code: "es".to_string(),
        name: "Spanish".to_string(),
        native_name: "Español".to_string(),
        plural_code: "es".to_string(),
        base_lang_code: String::new(),
        strings_count: 1000,
        translated_count: 450, // 45% -> NotReady
        translations_url: "https://translations.telegram.org/es".to_string(),
        is_official: true,
    };

    let warning = cloud_mgr.evaluate_switch_warning(&lang_es);
    match warning {
        LanguageSwitchDisposition::NotReady { percent, .. } => assert_eq!(percent, 45),
        _ => panic!("Expected NotReady for 45%"),
    }

    let lang_de = CloudLangPackLanguage {
        lang_code: "de".to_string(),
        name: "German".to_string(),
        native_name: "Deutsch".to_string(),
        plural_code: "de".to_string(),
        base_lang_code: String::new(),
        strings_count: 1000,
        translated_count: 850, // 85% -> WarningIncomplete
        translations_url: "https://translations.telegram.org/de".to_string(),
        is_official: true,
    };

    match cloud_mgr.evaluate_switch_warning(&lang_de) {
        LanguageSwitchDisposition::WarningIncomplete { percent, .. } => assert_eq!(percent, 85),
        _ => panic!("Expected WarningIncomplete for 85%"),
    }

    cloud_mgr.switch_to_language(&mut instance, &lang_de);
    assert_eq!(instance.id(), "de");
    assert_eq!(instance.name(), "German");
    assert_eq!(instance.native_name(), "Deutsch");

    // Apply delta
    let mut deltas = std::collections::HashMap::new();
    deltas.insert("lng_greeting".to_string(), "Hallo!".to_string());
    cloud_mgr.apply_difference(&mut instance, 42, deltas);

    assert_eq!(instance.version(), 42);
    assert_eq!(instance.get_value("lng_greeting"), Some("Hallo!"));

    // Serialization
    let serialized = instance.serialize();
    let mut restored = LangInstance::new();
    assert!(restored.fill_from_serialized(&serialized));
    assert_eq!(restored.id(), "de");
    assert_eq!(restored.version(), 42);
}

#[test]
fn test_translate_providers_url_and_mtproto() {
    let mtproto = TranslateMtprotoProvider::new(101);
    assert!(mtproto.supports_message_id());
    let req = TranslateProviderRequest {
        peer_id: 12345,
        msg_id: 6789,
        text: TextWithEntities::new("Hello friend"),
    };
    let res = mtproto.translate(req, "fr");
    assert!(res.text.is_some());
    assert!(res.text.unwrap().text.contains("[fr]"));

    let url_provider = TranslateUrlProvider::new("https://api.translate.test/?q=%q&from=%f&to=%t");
    assert!(!url_provider.supports_message_id());
    let formatted_url = url_provider.format_request_url("Hello World!", "en", "de");
    assert!(formatted_url.contains("q=Hello%20World%21"));
    assert!(formatted_url.contains("from=en"));
    assert!(formatted_url.contains("to=de"));

    // Segmented JSON parsing
    let segmented_json = r#"[[["Bonjour le monde","Hello World",null,null]]]"#;
    let parsed = TranslateUrlProvider::parse_json_response(segmented_json);
    assert_eq!(parsed, Some("Bonjour le monde".to_string()));
}
