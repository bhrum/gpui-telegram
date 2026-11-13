//! Subsystem tests for Module 08 Slice 9 (UI root, image, peer, theme).
use gpui_telegram_desktop::components::ui::color_contrast::ColorContrast;
use gpui_telegram_desktop::components::ui::color_int_conversion::ColorIntConversion;
use gpui_telegram_desktop::components::ui::empty_userpic::EmptyUserpic;
use gpui_telegram_desktop::components::ui::image::image_location::{
    ImageLocationImage, K_DOCUMENT_BASE_CACHE_TAG, K_PHOTO_BASE_CACHE_TAG,
};
use gpui_telegram_desktop::components::ui::passcode_strength::{
    PasscodeStrength, PasscodeStrengthAdvice, PasscodeStrengthBand,
};
use gpui_telegram_desktop::components::ui::passcode_strength_translit::PasscodeStrengthTranslit;
use gpui_telegram_desktop::components::ui::peer::color_sample::{ColorSamplePeer, K_PROFILE_COLOR_INDEX_COUNT};
use gpui_telegram_desktop::components::ui::unread_counter_format::UnreadCounterFormat;
use gpui_telegram_desktop::components::ui::userpic_view::{PeerUserpicShape, UserpicView};
use gpui_telegram_desktop::theme::ui::color_indices_style::ColorIndicesStyleSheet;
use gpui_telegram_desktop::theme::ui::passcode_strength_meter_style::PasscodeStrengthMeterWidgetsStyleSheet;

#[test]
fn test_color_contrast_wcag_aa() {
    // Identical colors have contrast 1.0
    let contrast_same = ColorContrast::count_contrast((255, 255, 255), (255, 255, 255));
    assert!((contrast_same - 1.0).abs() < 1e-3);

    // Black on white has maximum contrast 21.0
    let contrast_bw = ColorContrast::count_contrast((0, 0, 0), (255, 255, 255));
    assert!((contrast_bw - 21.0).abs() < 0.1);

    // White background is classified as light
    assert!(ColorContrast::is_light_background(255, 255, 255));
    // Black background is not light
    assert!(!ColorContrast::is_light_background(0, 0, 0));
}

#[test]
fn test_color_int_conversion() {
    let red_serialized = 0x00FF0000;
    let color = ColorIntConversion::color_from_serialized(red_serialized);
    assert!((color.r - 1.0).abs() < 1e-3);
    assert!((color.g - 0.0).abs() < 1e-3);
    assert!((color.b - 0.0).abs() < 1e-3);

    // u32::MAX returns None for optional serialized color
    assert!(ColorIntConversion::maybe_color_from_serialized(u32::MAX).is_none());
    assert!(ColorIntConversion::maybe_color_from_serialized(red_serialized).is_some());
}

#[test]
fn test_unread_counter_format() {
    assert_eq!(UnreadCounterFormat::format(0, false, false), "");
    assert_eq!(UnreadCounterFormat::format(42, false, false), "42");
    assert_eq!(UnreadCounterFormat::format(1500, true, true), "99+");
    assert_eq!(UnreadCounterFormat::format(2_000_000, false, true), "99999+");

    // Short format
    assert_eq!(UnreadCounterFormat::format_short(450), "450");
    assert_eq!(UnreadCounterFormat::format_short(1_500), "1K");
    assert_eq!(UnreadCounterFormat::format_short(2_500_000), "2M");
    assert_eq!(UnreadCounterFormat::format_short(3_000_000_000), "3B");
}

#[test]
fn test_passcode_strength_and_translit() {
    // Transliterate Russian "пароль" -> "parol"
    let translit = PasscodeStrengthTranslit::transliterate_string("пароль");
    assert_eq!(translit, "parol");

    // "1111" repeated -> VeryWeak, Repeat
    let (band1, advice1, _) = PasscodeStrength::evaluate("1111");
    assert_eq!(band1, PasscodeStrengthBand::VeryWeak);
    assert_eq!(advice1, PasscodeStrengthAdvice::Repeat);

    // Short passcode -> VeryWeak, TooShort
    let (band2, advice2, _) = PasscodeStrength::evaluate("12");
    assert_eq!(band2, PasscodeStrengthBand::VeryWeak);
    assert_eq!(advice2, PasscodeStrengthAdvice::TooShort);

    // Strong diverse password
    let (band3, advice3, entropy3) = PasscodeStrength::evaluate("Correct-Horse-Battery-Staple-2026!");
    assert_eq!(band3, PasscodeStrengthBand::Strong);
    assert_eq!(advice3, PasscodeStrengthAdvice::Fine);
    assert!(entropy3 > 64.0);
}

#[test]
fn test_image_location_and_cache_tags() {
    let loc_photo = ImageLocationImage::for_photo(1001, 9999, vec![1, 2, 3], 800, 600, 2);
    assert!(loc_photo.is_valid());
    assert_eq!(loc_photo.dc_id(), 2);
    let tag = loc_photo.cache_key_tag();
    assert_eq!(tag & K_PHOTO_BASE_CACHE_TAG, K_PHOTO_BASE_CACHE_TAG);

    let loc_doc = ImageLocationImage::for_document(2002, 8888, vec![4, 5], 320, 240, 1);
    assert!(loc_doc.is_valid());
    let doc_tag = loc_doc.cache_key_tag();
    assert_eq!(doc_tag & K_DOCUMENT_BASE_CACHE_TAG, K_DOCUMENT_BASE_CACHE_TAG);
}

#[test]
fn test_empty_userpic_and_shapes() {
    let initials = EmptyUserpic::extract_initials("Pavel Durov");
    assert_eq!(initials, "PD");

    let single_initial = EmptyUserpic::extract_initials("Telegram");
    assert_eq!(single_initial, "T");

    let userpic = EmptyUserpic::for_peer("Alice Smith", 42, 48.0);
    assert_eq!(userpic.initials(), "AS");

    let circle_view = UserpicView::new(48.0, PeerUserpicShape::Circle);
    assert!(circle_view.border_radius() > gpui::px(20.0));

    let square_view = UserpicView::new(48.0, PeerUserpicShape::Square);
    assert_eq!(square_view.border_radius(), gpui::px(0.0));
}

#[test]
fn test_peer_color_sample() {
    let sample = ColorSamplePeer::new(3);
    assert_eq!(sample.color_index(), 3);
    assert!(!sample.is_selected());

    let (c1, c2) = sample.gradient_stops();
    assert!(c1.h > 0.0);
    assert!(c2.h > 0.0);

    // Index wrapping modulo 8
    let wrapped_sample = ColorSamplePeer::new(8);
    assert_eq!(wrapped_sample.color_index(), 0);
    assert_eq!(K_PROFILE_COLOR_INDEX_COUNT, 8);
}

#[test]
fn test_theme_stylesheets() {
    let indices = ColorIndicesStyleSheet::default();
    assert_eq!(indices.red, 0);
    assert_eq!(indices.dark_blue, 7);

    let meter = PasscodeStrengthMeterWidgetsStyleSheet::default();
    assert_eq!(meter.segment_count, 4);
}
