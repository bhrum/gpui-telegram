//! Subsystem Test Suite: Module 08 Slice 3 (`ui/text`)
//! Validates authentic text layout, formatting, custom emoji, and options logic.

use gpui_telegram_desktop::components::ui::text::*;

#[test]
fn test_format_values_size_text() {
    assert_eq!(format_size_text(500), "500 B");
    assert_eq!(format_size_text(1024), "1.0 KB");
    assert_eq!(format_size_text(1536), "1.5 KB");
    assert_eq!(format_size_text(1024 * 1024), "1.0 MB");
    assert_eq!(format_size_text(1024 * 1024 * 5 + 512 * 1024), "5.5 MB");
}

#[test]
fn test_format_values_duration_and_played() {
    assert_eq!(format_duration_text(45), "00:45");
    assert_eq!(format_duration_text(125), "02:05");
    assert_eq!(format_duration_text(3665), "1:01:05");

    assert_eq!(format_duration_words(45), "45s");
    assert_eq!(format_duration_words(125), "2m 5s");
    assert_eq!(format_duration_words(120), "2m");

    assert_eq!(format_played_text(30, 125), "00:30 / 02:05");
    assert_eq!(format_image_size_text(1920, 1080), "1920×1080");
}

#[test]
fn test_format_values_currency_international() {
    // Telegram Stars
    assert_eq!(fill_amount_and_currency(500, "XTR", false), "⭐500");
    assert_eq!(fill_amount_and_currency(-100, "XTR", false), "⭐100");

    // USD (exponent 2, left=true, symbol $)
    assert_eq!(fill_amount_and_currency(1250, "USD", false), "$12.50");
    assert_eq!(fill_amount_and_currency(-500, "USD", false), "−$5.00");

    // JPY (exponent 0, strip_dot_zero=true)
    assert_eq!(fill_amount_and_currency(1200, "JPY", false), "¥1200");

    // EUR (exponent 2, left=false, space=true)
    let eur = fill_amount_and_currency(2500, "EUR", false);
    assert!(eur.contains('€') && eur.contains("25"));
}

#[test]
fn test_format_values_ttl_and_dialog_dates() {
    // TTL
    assert_eq!(format_ttl(3600.0 * 5.0), "5h");
    assert_eq!(format_ttl(86400.0 * 2.0), "2d");
    assert_eq!(format_ttl(86400.0 * 14.0), "2w");
    assert_eq!(format_ttl(86400.0 * 16.0), "2w 2d");

    // Dialogs date: under 20 hours -> time string
    let recent = format_dialogs_date(3600 * 5, 0, "14:20", "Wed", "12.05.26");
    assert_eq!(recent, "14:20");

    // Dialogs date: within 7 days -> weekday
    let week = format_dialogs_date(86400 * 3, 3, "14:20", "Wed", "12.05.26");
    assert_eq!(week, "Wed");

    // Dialogs date: older -> full date
    let old = format_dialogs_date(86400 * 30, 30, "14:20", "Wed", "12.05.26");
    assert_eq!(old, "12.05.26");
}

#[test]
fn test_format_song_name_composition() {
    // Fallback unknown track
    let empty = SongNameFormatter::new("", "", "");
    assert_eq!(empty.composed_name().title, "Unknown Track");
    assert_eq!(empty.to_string_formatted(), "Unknown Track");

    // Performer & Title with en-dash
    let song = SongNameFormatter::new("", "Yesterday", "The Beatles");
    assert_eq!(song.to_string_formatted(), "The Beatles \u{2013} Yesterday");

    // No dash option
    let song_no_dash = song.with_no_dash(true);
    assert_eq!(song_no_dash.to_string_formatted(), "The Beatles Yesterday");

    // Entity highlighting covers performer
    let (text, entity) = song_no_dash.text_with_semibold(true);
    assert_eq!(text, "The Beatles Yesterday");
    assert_eq!(entity, Some((0, "The Beatles".len())));
}

#[test]
fn test_format_song_document_name_voice() {
    let voice_today = format_voice_name("Alice", false, false, "10:30", 0, "May 10");
    assert_eq!(voice_today.to_string_formatted(), "Alice Today at 10:30");

    let voice_yesterday = format_voice_name("Bob", true, false, "15:45", 1, "May 9");
    assert_eq!(voice_yesterday.to_string_formatted(), "You Yesterday at 15:45");

    let (dl_video, is_bold) = format_downloads_name(DocumentMediaType::VideoFile, None, None);
    assert_eq!(dl_video, "Video");
    assert!(is_bold);

    let (dl_sticker, _) = format_downloads_name(DocumentMediaType::Sticker, Some("🔥"), None);
    assert_eq!(dl_sticker, "Sticker 🔥");
}

#[test]
fn test_text_options_and_presets() {
    let default_opts = item_text_default_options();
    assert!(default_opts.has_flag(TEXT_PARSE_LINKS));
    assert!(default_opts.has_flag(TEXT_PARSE_MARKDOWN));
    assert!(!default_opts.has_flag(TEXT_PARSE_BOT_COMMANDS));

    let bot_opts = item_text_bot_default_options();
    assert!(bot_opts.has_flag(TEXT_PARSE_BOT_COMMANDS));

    let name_opts = name_text_options();
    assert_eq!(name_opts.maxh, 1);
    assert_eq!(name_opts.maxw, 4096);

    let (web_title, web_desc) = webpage_text_options(600, 24, 16, 20);
    assert_eq!(web_title.maxh, 40);
    assert_eq!(web_title.maxw, 560);
    assert_eq!(web_desc.maxw, 560);
}

#[test]
fn test_text_lottie_custom_emoji_metrics() {
    let metrics = CustomEmojiVerticalMetrics::for_height(25);
    assert_eq!(metrics.descent, 5);
    assert_eq!(metrics.ascent, 20);

    let mut emoji = LottieCustomEmoji::new("icon_wave", 24, 60);
    assert!(emoji.ready());
    assert!(emoji.ready_in_default_state());

    emoji.jump_to(30);
    assert_eq!(emoji.current_frame, 30);
    assert!(!emoji.ready_in_default_state());

    emoji.unload();
    assert_eq!(emoji.current_frame, 0);
}

#[test]
fn test_text_isolated_emoji_scaling() {
    let single = check_isolated_emoji("🔥", 16.0);
    assert!(single.is_some());
    assert_eq!(single.unwrap().scale, 3.0);
    assert_eq!(single.unwrap().pixel_size, 48.0);

    let double = check_isolated_emoji("🎉 ⭐", 16.0);
    assert!(double.is_some());
    assert_eq!(double.unwrap().scale, 2.0);
    assert_eq!(double.unwrap().pixel_size, 32.0);

    let text_mixed = check_isolated_emoji("Hello 🔥", 16.0);
    assert!(text_mixed.is_none());

    let four_emoji = check_isolated_emoji("😀 😃 😄 😁", 16.0);
    assert!(four_emoji.is_none());
}

#[test]
fn test_text_parser_markdown_and_entities() {
    let opts = item_text_default_options();
    let parsed = parse_text_entities("Hello **world** with ||spoiler|| and `code` at @telegram", &opts);

    assert_eq!(parsed.text, "Hello world with spoiler and code at @telegram");
    assert_eq!(parsed.entities.len(), 4);

    assert!(has_entity_type(&parsed.entities, TextEntityType::Bold));
    assert!(has_entity_type(&parsed.entities, TextEntityType::Spoiler));
    assert!(has_entity_type(&parsed.entities, TextEntityType::Code));
    assert!(has_entity_type(&parsed.entities, TextEntityType::Mention));
}

#[test]
fn test_text_layout_engine_wrapping() {
    let text = "Telegram Desktop rich text layout engine in pure Rust and GPUI.";
    let layout = UiTextString::new(text, 150.0, 14.0);

    assert!(layout.lines.len() >= 2);
    assert!(layout.max_width > 0.0);
    assert!(layout.count_height(150.0) >= layout.min_height);
}
