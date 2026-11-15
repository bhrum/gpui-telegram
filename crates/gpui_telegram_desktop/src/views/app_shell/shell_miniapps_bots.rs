//! Mini-App overlay lifecycle, bot creator modal orchestration, and inline bot button handlers.

use gpui::{AppContext, AsyncApp, ClipboardItem, Context, Entity, WeakEntity};
use gpui_telegram_proto::miniapp_contracts::{CreateBotRequest, MiniAppLaunchMode};

use crate::components::bots::bot_attach_web_view::{BotAttachWebView, BotWebViewPendingAction};
use crate::i18n::tr_str;
use crate::views::app_shell::shell_actions::resolve_active_target_chat_id;
use crate::views::app_shell::shell_truth::{proto_keyboard_to_stage, SessionRequestFence};
use crate::views::boxes::peers::create_managed_bot_box::CreateManagedBotBoxPeers;
use crate::views::shell::AppShell;
use crate::views::stage::cards::{InlineButtonType, MessageContent};

enum CloudStorageOp {
    Refresh,
    Set { key: String, value: String },
    Delete { key: String },
}

fn current_session_fence(shell: &AppShell) -> SessionRequestFence {
    SessionRequestFence {
        auth_generation: shell.session_sync.auth_generation,
        request_generation: 0,
        token: shell.session_sync.session_token.clone().unwrap_or_default(),
        user_id: shell.session_sync.user_id.unwrap_or(0),
    }
}

fn fence_matches_shell(fence: &SessionRequestFence, shell: &AppShell) -> bool {
    fence.matches(
        shell.is_authenticated,
        shell.session_sync.auth_generation,
        0,
        shell.session_sync.session_token.as_deref(),
        shell.session_sync.user_id,
    )
}

fn set_bot_box_error(
    box_weak: &WeakEntity<CreateManagedBotBoxPeers>,
    err: impl std::fmt::Display,
    cx: &mut Context<AppShell>,
) {
    let msg = format!("❌ {}", err);
    let _ = box_weak.update(cx, |b, b_cx| {
        b.is_submitting = false;
        b.status_message = Some(msg);
        b_cx.notify();
    });
}

fn spawn_cloud_storage_op(
    shell: &AppShell,
    overlay: &Entity<BotAttachWebView>,
    fence: SessionRequestFence,
    bot_username: String,
    short_name: String,
    op: CloudStorageOp,
    cx: &mut Context<AppShell>,
) {
    let client = shell.client.clone();
    let ov_weak = overlay.downgrade();
    cx.spawn(async move |weak: WeakEntity<AppShell>, app: &mut AsyncApp| {
        let toast_after: Option<&'static str> = match op {
            CloudStorageOp::Refresh => None,
            CloudStorageOp::Set { key, value } => {
                let _ = client
                    .set_cloud_storage_item(&bot_username, &short_name, &key, &value)
                    .await;
                Some("CloudStorage Saved")
            }
            CloudStorageOp::Delete { key } => {
                let _ = client
                    .delete_cloud_storage_item(&bot_username, &short_name, &key)
                    .await;
                Some("CloudStorage Deleted")
            }
        };
        let res = client.get_cloud_storage(&bot_username, &short_name).await;
        let _ = weak.update(app, |shell, cx| {
            if !fence_matches_shell(&fence, shell) {
                return;
            }
            if let Ok(items) = res {
                let _ = ov_weak.update(cx, |w, w_cx| {
                    w.apply_cloud_storage(items);
                    if let Some(msg) = toast_after {
                        w.status_toast = Some(msg.to_string());
                    }
                    w_cx.notify();
                });
            }
        });
    })
    .detach();
}

impl AppShell {
    /// Lazily initializes or updates the attached Mini-App WebView overlay entity.
    pub fn ensure_miniapp_overlay(&mut self, cx: &mut Context<Self>) -> Entity<BotAttachWebView> {
        let pal = self.theme_state.palette;
        let lang = self.active_lang_id().to_string();
        if let Some(ref existing) = self.miniapps.miniapp_overlay {
            existing.update(cx, |w, _| {
                w.set_palette(pal);
                w.set_language(lang);
            });
            return existing.clone();
        }
        let ent = cx.new(|_| {
            let mut w = BotAttachWebView::new();
            w.set_palette(pal);
            w.set_language(lang);
            w
        });
        cx.observe(&ent, |shell: &mut AppShell, ov: Entity<BotAttachWebView>, cx: &mut Context<AppShell>| {
            shell.drain_miniapp_overlay_actions(&ov, cx);
        })
        .detach();
        self.miniapps.miniapp_overlay = Some(ent.clone());
        ent
    }

    /// Lazily initializes or updates the managed bot creation modal entity.
    pub fn ensure_create_bot_box(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Entity<CreateManagedBotBoxPeers> {
        let pal = self.theme_state.palette;
        let lang = self.active_lang_id().to_string();
        if let Some(ref existing) = self.miniapps.create_bot_box {
            existing.update(cx, |b, _| {
                b.set_palette(pal);
                b.set_language(lang);
            });
            return existing.clone();
        }
        let ent = cx.new(|_| {
            let mut b = CreateManagedBotBoxPeers::new();
            b.set_palette(pal);
            b.set_language(lang);
            b
        });
        cx.observe(
            &ent,
            |shell: &mut AppShell, box_ent: Entity<CreateManagedBotBoxPeers>, cx: &mut Context<AppShell>| {
                shell.drain_create_bot_box_actions(&box_ent, cx);
            },
        )
        .detach();
        self.miniapps.create_bot_box = Some(ent.clone());
        ent
    }

    /// Opens the natural-language bot and Mini-App creator modal.
    pub fn open_nl_creator_modal(&mut self, cx: &mut Context<Self>) {
        let box_ent = self.ensure_create_bot_box(cx);
        box_ent.update(cx, |b, b_cx| {
            b.open_ai_creator();
            b_cx.notify();
        });
        self.main_menu_drawer.update(cx, |m, _| m.close());
        self.sidebar.update(cx, |s, _| {
            s.is_account_popover_open = false;
            s.search_focused = false;
        });
        cx.notify();
    }

    /// Consumes pending submissions or close requests from the bot creation modal.
    pub fn drain_create_bot_box_actions(
        &mut self,
        box_ent: &Entity<CreateManagedBotBoxPeers>,
        cx: &mut Context<Self>,
    ) {
        let (nl_prompt, creds, closed) = box_ent.update(cx, |b, _| {
            (
                b.take_submit_nl(),
                b.take_submit_credentials(),
                b.take_close(),
            )
        });
        if closed {
            self.miniapps.create_bot_box = None;
            cx.notify();
            return;
        }
        if !self.is_authenticated || self.session_sync.session_token.is_none() {
            return;
        }
        let lang = self.active_lang_id().to_string();
        let fence = current_session_fence(self);
        if let Some(prompt) = nl_prompt {
            let client = self.client.clone();
            let box_weak = box_ent.downgrade();
            cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                let res = client.create_from_natural_language(&prompt).await;
                let _ = weak.update(app, |shell, cx| {
                    if !fence_matches_shell(&fence, shell) {
                        return;
                    }
                    match res {
                        Ok(created) => {
                            shell.miniapps.create_bot_box = None;
                            shell.window_runtime.enqueue_toast(
                                format!(
                                    "{} @{}",
                                    tr_str(&lang, "已创建 AI 机器人与小程序", "Created Bot & Mini App"),
                                    created.bot.username
                                ),
                                2800,
                            );
                            let cid = created.companion_chat_id.max(0) as u64;
                            if cid > 0 {
                                shell.open_or_select_chat(
                                    cid,
                                    Some(created.bot.display_name.clone()),
                                    true,
                                    cx,
                                );
                            }
                            shell.refresh_chats(cx);
                            shell.refresh_miniapps_catalog(None, cx);
                            if let Some(app_entry) = created.app {
                                shell.launch_miniapp(
                                    &app_entry.bot_username,
                                    &app_entry.short_name,
                                    if cid > 0 { Some(cid) } else { None },
                                    MiniAppLaunchMode::DirectLink,
                                    None,
                                    cx,
                                );
                            }
                        }
                        Err(e) => set_bot_box_error(&box_weak, e, cx),
                    }
                    cx.notify();
                });
            })
            .detach();
        } else if let Some((name, username, _token, _is_biz)) = creds {
            let client = self.client.clone();
            let box_weak = box_ent.downgrade();
            cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                let req = CreateBotRequest {
                    display_name: name,
                    username,
                    description: String::new(),
                    about: String::new(),
                    ai_persona: String::new(),
                    menu_button_text: None,
                    menu_button_url: None,
                    allow_attach_menu: true,
                    attach_peer_types: vec![
                        "users".to_string(),
                        "bots".to_string(),
                        "groups".to_string(),
                    ],
                    commands: Vec::new(),
                };
                let res = client.create_bot(&req).await;
                let _ = weak.update(app, |shell, cx| {
                    if !fence_matches_shell(&fence, shell) {
                        return;
                    }
                    match res {
                        Ok(created) => {
                            shell.miniapps.create_bot_box = None;
                            shell.window_runtime.enqueue_toast(
                                format!(
                                    "{} @{}",
                                    tr_str(&lang, "已创建机器人", "Bot Created"),
                                    created.bot.username
                                ),
                                2600,
                            );
                            if let Some(cid) = created.companion_chat_id.filter(|&id| id > 0) {
                                shell.open_or_select_chat(
                                    cid as u64,
                                    Some(created.bot.display_name.clone()),
                                    true,
                                    cx,
                                );
                            }
                            shell.refresh_chats(cx);
                            shell.refresh_miniapps_catalog(None, cx);
                        }
                        Err(e) => set_bot_box_error(&box_weak, e, cx),
                    }
                    cx.notify();
                });
            })
            .detach();
        }
    }

    /// Processes pending WebView actions emitted by the active Mini-App overlay.
    pub fn drain_miniapp_overlay_actions(
        &mut self,
        overlay: &Entity<BotAttachWebView>,
        cx: &mut Context<Self>,
    ) {
        let actions = overlay.update(cx, |w, _| w.take_pending_actions());
        if actions.is_empty() {
            return;
        }
        let lang = self.active_lang_id().to_string();
        let fence = current_session_fence(self);
        for act in actions {
            let fence = fence.clone();
            match act {
                BotWebViewPendingAction::Close => {
                    self.miniapps.is_miniapp_minimized = false;
                    self.miniapps.miniapp_overlay = None;
                    cx.notify();
                }
                BotWebViewPendingAction::Minimize => {
                    self.miniapps.is_miniapp_minimized = true;
                    cx.notify();
                }
                BotWebViewPendingAction::Reload {
                    bot_username,
                    short_name,
                    chat_id,
                } => {
                    self.launch_miniapp(
                        &bot_username,
                        &short_name,
                        chat_id.filter(|&id| id > 0).map(|id| id as u64),
                        MiniAppLaunchMode::DirectLink,
                        None,
                        cx,
                    );
                }
                BotWebViewPendingAction::AcceptDisclaimer {
                    bot_username,
                    short_name,
                    allow_write_access,
                } => {
                    let client = self.client.clone();
                    let lang_c = lang.clone();
                    cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                        let res = client
                            .install_mini_app(&bot_username, &short_name, allow_write_access)
                            .await;
                        let _ = weak.update(app, |shell, cx| {
                            if !fence_matches_shell(&fence, shell) {
                                return;
                            }
                            if let Ok(r) = res {
                                if let Some(toast) = r.toast {
                                    shell.window_runtime.enqueue_toast(toast, 2200);
                                } else {
                                    shell.window_runtime.enqueue_toast(
                                        tr_str(&lang_c, "已授权并添加到附件菜单", "Authorized and added to attach menu"),
                                        2200,
                                    );
                                }
                                shell.refresh_miniapps_catalog(None, cx);
                            }
                        });
                    })
                    .detach();
                }
                BotWebViewPendingAction::OpenCompanionChat {
                    companion_chat_id,
                    bot_username,
                } => {
                    self.miniapps.is_miniapp_minimized = true;
                    if companion_chat_id > 0 {
                        self.open_or_select_chat(
                            companion_chat_id as u64,
                            Some(bot_username.clone()),
                            true,
                            cx,
                        );
                    } else if !bot_username.trim().is_empty() {
                        self.open_bot_chat_by_username(&bot_username, cx);
                    }
                    cx.notify();
                }
                BotWebViewPendingAction::OpenInBrowser { path } => {
                    let base = self.client.config.server_url.trim_end_matches('/');
                    let full_url = if path.starts_with("http://") || path.starts_with("https://") {
                        path
                    } else if path.starts_with('/') {
                        format!("{base}{path}")
                    } else {
                        format!("{base}/{path}")
                    };
                    cx.write_to_clipboard(ClipboardItem::new_string(full_url.clone()));
                    #[cfg(target_os = "macos")]
                    let _ = std::process::Command::new("open").arg(&full_url).spawn();
                    #[cfg(target_os = "linux")]
                    let _ = std::process::Command::new("xdg-open").arg(&full_url).spawn();
                    self.window_runtime.enqueue_toast(
                        format!("{}: {}", tr_str(&lang, "已在浏览器打开/复制链接", "Opened / Copied URL"), full_url),
                        2600,
                    );
                }
                BotWebViewPendingAction::UninstallApp {
                    bot_username,
                    short_name,
                } => {
                    self.miniapps.is_miniapp_minimized = false;
                    self.miniapps.miniapp_overlay = None;
                    let client = self.client.clone();
                    let lang_c = lang.clone();
                    cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                        let res = client.uninstall_mini_app(&bot_username, &short_name).await;
                        let _ = weak.update(app, |shell, cx| {
                            if !fence_matches_shell(&fence, shell) {
                                return;
                            }
                            match res {
                                Ok(r) => {
                                    let msg = r.toast.unwrap_or_else(|| {
                                        tr_str(&lang_c, "已移除小程序", "Removed Mini App").to_string()
                                    });
                                    shell.window_runtime.enqueue_toast(msg, 2200);
                                    shell.refresh_miniapps_catalog(None, cx);
                                }
                                Err(e) => {
                                    shell.window_runtime.enqueue_toast(format!("Remove error: {}", e), 2400);
                                }
                            }
                            cx.notify();
                        });
                    })
                    .detach();
                }
                BotWebViewPendingAction::SendAppAction {
                    bot_username,
                    short_name,
                    req,
                } => {
                    let is_send_data = req.action_type.eq_ignore_ascii_case("send_data");
                    let source_cid = req.chat_id.filter(|&id| id > 0).map(|id| id as u64);
                    let client = self.client.clone();
                    let ov_weak = overlay.downgrade();
                    cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                        let res = client
                            .send_mini_app_action(&bot_username, &short_name, &req)
                            .await;
                        let _ = weak.update(app, |shell, cx| {
                            if !fence_matches_shell(&fence, shell) {
                                return;
                            }
                            match res {
                                Ok(resp) => {
                                    if let Some(ref toast) = resp.toast {
                                        shell.window_runtime.enqueue_toast(toast, 2400);
                                    }
                                    let refresh_companion = resp
                                        .companion_chat_id
                                        .filter(|&id| id > 0)
                                        .map(|id| id as u64)
                                        .filter(|_| resp.companion_bot_message.is_some());
                                    let close_after = resp.close_app;
                                    let _ = ov_weak.update(cx, |w, w_cx| {
                                        w.apply_action_response(resp);
                                        w_cx.notify();
                                    });
                                    if close_after {
                                        shell.miniapps.is_miniapp_minimized = false;
                                        shell.miniapps.miniapp_overlay = None;
                                    }
                                    if let Some(cid) = refresh_companion {
                                        if shell.stage.read(cx).active_chat_id == Some(cid) {
                                            shell.load_historical_messages(cid, cx);
                                        }
                                        shell.refresh_chats(cx);
                                    }
                                    if is_send_data {
                                        if let Some(scid) = source_cid {
                                            shell.load_historical_messages(scid, cx);
                                        }
                                    }
                                }
                                Err(e) => {
                                    let _ = ov_weak.update(cx, |w, w_cx| {
                                        w.is_loading = false;
                                        w.status_toast = Some(format!("⚠️ {}", e));
                                        w_cx.notify();
                                    });
                                }
                            }
                            cx.notify();
                        });
                    })
                    .detach();
                }
                BotWebViewPendingAction::CloudStorageRefresh {
                    bot_username,
                    short_name,
                } => spawn_cloud_storage_op(
                    self,
                    overlay,
                    fence,
                    bot_username,
                    short_name,
                    CloudStorageOp::Refresh,
                    cx,
                ),
                BotWebViewPendingAction::CloudStorageSet {
                    bot_username,
                    short_name,
                    key,
                    value,
                } => spawn_cloud_storage_op(
                    self,
                    overlay,
                    fence,
                    bot_username,
                    short_name,
                    CloudStorageOp::Set { key, value },
                    cx,
                ),
                BotWebViewPendingAction::CloudStorageDelete {
                    bot_username,
                    short_name,
                    key,
                } => spawn_cloud_storage_op(
                    self,
                    overlay,
                    fence,
                    bot_username,
                    short_name,
                    CloudStorageOp::Delete { key },
                    cx,
                ),
            }
        }
    }

    /// Handles inline keyboard button clicks originating from conversation stage messages.
    pub fn handle_stage_bot_button(
        &mut self,
        msg_id: u64,
        btn_type: InlineButtonType,
        cx: &mut Context<Self>,
    ) {
        let lang = self.active_lang_id();
        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        match btn_type {
            InlineButtonType::WebApp(url) => {
                if !self.launch_miniapp_from_uri(
                    &url,
                    active_chat,
                    MiniAppLaunchMode::InlineKeyboard,
                    cx,
                ) {
                    cx.write_to_clipboard(ClipboardItem::new_string(url.clone()));
                    self.window_runtime.enqueue_toast(
                        format!("{}: {}", tr_str(lang, "已复制应用链接", "Copied WebApp URL"), url),
                        2400,
                    );
                }
            }
            InlineButtonType::Url(url) => {
                if !self.launch_miniapp_from_uri(
                    &url,
                    active_chat,
                    MiniAppLaunchMode::InlineKeyboard,
                    cx,
                ) {
                    cx.write_to_clipboard(ClipboardItem::new_string(url.clone()));
                    #[cfg(target_os = "macos")]
                    let _ = std::process::Command::new("open").arg(&url).spawn();
                    self.window_runtime.enqueue_toast(
                        format!("{}: {}", tr_str(lang, "已复制链接", "Copied URL"), url),
                        2400,
                    );
                }
            }
            InlineButtonType::SwitchInline(q) => {
                let bot_uname = active_chat
                    .and_then(|cid| self.session_sync.rich_chats_by_id.get(&cid))
                    .and_then(|rc| rc.peer_username.as_deref().or(rc.username.as_deref()))
                    .map(|s| s.trim().trim_start_matches('@').to_string())
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "BotFather".to_string());
                self.composer.update(cx, |c, c_cx| {
                    c.prefill_inline_bot_query(&bot_uname, &q);
                    c_cx.notify();
                });
                cx.notify();
            }
            InlineButtonType::Callback(data) => {
                let trimmed = data.trim().to_string();
                if trimmed.is_empty() {
                    return;
                }
                let lower = trimmed.to_ascii_lowercase();
                if lower == "action_talk_mode"
                    || lower == "talk_mode"
                    || lower.starts_with("talk:")
                    || lower == "action_remote_computer"
                    || lower == "remote_computer"
                    || lower.starts_with("computer:")
                    || lower.starts_with("approval:")
                    || lower.starts_with("hermes:")
                {
                    self.execute_hermes_inline_callback(trimmed, cx);
                    return;
                }
                let Some(cid) = active_chat else {
                    return;
                };
                if self.session_sync.session_token.is_none() {
                    return;
                }
                let fence = current_session_fence(self);
                let client = self.client.clone();
                let data_for_task = trimmed.clone();
                cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                    let res = client
                        .send_chat_callback(cid as i64, msg_id as i64, &data_for_task)
                        .await;
                    let _ = weak.update(app, |shell, cx| {
                        if !fence_matches_shell(&fence, shell) {
                            return;
                        }
                        match res {
                            Ok(result) => {
                                if let Some(ref toast) = result.toast {
                                    shell.window_runtime.enqueue_toast(toast, 2400);
                                }
                                let updated_txt = result.updated_text.clone();
                                let updated_kb = result
                                    .inline_keyboard
                                    .as_deref()
                                    .and_then(proto_keyboard_to_stage);
                                if updated_txt.is_some() || updated_kb.is_some() {
                                    shell.stage.update(cx, |st, st_cx| {
                                        if let Some(m) =
                                            st.messages.iter_mut().find(|m| m.id == msg_id)
                                        {
                                            if let Some(ref new_txt) = updated_txt {
                                                if let MessageContent::Text {
                                                    ref mut text,
                                                    ref mut is_edited,
                                                } = m.content
                                                {
                                                    *text = new_txt.clone();
                                                    *is_edited = true;
                                                }
                                            }
                                            if updated_kb.is_some() {
                                                m.bot_keyboard = updated_kb;
                                            }
                                        }
                                        st_cx.notify();
                                    });
                                }
                                if let Some(next_cid) = result
                                    .companion_chat_id
                                    .filter(|&id| id > 0 && id as u64 != cid)
                                {
                                    let title = result
                                        .companion_bot
                                        .as_ref()
                                        .map(|b| b.display_name.clone());
                                    shell.open_or_select_chat(next_cid as u64, title, true, cx);
                                } else {
                                    shell.load_historical_messages(cid, cx);
                                }
                                if result.app.is_some()
                                    || result.companion_bot.is_some()
                                    || data_for_task.contains("newbot")
                                    || data_for_task.contains("newapp")
                                    || data_for_task.contains("deleteapp")
                                {
                                    shell.refresh_miniapps_catalog(None, cx);
                                    shell.refresh_chats(cx);
                                }
                            }
                            Err(e) => {
                                shell
                                    .window_runtime
                                    .enqueue_toast(format!("Callback failed: {}", e), 2400);
                            }
                        }
                        cx.notify();
                    });
                })
                .detach();
            }
        }
    }
}
