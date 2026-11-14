use gpui::{AppContext, AsyncApp, Context, Entity, WeakEntity};
use gpui_telegram_agent::{TalkCoordinator, TalkSessionState};
use gpui_telegram_client::net::hermes_client::{HermesCallInviteRequest, HermesTalkTurnRequest};
use gpui_telegram_client::net::RichSendMessageRequest;
use gpui_telegram_core::types::UserId;
use gpui_telegram_proto::contracts::features::{CallInviteRequest, CallSignalRequest};
use gpui_telegram_proto::message::{MessageContent as ProtoMessageContent, VoiceItem};
use gpui_telegram_webrtc::{SignalingFrame, TurnToken, WebRtcCallEngine};

use crate::i18n::tr_str;
use crate::views::app_shell::shell_actions::{
    calls_to_modal_logs, peer_avatar_color, resolve_active_target_chat_id,
};
use crate::views::app_shell::shell_truth::{derive_initials, SessionRequestFence};
use crate::views::call_overlay::TalkModeState;
use crate::views::computer_workbench::HostDeviceSpec;
use crate::views::shell::AppShell;
use crate::views::stage::cards::{
    DeliveryStatus, MessageContent, MessageDirection, MessageItemModel,
};
use crate::views::{CallOverlay, ComputerWorkbench};

fn signaling_frame_to_request(frame: SignalingFrame) -> Option<(CallSignalRequest, Option<u64>)> {
    match frame {
        SignalingFrame::Offer { call_id, sdp } => Some((
            CallSignalRequest {
                call_id,
                signal_type: "offer".into(),
                sdp: Some(sdp),
                candidate: None,
                sdp_mid: None,
                sdp_mline_index: None,
                reason: None,
            },
            None,
        )),
        SignalingFrame::Answer { call_id, sdp } => Some((
            CallSignalRequest {
                call_id,
                signal_type: "answer".into(),
                sdp: Some(sdp),
                candidate: None,
                sdp_mid: None,
                sdp_mline_index: None,
                reason: None,
            },
            None,
        )),
        SignalingFrame::IceCandidate {
            call_id,
            candidate,
            sdp_mid,
            sdp_mline_index,
        } => Some((
            CallSignalRequest {
                call_id,
                signal_type: "ice_candidate".into(),
                sdp: None,
                candidate: Some(candidate),
                sdp_mid,
                sdp_mline_index: sdp_mline_index.map(u32::from),
                reason: None,
            },
            None,
        )),
        SignalingFrame::Ringing { call_id } => Some((
            CallSignalRequest {
                call_id,
                signal_type: "ringing".into(),
                sdp: None,
                candidate: None,
                sdp_mid: None,
                sdp_mline_index: None,
                reason: None,
            },
            None,
        )),
        SignalingFrame::Accept { call_id } => Some((
            CallSignalRequest {
                call_id,
                signal_type: "accept".into(),
                sdp: None,
                candidate: None,
                sdp_mid: None,
                sdp_mline_index: None,
                reason: None,
            },
            None,
        )),
        SignalingFrame::Hangup { call_id, reason } => Some((
            CallSignalRequest {
                call_id,
                signal_type: "hangup".into(),
                sdp: None,
                candidate: None,
                sdp_mid: None,
                sdp_mline_index: None,
                reason: Some(reason),
            },
            None,
        )),
        SignalingFrame::Invite { .. } => None,
    }
}

impl AppShell {
    pub fn active_lang_id(&self) -> &'static str {
        if self.is_chinese_locale || self.preferences.localization.language_code.starts_with("zh") {
            "zh-CN"
        } else {
            "en"
        }
    }

    pub fn is_chat_bot(&self, chat_id: u64, cx: &gpui::App) -> bool {
        if let Some(rc) = self.rich_chats_by_id.get(&chat_id) {
            if rc.is_bot {
                return true;
            }
        }
        let sb = self.sidebar.read(cx);
        if sb.bot_ids.contains(&chat_id) {
            return true;
        }
        if sb.dialogs.iter().any(|d| {
            d.id == chat_id
                && (d.folder == "Bots"
                    || d.peer_name.contains("Bot")
                    || d.peer_name.contains("Autonomous Agent"))
        }) {
            return true;
        }
        let st = self.stage.read(cx);
        st.active_chat_id == Some(chat_id)
            && (st.chat_title.contains("Bot")
                || st.chat_title.contains("Autonomous Agent")
                || (700_000_000..750_000_000).contains(&chat_id))
    }

    pub fn is_active_chat_bot(&self, cx: &gpui::App) -> bool {
        let Some(chat_id) = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        ) else {
            return false;
        };
        self.is_chat_bot(chat_id, cx)
    }

    pub fn current_peer_user_id(&self, cx: &gpui::App) -> Option<u64> {
        let chat_id = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        )?;
        if let Some(pid) = self.chat_peer_user_ids.get(&chat_id).copied() {
            if pid > 0 {
                return Some(pid);
            }
        }
        if let Some(rc) = self.rich_chats_by_id.get(&chat_id) {
            if let Some(pid) = rc.peer_user_id {
                if pid > 0 {
                    return Some(pid);
                }
            }
            if rc.is_group || rc.is_channel {
                return None;
            }
        }
        if let Some(contact) = self
            .contacts_list
            .iter()
            .find(|c| c.chat_id == Some(chat_id as i64) || c.user_id == chat_id)
        {
            return Some(contact.user_id);
        }
        if (700_000_000..750_000_000).contains(&chat_id) {
            return Some(777000);
        }
        let sb = self.sidebar.read(cx);
        if let Some(d) = sb.dialogs.iter().find(|d| d.id == chat_id) {
            if d.folder == "Groups" || d.folder == "Channels" {
                return None;
            }
        }
        if self.is_chat_bot(chat_id, cx) {
            return Some(chat_id);
        }
        None
    }

    pub fn sync_calls_history(&mut self, cx: &mut Context<Self>) {
        let (Some(token), Some(uid)) = (self.session_token.clone(), self.user_id) else {
            return;
        };
        let fence = SessionRequestFence {
            auth_generation: self.auth_generation,
            request_generation: 0,
            token,
            user_id: uid,
        };
        self.panel_state.calls_loading = true;
        self.panel_state.calls_error = None;
        cx.notify();
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.list_calls().await;
            let _ = weak.update(app, |shell, cx| {
                if !fence.matches(
                    shell.is_authenticated,
                    shell.auth_generation,
                    0,
                    shell.session_token.as_deref(),
                    shell.user_id,
                ) {
                    return;
                }
                shell.panel_state.calls_loading = false;
                shell.panel_state.calls_loaded = true;
                match res {
                    Ok(calls) => {
                        shell.panel_state.calls_error = None;
                        let logs = calls_to_modal_logs(
                            &calls,
                            &shell.contacts_list,
                            shell.active_lang_id(),
                        );
                        shell
                            .modal_container
                            .update(cx, |m, _| m.dialog.sync_calls(logs));
                        shell.panel_state.calls = calls;
                    }
                    Err(err) => {
                        shell.panel_state.calls_error = Some(err.to_string());
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn bind_talk_overlay_observer(
        &self,
        overlay: &Entity<CallOverlay>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            overlay,
            |shell: &mut AppShell, ov: Entity<CallOverlay>, cx: &mut Context<AppShell>| {
                let (should_close, pending_hangup, rating_opt) = ov.update(cx, |o, _| {
                    let ph = if o.pending_hangup {
                        o.pending_hangup = false;
                        true
                    } else {
                        false
                    };
                    (o.should_close, ph, o.take_rating_submission())
                });
                if let Some(stars) = rating_opt {
                    if let Some(cid) = shell.active_call_id.clone() {
                        let c = shell.client.clone();
                        cx.spawn(async move |_: WeakEntity<AppShell>, _: &mut AsyncApp| {
                            let _ = c.rate_call(&cid, stars).await;
                        })
                        .detach();
                    }
                }
                if pending_hangup && !should_close {
                    if let Some(cid) = shell.active_call_id.clone() {
                        let c = shell.client.clone();
                        cx.spawn(async move |_: WeakEntity<AppShell>, _: &mut AsyncApp| {
                            let _ = c.call_hangup(&cid).await;
                        })
                        .detach();
                    }
                }
                if should_close {
                    shell.hangup_active_call(cx);
                }
            },
        )
        .detach();
    }

    pub fn start_ai_talk_mode_session(&mut self, is_video: bool, cx: &mut Context<Self>) {
        let (Some(token), Some(my_id)) = (self.session_token.clone(), self.user_id) else {
            return;
        };
        let fence = SessionRequestFence {
            auth_generation: self.auth_generation,
            request_generation: 0,
            token,
            user_id: my_id,
        };
        let lang = self.active_lang_id();
        let pal = self.palette;
        let chat_opt = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        let peer_name = chat_opt
            .and_then(|cid| {
                self.rich_chats_by_id
                    .get(&cid)
                    .map(|rc| {
                        rc.peer_display_name
                            .clone()
                            .filter(|s| !s.trim().is_empty())
                            .unwrap_or_else(|| rc.title.clone())
                    })
                    .filter(|s| !s.trim().is_empty())
            })
            .or_else(|| {
                let title = self.stage.read(cx).chat_title.clone();
                if title.trim().is_empty() {
                    None
                } else {
                    Some(title)
                }
            })
            .unwrap_or_else(|| "Hermes AI".to_string());
        let callee_id = chat_opt
            .and_then(|cid| {
                self.rich_chats_by_id
                    .get(&cid)
                    .and_then(|rc| rc.peer_user_id)
                    .or_else(|| self.chat_peer_user_ids.get(&cid).copied())
            })
            .or(chat_opt)
            .unwrap_or(777000);

        self.computer_overlay = None;
        self.media_viewer_overlay = None;
        self.iv_viewer_overlay = None;
        self.main_menu_drawer.update(cx, |d, _| d.close());
        self.talk_coordinator = TalkCoordinator::new();
        self.active_talk_session_is_ai = true;
        self.active_talk_bot_chat_id = chat_opt;
        self.active_talk_session_id = None;

        let overlay = cx.new(|_| {
            let mut o = CallOverlay::new_live(
                peer_name.clone(),
                Some("Hermes Voice Agent".to_string()),
                None,
                pal,
                lang,
            );
            o.is_camera_on = is_video;
            o.latency_ms = 42;
            o.set_state(TalkModeState::Thinking);
            o.add_transcript(
                "System",
                tr_str(
                    lang,
                    "正在建立 AI 全双工语音会话...",
                    "Connecting Hermes AI Talk Mode...",
                ),
                false,
            );
            o
        });
        self.bind_talk_overlay_observer(&overlay, cx);
        self.talk_mode_overlay = Some(overlay);
        cx.notify();

        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let invite_res = client
                .hermes_call_invite(&HermesCallInviteRequest {
                    callee_id,
                    is_video,
                    chat_id: chat_opt.map(|id| id as i64),
                })
                .await;
            let _ = weak.update(app, |shell, cx| {
                if !fence.matches(
                    shell.is_authenticated,
                    shell.auth_generation,
                    0,
                    shell.session_token.as_deref(),
                    shell.user_id,
                ) {
                    return;
                }
                match invite_res {
                    Ok(rec) => {
                        let sid = rec
                            .talk_session_id
                            .clone()
                            .unwrap_or_else(|| format!("talk_{}", rec.call_id));
                        shell.active_call_id = Some(rec.call_id.clone());
                        shell.active_talk_session_id = Some(sid.clone());
                        shell.talk_coordinator.session_id = sid;
                        let _ = shell.talk_coordinator.transition(TalkSessionState::Listening);
                        let bot_display = rec
                            .bot_name
                            .clone()
                            .filter(|s| !s.trim().is_empty())
                            .unwrap_or_else(|| peer_name.clone());
                        if let Some(ov) = shell.talk_mode_overlay.clone() {
                            ov.update(cx, |o, _| {
                                o.is_connected = true;
                                o.set_signal_bars(4);
                                o.latency_ms = 38;
                                o.set_state(TalkModeState::Listening);
                                o.add_transcript(
                                    &bot_display,
                                    tr_str(
                                        shell.active_lang_id(),
                                        "AI 实时语音已连接，可说话或发送语音回合。",
                                        "AI Talk Mode live. Speak, push-to-talk, or send a voice turn.",
                                    ),
                                    false,
                                );
                            });
                        }
                        shell.sync_calls_history(cx);
                    }
                    Err(err) => {
                        shell.talk_mode_overlay = None;
                        shell.active_talk_session_is_ai = false;
                        shell.active_talk_bot_chat_id = None;
                        shell.window_runtime.enqueue_toast(
                            format!(
                                "{}: {}",
                                tr_str(shell.active_lang_id(), "AI 语音连接失败", "AI Talk failed"),
                                err
                            ),
                            3200,
                        );
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub fn send_ai_talk_voice_turn(
        &mut self,
        transcript: String,
        barge_in: bool,
        cx: &mut Context<Self>,
    ) {
        let clean = transcript.trim().to_string();
        if clean.is_empty() && !barge_in {
            return;
        }
        let (Some(token), Some(my_id)) = (self.session_token.clone(), self.user_id) else {
            return;
        };
        let fence = SessionRequestFence {
            auth_generation: self.auth_generation,
            request_generation: 0,
            token,
            user_id: my_id,
        };
        let lang = self.active_lang_id();
        if barge_in || self.talk_coordinator.state == TalkSessionState::Speaking {
            self.talk_coordinator.handle_barge_in();
            self.audio_player.stop();
        }
        let _ = self.talk_coordinator.transition(TalkSessionState::Thinking);
        if let Some(ov) = self.talk_mode_overlay.clone() {
            ov.update(cx, |o, _| {
                o.set_state(TalkModeState::Thinking);
                if !clean.is_empty() {
                    o.add_transcript(tr_str(lang, "我", "You"), &clean, true);
                }
            });
        }
        cx.notify();

        let bot_chat_id = self.active_talk_bot_chat_id;
        let bot_id = bot_chat_id.and_then(|cid| {
            self.rich_chats_by_id
                .get(&cid)
                .and_then(|rc| rc.peer_user_id)
                .or_else(|| self.chat_peer_user_ids.get(&cid).copied())
                .or(Some(cid))
        });
        let session_id = self.active_talk_session_id.clone();
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client
                .hermes_talk_turn(&HermesTalkTurnRequest {
                    session_id,
                    chat_id: bot_chat_id.map(|id| id as i64),
                    bot_id,
                    transcript: clean,
                    voice_preset: Some("hermes-neural".to_string()),
                    barge_in: Some(barge_in),
                })
                .await;
            let _ = weak.update(app, |shell, cx| {
                if !fence.matches(
                    shell.is_authenticated,
                    shell.auth_generation,
                    0,
                    shell.session_token.as_deref(),
                    shell.user_id,
                ) {
                    return;
                }
                match res {
                    Ok(resp) => {
                        shell.active_talk_session_id = Some(resp.session_id.clone());
                        shell.talk_coordinator.session_id = resp.session_id.clone();
                        let _ = shell.talk_coordinator.transition(TalkSessionState::Speaking);
                        for seg in &resp.tts_segments {
                            let _ = shell
                                .talk_coordinator
                                .enqueue_audio(seg.as_bytes().to_vec());
                        }
                        if let Some(ov) = shell.talk_mode_overlay.clone() {
                            ov.update(cx, |o, _| {
                                if !resp.reply_text.trim().is_empty() {
                                    let peer = o.peer_name.clone();
                                    o.add_transcript(&peer, &resp.reply_text, false);
                                }
                                if resp.state == "listening" || resp.tts_segments.is_empty() {
                                    o.set_state(TalkModeState::Listening);
                                } else {
                                    o.set_state(TalkModeState::Speaking);
                                }
                            });
                        }
                        if resp.state == "listening" || resp.tts_segments.is_empty() {
                            shell.talk_coordinator.resume_listening();
                        }
                        if let Some(cid) = bot_chat_id {
                            shell.load_historical_messages(cid, cx);
                        }
                    }
                    Err(err) => {
                        shell.talk_coordinator.resume_listening();
                        if let Some(ov) = shell.talk_mode_overlay.clone() {
                            ov.update(cx, |o, _| o.set_state(TalkModeState::Listening));
                        }
                        shell.window_runtime.enqueue_toast(
                            format!(
                                "{}: {}",
                                tr_str(shell.active_lang_id(), "语音回合失败", "Talk turn failed"),
                                err
                            ),
                            2800,
                        );
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub fn interrupt_ai_talk_barge_in(&mut self, cx: &mut Context<Self>) {
        self.talk_coordinator.handle_barge_in();
        self.audio_player.stop();
        let lang = self.active_lang_id();
        if let Some(ov) = self.talk_mode_overlay.clone() {
            ov.update(cx, |o, _| {
                o.set_state(TalkModeState::Listening);
                o.add_transcript(
                    "System",
                    tr_str(
                        lang,
                        "已触发 <60ms 打断 (Barge-In)，恢复收听中",
                        "Barge-In triggered (<60ms), listening resumed",
                    ),
                    false,
                );
            });
        }
        let Some(token) = self.session_token.clone() else {
            cx.notify();
            return;
        };
        let _ = token;
        let client = self.client.clone();
        let session_id = self.active_talk_session_id.clone();
        let chat_id = self.active_talk_bot_chat_id.map(|id| id as i64);
        cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
            let _ = client
                .hermes_talk_turn(&HermesTalkTurnRequest {
                    session_id,
                    chat_id,
                    bot_id: None,
                    transcript: String::new(),
                    voice_preset: None,
                    barge_in: Some(true),
                })
                .await;
        })
        .detach();
        cx.notify();
    }

    pub fn start_webrtc_call(
        &mut self,
        peer_id: u64,
        peer_name: String,
        is_video: bool,
        cx: &mut Context<Self>,
    ) {
        if self.is_active_chat_bot(cx) {
            self.start_ai_talk_mode_session(is_video, cx);
            return;
        }
        let (Some(token), Some(my_id)) = (self.session_token.clone(), self.user_id) else {
            return;
        };
        let fence = SessionRequestFence {
            auth_generation: self.auth_generation,
            request_generation: 0,
            token,
            user_id: my_id,
        };
        let lang = self.active_lang_id();
        let pal = self.palette;
        self.computer_overlay = None;
        self.media_viewer_overlay = None;
        self.iv_viewer_overlay = None;
        self.active_talk_session_is_ai = false;
        self.active_talk_bot_chat_id = None;
        self.active_talk_session_id = None;
        self.main_menu_drawer.update(cx, |d, _| d.close());
        let overlay = cx.new(|_| {
            let mut o = CallOverlay::new_live(peer_name.clone(), None, None, pal, lang);
            o.is_camera_on = is_video;
            o
        });
        self.bind_talk_overlay_observer(&overlay, cx);
        self.talk_mode_overlay = Some(overlay);
        cx.notify();

        let chat_opt = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        )
        .map(|id| id as i64);
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let turn_json = client.get_turn_credentials().await.ok();
            let invite_res = client
                .call_invite(&CallInviteRequest {
                    callee_id: peer_id,
                    is_video,
                    chat_id: chat_opt,
                })
                .await;
            let _ = weak.update(app, |shell, cx| {
                if !fence.matches(
                    shell.is_authenticated,
                    shell.auth_generation,
                    0,
                    shell.session_token.as_deref(),
                    shell.user_id,
                ) {
                    return;
                }
                match invite_res {
                    Ok(rec) => {
                        let tokens: Vec<TurnToken> = turn_json
                            .as_ref()
                            .and_then(|v| {
                                v.get("iceServers")
                                    .or_else(|| v.get("turn_tokens"))
                                    .cloned()
                            })
                            .and_then(|v| serde_json::from_value(v).ok())
                            .unwrap_or_default();
                        shell.webrtc_engine = WebRtcCallEngine::new(tokens);
                        let num_id = rec
                            .call_id
                            .parse::<i64>()
                            .unwrap_or(rec.started_at_ms.max(1));
                        shell.webrtc_engine.start_outgoing(
                            num_id,
                            UserId(my_id),
                            UserId(peer_id),
                            is_video,
                        );
                        shell.active_call_id = Some(rec.call_id.clone());
                        let emojis = shell
                            .webrtc_engine
                            .emoji_fingerprint()
                            .map(|f| f.emoji_chars.join(" "));
                        let outbound = shell.webrtc_engine.drain_outbound_frames();
                        for frame in outbound {
                            if let Some((mut sig, target)) = signaling_frame_to_request(frame) {
                                sig.call_id = rec.call_id.clone();
                                let c2 = client.clone();
                                cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                                    let _ = c2.send_call_signal(sig, target.or(Some(peer_id))).await;
                                })
                                .detach();
                            }
                        }
                        if let Some(ov) = shell.talk_mode_overlay.clone() {
                            ov.update(cx, |o, _| {
                                if let Some(e) = emojis {
                                    o.e2ee_emojis = e;
                                }
                                o.is_connected = true;
                                o.set_signal_bars(4);
                                o.set_state(TalkModeState::Listening);
                            });
                        }
                        shell.sync_calls_history(cx);
                    }
                    Err(err) => {
                        shell.talk_mode_overlay = None;
                        shell.window_runtime.enqueue_toast(
                            format!(
                                "{}: {}",
                                tr_str(shell.active_lang_id(), "通话发起失败", "Call failed"),
                                err
                            ),
                            3200,
                        );
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub fn hangup_active_call(&mut self, cx: &mut Context<Self>) {
        self.webrtc_engine.hangup_local("user_hangup");
        let _ = self.webrtc_engine.drain_outbound_frames();
        self.talk_coordinator.close();
        self.audio_player.stop();
        self.active_talk_session_is_ai = false;
        self.active_talk_bot_chat_id = None;
        self.active_talk_session_id = None;
        if let Some(call_id) = self.active_call_id.take() {
            let client = self.client.clone();
            cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                let _ = client.call_hangup(&call_id).await;
                let _ = weak.update(app, |shell, cx| shell.sync_calls_history(cx));
            })
            .detach();
        }
        self.talk_mode_overlay = None;
        cx.notify();
    }

    pub fn open_computer_workbench_live(&mut self, cx: &mut Context<Self>) {
        let (Some(token), Some(my_id)) = (self.session_token.clone(), self.user_id) else {
            return;
        };
        let fence = SessionRequestFence {
            auth_generation: self.auth_generation,
            request_generation: 0,
            token,
            user_id: my_id,
        };
        self.talk_mode_overlay = None;
        self.active_talk_session_is_ai = false;
        self.media_viewer_overlay = None;
        self.iv_viewer_overlay = None;
        self.main_menu_drawer.update(cx, |d, _| d.close());
        let lang = self.active_lang_id();
        let pal = self.palette;
        let overlay = cx.new(|_| {
            ComputerWorkbench::new_live(
                tr_str(lang, "远程桌面主机", "Remote Desktop").into(),
                pal,
                lang,
            )
        });
        cx.observe(
            &overlay,
            |shell: &mut AppShell, ov: gpui::Entity<ComputerWorkbench>, cx: &mut Context<AppShell>| {
                let (close, act) = ov.update(cx, |w, _| (w.should_close, w.take_action()));
                if let Some(cmd) = act {
                    shell.window_runtime.enqueue_toast(cmd, 2400);
                }
                if close {
                    shell.hide_remote_computer(cx);
                }
                cx.notify();
            },
        )
        .detach();
        self.computer_overlay = Some(overlay);
        cx.notify();
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let payload = serde_json::json!({ "requested_by": my_id });
            let res = client.acquire_computer_lease(&payload).await;
            let _ = weak.update(app, |shell, cx| {
                if !fence.matches(
                    shell.is_authenticated,
                    shell.auth_generation,
                    0,
                    shell.session_token.as_deref(),
                    shell.user_id,
                ) {
                    return;
                }
                if let Some(ov) = shell.computer_overlay.clone() {
                    match res {
                        Ok(v) => {
                            let spec = HostDeviceSpec {
                                name: v
                                    .get("device_name")
                                    .and_then(|x| x.as_str())
                                    .unwrap_or("Cloud Host")
                                    .to_string(),
                                ip_address: v
                                    .get("host")
                                    .and_then(|x| x.as_str())
                                    .unwrap_or("")
                                    .to_string(),
                                os_name: v
                                    .get("os")
                                    .and_then(|x| x.as_str())
                                    .unwrap_or("Linux / macOS")
                                    .to_string(),
                                resolution: v
                                    .get("resolution")
                                    .and_then(|x| x.as_str())
                                    .unwrap_or("1920x1080")
                                    .to_string(),
                                latency_ms: v.get("latency_ms").and_then(|x| x.as_u64()).unwrap_or(0)
                                    as u32,
                                fps: v.get("fps").and_then(|x| x.as_u64()).unwrap_or(60) as u32,
                                is_tunnel_encrypted: true,
                            };
                            let frame = v
                                .get("stream_url")
                                .and_then(|x| x.as_str())
                                .map(str::to_string);
                            ov.update(cx, |w, _| w.sync_lease(spec, frame, true));
                        }
                        Err(err) => shell.window_runtime.enqueue_toast(
                            format!(
                                "{}: {}",
                                tr_str(
                                    shell.active_lang_id(),
                                    "无法获取远程会话",
                                    "Remote lease failed"
                                ),
                                err
                            ),
                            3200,
                        ),
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub fn toggle_voice_recording(&mut self, start: bool, cx: &mut Context<Self>) {
        if start {
            if self.talk_mode_overlay.is_some() && self.active_talk_session_is_ai {
                self.talk_coordinator.handle_barge_in();
                self.audio_player.stop();
                if let Some(ov) = self.talk_mode_overlay.clone() {
                    ov.update(cx, |o, _| o.set_state(TalkModeState::Listening));
                }
            }
            if let Err(err) = self.voice_recorder.start() {
                self.window_runtime.enqueue_toast(
                    format!(
                        "{}: {}",
                        tr_str(self.active_lang_id(), "麦克风不可用", "Microphone error"),
                        err
                    ),
                    3000,
                );
            }
            cx.notify();
            return;
        }
        let Ok(note) = self.voice_recorder.stop() else {
            return;
        };
        if self.talk_mode_overlay.is_some() && self.active_talk_session_is_ai {
            let composer_draft = self.composer.read(cx).text().trim().to_string();
            let transcript = if !composer_draft.is_empty() {
                self.composer.update(cx, |c, _| c.set_text(String::new()));
                composer_draft
            } else {
                format!(
                    "[Voice {}s]",
                    ((note.duration_ms.max(1000)) / 1000) as u32
                )
            };
            self.send_ai_talk_voice_turn(transcript, true, cx);
            return;
        }
        let Some(chat_id) = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        ) else {
            return;
        };
        let (Some(token), Some(my_id)) = (self.session_token.clone(), self.user_id) else {
            return;
        };
        let fence = SessionRequestFence {
            auth_generation: self.auth_generation,
            request_generation: 0,
            token,
            user_id: my_id,
        };
        let dur_secs = ((note.duration_ms.max(1000)) / 1000) as u32;
        let msg_id = self.next_message_id;
        self.next_message_id += 1;
        let sender = self.username.clone().unwrap_or_else(|| "Me".into());
        self.stage.update(cx, |st, _| {
            st.add_message(MessageItemModel {
                id: msg_id,
                sender_id: my_id,
                avatar_initials: derive_initials(&sender),
                avatar_color: peer_avatar_color(my_id),
                sender_name: sender,
                direction: MessageDirection::Outgoing,
                timestamp: String::new(),
                delivery_status: DeliveryStatus::Sending,
                reply_to: None,
                reactions: Vec::new(),
                content: MessageContent::Voice {
                    duration_secs: dur_secs,
                    waveform_5bit: note.waveform_5bit.clone(),
                    playback_position_secs: 0.0,
                    is_playing: false,
                    playback_speed: 1.0,
                },
                bot_keyboard: None,
                is_pinned: false,
            })
        });
        let client = self.client.clone();
        let wav = note.wav_bytes;
        let wave = note.waveform_5bit;
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            if let Ok(up) = client.upload_media("voice.wav", "audio/wav", wav).await {
                let req = RichSendMessageRequest {
                    chat_id: chat_id as i64,
                    text: Some("Voice message".into()),
                    content: Some(ProtoMessageContent::Voice(VoiceItem {
                        file_id: up.file_id,
                        duration_seconds: dur_secs,
                        waveform: wave,
                        mime_type: "audio/wav".into(),
                        size_bytes: up.size_bytes,
                    })),
                    entities: Vec::new(),
                    reply_to: None,
                    quote_text: None,
                    topic_id: None,
                    client_nonce: Some(client.next_client_nonce()),
                    forward_from_chat_id: None,
                    forward_from_msg_id: None,
                    silent: false,
                    scheduled_at_ms: None,
                    inline_keyboard: None,
                };
                let _ = client.send_message_durable(req).await;
                let _ = weak.update(app, |shell, cx| {
                    if fence.matches(
                        shell.is_authenticated,
                        shell.auth_generation,
                        0,
                        shell.session_token.as_deref(),
                        shell.user_id,
                    ) {
                        shell.load_historical_messages(chat_id, cx);
                    }
                });
            }
        })
        .detach();
    }
}
