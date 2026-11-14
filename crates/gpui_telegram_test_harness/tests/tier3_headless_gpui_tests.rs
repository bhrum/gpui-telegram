use gpui_telegram_desktop::theme::TdesktopThemePalette;
use gpui_telegram_desktop::views::app_shell::window_adaptive::{WindowAdaptive, WindowLayout};
use gpui_telegram_desktop::views::composer::{ImeState, KeyAction};
use gpui_telegram_desktop::views::stage::cards::{
    DeliveryStatus, MessageContent, MessageDirection, MessageItemModel, PollOption, PollType,
};
use gpui_telegram_desktop::views::{
    CallOverlay, CanonicalComposer, ChatSidebar, ContextInspector, ConversationStage,
    ModalContainer, ModalDialogKind, NavRail, TalkModeState,
};
use gpui_telegram_desktop::components::ui::layers::LayerStackOption;
use gpui_telegram_test_harness::tier3_headless::HeadlessTranscriptDriver;

#[test]
fn test_tier3_embedded_svg_and_locale_assets() {
    HeadlessTranscriptDriver::verify_all_embedded_assets();
    HeadlessTranscriptDriver::verify_themes_and_localization();
}

#[test]
fn test_tier3_responsive_breakpoints_ime_and_composer_flows() {
    let mut adaptive = WindowAdaptive::with_dimensions(520.0, 760.0);
    assert!(adaptive.is_one_column());
    adaptive.update_from_width(820.0);
    assert_eq!(adaptive.window_layout, WindowLayout::Normal);
    adaptive.update_from_width(1280.0);
    assert!(adaptive.is_three_column() && adaptive.is_chat_wide());

    let mut composer = CanonicalComposer::new();
    composer.ime_machine.on_ime_start();
    composer.ime_machine.on_ime_update("ni", 2);
    assert!(matches!(composer.ime_machine.state, ImeState::Composing { .. }));
    assert_eq!(composer.handle_enter(false), KeyAction::ImeCandidateChosen);
    composer.clear();
    composer.ime_machine.on_ime_start();
    composer.ime_machine.on_ime_commit("你好");
    assert_eq!(composer.handle_enter(false), KeyAction::ImeCandidateChosen);
    assert_eq!(composer.handle_enter(true), KeyAction::InsertNewline);
    assert_eq!(composer.text(), "你好\n");
    assert_eq!(composer.handle_enter(false), KeyAction::SendMessage);

    composer.begin_reply(10, "Alice".into(), "Ping".into());
    assert!(composer.replying_to.is_some());
    composer.cancel_reply_or_edit();
    composer.begin_edit(11, "Old text".into());
    assert!(composer.editing_message.is_some());
}

#[test]
fn test_tier3_sidebar_stage_reactions_polls_modals_and_call_overlay() {
    let mut rail = NavRail::new();
    rail.set_connected(true);
    rail.set_account("AL", "Alice");
    rail.set_language("zh-CN");
    assert!(!rail.active_destination.localized_label("zh-CN").is_empty());

    let mut sidebar = ChatSidebar::new();
    assert!(sidebar.dialogs.is_empty());
    sidebar.width = 320.0;
    assert_eq!(sidebar.width, 320.0);
    sidebar.open_contacts_modal();
    assert!(sidebar.is_contacts_modal_open);
    sidebar.close_contacts_modal();
    sidebar.open_create_group_modal();
    sidebar.push_creation_text("Rust Group");
    assert_eq!(sidebar.active_creation_field_text(), "Rust Group");

    let mut stage = ConversationStage::new();
    stage.set_chat(42, "Rust Group", "2 members", "RG", 0x2AABEE);
    stage.add_message(MessageItemModel {
        id: 1,
        sender_id: 7,
        sender_name: "Alice".into(),
        avatar_initials: "AL".into(),
        avatar_color: 0x2AABEE,
        direction: MessageDirection::Outgoing,
        timestamp: "12:00".into(),
        delivery_status: DeliveryStatus::Sent,
        reply_to: None,
        reactions: Vec::new(),
        content: MessageContent::Poll {
            question: "Pick?".into(),
            poll_type: PollType::Regular,
            options: vec![
                PollOption { id: 0, text: "Yes".into(), votes: 0, percentage: 0.0, is_selected: false, is_correct: false },
                PollOption { id: 1, text: "No".into(), votes: 0, percentage: 0.0, is_selected: false, is_correct: false },
            ],
            total_votes: 0,
            is_closed: false,
            selected_option_id: None,
        },
        bot_keyboard: None,
        is_pinned: false,
    });
    stage.toggle_reaction(1, "❤️");
    assert_eq!(stage.messages[0].reactions.len(), 1);
    stage.vote_poll(1, 0);
    if let MessageContent::Poll { total_votes, selected_option_id, .. } = &stage.messages[0].content {
        assert_eq!(*total_votes, 1);
        assert_eq!(*selected_option_id, Some(0));
    } else {
        panic!("expected poll card");
    }

    let mut modal = ModalContainer::new();
    modal.set_palette(TdesktopThemePalette::default());
    let _id = modal.show_dialog(
        "Forward",
        ModalDialogKind::NewGroup,
        Vec::new(),
        380.0,
        420.0,
        LayerStackOption::Normal,
    );
    assert!(modal.stack.has_active_layers());

    let mut inspector = ContextInspector::new();
    inspector.toggle_open();
    assert!(inspector.is_open);

    let mut call = CallOverlay::new("Alice".into());
    assert!(!call.e2ee_emojis.is_empty());
    call.state = TalkModeState::Speaking;
    call.is_mic_muted = true;
    assert!(call.is_mic_muted);
}
