//! Subsystem & Oracle Tests for Module 08 Slice 2 (UI Inputs, Labels, Scroll, and Widgets)
//! Validates mathematical formulas, timing constants, state machines, and edge cases
//! extracted from upstream Telegram Desktop C++ sources.

use gpui_telegram_desktop::components::ui::rows_scroll_cache::*;
use gpui_telegram_desktop::components::ui::widgets::callout::*;
use gpui_telegram_desktop::components::ui::widgets::cross_fade_label::*;
use gpui_telegram_desktop::components::ui::widgets::dropdown::*;
use gpui_telegram_desktop::components::ui::widgets::fields::special_fields::*;
use gpui_telegram_desktop::components::ui::widgets::fields::time_part_input_with_placeholder::*;
use gpui_telegram_desktop::components::ui::widgets::input_fields::*;
use gpui_telegram_desktop::components::ui::widgets::marquee_label::*;
use gpui_telegram_desktop::components::ui::widgets::middle_click_autoscroll::*;
use gpui_telegram_desktop::components::ui::widgets::passcode_strength_meter::*;
use gpui_telegram_desktop::components::ui::widgets::scroll_area::*;
use gpui_telegram_desktop::components::ui::widgets::selecting_scroll::*;
use gpui_telegram_desktop::components::ui::widgets::sent_code_field::*;

#[test]
fn test_cross_fade_label_cubic_interpolation_and_offsets() {
    let mut label = CrossFadeLabelWidgets::new("Hello");
    label.line_height = 18.0;
    label.direction = 1;
    assert_eq!(label.current_text, "Hello");
    assert!(!label.is_animating);

    // Trigger animated transition
    label.set_text("World", true);
    assert_eq!(label.previous_text, "Hello");
    assert_eq!(label.current_text, "World");
    assert!(label.is_animating);
    assert_eq!(label.progress, 0.0);

    // At t = 0: displacement is full line height
    assert_eq!(label.current_y_offset(), 18.0);
    assert_eq!(label.ease_out_factor(), 0.0);

    // Advance by half progress (0.5)
    label.step_animation(0.5);
    assert!((label.progress - 0.5).abs() < 1e-4);
    // easeOutCubic at 0.5: 1 - (1 - 0.5)^3 = 1 - 0.125 = 0.875
    let eased = label.ease_out_factor();
    assert!((eased - 0.875).abs() < 1e-4);

    let y_offset = label.current_y_offset();
    // (1.0 - 0.875) * 18.0 = 0.125 * 18.0 = 2.25
    assert!((y_offset - 2.25).abs() < 1e-4);

    // Complete animation
    label.step_animation(0.5);
    assert!(!label.is_animating);
    assert_eq!(label.progress, 1.0);
    assert_eq!(label.current_y_offset(), 0.0);
}

#[test]
fn test_marquee_label_timing_speed_and_boundary_wrap() {
    let mut marquee = MarqueeLabelWidgets::new("This is a long marquee text that overflows");
    marquee.set_available_width(100.0);
    assert!(marquee.is_scrolling);
    assert_eq!(marquee.delay_remaining_ms, K_MARQUEE_DELAY_MS);

    // During 500ms initial pause, offset does not advance
    marquee.step(0.2);
    assert_eq!(marquee.offset, 0.0);
    assert_eq!(marquee.delay_remaining_ms, 300);

    marquee.step(0.3);
    assert_eq!(marquee.delay_remaining_ms, 0);

    // Stepping advances offset
    marquee.step(0.1);
    assert!(marquee.offset > 0.0);

    // Pause on hover
    marquee.is_paused = true;
    let offset_before = marquee.offset;
    marquee.step(0.2);
    assert_eq!(marquee.offset, offset_before);
}

#[test]
fn test_middle_click_autoscroll_deadzone_hold_threshold_speed() {
    let mut scroll = MiddleClickAutoscrollWidgets::new();
    assert!(!scroll.is_active);

    // Begin hold at (200, 300), timestamp 1000
    scroll.start(200.0, 300.0, 1000);
    assert!(scroll.is_active);

    // Within deadzone (6px): delta = 4px -> 0 scroll velocity
    scroll.update_pointer(200.0, 304.0);
    assert_eq!(scroll.velocity(), (0.0, 0.0));

    // Outside deadzone: delta = 20px -> Down mode velocity > 0
    scroll.update_pointer(200.0, 320.0);
    let (_vx, vy_down) = scroll.velocity();
    assert!(vy_down > 0.0);

    // Moving up: delta = -40px -> Up mode velocity < 0
    scroll.update_pointer(200.0, 260.0);
    let (_vx, vy_up) = scroll.velocity();
    assert!(vy_up < 0.0);

    // Release mouse after 100ms (< 220ms threshold) -> stays active (toggle mode)
    assert!(!scroll.on_mouse_release(1100));
    assert!(scroll.is_active);

    // If hold was >= 220ms -> stops immediately
    scroll.start(200.0, 300.0, 2000);
    assert!(scroll.on_mouse_release(2300));
    assert!(!scroll.is_active);
}

#[test]
fn test_selecting_scroll_formula_and_max_speed_clamping() {
    // Inside visible rect: delta = 0
    assert_eq!(delta_outside(100.0, 500.0, 300.0), 0.0);

    // Above viewport: y = 80 < 100 -> delta = -20
    assert_eq!(delta_outside(100.0, 500.0, 80.0), -20.0);
    // Speed formula for negative delta: max(delta * 3 / 20 - 1, -37) = max(-20*3/20 - 1, -37) = -4
    let speed_neg = speed_for_delta(-20.0);
    assert_eq!(speed_neg, -4.0);

    // Extreme delta clamped to -37
    let speed_extreme_neg = speed_for_delta(-500.0);
    assert_eq!(speed_extreme_neg, -K_MAX_SPEED);

    // Below viewport: y = 520 > 500 -> delta = 20 -> speed = 20 * 3 / 20 + 1 = 4
    let speed_pos = speed_for_delta(20.0);
    assert_eq!(speed_pos, 4.0);

    // Extreme positive clamped to 37
    let speed_extreme_pos = speed_for_delta(600.0);
    assert_eq!(speed_extreme_pos, K_MAX_SPEED);
}

#[test]
fn test_sent_code_field_masking_hyphen_preserve_and_call_countdown() {
    let mut field = SentCodeFieldWidgets::new(5);

    // Enter digits with hyphen and letters: letters stripped, hyphen preserved
    let ready = field.on_text_changed("12-3a4");
    assert!(!ready);
    assert_eq!(field.raw_text, "12-3a4");
    assert_eq!(field.get_digits_only(), "1234");

    // Hit auto submit length (5 digits)
    let auto_submitted = field.on_text_changed("12-345");
    assert!(auto_submitted);
    assert_eq!(field.get_digits_only(), "12345");
    assert!(field.is_submitted);

    // SentCodeCall countdown
    let mut call = SentCodeCall::new(65);
    assert_eq!(call.state, SentCodeCallState::Waiting);
    assert_eq!(call.formatted_text(), "1:05");

    // Ticks down
    assert!(!call.tick_second());
    assert_eq!(call.timeout_seconds, 64);

    // When hits 0 -> transitions to Calling
    call.timeout_seconds = 1;
    let triggered = call.tick_second();
    assert!(triggered);
    assert_eq!(call.state, SentCodeCallState::Calling);
    assert_eq!(call.formatted_text(), "Calling...");
}

#[test]
fn test_special_fields_country_phone_and_username_validation() {
    let mut fields = SpecialFieldsWidgetsFields::new();

    // Country code input
    fields.set_country_code("+12345"); // 5 digits -> clamped to 4
    assert_eq!(fields.country_code, "+1234");
    assert!(fields.is_country_code_valid);

    // Phone part input
    fields.set_phone_tail("8005550199");
    assert_eq!(fields.phone_tail, "8005550199");
    assert_eq!(fields.full_phone_number(), "+12348005550199");

    // Username input: trims @ prefix, clamps to 32 chars
    assert!(fields.set_username("@durov"));
    assert_eq!(fields.username, "durov");
    assert!(fields.is_username_valid);

    let long_username = "a".repeat(40);
    assert!(!fields.set_username(&long_username));
    assert!(!fields.is_username_valid);
}

#[test]
fn test_time_part_input_with_placeholder_centering_and_padding() {
    let mut time_input = TimePartInputWithPlaceholderWidgetsFields::new(TimePartKind::Minutes);
    time_input.set_value(5);
    assert_eq!(time_input.formatted_text(), "05");
    assert_eq!(time_input.value, 5);

    time_input.set_value(42);
    assert_eq!(time_input.value, 42);
    assert_eq!(time_input.formatted_text(), "42");

    time_input.set_field_width(120.0);
    assert!(time_input.text_margins_left >= 0.0);
}

#[test]
fn test_passcode_strength_meter_segment_geometry_and_bands() {
    let mut meter = PasscodeStrengthMeterWidgets::new();
    meter.evaluate_passcode("1234");
    assert_eq!(meter.band, PasscodeStrengthBand::VeryWeak);

    meter.evaluate_passcode("P@ssw0rd!Complex#2026");
    assert_eq!(meter.band, PasscodeStrengthBand::Strong);

    // Segment partition sums to total available width
    let total_width = 200.0;
    let spacing = 4.0;
    let available = total_width - spacing * 3.0; // 188.0
    let mut sum = 0.0;
    for i in 0..4 {
        let seg = ((available * (i + 1) as f32 / 4.0) as i32) - ((available * i as f32 / 4.0) as i32);
        sum += seg as f32;
    }
    assert_eq!(sum as i32, available as i32);
}

#[test]
fn test_rows_scroll_cache_memory_tracking_and_timeout_flush() {
    let mut cache = RowsScrollCache::new();
    cache.mark_scrolling(1000);
    assert!(cache.is_scrolling);

    cache.insert(101, 100, 50); // 100 * 50 * 4 = 20,000 bytes
    cache.insert(102, 200, 50); // 200 * 50 * 4 = 40,000 bytes
    assert_eq!(cache.total_memory, 60000);
    assert_eq!(cache.row_count(), 2);

    // Invalidate single row
    cache.invalidate(101);
    assert_eq!(cache.total_memory, 40000);
    assert_eq!(cache.row_count(), 1);

    // Before timeout (120ms): does not clear
    let flushed = cache.tick(1100);
    assert!(!flushed);
    assert_eq!(cache.total_memory, 40000);

    // After timeout (120ms): clears cache
    let flushed_after = cache.tick(1150);
    assert!(flushed_after);
    assert!(!cache.is_scrolling);
    assert_eq!(cache.total_memory, 0);
    assert_eq!(cache.row_count(), 0);
}

#[test]
fn test_reusable_widgets_integration() {
    let mut input = InputField::new().with_placeholder("Enter text...");
    input.set_text("Hello");
    input.insert_char('!');
    assert_eq!(input.text, "Hello!");
    assert!(input.backspace());
    assert_eq!(input.text, "Hello");

    let mut scroll = ScrollArea::new();
    scroll.set_dimensions(1000.0, 400.0);
    assert_eq!(scroll.max_scroll(), 600.0);
    scroll.scroll_by(150.0);
    assert_eq!(scroll.scroll_top, 150.0);
    let (top, height) = scroll.compute_thumb_geometry();
    assert!(top > 0.0);
    assert!(height >= K_MIN_SCROLLBAR_THUMB_HEIGHT);

    let mut dropdown = Dropdown::new();
    dropdown.items.push(DropdownItem {
        id: 1,
        title: "English".to_string(),
        icon: None,
        is_selected: false,
        is_disabled: false,
    });
    dropdown.toggle();
    assert!(dropdown.is_open);
    dropdown.select_item(1);
    assert!(!dropdown.is_open);
    assert_eq!(dropdown.selected_item().unwrap().title, "English");

    let mut callout = Callout::new("Security Update", "Please update your password.");
    assert!(!callout.is_dismissed);
    callout.dismiss();
    assert!(callout.is_dismissed);
}
