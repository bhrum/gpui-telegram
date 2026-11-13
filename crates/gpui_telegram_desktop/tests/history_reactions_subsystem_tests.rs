//! Unit & Subsystem Tests for History Slice 7: Reactions & Interactive Overlays
//! Mandate #18 & #19 Verification Suite

use gpui_telegram_desktop::views::stage::reactions::button::{
    opacity_for_scale, scale_for_state, sort_reaction_pills, ButtonParameters,
    ReactionButtonState, ReactionKey, ReactionPill,
    K_ACTIVATE_DURATION_MS, K_BUTTON_EXPAND_DELAY_MS, K_COLLAPSE_DURATION_MS,
    K_EXPAND_DURATION_MS, K_MAX_REACTIONS_SCROLL_AT_ONCE,
    K_TOGGLE_DURATION_MS, REACTION_CORNER_ADDED_HEIGHT_MAX, REACTION_CORNER_SIZE,
    REACTION_CORNER_SKIP, REACTION_TAG_ARROW, REACTION_TAG_LEFT_RADIUS, REACTION_TAG_RIGHT_RADIUS,
};
use gpui_telegram_desktop::views::stage::reactions::list::{
    ReactorItem, WhoReactedList, K_PER_PAGE, K_PER_PAGE_FIRST,
};
use gpui_telegram_desktop::views::stage::reactions::paid_toast::{
    PaidReactionToast, PAID_REACTION_STAR_TIERS, UNDO_COUNTDOWN_TOTAL_MS,
};
use gpui_telegram_desktop::views::stage::reactions::selector::{
    ReactionCategory, ReactionsSelector, SelectorState, K_ACCEPT_CLICKS_AFTER_MS,
    K_DEFAULT_COLUMNS, K_EXPAND_DELAY_MS, K_EXPAND_DURATION_MS as SELECTOR_EXPAND_MS,
    K_FRAMES_COUNT, K_MIN_NON_TRANSPARENT_COLUMNS, K_SCALE_DURATION_MS,
};
use gpui_telegram_desktop::views::stage::reactions::strip::{
    ReactionsStrip, K_EMOJI_CACHE_INDEX, K_HOVER_SCALE, K_HOVER_SCALE_DURATION_MS,
    K_SIZE_FOR_DOWNSCALE,
};
use gpui_telegram_desktop::views::stage::reactions::transcribe::{
    TranscribeButton, TranscribeState, K_ARROW_PIVOT_FAR, K_ARROW_PIVOT_NEAR,
    K_IN_NON_CHOSEN_OPACITY, K_OUT_NON_CHOSEN_OPACITY, TRANSCRIBE_SIZE_AUDIO,
    TRANSCRIBE_SIZE_ROUND_VIDEO,
};
use gpui_telegram_desktop::views::stage::view::history_view_about_view::{
    AboutViewType, HistoryViewAboutViewView, ABOUT_VIEW_CHAT_INTRO_WIDTH,
    ABOUT_VIEW_NEW_PEER_WIDTH, ABOUT_VIEW_NEW_THREAD_ABOUT_ICON_OUTER,
    ABOUT_VIEW_NEW_THREAD_ABOUT_MAX_WIDTH, ABOUT_VIEW_PREMIUM_REQUIRED_WIDTH,
    ABOUT_VIEW_STARS_PER_MESSAGE_WIDTH,
};
use gpui_telegram_desktop::views::stage::view::history_view_chat_preview::HistoryViewChatPreviewView;
use gpui_telegram_desktop::views::stage::view::history_view_contact_status::{
    ContactStatusType, HistoryViewContactStatusView,
};
use gpui_telegram_desktop::views::stage::view::history_view_emoji_interactions::{
    HistoryViewEmojiInteractionsView, K_MAX_PLAYS, K_PREMIUM_SHIFT, K_SMALL_DELAY_MS,
};
use gpui_telegram_desktop::views::stage::view::history_view_paid_reaction_toast::HistoryViewPaidReactionToastView;
use gpui_telegram_desktop::views::stage::view::history_view_quick_action::{
    DoubleClickQuickAction, HistoryViewQuickActionView,
};
use gpui_telegram_desktop::views::stage::view::history_view_reaction_preview::HistoryViewReactionPreviewView;
use gpui_telegram_desktop::views::stage::view::history_view_self_forwards_tagger::HistoryViewSelfForwardsTaggerView;
use gpui_telegram_desktop::views::stage::view::history_view_send_action::{
    HistoryViewSendActionView, SendActionType,
};
use gpui_telegram_desktop::views::stage::view::history_view_sponsored_click_handler::HistoryViewSponsoredClickHandlerView;
use gpui_telegram_desktop::views::stage::view::history_view_sticker_toast::HistoryViewStickerToastView;
use gpui_telegram_desktop::views::stage::view::history_view_transcribe_button::HistoryViewTranscribeButtonView;
use gpui_telegram_desktop::views::stage::view::history_view_view_button::{
    HistoryViewViewButtonView, ViewButtonKind,
};
use gpui_telegram_desktop::views::stage::view::reactions::history_view_reactions::HistoryViewReactionsViewReactions;
use gpui_telegram_desktop::views::stage::view::reactions::history_view_reactions_button::HistoryViewReactionsButtonViewReactions;
use gpui_telegram_desktop::views::stage::view::reactions::history_view_reactions_list::HistoryViewReactionsListViewReactions;
use gpui_telegram_desktop::views::stage::view::reactions::history_view_reactions_selector::HistoryViewReactionsSelectorViewReactions;
use gpui_telegram_desktop::views::stage::view::reactions::history_view_reactions_strip::HistoryViewReactionsStripViewReactions;
use gpui_telegram_desktop::views::stage::view::reactions::history_view_reactions_tabs::HistoryViewReactionsTabsViewReactions;
use gpui_telegram_desktop::theme::history::history_view_about_view_style::HistoryViewAboutViewViewStyleSheet;

use gpui::{point, px};

#[test]
fn test_reaction_button_scale_and_opacity_math() {
    // Exact upstream formulas from history_view_reactions_button.cpp:L290-L305
    let scale_hidden = scale_for_state(ReactionButtonState::Hidden);
    let scale_shown = scale_for_state(ReactionButtonState::Shown);
    let scale_active = scale_for_state(ReactionButtonState::Active);
    let scale_inside = scale_for_state(ReactionButtonState::Inside);

    assert!((scale_hidden - 1.0 / 3.0).abs() < 1e-5);
    assert!((scale_shown - 2.0 / 3.0).abs() < 1e-5);
    assert!((scale_active - 1.0).abs() < 1e-5);
    assert!((scale_inside - 1.0).abs() < 1e-5);

    // Opacity interpolation
    assert_eq!(opacity_for_scale(scale_hidden), 0.0);
    assert_eq!(opacity_for_scale(scale_shown), 1.0);
    assert_eq!(opacity_for_scale(scale_active), 1.0);
    assert!((opacity_for_scale(0.5) - 0.5).abs() < 1e-5);
    assert_eq!(opacity_for_scale(0.0), 0.0);
    assert_eq!(opacity_for_scale(1.5), 1.0);

    // Constants
    assert_eq!(K_TOGGLE_DURATION_MS, 120);
    assert_eq!(K_ACTIVATE_DURATION_MS, 150);
    assert_eq!(K_EXPAND_DURATION_MS, 300);
    assert_eq!(K_COLLAPSE_DURATION_MS, 250);
    assert_eq!(K_BUTTON_EXPAND_DELAY_MS, 25);
    assert_eq!(K_MAX_REACTIONS_SCROLL_AT_ONCE, 2);
    assert_eq!(REACTION_CORNER_SIZE, 32.0);
    assert_eq!(REACTION_CORNER_SKIP, 4.0);
    assert_eq!(REACTION_CORNER_ADDED_HEIGHT_MAX, 200.0);
    assert_eq!(REACTION_TAG_ARROW, 5.0);
    assert_eq!(REACTION_TAG_RIGHT_RADIUS, 4.0);
    assert_eq!(REACTION_TAG_LEFT_RADIUS, 14.0);
}

#[test]
fn test_reaction_pills_sorting_tie_breakers() {
    // Exact sorting rules from history_view_reactions.cpp:L190-L206
    let pills = vec![
        ReactionPill {
            id: ReactionKey::Emoji("👍".into()),
            count: 5,
            chosen: false,
            paid: false,
            tag: false,
            tag_title: None,
            recent_avatars: Vec::new(),
            catalog_index: 2,
        },
        ReactionPill {
            id: ReactionKey::Emoji("❤️".into()),
            count: 10,
            chosen: true, // subtracts 1 for user vote -> effective count 9
            paid: false,
            tag: false,
            tag_title: None,
            recent_avatars: Vec::new(),
            catalog_index: 1,
        },
        ReactionPill {
            id: ReactionKey::Paid, // Paid reaction MUST sort first regardless of count!
            count: 1,
            chosen: false,
            paid: true,
            tag: false,
            tag_title: None,
            recent_avatars: Vec::new(),
            catalog_index: 10,
        },
        ReactionPill {
            id: ReactionKey::Emoji("🔥".into()),
            count: 9, // effective count 9 (tie with ❤️, but ❤️ has catalog_index 1 < 3)
            chosen: false,
            paid: false,
            tag: false,
            tag_title: None,
            recent_avatars: Vec::new(),
            catalog_index: 3,
        },
    ];

    let sorted = sort_reaction_pills(pills);
    assert_eq!(sorted.len(), 4);
    // Index 0 must be Paid reaction!
    assert!(sorted[0].id.is_paid());
    // Index 1 must be ❤️ (effective count 9, catalog 1)
    assert_eq!(sorted[1].id, ReactionKey::Emoji("❤️".into()));
    // Index 2 must be 🔥 (effective count 9, catalog 3)
    assert_eq!(sorted[2].id, ReactionKey::Emoji("🔥".into()));
    // Index 3 must be 👍 (count 5)
    assert_eq!(sorted[3].id, ReactionKey::Emoji("👍".into()));
}

#[test]
fn test_reactions_button_fsm_and_parameters() {
    let mut btn = HistoryViewReactionsButtonViewReactions::new();
    assert_eq!(btn.state, ReactionButtonState::Shown);
    assert!(!btn.is_hidden);

    // Apply parameters with outside=true -> should transition to Hidden
    btn.apply_parameters(ButtonParameters {
        center: point(px(100.0), px(200.0)),
        pointer: point(px(0.0), px(0.0)),
        global_pointer: point(px(0.0), px(0.0)),
        reactions_count: 3,
        visible_top: px(50.0),
        visible_bottom: px(600.0),
        outside: true,
        cursor_left: false,
    });
    assert_eq!(btn.state, ReactionButtonState::Hidden);
    assert!(btn.is_hidden);

    // Apply parameters inside
    btn.apply_parameters(ButtonParameters {
        center: point(px(100.0), px(200.0)),
        pointer: point(px(100.0), px(200.0)),
        global_pointer: point(px(100.0), px(200.0)),
        reactions_count: 5,
        visible_top: px(50.0),
        visible_bottom: px(600.0),
        outside: false,
        cursor_left: true,
    });
    assert_eq!(btn.state, ReactionButtonState::Active);
    assert!(!btn.is_hidden);
    assert!(btn.expand_up());
}

#[test]
fn test_reactions_selector_categories_and_search() {
    let mut selector = ReactionsSelector::new();
    assert_eq!(selector.state, SelectorState::Showing);
    assert_eq!(selector.columns, K_DEFAULT_COLUMNS);
    assert_eq!(K_DEFAULT_COLUMNS, 8);
    assert_eq!(K_MIN_NON_TRANSPARENT_COLUMNS, 7);
    assert_eq!(K_FRAMES_COUNT, 32);
    assert_eq!(K_SCALE_DURATION_MS, 120);
    assert_eq!(SELECTOR_EXPAND_MS, 300);
    assert_eq!(K_EXPAND_DELAY_MS, 40);
    assert_eq!(K_ACCEPT_CLICKS_AFTER_MS, 300);

    // Expand
    selector.set_expanded(true);
    assert_eq!(selector.state, SelectorState::Expanded);

    // Search query filter
    selector.search_query = "fire".into();
    let _filtered = selector.filtered_reactions();
    // Default reactions has 🔥 or emojis
    selector.search_query = "❤️".into();
    let heart = selector.filtered_reactions();
    assert_eq!(heart.len(), 1);
    assert_eq!(heart[0].key, ReactionKey::Emoji("❤️".into()));

    // Clear search
    selector.search_query = String::new();
    selector.active_category = ReactionCategory::Top;
    assert!(!selector.filtered_reactions().is_empty());
}

#[test]
fn test_reactions_strip_hover_scaling_and_selection() {
    let mut strip = ReactionsStrip::default();
    assert_eq!(K_SIZE_FOR_DOWNSCALE, 96);
    assert_eq!(K_EMOJI_CACHE_INDEX, 0);
    assert_eq!(K_HOVER_SCALE_DURATION_MS, 200);
    assert!((K_HOVER_SCALE - 1.24).abs() < 1e-4);

    // Hover item 1
    strip.set_hovered(Some(1));
    assert_eq!(strip.hovered_index, Some(1));
    assert_eq!(strip.items[1].scale, K_HOVER_SCALE);
    assert_eq!(strip.items[0].scale, 1.0);

    // Select item 0
    let selected = strip.select_reaction(0);
    assert_eq!(selected, Some(ReactionKey::Emoji("👍".into())));
    assert!(strip.items[0].is_selected);
    assert!(!strip.items[1].is_selected);
}

#[test]
fn test_who_reacted_list_tabs_and_moderation() {
    let items = vec![
        ReactorItem {
            peer_id: 101,
            name: "Alice".into(),
            avatar_url: None,
            reaction: ReactionKey::Emoji("👍".into()),
            date: 1000,
            paid_stars: None,
        },
        ReactorItem {
            peer_id: 102,
            name: "Bob".into(),
            avatar_url: None,
            reaction: ReactionKey::Paid,
            date: 1001,
            paid_stars: Some(100),
        },
    ];

    let mut list = WhoReactedList::new(items, true);
    assert_eq!(K_PER_PAGE_FIRST, 20);
    assert_eq!(K_PER_PAGE, 100);
    assert!(list.can_moderate_by_delete_messages());

    // All tab
    assert_eq!(list.filter_by_tab().len(), 2);

    // Filter by Paid
    list.active_tab = Some(ReactionKey::Paid);
    let paid_only = list.filter_by_tab();
    assert_eq!(paid_only.len(), 1);
    assert_eq!(paid_only[0].name, "Bob");
    assert_eq!(paid_only[0].paid_stars, Some(100));
}

#[test]
fn test_paid_reaction_toast_star_tiers_and_undo() {
    assert_eq!(PAID_REACTION_STAR_TIERS, [1, 5, 10, 50, 100, 500, 1000, 2500, 5000, 10000]);
    assert_eq!(UNDO_COUNTDOWN_TOTAL_MS, 5000);

    let mut toast = PaidReactionToast::new(42, 500, false);
    assert_eq!(toast.stars_count, 500);
    assert_eq!(toast.seconds_left(), 5);
    assert_eq!(toast.particles.len(), 16);

    // Tick 1500ms
    let still_active = toast.tick_countdown(1500);
    assert!(still_active);
    assert_eq!(toast.remaining_ms, 3500);
    assert_eq!(toast.seconds_left(), 4);

    // Undo action
    toast.undo();
    assert!(toast.is_undone);
}

#[test]
fn test_transcribe_button_state_machine_and_dimensions() {
    assert_eq!(TRANSCRIBE_SIZE_AUDIO, 30.0);
    assert_eq!(TRANSCRIBE_SIZE_ROUND_VIDEO, 36.0);
    assert!((K_ARROW_PIVOT_NEAR - 0.349).abs() < 1e-4);
    assert!((K_ARROW_PIVOT_FAR - 0.651).abs() < 1e-4);
    assert_eq!(K_IN_NON_CHOSEN_OPACITY, 0.12);
    assert_eq!(K_OUT_NON_CHOSEN_OPACITY, 0.18);

    // Audio note without premium -> triggers PremiumLocked
    let mut btn_free = TranscribeButton::new(10, false, false);
    assert_eq!(btn_free.state, TranscribeState::Idle);
    assert_eq!(btn_free.button_size(), 30.0);
    btn_free.trigger_click();
    assert_eq!(btn_free.state, TranscribeState::PremiumLocked);

    // Video note with premium -> triggers Loading
    let mut btn_premium = TranscribeButton::new(11, true, true);
    assert_eq!(btn_premium.button_size(), 36.0);
    btn_premium.trigger_click();
    assert_eq!(btn_premium.state, TranscribeState::Loading);

    // Set transcribed text
    btn_premium.set_transcribed("Hello from voice note!".into());
    assert_eq!(btn_premium.state, TranscribeState::Transcribed);
    assert!(btn_premium.is_opened);
    assert_eq!(btn_premium.transcript_text, Some("Hello from voice note!".into()));

    // Toggle collapse
    btn_premium.trigger_click();
    assert!(!btn_premium.is_opened);
}

#[test]
fn test_about_view_metrics_and_style() {
    assert_eq!(ABOUT_VIEW_PREMIUM_REQUIRED_WIDTH, 186.0);
    assert_eq!(ABOUT_VIEW_STARS_PER_MESSAGE_WIDTH, 226.0);
    assert_eq!(ABOUT_VIEW_CHAT_INTRO_WIDTH, 224.0);
    assert_eq!(ABOUT_VIEW_NEW_PEER_WIDTH, 320.0);
    assert_eq!(ABOUT_VIEW_NEW_THREAD_ABOUT_MAX_WIDTH, 180.0);
    assert_eq!(ABOUT_VIEW_NEW_THREAD_ABOUT_ICON_OUTER, 60.0);

    let about = HistoryViewAboutViewView::new(
        "Telegram News".into(),
        "Official news channel".into(),
        AboutViewType::ChannelAbout,
    );
    assert_eq!(about.title, "Telegram News");
    assert_eq!(about.view_type, AboutViewType::ChannelAbout);

    let style = HistoryViewAboutViewViewStyleSheet::new();
    assert_eq!(style.premium_required_width, px(186.0));
    assert_eq!(style.stars_per_message_width, px(226.0));
    assert_eq!(style.chat_intro_width, px(224.0));
    assert_eq!(style.new_peer_width, px(320.0));
    assert_eq!(style.new_bot_thread_top_skip, px(24.0));
}

#[test]
fn test_interactive_overlays_authentic_implementations() {
    // 1. Contact Status
    let contact = HistoryViewContactStatusView::new(55, ContactStatusType::AddContact);
    assert_eq!(contact.peer_id, 55);
    assert_eq!(contact.status_type, ContactStatusType::AddContact);
    assert!(contact.is_visible);

    // 2. Quick Action
    let mut quick = HistoryViewQuickActionView::new();
    assert_eq!(quick.current_action, DoubleClickQuickAction::Reply);
    assert_eq!(quick.handle_double_click(1), Some("reply".to_string()));
    quick.set_action(DoubleClickQuickAction::React);
    assert_eq!(quick.handle_double_click(1), Some("react:❤️".to_string()));

    // 3. Emoji Interactions
    let mut emoji_inter = HistoryViewEmojiInteractionsView::new();
    assert!((K_PREMIUM_SHIFT - 21.0 / 240.0).abs() < 1e-4);
    assert_eq!(K_MAX_PLAYS, 5);
    assert_eq!(K_SMALL_DELAY_MS, 200);
    assert!(emoji_inter.play_emoji("❤️".into()));
    assert_eq!(emoji_inter.current_interaction.as_ref().unwrap().plays_count, 1);

    // 4. Chat Preview
    let mut preview = HistoryViewChatPreviewView::new(77, "Private Chat".into());
    preview.add_preview_message("Alice".into(), "See you tomorrow!".into());
    assert_eq!(preview.preview_messages.len(), 1);

    // 5. Self Forwards Tagger
    let mut tagger = HistoryViewSelfForwardsTaggerView::new(99);
    tagger.add_tag("recipes".into());
    assert_eq!(tagger.active_tags, vec!["recipes"]);
    tagger.remove_tag("recipes");
    assert!(tagger.active_tags.is_empty());

    // 6. Send Action Painter
    let mut send = HistoryViewSendActionView::new(88, "Charlie".into(), SendActionType::Typing);
    assert_eq!(send.description(), "Charlie is typing...");
    send.tick_animation();
    assert_eq!(send.animation_frame, 1);

    // 7. Sponsored Click Handler
    let mut sponsored = HistoryViewSponsoredClickHandlerView::new(
        "bot_welcome".into(),
        true,
        "Promoted Channel".into(),
    );
    assert_eq!(sponsored.handle_click(), "internal_tg://bot_welcome");

    // 8. Sticker Toast
    let sticker_toast = HistoryViewStickerToastView::new("Duck Pack".into(), "🦆".into(), true);
    assert_eq!(sticker_toast.pack_title, "Duck Pack");
    assert!(sticker_toast.is_added);

    // 9. View Button
    let view_btn = HistoryViewViewButtonView::new(123, ViewButtonKind::Giveaway);
    assert_eq!(view_btn.text, "View Giveaway in Chat");

    // 10. Reactions Views
    let mut reactions_mgr = HistoryViewReactionsViewReactions::new(456);
    reactions_mgr.add_reaction(ReactionKey::Emoji("👍".into()), true);
    assert_eq!(reactions_mgr.pills.len(), 1);
    assert!(reactions_mgr.pills[0].chosen);

    let list_view = HistoryViewReactionsListViewReactions::new(456, false);
    assert_eq!(list_view.page_size, K_PER_PAGE_FIRST);

    let sel_view = HistoryViewReactionsSelectorViewReactions::new();
    assert!(!sel_view.inner.reactions.is_empty());

    let strip_view = HistoryViewReactionsStripViewReactions::new();
    assert!(strip_view.is_visible);

    let tabs_view = HistoryViewReactionsTabsViewReactions::default();
    assert_eq!(tabs_view.tabs.len(), 3);

    let toast_view = HistoryViewPaidReactionToastView::new(456, 100);
    assert_eq!(toast_view.inner.stars_count, 100);

    let trans_view = HistoryViewTranscribeButtonView::new(456, false, true);
    assert_eq!(trans_view.inner.button_size(), 30.0);

    let prev_view = HistoryViewReactionPreviewView::new("🔥".into(), 456);
    assert_eq!(prev_view.reaction_or_sticker_id, "🔥");
}
