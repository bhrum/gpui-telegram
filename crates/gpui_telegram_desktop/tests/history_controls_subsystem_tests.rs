//! Subsystem tests for Module 06 Slice 8: History View Controls Subsystem (`SPEC-06-HISTORY-SLICE-8`).
//! Verifies voice recording gestures, timers, character limits, AI sparkles, forward options, and zero boilerplate.

use gpui_telegram_desktop::views::composer::controls::{
    calc_sparkle_opacities, format_character_limit, AiTooltipManager, ComposeControlsMode,
    ComposeControlsModel, DraftOptionsFsm, DraftSection, ForwardOptions, ForwardPanelModel,
    ForwardedItem, SendButtonType, StopType, VoiceRecordBar, VoiceRecordState, WriteRestriction,
    WriteRestrictionType, AI_ANIMATION_DURATION_MS, AUDIO_VOICE_MAX_LENGTH_SECS,
    AUDIO_VOICE_UPDATE_VIEW_MS, CHARACTERS_OVER_LIMIT_CAP, LOCK_HEIGHT_PX,
    NAME_NO_CAPTIONS_VERSION, NAME_WITH_CAPTIONS_VERSION, SLIDE_CANCEL_THRESHOLD_PX,
    UNICODE_MINUS,
};

use gpui_telegram_desktop::views::stage::view::controls::{
    compose_controls_common::ComposeControlsCommonViewControls,
    history_view_bot_menu_button::HistoryViewBotMenuButtonViewControls,
    history_view_bottom_controls::{
        BottomControlsAction, HistoryViewBottomControlsViewControls,
    },
    history_view_characters_limit::{
        count_digits, HistoryViewCharactersLimitViewControls,
    },
    history_view_compose_ai_button::HistoryViewComposeAiButtonViewControls,
    history_view_compose_ai_tooltip::HistoryViewComposeAiTooltipViewControls,
    history_view_compose_controls::HistoryViewComposeControlsViewControls,
    history_view_compose_media_edit_manager::HistoryViewComposeMediaEditManagerViewControls,
    history_view_compose_search::HistoryViewComposeSearchViewControls,
    history_view_compose_stash::HistoryViewComposeStashViewControls,
    history_view_compose_stash_hint::HistoryViewComposeStashHintViewControls,
    history_view_draft_options::HistoryViewDraftOptionsViewControls,
    history_view_forward_panel::HistoryViewForwardPanelViewControls,
    history_view_rich_draft_preview::HistoryViewRichDraftPreviewViewControls,
    history_view_suggest_options::HistoryViewSuggestOptionsViewControls,
    history_view_top_controls::HistoryViewTopControlsViewControls,
    history_view_ttl_button::{format_ttl_tiny, HistoryViewTtlButtonViewControls},
    history_view_video_cover_uploader::HistoryViewVideoCoverUploaderViewControls,
    history_view_voice_record_bar::HistoryViewVoiceRecordBarViewControls,
    history_view_voice_record_button::HistoryViewVoiceRecordButtonViewControls,
    history_view_webpage_processor::HistoryViewWebpageProcessorViewControls,
};

#[test]
fn test_voice_record_gesture_lock_and_cancel_thresholds() {
    let mut bar = VoiceRecordBar::new();
    assert_eq!(bar.state, VoiceRecordState::Idle);

    // 1. Start holding voice record
    bar.start_holding(false);
    assert!(bar.is_active());
    assert!(!bar.is_locked());

    // 2. Test upward drag progress calculation
    assert_eq!(bar.calc_lock_progress(0.0), 0.0);
    assert_eq!(bar.calc_lock_progress(20.0), 0.0); // dragging down does not increase lock progress
    assert!((bar.calc_lock_progress(-LOCK_HEIGHT_PX) - 1.0).abs() < 1e-4);
    assert!((bar.calc_lock_progress(-LOCK_HEIGHT_PX * 0.5) - 0.5).abs() < 1e-4);

    // 3. Upward drag triggers lock state when reaching height or threshold
    bar.handle_drag(0.0, -LOCK_HEIGHT_PX);
    assert!(bar.is_locked());
    match &bar.state {
        VoiceRecordState::Locked { is_video, .. } => {
            assert!(!*is_video);
        }
        _ => panic!("Expected Locked state"),
    }

    // 4. Test slide-to-cancel threshold
    let mut cancel_bar = VoiceRecordBar::new();
    cancel_bar.start_holding(false);
    // Drag left past cancel threshold (-80px)
    cancel_bar.handle_drag(SLIDE_CANCEL_THRESHOLD_PX - 10.0, 0.0);
    match &cancel_bar.state {
        VoiceRecordState::Discarded { reason } => {
            assert!(reason.contains("Slid left"));
        }
        _ => panic!("Expected Discarded state after dragging left"),
    }
}

#[test]
fn test_voice_record_duration_timer_and_max_length() {
    let mut bar = VoiceRecordBar::new();
    bar.start_holding(false);

    // Tick 200ms increments
    bar.tick_duration(AUDIO_VOICE_UPDATE_VIEW_MS);
    bar.record_updated(350, 42);
    bar.record_updated(500, 78);

    match &bar.state {
        VoiceRecordState::Holding {
            duration_ms,
            samples,
            level,
            ..
        } => {
            assert_eq!(*duration_ms, 200);
            assert_eq!(*level, 500);
            assert_eq!(samples.len(), 2);
        }
        _ => panic!("Expected Holding state"),
    }

    // Advance beyond maximum length (6,000s = 6,000,000ms)
    bar.tick_duration(7_000_000);
    match &bar.state {
        VoiceRecordState::Holding { duration_ms, .. } => {
            assert_eq!(*duration_ms, AUDIO_VOICE_MAX_LENGTH_SECS * 1000);
        }
        _ => panic!("Expected Holding state capped at max duration"),
    }
    assert_eq!(bar.formatted_duration(), "100:00");

    // Toggle TTL view once
    assert!(!bar.view_once);
    bar.toggle_view_once();
    assert!(bar.view_once);

    // Explicit send
    bar.stop(StopType::Send);
    match &bar.state {
        VoiceRecordState::Sent {
            duration_ms,
            view_once,
            waveform,
            ..
        } => {
            assert_eq!(*duration_ms, 6_000_000);
            assert!(*view_once);
            assert_eq!(waveform.len(), 2);
        }
        _ => panic!("Expected Sent state"),
    }
}

#[test]
fn test_characters_limit_label_formatting() {
    let max_len = 4096;

    // 1. Within limit: displays remaining characters, error flag false
    let (rem1, err1) = format_character_limit(100, max_len);
    assert_eq!(rem1, "3996");
    assert!(!err1);

    let (rem2, err2) = format_character_limit(4096, max_len);
    assert_eq!(rem2, "0");
    assert!(!err2);

    // 2. Exceeded limit: displays unicode minus and count, error flag true
    let (exc1, err_exc1) = format_character_limit(4100, max_len);
    assert!(err_exc1);
    assert_eq!(exc1, format!("{}4", UNICODE_MINUS));

    // 3. Exceeded by more than 999: capped at 999
    let (exc2, err_exc2) = format_character_limit(6000, max_len);
    assert!(err_exc2);
    assert_eq!(exc2, format!("{}{}", UNICODE_MINUS, CHARACTERS_OVER_LIMIT_CAP));

    // 4. Test logarithmic digit counting logic
    assert_eq!(count_digits(0), 1);
    assert_eq!(count_digits(5), 1);
    assert_eq!(count_digits(42), 2);
    assert_eq!(count_digits(999), 3);
    assert_eq!(count_digits(1000), 4);
}

#[test]
fn test_compose_ai_sparkle_animation_cycle() {
    assert_eq!(AI_ANIMATION_DURATION_MS, 640);

    // Phase 1: 0.00 <= p < 0.25 (star1 fades out, star2 full)
    let (s1, s2) = calc_sparkle_opacities(0.0);
    assert!((s1 - 1.0).abs() < 1e-4);
    assert!((s2 - 1.0).abs() < 1e-4);

    let (s1_mid, s2_mid) = calc_sparkle_opacities(0.125);
    assert!((s1_mid - 0.5).abs() < 1e-4);
    assert!((s2_mid - 1.0).abs() < 1e-4);

    // Phase 2: 0.25 <= p < 0.50 (star1 off, star2 fades out)
    let (s1_p2, s2_p2) = calc_sparkle_opacities(0.25);
    assert_eq!(s1_p2, 0.0);
    assert!((s2_p2 - 1.0).abs() < 1e-4);

    // Phase 3: 0.50 <= p < 0.75 (star1 fades in, star2 off)
    let (s1_p3, s2_p3) = calc_sparkle_opacities(0.50);
    assert_eq!(s1_p3, 0.0);
    assert_eq!(s2_p3, 0.0);

    // Phase 4: 0.75 <= p <= 1.00 (star1 full, star2 fades in)
    let (s1_p4, s2_p4) = calc_sparkle_opacities(1.0);
    assert!((s1_p4 - 1.0).abs() < 1e-4);
    assert!((s2_p4 - 1.0).abs() < 1e-4);

    // Test Tooltip Manager dismissal persistence
    let mut tooltip = AiTooltipManager::new("pref_ai_compose", "Use AI to write");
    assert!(tooltip.should_display(true));
    assert!(!tooltip.should_display(false));

    tooltip.hide_and_remember();
    assert!(tooltip.is_dismissed);
    assert!(!tooltip.should_display(true));
}

#[test]
fn test_forward_panel_options_carousel() {
    assert_eq!(NAME_WITH_CAPTIONS_VERSION, -2);
    assert_eq!(NAME_NO_CAPTIONS_VERSION, -3);

    let items = vec![
        ForwardedItem {
            id: 101,
            from_name: "Alice".to_string(),
            text_preview: "Check out this project!".to_string(),
            has_media: false,
            has_caption: false,
        },
        ForwardedItem {
            id: 102,
            from_name: "Alice".to_string(),
            text_preview: "Photo of landscape".to_string(),
            has_media: true,
            has_caption: true,
        },
    ];

    let mut panel = ForwardPanelModel::with_items(items);
    assert!(!panel.is_empty());
    assert_eq!(panel.options, ForwardOptions::PreserveInfo);
    assert_eq!(panel.header_text(), "Forward 2 messages from Alice");

    // Cycle to NoNames
    let opt1 = panel.edit_to_next_option();
    assert_eq!(opt1, ForwardOptions::NoNames);
    assert_eq!(panel.header_text(), "Forward 2 messages (hide sender names)");

    // Cycle to NoNamesAndCaptions
    let opt2 = panel.edit_to_next_option();
    assert_eq!(opt2, ForwardOptions::NoNamesAndCaptions);
    assert_eq!(panel.header_text(), "Forward 2 messages (hide names and captions)");

    // Cycle back to PreserveInfo
    let opt3 = panel.edit_to_next_option();
    assert_eq!(opt3, ForwardOptions::PreserveInfo);
    assert_eq!(panel.header_text(), "Forward 2 messages from Alice");
}

#[test]
fn test_draft_options_fsm_sections() {
    let mut fsm = DraftOptionsFsm::new();
    assert_eq!(fsm.active_section, DraftSection::Reply);

    // Switch section
    fsm.switch_section(DraftSection::Forward);
    assert_eq!(fsm.active_section, DraftSection::Forward);

    fsm.switch_section(DraftSection::Link);
    assert_eq!(fsm.active_section, DraftSection::Link);

    // Toggle invert media (media above text)
    assert!(!fsm.webpage.invert_media);
    assert!(fsm.toggle_invert_media());
    assert!(fsm.webpage.invert_media);

    // Toggle large media
    assert!(fsm.webpage.force_large_media);
    assert!(!fsm.toggle_large_media());
    assert!(!fsm.webpage.force_large_media);

    // Quote management
    fsm.set_quote("Rust GPUI is fast", 12);
    assert_eq!(fsm.reply.quote_text.as_deref(), Some("Rust GPUI is fast"));
    assert_eq!(fsm.reply.quote_offset, 12);

    fsm.clear_quote();
    assert_eq!(fsm.reply.quote_text, None);
}

#[test]
fn test_write_restrictions_and_send_button_mode() {
    let mut model = ComposeControlsModel::new();
    assert_eq!(model.send_button, SendButtonType::RecordVoice);
    assert!(!model.restriction.is_restricted());

    // Typing text switches to Send
    model.set_text("Hello Telegram");
    assert_eq!(model.send_button, SendButtonType::Send);

    // Clearing text returns to RecordVoice
    model.set_text("");
    assert_eq!(model.send_button, SendButtonType::RecordVoice);

    // Scheduled mode switches to Schedule
    model.mode = ComposeControlsMode::Scheduled;
    model.update_send_button_type();
    assert_eq!(model.send_button, SendButtonType::Schedule);

    // Editing message switches to SaveEdit
    model.set_edit_message(42, "Old text");
    assert_eq!(model.send_button, SendButtonType::SaveEdit);

    // Write restriction
    model.restriction = WriteRestriction {
        restriction_type: WriteRestrictionType::PremiumRequired,
        text: "Only Premium subscribers can send messages in this group".to_string(),
        button: "Subscribe to Premium".to_string(),
        boosts_to_lift: 0,
    };
    assert!(model.restriction.is_restricted());
}

#[test]
fn test_stage_controls_models_and_formatters() {
    // 1. TTL tiny formatter
    assert_eq!(format_ttl_tiny(30), "30s");
    assert_eq!(format_ttl_tiny(300), "5m");
    assert_eq!(format_ttl_tiny(7200), "2h");
    assert_eq!(format_ttl_tiny(86400), "1d");
    assert_eq!(format_ttl_tiny(604800), "1w");

    // 2. Bottom controls action detection
    let mut bottom = HistoryViewBottomControlsViewControls::new();
    assert_eq!(bottom.active_action(), None);

    bottom.is_blocked = true;
    assert_eq!(bottom.active_action(), Some(BottomControlsAction::Unblock));

    bottom.is_blocked = false;
    bottom.is_bot_start = true;
    assert_eq!(bottom.active_action(), Some(BottomControlsAction::BotStart));

    // 3. Webpage processor link extraction
    let mut processor = HistoryViewWebpageProcessorViewControls::new();
    processor.parse_input_text("Check this out https://telegram.org and https://rust-lang.org");
    assert_eq!(processor.parsed_links.len(), 2);
    assert!(processor.parsed.is_some());
    assert_eq!(processor.parsed.unwrap().url, "https://telegram.org");

    // 4. Stash hint change tracking
    let mut stash_hint = HistoryViewComposeStashHintViewControls::new();
    stash_hint.track_change(100, 12345, 1000);
    assert_eq!(stash_hint.peak_length, 100);
    assert!(!stash_hint.is_toast_shown);

    // User cuts entire 100-character draft
    stash_hint.track_change(0, 12345, 1500);
    assert!(stash_hint.is_toast_shown);
    assert_eq!(stash_hint.memory.as_ref().unwrap().length, 100);
}

#[test]
fn test_zero_boilerplate_stubs_in_history_view_controls() {
    let controls_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/views/stage/view/controls");

    assert!(controls_dir.exists(), "controls directory must exist");

    let entries = std::fs::read_dir(&controls_dir)
        .expect("Failed to read stage/view/controls directory");

    let mut scanned_count = 0;
    for entry in entries {
        let entry = entry.expect("Directory entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            scanned_count += 1;
            let content = std::fs::read_to_string(&path)
                .expect("Failed to read source file");

            assert!(
                !content.contains("pub label: String, pub value: i64"),
                "File {:?} contains prohibited boilerplate Pattern A!",
                path
            );
            assert!(
                !content.contains("pub label: String"),
                "File {:?} contains prohibited boilerplate Pattern A `pub label: String`!",
                path
            );
            assert!(
                !content.contains("pub value: i64"),
                "File {:?} contains prohibited boilerplate Pattern A `pub value: i64`!",
                path
            );
            assert!(
                !content.contains("Autogenerated authentic GPUI Telegram view component"),
                "File {:?} contains prohibited boilerplate Pattern B header!",
                path
            );
        }
    }

    assert!(
        scanned_count >= 21,
        "Expected at least 21 controls files, found {}",
        scanned_count
    );
}

#[test]
fn test_all_21_stage_controls_instantiations() {
    let mut common = ComposeControlsCommonViewControls::new();
    common.set_active(true);
    assert!(common.is_active);

    let mut bot_menu = HistoryViewBotMenuButtonViewControls::new();
    assert!(bot_menu.refresh("Commands", false));
    assert_eq!(bot_menu.text, "Commands");

    let mut bottom = HistoryViewBottomControlsViewControls::new();
    bottom.set_report_mode(true, 5);
    assert_eq!(bottom.active_action(), Some(BottomControlsAction::Report));

    let mut char_limit = HistoryViewCharactersLimitViewControls::new();
    char_limit.set_left(15);
    assert!(char_limit.is_visible);

    let mut ai_btn = HistoryViewComposeAiButtonViewControls::new();
    ai_btn.tick(320);
    assert!(ai_btn.progress > 0.0);

    let mut ai_tip = HistoryViewComposeAiTooltipViewControls::new();
    ai_tip.hide_and_remember();
    assert!(ai_tip.is_dismissed);

    let mut comp_controls = HistoryViewComposeControlsViewControls::new();
    comp_controls.set_text("Hello");
    assert_eq!(comp_controls.model.text, "Hello");

    let mut media_edit = HistoryViewComposeMediaEditManagerViewControls::new();
    assert!(media_edit.toggle_spoiler());
    assert!(media_edit.config.spoilered);

    let mut search = HistoryViewComposeSearchViewControls::new();
    search.set_query("test query");
    assert_eq!(search.state.query, "test query");

    let mut stash = HistoryViewComposeStashViewControls::new();
    stash.stash_draft("key1", "text", Some(123), vec![]);
    assert!(stash.can_exchange());

    let mut stash_hint = HistoryViewComposeStashHintViewControls::new();
    stash_hint.track_change(100, 999, 100);
    stash_hint.track_change(0, 999, 200);
    assert!(stash_hint.is_toast_shown);

    let mut draft_opts = HistoryViewDraftOptionsViewControls::new();
    draft_opts.open(DraftSection::Forward);
    assert!(draft_opts.view.is_open);

    let mut fwd_panel = HistoryViewForwardPanelViewControls::new();
    assert_eq!(fwd_panel.cycle_options(), ForwardOptions::NoNames);

    let mut rich_preview = HistoryViewRichDraftPreviewViewControls::new();
    rich_preview.set_article("Title", "Desc", "https://t.me");
    assert!(rich_preview.is_visible);

    let mut suggest_opts = HistoryViewSuggestOptionsViewControls::new();
    suggest_opts.set_stars_price(100);
    assert_eq!(suggest_opts.price_after_commission_stars(), 85);

    let mut top_controls = HistoryViewTopControlsViewControls::new();
    top_controls.set_pinned(42, "Bob", "Pinned snippet");
    assert!(top_controls.is_visible);

    let mut ttl_btn = HistoryViewTtlButtonViewControls::new();
    ttl_btn.set_ttl(86400);
    assert_eq!(ttl_btn.badge_text(), "1d");

    let mut cover_uploader = HistoryViewVideoCoverUploaderViewControls::new();
    cover_uploader.start_upload("cover.jpg");
    assert!(cover_uploader.state.is_uploading);

    let mut voice_bar = HistoryViewVoiceRecordBarViewControls::new();
    voice_bar.start_holding(false);
    assert!(voice_bar.is_active());

    let voice_btn = HistoryViewVoiceRecordButtonViewControls::new();
    assert!(voice_btn.in_circle(0.0, 0.0));

    let mut webpage_proc = HistoryViewWebpageProcessorViewControls::new();
    webpage_proc.parse_input_text("Visit https://telegram.org");
    assert!(webpage_proc.parsed.is_some());
}
