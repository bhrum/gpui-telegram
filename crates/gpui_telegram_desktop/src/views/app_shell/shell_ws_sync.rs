//! Outgoing text message dispatch, WebSocket inbound stream processing, ACK/PTS reconciliation,
//! and gap-recovery diff application for `AppShell`.

use std::sync::Arc;

use gpui::{AsyncApp, Context, WeakEntity};
use gpui_telegram_client::net::RichSendMessageRequest;
use gpui_telegram_client::SyncOutcome;
use gpui_telegram_core::types::ChatId;
use gpui_telegram_proto::sync::{
    PtsUpdate, SyncGetDifferenceRequest, SyncGetDifferenceResponse, UpdateEvent,
};
use gpui_telegram_proto::ws::{WsClientFrame, WsServerFrame};

use crate::i18n::tr_str;
use crate::views::app_shell::shell_actions::peer_avatar_color;
use crate::views::app_shell::shell_state::reconcile_stage_message_list;
use crate::views::app_shell::shell_truth::{
    derive_initials, format_authoritative_timestamp, validate_selected_send_target,
    SessionRequestFence,
};
use crate::views::chat_sidebar::DialogDeliveryStatus;
use crate::views::shell::AppShell;
use crate::views::stage::cards::{
    DeliveryStatus, MessageContent, MessageDirection, MessageItemModel,
};

impl AppShell {
    /// Appends an optimistic outgoing text message to the stage/sidebar and dispatches `send_message_rich`.
    pub fn append_outgoing_text(&mut self, text: String, cx: &mut Context<Self>) {
        let verified_target = {
            let sb = self.sidebar.read(cx);
            let st = self.stage.read(cx);
            validate_selected_send_target(
                self.is_authenticated,
                self.session_sync.session_token.as_deref(),
                self.session_sync.user_id,
                sb,
                st,
            )
        };
        let Some(target) = verified_target else {
            return;
        };
        let active_chat_id = target.chat_id;
        let my_user_id = target.user_id;
        let Some(token) = self.session_sync.session_token.clone() else {
            return;
        };

        let msg_id = self.client.next_client_nonce();
        self.session_sync
            .pending_outgoing_nonces
            .insert(msg_id, active_chat_id);

        let peer_name = self.stage.read(cx).chat_title.clone();
        let my_name = self
            .session_sync
            .username
            .clone()
            .unwrap_or_else(|| "Me".to_string());
        let my_initials = derive_initials(&my_name);
        let my_color = peer_avatar_color(my_user_id);
        let now_ts = format_authoritative_timestamp(Some(chrono::Utc::now().timestamp_millis()));
        let fence = SessionRequestFence {
            auth_generation: self.session_sync.auth_generation,
            request_generation: 0,
            token: token.clone(),
            user_id: my_user_id,
        };

        self.stage.update(cx, |st, st_cx| {
            st.add_message(MessageItemModel {
                id: msg_id,
                sender_id: my_user_id,
                sender_name: my_name.clone(),
                avatar_initials: my_initials,
                avatar_color: my_color,
                direction: MessageDirection::Outgoing,
                timestamp: now_ts.clone(),
                delivery_status: DeliveryStatus::Sending,
                reply_to: None,
                reactions: vec![],
                content: MessageContent::Text {
                    text: text.clone(),
                    is_edited: false,
                },
                bot_keyboard: None,
                is_pinned: false,
            });
            st_cx.notify();
        });

        let is_bot_dialog = {
            let sb = self.sidebar.read(cx);
            sb.bot_ids.contains(&active_chat_id)
                || sb
                    .dialogs
                    .iter()
                    .any(|d| d.id == active_chat_id && d.folder == "Bots")
        };
        self.sidebar.update(cx, |sb, sb_cx| {
            if let Some(d) = sb.dialogs.iter_mut().find(|d| d.id == active_chat_id) {
                d.last_message = super::shell_pts_updates::format_sidebar_preview(&text);
                d.timestamp = now_ts.clone();
                d.delivery_status = DialogDeliveryStatus::Sending;
            }
            sb_cx.notify();
        });

        // Clear any pending draft for this chat upon sending
        self.session_sync.last_draft_text_by_chat.remove(&active_chat_id);
        self.session_sync.last_saved_draft_by_chat.remove(&active_chat_id);
        self.session_sync.draft_debounce_gen = self.session_sync.draft_debounce_gen.wrapping_add(1);
        self.sidebar.update(cx, |sb, _| sb.set_draft(active_chat_id, String::new()));

        let send_as_peer_id = self
            .session_sync
            .send_as_peers
            .chosen(active_chat_id)
            .or_else(|| self.composer.read(cx).selected_send_as_peer_id);

        {
            let client = self.client.clone();
            let draft_token = token.clone();
            let http_req = RichSendMessageRequest {
                chat_id: active_chat_id as i64,
                text: Some(text.clone()),
                content: Some(gpui_telegram_proto::message::MessageContent::plain_text(
                    text.clone(),
                )),
                entities: Vec::new(),
                reply_to: None,
                quote_text: None,
                topic_id: None,
                client_nonce: Some(msg_id),
                forward_from_chat_id: None,
                forward_from_msg_id: None,
                silent: false,
                scheduled_at_ms: None,
                inline_keyboard: None,
                send_as_peer_id,
            };
            let sent_msg_id = msg_id;
            let http_fence = fence.clone();

            cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
                let res = client.send_message_durable(http_req).await;
                let clear_draft = gpui_telegram_proto::contracts::DraftDto {
                    chat_id: active_chat_id as i64,
                    text: String::new(),
                    entities: Vec::new(),
                    reply_to_msg_id: None,
                    topic_id: None,
                    date_ms: chrono::Utc::now().timestamp_millis(),
                };
                client
                    .local_store
                    .set_draft(gpui_telegram_core::types::ChatId(active_chat_id as i64), None);
                let _ = client
                    .http_client
                    .save_draft(&draft_token, active_chat_id as i64, &clear_draft)
                    .await;
                let _ = weak_shell.update(app, |shell, cx| {
                    if !http_fence.matches(
                        shell.is_authenticated,
                        shell.session_sync.auth_generation,
                        0,
                        shell.session_sync.session_token.as_deref(),
                        shell.session_sync.user_id,
                    ) {
                        return;
                    }
                    shell.session_sync.pending_outgoing_nonces.remove(&sent_msg_id);
                    match res {
                        Ok(resp) => {
                            let server_id = u64::try_from(resp.message_id).unwrap_or(0);
                            if server_id > 0 {
                                shell
                                    .session_sync
                                    .resolved_outgoing_nonces
                                    .insert(sent_msg_id, server_id);
                            }
                            shell.reconcile_outgoing_message(
                                active_chat_id,
                                sent_msg_id,
                                server_id,
                                DeliveryStatus::Sent,
                                resp.pts,
                                cx,
                            );
                        }
                        Err(err) => {
                            shell.session_sync.pending_nonce_reactions.remove(&sent_msg_id);
                            let err_text = format!(
                                "{}: {}",
                                tr_str(shell.active_lang_id(), "发送失败", "Message send failed"),
                                err
                            );
                            shell.window_runtime.enqueue_toast(err_text, 3500);
                        }
                    }
                    cx.notify();
                });
            })
            .detach();
        }

        let is_ai_bot = is_bot_dialog
            || self.is_chat_bot(active_chat_id, cx)
            || peer_name.contains("Autonomous Agent")
            || peer_name.contains("Bot");

        if is_ai_bot {
            let lang = self.active_lang_id();
            self.sidebar.update(cx, |sb, sb_cx| {
                sb.set_typing(
                    active_chat_id,
                    Some(
                        tr_str(lang, "AI 助手正在思考...", "Assistant is thinking...").to_string(),
                    ),
                );
                sb_cx.notify();
            });
        }
    }

    /// Starts the session-fenced WebSocket inbound frame receiver loop upon authentication.
    pub fn start_ws_inbound_stream(&mut self, cx: &mut Context<Self>) {
        if self.session_sync.ws_stream_started {
            return;
        }
        self.session_sync.ws_stream_started = true;

        let stream_client = self.client.clone();
        let stream_auth_gen = self.session_sync.auth_generation;
        let mut frame_rx = stream_client.ws_channel.subscribe();

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            while let Ok(frame) = frame_rx.recv().await {
                let keep_running = weak_shell
                    .update(app, |shell, cx| {
                        if !shell.is_authenticated
                            || shell.session_sync.auth_generation != stream_auth_gen
                            || !Arc::ptr_eq(&shell.client, &stream_client)
                        {
                            return false;
                        }
                        shell.handle_ws_server_frame(frame, cx);
                        true
                    })
                    .unwrap_or(false);
                if !keep_running {
                    break;
                }
            }
        })
        .detach();
    }

    /// Dispatches an inbound `WsServerFrame` from the WebSocket channel to the appropriate handler.
    pub fn handle_ws_server_frame(&mut self, frame: WsServerFrame, cx: &mut Context<Self>) {
        match frame {
            WsServerFrame::Ack {
                client_nonce,
                message_id,
                pts,
                ..
            } => {
                self.handle_ws_ack(client_nonce, message_id, pts, cx);
            }
            WsServerFrame::PtsPush(pts_update) => {
                self.handle_ws_pts_push(pts_update, cx);
            }
            WsServerFrame::SyncDiff(diff_response) => {
                self.handle_ws_sync_diff(diff_response, cx);
            }
            WsServerFrame::Update(update_event) => {
                if let Some(active_chat) = self.stage.read(cx).active_chat_id {
                    self.apply_update_event(ChatId(active_chat as i64), update_event, cx);
                } else {
                    self.apply_update_event(ChatId(0), update_event, cx);
                }
            }
            WsServerFrame::Pong { .. } => {}
            WsServerFrame::Error { code, .. } => {
                if code == 401 {
                    self.logout(cx);
                    self.auth_wizard.update(cx, |w, wcx| {
                        w.set_login_error(Some(
                            "Session expired (401). Please sign in again.".to_string(),
                        ));
                        wcx.notify();
                    });
                }
            }
        }
    }

    /// Reconciles an optimistic outgoing `local_id` to authoritative `server_id`, updating stage/sidebar and flushing queued reactions.
    pub fn reconcile_outgoing_message(
        &mut self,
        chat_id: u64,
        local_id: u64,
        server_id: u64,
        min_status: DeliveryStatus,
        pts: u64,
        cx: &mut Context<Self>,
    ) {
        if pts > 0 {
            self.client.sync_coordinator.set_pts(ChatId(chat_id as i64), pts);
        }
        self.stage.update(cx, |st, st_cx| {
            let mut changed = false;
            if st.active_chat_id == Some(chat_id)
                && reconcile_stage_message_list(&mut st.messages, local_id, server_id, min_status)
            {
                st.remap_message_id(local_id, server_id);
                st.row_height_cache.rebuild_prefix_sums(&st.messages);
                changed = true;
            }
            if let Some(history) = st.chat_histories.get_mut(&chat_id) {
                reconcile_stage_message_list(history, local_id, server_id, min_status);
            }
            if changed {
                st_cx.notify();
            }
        });
        self.sidebar.update(cx, |sb, sb_cx| {
            if let Some(d) = sb.dialogs.iter_mut().find(|d| d.id == chat_id) {
                d.delivery_status = if matches!(min_status, DeliveryStatus::Read) {
                    DialogDeliveryStatus::Read
                } else {
                    DialogDeliveryStatus::Sent
                };
            }
            sb_cx.notify();
        });
        if server_id > 0 {
            if let Some(queued) = self.session_sync.pending_nonce_reactions.remove(&local_id) {
                for (emoji, is_added) in queued {
                    self.dispatch_reaction_mutation(chat_id, server_id, emoji, is_added, cx);
                }
            }
        }
        cx.notify();
    }

    fn handle_ws_ack(
        &mut self,
        client_nonce: u64,
        message_id: i64,
        pts: u64,
        cx: &mut Context<Self>,
    ) {
        let tracked_chat = self
            .session_sync
            .pending_outgoing_nonces
            .remove(&client_nonce);
        let server_id = u64::try_from(message_id).unwrap_or(0);
        if server_id > 0 {
            self.session_sync
                .resolved_outgoing_nonces
                .insert(client_nonce, server_id);
        }
        if let Some(cid) = tracked_chat.or(self.stage.read(cx).active_chat_id) {
            self.reconcile_outgoing_message(
                cid,
                client_nonce,
                server_id,
                DeliveryStatus::Sent,
                pts,
                cx,
            );
        }
    }

    fn handle_ws_pts_push(&mut self, pts_update: PtsUpdate, cx: &mut Context<Self>) {
        let chat_id = pts_update.chat_id;
        match self.client.sync_coordinator.evaluate_update(&pts_update) {
            SyncOutcome::Contiguous => {
                self.apply_update_event_with_pts(
                    chat_id,
                    Some(pts_update.pts),
                    pts_update.event,
                    cx,
                );
                let drained = self.client.sync_coordinator.drain_contiguous_buffer(chat_id);
                for buffered in drained {
                    self.apply_update_event_with_pts(
                        chat_id,
                        Some(buffered.pts),
                        buffered.event,
                        cx,
                    );
                }
            }
            SyncOutcome::Duplicate => {}
            SyncOutcome::GapDetected { last_seen, .. } => {
                self.client.sync_coordinator.buffer_gap_update(pts_update);
                let req = SyncGetDifferenceRequest::new(chat_id, last_seen, 100);
                self.client.ws_channel.send(WsClientFrame::Sync(req));
            }
        }
    }

    fn handle_ws_sync_diff(&mut self, diff: SyncGetDifferenceResponse, cx: &mut Context<Self>) {
        let chat_id = diff.chat_id;
        let diff_pts = diff.pts.max(diff.current_pts);
        let diff_pts_opt = (diff_pts > 0).then_some(diff_pts);

        for msg in diff.new_messages {
            self.apply_update_event_with_pts(chat_id, diff_pts_opt, UpdateEvent::NewMessage(msg), cx);
        }

        for update in diff.other_updates {
            self.apply_update_event_with_pts(chat_id, None, update, cx);
        }

        for pts_up in diff.updates {
            let cid = if pts_up.chat_id.0 > 0 { pts_up.chat_id } else { chat_id };
            self.apply_update_event_with_pts(cid, Some(pts_up.pts), pts_up.event, cx);
        }

        self.client.sync_coordinator.set_pts(chat_id, diff_pts);

        if diff.has_more {
            let req = SyncGetDifferenceRequest::new(chat_id, diff_pts, 100);
            self.client.ws_channel.send(WsClientFrame::Sync(req));
        } else {
            let drained = self.client.sync_coordinator.drain_contiguous_buffer(chat_id);
            for buffered in drained {
                self.apply_update_event_with_pts(
                    chat_id,
                    Some(buffered.pts),
                    buffered.event,
                    cx,
                );
            }
        }
        cx.notify();
    }
}
