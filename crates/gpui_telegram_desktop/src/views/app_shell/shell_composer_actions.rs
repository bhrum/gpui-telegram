use std::path::PathBuf;

use gpui::{AsyncApp, Context, WeakEntity};
use gpui_telegram_client::net::RichSendMessageRequest;
use gpui_telegram_proto::message::MessageContent as ProtoMessageContent;

use crate::i18n::tr_str;
use crate::views::app_shell::shell_actions::resolve_active_target_chat_id;
use crate::views::app_shell::shell_truth::try_consume_composer_for_send;
use crate::views::boxes::modal_types::ModalDialogKind;
use crate::views::composer::voice_gesture::VoiceGestureState;
use crate::views::shell::AppShell;

impl AppShell {
    pub fn send_rich_text_options(&mut self, text: String, silent: bool, scheduled_at_ms: Option<i64>, cx: &mut Context<Self>) {
        let trimmed = text.trim().to_string();
        if trimmed.is_empty() { return; }
        let Some(chat_id) = resolve_active_target_chat_id(self.stage.read(cx).active_chat_id, self.sidebar.read(cx).active_chat_id) else {
            return;
        };
        let req = RichSendMessageRequest {
            chat_id: chat_id as i64, text: Some(trimmed.clone()), content: Some(ProtoMessageContent::plain_text(trimmed)),
            entities: Vec::new(), reply_to: None, quote_text: None, topic_id: None,
            client_nonce: Some(self.client.next_client_nonce()), forward_from_chat_id: None,
            forward_from_msg_id: None, silent, scheduled_at_ms, inline_keyboard: None,
        };
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let _ = client.send_message_durable(req).await;
            let _ = weak.update(app, |s, cx| s.load_historical_messages(chat_id, cx));
        }).detach();
    }

    pub fn upload_and_send_paths(&mut self, paths: Vec<PathBuf>, cx: &mut Context<Self>) {
        let Some(chat_id) = resolve_active_target_chat_id(self.stage.read(cx).active_chat_id, self.sidebar.read(cx).active_chat_id) else {
            return;
        };
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            for path in paths {
                let Ok(bytes) = tokio::fs::read(&path).await else { continue; };
                let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("attachment.bin").to_string();
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
                let (mime, kind) = match ext.as_str() {
                    "png" => ("image/png", "photo"),
                    "jpg" | "jpeg" | "webp" | "gif" => ("image/jpeg", "photo"),
                    "mp4" | "mov" | "webm" => ("video/mp4", "video"),
                    _ => ("application/octet-stream", "doc"),
                };
                if let Ok(up) = client.upload_media(&filename, mime, bytes).await {
                    let content = match kind {
                        "photo" => ProtoMessageContent::Photo {
                            file_id: up.file_id.clone(), caption: None,
                            width: up.width.unwrap_or(0), height: up.height.unwrap_or(0),
                        },
                        "video" => ProtoMessageContent::Video {
                            file_id: up.file_id.clone(), duration: up.duration_seconds.unwrap_or(0),
                            width: up.width.unwrap_or(0), height: up.height.unwrap_or(0), caption: None,
                        },
                        _ => ProtoMessageContent::Document {
                            file_id: up.file_id.clone(), file_name: filename.clone(),
                            mime_type: mime.to_string(), size_bytes: up.size_bytes,
                        },
                    };
                    let req = RichSendMessageRequest {
                        chat_id: chat_id as i64, text: Some(filename), content: Some(content),
                        entities: Vec::new(), reply_to: None, quote_text: None, topic_id: None,
                        client_nonce: Some(client.next_client_nonce()), forward_from_chat_id: None,
                        forward_from_msg_id: None, silent: false, scheduled_at_ms: None, inline_keyboard: None,
                    };
                    let _ = client.send_message_durable(req).await;
                }
            }
            let _ = weak.update(app, |s, cx| s.load_historical_messages(chat_id, cx));
        }).detach();
    }

    pub fn handle_composer_pending_actions(&mut self, cx: &mut Context<Self>) {
        let lang = self.active_lang_id();
        let active_chat = resolve_active_target_chat_id(self.stage.read(cx).active_chat_id, self.sidebar.read(cx).active_chat_id);
        let is_authenticated = self.is_authenticated;
        let session_token = self.session_token.clone();
        let user_id = self.user_id;
        let submitted = self.sidebar.update(cx, |sb, sb_cx| {
            self.stage.update(sb_cx, |st, st_cx| {
                self.composer.update(st_cx, |c, _| {
                    try_consume_composer_for_send(
                        is_authenticated,
                        session_token.as_deref(),
                        user_id,
                        sb,
                        st,
                        c,
                    )
                    .map(|(_, text)| text)
                })
            })
        });
        let (silent_text, sched_text, edit_commit, attached_files, sticker, dice, gif, open_poll, open_loc, open_contact, is_rec_ui, is_rec_done) = self.composer.update(cx, |c, _| {
            let rec_ui = c.is_recording_voice();
            let rec_done = matches!(c.voice_gesture.state, VoiceGestureState::Completed { .. });
            if rec_done || matches!(c.voice_gesture.state, VoiceGestureState::Discarded) {
                c.voice_gesture.state = VoiceGestureState::Idle;
            }
            (
                c.pending_send_silent.take(), c.pending_schedule_message.take(),
                c.pending_edit_commit.take(), c.pending_attached_files.take(),
                c.pending_send_sticker_emoji.take(), c.pending_send_dice.take(),
                c.pending_send_gif.take(),
                if c.pending_open_poll_modal { c.pending_open_poll_modal = false; true } else { false },
                if c.pending_open_location_modal { c.pending_open_location_modal = false; true } else { false },
                if c.pending_open_contact_modal { c.pending_open_contact_modal = false; true } else { false },
                rec_ui, rec_done,
            )
        });

        if is_rec_ui && !self.voice_recorder.is_recording() {
            self.toggle_voice_recording(true, cx);
        } else if !is_rec_ui && self.voice_recorder.is_recording() {
            if is_rec_done {
                self.toggle_voice_recording(false, cx);
            } else {
                let _ = self.voice_recorder.stop();
            }
        }
        if let Some(text) = submitted { self.append_outgoing_text(text, cx); }
        if let Some(text) = silent_text { self.send_rich_text_options(text, true, None, cx); }
        if let Some(text) = sched_text {
            let sched_ms = chrono::Utc::now().timestamp_millis() + 60_000;
            self.send_rich_text_options(text, false, Some(sched_ms), cx);
        }
        if let Some((mid, new_text)) = edit_commit {
            if let Some(cid) = active_chat {
                self.stage.update(cx, |st, _| st.edit_message_text(mid, new_text.clone()));
                let client = self.client.clone();
                cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                    let _ = client.edit_message(cid as i64, mid as i64, new_text, Vec::new()).await;
                }).detach();
            }
        }
        if let Some(paths) = attached_files { self.upload_and_send_paths(paths, cx); }
        if let Some(emoji) = sticker { self.append_outgoing_text(emoji, cx); }
        if let Some(d) = dice { self.append_outgoing_text(d, cx); }
        if let Some((_id, title)) = gif { self.append_outgoing_text(title, cx); }
        if open_poll { self.open_modal_dialog(tr_str(lang, "创建投票", "Create Poll"), ModalDialogKind::CreatePoll, 420.0, 480.0, cx); }
        if open_loc { self.window_runtime.enqueue_toast(tr_str(lang, "请选择地图位置并发送", "Select a location to share"), 2200); }
        if open_contact {
            self.refresh_contacts(cx);
            self.open_modal_dialog(tr_str(lang, "分享联系人", "Share Contact"), ModalDialogKind::Contacts, 420.0, 480.0, cx);
        }
    }
}
