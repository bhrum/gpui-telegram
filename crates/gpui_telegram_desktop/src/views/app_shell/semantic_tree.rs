use chrono::Utc;
use gpui::{App, Context};
use gpui_telegram_proto::semantic_tree::{
    ActionExecutionStatus, SemanticAction, SemanticActionReceipt, SemanticComposerNode,
    SemanticInspectorNode, SemanticModalNode, SemanticNavRailNode, SemanticSidebarNode,
    SemanticStageNode, SemanticWindowNode, SemanticWorkspaceTree,
};
use serde_json::json;
use uuid::Uuid;

use crate::views::app_shell::modal_container::ModalContainer;
use crate::views::app_shell::shell_truth::{
    format_authoritative_timestamp, select_authoritative_dialog,
};
use crate::views::app_shell::window_adaptive::WindowLayout;
use crate::components::ui::layers::LayerStackOption;
use crate::views::nav_rail::NavDestination;
use crate::views::shell::{ActiveWorkspacePanel, AppShell};
use crate::views::stage::cards::{
    DeliveryStatus, MessageContent, MessageDirection, MessageItemModel,
};

fn window_layout_label(layout: WindowLayout) -> String {
    match layout {
        WindowLayout::OneColumn => "OneColumn".to_string(),
        WindowLayout::Normal => "Normal".to_string(),
        WindowLayout::ThreeColumn => "ThreeColumn".to_string(),
    }
}

fn active_panel_label(panel: ActiveWorkspacePanel) -> String {
    match panel {
        ActiveWorkspacePanel::Chats => "Chats".to_string(),
        ActiveWorkspacePanel::Calls => "Calls".to_string(),
        ActiveWorkspacePanel::TaskCenter => "TaskCenter".to_string(),
        ActiveWorkspacePanel::McpTools => "McpTools".to_string(),
        ActiveWorkspacePanel::Settings => "Settings".to_string(),
    }
}

fn nav_dest_label(dest: NavDestination) -> String {
    match dest {
        NavDestination::Chats => "Chats".to_string(),
        NavDestination::Folders => "Folders".to_string(),
        NavDestination::Calls => "Calls".to_string(),
        NavDestination::TaskCenter => "TaskCenter".to_string(),
        NavDestination::McpTools => "McpTools".to_string(),
        NavDestination::TalkMode => "TalkMode".to_string(),
        NavDestination::RemoteComputer => "RemoteComputer".to_string(),
        NavDestination::SavedMessages => "SavedMessages".to_string(),
        NavDestination::Settings => "Settings".to_string(),
    }
}

impl AppShell {
    /// Captures the live semantic state tree of `AppShell` and its child GPUI entities.
    pub fn inspect_state(&self, cx: &App) -> SemanticWorkspaceTree {
        let nav = self.nav_rail.read(cx);
        let sb = self.sidebar.read(cx);
        let st = self.stage.read(cx);
        let comp = self.composer.read(cx);
        let insp = self.inspector.read(cx);
        let modal = self.modal_container.read(cx);

        let window_layout = window_layout_label(self.app.window_runtime.adaptive.window_layout);
        let selected_chat_id = st.active_chat_id.or(sb.active_chat_id);
        let composer_text = comp.text().to_string();
        let active_modal_node = if modal.is_modal_active() {
            let title = modal
                .current_box()
                .map(|b| b.title.clone())
                .unwrap_or_else(|| "modal".to_string());
            Some(SemanticModalNode {
                modal_id: title.clone(),
                title,
            })
        } else {
            None
        };
        let active_modal = active_modal_node.as_ref().map(|m| m.modal_id.clone());
        let is_call_active =
            self.rtc_talk.active_call_id.is_some() || self.rtc_talk.talk_mode_overlay.is_some();

        let mut tree = SemanticWorkspaceTree {
            timestamp_ms: Utc::now().timestamp_millis(),
            window_layout: window_layout.clone(),
            selected_chat_id,
            composer_text: composer_text.clone(),
            active_modal,
            is_call_active,
            window: SemanticWindowNode {
                layout: window_layout,
                width_px: self.app.window_runtime.adaptive.current_width,
                height_px: self.app.window_runtime.adaptive.current_height,
                active_panel: active_panel_label(self.active_panel),
                is_authenticated: self.is_authenticated,
            },
            nav_rail: SemanticNavRailNode {
                active_destination: nav_dest_label(nav.active_destination),
                connected: self.session_sync.ws_stream_started,
            },
            sidebar: SemanticSidebarNode {
                dialog_count: sb.dialogs.len(),
                selected_chat_id: sb.active_chat_id,
                search_query: sb.search_query.clone(),
            },
            stage: SemanticStageNode {
                active_chat_id: st.active_chat_id,
                chat_title: st.chat_title.clone(),
                message_count: st.messages.len(),
                is_group_chat: st.is_group_chat,
                pinned_message: st.pinned_message.clone(),
            },
            composer: SemanticComposerNode {
                text: composer_text,
                is_editing: comp.editing_message.is_some(),
                has_reply_target: comp.replying_to.is_some(),
            },
            inspector: SemanticInspectorNode {
                is_open: insp.is_open,
                profile_name: insp.profile.name.clone(),
                shared_media_count: insp.media_items.len(),
            },
            active_modal_node,
            state_hash: String::new(),
        };
        tree.recompute_state_hash();
        tree
    }

    /// Executes a structured `SemanticAction` against live `AppShell` entities and returns a verifiable receipt.
    pub fn perform_action(
        &mut self,
        action: SemanticAction,
        cx: &mut Context<Self>,
    ) -> SemanticActionReceipt {
        let action_id = Uuid::new_v4().to_string();
        let (action_type, success, error, mutation_summary) = match &action {
            SemanticAction::SelectChat { chat_id } => {
                let exists = self
                    .sidebar
                    .read(cx)
                    .dialogs
                    .iter()
                    .any(|d| d.id == *chat_id);
                if !exists {
                    (
                        "SelectChat",
                        false,
                        Some(format!("Unknown chat_id: {chat_id}")),
                        json!({ "chat_id": chat_id, "selected": false }),
                    )
                } else {
                    if self.is_authenticated && self.session_sync.session_token.is_some() {
                        self.sync_selected_chat_inspector_and_stage(*chat_id, cx);
                    } else {
                        let sb_entity = self.sidebar.clone();
                        let st_entity = self.stage.clone();
                        let insp_entity = self.inspector.clone();
                        sb_entity.update(cx, |sb, sb_cx| {
                            st_entity.update(sb_cx, |st, st_cx| {
                                insp_entity.update(st_cx, |insp, _| {
                                    let _ = select_authoritative_dialog(sb, st, insp, *chat_id);
                                });
                            });
                        });
                    }
                    (
                        "SelectChat",
                        true,
                        None,
                        json!({ "chat_id": chat_id, "selected": true }),
                    )
                }
            }
            SemanticAction::ComposeText { text } => {
                let next_text = text.clone();
                self.composer.update(cx, |c, _| {
                    c.set_text(next_text.clone());
                });
                (
                    "ComposeText",
                    true,
                    None,
                    json!({ "text_len": next_text.len() }),
                )
            }
            SemanticAction::SendMessage => {
                let selected_chat = self
                    .stage
                    .read(cx)
                    .active_chat_id
                    .or_else(|| self.sidebar.read(cx).active_chat_id);
                let text = self.composer.read(cx).text().to_string();
                if selected_chat.is_none() {
                    (
                        "SendMessage",
                        false,
                        Some("No chat selected".to_string()),
                        json!({ "sent": false }),
                    )
                } else if text.trim().is_empty() {
                    (
                        "SendMessage",
                        false,
                        Some("Cannot send empty message".to_string()),
                        json!({ "sent": false }),
                    )
                } else {
                    if self.is_authenticated && self.session_sync.session_token.is_some() {
                        self.append_outgoing_text(text.clone(), cx);
                    } else {
                        let next_id = self.session_sync.next_message_id;
                        self.session_sync.next_message_id = next_id.saturating_add(1);
                        let msg = MessageItemModel {
                            id: next_id,
                            sender_id: self.session_sync.user_id.unwrap_or(1),
                            sender_name: self
                                .session_sync
                                .username
                                .clone()
                                .unwrap_or_else(|| "Me".to_string()),
                            avatar_initials: "ME".to_string(),
                            avatar_color: 0,
                            direction: MessageDirection::Outgoing,
                            timestamp: format_authoritative_timestamp(Some(Utc::now().timestamp_millis())),
                            delivery_status: DeliveryStatus::Sent,
                            reply_to: None,
                            reactions: Vec::new(),
                            content: MessageContent::Text {
                                text: text.clone(),
                                is_edited: false,
                            },
                            bot_keyboard: None,
                            is_pinned: false,
                        };
                        self.stage.update(cx, |st, _| {
                            st.messages.push(msg);
                        });
                    }
                    self.composer.update(cx, |c, _| c.clear());
                    (
                        "SendMessage",
                        true,
                        None,
                        json!({ "sent": true, "chat_id": selected_chat }),
                    )
                }
            }
            SemanticAction::OpenModal { modal_id } => {
                let trimmed = modal_id.trim();
                if trimmed.is_empty() {
                    (
                        "OpenModal",
                        false,
                        Some("Cannot open modal with empty modal_id".to_string()),
                        json!({ "modal_id": modal_id, "open": false }),
                    )
                } else {
                    let title = trimmed.to_string();
                    self.modal_container.update(cx, |m: &mut ModalContainer, _| {
                        let _ = m.show_modal(title.clone(), 420.0, 320.0, LayerStackOption::Normal);
                    });
                    (
                        "OpenModal",
                        true,
                        None,
                        json!({ "modal_id": trimmed, "open": true }),
                    )
                }
            }
            SemanticAction::CloseModal => {
                self.modal_container.update(cx, |m: &mut ModalContainer, _| {
                    m.hide_all();
                });
                ("CloseModal", true, None, json!({ "open": false }))
            }
            SemanticAction::ToggleCall { peer_id, is_video } => {
                let currently_active = self.rtc_talk.active_call_id.is_some()
                    || self.rtc_talk.talk_mode_overlay.is_some();
                if currently_active {
                    self.hangup_active_call(cx);
                } else if self.is_authenticated && self.session_sync.session_token.is_some() {
                    let peer_name = format!("Peer {}", peer_id);
                    self.start_webrtc_call(*peer_id, peer_name, *is_video, cx);
                } else {
                    self.rtc_talk.active_call_id = Some(format!("semantic_call_{peer_id}"));
                    let _ = self
                        .rtc_talk
                        .talk_coordinator
                        .transition(gpui_telegram_agent::TalkSessionState::Listening);
                }
                (
                    "ToggleCall",
                    true,
                    None,
                    json!({ "peer_id": peer_id, "is_video": is_video, "is_call_active": !currently_active }),
                )
            }
        };

        cx.notify();
        let snapshot = self.inspect_state(cx);
        SemanticActionReceipt {
            action_id,
            action_type: action_type.to_string(),
            success,
            status: if success {
                ActionExecutionStatus::Succeeded
            } else {
                ActionExecutionStatus::Failed
            },
            error,
            timestamp_ms: snapshot.timestamp_ms,
            mutation_summary,
            resulting_state_hash: snapshot.state_hash,
        }
    }
}
