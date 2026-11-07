//! Subsystem unit tests for UI Chat Styling, Badges & UI Controls (Module 08 Slice 5).
//! Validates Mandate #18 anti-bluffing exact math, upstream C++ line citations, and algorithms.

use gpui_telegram_desktop::components::ui::cached_round_corners::*;
use gpui_telegram_desktop::components::ui::unread_counter_format::*;
use gpui_telegram_desktop::components::ui::unread_badge_paint::*;
use gpui_telegram_desktop::components::ui::userpic_view::*;
use gpui_telegram_desktop::components::ui::chat::chat_style::*;
use gpui_telegram_desktop::components::ui::chat::group_call_bar::*;
use gpui_telegram_desktop::components::ui::controls::send_button::*;
use gpui_telegram_desktop::components::ui::controls::window_outdated_bar::*;
use gpui_telegram_desktop::components::ui::controls::subsection_tabs_slider::*;
use gpui_telegram_desktop::components::ui::controls::subsection_tabs_slider_reorder::*;
use gpui_telegram_desktop::components::ui::controls::swipe_handler::*;
use gpui_telegram_desktop::components::ui::controls::ton_common::*;
use gpui_telegram_desktop::components::ui::controls::title_sub_widget::*;
use gpui_telegram_desktop::components::ui::controls::ttl_media::*;

#[test]
fn test_cached_round_corners_presets_and_math() {
    let none = CachedRoundCorners::for_preset(ImageRoundRadius::None);
    assert!(!none.is_rounded());

    let small = CachedRoundCorners::for_preset(ImageRoundRadius::Small);
    assert!(small.is_rounded());
    assert_eq!(small.radius(), gpui::px(4.0));

    let large = CachedRoundCorners::for_preset(ImageRoundRadius::Large);
    assert_eq!(large.radius(), gpui::px(10.0));

    let ellipse = CachedRoundCorners::for_preset(ImageRoundRadius::Ellipse);
    assert!(ellipse.is_circle());
}

#[test]
fn test_unread_counter_formatting_and_metric_abbreviations() {
    // Narrow mode threshold caps (unread_counter_format.cpp:L19-L24)
    assert_eq!(UnreadCounterFormat::format(1000, true, true), "99+");
    assert_eq!(UnreadCounterFormat::format(500, false, true), "500");
    assert_eq!(UnreadCounterFormat::format(1_000_000, false, true), "99999+");
    assert_eq!(UnreadCounterFormat::format(50, false, false), "50");

    // Metric abbreviation (unread_counter_format.cpp:L29-L38)
    assert_eq!(UnreadCounterFormat::format_short(999), "999");
    assert_eq!(UnreadCounterFormat::format_short(1_500), "1K");
    assert_eq!(UnreadCounterFormat::format_short(2_500_000), "2M");
    assert_eq!(UnreadCounterFormat::format_short(3_000_000_000), "3B");
}

#[test]
fn test_unread_badge_paint_bounds_and_pill_geometry() {
    let painter = UnreadBadgePaint::new(UnreadBadgeSize::Dialogs);
    assert_eq!(painter.corner_radius(), gpui::px(9.0));

    let bounds = painter.compute_bounds(gpui::px(10.0), gpui::Point::default());
    // Minimum width is height (18px)
    assert!(bounds.size.width >= gpui::px(18.0));
}

#[test]
fn test_userpic_view_shapes_and_constants() {
    let circle = UserpicView::new(48.0, PeerUserpicShape::Circle);
    assert_eq!(circle.border_radius(), gpui::px(24.0));

    let forum = UserpicView::new(48.0, PeerUserpicShape::Forum);
    assert_eq!(forum.border_radius(), gpui::px(48.0 * 0.35));

    // Upstream community card stack constants (userpic_view.cpp:L19-L27)
    assert_eq!(K_PEEK, 0.3);
    assert_eq!(K_COVER, 0.5);
    assert_eq!(K_CARD1_SCALE, 0.82);
    assert_eq!(K_CARD2_SCALE, 0.70);
}

#[test]
fn test_chat_style_service_radius_and_inverted_shrink_math() {
    let metrics = ServiceMsgRadiusMetrics {
        min_message_height: 24.0,
        min_row_height: 20.0,
    };
    // Radius = 24 / 2 = 12
    assert_eq!(metrics.history_service_msg_radius(), 12.0);
    // Inverted Radius = 20 - 12 = 8
    assert_eq!(metrics.history_service_msg_inverted_radius(), 8.0);
    // Inverted Shrink = (8 * 2) / 3 = 5.3333335
    let shrink = metrics.history_service_msg_inverted_shrink();
    assert!((shrink - 5.3333).abs() < 0.001);

    // Quote opacity constants (chat_style.cpp:L56-L60)
    assert_eq!(K_DEFAULT_BG_OPACITY, 0.12);
    assert_eq!(K_DEFAULT_OUTLINE1_OPACITY, 0.90);
    assert_eq!(K_DEFAULT_OUTLINE2_OPACITY, 0.30);
    assert_eq!(K_DEFAULT_OUTLINE3_OPACITY, 0.60);
}

#[test]
fn test_group_call_bar_countdown_formatting() {
    // 2 days remaining (group_call_bar.cpp:L73-L94)
    let (d, late) = format_scheduled_left(2 * 86400);
    assert_eq!(d, "2d");
    assert!(!late);

    // 1 hour 30 min
    let (h, _) = format_scheduled_left(3600 + 30 * 60);
    assert_eq!(h, "01:30:00");

    // 45 seconds
    let (m, _) = format_scheduled_left(45);
    assert_eq!(m, "00:45");

    // Late 15 seconds (minus sign \u{2212})
    let (l, late2) = format_scheduled_left(-15);
    assert_eq!(l, "\u{2212}00:15");
    assert!(late2);
}

#[test]
fn test_send_button_slowmode_and_types() {
    assert_eq!(format_slowmode_delay(75), "1:15");
    assert_eq!(format_slowmode_delay(30), "0:30");

    let btn = SendButtonControls::new().with_slowmode(120);
    assert_eq!(btn.button_type, SendButtonType::Slowmode);
    assert_eq!(btn.slowmode_delay_seconds, 120);
}

#[test]
fn test_window_outdated_bar_date_packing_and_throttling() {
    assert_eq!(pack_date(2026, 11, 7), 20261107);
    assert_eq!(K_MINIMAL_SKIP_DAYS, 7);
    assert_eq!(K_SOON_SKIP_DAYS, 30);
    assert_eq!(K_NOW_SKIP_DAYS, 90);
}

#[test]
fn test_subsection_tabs_slider_and_reorder() {
    let mut slider = SubsectionTabsSliderControls::new(false);
    slider.add_tab("General", true);
    slider.add_tab("Notifications", false);
    slider.add_tab("Privacy", false);

    assert_eq!(slider.pinned_count, 1);
    assert_eq!(slider.tabs.len(), 3);

    // Reorder tabs
    let reordered = slider.reorder_tabs(1, 2);
    assert!(reordered);
    assert_eq!(slider.tabs[1].title, "Privacy");
    assert_eq!(slider.tabs[2].title, "Notifications");

    // Test drag reorder controller
    let mut reorder = SubsectionTabsSliderReorderControls::new();
    reorder.add_pinned_interval(0, 1);
    assert!(reorder.is_index_pinned(0));
    assert!(!reorder.is_index_pinned(1));

    assert!(!reorder.start(0)); // cannot drag pinned
    assert!(reorder.start(1));  // can drag normal tab
    reorder.update_shift(25.0);
    assert_eq!(reorder.current_shift, 25.0);
    let finished = reorder.finish();
    assert_eq!(finished, Some((1, 25.0)));
}

#[test]
fn test_swipe_handler_logarithmic_damping_curve() {
    assert_eq!(damped_overswipe(0.0, 1.0), 0.0);
    let damp1 = damped_overswipe(50.0, 1.0);
    let damp2 = damped_overswipe(100.0, 1.0);
    assert!(damp1 > 0.0);
    assert!(damp2 > damp1);
    // Damping should grow sublinearly (logarithmic)
    assert!(damp2 < damp1 * 2.0);

    let mut handler = SwipeHandlerControls::new(60.0);
    let trans1 = handler.update_translation(30.0);
    assert_eq!(trans1, 30.0);
    assert!(!handler.is_triggered);

    let trans2 = handler.update_translation(50.0); // total 80.0 > 60.0 threshold
    assert!(handler.is_triggered);
    assert!(trans2 > 60.0);
}

#[test]
fn test_ton_amount_formatting_and_parsing() {
    let fmt1 = format_ton_amount(1_500_000_000, false, false);
    assert_eq!(fmt1.full, "1.5");

    let fmt2 = format_ton_amount(2_000_000_000, true, false);
    assert_eq!(fmt2.full, "+2");

    let fmt_neg = format_ton_amount(-500_000_000, false, false);
    assert_eq!(fmt_neg.full, "−0.5");

    let parsed = parse_ton_amount_string("12.345");
    assert_eq!(parsed, Some(12_345_000_000));
}

#[test]
fn test_title_sub_widget_alignment_geometry() {
    let x_left = calculate_title_sub_x(TitleAlign::Left, 400.0, 100.0, 40.0, 16.0, 10.0, 10.0);
    let x_center = calculate_title_sub_x(TitleAlign::Center, 400.0, 100.0, 40.0, 16.0, 10.0, 10.0);
    let x_right = calculate_title_sub_x(TitleAlign::Right, 400.0, 100.0, 40.0, 16.0, 10.0, 10.0);

    assert_eq!(x_center, 150.0);
    assert!(x_left < x_center);
    assert!(x_right > x_center);
}

#[test]
fn test_ttl_media_countdown_and_fraction() {
    let ttl = TtlMediaControls::new(100, 30).with_now(80);
    assert_eq!(ttl.seconds_left(), 20);
    let frac = ttl.fraction_left();
    assert!((frac - (20.0 / 30.0)).abs() < 0.001);
}
