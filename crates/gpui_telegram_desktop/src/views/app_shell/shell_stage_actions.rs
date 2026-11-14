use gpui::{AppContext, AsyncApp, ClipboardItem, Context, Entity, WeakEntity};
use gpui_telegram_client::net::hermes_client::{
    HermesApprovalDecisionRequest, HermesCommandRequest, HermesRichChatDto,
};

use crate::i18n::tr_str;
use crate::theme::TdesktopThemeMode;
use crate::views::app_shell::shell_actions::{
    build_chat_profile_facts, map_channel_stats_to_chart, map_chat_members_to_inspector,
    map_stage_media_to_gallery, resolve_active_target_chat_id,
};
use crate::views::app_shell::shell_theme::{
    compute_shell_palette, propagate_theme_and_locale, tdesktop_mode_to_app_theme, ShellThemeTargets,
};
use crate::views::app_shell::shell_truth::{
    derive_initials, select_authoritative_dialog, SessionRequestFence,
};
use crate::views::boxes::modal_types::ModalDialogKind;
use crate::views::iv::IvViewer;
use crate::views::media_viewer::MediaViewerItem;
use crate::views::nav_rail::NavDestination;
use crate::views::shell::AppShell;
use crate::views::MediaViewer;

fn normalized_username(raw: Option<&str>) -> String {
    let trimmed = raw.unwrap_or("").trim();
    if trimmed.is_empty() {
        String::new()
    } else if trimmed.starts_with('@') {
        trimmed.to_string()
    } else {
        format!("@{trimmed}")
    }
}

fn enrich_facts_with_rich_chat(
    mut facts: Vec<(String, String)>,
    rc: &HermesRichChatDto,
    lang: &str,
) -> Vec<(String, String)> {
    if rc.is_bot {
        facts.push((
            tr_str(lang, "智能体引擎", "Agent Engine").to_string(),
            "Hermes Rust Fusion · Talk Mode & Tools".to_string(),
        ));
    } else if rc.members_count > 0
        && !facts
            .iter()
            .any(|(k, _)| k == "Members" || k == "成员")
    {
        facts.push((
            tr_str(lang, "成员", "Members").to_string(),
            rc.members_count.to_string(),
        ));
    }
    facts
}

impl AppShell {
    pub fn sync_theme_and_locale_to_views(&mut self, cx: &mut Context<Self>) {
        let lang = self.active_lang_id();
        let (mode, pal) = compute_shell_palette(
            &self.preferences,
            Some(self.theme_mode),
            self.accent_color,
            self.bubble_radius_px,
            self.wallpaper_id.as_deref(),
        );
        self.theme_mode = mode;
        self.palette = pal;
        let targets = ShellThemeTargets {
            nav_rail: &self.nav_rail,
            main_menu_drawer: &self.main_menu_drawer,
            sidebar: &self.sidebar,
            stage: &self.stage,
            composer: &self.composer,
            inspector: &self.inspector,
            modal_container: &self.modal_container,
            tasks_workspace: &self.tasks_workspace,
            talk_mode_overlay: self.talk_mode_overlay.as_ref(),
            computer_overlay: self.computer_overlay.as_ref(),
            media_viewer_overlay: self.media_viewer_overlay.as_ref(),
            iv_viewer_overlay: self.iv_viewer_overlay.as_ref(),
            settings_main: &self.settings_main,
            settings_information: &self.settings_information,
            settings_notifications: &self.settings_notifications,
            settings_notif_reactions: &self.settings_notif_reactions,
            settings_notif_type: &self.settings_notif_type,
            settings_privacy: &self.settings_privacy,
            settings_blocked_peers: &self.settings_blocked_peers,
            settings_active_sessions: &self.settings_active_sessions,
            settings_local_passcode: &self.settings_local_passcode,
            settings_passkeys: &self.settings_passkeys,
            settings_global_ttl: &self.settings_global_ttl,
            settings_websites: &self.settings_websites,
            settings_chat: &self.settings_chat,
            settings_folders: &self.settings_folders,
            settings_advanced: &self.settings_advanced,
            settings_local_storage: &self.settings_local_storage,
            settings_power_saving: &self.settings_power_saving,
            settings_calls: &self.settings_calls,
            settings_credits: &self.settings_credits,
            settings_premium: &self.settings_premium,
            settings_shortcuts: &self.settings_shortcuts,
            settings_business: &self.settings_business,
            settings_search: &self.settings_search,
        };
        propagate_theme_and_locale(targets, &pal, lang, cx);
    }

    pub fn apply_theme_selection(
        &mut self,
        mode: TdesktopThemeMode,
        accent: Option<u32>,
        wallpaper: Option<String>,
        cx: &mut Context<Self>,
    ) {
        self.theme_mode = mode;
        if let Some(a) = accent {
            self.accent_color = Some(a);
        }
        if let Some(w) = wallpaper {
            self.wallpaper_id = Some(w);
        }
        self.update_preferences(cx, |p| {
            let mapped = tdesktop_mode_to_app_theme(mode);
            p.theme_mode = mapped;
            p.appearance.theme_mode = mapped;
            p.appearance.chat_preset = mode.as_name().to_string();
        });
    }

    pub fn sync_selected_chat_inspector_and_stage(
        &mut self,
        chat_id: u64,
        cx: &mut Context<Self>,
    ) {
        let mut selected = false;
        let mut max_msg_id = 0i64;
        let (mut viewer_items, mut media, mut files, mut links, mut voices) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
        let (Some(token), Some(uid)) = (self.session_token.clone(), self.user_id) else {
            return;
        };
        let fence = SessionRequestFence {
            auth_generation: self.auth_generation,
            request_generation: 0,
            token,
            user_id: uid,
        };
        self.sidebar.update(cx, |sb, sb_cx| {
            self.stage.update(sb_cx, |st, st_cx| {
                self.inspector.update(st_cx, |insp, _| {
                    selected = select_authoritative_dialog(sb, st, insp, chat_id);
                    if selected {
                        max_msg_id = st.messages.iter().map(|m| m.id as i64).max().unwrap_or(0);
                        (viewer_items, media, files, links, voices) =
                            map_stage_media_to_gallery(&st.messages);
                        let _ = viewer_items;
                        insp.sync_shared_content(media, files, links, voices, Vec::new());
                        let fixed_init = derive_initials(&insp.profile.name);
                        if (insp.profile.avatar_initials.is_empty()
                            || insp.profile.avatar_initials == "?")
                            && !fixed_init.is_empty()
                        {
                            insp.profile.avatar_initials = fixed_init;
                        }
                    }
                });
            });
        });
        if !selected {
            return;
        }
        self.load_historical_messages(chat_id, cx);
        let lang = self.active_lang_id().to_string();
        let contacts = self.contacts_list.clone();
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let details = client.get_chat(chat_id as i64).await.ok();
            let rich_chat = client.hermes_get_chat_rich(chat_id as i64).await.ok();
            let members = client.get_chat_members(chat_id as i64).await.ok();
            let invites = client.list_invites(chat_id as i64).await.ok();
            let pins = client.list_pinned_messages(chat_id as i64).await.ok();
            let draft = client.get_draft(chat_id as i64).await.ok().flatten();
            let stats = client.get_channel_statistics(chat_id as i64).await.ok();
            if max_msg_id > 0 {
                let _ = client.mark_chat_read(chat_id as i64, max_msg_id, None).await;
            }
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
                if let Some(ref rc) = rich_chat {
                    shell.rich_chats_by_id.insert(chat_id, rc.clone());
                    if let Some(pid) = rc.peer_user_id {
                        if pid > 0 {
                            shell.chat_peer_user_ids.insert(chat_id, pid);
                        }
                    }
                    if rc.is_bot {
                        shell.sidebar.update(cx, |sb, _| {
                            sb.bot_ids.insert(chat_id);
                        });
                    }
                }
                shell.inspector.update(cx, |insp, _| {
                    let mut username = String::new();
                    let mut phone = String::new();
                    let mut bio = String::new();
                    let mut status = insp.profile.status.clone();
                    let mut avatar_path = details.as_ref().and_then(|d| d.photo_file_id.clone());
                    let mut facts = details
                        .as_ref()
                        .map(|d| build_chat_profile_facts(d, &lang))
                        .unwrap_or_default();

                    if let Some(ref rc) = rich_chat {
                        let resolved_title = rc
                            .peer_display_name
                            .as_deref()
                            .filter(|s| !s.trim().is_empty())
                            .or_else(|| {
                                let t = rc.title.trim();
                                if t.is_empty() { None } else { Some(t) }
                            });
                        if let Some(title) = resolved_title {
                            insp.profile.name = title.to_string();
                            let init = derive_initials(title);
                            if !init.is_empty() {
                                insp.profile.avatar_initials = init;
                            }
                        }
                        username = normalized_username(
                            rc.peer_username.as_deref().or(rc.username.as_deref()),
                        );
                        phone = rc.peer_phone.clone().unwrap_or_default();
                        bio = rc
                            .peer_bio
                            .as_deref()
                            .or(rc.about.as_deref())
                            .unwrap_or("")
                            .trim()
                            .to_string();
                        if rc.is_bot {
                            status = tr_str(
                                &lang,
                                "AI 机器人 · Hermes 智能体",
                                "AI Bot · Hermes Agent",
                            )
                            .to_string();
                        }
                        facts = enrich_facts_with_rich_chat(facts, rc, &lang);
                    }
                    if username.is_empty() {
                        if let Some(c) = rich_chat
                            .as_ref()
                            .and_then(|rc| rc.peer_user_id)
                            .and_then(|pid| contacts.iter().find(|c| c.user_id == pid))
                            .or_else(|| {
                                contacts
                                    .iter()
                                    .find(|c| c.chat_id == Some(chat_id as i64) || c.user_id == chat_id)
                            })
                        {
                            username = normalized_username(c.username.as_deref());
                            if phone.is_empty() {
                                phone = c.phone.clone().unwrap_or_default();
                            }
                        }
                    }
                    if bio.is_empty() {
                        if let Some(ref d) = details {
                            if let Some(ref about) = d.about {
                                if !about.trim().is_empty() {
                                    bio = about.trim().to_string();
                                }
                            }
                        }
                    }
                    if !bio.is_empty() {
                        insp.profile.bio = bio;
                    }
                    if avatar_path.is_none() {
                        avatar_path = insp.profile.avatar_path.clone();
                    }
                    insp.sync_profile_details(username, phone, status, avatar_path, facts);
                    if let Some(ref mems) = members {
                        let inv = invites
                            .as_ref()
                            .and_then(|v| v.first())
                            .map(|i| i.link.clone())
                            .unwrap_or_default();
                        let is_grp = rich_chat
                            .as_ref()
                            .map(|rc| rc.is_group || rc.is_channel)
                            .unwrap_or_else(|| {
                                details
                                    .as_ref()
                                    .map(|d| d.chat_type != 0)
                                    .unwrap_or(!mems.is_empty())
                            });
                        insp.sync_members(
                            map_chat_members_to_inspector(mems, &contacts),
                            inv,
                            is_grp,
                        );
                    }
                    if let Some(ref st) = stats {
                        let (lvl, cur, tgt, bars) = map_channel_stats_to_chart(st);
                        insp.sync_channel_statistics(lvl, cur, tgt, bars);
                    }
                });
                if let Some(p) = pins.and_then(|v| v.into_iter().next()) {
                    shell.stage.update(cx, |st, _| {
                        st.pinned_message = Some(p.preview_text);
                        st.sync_pinned_tracker();
                    });
                }
                if let Some(dr) = draft {
                    shell
                        .sidebar
                        .update(cx, |sb, _| sb.set_draft(chat_id, dr.text.clone()));
                    if !dr.text.is_empty() && shell.composer.read(cx).text().is_empty() {
                        shell.composer.update(cx, |c, _| c.set_text(dr.text));
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub fn execute_hermes_inline_callback(
        &mut self,
        raw_action: String,
        cx: &mut Context<Self>,
    ) {
        let trimmed = raw_action.trim().to_string();
        let lower = trimmed.to_ascii_lowercase();
        let lang = self.active_lang_id();

        if lower == "action_talk_mode"
            || lower == "talk_mode"
            || lower == "talk:start"
            || lower == "/talk"
            || lower == "launch talk mode"
            || lower.contains("talk_mode")
            || lower.starts_with("talk:")
        {
            self.nav_rail.update(cx, |r, _| {
                r.set_active_destination(NavDestination::TalkMode);
            });
            self.start_ai_talk_mode_session(false, cx);
            return;
        }

        if lower == "action_remote_computer"
            || lower == "remote_computer"
            || lower == "computer:open"
            || lower == "/computer"
            || lower == "remote computer"
            || lower.contains("remote_computer")
            || lower.starts_with("computer:")
        {
            self.nav_rail.update(cx, |r, _| {
                r.set_active_destination(NavDestination::RemoteComputer);
            });
            self.open_computer_workbench_live(cx);
            return;
        }

        if lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("url:") {
            let url = trimmed.trim_start_matches("url:").to_string();
            cx.write_to_clipboard(ClipboardItem::new_string(url.clone()));
            self.window_runtime.enqueue_toast(
                format!("{}: {}", tr_str(lang, "已复制链接", "Copied URL"), url),
                2600,
            );
            return;
        }

        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        let bot_id = active_chat.and_then(|cid| {
            self.rich_chats_by_id
                .get(&cid)
                .and_then(|rc| rc.peer_user_id)
                .or_else(|| self.chat_peer_user_ids.get(&cid).copied())
        });
        let session_id = self.active_talk_session_id.clone();

        if lower.starts_with("approval:")
            || lower.starts_with("/approve")
            || lower.starts_with("/deny")
            || lower.starts_with("hermes:")
        {
            let client = self.client.clone();
            let (Some(token), Some(uid)) = (self.session_token.clone(), self.user_id) else {
                return;
            };
            let fence = SessionRequestFence {
                auth_generation: self.auth_generation,
                request_generation: 0,
                token,
                user_id: uid,
            };
            cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                let toast = if lower.starts_with("approval:") {
                    let parts: Vec<&str> = trimmed.split(':').collect();
                    let approval_id = parts.get(1).copied().unwrap_or("");
                    let dec = parts.get(2).copied().unwrap_or("approve_once");
                    match client
                        .hermes_decide_approval(
                            approval_id,
                            &HermesApprovalDecisionRequest {
                                decision: dec.to_string(),
                                reason: Some("Inline callback button".to_string()),
                            },
                        )
                        .await
                    {
                        Ok(a) => format!("Approval {}: {}", a.approval_id, a.status),
                        Err(e) => format!("Approval error: {}", e),
                    }
                } else {
                    let cmd_text = trimmed
                        .strip_prefix("hermes:")
                        .unwrap_or(&trimmed)
                        .trim()
                        .to_string();
                    match client
                        .hermes_run_command(&HermesCommandRequest {
                            command: cmd_text,
                            session_id,
                            chat_id: active_chat.map(|id| id as i64),
                            bot_id,
                        })
                        .await
                    {
                        Ok(resp) => resp
                            .reply_text
                            .filter(|s| !s.trim().is_empty())
                            .unwrap_or_else(|| {
                                if resp.output.trim().is_empty() {
                                    format!("Executed {}", resp.command)
                                } else {
                                    resp.output
                                }
                            }),
                        Err(e) => format!("Hermes command failed: {}", e),
                    }
                };
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
                    let summary: String = toast.lines().next().unwrap_or(&toast).chars().take(120).collect();
                    shell.window_runtime.enqueue_toast(summary, 3000);
                    shell.refresh_tasks_and_mcp(cx);
                    if let Some(cid) = active_chat {
                        shell.load_historical_messages(cid, cx);
                    }
                    cx.notify();
                });
            })
            .detach();
            return;
        }

        self.append_outgoing_text(trimmed, cx);
    }

    pub fn handle_stage_pending_actions(&mut self, cx: &mut Context<Self>) {
        let lang = self.active_lang_id();
        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        let (
            call_v,
            call_vid,
            media_single,
            media_gal,
            iv_url,
            edit_req,
            fwd_ids,
            del_ids,
            pin_req,
            tr_id,
            poll_vote,
            react_req,
            voice_play,
            dl_req,
            drops,
            open_prof,
            clear_hist,
            export_chat,
            toggle_mute,
            bot_act,
        ) = self.stage.update(cx, |st, _| {
            (
                if st.pending_call_request {
                    st.pending_call_request = false;
                    true
                } else {
                    false
                },
                if st.pending_video_call {
                    st.pending_video_call = false;
                    true
                } else {
                    false
                },
                st.pending_media_url.take(),
                st.pending_media_viewer.take(),
                st.pending_iv_url.take(),
                st.pending_edit_message.take(),
                st.pending_forward_message_ids.take(),
                st.pending_delete_message_ids.take(),
                st.pending_pin_toggle.take(),
                st.pending_translate_message_id.take(),
                st.pending_poll_vote.take(),
                st.pending_reaction_toggle.take(),
                st.pending_voice_playback.take(),
                st.pending_download_url.take(),
                st.pending_dropped_paths.take(),
                st.pending_open_profile_peer_id.take(),
                if st.pending_clear_history {
                    st.pending_clear_history = false;
                    true
                } else {
                    false
                },
                if st.pending_export_chat {
                    st.pending_export_chat = false;
                    true
                } else {
                    false
                },
                if st.pending_toggle_mute {
                    st.pending_toggle_mute = false;
                    true
                } else {
                    false
                },
                st.pending_bot_action.take(),
            )
        });
        if call_v || call_vid {
            if self.is_active_chat_bot(cx) {
                self.start_ai_talk_mode_session(call_vid, cx);
            } else if let Some(pid) = self.current_peer_user_id(cx) {
                let name = self.stage.read(cx).chat_title.clone();
                self.start_webrtc_call(pid, name, call_vid, cx);
            } else {
                self.window_runtime.enqueue_toast(
                    tr_str(
                        lang,
                        "当前群组或会话未绑定直接通话用户，请进入私聊或机器人对话。",
                        "No direct user peer resolved for WebRTC call in this chat.",
                    ),
                    3000,
                );
            }
        }
        if let Some((items, idx)) = media_gal {
            let mv_items: Vec<MediaViewerItem> = items
                .into_iter()
                .enumerate()
                .map(|(i, (url, is_video, caption))| MediaViewerItem {
                    id: i as u64 + 1,
                    url,
                    is_video,
                    caption,
                    sender_name: String::new(),
                    date: String::new(),
                    duration_secs: None,
                })
                .collect();
            let pal = self.palette;
            let overlay = cx.new(|_| {
                let mut m = MediaViewer::from_gallery(mv_items, idx);
                m.apply_theme(pal, lang);
                m
            });
            cx.observe(
                &overlay,
                |shell: &mut AppShell, ov: gpui::Entity<MediaViewer>, cx: &mut Context<AppShell>| {
                    if ov.read(cx).should_close {
                        shell.hide_media_viewer();
                        cx.notify();
                    }
                },
            )
            .detach();
            self.media_viewer_overlay = Some(overlay);
        } else if let Some((url, is_vid)) = media_single {
            let pal = self.palette;
            let overlay = cx.new(|_| {
                let mut m = MediaViewer::new(url, is_vid);
                m.apply_theme(pal, lang);
                m
            });
            cx.observe(
                &overlay,
                |shell: &mut AppShell, ov: gpui::Entity<MediaViewer>, cx: &mut Context<AppShell>| {
                    if ov.read(cx).should_close {
                        shell.hide_media_viewer();
                        cx.notify();
                    }
                },
            )
            .detach();
            self.media_viewer_overlay = Some(overlay);
        }
        if let Some((url, title)) = iv_url {
            let pal = self.palette;
            let overlay = cx.new(|_| {
                let mut iv = IvViewer::new(url, title);
                iv.apply_theme(pal, lang);
                iv
            });
            cx.observe(
                &overlay,
                |shell: &mut AppShell, ov: gpui::Entity<IvViewer>, cx: &mut Context<AppShell>| {
                    let (close, share) = ov.update(cx, |v, _| (v.should_close, v.pending_share_url.take()));
                    if let Some(u) = share {
                        shell.append_outgoing_text(u, cx);
                    }
                    if close {
                        shell.iv_viewer_overlay = None;
                        cx.notify();
                    }
                },
            )
            .detach();
            self.iv_viewer_overlay = Some(overlay);
        }
        if let Some((mid, existing)) = edit_req {
            self.composer.update(cx, |c, _| c.begin_edit(mid, existing));
        }
        if let Some(ids) = fwd_ids {
            let preview = format!("{} {}", ids.len(), tr_str(lang, "条消息", "messages"));
            self.open_modal_dialog(
                tr_str(lang, "转发消息", "Forward Messages"),
                ModalDialogKind::ForwardMessages {
                    message_ids: ids,
                    preview_snippet: preview,
                },
                420.0,
                460.0,
                cx,
            );
        }
        if let Some(ids) = del_ids {
            let peer = self.stage.read(cx).chat_title.clone();
            self.open_modal_dialog(
                tr_str(lang, "删除消息", "Delete Messages"),
                ModalDialogKind::DeleteMessages {
                    message_ids: ids,
                    peer_name: peer,
                },
                380.0,
                260.0,
                cx,
            );
        }
        if let Some((mid, pin)) = pin_req {
            if pin {
                let peer = self.stage.read(cx).chat_title.clone();
                self.open_modal_dialog(
                    tr_str(lang, "置顶消息", "Pin Message"),
                    ModalDialogKind::PinMessage {
                        message_id: mid,
                        peer_name: peer,
                        is_group: false,
                    },
                    380.0,
                    240.0,
                    cx,
                );
            } else if let Some(cid) = active_chat {
                self.stage.update(cx, |st, _| st.toggle_pin_message(mid));
                let c = self.client.clone();
                cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                    let _ = c.unpin_message(cid as i64, mid as i64).await;
                })
                .detach();
            }
        }
        if let Some(mid) = tr_id {
            if let Some(text) = self.stage.read(cx).copy_message_text(mid) {
                self.open_modal_dialog(
                    tr_str(lang, "翻译消息", "Translate Message"),
                    ModalDialogKind::TranslateMessage {
                        original_text: text,
                    },
                    420.0,
                    320.0,
                    cx,
                );
            }
        }
        if let Some((mid, opt)) = poll_vote {
            self.stage.update(cx, |st, _| st.vote_poll(mid, opt));
            let c = self.client.clone();
            cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                let _ = c.vote_poll(mid, vec![opt as u8]).await;
            })
            .detach();
        }
        if let Some((mid, emoji)) = react_req {
            self.stage
                .update(cx, |st, _| st.toggle_reaction(mid, &emoji));
            if let Some(cid) = active_chat {
                let c = self.client.clone();
                cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                    let _ = c.react_to_message(cid as i64, mid as i64, emoji, None).await;
                })
                .detach();
            }
        }
        if let Some((mid, play, _pos, spd)) = voice_play {
            self.audio_player.set_speed(spd as f64);
            if !play {
                let _ = self.audio_player.pause();
            } else {
                let key = format!("voice_{mid}");
                let c = self.client.clone();
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    if let Ok((_, bytes, _)) = c.download_media_range(&key, 0, None).await {
                        let _ = weak.update(app, |s, _| {
                            let _ = s.audio_player.play_wav_bytes(key, &bytes);
                        });
                    }
                })
                .detach();
            }
        }
        if let Some((url, name)) = dl_req {
            let c = self.client.clone();
            cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                let _ = c.download_media_range(&url, 0, None).await;
                let _ = weak.update(app, |s, _| {
                    s.window_runtime
                        .enqueue_toast(format!("Saved: {name}"), 2500)
                });
            })
            .detach();
        }
        if let Some(paths) = drops {
            self.upload_and_send_paths(paths, cx);
        }
        if open_prof.is_some() {
            self.inspector.update(cx, |i, _| i.open());
        }
        if clear_hist {
            if let Some(cid) = active_chat {
                self.sidebar.update(cx, |sb, _| sb.clear_history(cid));
                self.stage.update(cx, |st, _| st.messages.clear());
            }
        }
        if export_chat {
            let title = self.stage.read(cx).chat_title.clone();
            self.open_modal_dialog(
                tr_str(lang, "导出聊天", "Export Chat"),
                ModalDialogKind::ExportData { chat_title: title },
                400.0,
                320.0,
                cx,
            );
        }
        if toggle_mute {
            if let Some(cid) = active_chat {
                self.sidebar.update(cx, |sb, _| sb.toggle_mute_dialog(cid));
                let muted = self
                    .sidebar
                    .read(cx)
                    .dialogs
                    .iter()
                    .find(|d| d.id == cid)
                    .map(|d| d.is_muted)
                    .unwrap_or(true);
                let c = self.client.clone();
                cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                    let _ = c.set_chat_muted(cid as i64, muted, None).await;
                })
                .detach();
            }
        }
        if let Some(act) = bot_act {
            self.execute_hermes_inline_callback(act, cx);
        }
    }
}
