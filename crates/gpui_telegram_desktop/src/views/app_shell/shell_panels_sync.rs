//! Secondary panel synchronization: folders, contacts, wallet credits, inspector, and sidebar actions.

use gpui::{AppContext, AsyncApp, ClipboardItem, Context, WeakEntity};
use gpui_telegram_proto::miniapp_contracts::MiniAppLaunchMode;

use crate::i18n::tr_str;
use crate::views::app_shell::shell_actions::{contacts_to_modal_rows, resolve_active_target_chat_id};
use crate::views::app_shell::shell_settings_sync::{apply_folders_to_shell, apply_wallet_to_shell};
use crate::views::app_shell::shell_truth::SessionRequestFence;
use crate::views::boxes::modal_types::ModalDialogKind;
use crate::views::drawer::inspector_tabs::InspectorAction;
use crate::views::shell::AppShell;
use crate::views::sidebar::dialogs_row_view::SidebarContextAction;
use crate::views::MediaViewer;

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

impl AppShell {
    /// Fetches chat folders from backend and updates navigation rail and folder settings.
    pub fn refresh_folders(&mut self, cx: &mut Context<Self>) {
        let Some(fence) = session_fence(self) else {
            return;
        };
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            if let Ok(folders) = client.list_folders().await {
                let _ = weak.update(app, |shell, cx| {
                    if !fence_matches(&fence, shell) {
                        return;
                    }
                    apply_folders_to_shell(shell, &folders, cx);
                    cx.notify();
                });
            }
        })
        .detach();
    }

    /// Fetches user contacts from backend and syncs both shell cache and contacts modal.
    pub fn refresh_contacts(&mut self, cx: &mut Context<Self>) {
        let Some(fence) = session_fence(self) else {
            return;
        };
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            if let Ok(list) = client.list_contacts().await {
                let _ = weak.update(app, |shell, cx| {
                    if !fence_matches(&fence, shell) {
                        return;
                    }
                    let rows = contacts_to_modal_rows(&list);
                    let contact_cids: std::collections::HashSet<u64> = list
                        .iter()
                        .filter(|c| !c.is_bot)
                        .filter_map(|c| c.chat_id.map(|cid| cid as u64))
                        .collect();
                    shell.session_sync.contacts_list = list;
                    shell.sidebar.update(cx, |sb, sb_cx| {
                        sb.contact_chat_ids = contact_cids;
                        sb.recompute_folder_badges();
                        sb_cx.notify();
                    });
                    shell
                        .modal_container
                        .update(cx, |m, _| m.dialog.sync_contacts(rows));
                    cx.notify();
                });
            }
        })
        .detach();
    }

    /// Fetches Stars and TON wallet balance/transactions and lazily updates credits settings.
    pub fn refresh_wallet(&mut self, cx: &mut Context<Self>) {
        let Some(fence) = session_fence(self) else {
            return;
        };
        let uname = self.session_sync.username.clone().unwrap_or_default();
        self.settings_router.update(cx, |sr, scx| {
            if let Some(ref sm) = sr.settings_main {
                sm.update(scx, |sm, _| {
                    if sm.profile.name.is_empty() && sm.profile.username.is_empty() {
                        sm.sync_profile(uname.clone(), String::new(), uname, String::new(), false, false);
                    }
                });
            }
        });
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let bal = client.wallet_balance().await.ok();
            let txs = client.wallet_transactions().await.unwrap_or_default();
            let _ = weak.update(app, |shell, cx| {
                if !fence_matches(&fence, shell) {
                    return;
                }
                if let Some(b) = bal {
                    apply_wallet_to_shell(shell, &b, &txs, cx);
                }
                cx.notify();
            });
        })
        .detach();
    }

    /// Routes actions emitted by the right-side `ContextInspector` drawer.
    pub fn handle_inspector_action(&mut self, action: InspectorAction, cx: &mut Context<Self>) {
        let lang = self.active_lang_id();
        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        match action {
            InspectorAction::StartVoiceCall(name) => {
                if self.is_active_chat_bot(cx) {
                    self.start_ai_talk_mode_session(false, cx);
                } else if let Some(pid) = self.current_peer_user_id(cx) {
                    self.start_webrtc_call(pid, name, false, cx);
                } else {
                    self.window_runtime.enqueue_toast(
                        tr_str(lang, "请选择私聊用户或机器人对话", "Select a direct peer or bot chat"),
                        2600,
                    );
                }
            }
            InspectorAction::StartVideoCall(name) => {
                if self.is_active_chat_bot(cx) {
                    self.start_ai_talk_mode_session(true, cx);
                } else if let Some(pid) = self.current_peer_user_id(cx) {
                    self.start_webrtc_call(pid, name, true, cx);
                } else {
                    self.window_runtime.enqueue_toast(
                        tr_str(lang, "请选择私聊用户或机器人对话", "Select a direct peer or bot chat"),
                        2600,
                    );
                }
            }
            InspectorAction::CopyText(txt) => {
                cx.write_to_clipboard(ClipboardItem::new_string(txt))
            }
            InspectorAction::ToggleNotifications(enabled) => {
                if let Some(cid) = active_chat {
                    let c = self.client.clone();
                    cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                        let _ = c.set_chat_muted(cid as i64, !enabled, None).await;
                    })
                    .detach();
                }
            }
            InspectorAction::OpenMedia { url, is_video } => {
                let pal = self.theme_state.palette;
                self.overlays.media_viewer_overlay = Some(cx.new(|_| {
                    let mut m = MediaViewer::new(url, is_video);
                    m.apply_theme(pal, lang);
                    m
                }));
            }
            InspectorAction::OpenEditPeer => {
                let prof = &self.inspector.read(cx).profile;
                self.open_modal_dialog(
                    tr_str(lang, "编辑资料", "Edit Info"),
                    ModalDialogKind::EditPeer {
                        peer_title: prof.name.clone(),
                        peer_username: prof.username.clone(),
                        peer_bio: prof.bio.clone(),
                        is_channel: false,
                    },
                    420.0,
                    380.0,
                    cx,
                );
            }
            InspectorAction::RevokeInviteLink => {
                if let Some(cid) = active_chat {
                    let c = self.client.clone();
                    cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                        if let Ok(invs) = c.list_invites(cid as i64).await {
                            if let Some(first) = invs.first() {
                                let _ = c.revoke_invite(cid as i64, &first.invite_id).await;
                            }
                        }
                    })
                    .detach();
                }
            }
            InspectorAction::OpenAddMember => self.open_modal_dialog(
                tr_str(lang, "添加成员", "Add Member"),
                ModalDialogKind::Contacts,
                400.0,
                440.0,
                cx,
            ),
            InspectorAction::ChangeAutoDelete(secs) => {
                if let Some(cid) = active_chat {
                    let ttl_u32 = match secs.trim() {
                        "24 Hours" | "1d" => 86_400,
                        "7 Days" | "1w" => 604_800,
                        "30 Days" | "1m" => 2_592_000,
                        "Off" | "0" | "" => 0,
                        other => other.parse::<u32>().unwrap_or(0),
                    };
                    self.stage.update(cx, |st, st_cx| {
                        st.message_ttl_seconds = ttl_u32;
                        st_cx.notify();
                    });
                    if let Some(rc) = self.session_sync.rich_chats_by_id.get_mut(&cid) {
                        rc.message_ttl_seconds = ttl_u32;
                    }
                    let c = self.client.clone();
                    cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                        let _ = c.set_chat_ttl(cid as i64, ttl_u32).await;
                    })
                    .detach();
                }
            }
            InspectorAction::CycleMemberRole(_) => {}
            InspectorAction::LaunchMiniApp(url) => {
                if !self.launch_miniapp_from_uri(
                    &url,
                    active_chat,
                    MiniAppLaunchMode::MenuButton,
                    cx,
                ) {
                    if let Some(cid) = active_chat {
                        let peer_uid = self
                            .session_sync
                            .rich_chats_by_id
                            .get(&cid)
                            .and_then(|rc| rc.peer_user_id)
                            .or_else(|| self.session_sync.chat_peer_user_ids.get(&cid).copied());
                        if let Some(app) = self
                            .miniapps
                            .catalog_miniapps
                            .iter()
                            .chain(self.miniapps.installed_miniapps.iter())
                            .find(|a| {
                                a.direct_chat_id == Some(cid as i64)
                                    || peer_uid.is_some_and(|pid| a.bot_user_id == pid as i64)
                            })
                            .cloned()
                        {
                            self.launch_miniapp(
                                &app.bot_username,
                                &app.short_name,
                                Some(cid),
                                MiniAppLaunchMode::MenuButton,
                                None,
                                cx,
                            );
                        }
                    }
                }
            }
            InspectorAction::ToggleAttachMenuBot(bot_id) => {
                let in_attach_now = self.inspector.read(cx).profile.in_attach_menu;
                let target_app = self
                    .miniapps
                    .installed_miniapps
                    .iter()
                    .chain(self.miniapps.catalog_miniapps.iter())
                    .find(|a| {
                        (bot_id > 0 && a.bot_user_id == bot_id as i64)
                            || active_chat.is_some_and(|cid| a.direct_chat_id == Some(cid as i64))
                    })
                    .cloned();
                if let Some(app) = target_app {
                    let client = self.client.clone();
                    let Some(token) = self.session_sync.session_token.clone() else {
                        return;
                    };
                    let fence = SessionRequestFence {
                        auth_generation: self.session_sync.auth_generation,
                        request_generation: 0,
                        token,
                        user_id: self.session_sync.user_id.unwrap_or(0),
                    };
                    let bot_un = app.bot_username.clone();
                    let short = app.short_name.clone();
                    cx.spawn(async move |weak: WeakEntity<Self>, app_cx: &mut AsyncApp| {
                        let res = if in_attach_now {
                            client.uninstall_mini_app(&bot_un, &short).await
                        } else {
                            client.install_mini_app(&bot_un, &short, true).await
                        };
                        let _ = weak.update(app_cx, |shell, cx| {
                            if !fence_matches(&fence, shell) {
                                return;
                            }
                            if let Ok(r) = res {
                                if let Some(t) = r.toast {
                                    shell.window_runtime.enqueue_toast(t, 2200);
                                }
                                shell.refresh_miniapps_catalog(None, cx);
                            }
                        });
                    })
                    .detach();
                } else {
                    self.window_runtime.enqueue_toast(
                        tr_str(lang, "该机器人暂未绑定可安装的小程序", "No Mini App available for this bot"),
                        2200,
                    );
                }
            }
            InspectorAction::RunBotCommand(cmd) => {
                let trimmed = cmd.trim();
                if !trimmed.is_empty() {
                    let slash = if trimmed.starts_with('/') {
                        trimmed.to_string()
                    } else {
                        format!("/{trimmed}")
                    };
                    self.append_outgoing_text(slash, cx);
                }
            }
            act @ (InspectorAction::OpenBotComputersSubpage
            | InspectorAction::CloseBotComputersSubpage
            | InspectorAction::OpenRemoteWorkbench { .. }
            | InspectorAction::OpenComputerDetailInSettings { .. }
            | InspectorAction::OpenCreateCloudComputerModal { .. }
            | InspectorAction::BindBotToComputer { .. }
            | InspectorAction::SaveBotComputerConfig { .. }) => {
                let _ = self.handle_computer_inspector_action(act, cx);
            }
        }
        cx.notify();
    }

    /// Handles pin, mute, archive, clear history, and delete context-menu actions from the sidebar.
    pub fn handle_sidebar_context_action(
        &mut self,
        action: SidebarContextAction,
        cx: &mut Context<Self>,
    ) {
        self.sidebar
            .update(cx, |sb, _| sb.apply_context_action(action.clone()));
        let client = self.client.clone();
        match action {
            SidebarContextAction::TogglePin(id) => {
                let pinned = self
                    .sidebar
                    .read(cx)
                    .dialogs
                    .iter()
                    .find(|d| d.id == id)
                    .map(|d| d.is_pinned)
                    .unwrap_or(false);
                if self.stage.read(cx).active_chat_id == Some(id) {
                    self.stage.update(cx, |st, _| st.is_pinned_chat = pinned);
                }
                cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                    let _ = client.set_chat_pinned(id as i64, pinned).await;
                })
                .detach();
            }
            SidebarContextAction::ToggleMute(id) => {
                let muted = self
                    .sidebar
                    .read(cx)
                    .dialogs
                    .iter()
                    .find(|d| d.id == id)
                    .map(|d| d.is_muted)
                    .unwrap_or(false);
                if self.stage.read(cx).active_chat_id == Some(id) {
                    self.stage.update(cx, |st, _| st.is_muted_chat = muted);
                }
                cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                    let _ = client.set_chat_muted(id as i64, muted, None).await;
                })
                .detach();
            }
            SidebarContextAction::ToggleArchive(id) => {
                let arch = self.sidebar.read(cx).archived_ids.contains(&id);
                cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                    let _ = client.set_chat_archived(id as i64, arch).await;
                })
                .detach();
            }
            SidebarContextAction::DeleteChat(id) => {
                self.session_sync.rich_chats_by_id.remove(&id);
                if self.stage.read(cx).active_chat_id == Some(id) {
                    let next_id = self.sidebar.read(cx).dialogs.first().map(|d| d.id);
                    if let Some(nid) = next_id {
                        self.sidebar.update(cx, |sb, _| sb.select_dialog(nid));
                        self.sync_selected_chat_inspector_and_stage(nid, cx);
                    } else {
                        self.stage.update(cx, |st, _| st.clear_selection());
                    }
                }
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client.leave_or_delete_chat(id as i64).await;
                    let _ = weak.update(app, |s, cx| s.refresh_chats(cx));
                })
                .detach();
            }
            SidebarContextAction::ClearHistory(id) => {
                self.stage.update(cx, |st, _| {
                    st.chat_histories.insert(id, Vec::new());
                    if st.active_chat_id == Some(id) {
                        st.messages.clear();
                        st.row_height_cache.rebuild_prefix_sums(&st.messages);
                    }
                });
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let _ = client.clear_chat_history(id as i64).await;
                    let _ = weak.update(app, |s, cx| s.refresh_chats(cx));
                })
                .detach();
            }
            SidebarContextAction::ToggleUnread(_) => {}
        }
        cx.notify();
    }
}
