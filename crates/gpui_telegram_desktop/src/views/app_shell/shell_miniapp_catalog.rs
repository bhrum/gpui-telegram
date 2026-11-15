//! Mini-App catalog synchronization, bot chat routing, and Mini-App launcher entry points.

use gpui::{AppContext, AsyncApp, Context, WeakEntity};
use gpui_telegram_client::net::http_client::AddContactHttpRequest;
use gpui_telegram_proto::miniapp_contracts::{
    parse_miniapp_uri, BotRegistryEntry, MiniAppEntry, MiniAppLaunchMode, BOTFATHER_USER_ID,
};

use crate::components::bots::bot_attach_web_view::{peer_types, peer_types_mask_from_strings};
use crate::i18n::tr_str;
use crate::views::app_shell::peer_avatar_color;
use crate::views::app_shell::shell_actions::resolve_active_target_chat_id;
use crate::views::app_shell::shell_state::ShellMiniAppState;
use crate::views::app_shell::shell_truth::{derive_initials, SessionRequestFence};
use crate::views::chat_sidebar::{DialogDeliveryStatus, DialogItemModel, SidebarCatalogAppItem};
use crate::views::composer::attachments_drawer::ComposerAttachMiniApp;
use crate::views::nav_rail::NavDestination;
use crate::views::shell::{ActiveWorkspacePanel, AppShell};

fn session_fence(shell: &AppShell) -> Option<SessionRequestFence> {
    let token = shell.session_sync.session_token.clone()?;
    Some(SessionRequestFence {
        auth_generation: shell.session_sync.auth_generation,
        request_generation: 0,
        token,
        user_id: shell.session_sync.user_id.unwrap_or(0),
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

fn build_sidebar_catalog_items(miniapps: &ShellMiniAppState) -> Vec<SidebarCatalogAppItem> {
    miniapps
        .catalog_miniapps
        .iter()
        .map(|app_entry| {
            let is_inst = app_entry.is_installed
                || miniapps
                    .installed_miniapps
                    .iter()
                    .any(|i| i.app_id == app_entry.app_id);
            let is_mine = app_entry.created_by_nl
                || miniapps
                    .my_bots
                    .iter()
                    .any(|b| b.bot_user_id == app_entry.bot_user_id);
            let icon_initials = if !app_entry.icon_emoji.trim().is_empty() {
                app_entry.icon_emoji.clone()
            } else {
                derive_initials(&app_entry.title)
            };
            SidebarCatalogAppItem {
                bot_id: app_entry.bot_user_id.max(0) as u64,
                bot_username: app_entry.bot_username.clone(),
                title: app_entry.title.clone(),
                short_name: app_entry.short_name.clone(),
                description: app_entry.description.clone(),
                category: app_entry.category.clone(),
                icon_initials,
                avatar_color: peer_avatar_color(app_entry.bot_user_id.max(0) as u64),
                installs_count: app_entry.install_count.max(0) as u64,
                is_installed: is_inst,
                is_mine,
                webview_url: if app_entry.web_url.is_empty() {
                    app_entry.gpui_uri()
                } else {
                    app_entry.web_url.clone()
                },
                companion_chat_id: app_entry
                    .direct_chat_id
                    .filter(|&id| id > 0)
                    .map(|id| id as u64),
            }
        })
        .collect()
}

fn build_attach_mini_apps(miniapps: &ShellMiniAppState) -> Vec<ComposerAttachMiniApp> {
    let mut list: Vec<&MiniAppEntry> = Vec::new();
    for app in &miniapps.installed_miniapps {
        if !list.iter().any(|x| x.app_id == app.app_id) {
            list.push(app);
        }
    }
    for app in &miniapps.catalog_miniapps {
        if app.is_installed && !list.iter().any(|x| x.app_id == app.app_id) {
            list.push(app);
        }
    }
    list.into_iter()
        .map(|app| {
            let mut mask = peer_types_mask_from_strings(&app.peer_types);
            if mask == 0 {
                mask = peer_types::USER
                    | peer_types::BOT
                    | peer_types::GROUP
                    | peer_types::BROADCAST;
            }
            ComposerAttachMiniApp {
                bot_id: app.bot_user_id.max(0) as u64,
                title: app.title.clone(),
                short_name: app.short_name.clone(),
                bot_username: app.bot_username.clone(),
                webview_url: if app.web_url.is_empty() {
                    app.gpui_uri()
                } else {
                    app.web_url.clone()
                },
                peer_types_mask: mask,
                icon_color: peer_avatar_color(app.bot_user_id.max(0) as u64),
            }
        })
        .collect()
}

fn clear_bot_composer_and_inspector(
    shell: &mut AppShell,
    attach_apps: Vec<ComposerAttachMiniApp>,
    peer_mask: u8,
    cx: &mut Context<AppShell>,
) {
    shell.composer.update(cx, |comp, c_cx| {
        comp.set_attach_mini_apps(attach_apps, peer_mask);
        comp.clear_bot_menu(Some(c_cx));
        c_cx.notify();
    });
    shell.inspector.update(cx, |insp, insp_cx| {
        insp.sync_bot_details(false, 0, String::new(), Vec::new(), None, None, false);
        insp_cx.notify();
    });
}

fn activate_chats_workspace(shell: &mut AppShell, cx: &mut Context<AppShell>) {
    shell.active_panel = ActiveWorkspacePanel::Chats;
    shell.last_handled_nav_dest = Some(NavDestination::Chats);
    shell.nav_rail.update(cx, |r, r_cx| {
        r.set_active_destination(NavDestination::Chats);
        r_cx.notify();
    });
    shell.main_menu_drawer.update(cx, |m, m_cx| {
        m.close();
        m_cx.notify();
    });
}

impl AppShell {
    /// Fetches catalog, installed Mini-Apps, attach-menu bots, and owned bots from backend.
    pub fn refresh_miniapps_catalog(&mut self, query: Option<String>, cx: &mut Context<Self>) {
        if !self.is_authenticated {
            return;
        }
        let Some(fence) = session_fence(self) else {
            return;
        };
        let client = self.client.clone();
        let q_trimmed = query
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string);
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let catalog_res = client.list_mini_apps(q_trimmed.as_deref(), None).await;
            let installed_res = client.list_installed_mini_apps().await;
            let attach_res = client.list_attach_menu_bots().await;
            let my_bots_res = client.list_my_bots().await;
            let _ = weak.update(app, |shell, cx| {
                if !fence_matches(&fence, shell) {
                    return;
                }
                if let Ok(cat) = catalog_res {
                    shell.miniapps.catalog_miniapps = cat;
                }
                if let Ok(inst) = installed_res {
                    shell.miniapps.installed_miniapps = inst;
                }
                if let Ok(att) = attach_res {
                    shell.miniapps.attach_menu_bots = att;
                }
                if let Ok(mine) = my_bots_res {
                    shell.miniapps.my_bots = mine;
                }
                let mapped = build_sidebar_catalog_items(&shell.miniapps);
                shell.sidebar.update(cx, |sb, sb_cx| {
                    sb.set_catalog_mini_apps(mapped);
                    sb_cx.notify();
                });
                shell.sync_composer_and_inspector_miniapp_state(cx);
                cx.notify();
            });
        })
        .detach();
    }

    /// Synchronizes attach-menu apps, bot menu button, and bot details to composer and inspector.
    pub fn sync_composer_and_inspector_miniapp_state(&mut self, cx: &mut Context<Self>) {
        let lang = self.active_lang_id().to_string();
        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        let attach_apps = build_attach_mini_apps(&self.miniapps);

        let Some(cid) = active_chat else {
            clear_bot_composer_and_inspector(
                self,
                attach_apps,
                peer_types::USER | peer_types::BOT,
                cx,
            );
            return;
        };

        let rc_opt = self.session_sync.rich_chats_by_id.get(&cid).cloned();
        let sb_is_bot = self.sidebar.read(cx).bot_ids.contains(&cid);
        let peer_mask = if let Some(ref rc) = rc_opt {
            if rc.is_channel {
                peer_types::BROADCAST
            } else if rc.is_group || rc.chat_type > 0 {
                peer_types::GROUP
            } else if rc.is_bot || sb_is_bot {
                peer_types::BOT
            } else {
                peer_types::USER
            }
        } else if sb_is_bot {
            peer_types::BOT
        } else {
            peer_types::USER
        };

        let is_bot_chat = rc_opt.as_ref().is_some_and(|rc| rc.is_bot) || sb_is_bot;
        if !is_bot_chat {
            clear_bot_composer_and_inspector(self, attach_apps, peer_mask, cx);
            return;
        }

        let peer_uid = rc_opt
            .as_ref()
            .and_then(|rc| rc.peer_user_id)
            .or_else(|| self.session_sync.chat_peer_user_ids.get(&cid).copied())
            .unwrap_or(0);
        let rc_username = rc_opt
            .as_ref()
            .and_then(|rc| rc.peer_username.as_deref().or(rc.username.as_deref()))
            .unwrap_or("")
            .trim()
            .trim_start_matches('@')
            .to_string();

        let bot_reg: Option<&BotRegistryEntry> = self
            .miniapps
            .attach_menu_bots
            .iter()
            .chain(self.miniapps.my_bots.iter())
            .find(|b| {
                (peer_uid > 0 && b.bot_user_id == peer_uid as i64)
                    || (!rc_username.is_empty() && b.username.eq_ignore_ascii_case(&rc_username))
            });
        let matched_app: Option<&MiniAppEntry> = self
            .miniapps
            .installed_miniapps
            .iter()
            .chain(self.miniapps.catalog_miniapps.iter())
            .find(|a| {
                a.direct_chat_id == Some(cid as i64)
                    || (peer_uid > 0 && a.bot_user_id == peer_uid as i64)
                    || (!rc_username.is_empty()
                        && a.bot_username.eq_ignore_ascii_case(&rc_username))
            });

        let bot_username = if !rc_username.is_empty() {
            rc_username.clone()
        } else if let Some(b) = bot_reg {
            b.username.clone()
        } else if let Some(a) = matched_app {
            a.bot_username.clone()
        } else {
            String::new()
        };
        let bot_id = if peer_uid > 0 {
            peer_uid
        } else if let Some(b) = bot_reg {
            b.bot_user_id.max(0) as u64
        } else if let Some(a) = matched_app {
            a.bot_user_id.max(0) as u64
        } else {
            cid
        };

        let menu_url = rc_opt
            .as_ref()
            .and_then(|rc| rc.menu_button_url.clone())
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                bot_reg
                    .map(|b| b.menu_button_url.clone())
                    .filter(|s| !s.trim().is_empty())
            })
            .or_else(|| matched_app.map(|a| a.gpui_uri()));
        let menu_text = rc_opt
            .as_ref()
            .and_then(|rc| rc.menu_button_text.clone())
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                bot_reg
                    .map(|b| b.menu_button_text.clone())
                    .filter(|s| !s.trim().is_empty())
            })
            .or_else(|| {
                menu_url
                    .as_ref()
                    .map(|_| tr_str(&lang, "打开小程序", "Open App").to_string())
            });

        let has_miniapp = menu_url.is_some()
            || rc_opt.as_ref().is_some_and(|rc| rc.has_mini_app)
            || matched_app.is_some();

        self.composer.update(cx, |comp, c_cx| {
            comp.set_attach_mini_apps(attach_apps, peer_mask);
            if has_miniapp {
                let btn_label = menu_text
                    .clone()
                    .unwrap_or_else(|| tr_str(&lang, "打开小程序", "Open App").to_string());
                let btn_url = menu_url.clone().unwrap_or_else(|| {
                    matched_app
                        .map(|a| a.gpui_uri())
                        .unwrap_or_else(|| format!("gpui-miniapp://{}/app", bot_username))
                });
                comp.sync_bot_menu_with_cx(bot_id, bot_username.clone(), btn_label, btn_url, c_cx);
            } else {
                comp.clear_bot_menu(Some(c_cx));
            }
            c_cx.notify();
        });

        let persona_prompt = bot_reg
            .map(|b| b.ai_persona.clone())
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                rc_opt
                    .as_ref()
                    .and_then(|rc| rc.peer_bio.clone())
                    .filter(|s| !s.trim().is_empty())
            })
            .unwrap_or_default();
        let commands: Vec<(String, String)> = bot_reg
            .map(|b| {
                b.commands
                    .iter()
                    .map(|c| (c.command.clone(), c.description.clone()))
                    .collect()
            })
            .unwrap_or_else(|| {
                if bot_username.eq_ignore_ascii_case("BotFather") {
                    vec![
                        ("/newbot".into(), tr_str(&lang, "创建新机器人", "Create a new bot").into()),
                        ("/newapp".into(), tr_str(&lang, "创建小程序", "Create a Mini App").into()),
                        ("/mybots".into(), tr_str(&lang, "管理我的机器人", "Manage my bots").into()),
                        ("/myapps".into(), tr_str(&lang, "管理我的小程序", "Manage my apps").into()),
                    ]
                } else {
                    Vec::new()
                }
            });
        let in_attach = matched_app.is_some_and(|a| {
            a.is_installed
                || self
                    .miniapps
                    .installed_miniapps
                    .iter()
                    .any(|i| i.app_id == a.app_id)
        });
        let app_title = matched_app.map(|a| a.title.clone()).or(menu_text);
        self.inspector.update(cx, |insp, insp_cx| {
            insp.sync_bot_details(
                true,
                bot_id,
                persona_prompt,
                commands,
                app_title,
                menu_url,
                in_attach,
            );
            insp_cx.notify();
        });
    }

    /// Selects an existing dialog or inserts a provisional row before loading the stage.
    pub fn open_or_select_chat(
        &mut self,
        chat_id: u64,
        title_hint: Option<String>,
        is_bot: bool,
        cx: &mut Context<Self>,
    ) {
        let mut inserted_new = false;
        let sel_rev = self.sidebar.update(cx, |sb, sb_cx| {
            if !sb.dialogs.iter().any(|d| d.id == chat_id) {
                let peer_name = title_hint
                    .clone()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| format!("Chat #{chat_id}"));
                let init = derive_initials(&peer_name);
                sb.dialogs.insert(
                    0,
                    DialogItemModel {
                        id: chat_id,
                        peer_name,
                        avatar_initials: if init.is_empty() { "TG".into() } else { init },
                        avatar_color: peer_avatar_color(chat_id),
                        last_message: String::new(),
                        timestamp: "刚刚".to_string(),
                        unread_count: 0,
                        is_muted: false,
                        is_pinned: false,
                        is_online: is_bot,
                        typing_indicator: None,
                        delivery_status: DialogDeliveryStatus::Read,
                        folder: if is_bot {
                            "Bots".to_string()
                        } else {
                            "All".to_string()
                        },
                    },
                );
                inserted_new = true;
            }
            if is_bot {
                sb.bot_ids.insert(chat_id);
            }
            sb.active_folder = "All".to_string();
            sb.search_focused = false;
            sb.is_account_popover_open = false;
            sb.is_plus_menu_open = false;
            sb.select_dialog_from_user(chat_id);
            sb_cx.notify();
            sb.selection_revision
        });
        self.overlays.last_sidebar_selection_revision = sel_rev;
        activate_chats_workspace(self, cx);
        self.sync_selected_chat_inspector_and_stage(chat_id, cx);
        if inserted_new {
            self.refresh_chats(cx);
        }
        cx.notify();
    }

    /// Opens the companion direct chat with `@BotFather`.
    pub fn open_botfather_chat(&mut self, cx: &mut Context<Self>) {
        self.open_bot_chat_by_username("BotFather", cx);
    }

    /// Opens or resolves a bot DM chat by username and focuses it in the workspace.
    pub fn open_bot_chat_by_username(&mut self, bot_username: &str, cx: &mut Context<Self>) {
        self.main_menu_drawer.update(cx, |m, m_cx| {
            m.close();
            m_cx.notify();
        });
        self.sidebar.update(cx, |s, s_cx| {
            s.is_account_popover_open = false;
            s.is_plus_menu_open = false;
            s.search_focused = false;
            s_cx.notify();
        });
        let target_un = bot_username.trim().trim_start_matches('@').to_string();
        if target_un.is_empty() {
            return;
        }
        let existing_cid = self
            .session_sync
            .rich_chats_by_id
            .iter()
            .find(|(_, rc)| {
                (target_un.eq_ignore_ascii_case("BotFather")
                    && rc.peer_user_id == Some(BOTFATHER_USER_ID as u64))
                    || rc
                        .peer_username
                        .as_deref()
                        .is_some_and(|u| u.trim_start_matches('@').eq_ignore_ascii_case(&target_un))
            })
            .map(|(cid, _)| *cid);
        if let Some(cid) = existing_cid {
            self.open_or_select_chat(cid, Some(target_un), true, cx);
            return;
        }
        let Some(fence) = session_fence(self) else {
            return;
        };
        let token = fence.token.clone();
        let client = self.client.clone();
        let req = AddContactHttpRequest {
            first_name: target_un.clone(),
            last_name: None,
            username: Some(target_un.clone()),
            phone: None,
            is_bot: true,
            bot_system_prompt: None,
        };
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.add_contact(&token, req).await;
            let _ = weak.update(app, |shell, cx| {
                if !fence_matches(&fence, shell) {
                    return;
                }
                if let Ok(resp) = res {
                    shell.open_or_select_chat(resp.chat_id, Some(resp.title), true, cx);
                    shell.refresh_chats(cx);
                }
            });
        })
        .detach();
    }

    /// Switches sidebar to the Mini-Apps catalog view and refreshes available apps.
    pub fn open_miniapps_center(&mut self, cx: &mut Context<Self>) {
        activate_chats_workspace(self, cx);
        self.sidebar.update(cx, |sb, sb_cx| {
            sb.is_account_popover_open = false;
            sb.is_plus_menu_open = false;
            sb.open_apps_center();
            sb_cx.notify();
        });
        self.refresh_miniapps_catalog(None, cx);
        cx.notify();
    }

    /// Launches a Mini-App overlay for the specified bot username and short name.
    pub fn launch_miniapp(
        &mut self,
        bot_username: &str,
        short_name: &str,
        source_chat_id: Option<u64>,
        launch_mode: MiniAppLaunchMode,
        start_param: Option<String>,
        cx: &mut Context<Self>,
    ) {
        let bot_un = bot_username.trim().trim_start_matches('@').to_string();
        let short = short_name.trim().to_string();
        if bot_un.is_empty() || short.is_empty() {
            return;
        }
        let Some(fence) = session_fence(self) else {
            return;
        };
        let ov = self.ensure_miniapp_overlay(cx);
        self.miniapps.is_miniapp_minimized = false;
        let title_hint = self
            .miniapps
            .catalog_miniapps
            .iter()
            .chain(self.miniapps.installed_miniapps.iter())
            .find(|a| {
                a.bot_username.eq_ignore_ascii_case(&bot_un)
                    && a.short_name.eq_ignore_ascii_case(&short)
            })
            .map(|a| a.title.clone())
            .unwrap_or_else(|| short.clone());
        ov.update(cx, |w, w_cx| {
            w.is_open = true;
            w.is_loading = true;
            w.disclaimer_accepted = true;
            w.bot_username = bot_un.clone();
            w.source_chat_id = source_chat_id.map(|id| id as i64);
            w.ui_schema.title = title_hint;
            w_cx.notify();
        });
        let client = self.client.clone();
        let ov_weak = ov.downgrade();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client
                .launch_mini_app(
                    &bot_un,
                    &short,
                    source_chat_id.map(|id| id as i64),
                    launch_mode,
                    start_param.as_deref(),
                )
                .await;
            let _ = weak.update(app, |shell, cx| {
                if !fence_matches(&fence, shell) {
                    return;
                }
                match res {
                    Ok(ctx) => {
                        let _ = ov_weak.update(cx, |w, w_cx| {
                            w.load_context(ctx);
                            w_cx.notify();
                        });
                    }
                    Err(e) => {
                        let _ = ov_weak.update(cx, |w, w_cx| {
                            w.is_loading = false;
                            w.close();
                            w_cx.notify();
                        });
                        shell.miniapps.miniapp_overlay = None;
                        shell.window_runtime.enqueue_toast(
                            format!("Mini App launch failed: {}", e),
                            2600,
                        );
                    }
                }
                cx.notify();
            });
        })
        .detach();
        cx.notify();
    }

    /// Parses a Mini-App URI or resolves the active chat's associated Mini-App to launch it.
    pub fn launch_miniapp_from_uri(
        &mut self,
        raw_uri: &str,
        source_chat_id: Option<u64>,
        launch_mode: MiniAppLaunchMode,
        cx: &mut Context<Self>,
    ) -> bool {
        if let Some((bot_un, short_name, start_param)) = parse_miniapp_uri(raw_uri) {
            self.launch_miniapp(
                &bot_un,
                &short_name,
                source_chat_id,
                launch_mode,
                start_param,
                cx,
            );
            return true;
        }
        if let Some(cid) = source_chat_id {
            let peer_uid = self
                .session_sync
                .rich_chats_by_id
                .get(&cid)
                .and_then(|rc| rc.peer_user_id)
                .or_else(|| self.session_sync.chat_peer_user_ids.get(&cid).copied());
            if let Some(app) = self
                .miniapps
                .installed_miniapps
                .iter()
                .chain(self.miniapps.catalog_miniapps.iter())
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
                    launch_mode,
                    None,
                    cx,
                );
                return true;
            }
        }
        false
    }

    /// Launches a Mini-App from the composer attach menu or bot menu button.
    pub fn launch_miniapp_from_composer(
        &mut self,
        bot_id: u64,
        bot_username: &str,
        short_name: &str,
        url: &str,
        cx: &mut Context<Self>,
    ) {
        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        let mode = if !short_name.trim().is_empty() && url.contains("gpui-miniapp://") {
            MiniAppLaunchMode::MenuButton
        } else {
            MiniAppLaunchMode::AttachMenu
        };
        if !bot_username.trim().is_empty() && !short_name.trim().is_empty() {
            self.launch_miniapp(bot_username, short_name, active_chat, mode, None, cx);
            return;
        }
        if self.launch_miniapp_from_uri(url, active_chat, mode, cx) {
            return;
        }
        let target_un = bot_username.trim().trim_start_matches('@');
        if let Some(app) = self
            .miniapps
            .catalog_miniapps
            .iter()
            .chain(self.miniapps.installed_miniapps.iter())
            .find(|a| {
                (bot_id > 0 && a.bot_user_id == bot_id as i64)
                    || (!target_un.is_empty() && a.bot_username.eq_ignore_ascii_case(target_un))
            })
            .cloned()
        {
            self.launch_miniapp(
                &app.bot_username,
                &app.short_name,
                active_chat,
                mode,
                None,
                cx,
            );
        }
    }

    /// Installs (if needed) and launches a Mini-App selected from the sidebar catalog.
    pub fn install_and_launch_miniapp(
        &mut self,
        item: SidebarCatalogAppItem,
        cx: &mut Context<Self>,
    ) {
        if item.is_installed {
            self.launch_miniapp(
                &item.bot_username,
                &item.short_name,
                item.companion_chat_id,
                MiniAppLaunchMode::DirectLink,
                None,
                cx,
            );
            return;
        }
        let Some(fence) = session_fence(self) else {
            return;
        };
        let client = self.client.clone();
        let bot_un = item.bot_username.clone();
        let short = item.short_name.clone();
        let fallback_cid = item.companion_chat_id;
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.install_mini_app(&bot_un, &short, true).await;
            let _ = weak.update(app, |shell, cx| {
                if !fence_matches(&fence, shell) {
                    return;
                }
                match res {
                    Ok(r) => {
                        if let Some(ref toast) = r.toast {
                            shell.window_runtime.enqueue_toast(toast, 2200);
                        }
                        let cid = r
                            .companion_chat_id
                            .filter(|&id| id > 0)
                            .map(|id| id as u64)
                            .or(fallback_cid);
                        shell.refresh_chats(cx);
                        shell.refresh_miniapps_catalog(None, cx);
                        shell.launch_miniapp(
                            &bot_un,
                            &short,
                            cid,
                            MiniAppLaunchMode::DirectLink,
                            None,
                            cx,
                        );
                    }
                    Err(e) => {
                        shell
                            .window_runtime
                            .enqueue_toast(format!("Install failed: {}", e), 2400);
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }
}
