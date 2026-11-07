//! Subsystem Tests for Module 08 Slice 1: UI Core Widgets
//! Validates authentic behavior, mathematical constants, and state transitions
//! matching upstream Telegram Desktop C++ implementations.

use gpui_telegram_desktop::components::ui::widgets::*;

#[test]
fn test_chat_filters_tabs_mode_resolvers() {
    use chat_filters_tabs_mode::*;
    assert_eq!(horizontal_chats_filters_tabs_mode(ChatsFiltersTabsMode::Default), ChatsFiltersTabsMode::TextOnly);
    assert_eq!(horizontal_chats_filters_tabs_mode(ChatsFiltersTabsMode::IconsOnly), ChatsFiltersTabsMode::IconsOnly);
    assert_eq!(vertical_chats_filters_tabs_mode(ChatsFiltersTabsMode::Default), ChatsFiltersTabsMode::TextAndIcons);

    let mut w = ChatFiltersTabsModeWidgets::new(ChatsFiltersTabsMode::Default, false);
    assert_eq!(w.effective_mode(), ChatsFiltersTabsMode::TextOnly);
    assert!(w.shows_text());
    assert!(!w.shows_icons());

    w.set_vertical(true);
    assert_eq!(w.effective_mode(), ChatsFiltersTabsMode::TextAndIcons);
    assert!(w.shows_text());
    assert!(w.shows_icons());
}

#[test]
fn test_chat_filters_tabs_slider_unread_and_locked() {
    use chat_filters_tabs_slider::*;
    let mut slider = ChatFiltersTabsSliderWidgets::new();
    let sections = vec![
        FilterTabSection::new(1, "All Chats"),
        FilterTabSection::new(2, "Channels"),
        FilterTabSection::new(3, "Work"),
    ];
    slider.set_sections(sections);
    assert_eq!(slider.sections.len(), 3);

    // Unread capping: 0 -> None, 5 -> "5", 150 -> "99+"
    slider.set_unread_count(0, 0, false);
    assert_eq!(slider.sections[0].unread_label(), None);

    slider.set_unread_count(1, 5, false);
    assert_eq!(slider.sections[1].unread_label(), Some("5".to_string()));

    slider.set_unread_count(2, 150, true);
    assert_eq!(slider.sections[2].unread_label(), Some(UNREAD_MAX_STRING.to_string()));

    // Locked folder separator positioning
    slider.set_locked_from(Some(2));
    assert!(slider.calculate_locked_from_x().is_some());
}

#[test]
fn test_chat_filters_tabs_slider_reorder_scroll_factor() {
    use chat_filters_tabs_slider_reorder::*;
    let mut reorder = ChatFiltersTabsSliderReorderWidgets::new();
    assert_eq!(reorder.scroll_factor, K_SCROLL_FACTOR);

    reorder.start(4, 1, 100.0);
    assert!(reorder.is_active);
    assert_eq!(reorder.dragged_index, Some(1));

    // Pointer near left edge (< 30px) triggers negative auto-scroll
    reorder.mouse_move(10.0, &[80.0, 80.0, 80.0, 80.0], 400.0);
    assert!(reorder.scroll_offset < 0.0);

    reorder.cancel();
    assert!(!reorder.is_active);
}

#[test]
fn test_chat_filters_tabs_strip_selection() {
    use chat_filters_tabs_strip::*;
    let mut strip = ChatFiltersTabsStripWidgets::new();
    strip.set_folders(vec![
        ChatFolderTabItem {
            filter_id: 101,
            title: "Folder A".to_string(),
            icon_name: None,
            unread_count: 2,
            is_muted: false,
            is_locked: false,
        },
        ChatFolderTabItem {
            filter_id: 102,
            title: "Folder B".to_string(),
            icon_name: None,
            unread_count: 0,
            is_muted: false,
            is_locked: true,
        },
    ]);

    strip.select_filter(102);
    assert_eq!(strip.active_filter_id, Some(102));
    assert_eq!(strip.slider.active_index, 1);
}

#[test]
fn test_continuous_sliders_wheel_and_keyboard_math() {
    use continuous_sliders::*;
    let mut slider = ContinuousSlidersWidgets::new();
    slider.set_value(0.5);

    // Mouse press on track
    let val = slider.on_mouse_press(150.0, 100.0, 100.0);
    assert_eq!(val, 0.5);

    // Wheel delta: step = 120, coef = 1/1200
    // On Mac, delta_y positive increases seek
    let new_val = slider.on_wheel_delta(0.0, 120.0, true, 1000);
    assert!((new_val - (0.5 + 120.0 * WHEEL_COEF)).abs() < 1e-4);

    // Wheel debouncing timeout
    assert!(!slider.check_wheel_timeout(1500));
    assert!(slider.check_wheel_timeout(2100));

    // Keyboard step
    let stepped = slider.step_by(0.1);
    assert!(stepped > 0.5);
}

#[test]
fn test_discrete_sliders_ranges_and_selection() {
    use discrete_sliders::*;
    let mut slider = DiscreteSlidersWidgets::new();
    slider.set_sections(&["Slow", "Medium", "Fast"], 300.0);
    assert_eq!(slider.sections.len(), 3);
    assert_eq!(slider.get_index_from_position(50.0), 0);
    assert_eq!(slider.get_index_from_position(150.0), 1);
    assert_eq!(slider.get_index_from_position(250.0), 2);

    slider.on_mouse_press(250.0);
    assert_eq!(slider.active_index, 2);
}

#[test]
fn test_slider_natural_width() {
    use slider_natural_width::*;
    let mut nw = SliderNaturalWidthWidgets::new();
    let computed = nw.compute_natural_width(&[40.0, 50.0, 60.0], 8.0);
    // 40 + 50 + 60 + 2*8 + 2*16 = 150 + 16 + 32 = 198
    assert_eq!(computed, 198.0);
}

#[test]
fn test_sliding_tabs_direction() {
    use sliding_tabs::*;
    let mut tabs = SlidingTabsWidgets::new(3);
    assert_eq!(tabs.count, 3);
    assert_eq!(tabs.shown_index, 0);

    // Slide right from 0 to 2
    tabs.show_tab(2, SlideTransitionType::Animated);
    assert_eq!(tabs.shown_index, 2);
    assert_eq!(tabs.slide_direction, Some(SlideDirection::Right));
    assert!(tabs.is_animating);

    // Step animation to completion
    assert!(tabs.step_animation(1.0));
    assert!(!tabs.is_animating);

    // Slide left from 2 to 1
    tabs.show_tab(1, SlideTransitionType::Animated);
    assert_eq!(tabs.slide_direction, Some(SlideDirection::Left));
}

#[test]
fn test_gradient_round_button_glare() {
    use gradient_round_button::*;
    let mut btn = GradientRoundButtonWidgets::new("Boost Channel");
    btn.start_glare();
    assert!(btn.glare_active);
    assert_eq!(btn.glare_progress, 0.0);

    btn.step_glare(0.5);
    assert_eq!(btn.glare_progress, 0.5);
    let x = btn.compute_glare_x(100.0);
    // (-32) + (100 + 64) * 0.5 = -32 + 82 = 50
    assert_eq!(x, 50.0);

    btn.step_glare(0.6);
    assert!(!btn.glare_active);
}

#[test]
fn test_glare_tooltip_timing() {
    use glare_tooltip::*;
    let mut tip = GlareTooltipWidgets::new("Feature unlocked");
    tip.fade(true);
    assert!(tip.is_visible);
    assert!(tip.glare_active);

    let dur = tip.calculate_glare_duration_ms(100.0, 20.0);
    // glare_size = 60, glare_range = 160, dur = 160 * 320 / 60 = 853ms
    assert!(dur > 800 && dur < 900);
}
#[test]
fn test_marquee_label_overflow_and_ramps() {
    use marquee_label::*;
    let mut ml = MarqueeLabelWidgets::new("Very Long Track Title Scrolling Continuously");
    ml.set_available_width(100.0);
    assert!(ml.is_scrolling);

    // Initial pause delay
    ml.step(0.1);
    assert_eq!(ml.offset, 0.0);

    // Step past delay (0.5s)
    ml.step(0.5);
    ml.step(0.1);
    assert!(ml.offset > 0.0);

    // Fade ramps
    assert!(ml.edge_fade_opacity(0.0) < 0.1);
    assert_eq!(ml.edge_fade_opacity(50.0), 1.0);
}

#[test]
fn test_cross_fade_label_displacement() {
    use cross_fade_label::*;
    let mut cfl = CrossFadeLabelWidgets::new("Initial Value");
    cfl.set_text("Updated Value", true);
    assert!(cfl.is_animating);
    assert_eq!(cfl.previous_text, "Initial Value");
    assert_eq!(cfl.current_text, "Updated Value");

    let y_offset = cfl.current_y_offset();
    assert_eq!(y_offset, 18.0); // full line height displacement

    cfl.step_animation(1.0);
    assert!(!cfl.is_animating);
    assert_eq!(cfl.current_y_offset(), 0.0);
}

#[test]
fn test_horizontal_fit_container_distribution() {
    use horizontal_fit_container::*;
    let mut container = HorizontalFitContainerWidgets::new(10.0);
    container.set_container_size(320.0, 40.0);
    container.add("Cancel");
    container.add("OK");
    // count = 2, total_spacing = 10, width = (320 - 10) / 2 = 155
    assert_eq!(container.buttons[0].width, 155.0);
    assert_eq!(container.buttons[1].width, 155.0);
    assert_eq!(container.buttons[1].x, 165.0);
}

#[test]
fn test_level_meter_active_segments() {
    use level_meter::*;
    let mut meter = LevelMeterWidgets::new();
    meter.line_count = 10;
    meter.set_value(0.5);

    // lines 0..4 (1/10 to 5/10) active, lines 5..9 inactive
    assert!(meter.is_line_active(0));
    assert!(meter.is_line_active(4));
    assert!(!meter.is_line_active(5));
}

#[test]
fn test_passcode_strength_meter_entropy() {
    use passcode_strength_meter::*;
    let mut meter = PasscodeStrengthMeterWidgets::new();
    meter.evaluate_passcode("123");
    assert_eq!(meter.advice, PasscodeStrengthAdvice::TooShort);

    meter.evaluate_passcode("11111");
    assert_eq!(meter.advice, PasscodeStrengthAdvice::Repeat);

    meter.evaluate_passcode("1234");
    assert_eq!(meter.advice, PasscodeStrengthAdvice::Sequence);

    meter.evaluate_passcode("Telegram@2026Secure!");
    assert_eq!(meter.band, PasscodeStrengthBand::Strong);
    assert_eq!(meter.advice, PasscodeStrengthAdvice::Fine);
}

#[test]
fn test_multi_select_tags_and_backspace() {
    use multi_select::*;
    let mut ms = MultiSelectWidgets::new();
    ms.add_item(1001, "Alice");
    ms.add_item(1002, "Bob");
    assert_eq!(ms.items.len(), 2);

    // Backspace with non-empty query does nothing to items
    ms.set_query("ch");
    assert_eq!(ms.on_backspace(), None);

    // Backspace with empty query removes last item (Bob)
    ms.set_query("");
    assert_eq!(ms.on_backspace(), Some(1002));
    assert_eq!(ms.items.len(), 1);
}

#[test]
fn test_expandable_peer_list_and_participants_check() {
    use expandable_peer_list::*;
    use participants_check_view::*;
    let mut list = ExpandablePeerListWidgets::new();
    list.set_participants(vec![
        ExpandablePeerItem { peer_id: 1, title: "Peer 1".to_string(), is_selected: false },
        ExpandablePeerItem { peer_id: 2, title: "Peer 2".to_string(), is_selected: false },
    ]);
    assert_eq!(list.expander_label(), "Expand (2)");
    list.toggle_expanded();
    assert_eq!(list.expander_label(), "Collapse (2)");

    let mut check = ParticipantsCheckViewWidgets::new("Admins (5)", false);
    assert!(!check.is_checked);
    check.set_checked(true);
    assert!(check.is_checked);
    check.toggle_expanded();
    assert_eq!(check.arrow_rotation, 180.0);
}

#[test]
fn test_peer_bubble_and_selecting_scroll() {
    use peer_bubble::*;
    use selecting_scroll::*;
    let bubble = PeerBubbleWidgets::new(42, "Channel News");
    assert!(bubble.natural_width > 50.0);

    // Selecting scroll speed formula
    assert_eq!(delta_outside(100.0, 500.0, 50.0), -50.0);
    let speed = speed_for_delta(-50.0);
    // delta < 0: max(-50 * 3 / 20 - 1, -37) = max(-8.5, -37) = -8.5
    assert!((speed - (-8.5)).abs() < 1e-4);

    let max_speed = speed_for_delta(500.0);
    assert_eq!(max_speed, K_MAX_SPEED);
}

#[test]
fn test_middle_click_autoscroll_deadzone_and_drum_picker() {
    use middle_click_autoscroll::*;
    use vertical_drum_picker::*;
    let mut auto = MiddleClickAutoscrollWidgets::new();
    auto.start(200.0, 200.0, 1000);
    auto.update_pointer(203.0, 200.0); // within 6px deadzone
    assert_eq!(auto.velocity(), (0.0, 0.0));

    auto.update_pointer(250.0, 200.0); // 50px delta -> outside deadzone
    let (vx, vy) = auto.velocity();
    assert!(vx > 0.0);
    assert_eq!(vy, 0.0);

    // Vertical drum perspective scale
    let center_scale = VerticalDrumPickerWidgets::compute_perspective_scale(0.0);
    assert_eq!(center_scale, 1.0);
    let edge_scale = VerticalDrumPickerWidgets::compute_perspective_scale(1.0);
    assert_eq!(edge_scale, K_MIN_Y_SCALE);
}

#[test]
fn test_sent_code_special_fields_and_color_editor() {
    use sent_code_field::*;
    use fields::special_fields::*;
    use fields::time_part_input_with_placeholder::*;
    use color_editor::*;

    let mut sc = SentCodeFieldWidgets::new(5);
    assert!(!sc.on_text_changed("12-3"));
    assert_eq!(sc.get_digits_only(), "123");
    assert!(sc.on_text_changed("12-345"));
    assert_eq!(sc.get_digits_only(), "12345");
    assert!(sc.is_submitted);

    let mut sf = SpecialFieldsWidgetsFields::new();
    sf.set_country_code("+12345");
    assert_eq!(sf.country_code, "+1234");
    sf.set_phone_tail("987654321");
    assert_eq!(sf.full_phone_number(), "+1234987654321");
    assert!(sf.set_username("@telegram_user"));
    assert_eq!(sf.username, "telegram_user");
    assert!(!sf.set_username("1bad")); // cannot start with digit

    let mut tp = TimePartInputWithPlaceholderWidgetsFields::new(TimePartKind::Minutes);
    tp.set_value(45);
    assert_eq!(tp.value, 45);
    tp.set_value(100);
    assert_eq!(tp.value, 60);

    let mut ce = ColorEditorWidgets::new_rgb(255, 0, 0);
    assert_eq!(ce.hex_string, "#FF0000");
    assert_eq!(ce.hsb.hue, 0.0);
    assert_eq!(ce.hsb.saturation, 1.0);
    assert_eq!(ce.hsb.brightness, 1.0);

    assert!(ce.set_hex("#2AABEE"));
    assert_eq!(ce.red, 0x2A);
    assert_eq!(ce.green, 0xAB);
    assert_eq!(ce.blue, 0xEE);
}
