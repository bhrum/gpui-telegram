//! Subsystem Integration Tests for Module 07 Slice 1: Chat Helpers Input, Tabbed Selector & Autocomplete
//!
//! Verifies:
//! - Mandate #15: Genuine implementations without fake/dummy facades or { id, label, value } boilerplate.
//! - Mandate #18: Faithful C++ logic, exact constants, debounce thresholds, selection navigation, and geometry math.

use gpui::px;
use gpui_telegram_desktop::theme::chat_helpers::{
    ChatHelpersStyleSheet, EmojiPickerOverlayStyleSheet,
};
use gpui_telegram_desktop::views::composer::compose::{
    ComposeFeatures, ComposeShow, PauseReasons,
};
use gpui_telegram_desktop::views::composer::emoji_interactions::EmojiInteractions;
use gpui_telegram_desktop::views::composer::emoji_keywords::{skip_exact_keyword, EmojiKeywords};
use gpui_telegram_desktop::views::composer::emoji_list_widget::{
    EmojiListWidget, EmojiSection, SkinToneModifier,
};
use gpui_telegram_desktop::views::composer::emoji_picker_overlay::{
    EmojiPickerOverlay, PickerOverlayMode,
};
use gpui_telegram_desktop::views::composer::emoji_sets_manager::{EmojiSetState, EmojiSetsManager};
use gpui_telegram_desktop::views::composer::emoji_suggestions_widget::EmojiSuggestionsWidget;
use gpui_telegram_desktop::views::composer::field_autocomplete::{
    AutocompleteChooseMethod, AutocompleteType, BotCommandRow, FieldAutocomplete, HashtagRow,
    MentionRow, StickerRow,
};
use gpui_telegram_desktop::views::composer::gifs_list_widget::{GifItem, GifsListWidget};
use gpui_telegram_desktop::views::composer::gifs_search::{GifsSearch, SearchState};
use gpui_telegram_desktop::views::composer::global_shortcuts::{
    ComposerAction, GlobalShortcuts, KeyModifier,
};
use gpui_telegram_desktop::views::composer::input_field::{FormattingTag, InputField};
use gpui_telegram_desktop::views::composer::message_field::MessageField;
use gpui_telegram_desktop::views::composer::tabbed_panel::TabbedPanel;
use gpui_telegram_desktop::views::composer::tabbed_selector::{
    SelectorTab, SlideDirection, TabbedSelector, TabbedSelectorMode,
};

#[test]
fn test_chat_helpers_theme_metrics() {
    let st = ChatHelpersStyleSheet::new();
    assert_eq!(st.emoji_pan_width, px(345.0));
    assert_eq!(st.emoji_pan_max_height, px(640.0));
    assert_eq!(st.emoji_pan_min_height, px(278.0));
    assert_eq!(st.emoji_pan_height_ratio, 0.75);
    assert_eq!(st.emoji_pan_radius, px(8.0));
    assert_eq!(st.compose_field_max_height, px(224.0));
    assert_eq!(st.desired_item_size, px(37.0));
    assert_eq!(st.header_height, px(33.0));
    assert_eq!(st.footer_height, px(36.0));
    assert_eq!(st.composer_min_height(), px(38.0));

    // Clamp panel content height
    let clamped_low = st.clamp_panel_height(200.0);
    assert_eq!(clamped_low, px(278.0)); // min clamp
    let clamped_high = st.clamp_panel_height(1000.0);
    assert_eq!(clamped_high, px(640.0)); // max clamp
    let clamped_mid = st.clamp_panel_height(500.0);
    assert_eq!(clamped_mid, px(375.0)); // 500 * 0.75 = 375
}

#[test]
fn test_emoji_picker_overlay_style_metrics() {
    let st = EmojiPickerOverlayStyleSheet::new();
    assert_eq!(st.expanded_radius, px(20.0));
    assert_eq!(st.item_size, px(30.0));
    assert_eq!(st.item_skip, px(4.0));
    assert_eq!(st.strip_height, px(40.0));
    assert_eq!(st.expanded_height, px(220.0));
    assert_eq!(st.item_advance(), px(34.0));
}

#[test]
fn test_tabbed_panel_geometry_math() {
    let mut panel = TabbedPanel::new();
    panel.bottom = px(600.0);

    // Update regular bottom-aligned content height
    panel.update_content_height(px(800.0), px(14.0), px(0.0));
    // available = 600.0, wanted = 600 * 0.75 = 450.0. wanted_content = 450 - 14 = 436.0
    assert_eq!(panel.content_height, px(436.0));
    assert_eq!(panel.computed_top, px(600.0 - 14.0 - 436.0));

    // Drop down mode
    panel.drop_down = true;
    panel.top = px(50.0);
    panel.update_content_height(px(800.0), px(14.0), px(0.0));
    assert_eq!(panel.computed_top, px(50.0));

    // Horizontal offset
    panel.right = px(20.0);
    panel.panel_width = px(345.0);
    panel.update_horizontal_position(px(1000.0));
    // right_bound = max(20, 345) = 345. computed_right = 1000 - 345 = 655
    assert_eq!(panel.computed_right, px(655.0));
}

#[test]
fn test_tabbed_selector_tab_switching_and_modes() {
    let mut selector = TabbedSelector::new();
    assert_eq!(selector.current_tab, SelectorTab::Emoji);

    // Switch from Emoji to Stickers -> RightToLeft slide
    let dir1 = selector.switch_tab(SelectorTab::Stickers);
    assert_eq!(dir1, Some(SlideDirection::RightToLeft));
    assert_eq!(selector.current_tab, SelectorTab::Stickers);

    // Switch from Stickers to Emoji -> LeftToRight slide
    let dir2 = selector.switch_tab(SelectorTab::Emoji);
    assert_eq!(dir2, Some(SlideDirection::LeftToRight));

    // Mode restrictions
    let emoji_only = TabbedSelector::new().with_mode(TabbedSelectorMode::EmojiOnly);
    assert!(emoji_only.is_tab_supported(SelectorTab::Emoji));
    assert!(!emoji_only.is_tab_supported(SelectorTab::Stickers));
    assert!(!emoji_only.is_tab_supported(SelectorTab::Gifs));

    let stickers_only = TabbedSelector::new().with_mode(TabbedSelectorMode::StickersOnly);
    assert!(!stickers_only.is_tab_supported(SelectorTab::Emoji));
    assert!(stickers_only.is_tab_supported(SelectorTab::Stickers));
}

#[test]
fn test_field_autocomplete_triggers_and_navigation() {
    let mut ac = FieldAutocomplete::new();
    ac.mention_rows = vec![
        MentionRow {
            user_id: 1,
            username: "alice".to_string(),
            name: "Alice Smith".to_string(),
            is_bot: false,
        },
        MentionRow {
            user_id: 2,
            username: "bob".to_string(),
            name: "Bob Jones".to_string(),
            is_bot: false,
        },
    ];
    ac.hashtag_rows = vec![HashtagRow {
        tag: "rust".to_string(),
    }];
    ac.bot_command_rows = vec![BotCommandRow {
        command: "start".to_string(),
        description: "Start the bot".to_string(),
        bot_user_id: 10,
    }];
    ac.sticker_rows = vec![
        StickerRow {
            document_id: 100,
            emoji: "👍".to_string(),
        },
        StickerRow {
            document_id: 101,
            emoji: "❤️".to_string(),
        },
    ];

    // Trigger @ (Mentions)
    ac.show_filtered("@ali");
    assert!(ac.is_open);
    assert_eq!(ac.autocomplete_type, AutocompleteType::Mentions);
    assert_eq!(ac.filter, "ali");
    assert_eq!(ac.selected_index, 0);

    // Selection move down
    assert!(ac.move_selection("Down"));
    assert_eq!(ac.selected_index, 1);
    assert_eq!(
        ac.choose_selected(AutocompleteChooseMethod::ByEnter),
        Some("@bob".to_string())
    );

    // Selection wrap
    assert!(ac.move_selection("Down"));
    assert_eq!(ac.selected_index, 0);

    // Trigger # (Hashtags)
    ac.show_filtered("#rust");
    assert_eq!(ac.autocomplete_type, AutocompleteType::Hashtags);
    assert_eq!(
        ac.choose_selected(AutocompleteChooseMethod::ByEnter),
        Some("#rust".to_string())
    );

    // Trigger / (Bot Commands)
    ac.show_filtered("/start");
    assert_eq!(ac.autocomplete_type, AutocompleteType::BotCommands);
    assert_eq!(
        ac.choose_selected(AutocompleteChooseMethod::ByEnter),
        Some("/start".to_string())
    );
}

#[test]
fn test_emoji_suggestions_colon_trigger() {
    let mut sug = EmojiSuggestionsWidget::new();

    // Valid colon trigger at beginning
    let q1 = sug.parse_query(":smile", 6);
    assert_eq!(q1, Some("smile".to_string()));
    assert_eq!(sug.replacement_range(), 0..6);

    // Valid colon trigger with preceding space
    let q2 = sug.parse_query("hello :heart", 12);
    assert_eq!(q2, Some("heart".to_string()));
    assert_eq!(sug.replacement_range(), 6..12);

    // Invalid colon trigger with alphanumeric preceding char (e.g. url or mid-word)
    let q3 = sug.parse_query("foo:bar", 7);
    assert_eq!(q3, None);
    assert!(!sug.is_visible);

    // Invalid colon trigger with trailing space
    let q4 = sug.parse_query("hello: ", 7);
    assert_eq!(q4, None);
    assert!(!sug.is_visible);
}

#[test]
fn test_emoji_interactions_state_machine() {
    let mut interactions = EmojiInteractions::new();
    let variants = vec![1001, 1002, 1003];

    // First outgoing interaction
    let anim1 = interactions
        .start_outgoing(42, "🎉", &variants, 1000)
        .expect("should create animation");
    assert_eq!(anim1.emoticon, "🎉");
    assert!(variants.contains(&anim1.document_id));

    // Consecutive outgoing interaction on same message should deduplicate variant if possible
    let anim2 = interactions
        .start_outgoing(42, "🎉", &variants, 1200)
        .expect("should create animation");
    if variants.len() > 1 {
        assert_ne!(anim1.document_id, anim2.document_id);
    }

    // Purge on message destroyed
    interactions.on_message_destroyed(42);
    assert!(interactions.outgoing_animations.get(&42).is_none());
}

#[test]
fn test_emoji_keywords_heuristics_and_search() {
    // Upstream SkipExactKeyword heuristics
    assert!(skip_exact_keyword("en", "10"));
    assert!(!skip_exact_keyword("en", "us"));
    assert!(!skip_exact_keyword("en", "ok"));
    assert!(!skip_exact_keyword("en", "hi"));
    assert!(skip_exact_keyword("en", "xy")); // non-whitelisted 2-char English
    assert!(skip_exact_keyword("en", "%")); // single char non-letter

    let kw = EmojiKeywords::new();
    let results = kw.search("smile");
    assert!(!results.is_empty());
    assert!(results.iter().any(|m| m.emoji == "😊"));
}

#[test]
fn test_emoji_sets_manager() {
    let mut mgr = EmojiSetsManager::new();
    assert_eq!(mgr.sets.len(), 4);
    assert_eq!(mgr.sets[0].name, "Apple");
    assert_eq!(mgr.sets[1].name, "Android");
    assert_eq!(mgr.sets[1].size_bytes, 7_854_643);

    // Cannot switch to uninstalled cloud set
    assert!(!mgr.set_active(1));

    // Simulate download completion
    mgr.update_progress(1, 1.0);
    assert_eq!(mgr.sets[1].state, EmojiSetState::Installed);
    assert!(mgr.set_active(1));
    assert_eq!(mgr.active_set_id, 1);
}

#[test]
fn test_emoji_picker_overlay_modes() {
    let mut overlay = EmojiPickerOverlay::new();
    assert_eq!(overlay.mode, PickerOverlayMode::CompactStrip);
    assert_eq!(overlay.current_height(), px(40.0));

    overlay.toggle_expanded();
    assert_eq!(overlay.mode, PickerOverlayMode::ExpandedGrid);
    assert_eq!(overlay.current_height(), px(220.0));

    overlay.toggle_expanded();
    assert_eq!(overlay.mode, PickerOverlayMode::CompactStrip);

    let cell = overlay.select_cell(0);
    assert!(cell.is_some());
}

#[test]
fn test_emoji_list_widget_and_skin_tones() {
    let mut widget = EmojiListWidget::new();
    assert!(!widget.entries.is_empty());

    // Switch section
    widget.switch_section(EmojiSection::Food);
    assert_eq!(widget.active_section, EmojiSection::Food);
    assert!(!widget.filtered_entries.is_empty());

    // Skin tone modification
    let entry_idx = widget
        .entries
        .iter()
        .position(|e| e.supports_skin_tones)
        .expect("should have skin-tone capable emoji");
    widget.set_skin_tone(entry_idx, SkinToneModifier::Dark);
    assert_eq!(
        widget.entries[entry_idx].active_skin_tone,
        SkinToneModifier::Dark
    );
}

#[test]
fn test_input_field_and_formatting() {
    let mut input = InputField::new();
    input.set_text("Hello World".to_string());
    assert_eq!(input.cursor, 11);

    // Select "World" and apply Bold
    input.selection = Some(6..11);
    input.apply_formatting(FormattingTag::Bold);
    assert_eq!(input.formatting.len(), 1);
    assert_eq!(input.formatting[0].tag, FormattingTag::Bold);

    // Undo / Redo
    input.selection = None;
    input.insert_text("!");
    assert_eq!(input.text, "Hello World!");
    assert!(input.undo());
    assert_eq!(input.text, "Hello World");
    assert!(input.redo());
    assert_eq!(input.text, "Hello World!");

    // Multi-line height expansion
    let h1 = input.calculate_content_height();
    input.set_text("Line 1\nLine 2\nLine 3\nLine 4\nLine 5".to_string());
    let h2 = input.calculate_content_height();
    assert!(h2 > h1);
}

#[test]
fn test_message_field_link_parsing() {
    let mut mf = MessageField::new();
    mf.set_text("Visit https://telegram.org or tonsite://channel today".to_string());

    assert_eq!(mf.parsed_links.len(), 2);
    assert_eq!(mf.parsed_links[0].url, "https://telegram.org");
    assert_eq!(mf.parsed_links[1].url, "tonsite://channel");
    assert_eq!(mf.webpage_preview_url.as_deref(), Some("https://telegram.org"));
    assert!(mf.is_typing_active());
}

#[test]
fn test_global_shortcuts() {
    let shortcuts = GlobalShortcuts::new();

    // Cmd+B -> Bold
    let act_bold = shortcuts.resolve_key("b", KeyModifier::CMD, false, false);
    assert_eq!(
        act_bold,
        Some(ComposerAction::ApplyFormatting(FormattingTag::Bold))
    );

    // Cmd+Shift+P -> Spoiler
    let act_spoil = shortcuts.resolve_key("p", KeyModifier::CMD_SHIFT, false, false);
    assert_eq!(
        act_spoil,
        Some(ComposerAction::ApplyFormatting(FormattingTag::Spoiler))
    );

    // Enter -> Send, Shift+Enter -> Newline
    let act_send = shortcuts.resolve_key("Enter", KeyModifier::NONE, false, false);
    assert_eq!(act_send, Some(ComposerAction::SendMessage));
    let act_nl = shortcuts.resolve_key("Enter", KeyModifier::SHIFT, false, false);
    assert_eq!(act_nl, Some(ComposerAction::InsertNewline));

    // Up on empty field -> Edit last message
    let act_edit = shortcuts.resolve_key("ArrowUp", KeyModifier::NONE, true, false);
    assert_eq!(act_edit, Some(ComposerAction::EditLastMessage));

    // Autocomplete navigation override
    let act_nav = shortcuts.resolve_key("ArrowDown", KeyModifier::NONE, false, true);
    assert_eq!(act_nav, Some(ComposerAction::NavigateAutocompleteDown));
}

#[test]
fn test_gifs_list_and_search() {
    let mut list = GifsListWidget::new();
    let gif = GifItem {
        document_id: 999,
        width: 320,
        height: 240,
        duration_seconds: 2.5,
        is_saved: false,
        thumbnail_url: "thumb.jpg".to_string(),
        stream_url: "stream.mp4".to_string(),
    };
    list.search_results.push(gif);

    // Toggle save GIF
    assert!(list.toggle_saved_gif(999));
    assert_eq!(list.saved_gifs.len(), 1);
    assert!(list.saved_gifs[0].is_saved);

    // Toggle again removes from saved
    assert!(!list.toggle_saved_gif(999));
    assert!(list.saved_gifs.is_empty());

    // Search state machine
    let mut search = GifsSearch::new();
    search.set_query("cat");
    assert!(matches!(search.state, SearchState::Debouncing { .. }));
    let q = search.on_debounce_elapsed();
    assert_eq!(q, Some("cat".to_string()));
    assert!(matches!(search.state, SearchState::Searching { .. }));
}

#[test]
fn test_compose_features_and_show() {
    let features = ComposeFeatures::default();
    assert!(features.send_as);
    assert!(features.ttl_info);
    assert!(features.attachments);
    assert!(features.has_any_autocomplete());

    let mut show = ComposeShow::new();
    assert!(!show.is_paused(PauseReasons::NONE));
    show.pause(PauseReasons::TABBED_PANEL);
    assert!(show.is_paused(PauseReasons::TABBED_PANEL));
    show.unpause(PauseReasons::TABBED_PANEL);
    assert!(!show.is_paused(PauseReasons::TABBED_PANEL));
}
