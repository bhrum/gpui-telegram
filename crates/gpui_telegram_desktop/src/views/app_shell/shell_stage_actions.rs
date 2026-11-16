//! Stage action dispatch, chat inspector synchronization, bot peer detection, and theme propagation.

use std::mem;

use gpui::{AppContext, AsyncApp, ClipboardItem, Context, WeakEntity};
use gpui_telegram_client::net::hermes_client::{
    HermesApprovalDecisionRequest, HermesCommandRequest, HermesRichChatDto,
};
use gpui_telegram_proto::miniapp_contracts::{parse_miniapp_uri, MiniAppLaunchMode};

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
    is_fabushi_bot: bool,
    lang: &str,
) -> Vec<(String, String)> {
    if rc.is_bot {
        if is_fabushi_bot {
            facts.push((
                tr_str(lang, "智能体引擎", "Agent Engine").to_string(),
                "Fabushi AI · Talk Mode & Tools".to_string(),
            ));
        }
    } else if rc.members_count > 0 && !facts.iter().any(|(k, _)| k == "Members" || k == "成员") {
        facts.push((tr_str(lang, "成员", "Members").to_string(), rc.members_count.to_string()));
    }
    facts
}

fn session_fence(shell: &AppShell) -> Option<SessionRequestFence> {
    let (Some(token), Some(uid)) =
        (shell.session_sync.session_token.clone(), shell.session_sync.user_id)
    else {
        return None;
    };
    Some(SessionRequestFence {
        auth_generation: shell.session_sync.auth_generation,
        request_generation: 0,
        token,
        user_id: uid,
    })
}

fn fence_matches(fence: &SessionRequestFence, shell: &AppShell) -> bool {
    fence.matches(
        shell.is_authenticated,
        shell.session_sync.auth_generation,
        0,
        shell.session_sync.session_token.as_deref(),
        shell.session_sync.user_id,
    )
}

fn attach_media_viewer_overlay(
    shell: &mut AppShell,
    mut viewer: MediaViewer,
    lang: &'static str,
    cx: &mut Context<AppShell>,
) {
    viewer.apply_theme(shell.theme_state.palette, lang);
    let overlay = cx.new(|_| viewer);
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
    shell.overlays.media_viewer_overlay = Some(overlay);
}

fn toggle_chat_dialog_flag(
    shell: &mut AppShell,
    cid: u64,
    is_pin: bool,
    lang: &'static str,
    cx: &mut Context<AppShell>,
) {
    shell.sidebar.update(cx, |sb, sb_cx| {
        if is_pin { sb.toggle_pin_dialog(cid); } else { sb.toggle_mute_dialog(cid); }
        sb_cx.notify();
    });
    let flag = shell
        .sidebar
        .read(cx)
        .dialogs
        .iter()
        .find(|d| d.id == cid)
        .map(|d| if is_pin { d.is_pinned } else { d.is_muted })
        .unwrap_or(true);
    shell.stage.update(cx, |st, _| {
        if is_pin { st.is_pinned_chat = flag; } else { st.is_muted_chat = flag; }
    });
    if let Some(rc) = shell.session_sync.rich_chats_by_id.get_mut(&cid) {
        if is_pin { rc.is_pinned = flag; } else { rc.is_muted = flag; }
    }
    let toast_msg = match (is_pin, flag) {
        (true, true) => tr_str(lang, "已置顶该对话", "Chat pinned"),
        (true, false) => tr_str(lang, "已取消置顶", "Chat unpinned"),
        (false, true) => tr_str(lang, "已开启消息免打扰", "Notifications muted"),
        (false, false) => tr_str(lang, "已恢复消息通知", "Notifications unmuted"),
    };
    shell.window_runtime.enqueue_toast(toast_msg, 2000);
    let c = shell.client.clone();
    cx.spawn(async move |_: WeakEntity<AppShell>, _: &mut AsyncApp| {
        if is_pin {
            let _ = c.set_chat_pinned(cid as i64, flag).await;
        } else {
            let _ = c.set_chat_muted(cid as i64, flag, None).await;
        }
    })
    .detach();
}

impl AppShell {
    /// Computes the active palette and propagates theme and locale to all mounted views and overlays.
    pub fn sync_theme_and_locale_to_views(&mut self, cx: &mut Context<Self>) {
        let lang = self.active_lang_id();
        let (mode, pal) = compute_shell_palette(
            &self.preferences,
            Some(self.theme_state.theme_mode),
            self.theme_state.accent_color,
            self.theme_state.bubble_radius_px,
            self.theme_state.wallpaper_id.as_deref(),
        );
        self.theme_state.theme_mode = mode;
        self.theme_state.palette = pal;
        let targets = ShellThemeTargets {
            nav_rail: &self.nav_rail,
            main_menu_drawer: &self.main_menu_drawer,
            sidebar: &self.sidebar,
            stage: &self.stage,
            composer: &self.composer,
            inspector: &self.inspector,
            modal_container: &self.modal_container,
            tasks_workspace: &self.tasks_workspace,
            talk_mode_overlay: self.rtc_talk.talk_mode_overlay.as_ref(),
            computer_overlay: self.overlays.computer_overlay.as_ref(),
            media_viewer_overlay: self.overlays.media_viewer_overlay.as_ref(),
            iv_viewer_overlay: self.overlays.iv_viewer_overlay.as_ref(),
            auth_wizard: &self.auth_wizard,
            settings_router: &self.settings_router,
            miniapp_overlay: self.miniapps.miniapp_overlay.as_ref(),
            create_bot_box: self.miniapps.create_bot_box.as_ref(),
        };
        propagate_theme_and_locale(targets, &pal, lang, cx);
    }

    /// Updates active theme mode, accent color, and wallpaper selection and persists preferences.
    pub fn apply_theme_selection(
        &mut self,
        mode: TdesktopThemeMode,
        accent: Option<u32>,
        wallpaper: Option<String>,
        cx: &mut Context<Self>,
    ) {
        self.theme_state.theme_mode = mode;
        if let Some(a) = accent {
            self.theme_state.accent_color = Some(a);
        }
        if let Some(w) = wallpaper {
            self.theme_state.wallpaper_id = Some(w);
        }
        self.update_preferences(cx, |p| {
            let mapped = tdesktop_mode_to_app_theme(mode);
            p.theme_mode = mapped;
            p.appearance.theme_mode = mapped;
            p.appearance.chat_preset = mode.as_name().to_string();
        });
    }

    /// Synchronizes sidebar/stage/inspector selection and asynchronously fetches authoritative chat details.
    pub fn sync_selected_chat_inspector_and_stage(&mut self, chat_id: u64, cx: &mut Context<Self>) {
        let mut selected = false;
        let mut max_msg_id = 0i64;
        let Some(fence) = session_fence(self) else {
            return;
        };
        let rich_meta_opt = self.session_sync.rich_chats_by_id.get(&chat_id).cloned();
        self.sidebar.update(cx, |sb, sb_cx| {
            let dlg_flags = sb.dialogs.iter().find(|d| d.id == chat_id).map(|d| (d.is_muted, d.is_pinned));
            self.stage.update(sb_cx, |st, st_cx| {
                self.inspector.update(st_cx, |insp, _| {
                    selected = select_authoritative_dialog(sb, st, insp, chat_id);
                    if selected {
                        if let Some((muted, pinned)) = dlg_flags {
                            st.is_muted_chat = muted;
                            st.is_pinned_chat = pinned;
                        }
                        if let Some(ref rc) = rich_meta_opt {
                            st.is_muted_chat = rc.is_muted || st.is_muted_chat;
                            st.is_pinned_chat = rc.is_pinned || st.is_pinned_chat;
                            st.is_peer_blocked = rc.is_blocked;
                            st.message_ttl_seconds = rc.message_ttl_seconds;
                        }
                        max_msg_id = st.messages.iter().map(|m| m.id as i64).max().unwrap_or(0);
                        let (_, media, files, links, voices) = map_stage_media_to_gallery(&st.messages);
                        insp.sync_shared_content(media, files, links, voices, Vec::new());
                        let fixed_init = derive_initials(&insp.profile.name);
                        if (insp.profile.avatar_initials.is_empty() || insp.profile.avatar_initials == "?")
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
        self.miniapps.pending_prestart_bot = None;
        let sb_is_bot = self.sidebar.read(cx).bot_ids.contains(&chat_id);
        let fallback_bot_target = rich_meta_opt
            .as_ref()
            .and_then(|rc| rc.peer_username.clone().or_else(|| rc.username.clone()))
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                rich_meta_opt
                    .as_ref()
                    .and_then(|rc| rc.peer_user_id)
                    .filter(|&id| id > 0)
                    .map(|id| id.to_string())
            })
            .or_else(|| {
                self.session_sync
                    .chat_peer_user_ids
                    .get(&chat_id)
                    .filter(|&&id| id > 0)
                    .map(|id| id.to_string())
            });
        self.load_historical_messages(chat_id, cx);
        self.sync_composer_and_inspector_miniapp_state(cx);
        self.sync_active_bot_computer_to_inspector(cx);
        self.sync_fabushi_bots_to_views(cx);
        let lang = self.active_lang_id().to_string();
        let contacts = self.session_sync.contacts_list.clone();
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let details = client.get_chat(chat_id as i64).await.ok();
            let rich_chat = client.hermes_get_chat_rich(chat_id as i64).await.ok();
            let rc_is_bot_flag = rich_chat.as_ref().is_some_and(|rc| rc.is_bot) || sb_is_bot;
            let bot_target = if rc_is_bot_flag {
                rich_chat
                    .as_ref()
                    .and_then(|rc| rc.peer_username.clone().or_else(|| rc.username.clone()))
                    .filter(|s| !s.trim().is_empty())
                    .or_else(|| {
                        rich_chat
                            .as_ref()
                            .and_then(|rc| rc.peer_user_id)
                            .filter(|&id| id > 0)
                            .map(|id| id.to_string())
                    })
                    .or(fallback_bot_target)
            } else {
                None
            };
            let bot_profile = if let Some(ref target) = bot_target {
                client.get_bot_public_profile(target).await.ok()
            } else {
                None
            };
            let members = client.get_chat_members(chat_id as i64).await.ok();
            let invites = client.list_invites(chat_id as i64).await.ok();
            let pins = client.list_pinned_messages(chat_id as i64).await.ok();
            let draft = client.get_draft(chat_id as i64).await.ok().flatten();
            let stats = client.get_channel_statistics(chat_id as i64).await.ok();
            if max_msg_id > 0 {
                let _ = client.mark_chat_read(chat_id as i64, max_msg_id, None).await;
            }
            let _ = weak.update(app, |shell, cx| {
                if !fence_matches(&fence, shell) {
                    return;
                }
                if let Some(ref prof) = bot_profile {
                    let key = prof.bot.username.trim().trim_start_matches('@').to_ascii_lowercase();
                    if !key.is_empty() {
                        shell.miniapps.bot_profiles_cache.insert(key, prof.clone());
                    }
                }
                if let Some(ref rc) = rich_chat {
                    shell.session_sync.rich_chats_by_id.insert(chat_id, rc.clone());
                    if let Some(pid) = rc.peer_user_id.filter(|&pid| pid > 0) {
                        shell.session_sync.chat_peer_user_ids.insert(chat_id, pid);
                    }
                    if rc.is_bot {
                        shell.sidebar.update(cx, |sb, _| { sb.bot_ids.insert(chat_id); });
                    }
                    let lang_for_stage = lang.clone();
                    shell.stage.update(cx, |st, st_cx| {
                        if st.active_chat_id == Some(chat_id) {
                            st.is_muted_chat = rc.is_muted;
                            st.is_pinned_chat = rc.is_pinned;
                            st.is_peer_blocked = rc.is_blocked;
                            st.is_group_chat = rc.is_group || rc.is_channel || matches!(rc.chat_type, 1 | 2 | 3);
                            st.is_saved_messages = rc.chat_type == 5
                                || rc.kind.as_deref() == Some("saved")
                                || (rc.chat_type == 0 && rc.peer_user_id.is_none() && rc.members_count <= 1 && !rc.is_bot);
                            st.message_ttl_seconds = rc.message_ttl_seconds;
                            if st.is_group_chat && rc.members_count > 0 && st.chat_subtitle.trim().is_empty() {
                                st.chat_subtitle = if lang_for_stage.starts_with("zh") {
                                    format!("{} 位成员", rc.members_count)
                                } else {
                                    format!("{} members", rc.members_count)
                                };
                            }
                            st_cx.notify();
                        }
                    });
                }
                let viewer_uid = shell.session_sync.user_id;
                let peer_uid_for_fabushi = rich_chat
                    .as_ref()
                    .and_then(|rc| rc.peer_user_id)
                    .or_else(|| bot_profile.as_ref().map(|p| p.bot.bot_user_id.max(0) as u64));
                let is_fabushi_owner_bot = peer_uid_for_fabushi.is_some_and(|uid| {
                    shell
                        .miniapps
                        .fabushi_bots
                        .iter()
                        .any(|fb| fb.bot_user_id > 0 && fb.bot_user_id as u64 == uid)
                }) || bot_profile.as_ref().is_some_and(|p| {
                    p.bot.is_fabushi_bot && viewer_uid.is_some_and(|u| p.bot.owner_user_id == u as i64)
                });
                shell.inspector.update(cx, |insp, _| {
                    let mut username = String::new();
                    let mut phone = String::new();
                    let mut bio = String::new();
                    let mut status = insp.profile.status.clone();
                    let mut avatar_path = details.as_ref().and_then(|d| d.photo_file_id.clone());
                    let mut facts = details.as_ref().map(|d| build_chat_profile_facts(d, &lang)).unwrap_or_default();

                    if let Some(ref rc) = rich_chat {
                        let resolved_title = rc
                            .peer_display_name
                            .as_deref()
                            .filter(|s| !s.trim().is_empty())
                            .or_else(|| Some(rc.title.trim()).filter(|t| !t.is_empty()));
                        if let Some(title) = resolved_title {
                            insp.profile.name = title.to_string();
                            let init = derive_initials(title);
                            if !init.is_empty() {
                                insp.profile.avatar_initials = init;
                            }
                        }
                        username = normalized_username(rc.peer_username.as_deref().or(rc.username.as_deref()));
                        phone = rc.peer_phone.clone().unwrap_or_default();
                        bio = rc.peer_bio.as_deref().or(rc.about.as_deref()).unwrap_or("").trim().to_string();
                        if rc.is_bot {
                            if is_fabushi_owner_bot {
                                status = tr_str(&lang, "AI 机器人 · Fabushi 智能体", "AI Bot · Fabushi Agent").to_string();
                            } else if let Some(ref prof) = bot_profile {
                                if prof.bot.mau_count > 0 {
                                    status = crate::views::drawer::bot_computer_inspector::format_bot_mau_subtitle(
                                        prof.bot.mau_count,
                                        &lang,
                                    );
                                } else {
                                    status = tr_str(&lang, "机器人", "bot").to_string();
                                }
                            } else {
                                status = tr_str(&lang, "机器人", "bot").to_string();
                            }
                        }
                        facts = enrich_facts_with_rich_chat(facts, rc, is_fabushi_owner_bot, &lang);
                    }
                    if let Some(ref prof) = bot_profile {
                        if username.is_empty() && !prof.bot.username.trim().is_empty() {
                            username = normalized_username(Some(prof.bot.username.as_str()));
                        }
                        if bio.is_empty() {
                            let b = if !prof.bot.about.trim().is_empty() {
                                prof.bot.about.trim()
                            } else {
                                prof.bot.description.trim()
                            };
                            if !b.is_empty() {
                                bio = b.to_string();
                            }
                        }
                    }
                    if username.is_empty() {
                        let contact = rich_chat
                            .as_ref()
                            .and_then(|rc| rc.peer_user_id)
                            .and_then(|pid| contacts.iter().find(|c| c.user_id == pid))
                            .or_else(|| contacts.iter().find(|c| c.chat_id == Some(chat_id as i64) || c.user_id == chat_id));
                        if let Some(c) = contact {
                            username = normalized_username(c.username.as_deref());
                            if phone.is_empty() {
                                phone = c.phone.clone().unwrap_or_default();
                            }
                        }
                    }
                    if bio.is_empty() {
                        if let Some(about) = details.as_ref().and_then(|d| d.about.as_ref()).filter(|a| !a.trim().is_empty()) {
                            bio = about.trim().to_string();
                        }
                    }
                    if !bio.is_empty() {
                        insp.profile.bio = bio;
                    }
                    if avatar_path.is_none() {
                        avatar_path = insp.profile.avatar_path.clone();
                    }
                    insp.sync_profile_details(username, phone, status, avatar_path, facts);
                    let inv = invites.as_ref().and_then(|v| v.first()).map(|i| i.link.clone()).unwrap_or_default();
                    let is_grp = rich_chat
                        .as_ref()
                        .map(|rc| rc.is_group || rc.is_channel || matches!(rc.chat_type, 1 | 2 | 3))
                        .unwrap_or_else(|| details.as_ref().is_some_and(|d| d.chat_type != 0) && !rc_is_bot_flag);
                    if is_grp {
                        if let Some(ref mems) = members {
                            insp.sync_members(map_chat_members_to_inspector(mems, &contacts), inv, true);
                        } else {
                            insp.sync_members(Vec::new(), inv, true);
                        }
                    } else {
                        insp.sync_members(Vec::new(), String::new(), false);
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
                    shell.sidebar.update(cx, |sb, _| sb.set_draft(chat_id, dr.text.clone()));
                    if !dr.text.is_empty() && shell.composer.read(cx).text().is_empty() {
                        shell.composer.update(cx, |c, _| c.set_text(dr.text));
                    }
                }
                shell.sync_composer_and_inspector_miniapp_state(cx);
                shell.sync_active_bot_computer_to_inspector(cx);
                shell.sync_fabushi_bots_to_views(cx);
                cx.notify();
            });
        })
        .detach();
    }

    /// Dispatches Hermes inline callback commands, approvals, Talk Mode, Remote Computer, or Mini-Apps.
    pub fn execute_hermes_inline_callback(&mut self, raw_action: String, cx: &mut Context<Self>) {
        let trimmed = raw_action.trim().to_string();
        let lower = trimmed.to_ascii_lowercase();
        let lang = self.active_lang_id();

        if matches!(
            lower.as_str(),
            "hermes:open_ai_providers"
                | "open_ai_providers"
                | "action_open_ai_providers"
                | "/providers"
                | "/magpie"
        ) {
            self.open_ai_providers_settings(cx);
            return;
        }

        if matches!(
            lower.as_str(),
            "action_talk_mode" | "talk_mode" | "talk:start" | "/talk" | "launch talk mode"
        ) || lower.contains("talk_mode")
            || lower.starts_with("talk:")
        {
            self.nav_rail.update(cx, |r, _| r.set_active_destination(NavDestination::TalkMode));
            self.start_ai_talk_mode_session(false, cx);
            return;
        }

        if matches!(
            lower.as_str(),
            "action_remote_computer" | "remote_computer" | "computer:open" | "/computer" | "remote computer"
        ) || lower.contains("remote_computer")
            || lower.starts_with("computer:")
        {
            self.nav_rail.update(cx, |r, _| r.set_active_destination(NavDestination::RemoteComputer));
            self.open_computer_workbench_live(cx);
            return;
        }

        if parse_miniapp_uri(&trimmed).is_some() {
            let active_chat = resolve_active_target_chat_id(
                self.stage.read(cx).active_chat_id,
                self.sidebar.read(cx).active_chat_id,
            );
            self.launch_miniapp_from_uri(&trimmed, active_chat, MiniAppLaunchMode::InlineKeyboard, cx);
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
        let bot_id = self.active_chat_bot_id(cx).or_else(|| {
            active_chat.and_then(|cid| {
                self.session_sync
                    .rich_chats_by_id
                    .get(&cid)
                    .and_then(|rc| rc.peer_user_id)
                    .or_else(|| self.session_sync.chat_peer_user_ids.get(&cid).copied())
            })
        });
        let session_id = self.rtc_talk.active_talk_session_id.clone();

        if lower.starts_with("approval:")
            || lower.starts_with("/approve")
            || lower.starts_with("/deny")
            || lower.starts_with("hermes:")
        {
            let client = self.client.clone();
            let Some(fence) = session_fence(self) else {
                return;
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
                    let cmd_text = trimmed.strip_prefix("hermes:").unwrap_or(&trimmed).trim().to_string();
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
                    if !fence_matches(&fence, shell) {
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

    /// Drains pending user intents queued by `ConversationStage` and routes each to shell actions or RPCs.
    pub fn handle_stage_pending_actions(&mut self, cx: &mut Context<Self>) {
        let lang = self.active_lang_id();
        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        let (
            (call_v, call_vid),
            (media_single, media_gal, iv_url),
            (edit_req, fwd_ids, del_ids, pin_req, tr_id, poll_vote, react_reqs),
            (voice_play, dl_req, drops, open_prof),
            (clear_hist, export_chat, toggle_mute, change_wallpaper, toggle_pin_chat, set_ttl_secs, toggle_block_peer, delete_or_leave_chat),
            (bot_act, bot_btn_act),
        ) = self.stage.update(cx, |st, _| {
            (
                (mem::take(&mut st.pending_call_request), mem::take(&mut st.pending_video_call)),
                (st.pending_media_url.take(), st.pending_media_viewer.take(), st.pending_iv_url.take()),
                (
                    st.pending_edit_message.take(), st.pending_forward_message_ids.take(),
                    st.pending_delete_message_ids.take(), st.pending_pin_toggle.take(),
                    st.pending_translate_message_id.take(), st.pending_poll_vote.take(),
                    st.drain_pending_reaction_actions(),
                ),
                (
                    st.pending_voice_playback.take(), st.pending_download_url.take(),
                    st.pending_dropped_paths.take(), st.pending_open_profile_peer_id.take(),
                ),
                (
                    mem::take(&mut st.pending_clear_history), mem::take(&mut st.pending_export_chat),
                    mem::take(&mut st.pending_toggle_mute), mem::take(&mut st.pending_change_wallpaper),
                    mem::take(&mut st.pending_toggle_pin_chat), st.pending_set_ttl_seconds.take(),
                    mem::take(&mut st.pending_toggle_block_peer), mem::take(&mut st.pending_delete_or_leave_chat),
                ),
                (st.pending_bot_action.take(), st.pending_bot_button_action.take()),
            )
        });
        if self.stage.read(cx).more_menu_open && self.sidebar.read(cx).is_plus_menu_open {
            self.sidebar.update(cx, |sb, sb_cx| { sb.is_plus_menu_open = false; sb_cx.notify(); });
        }
        if call_v || call_vid {
            if self.is_active_chat_bot(cx) {
                self.start_ai_talk_mode_session(call_vid, cx);
            } else if let Some(pid) = self.current_peer_user_id(cx) {
                let name = self.stage.read(cx).chat_title.clone();
                self.start_webrtc_call(pid, name, call_vid, cx);
            } else {
                self.window_runtime.enqueue_toast(
                    tr_str(lang, "当前群组或会话未绑定直接通话用户，请进入私聊或机器人对话。", "No direct user peer resolved for WebRTC call in this chat."),
                    3000,
                );
            }
        }
        if let Some((items, idx)) = media_gal {
            let mv_items = items
                .into_iter()
                .enumerate()
                .map(|(i, (url, is_video, caption))| MediaViewerItem {
                    id: i as u64 + 1, url, is_video, caption,
                    sender_name: String::new(), date: String::new(), duration_secs: None,
                })
                .collect();
            attach_media_viewer_overlay(self, MediaViewer::from_gallery(mv_items, idx), lang, cx);
        } else if let Some((url, is_vid)) = media_single {
            attach_media_viewer_overlay(self, MediaViewer::new(url, is_vid), lang, cx);
        }
        if let Some((url, title)) = iv_url {
            let pal = self.theme_state.palette;
            let overlay = cx.new(|_| { let mut iv = IvViewer::new(url, title); iv.apply_theme(pal, lang); iv });
            cx.observe(&overlay, |shell: &mut AppShell, ov: gpui::Entity<IvViewer>, cx: &mut Context<AppShell>| {
                let (close, share) = ov.update(cx, |v, _| (v.should_close, v.pending_share_url.take()));
                if let Some(u) = share {
                    shell.append_outgoing_text(u, cx);
                }
                if close {
                    shell.overlays.iv_viewer_overlay = None;
                    cx.notify();
                }
            })
            .detach();
            self.overlays.iv_viewer_overlay = Some(overlay);
        }
        if let Some((mid, existing)) = edit_req {
            self.composer.update(cx, |c, _| c.begin_edit(mid, existing));
        }
        if let Some(ids) = fwd_ids {
            let preview_snippet = format!("{} {}", ids.len(), tr_str(lang, "条消息", "messages"));
            let kind = ModalDialogKind::ForwardMessages { message_ids: ids, preview_snippet };
            self.open_modal_dialog(tr_str(lang, "转发消息", "Forward Messages"), kind, 420.0, 460.0, cx);
        }
        if let Some(ids) = del_ids {
            let peer_name = self.stage.read(cx).chat_title.clone();
            let kind = ModalDialogKind::DeleteMessages { message_ids: ids, peer_name };
            self.open_modal_dialog(tr_str(lang, "删除消息", "Delete Messages"), kind, 380.0, 260.0, cx);
        }
        if let Some((mid, pin)) = pin_req {
            if pin {
                let peer_name = self.stage.read(cx).chat_title.clone();
                let kind = ModalDialogKind::PinMessage { message_id: mid, peer_name, is_group: false };
                self.open_modal_dialog(tr_str(lang, "置顶消息", "Pin Message"), kind, 380.0, 240.0, cx);
            } else if let Some(cid) = active_chat {
                self.stage.update(cx, |st, _| st.toggle_pin_message(mid));
                let c = self.client.clone();
                cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| { let _ = c.unpin_message(cid as i64, mid as i64).await; }).detach();
            }
        }
        if let Some(mid) = tr_id {
            if let Some(original_text) = self.stage.read(cx).copy_message_text(mid) {
                let kind = ModalDialogKind::TranslateMessage { original_text };
                self.open_modal_dialog(tr_str(lang, "翻译消息", "Translate Message"), kind, 420.0, 320.0, cx);
            }
        }
        if let Some((mid, opt)) = poll_vote {
            self.stage.update(cx, |st, _| st.vote_poll(mid, opt));
            let c = self.client.clone();
            cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| { let _ = c.vote_poll(mid, vec![opt as u8]).await; }).detach();
        }
        if let Some(cid) = active_chat {
            for (mid, emoji, is_added) in react_reqs {
                self.dispatch_reaction_mutation(cid, mid, emoji, is_added, cx);
            }
        }
        if let Some((mid, play, _pos, spd)) = voice_play {
            self.rtc_talk.audio_player.set_speed(spd as f64);
            if !play {
                let _ = self.rtc_talk.audio_player.pause();
            } else {
                let key = format!("voice_{mid}");
                let c = self.client.clone();
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    if let Ok((_, bytes, _)) = c.download_media_range(&key, 0, None).await {
                        let _ = weak.update(app, |s, _| { let _ = s.rtc_talk.audio_player.play_wav_bytes(key, &bytes); });
                    }
                })
                .detach();
            }
        }
        if let Some((url, name)) = dl_req {
            let c = self.client.clone();
            cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                let _ = c.download_media_range(&url, 0, None).await;
                let _ = weak.update(app, |s, cx| {
                    let pfx = tr_str(s.active_lang_id(), "已保存文件", "Saved");
                    s.window_runtime.enqueue_toast(format!("{pfx}: {name}"), 2500);
                    cx.notify();
                });
            })
            .detach();
        }
        if let Some(paths) = drops {
            self.upload_and_send_paths(paths, cx);
        }
        if open_prof.is_some() {
            self.set_inspector_open(true, cx);
        }
        if clear_hist {
            let peer_name = self.stage.read(cx).chat_title.clone();
            self.open_modal_dialog(tr_str(lang, "清空聊天记录", "Clear History"), ModalDialogKind::ClearChatHistory { peer_name }, 390.0, 250.0, cx);
        }
        if export_chat {
            let chat_title = self.stage.read(cx).chat_title.clone();
            self.open_modal_dialog(tr_str(lang, "导出聊天", "Export Chat"), ModalDialogKind::ExportData { chat_title }, 400.0, 320.0, cx);
        }
        if let (true, Some(cid)) = (toggle_mute, active_chat) {
            toggle_chat_dialog_flag(self, cid, false, lang, cx);
        }
        if change_wallpaper {
            let peer_name = Some(self.stage.read(cx).chat_title.clone());
            self.open_modal_dialog(tr_str(lang, "聊天背景设置", "Chat Wallpaper"), ModalDialogKind::BackgroundPicker { peer_name }, 440.0, 420.0, cx);
        }
        if let (true, Some(cid)) = (toggle_pin_chat, active_chat) {
            toggle_chat_dialog_flag(self, cid, true, lang, cx);
        }
        if let (Some(ttl_secs), Some(cid)) = (set_ttl_secs, active_chat) {
            let prev_ttl = self.session_sync.rich_chats_by_id.get(&cid).map(|rc| rc.message_ttl_seconds).unwrap_or(0);
            if let Some(rc) = self.session_sync.rich_chats_by_id.get_mut(&cid) {
                rc.message_ttl_seconds = ttl_secs;
            }
            let toast_msg = if ttl_secs == 0 {
                tr_str(lang, "已关闭自动删除消息", "Auto-delete turned off")
            } else if ttl_secs <= 86_400 {
                tr_str(lang, "自动删除已设为 1 天", "Auto-delete set to 1 day")
            } else if ttl_secs <= 604_800 {
                tr_str(lang, "自动删除已设为 1 周", "Auto-delete set to 1 week")
            } else {
                tr_str(lang, "自动删除已设为 1 个月", "Auto-delete set to 1 month")
            };
            self.window_runtime.enqueue_toast(toast_msg, 2200);
            let c = self.client.clone();
            let lang_owned = lang.to_string();
            cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                if c.set_chat_ttl(cid as i64, ttl_secs).await.is_err() {
                    let _ = weak.update(app, |s, cx| {
                        if let Some(rc) = s.session_sync.rich_chats_by_id.get_mut(&cid) {
                            rc.message_ttl_seconds = prev_ttl;
                        }
                        s.stage.update(cx, |st, st_cx| {
                            if st.active_chat_id == Some(cid) { st.message_ttl_seconds = prev_ttl; st_cx.notify(); }
                        });
                        s.window_runtime.enqueue_toast(tr_str(&lang_owned, "自动删除设置保存失败", "Failed to update auto-delete timer"), 2500);
                    });
                }
            })
            .detach();
        }
        if toggle_block_peer {
            let prev_blocked = self.stage.read(cx).is_peer_blocked;
            let my_uid = self.session_sync.user_id;
            if let Some(peer_uid) = self.current_peer_user_id(cx).filter(|&pid| pid > 0 && Some(pid) != my_uid) {
                let blocked_now = !prev_blocked;
                self.stage.update(cx, |st, st_cx| { st.is_peer_blocked = blocked_now; st_cx.notify(); });
                if let Some(rc) = active_chat.and_then(|cid| self.session_sync.rich_chats_by_id.get_mut(&cid)) {
                    rc.is_blocked = blocked_now;
                }
                let toast_msg = if blocked_now { tr_str(lang, "已屏蔽该用户", "User blocked") } else { tr_str(lang, "已取消屏蔽该用户", "User unblocked") };
                self.window_runtime.enqueue_toast(toast_msg, 2200);
                let c = self.client.clone();
                let lang_owned = lang.to_string();
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let res = if blocked_now { c.block_user(peer_uid).await.map(|_| ()) } else { c.unblock_user(peer_uid).await.map(|_| ()) };
                    let _ = weak.update(app, |s, cx| {
                        if res.is_ok() {
                            s.refresh_settings_truth(cx);
                        } else {
                            if let Some(rc) = active_chat.and_then(|cid| s.session_sync.rich_chats_by_id.get_mut(&cid)) {
                                rc.is_blocked = prev_blocked;
                            }
                            s.stage.update(cx, |st, st_cx| {
                                if st.active_chat_id == active_chat { st.is_peer_blocked = prev_blocked; st_cx.notify(); }
                            });
                            s.window_runtime.enqueue_toast(tr_str(&lang_owned, "屏蔽状态同步失败", "Failed to update block status"), 2500);
                        }
                    });
                })
                .detach();
            } else {
                self.window_runtime.enqueue_toast(tr_str(lang, "群组或收藏夹不支持单用户屏蔽", "Blocking applies only to direct user peers"), 2200);
            }
        }
        if delete_or_leave_chat {
            let st_ref = self.stage.read(cx);
            let peer_name = st_ref.chat_title.clone();
            let is_group = st_ref.is_group_chat
                || st_ref.chat_subtitle.contains("member")
                || st_ref.chat_subtitle.contains("subscriber")
                || st_ref.chat_subtitle == "group"
                || active_chat.and_then(|cid| self.session_sync.rich_chats_by_id.get(&cid)).is_some_and(|rc| rc.is_group || rc.is_channel || matches!(rc.chat_type, 1 | 2 | 3));
            let header = if is_group { tr_str(lang, "退出群组", "Leave Group") } else { tr_str(lang, "删除对话", "Delete Chat") };
            self.open_modal_dialog(header, ModalDialogKind::LeaveOrDeleteChat { peer_name, is_group }, 390.0, 230.0, cx);
        }
        if let Some(act) = bot_act {
            self.execute_hermes_inline_callback(act, cx);
        }
        if let Some((btn_msg_id, btn)) = bot_btn_act {
            self.handle_stage_bot_button(btn_msg_id, btn, cx);
        }
    }
}
