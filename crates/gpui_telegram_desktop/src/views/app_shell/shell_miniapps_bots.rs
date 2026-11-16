//! Mini-App overlay lifecycle, bot creator modal orchestration, and inline bot button handlers.

use gpui::{AppContext, AsyncApp, ClipboardItem, Context, Entity, WeakEntity};
use gpui_telegram_proto::miniapp_contracts::{
    derive_safe_agent_id, normalize_fabushi_avatar_color, normalize_fabushi_avatar_shape,
    parse_miniapp_uri, AllowSendMessageRequest, BotRegistryEntry, CreateBotRequest,
    FabushiBotSummaryDto, MiniAppLaunchMode,
};

use crate::components::bots::bot_attach_web_view::{BotAttachWebView, BotWebViewPendingAction};
use crate::i18n::tr_str;
use crate::views::app_shell::shell_actions::resolve_active_target_chat_id;
use crate::views::app_shell::shell_truth::{proto_keyboard_to_stage, SessionRequestFence};
use crate::views::boxes::modal_types::ModalDialogKind;
use crate::views::boxes::peers::bot_profile_box::{BotProfileBox, BotProfileBoxAction};
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

fn upsert_fabushi_bot_summary(shell: &mut AppShell, summary: FabushiBotSummaryDto) {
    if let Some(slot) = shell
        .miniapps
        .fabushi_bots
        .iter_mut()
        .find(|b| b.bot_user_id == summary.bot_user_id)
    {
        *slot = summary;
    } else {
        shell.miniapps.fabushi_bots.push(summary);
    }
}

fn fabushi_summary_from_bot_registry(
    bot: &BotRegistryEntry,
    companion_chat_id: Option<i64>,
    fallback_shape: Option<&str>,
    fallback_color: Option<&str>,
) -> FabushiBotSummaryDto {
    let agent_id = if bot.agent_id.trim().is_empty() {
        derive_safe_agent_id(None, &bot.username, bot.bot_user_id)
    } else {
        bot.agent_id.clone()
    };
    let raw_shape = if bot.avatar_shape.trim().is_empty() {
        fallback_shape.unwrap_or("")
    } else {
        bot.avatar_shape.as_str()
    };
    let raw_color = if bot.avatar_color.trim().is_empty() {
        fallback_color.unwrap_or("")
    } else {
        bot.avatar_color.as_str()
    };
    let avatar_shape = normalize_fabushi_avatar_shape(raw_shape, &agent_id);
    let avatar_color = normalize_fabushi_avatar_color(raw_color, &agent_id);
    let description = if !bot.ai_persona.trim().is_empty() {
        bot.ai_persona.clone()
    } else {
        bot.description.clone()
    };
    FabushiBotSummaryDto {
        bot_user_id: bot.bot_user_id,
        owner_user_id: bot.owner_user_id,
        agent_id,
        username: bot.username.clone(),
        display_name: bot.display_name.clone(),
        description,
        title: bot.about.clone(),
        avatar_shape,
        avatar_color,
        preferred_model: bot.preferred_model.clone(),
        thinking_level: if bot.thinking_level.trim().is_empty() {
            "medium".to_string()
        } else {
            bot.thinking_level.clone()
        },
        approval_mode: if bot.approval_mode.trim().is_empty() {
            "prompt".to_string()
        } else {
            bot.approval_mode.clone()
        },
        toolset_preset: if bot.toolset_preset.trim().is_empty() {
            "full".to_string()
        } else {
            bot.toolset_preset.clone()
        },
        notify_on_agent_updates: true,
        hidden_from_sidebar: false,
        memory_entries_count: 0,
        skills_count: 0,
        execution_state: "idle".to_string(),
        companion_chat_id,
    }
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

    /// Opens the first-class Fabushi Bot creator modal (isolated workspace + Hermes agent).
    pub fn open_fabushi_creator_modal(&mut self, cx: &mut Context<Self>) {
        let box_ent = self.ensure_create_bot_box(cx);
        box_ent.update(cx, |b, b_cx| {
            b.open_fabushi_creator();
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
        let (fabushi_req, nl_prompt, creds, closed, av_shape, av_color) =
            box_ent.update(cx, |b, _| {
                let (av_shape, av_color) = b.effective_avatar_choice();
                (
                    b.take_submit_fabushi(),
                    b.take_submit_nl(),
                    b.take_submit_credentials(),
                    b.take_close(),
                    av_shape,
                    av_color,
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
        if let Some(req) = fabushi_req {
            let client = self.client.clone();
            let box_weak = box_ent.downgrade();
            cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                let res = client.create_fabushi_bot(&req).await;
                let _ = weak.update(app, |shell, cx| {
                    if !fence_matches_shell(&fence, shell) {
                        return;
                    }
                    match res {
                        Ok(created) => {
                            shell.miniapps.create_bot_box = None;
                            upsert_fabushi_bot_summary(shell, created.bot.clone());
                            shell.window_runtime.enqueue_toast(
                                format!(
                                    "{} @{}",
                                    tr_str(&lang, "已创建 Fabushi 智能体", "Created Fabushi Bot"),
                                    created.bot.agent_id
                                ),
                                2800,
                            );
                            let cid = created.companion_chat_id.max(0) as u64;
                            shell.sync_fabushi_bots_to_views(cx);
                            shell.refresh_chats(cx);
                            shell.refresh_miniapps_catalog(None, cx);
                            if cid > 0 {
                                shell.open_or_select_chat(
                                    cid,
                                    Some(created.bot.display_name.clone()),
                                    true,
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
        } else if let Some(prompt) = nl_prompt {
            let client = self.client.clone();
            let box_weak = box_ent.downgrade();
            cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                let res = client
                    .create_from_natural_language_with_avatar(
                        &prompt,
                        av_shape.clone(),
                        av_color.clone(),
                    )
                    .await;
                let _ = weak.update(app, |shell, cx| {
                    if !fence_matches_shell(&fence, shell) {
                        return;
                    }
                    match res {
                        Ok(created) => {
                            shell.miniapps.create_bot_box = None;
                            let summary = fabushi_summary_from_bot_registry(
                                &created.bot,
                                (created.companion_chat_id > 0).then_some(created.companion_chat_id),
                                av_shape.as_deref(),
                                av_color.as_deref(),
                            );
                            upsert_fabushi_bot_summary(shell, summary);
                            shell.sync_fabushi_bots_to_views(cx);
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
                    avatar_shape: av_shape.clone(),
                    avatar_color: av_color.clone(),
                    ..Default::default()
                };
                let res = client.create_bot(&req).await;
                let _ = weak.update(app, |shell, cx| {
                    if !fence_matches_shell(&fence, shell) {
                        return;
                    }
                    match res {
                        Ok(created) => {
                            shell.miniapps.create_bot_box = None;
                            let summary = fabushi_summary_from_bot_registry(
                                &created.bot,
                                created.companion_chat_id,
                                av_shape.as_deref(),
                                av_color.as_deref(),
                            );
                            upsert_fabushi_bot_summary(shell, summary);
                            shell.sync_fabushi_bots_to_views(cx);
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
                BotWebViewPendingAction::AllowBotSendMessage {
                    bot_id,
                    bot_username,
                    app_id,
                    app_title: _,
                } => {
                    let client = self.client.clone();
                    let ov_weak = overlay.downgrade();
                    let lang_c = lang.clone();
                    let resolved_id_hint = if bot_id > 0 {
                        bot_id
                    } else {
                        self.miniapps
                            .bot_profiles_cache
                            .get(&bot_username.trim().trim_start_matches('@').to_ascii_lowercase())
                            .map(|p| p.bot.bot_user_id.max(0) as u64)
                            .unwrap_or(0)
                    };
                    cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                        let mut target_bot_id = resolved_id_hint;
                        if target_bot_id == 0 && !bot_username.trim().is_empty() {
                            if let Ok(prof) = client.get_bot_public_profile(&bot_username).await {
                                target_bot_id = prof.bot.bot_user_id.max(0) as u64;
                            }
                        }
                        if target_bot_id == 0 {
                            let _ = weak.update(app, |shell, cx| {
                                let _ = ov_weak.update(cx, |w, w_cx| {
                                    w.resolve_write_access(false);
                                    w_cx.notify();
                                });
                                shell.window_runtime.enqueue_toast(
                                    tr_str(&lang_c, "无法解析机器人 ID", "Unable to resolve bot id"),
                                    2200,
                                );
                            });
                            return;
                        }
                        let req = AllowSendMessageRequest {
                            app_id,
                            web_app_name: None,
                            from_attachment_menu: false,
                        };
                        let res = client.allow_bot_send_message(target_bot_id, &req).await;
                        let _ = weak.update(app, |shell, cx| {
                            if !fence_matches_shell(&fence, shell) {
                                return;
                            }
                            match res {
                                Ok(resp) => {
                                    let _ = ov_weak.update(cx, |w, w_cx| {
                                        w.resolve_write_access(true);
                                        w_cx.notify();
                                    });
                                    let k = bot_username
                                        .trim()
                                        .trim_start_matches('@')
                                        .to_ascii_lowercase();
                                    if let Some(prof) = shell.miniapps.bot_profiles_cache.get_mut(&k)
                                    {
                                        prof.allows_write_to_pm = true;
                                        prof.bot.allows_write_to_pm = true;
                                    }
                                    shell.refresh_chats(cx);
                                    shell.refresh_recent_miniapps(cx);
                                    if resp.chat_id > 0
                                        && shell.stage.read(cx).active_chat_id
                                            == Some(resp.chat_id as u64)
                                    {
                                        shell.load_historical_messages(resp.chat_id as u64, cx);
                                    }
                                    shell.window_runtime.enqueue_toast(
                                        if !resp.service_text.trim().is_empty() {
                                            resp.service_text
                                        } else {
                                            tr_str(
                                                &lang_c,
                                                "已允许此机器人向您发送消息",
                                                "Bot is now allowed to message you",
                                            )
                                            .to_string()
                                        },
                                        2400,
                                    );
                                }
                                Err(e) => {
                                    let _ = ov_weak.update(cx, |w, w_cx| {
                                        w.resolve_write_access(false);
                                        w_cx.notify();
                                    });
                                    shell.window_runtime.enqueue_toast(
                                        format!("Allow write access failed: {}", e),
                                        2400,
                                    );
                                }
                            }
                            cx.notify();
                        });
                    })
                    .detach();
                }
                BotWebViewPendingAction::OpenBotProfile(bot_un) => {
                    self.miniapps.is_miniapp_minimized = true;
                    self.open_bot_profile_popup(&bot_un, cx);
                }
                BotWebViewPendingAction::OpenMiniAppTos => {
                    self.miniapps.is_miniapp_minimized = true;
                    self.miniapps.miniapp_tos_modal_open = true;
                    cx.notify();
                }
                BotWebViewPendingAction::OpenTelegramLink(raw_link) => {
                    let active_chat = resolve_active_target_chat_id(
                        self.stage.read(cx).active_chat_id,
                        self.sidebar.read(cx).active_chat_id,
                    );
                    if parse_miniapp_uri(&raw_link).is_some() {
                        let _ = self.launch_miniapp_from_uri(
                            &raw_link,
                            active_chat,
                            MiniAppLaunchMode::DirectLink,
                            cx,
                        );
                    } else {
                        let clean = raw_link
                            .trim()
                            .trim_start_matches("https://t.me/")
                            .trim_start_matches("http://t.me/")
                            .trim_start_matches("tg://resolve?domain=")
                            .trim_start_matches('/')
                            .split(['?', '/'])
                            .next()
                            .unwrap_or("")
                            .trim_start_matches('@')
                            .to_string();
                        if !clean.is_empty() {
                            self.miniapps.is_miniapp_minimized = true;
                            self.open_bot_profile_popup(&clean, cx);
                        }
                    }
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

    /// Copies an external URL, opens it in the system browser, and shows a localized toast.
    pub fn open_external_url_with_toast(&mut self, url: &str, cx: &mut Context<Self>) {
        let lang = self.active_lang_id();
        let trimmed = url.trim();
        if trimmed.is_empty() {
            self.window_runtime.enqueue_toast(
                tr_str(lang, "暂无可用链接", "No URL available"),
                2000,
            );
            return;
        }
        cx.write_to_clipboard(ClipboardItem::new_string(trimmed.to_string()));
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            #[cfg(target_os = "macos")]
            let _ = std::process::Command::new("open").arg(trimmed).spawn();
            #[cfg(target_os = "linux")]
            let _ = std::process::Command::new("xdg-open").arg(trimmed).spawn();
        }
        self.window_runtime.enqueue_toast(
            format!("{}: {}", tr_str(lang, "已打开链接", "Opened URL"), trimmed),
            2400,
        );
    }

    /// Toggles block/stop state for `bot_id` via `client.set_bot_blocked` and updates all views.
    pub fn toggle_bot_block_status(&mut self, bot_id: u64, cx: &mut Context<Self>) {
        if bot_id == 0 {
            return;
        }
        if !self.is_authenticated || self.session_sync.session_token.is_none() {
            return;
        }
        let fence = current_session_fence(self);
        let currently_blocked = self
            .miniapps
            .bot_profile_box
            .as_ref()
            .map(|b| b.read(cx))
            .filter(|b| b.profile.bot.bot_user_id == bot_id as i64)
            .map(|b| b.profile.is_blocked)
            .or_else(|| {
                self.miniapps
                    .bot_profiles_cache
                    .values()
                    .find(|p| p.bot.bot_user_id == bot_id as i64)
                    .map(|p| p.is_blocked)
            })
            .unwrap_or_else(|| self.inspector.read(cx).profile.is_blocked);
        let next_blocked = !currently_blocked;

        if let Some(ref bx) = self.miniapps.bot_profile_box {
            bx.update(cx, |b, b_cx| {
                if b.profile.bot.bot_user_id == bot_id as i64 {
                    b.profile.is_blocked = next_blocked;
                    b.profile.bot.is_blocked_by_viewer = next_blocked;
                    b_cx.notify();
                }
            });
        }
        for prof in self.miniapps.bot_profiles_cache.values_mut() {
            if prof.bot.bot_user_id == bot_id as i64 {
                prof.is_blocked = next_blocked;
                prof.bot.is_blocked_by_viewer = next_blocked;
            }
        }
        self.inspector.update(cx, |insp, insp_cx| {
            if insp.profile.bot_id == bot_id {
                insp.profile.is_blocked = next_blocked;
                insp_cx.notify();
            }
        });
        for rc in self.session_sync.rich_chats_by_id.values_mut() {
            if rc.peer_user_id == Some(bot_id) {
                rc.is_blocked = next_blocked;
            }
        }
        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        if let Some(cid) = active_chat {
            if self.session_sync.rich_chats_by_id.get(&cid).is_some_and(|rc| rc.peer_user_id == Some(bot_id)) {
                self.stage.update(cx, |st, st_cx| {
                    st.is_peer_blocked = next_blocked;
                    st_cx.notify();
                });
            }
        }

        let client = self.client.clone();
        let lang_c = self.active_lang_id().to_string();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.set_bot_blocked(bot_id, next_blocked).await;
            let _ = weak.update(app, |shell, cx| {
                if !fence_matches_shell(&fence, shell) {
                    return;
                }
                match res {
                    Ok(_) => {
                        shell.enqueue_localized_toast(
                            if next_blocked { "已停用并屏蔽此机器人" } else { "已重新启用此机器人" },
                            if next_blocked { "Bot stopped and blocked" } else { "Bot restarted and unblocked" },
                            2200,
                            cx,
                        );
                        shell.refresh_chats(cx);
                        shell.sync_composer_and_inspector_miniapp_state(cx);
                    }
                    Err(e) => {
                        shell.window_runtime.enqueue_toast(
                            format!("{}: {}", tr_str(&lang_c, "操作失败", "Failed"), e),
                            2400,
                        );
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    /// Processes pending actions emitted by `BotProfileBox`.
    pub fn drain_bot_profile_box_actions(
        &mut self,
        box_entity: &Entity<BotProfileBox>,
        cx: &mut Context<Self>,
    ) {
        let (actions, prof) = box_entity.update(cx, |b, _| (b.take_actions(), b.profile.clone()));
        if actions.is_empty() {
            return;
        }
        let lang = self.active_lang_id();
        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        for act in actions {
            match act {
                BotProfileBoxAction::Close => {
                    self.miniapps.bot_profile_box = None;
                }
                BotProfileBoxAction::OpenApp {
                    bot_username,
                    short_name,
                } => {
                    self.miniapps.bot_profile_box = None;
                    let src_cid = prof
                        .existing_chat_id
                        .filter(|&id| id > 0)
                        .map(|id| id as u64)
                        .or(active_chat);
                    self.launch_miniapp(
                        &bot_username,
                        &short_name,
                        src_cid,
                        MiniAppLaunchMode::MainMiniApp,
                        None,
                        cx,
                    );
                }
                BotProfileBoxAction::OpenMessageChat {
                    bot_id,
                    bot_username,
                    display_name,
                    description,
                    existing_chat_id,
                    has_started_chat,
                } => {
                    self.miniapps.bot_profile_box = None;
                    self.open_bot_message_chat_or_prestart(
                        bot_id,
                        &bot_username,
                        &display_name,
                        &description,
                        existing_chat_id,
                        has_started_chat,
                        cx,
                    );
                }
                BotProfileBoxAction::ToggleMute => {
                    if let Some(cid) = prof.existing_chat_id.filter(|&id| id > 0) {
                        let next_muted = !prof.is_muted;
                        box_entity.update(cx, |b, b_cx| {
                            b.profile.is_muted = next_muted;
                            b_cx.notify();
                        });
                        let key = prof
                            .bot
                            .username
                            .trim()
                            .trim_start_matches('@')
                            .to_ascii_lowercase();
                        if let Some(cached) = self.miniapps.bot_profiles_cache.get_mut(&key) {
                            cached.is_muted = next_muted;
                        }
                        let c = self.client.clone();
                        cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                            let _ = c.set_chat_muted(cid, next_muted, None).await;
                        })
                        .detach();
                        self.enqueue_localized_toast(
                            if next_muted { "已静音此机器人" } else { "已开启机器人通知" },
                            if next_muted { "Bot notifications muted" } else { "Bot notifications enabled" },
                            1800,
                            cx,
                        );
                    } else {
                        self.enqueue_localized_toast(
                            "请先开始对话以设置通知状态",
                            "Start the bot chat first to toggle notifications",
                            2000,
                            cx,
                        );
                    }
                }
                BotProfileBoxAction::AddToGroup => {
                    self.miniapps.bot_profile_box = None;
                    let title = tr_str(lang, "添加到群组", "Add to Group");
                    self.open_modal_dialog(title, ModalDialogKind::NewGroup, 420.0, 380.0, cx);
                }
                BotProfileBoxAction::SendBotHelp | BotProfileBoxAction::OpenBotSettings => {
                    let cmd = if matches!(act, BotProfileBoxAction::SendBotHelp) {
                        "/help"
                    } else {
                        "/settings"
                    };
                    self.miniapps.bot_profile_box = None;
                    self.open_bot_message_chat_or_prestart(
                        prof.bot.bot_user_id.max(0) as u64,
                        &prof.bot.username,
                        &prof.bot.display_name,
                        &prof.bot.description,
                        prof.existing_chat_id,
                        prof.has_started_chat,
                        cx,
                    );
                    if prof.existing_chat_id.is_some_and(|id| id > 0) {
                        self.append_outgoing_text(cmd.to_string(), cx);
                    }
                }
                BotProfileBoxAction::OpenPrivacyPolicy(url) => {
                    self.open_external_url_with_toast(&url, cx);
                }
                BotProfileBoxAction::OpenMiniAppTos => {
                    self.miniapps.miniapp_tos_modal_open = true;
                }
                BotProfileBoxAction::Report => {
                    self.enqueue_localized_toast(
                        "已提交举报，感谢您的反馈",
                        "Report submitted. Thank you for your feedback.",
                        2200,
                        cx,
                    );
                }
                BotProfileBoxAction::ToggleStopAndBlock => {
                    let bot_id = prof.bot.bot_user_id.max(0) as u64;
                    self.toggle_bot_block_status(bot_id, cx);
                }
            }
        }
        cx.notify();
    }
}
