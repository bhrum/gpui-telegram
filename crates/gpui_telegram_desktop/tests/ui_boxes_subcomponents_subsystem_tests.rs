//! Subsystem tests for Module 08 Slice 10 (`ui/boxes` subcomponents).
//! Covers all 22 box subcomponents with exact upstream C++ oracle tests and invariant verifications.

use gpui_telegram_desktop::components::ui::boxes::*;

#[test]
fn test_about_cocoon_constants_and_particles() {
    assert_eq!(about_cocoon_box::K_PARTICLES_COUNT, 50);
    assert_eq!(about_cocoon_box::K_PARTICLE_COLORS, 12);
    assert_eq!(about_cocoon_box::GRADIENT_EDGE, (0x06, 0x11, 0x29));
    assert_eq!(about_cocoon_box::GRADIENT_CENTER, (0x09, 0x16, 0x43));

    let first_color = about_cocoon_box::AboutCocoonBoxBoxes::interpolate_particle_color(0);
    assert_eq!(first_color, about_cocoon_box::GRADIENT_LEFT);

    let last_color = about_cocoon_box::AboutCocoonBoxBoxes::interpolate_particle_color(11);
    assert_eq!(last_color, about_cocoon_box::GRADIENT_RIGHT);

    let mut box_view = AboutCocoonBoxBoxes::new();
    assert_eq!(box_view.features.len(), 3);
    assert!(box_view.is_open);
    box_view.mark_understood();
    assert!(box_view.understood_clicked);
    assert!(!box_view.is_open);
}

#[test]
fn test_boost_box_progress_and_reasons() {
    let mut boost = BoostBoxBoxes::new("Test Channel", false);
    assert_eq!(boost.counters.level, 1);
    assert_eq!(boost.counters.boosts, 3);
    assert_eq!(boost.counters.next_level_boosts, 5);

    // Progress: (3 - 0) / (5 - 0) = 0.6
    let frac = boost.progress_fraction();
    assert!((frac - 0.6).abs() < 0.001);

    // Apply boost
    assert!(boost.apply_boost());
    assert_eq!(boost.counters.boosts, 4);
    assert!(boost.fireworks_active);

    // Level up
    assert!(boost.apply_boost());
    assert_eq!(boost.counters.level, 2);
    assert_eq!(boost.counters.this_level_boosts, 5);

    // Variant reasons
    let reason = boost_box::AskBoostReason::Autotranslate { required_level: 3 };
    boost.selected_reason = Some(reason);
    assert!(matches!(boost.selected_reason, Some(boost_box::AskBoostReason::Autotranslate { required_level: 3 })));
}

#[test]
fn test_calendar_box_geometry_and_days_in_month() {
    assert_eq!(calendar_box::K_DAYS_IN_WEEK, 7);
    assert_eq!(calendar_box::K_THUMBNAILS_PER_PAINT, 4);
    assert_eq!(calendar_box::K_KEEP_MONTHS_AROUND, 2);
    assert_eq!(calendar_box::K_MAX_ROWS, 6);
    assert_eq!(calendar_box::K_MONTHS_COUNT, 12);
    assert_eq!(calendar_box::K_EMPTY_SELECTION, i32::MIN / 2);

    // Leap year checks
    assert_eq!(calendar_box::CalendarBoxBoxes::days_in_month(2024, 2), 29); // Leap year
    assert_eq!(calendar_box::CalendarBoxBoxes::days_in_month(2023, 2), 28); // Common year
    assert_eq!(calendar_box::CalendarBoxBoxes::days_in_month(2000, 2), 29); // 400-year leap
    assert_eq!(calendar_box::CalendarBoxBoxes::days_in_month(1900, 2), 28); // 100-year non-leap
    assert_eq!(calendar_box::CalendarBoxBoxes::days_in_month(2026, 11), 30);

    let mut cal = CalendarBoxBoxes::new();
    assert_eq!(cal.current_month.month, 11);
    cal.next_month();
    assert_eq!(cal.current_month.month, 12);
    cal.next_month();
    assert_eq!(cal.current_month.month, 1);
    assert_eq!(cal.current_month.year, 2027);

    cal.select_day(31);
    assert_eq!(cal.selected_date.unwrap().day, 31);
}

#[test]
fn test_choose_date_time_minimal_schedule() {
    assert_eq!(choose_date_time::K_MINIMAL_SCHEDULE, 10);
    assert_eq!(choose_date_time::SCHEDULE_HEIGHT, 95.0);
    assert_eq!(choose_date_time::SCHEDULE_DATE_WIDTH, 136.0);
    assert_eq!(choose_date_time::SCHEDULE_TIME_WIDTH, 72.0);

    let now = 1000;
    let picker = ChooseDateTimeBoxes::new(now);
    assert!(picker.is_valid_schedule(now)); // 1000 + 600 >= 1010

    let past_picker = ChooseDateTimeBoxes {
        scheduled_timestamp: 1005,
        repeat_option: choose_date_time::ScheduleRepeat::None,
        submit_enabled: true,
        hours: 10,
        minutes: 0,
        title: "Past".to_string(),
    };
    assert!(!past_picker.is_valid_schedule(now)); // 1005 < 1010
}

#[test]
fn test_choose_font_and_language_boxes() {
    assert_eq!(choose_font_box::K_MIN_TEXT_WIDTH, 120);
    let mut font_box = ChooseFontBoxBoxes::new("Inter");
    font_box.select_family("JetBrains Mono");
    assert_eq!(font_box.save(), "JetBrains Mono");
    assert!(font_box.is_saved);

    let mut lang_box = ChooseLanguageBoxBoxes::new(true);
    assert_eq!(lang_box.languages.len(), 5);
    lang_box.search_query = "chin".to_string();
    let filtered = lang_box.filtered_languages();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].id, "zh");

    lang_box.toggle_language("zh");
    assert!(lang_box.selected_ids.contains(&"zh".to_string()));
}
#[test]
fn test_choose_time_and_collectible_info() {
    let ct = ChooseTimeBoxes::new(3665, false); // 1h 1m 5s
    assert_eq!(ct.days, 0);
    assert_eq!(ct.hours, 1);
    assert_eq!(ct.minutes, 1);
    assert_eq!(ct.seconds, 5);
    assert_eq!(ct.total_seconds(), 3665);

    let (d, h, m, s) = ChooseTimeBoxes::seconds_to_components(90061, false); // 1d 1h 1m 1s
    assert_eq!(d, 1);
    assert_eq!(h, 1);
    assert_eq!(m, 1);
    assert_eq!(s, 1);

    // Collectible detection
    let p_type = collectible_info_box::detect_collectible_type("+888 0123 4567");
    assert_eq!(p_type, collectible_info_box::CollectibleType::Phone);
    let u_type = collectible_info_box::detect_collectible_type("telegram_whale");
    assert_eq!(u_type, collectible_info_box::CollectibleType::Username);

    let formatted_p = collectible_info_box::format_collectible_entity(p_type, "+888 0123 4567");
    assert_eq!(formatted_p, "+88801234567");
    let formatted_u = collectible_info_box::format_collectible_entity(u_type, "telegram_whale");
    assert_eq!(formatted_u, "@telegram_whale");

    let coll = CollectibleInfoBoxBoxes::new("+888 0123");
    assert_eq!(coll.formatted_ton_price(), "10.50 TON");
}

#[test]
fn test_confirm_and_confirm_phone_boxes() {
    let mut confirm = ConfirmBoxBoxes::new("Delete", "Delete message?");
    confirm.confirm();
    assert_eq!(confirm.result, Some(confirm_box::ConfirmBoxResult::Confirmed));
    assert!(!confirm.is_open);

    let mut strict_confirm = ConfirmBoxBoxes::new("Alert", "Critical alert");
    strict_confirm.is_strict_cancel = true;
    strict_confirm.dismiss();
    assert_eq!(strict_confirm.result, Some(confirm_box::ConfirmBoxResult::Dismissed));

    let mut phone_box = ConfirmPhoneBoxBoxes::new("+1234567890", 5);
    assert!(!phone_box.enter_digit('1'));
    assert!(!phone_box.enter_digit('2'));
    assert!(!phone_box.enter_digit('3'));
    assert!(!phone_box.enter_digit('4'));
    assert!(phone_box.enter_digit('5')); // 5th digit triggers auto-submit!
    assert!(phone_box.is_waiting_check);
    assert!(phone_box.check_requested);
}

#[test]
fn test_country_select_and_birthday_boxes() {
    assert_eq!(country_select_box::COUNTRY_ROW_HEIGHT, 36.0);
    let mut country_box = CountrySelectBoxBoxes::new(country_select_box::CountrySelectType::Phones);
    country_box.filter_query = "+49".to_string();
    let filtered = country_box.filtered_entries();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].country, "Germany");

    let mut bday_box = EditBirthdayBoxBoxes::new(
        edit_birthday_box::BirthdayData { day: 31, month: 2, year: Some(2024) },
        edit_birthday_box::EditBirthdayType::Edit,
    );
    // February 2024 has 29 days, clamped
    assert_eq!(bday_box.birthday.day, 29);

    bday_box.set_date(31, 4, Some(2020)); // April has 30 days
    assert_eq!(bday_box.birthday.day, 30);
    assert_eq!(bday_box.save().month, 4);
    assert!(bday_box.is_saved);
}

#[test]
fn test_factcheck_and_invite_link_boxes() {
    let mut fc = EditFactcheckBoxBoxes::new("Original factcheck", 100);
    assert_eq!(fc.state, edit_factcheck_box::FactcheckButtonState::Initial);

    fc.update_text("New edited text");
    assert_eq!(fc.state, edit_factcheck_box::FactcheckButtonState::Changed);
    assert_eq!(fc.remaining_chars(), 85);

    fc.update_text("   ");
    assert_eq!(fc.state, edit_factcheck_box::FactcheckButtonState::Removed);
    assert_eq!(fc.save(), None);
    assert!(fc.is_removed);

    // Invite link
    assert_eq!(edit_invite_link::K_MAX_LIMIT, i32::MAX);
    assert_eq!(edit_invite_link::K_HOUR, 3600);
    assert_eq!(edit_invite_link::K_DAY, 86400);
    assert_eq!(edit_invite_link::K_MAX_LABEL_LENGTH, 32);

    let mut invite = EditInviteLinkBoxes::default();
    invite.set_lifetime_preset(1000, "day");
    assert_eq!(invite.fields.expire_date, 1000 + 86400);
    invite.set_label("Very long label text that exceeds thirty two characters limit easily");
    assert_eq!(invite.fields.label.len(), 32);

    let mut session = EditInviteLinkSessionBoxes::new(true);
    assert!(session.toggle_subscription(true));
    assert!(session.subscription_enabled);
    session.set_monthly_amount(500);
    assert_eq!(session.monthly_credits_amount, 500);
}

#[test]
fn test_emoji_stake_qr_rate_and_time_picker() {
    assert_eq!(emoji_stake_box::K_PER_ROW, 4);
    let mut stake = EmojiStakeBoxBoxes::new(100, 1000);
    assert!(stake.can_submit());
    assert!(stake.submit());
    assert_eq!(stake.submitted_stake, Some(100));

    // Peer QR
    assert_eq!(peer_qr_box::K_TOAST_DURATION_MS, 2800);
    assert_eq!(peer_qr_box::K_SHARE_CUTOFF_MS, 3000);
    assert_eq!(peer_qr_box::K_MAX_EMOJI_PICKS, 6);
    let mut qr = PeerQrBoxBoxes::new("https://t.me/telegram", "Telegram");
    assert_eq!(qr.copy_link(), "https://t.me/telegram");
    assert!(qr.toast_active);

    // Rate call
    assert_eq!(rate_call_box::K_MAX_RATING, 5);
    assert_eq!(rate_call_box::K_RATE_CALL_COMMENT_LENGTH_MAX, 200);
    let mut rate = RateCallBoxBoxes::new();
    rate.set_rating(5);
    rate.set_comment("Crystal clear audio call");
    assert!(rate.submit());
    assert_eq!(rate.submitted_result.unwrap().rating, 5);

    // Report box
    assert_eq!(report_box_graphics::K_REPORT_REASON_LENGTH_MAX, 512);
    let mut report = ReportBoxGraphicsBoxes::new(report_box_graphics::ReportSource::Story);
    report.select_reason(report_box_graphics::ReportReason::Spam);
    assert!(report.submit());

    // Show or premium
    assert_eq!(show_or_premium_box::K_SHOW_OR_LINE_OPACITY, 0.3);
    let mut show_box = ShowOrPremiumBoxBoxes::new(show_or_premium_box::ShowOrPremium::LastSeen, "Bob");
    show_box.choose_just_show();
    assert_eq!(show_box.choice, Some(show_or_premium_box::ShowOrPremiumChoice::JustShow));

    // Single choice
    let mut single = SingleChoiceBoxBoxes::new("Mode", vec!["A".to_string(), "B".to_string()], 0);
    single.choose_index(1);
    assert_eq!(single.selected_text(), Some("B"));
    assert!(single.is_closed);

    // Time picker
    let intervals = time_picker_box::default_time_picker_values();
    assert_eq!(intervals.len(), 16);
    assert_eq!(intervals[0], 900); // 15m
    assert_eq!(intervals[8], 86400); // 1d

    // Nearest search tie-breaker
    let nearest_1 = time_picker_box::TimePickerBoxBoxes::find_nearest_index(&intervals, 80000); // closer to 86400 (diff 6400) than 43200 (diff 36800)
    assert_eq!(nearest_1, 8); // 86400

    let mut tp = TimePickerBoxBoxes::new(86400);
    assert_eq!(tp.confirm(), 86400);
    assert!(tp.is_confirmed);
}
