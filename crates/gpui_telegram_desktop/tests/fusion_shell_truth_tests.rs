//! FUS-D004 Integration & Contract Tests (`docs/specs/fusion-truthful-initial-state-20261108.md`).
//!
//! Verifies:
//! 1. Initial and reset workspace states contain no fabricated peers, stories, folders, or histories.
//! 2. `apply_authoritative_chat_list` replaces `sidebar.dialogs` with server-returned `ChatSummary` rows
//!    instead of merging with fabricated/stale rows, and derives no fabricated online status or timestamps.
//! 3. Empty server chat list clears `sidebar.dialogs`, selection, `stage`, and `inspector`.
//! 4. Selection is preserved across refresh only when the selected `chat_id` remains present in the server
//!    response; otherwise selection becomes `None` without silently picking an arbitrary peer.
//! 5. Account switch and logout reset all account-bound state (`sidebar`, `stage`, `inspector`).
//! 6. Stale async chat-list and history responses from a logged-out session, previous account, or older
//!    request generation are fenced and ignored.
//! 7. Selected-chat send guard rejects sends when unauthenticated or when no authoritative server chat is
//!    selected, preserving the user's draft in `CanonicalComposer`.

use gpui_telegram_client::net::http_client::HistoricalMessage;
use gpui_telegram_client::ChatSummary;
use gpui_telegram_desktop::views::shell::{
    apply_authoritative_chat_list, apply_fenced_chat_list_response, apply_fenced_history_response,
    peer_subtitle_for_dialog, reset_account_conversation_views, select_authoritative_dialog,
    try_consume_composer_for_send, validate_selected_send_target, SessionRequestFence,
};
use gpui_telegram_desktop::views::{
    CanonicalComposer, ChatSidebar, ContextInspector, ConversationStage,
};

fn sample_server_chats() -> Vec<ChatSummary> {
    vec![
        ChatSummary {
            chat_id: 101,
            title: "Product Sync".to_string(),
            last_message: None,
            unread_count: 2,
            avatar_color: Some(0x2AABEE),
            updated_at: None,
        },
        ChatSummary {
            chat_id: 202,
            title: "Alice Chen".to_string(),
            last_message: Some("Ready for review".to_string()),
            unread_count: 0,
            avatar_color: Some(0x8E44AD),
            updated_at: Some(1_700_000_000_000),
        },
    ]
}

#[test]
fn fus_d004_initial_and_reset_workspace_state_contains_no_fabricated_peers_or_histories() {
    let mut sidebar = ChatSidebar::new();
    let mut stage = ConversationStage::new();
    let mut inspector = ContextInspector::new();

    assert!(sidebar.dialogs.is_empty());
    assert_eq!(sidebar.active_chat_id, None);
    assert_eq!(sidebar.inner_widget.selected_index, None);
    assert!(sidebar.stories.elements().is_empty());
    assert!(sidebar.top_peers.entries().is_empty());
    assert!(!sidebar.suggestions.is_visible);

    assert_eq!(stage.active_chat_id, None);
    assert!(stage.messages.is_empty());
    assert!(stage.chat_histories.is_empty());
    assert!(stage.chat_pinned_messages.is_empty());
    assert!(stage.pinned_message.is_none());

    assert!(inspector.profile.name.is_empty());
    assert!(inspector.profile.status.is_empty());
    assert!(inspector.profile.facts.is_empty());
    assert!(inspector.media_items.is_empty());

    reset_account_conversation_views(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        "https://api.fusion.example",
        Some("alice"),
    );

    assert!(sidebar.dialogs.is_empty());
    assert_eq!(sidebar.active_chat_id, None);
    assert_eq!(sidebar.folders, vec![("All".to_string(), 0)]);
    assert_eq!(stage.active_chat_id, None);
    assert_eq!(stage.chat_title, "Select a chat");
    assert!(stage.chat_subtitle.is_empty());
    assert!(stage.messages.is_empty());
    assert!(stage.chat_histories.is_empty());
    assert!(inspector.profile.name.is_empty());
    assert!(inspector.profile.status.is_empty());
    assert!(!inspector.is_open);
}

#[test]
fn fus_d004_authoritative_chat_list_replaces_stale_or_fabricated_rows_without_merging() {
    let mut sidebar = ChatSidebar::new_demo();
    let mut stage = ConversationStage::new_demo();
    let mut inspector = ContextInspector::new_demo();

    // Precondition: demo constructors have seeded rows 1..=6 and active_chat_id = Some(3).
    assert_eq!(sidebar.dialogs.len(), 6);
    assert_eq!(sidebar.active_chat_id, Some(3));
    assert_eq!(stage.active_chat_id, Some(3));
    assert!(!stage.chat_histories.is_empty());

    let server_chats = sample_server_chats();
    let preserved = apply_authoritative_chat_list(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        &server_chats,
    );

    // Prior selection (3) is not in [101, 202], so selection must be cleared to None.
    assert_eq!(preserved, None);
    assert_eq!(sidebar.active_chat_id, None);
    assert_eq!(sidebar.inner_widget.selected_index, None);
    assert_eq!(stage.active_chat_id, None);
    assert_eq!(stage.chat_title, "Select a chat");
    assert!(stage.chat_subtitle.is_empty());
    assert!(stage.messages.is_empty());
    assert!(inspector.profile.name.is_empty());
    assert!(inspector.profile.status.is_empty());

    // Only authoritative server chats [101, 202] exist; no demo rows (1..=6) remain.
    let dialog_ids: Vec<u64> = sidebar.dialogs.iter().map(|d| d.id).collect();
    assert_eq!(dialog_ids, vec![101, 202]);
    assert!(stage.chat_histories.is_empty());

    // Verify no fabricated online status, timestamp, or placeholder last_message when absent.
    let first = &sidebar.dialogs[0];
    assert_eq!(first.id, 101);
    assert_eq!(first.peer_name, "Product Sync");
    assert_eq!(first.last_message, "");
    assert_eq!(first.timestamp, "");
    assert!(!first.is_online);
    assert_eq!(peer_subtitle_for_dialog(first), "");

    let second = &sidebar.dialogs[1];
    assert_eq!(second.id, 202);
    assert_eq!(second.peer_name, "Alice Chen");
    assert_eq!(second.last_message, "Ready for review");
    assert!(!second.timestamp.is_empty());
    assert!(!second.is_online);
    assert_eq!(peer_subtitle_for_dialog(second), "");
}

#[test]
fn fus_d004_empty_server_chat_list_clears_dialogs_selection_stage_and_inspector() {
    let mut sidebar = ChatSidebar::new();
    let mut stage = ConversationStage::new();
    let mut inspector = ContextInspector::new();

    let server_chats = sample_server_chats();
    apply_authoritative_chat_list(&mut sidebar, &mut stage, &mut inspector, &server_chats);
    assert!(select_authoritative_dialog(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        101
    ));
    assert_eq!(sidebar.active_chat_id, Some(101));
    assert_eq!(stage.active_chat_id, Some(101));
    assert_eq!(inspector.profile.name, "Product Sync");

    // Server now returns an empty list `[]`.
    let preserved = apply_authoritative_chat_list(&mut sidebar, &mut stage, &mut inspector, &[]);
    assert_eq!(preserved, None);
    assert!(sidebar.dialogs.is_empty());
    assert_eq!(sidebar.inner_widget.total_rows, 0);
    assert_eq!(sidebar.active_chat_id, None);
    assert_eq!(sidebar.inner_widget.selected_index, None);
    assert_eq!(stage.active_chat_id, None);
    assert_eq!(stage.chat_title, "Select a chat");
    assert!(stage.chat_subtitle.is_empty());
    assert!(stage.messages.is_empty());
    assert!(stage.chat_histories.is_empty());
    assert!(inspector.profile.name.is_empty());
    assert!(inspector.profile.status.is_empty());
}

#[test]
fn fus_d004_refresh_preserves_existing_selection_only_when_still_present_in_server_list() {
    let mut sidebar = ChatSidebar::new();
    let mut stage = ConversationStage::new();
    let mut inspector = ContextInspector::new();

    let initial_chats = sample_server_chats();
    apply_authoritative_chat_list(&mut sidebar, &mut stage, &mut inspector, &initial_chats);

    // User explicitly selects chat 202.
    assert!(select_authoritative_dialog(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        202
    ));
    assert_eq!(sidebar.active_chat_id, Some(202));
    assert_eq!(sidebar.inner_widget.selected_index, Some(1));
    assert_eq!(stage.active_chat_id, Some(202));
    assert_eq!(stage.chat_title, "Alice Chen");
    assert_eq!(stage.chat_subtitle, "");
    assert_eq!(inspector.profile.name, "Alice Chen");
    assert_eq!(inspector.profile.status, "");

    // Refresh with 202 still present (at index 0 with updated title).
    let updated_chats = vec![
        ChatSummary {
            chat_id: 202,
            title: "Alice Chen (Core)".to_string(),
            last_message: Some("Updated message".to_string()),
            unread_count: 1,
            avatar_color: Some(0x8E44AD),
            updated_at: None,
        },
        ChatSummary {
            chat_id: 303,
            title: "Infra Ops".to_string(),
            last_message: None,
            unread_count: 0,
            avatar_color: Some(0x16A085),
            updated_at: None,
        },
    ];
    let preserved = apply_authoritative_chat_list(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        &updated_chats,
    );
    assert_eq!(preserved, Some(202));
    assert_eq!(sidebar.active_chat_id, Some(202));
    assert_eq!(sidebar.inner_widget.selected_index, Some(0));
    assert_eq!(stage.active_chat_id, Some(202));
    assert_eq!(stage.chat_title, "Alice Chen (Core)");
    assert_eq!(stage.chat_subtitle, "");
    assert_eq!(inspector.profile.name, "Alice Chen (Core)");

    // Refresh where 202 is no longer present: selection must become None (never auto-select 303).
    let without_202 = vec![ChatSummary {
        chat_id: 303,
        title: "Infra Ops".to_string(),
        last_message: None,
        unread_count: 0,
        avatar_color: Some(0x16A085),
        updated_at: None,
    }];
    let preserved_after_removal = apply_authoritative_chat_list(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        &without_202,
    );
    assert_eq!(preserved_after_removal, None);
    assert_eq!(sidebar.active_chat_id, None);
    assert_eq!(stage.active_chat_id, None);
    assert!(inspector.profile.name.is_empty());
}

#[test]
fn fus_d004_explicit_selection_rejects_non_authoritative_id_and_updates_views_coherently() {
    let mut sidebar = ChatSidebar::new();
    let mut stage = ConversationStage::new();
    let mut inspector = ContextInspector::new();

    apply_authoritative_chat_list(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        &sample_server_chats(),
    );

    // Attempting to select non-existent demo ID 4 must fail and leave views unselected.
    assert!(!select_authoritative_dialog(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        4
    ));
    assert_eq!(sidebar.active_chat_id, None);
    assert_eq!(stage.active_chat_id, None);
    assert!(inspector.profile.name.is_empty());

    // Selecting authoritative ID 101 coherently updates all three views.
    assert!(select_authoritative_dialog(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        101
    ));
    assert_eq!(sidebar.active_chat_id, Some(101));
    assert_eq!(stage.active_chat_id, Some(101));
    assert_eq!(stage.chat_title, "Product Sync");
    assert_eq!(stage.chat_subtitle, "");
    assert_eq!(inspector.profile.name, "Product Sync");
    assert_eq!(inspector.profile.status, "");
}

#[test]
fn fus_d004_account_switch_and_logout_reset_clears_previous_account_state() {
    let mut sidebar = ChatSidebar::new();
    let mut stage = ConversationStage::new();
    let mut inspector = ContextInspector::new();

    // Account A populates chats, selects 101, and loads history.
    apply_authoritative_chat_list(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        &sample_server_chats(),
    );
    assert!(select_authoritative_dialog(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        101
    ));
    let fence_a = SessionRequestFence {
        auth_generation: 1,
        request_generation: 1,
        token: "token_account_a".to_string(),
        user_id: 10,
    };
    assert!(apply_fenced_history_response(
        &fence_a,
        true,
        1,
        1,
        Some("token_account_a"),
        Some(10),
        &sidebar,
        &mut stage,
        101,
        vec![HistoricalMessage {
            message_id: 5001,
            chat_id: 101,
            sender_id: 10,
            text: "Account A confidential message".to_string(),
            date: None,
            is_outgoing: Some(true),
        }],
    ));
    inspector.is_open = true;
    assert_eq!(stage.messages.len(), 1);
    assert!(stage.chat_histories.contains_key(&101));

    // Logout or switch to Account B resets all account-bound views.
    reset_account_conversation_views(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        "https://api.fusion.example",
        Some("bob"),
    );

    assert!(sidebar.dialogs.is_empty());
    assert_eq!(sidebar.active_chat_id, None);
    assert_eq!(stage.active_chat_id, None);
    assert!(stage.messages.is_empty());
    assert!(stage.chat_histories.is_empty());
    assert!(stage.chat_pinned_messages.is_empty());
    assert!(stage.chat_scroll_positions.is_empty());
    assert!(inspector.profile.name.is_empty());
    assert!(!inspector.is_open);
}

#[test]
fn fus_d004_stale_async_chat_list_and_history_responses_are_fenced() {
    let mut sidebar = ChatSidebar::new();
    let mut stage = ConversationStage::new();
    let mut inspector = ContextInspector::new();

    let stale_fence = SessionRequestFence {
        auth_generation: 1,
        request_generation: 1,
        token: "token_account_a".to_string(),
        user_id: 10,
    };

    // 1. Stale chat list arriving after logout (`is_authenticated = false`) is ignored.
    assert!(!apply_fenced_chat_list_response(
        &stale_fence,
        false,
        2,
        0,
        None,
        None,
        &mut sidebar,
        &mut stage,
        &mut inspector,
        &sample_server_chats(),
    ));
    assert!(sidebar.dialogs.is_empty());

    // 2. Stale chat list arriving after switching to Account B (`user_id = 20`, `token_account_b`) is ignored.
    assert!(!apply_fenced_chat_list_response(
        &stale_fence,
        true,
        2,
        1,
        Some("token_account_b"),
        Some(20),
        &mut sidebar,
        &mut stage,
        &mut inspector,
        &sample_server_chats(),
    ));
    assert!(sidebar.dialogs.is_empty());

    // 3. Stale chat list from an older request_generation (1 vs current 2) is ignored.
    assert!(!apply_fenced_chat_list_response(
        &stale_fence,
        true,
        1,
        2,
        Some("token_account_a"),
        Some(10),
        &mut sidebar,
        &mut stage,
        &mut inspector,
        &sample_server_chats(),
    ));
    assert!(sidebar.dialogs.is_empty());

    // 4. Matching latest fence applies the authoritative chat list.
    assert!(apply_fenced_chat_list_response(
        &stale_fence,
        true,
        1,
        1,
        Some("token_account_a"),
        Some(10),
        &mut sidebar,
        &mut stage,
        &mut inspector,
        &sample_server_chats(),
    ));
    assert_eq!(sidebar.dialogs.len(), 2);

    // 5. History response with mismatched auth_generation, user_id, or older history_request_generation is ignored.
    assert!(select_authoritative_dialog(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        101
    ));
    let stale_history_msg = vec![HistoricalMessage {
        message_id: 9001,
        chat_id: 101,
        sender_id: 10,
        text: "Should be blocked when fence is stale".to_string(),
        date: None,
        is_outgoing: Some(true),
    }];
    assert!(!apply_fenced_history_response(
        &stale_fence,
        true,
        1,
        2, // newer history request was issued
        Some("token_account_a"),
        Some(10),
        &sidebar,
        &mut stage,
        101,
        stale_history_msg.clone(),
    ));
    assert!(stage.messages.is_empty());

    // 6. Matching history fence populates history without fabricating `"Earlier"` when `date` is `None`.
    assert!(apply_fenced_history_response(
        &stale_fence,
        true,
        1,
        1,
        Some("token_account_a"),
        Some(10),
        &sidebar,
        &mut stage,
        101,
        stale_history_msg,
    ));
    assert_eq!(stage.messages.len(), 1);
    assert_eq!(stage.messages[0].timestamp, "");
}

#[test]
fn fus_d004_send_guard_rejects_unselected_or_unauthenticated_sends_and_preserves_composer_draft() {
    let mut sidebar = ChatSidebar::new();
    let mut stage = ConversationStage::new();
    let mut inspector = ContextInspector::new();
    let mut composer = CanonicalComposer::new();

    apply_authoritative_chat_list(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        &sample_server_chats(),
    );

    // User types a draft while no chat is selected (`active_chat_id == None`).
    composer.set_text("Draft before selecting any chat");

    // 1. Unselected send via Enter path: rejected, draft remains intact.
    assert!(validate_selected_send_target(
        true,
        Some("token_a"),
        Some(10),
        &sidebar,
        &stage
    )
    .is_none());
    assert!(try_consume_composer_for_send(
        true,
        Some("token_a"),
        Some(10),
        &sidebar,
        &stage,
        &mut composer,
    )
    .is_none());
    assert_eq!(composer.text(), "Draft before selecting any chat");

    // 2. Unselected send via composer UI button click (`submitted_message` set, buffer cleared):
    //    rejected and draft is restored into `composer.text()`.
    let clicked_draft = composer.text().to_string();
    composer.clear();
    composer.submitted_message = Some(clicked_draft);
    assert!(try_consume_composer_for_send(
        true,
        Some("token_a"),
        Some(10),
        &sidebar,
        &stage,
        &mut composer,
    )
    .is_none());
    assert_eq!(composer.submitted_message, None);
    assert_eq!(composer.text(), "Draft before selecting any chat");

    // 3. Mismatched or non-authoritative selection (e.g. fabricated chat ID 1) is rejected.
    sidebar.active_chat_id = Some(1);
    stage.active_chat_id = Some(1);
    assert!(try_consume_composer_for_send(
        true,
        Some("token_a"),
        Some(10),
        &sidebar,
        &stage,
        &mut composer,
    )
    .is_none());
    assert_eq!(composer.text(), "Draft before selecting any chat");

    // 4. Coherently selecting authoritative chat 101 while unauthenticated still rejects and preserves draft.
    assert!(select_authoritative_dialog(
        &mut sidebar,
        &mut stage,
        &mut inspector,
        101
    ));
    assert!(try_consume_composer_for_send(
        false,
        Some("token_a"),
        Some(10),
        &sidebar,
        &stage,
        &mut composer,
    )
    .is_none());
    assert_eq!(composer.text(), "Draft before selecting any chat");

    // 5. Authenticated + coherently selected authoritative chat 101 consumes draft and returns verified target.
    let (target, consumed_text) = try_consume_composer_for_send(
        true,
        Some("token_a"),
        Some(10),
        &sidebar,
        &stage,
        &mut composer,
    )
    .expect("send should succeed once an authoritative chat is selected");
    assert_eq!(target.chat_id, 101);
    assert_eq!(target.user_id, 10);
    assert_eq!(target.peer_name, "Product Sync");
    assert_eq!(consumed_text, "Draft before selecting any chat");
    assert_eq!(composer.text(), "");
}

#[test]
fn fus_d004_shell_source_contract_eliminates_unwrap_or_1_and_fixed_chat_4_routing() {
    let shell_src = include_str!("../src/views/shell.rs");
    assert!(
        !shell_src.contains("active_chat_id.unwrap_or(1)"),
        "shell.rs must not route outgoing messages or updates to fallback chat_id 1"
    );
    assert!(
        !shell_src.contains("user_id.unwrap_or(1)"),
        "shell.rs must not fabricate sender user_id 1"
    );
    assert!(
        !shell_src.contains("active_chat_id == 4"),
        "shell.rs must not hardcode chat_id 4 as an AI bot"
    );
    assert!(
        !shell_src.contains("\"last seen recently\""),
        "shell.rs must not fabricate 'last seen recently' subtitles"
    );
}
