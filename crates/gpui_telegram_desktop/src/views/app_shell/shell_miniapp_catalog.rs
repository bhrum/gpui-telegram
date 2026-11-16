//! Mini-App catalog synchronization, bot chat routing, and Mini-App launcher entry points.

use std::collections::HashMap;

use gpui::{AppContext, AsyncApp, Context, WeakEntity};
use gpui_telegram_client::net::http_client::AddContactHttpRequest;
use gpui_telegram_proto::miniapp_contracts::{
    parse_miniapp_uri, AcceptMiniAppTosRequest, BotPublicProfileDto, BotRegistryEntry,
    FabushiBotSummaryDto, MiniAppEntry, MiniAppLaunchMode, BOTFATHER_USER_ID,
};

use crate::components::bots::bot_attach_web_view::{peer_types, peer_types_mask_from_strings};
use crate::i18n::tr_str;
use crate::views::app_shell::peer_avatar_color;
use crate::views::app_shell::shell_actions::resolve_active_target_chat_id;
use crate::views::app_shell::shell_state::{PendingMiniAppConfirm, ShellMiniAppState};
use crate::views::app_shell::shell_truth::{derive_initials, SessionRequestFence};
use crate::views::canonical_composer::BotSlashCommandItem;
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

fn mini_app_entry_to_sidebar_item(
    app_entry: &MiniAppEntry,
    miniapps: &ShellMiniAppState,
) -> SidebarCatalogAppItem {
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
    let clean_un = app_entry
        .bot_username
        .trim()
        .trim_start_matches('@')
        .to_ascii_lowercase();
    let is_verified = miniapps
        .bot_profiles_cache
        .get(&clean_un)
        .is_some_and(|p| p.bot.is_official)
        || miniapps
            .attach_menu_bots
            .iter()
            .chain(miniapps.my_bots.iter())
            .any(|b| b.bot_user_id == app_entry.bot_user_id && b.is_official);
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
        mau_count: app_entry.mau_count.max(0) as u64,
        is_verified,
    }
}

fn build_sidebar_catalog_items(miniapps: &ShellMiniAppState) -> Vec<SidebarCatalogAppItem> {
    miniapps
        .catalog_miniapps
        .iter()
        .map(|app_entry| mini_app_entry_to_sidebar_item(app_entry, miniapps))
        .collect()
}

fn build_sidebar_recent_items(miniapps: &ShellMiniAppState) -> Vec<SidebarCatalogAppItem> {
    miniapps
        .recent_mini_apps
        .iter()
        .map(|app_entry| mini_app_entry_to_sidebar_item(app_entry, miniapps))
        .collect()
}

fn build_bot_main_app_uris(shell: &AppShell) -> HashMap<u64, String> {
    let mut map = HashMap::new();
    for (&cid, rc) in &shell.session_sync.rich_chats_by_id {
        if !rc.is_bot || rc.is_group || rc.is_channel || matches!(rc.chat_type, 1 | 2 | 3) {
            continue;
        }
        let peer_uid = rc
            .peer_user_id
            .or_else(|| shell.session_sync.chat_peer_user_ids.get(&cid).copied());
        let rc_un = rc
            .peer_username
            .as_deref()
            .or(rc.username.as_deref())
            .unwrap_or("")
            .trim()
            .trim_start_matches('@');
        if rc_un.eq_ignore_ascii_case("BotFather")
            || peer_uid == Some(BOTFATHER_USER_ID as u64)
        {
            continue;
        }
        let prof_key = rc_un.to_ascii_lowercase();
        let cached_prof = if prof_key.is_empty() {
            None
        } else {
            shell.miniapps.bot_profiles_cache.get(&prof_key)
        };
        let bot_reg = shell
            .miniapps
            .attach_menu_bots
            .iter()
            .chain(shell.miniapps.my_bots.iter())
            .find(|b| {
                peer_uid.is_some_and(|p| b.bot_user_id == p as i64)
                    || (!rc_un.is_empty() && b.username.eq_ignore_ascii_case(rc_un))
            });
        let main_app = cached_prof.and_then(|p| p.main_app.as_ref()).or_else(|| {
            shell
                .miniapps
                .recent_mini_apps
                .iter()
                .chain(shell.miniapps.installed_miniapps.iter())
                .chain(shell.miniapps.catalog_miniapps.iter())
                .find(|a| {
                    (a.is_main_app || a.direct_chat_id == Some(cid as i64))
                        && (peer_uid.is_some_and(|p| a.bot_user_id == p as i64)
                            || (!rc_un.is_empty() && a.bot_username.eq_ignore_ascii_case(rc_un)))
                })
        });
        let uri = if let Some(app) = main_app {
            Some(app.gpui_uri())
        } else if let Some(reg) = bot_reg {
            if reg.has_main_app && !reg.main_app_short_name.trim().is_empty() {
                Some(format!(
                    "gpui-miniapp://{}/{}",
                    reg.username.trim_start_matches('@'),
                    reg.main_app_short_name.trim()
                ))
            } else if reg.menu_button_type.eq_ignore_ascii_case("web_app")
                && !reg.menu_button_url.trim().is_empty()
            {
                Some(reg.menu_button_url.clone())
            } else {
                None
            }
        } else if let Some(url) = rc
            .menu_button_url
            .as_deref()
            .filter(|u| !u.trim().is_empty() && rc.has_mini_app)
        {
            Some(url.to_string())
        } else {
            None
        };
        if let Some(u) = uri {
            map.insert(cid, u);
        }
    }
    map
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
        comp.clear_bot_slash_commands();
        comp.set_hermes_slash_enabled(false);
        c_cx.notify();
    });
    shell.inspector.update(cx, |insp, insp_cx| {
        insp.sync_bot_details(false, 0, String::new(), Vec::new(), None, None, false);
        insp.sync_bot_profile_extra(0, false, String::new(), false, false, false, false, false);
        insp_cx.notify();
    });
    shell.stage.update(cx, |st, st_cx| {
        st.set_bot_prestart_state(None, false, false);
        st_cx.notify();
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
            let recent_res = client.list_recent_mini_apps().await;
            let attach_res = client.list_attach_menu_bots().await;
            let my_bots_res = client.list_my_bots().await;
            let fabushi_res = client.list_fabushi_bots().await;
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
                if let Ok(rec) = recent_res {
                    shell.miniapps.recent_mini_apps = rec;
                }
                if let Ok(att) = attach_res {
                    shell.miniapps.attach_menu_bots = att;
                }
                if let Ok(mine) = my_bots_res {
                    shell.miniapps.my_bots = mine;
                }
                if let Ok(fb) = fabushi_res {
                    shell.miniapps.fabushi_bots = fb;
                }
                let mapped = build_sidebar_catalog_items(&shell.miniapps);
                let mapped_recent = build_sidebar_recent_items(&shell.miniapps);
                let main_app_uris = build_bot_main_app_uris(shell);
                shell.sidebar.update(cx, |sb, sb_cx| {
                    sb.set_catalog_mini_apps(mapped);
                    sb.set_recent_mini_apps(mapped_recent);
                    sb.set_bot_main_app_uris(main_app_uris);
                    sb_cx.notify();
                });
                shell.sync_composer_and_inspector_miniapp_state(cx);
                shell.sync_fabushi_bots_to_views(cx);
                cx.notify();
            });
        })
        .detach();
    }

    /// Resolves the direct Fabushi Bot user id for the currently selected chat (returns `None` in groups/non-bot DMs).
    pub fn active_chat_bot_id(&self, cx: &gpui::App) -> Option<u64> {
        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        )?;
        if let Some(rc) = self.session_sync.rich_chats_by_id.get(&active_chat) {
            if rc.is_group || rc.is_channel || matches!(rc.chat_type, 1 | 2 | 3) {
                return None;
            }
            if rc.is_bot {
                if let Some(pid) = rc.peer_user_id.filter(|&p| p > 0) {
                    return Some(pid);
                }
            }
        }
        if let Some(fb) = self
            .miniapps
            .fabushi_bots
            .iter()
            .find(|b| b.companion_chat_id == Some(active_chat as i64))
        {
            if fb.bot_user_id > 0 {
                return Some(fb.bot_user_id as u64);
            }
        }
        if self.sidebar.read(cx).bot_ids.contains(&active_chat) {
            return self
                .session_sync
                .chat_peer_user_ids
                .get(&active_chat)
                .copied()
                .filter(|&p| p > 0);
        }
        None
    }

    /// Propagates cached Fabushi Bot geometric visuals and active Stage/Inspector metadata.
    pub fn sync_fabushi_bots_to_views(&mut self, cx: &mut Context<Self>) {
        let mut visuals: std::collections::HashMap<u64, (String, u32)> =
            std::collections::HashMap::new();
        let mut grok_visuals: std::collections::HashMap<
            u64,
            crate::views::chat_sidebar::BotGrokAvatarVisual,
        > = std::collections::HashMap::new();
        let mut companion_cids: Vec<u64> = Vec::new();
        for fb in &self.miniapps.fabushi_bots {
            let color = crate::components::avatar::parse_fabushi_hex_color(&fb.avatar_color)
                .unwrap_or(self.theme_state.palette.accent);
            let pair = (fb.avatar_shape.clone(), color);
            let grok = crate::views::chat_sidebar::BotGrokAvatarVisual {
                agent_id: fb.agent_id.clone(),
                shape: fb.avatar_shape.clone(),
                color: fb.avatar_color.clone(),
                execution_state: fb.execution_state.clone(),
            };
            if let Some(cid) = fb.companion_chat_id.filter(|&c| c > 0) {
                let cid_u64 = cid as u64;
                visuals.insert(cid_u64, pair.clone());
                grok_visuals.insert(cid_u64, grok.clone());
                companion_cids.push(cid_u64);
            }
            if fb.bot_user_id > 0 {
                let bot_uid = fb.bot_user_id as u64;
                for (&cid, &peer_uid) in &self.session_sync.chat_peer_user_ids {
                    if peer_uid == bot_uid {
                        let is_direct = self
                            .session_sync
                            .rich_chats_by_id
                            .get(&cid)
                            .map(|rc| !rc.is_group && !rc.is_channel && !matches!(rc.chat_type, 1 | 2 | 3))
                            .unwrap_or(true);
                        if is_direct {
                            visuals.insert(cid, pair.clone());
                            grok_visuals.insert(cid, grok.clone());
                            companion_cids.push(cid);
                        }
                    }
                }
                for (&cid, rc) in &self.session_sync.rich_chats_by_id {
                    if rc.is_bot && rc.peer_user_id == Some(bot_uid) {
                        visuals.insert(cid, pair.clone());
                        grok_visuals.insert(cid, grok.clone());
                        companion_cids.push(cid);
                    }
                }
            }
        }
        self.sidebar.update(cx, |sb, sb_cx| {
            sb.bot_fabushi_visuals = visuals;
            sb.bot_grok_avatars = grok_visuals;
            for cid in companion_cids {
                sb.bot_ids.insert(cid);
            }
            sb_cx.notify();
        });

        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        let active_bot_uid = self.active_chat_bot_id(cx);
        let active_fab: Option<FabushiBotSummaryDto> = self
            .miniapps
            .fabushi_bots
            .iter()
            .find(|fb| {
                (active_chat.is_some() && fb.companion_chat_id == active_chat.map(|c| c as i64))
                    || (active_bot_uid.is_some()
                        && fb.bot_user_id > 0
                        && Some(fb.bot_user_id as u64) == active_bot_uid)
            })
            .cloned();
        self.stage.update(cx, |st, st_cx| {
            st.set_active_fabushi_bot(active_fab.clone());
            st_cx.notify();
        });
        self.inspector.update(cx, |insp, insp_cx| {
            insp.set_active_fabushi_bot(active_fab);
            insp_cx.notify();
        });
    }

    /// Synchronizes attach-menu apps, per-bot `/` commands, bot menu button, Stage pre-start card,
    /// and Inspector extra fields from cached bot registry and `BotPublicProfileDto`.
    pub fn sync_composer_and_inspector_miniapp_state(&mut self, cx: &mut Context<Self>) {
        let lang = self.active_lang_id().to_string();
        let active_chat = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        );
        let attach_apps = build_attach_mini_apps(&self.miniapps);
        let prestart_opt = if active_chat.is_none() {
            self.miniapps.pending_prestart_bot.clone()
        } else {
            None
        };

        if active_chat.is_none() && prestart_opt.is_none() {
            clear_bot_composer_and_inspector(
                self,
                attach_apps,
                peer_types::USER | peer_types::BOT,
                cx,
            );
            return;
        }

        let (peer_mask, rc_opt, sb_is_bot, peer_uid, rc_username) = if let Some(cid) = active_chat {
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
            (peer_mask, rc_opt, sb_is_bot, peer_uid, rc_username)
        } else if let Some(ref pre) = prestart_opt {
            (
                peer_types::BOT,
                None,
                true,
                pre.bot_id,
                pre.bot_username.trim().trim_start_matches('@').to_string(),
            )
        } else {
            return;
        };
        let _ = sb_is_bot;

        let cached_profile: Option<&BotPublicProfileDto> = if !rc_username.is_empty() {
            self.miniapps
                .bot_profiles_cache
                .get(&rc_username.to_ascii_lowercase())
        } else {
            None
        }
        .or_else(|| {
            if peer_uid > 0 {
                self.miniapps
                    .bot_profiles_cache
                    .values()
                    .find(|p| p.bot.bot_user_id == peer_uid as i64)
            } else {
                None
            }
        });

        let bot_reg: Option<&BotRegistryEntry> = self
            .miniapps
            .attach_menu_bots
            .iter()
            .chain(self.miniapps.my_bots.iter())
            .find(|b| {
                (peer_uid > 0 && b.bot_user_id == peer_uid as i64)
                    || (!rc_username.is_empty() && b.username.eq_ignore_ascii_case(&rc_username))
            });
        let bot_reg_or_prof: Option<&BotRegistryEntry> =
            cached_profile.map(|p| &p.bot).or(bot_reg);

        let matched_app: Option<&MiniAppEntry> = cached_profile
            .and_then(|p| p.main_app.as_ref().or_else(|| p.apps.first()))
            .or_else(|| {
                self.miniapps
                    .installed_miniapps
                    .iter()
                    .chain(self.miniapps.catalog_miniapps.iter())
                    .chain(self.miniapps.recent_mini_apps.iter())
                    .find(|a| {
                        active_chat.is_some_and(|cid| a.direct_chat_id == Some(cid as i64))
                            || (peer_uid > 0 && a.bot_user_id == peer_uid as i64)
                            || (!rc_username.is_empty()
                                && a.bot_username.eq_ignore_ascii_case(&rc_username))
                    })
            });

        let bot_username = if !rc_username.is_empty() {
            rc_username.clone()
        } else if let Some(b) = bot_reg_or_prof {
            b.username.clone()
        } else if let Some(a) = matched_app {
            a.bot_username.clone()
        } else {
            String::new()
        };
        let bot_id = if peer_uid > 0 {
            peer_uid
        } else if let Some(b) = bot_reg_or_prof {
            b.bot_user_id.max(0) as u64
        } else if let Some(a) = matched_app {
            a.bot_user_id.max(0) as u64
        } else {
            active_chat.unwrap_or(0)
        };
        let bot_display_name = prestart_opt
            .as_ref()
            .map(|p| p.display_name.clone())
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                bot_reg_or_prof
                    .map(|b| b.display_name.clone())
                    .filter(|s| !s.trim().is_empty())
            })
            .or_else(|| {
                rc_opt
                    .as_ref()
                    .and_then(|rc| rc.peer_display_name.clone().or_else(|| Some(rc.title.clone())))
                    .filter(|s| !s.trim().is_empty())
            })
            .unwrap_or_else(|| bot_username.clone());

        let is_botfather = bot_username.eq_ignore_ascii_case("BotFather")
            || bot_id == BOTFATHER_USER_ID as u64;
        let viewer_uid = self.session_sync.user_id;
        let is_viewer_owner_fabushi = !is_botfather
            && (self.miniapps.fabushi_bots.iter().any(|fb| {
                (bot_id > 0 && fb.bot_user_id == bot_id as i64)
                    || (!bot_username.is_empty()
                        && fb.username.eq_ignore_ascii_case(&bot_username))
                    || active_chat.is_some_and(|cid| fb.companion_chat_id == Some(cid as i64))
            }) || bot_reg_or_prof.is_some_and(|b| {
                b.is_fabushi_bot && viewer_uid.is_some_and(|u| b.owner_user_id == u as i64)
            }));
        let is_viewer_owner = !is_botfather
            && (is_viewer_owner_fabushi
                || bot_reg_or_prof
                    .is_some_and(|b| viewer_uid.is_some_and(|u| b.owner_user_id == u as i64)));

        // Strict menu button rule: ONLY activate when menu_button_type == "web_app" or explicit URL is set.
        let menu_type = bot_reg_or_prof
            .map(|b| b.menu_button_type.as_str())
            .unwrap_or("");
        let explicit_menu_url = bot_reg_or_prof
            .map(|b| b.menu_button_url.as_str())
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                rc_opt
                    .as_ref()
                    .and_then(|rc| rc.menu_button_url.as_deref())
                    .filter(|s| !s.trim().is_empty())
            })
            .map(str::to_string);
        let has_web_app_menu =
            menu_type.eq_ignore_ascii_case("web_app") || explicit_menu_url.is_some();
        let exact_menu_text = bot_reg_or_prof
            .map(|b| b.menu_button_text.as_str())
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                rc_opt
                    .as_ref()
                    .and_then(|rc| rc.menu_button_text.as_deref())
                    .filter(|s| !s.trim().is_empty())
            })
            .map(str::to_string);

        // Strict per-bot commands list.
        let commands: Vec<(String, String)> = bot_reg_or_prof
            .map(|b| {
                b.commands
                    .iter()
                    .map(|c| (c.command.clone(), c.description.clone()))
                    .collect()
            })
            .filter(|v: &Vec<(String, String)>| !v.is_empty())
            .unwrap_or_else(|| {
                if is_botfather {
                    vec![
                        ("newbot".into(), tr_str(&lang, "创建新机器人", "Create a new bot").into()),
                        ("newapp".into(), tr_str(&lang, "创建小程序", "Create a Mini App").into()),
                        ("mybots".into(), tr_str(&lang, "管理我的机器人", "Manage my bots").into()),
                        ("myapps".into(), tr_str(&lang, "管理我的小程序", "Manage my apps").into()),
                    ]
                } else {
                    Vec::new()
                }
            });
        let bot_initials = {
            let i = derive_initials(&bot_display_name);
            if i.is_empty() {
                "BT".to_string()
            } else {
                i
            }
        };
        let bot_color = peer_avatar_color(if bot_id > 0 {
            bot_id
        } else {
            active_chat.unwrap_or(1)
        });
        let slash_items: Vec<BotSlashCommandItem> = commands
            .iter()
            .filter_map(|(cmd, desc)| {
                let clean = cmd.trim().trim_start_matches('/').to_string();
                if clean.is_empty() {
                    None
                } else {
                    Some(BotSlashCommandItem {
                        command: clean,
                        description: desc.clone(),
                        bot_username: bot_username.clone(),
                        avatar_initials: bot_initials.clone(),
                        avatar_color: bot_color,
                    })
                }
            })
            .collect();

        self.composer.update(cx, |comp, c_cx| {
            comp.set_attach_mini_apps(attach_apps, peer_mask);
            comp.set_bot_slash_commands(slash_items);
            comp.set_hermes_slash_enabled(is_viewer_owner_fabushi);
            if has_web_app_menu {
                let btn_url = explicit_menu_url
                    .clone()
                    .or_else(|| {
                        cached_profile
                            .and_then(|p| p.main_app.as_ref())
                            .map(|a| a.gpui_uri())
                    })
                    .or_else(|| matched_app.map(|a| a.gpui_uri()))
                    .unwrap_or_else(|| format!("gpui-miniapp://{bot_username}/app"));
                let btn_label = exact_menu_text
                    .clone()
                    .or_else(|| {
                        if menu_type.eq_ignore_ascii_case("web_app") {
                            matched_app.map(|a| a.title.clone())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| tr_str(&lang, "打开", "Open").to_string());
                comp.sync_bot_menu_with_cx(bot_id, bot_username.clone(), btn_label, btn_url, c_cx);
            } else {
                comp.clear_bot_menu(Some(c_cx));
            }
            c_cx.notify();
        });

        let persona_prompt = if is_viewer_owner_fabushi {
            bot_reg_or_prof
                .map(|b| b.ai_persona.clone())
                .filter(|s| !s.trim().is_empty())
                .unwrap_or_default()
        } else {
            String::new()
        };
        let in_attach = matched_app.is_some_and(|a| {
            a.is_installed
                || self
                    .miniapps
                    .installed_miniapps
                    .iter()
                    .any(|i| i.app_id == a.app_id)
        });
        let main_app_uri = cached_profile
            .and_then(|p| p.main_app.as_ref())
            .map(|a| a.gpui_uri())
            .or_else(|| {
                bot_reg_or_prof
                    .filter(|b| b.has_main_app && !b.main_app_short_name.trim().is_empty())
                    .map(|b| {
                        format!(
                            "gpui-miniapp://{}/{}",
                            b.username.trim_start_matches('@'),
                            b.main_app_short_name.trim()
                        )
                    })
            })
            .or(explicit_menu_url.clone())
            .or_else(|| matched_app.map(|a| a.gpui_uri()));
        let app_title = matched_app
            .map(|a| a.title.clone())
            .or_else(|| exact_menu_text.clone());

        let mau_count = bot_reg_or_prof
            .map(|b| b.mau_count)
            .filter(|&c| c > 0)
            .or_else(|| matched_app.map(|a| a.mau_count))
            .unwrap_or(0);
        let has_main_app = cached_profile.is_some_and(|p| p.main_app.is_some())
            || bot_reg_or_prof.is_some_and(|b| b.has_main_app)
            || has_web_app_menu;
        let privacy_policy_url = bot_reg_or_prof
            .map(|b| b.privacy_policy_url.clone())
            .filter(|s| !s.trim().is_empty())
            .or_else(|| matched_app.map(|a| a.privacy_policy_url.clone()))
            .unwrap_or_default();
        let has_settings = bot_reg_or_prof.is_some_and(|b| b.has_settings);
        let can_join_groups = bot_reg_or_prof.is_some_and(|b| b.can_join_groups);
        let is_verified = bot_reg_or_prof.is_some_and(|b| b.is_official);
        let is_blocked = cached_profile
            .map(|p| p.is_blocked)
            .or_else(|| bot_reg_or_prof.map(|b| b.is_blocked_by_viewer))
            .or_else(|| rc_opt.as_ref().map(|rc| rc.is_blocked))
            .unwrap_or(false);

        self.inspector.update(cx, |insp, insp_cx| {
            insp.sync_bot_details(
                true,
                bot_id,
                persona_prompt,
                commands,
                app_title,
                main_app_uri,
                in_attach,
            );
            insp.sync_bot_profile_extra(
                mau_count,
                has_main_app,
                privacy_policy_url,
                has_settings,
                can_join_groups,
                is_viewer_owner,
                is_verified,
                is_blocked,
            );
            insp_cx.notify();
        });

        let desc_text = prestart_opt
            .as_ref()
            .map(|p| p.description.clone())
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                bot_reg_or_prof
                    .map(|b| b.description.clone())
                    .filter(|s| !s.trim().is_empty())
            })
            .or_else(|| {
                rc_opt
                    .as_ref()
                    .and_then(|rc| rc.peer_bio.clone())
                    .filter(|s| !s.trim().is_empty())
            });
        let stage_has_msgs = !self.stage.read(cx).messages.is_empty();
        let is_prestart = prestart_opt.is_some();
        let show_start_bar = is_prestart
            || (!stage_has_msgs && cached_profile.is_some_and(|p| !p.has_started_chat));
        let desc_card = if show_start_bar || !stage_has_msgs {
            desc_text
        } else {
            None
        };
        self.stage.update(cx, |st, st_cx| {
            st.set_bot_prestart_state(desc_card, show_start_bar, is_viewer_owner_fabushi);
            st_cx.notify();
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

    /// Refreshes only the user's recently launched Mini Apps and pushes them to `ChatSidebar`.
    pub fn refresh_recent_miniapps(&mut self, cx: &mut Context<Self>) {
        if !self.is_authenticated {
            return;
        }
        let Some(fence) = session_fence(self) else {
            return;
        };
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let recent_res = client.list_recent_mini_apps().await;
            let _ = weak.update(app, |shell, cx| {
                if !fence_matches(&fence, shell) {
                    return;
                }
                if let Ok(rec) = recent_res {
                    shell.miniapps.recent_mini_apps = rec;
                    let mapped_recent = build_sidebar_recent_items(&shell.miniapps);
                    let main_app_uris = build_bot_main_app_uris(shell);
                    shell.sidebar.update(cx, |sb, sb_cx| {
                        sb.set_recent_mini_apps(mapped_recent);
                        sb.set_bot_main_app_uris(main_app_uris);
                        sb_cx.notify();
                    });
                }
                cx.notify();
            });
        })
        .detach();
    }

    /// Returns `true` when the user has already accepted the Mini App ToS for this bot/app.
    pub fn is_miniapp_tos_accepted(
        &self,
        bot_id: u64,
        bot_username: &str,
        short_name: &str,
    ) -> bool {
        let clean_un = bot_username.trim().trim_start_matches('@');
        if bot_id > 0 && self.miniapps.accepted_app_tos_bots.contains(&bot_id) {
            return true;
        }
        let prof_match = if !clean_un.is_empty() {
            self.miniapps
                .bot_profiles_cache
                .get(&clean_un.to_ascii_lowercase())
        } else {
            None
        }
        .or_else(|| {
            if bot_id > 0 {
                self.miniapps
                    .bot_profiles_cache
                    .values()
                    .find(|p| p.bot.bot_user_id == bot_id as i64)
            } else {
                None
            }
        });
        if let Some(p) = prof_match {
            if p.app_tos_accepted
                || p.bot.app_tos_accepted
                || p.main_app.as_ref().is_some_and(|a| a.app_tos_accepted)
                || (!short_name.is_empty()
                    && p.apps
                        .iter()
                        .any(|a| a.short_name.eq_ignore_ascii_case(short_name) && a.app_tos_accepted))
            {
                return true;
            }
        }
        if self
            .miniapps
            .recent_mini_apps
            .iter()
            .chain(self.miniapps.installed_miniapps.iter())
            .chain(self.miniapps.catalog_miniapps.iter())
            .any(|a| {
                a.app_tos_accepted
                    && ((bot_id > 0 && a.bot_user_id == bot_id as i64)
                        || (!clean_un.is_empty() && a.bot_username.eq_ignore_ascii_case(clean_un)))
                    && (short_name.is_empty() || a.short_name.eq_ignore_ascii_case(short_name))
            })
        {
            return true;
        }
        self.miniapps
            .attach_menu_bots
            .iter()
            .chain(self.miniapps.my_bots.iter())
            .any(|b| {
                b.app_tos_accepted
                    && ((bot_id > 0 && b.bot_user_id == bot_id as i64)
                        || (!clean_un.is_empty() && b.username.eq_ignore_ascii_case(clean_un)))
            })
    }

    /// Gates chat-originated launches behind Telegram's Mini App ToS confirmation modal if unaccepted.
    pub fn request_launch_miniapp_with_confirm(
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
        let clean_lower = bot_un.to_ascii_lowercase();
        let prof_opt = self.miniapps.bot_profiles_cache.get(&clean_lower).cloned();
        let app_opt = prof_opt
            .as_ref()
            .and_then(|p| {
                p.apps
                    .iter()
                    .find(|a| a.short_name.eq_ignore_ascii_case(&short))
                    .cloned()
                    .or_else(|| p.main_app.clone())
            })
            .or_else(|| {
                self.miniapps
                    .recent_mini_apps
                    .iter()
                    .chain(self.miniapps.installed_miniapps.iter())
                    .chain(self.miniapps.catalog_miniapps.iter())
                    .find(|a| {
                        a.bot_username.eq_ignore_ascii_case(&bot_un)
                            && a.short_name.eq_ignore_ascii_case(&short)
                    })
                    .cloned()
            });
        let reg_opt = self
            .miniapps
            .attach_menu_bots
            .iter()
            .chain(self.miniapps.my_bots.iter())
            .find(|b| b.username.eq_ignore_ascii_case(&bot_un));
        let bot_id = prof_opt
            .as_ref()
            .map(|p| p.bot.bot_user_id.max(0) as u64)
            .filter(|&id| id > 0)
            .or_else(|| app_opt.as_ref().map(|a| a.bot_user_id.max(0) as u64).filter(|&id| id > 0))
            .or_else(|| reg_opt.map(|b| b.bot_user_id.max(0) as u64).filter(|&id| id > 0))
            .or_else(|| {
                source_chat_id
                    .and_then(|cid| self.session_sync.rich_chats_by_id.get(&cid))
                    .and_then(|rc| rc.peer_user_id)
            })
            .or_else(|| {
                source_chat_id.and_then(|cid| self.session_sync.chat_peer_user_ids.get(&cid).copied())
            })
            .unwrap_or(0);

        if self.is_miniapp_tos_accepted(bot_id, &bot_un, &short) {
            self.launch_miniapp(&bot_un, &short, source_chat_id, launch_mode, start_param, cx);
            return;
        }

        let bot_display_name = prof_opt
            .as_ref()
            .map(|p| p.bot.display_name.clone())
            .filter(|s| !s.trim().is_empty())
            .or_else(|| reg_opt.map(|b| b.display_name.clone()).filter(|s| !s.trim().is_empty()))
            .or_else(|| app_opt.as_ref().map(|a| a.bot_display_name.clone()).filter(|s| !s.trim().is_empty()))
            .unwrap_or_else(|| bot_un.clone());
        let app_title = app_opt
            .as_ref()
            .map(|a| a.title.clone())
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| short.clone());
        let app_id = app_opt.as_ref().map(|a| a.app_id.clone());

        self.miniapps.pending_miniapp_confirm = Some(PendingMiniAppConfirm {
            bot_id,
            bot_username: bot_un,
            bot_display_name,
            app_id,
            short_name: short,
            app_title,
            source_chat_id,
            launch_mode,
            start_param,
        });
        cx.notify();
    }

    /// Confirms the pending Mini App launch dialog, persists ToS acceptance, and launches the Mini App.
    pub fn confirm_pending_miniapp_launch(&mut self, cx: &mut Context<Self>) {
        let Some(confirm) = self.miniapps.pending_miniapp_confirm.take() else {
            return;
        };
        let mut resolved_bot_id = confirm.bot_id;
        if resolved_bot_id > 0 {
            self.miniapps.accepted_app_tos_bots.insert(resolved_bot_id);
        }
        let cache_key = confirm.bot_username.trim().trim_start_matches('@').to_ascii_lowercase();
        if let Some(prof) = self.miniapps.bot_profiles_cache.get_mut(&cache_key) {
            prof.app_tos_accepted = true;
            prof.bot.app_tos_accepted = true;
            if let Some(ref mut ma) = prof.main_app {
                ma.app_tos_accepted = true;
            }
            for app in &mut prof.apps {
                app.app_tos_accepted = true;
            }
        }
        for app in self
            .miniapps
            .catalog_miniapps
            .iter_mut()
            .chain(self.miniapps.installed_miniapps.iter_mut())
            .chain(self.miniapps.recent_mini_apps.iter_mut())
        {
            if (resolved_bot_id > 0 && app.bot_user_id == resolved_bot_id as i64)
                || app.bot_username.eq_ignore_ascii_case(&confirm.bot_username)
            {
                app.app_tos_accepted = true;
            }
        }

        if let Some(fence) = session_fence(self) {
            let client = self.client.clone();
            let bot_un = confirm.bot_username.clone();
            let app_id = confirm.app_id.clone();
            cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                if resolved_bot_id == 0 {
                    if let Ok(prof) = client.get_bot_public_profile(&bot_un).await {
                        resolved_bot_id = prof.bot.bot_user_id.max(0) as u64;
                    }
                }
                if resolved_bot_id > 0 {
                    let _ = client
                        .accept_mini_app_tos(
                            resolved_bot_id,
                            &AcceptMiniAppTosRequest {
                                app_id,
                                allow_write_access: None,
                            },
                        )
                        .await;
                    let _ = weak.update(app, |shell, _| {
                        if fence_matches(&fence, shell) {
                            shell.miniapps.accepted_app_tos_bots.insert(resolved_bot_id);
                        }
                    });
                }
            })
            .detach();
        }

        self.launch_miniapp(
            &confirm.bot_username,
            &confirm.short_name,
            confirm.source_chat_id,
            confirm.launch_mode,
            confirm.start_param,
            cx,
        );
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
        let server_base = self.client.http_client.base_url.clone();
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
            w.set_server_base_url(server_base);
            w.is_open = true;
            w.is_loading = true;
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
                    Ok(mut ctx) => {
                        let uid = ctx.app.bot_user_id.max(0) as u64;
                        if uid > 0 && shell.miniapps.accepted_app_tos_bots.contains(&uid) {
                            ctx.app.app_tos_accepted = true;
                        }
                        let _ = ov_weak.update(cx, |w, w_cx| {
                            w.load_context(ctx);
                            w_cx.notify();
                        });
                        shell.refresh_recent_miniapps(cx);
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
        let needs_confirm = matches!(
            launch_mode,
            MiniAppLaunchMode::MenuButton
                | MiniAppLaunchMode::InlineKeyboard
                | MiniAppLaunchMode::AttachMenu
                | MiniAppLaunchMode::MainMiniApp
        );
        if let Some((bot_un, short_name, start_param)) = parse_miniapp_uri(raw_uri) {
            if needs_confirm {
                self.request_launch_miniapp_with_confirm(
                    &bot_un,
                    &short_name,
                    source_chat_id,
                    launch_mode,
                    start_param,
                    cx,
                );
            } else {
                self.launch_miniapp(
                    &bot_un,
                    &short_name,
                    source_chat_id,
                    launch_mode,
                    start_param,
                    cx,
                );
            }
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
                if needs_confirm {
                    self.request_launch_miniapp_with_confirm(
                        &app.bot_username,
                        &app.short_name,
                        Some(cid),
                        launch_mode,
                        None,
                        cx,
                    );
                } else {
                    self.launch_miniapp(
                        &app.bot_username,
                        &app.short_name,
                        Some(cid),
                        launch_mode,
                        None,
                        cx,
                    );
                }
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
            self.request_launch_miniapp_with_confirm(
                bot_username,
                short_name,
                active_chat,
                mode,
                None,
                cx,
            );
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
            self.request_launch_miniapp_with_confirm(
                &app.bot_username,
                &app.short_name,
                active_chat,
                mode,
                None,
                cx,
            );
        }
    }

    /// Fetches `BotPublicProfileDto` without creating any DM chat and opens centered `BotProfileBox`.
    pub fn open_bot_profile_popup(&mut self, bot_target: &str, cx: &mut Context<Self>) {
        let clean = bot_target.trim().trim_start_matches('@').to_string();
        if clean.is_empty() {
            return;
        }
        let cache_key = clean.to_ascii_lowercase();
        if let Some(cached) = self.miniapps.bot_profiles_cache.get(&cache_key).cloned() {
            self.show_bot_profile_box_entity(cached, cx);
        }
        let Some(fence) = session_fence(self) else {
            return;
        };
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.get_bot_public_profile(&clean).await;
            let _ = weak.update(app, |shell, cx| {
                if !fence_matches(&fence, shell) {
                    return;
                }
                match res {
                    Ok(prof) => {
                        let k = prof
                            .bot
                            .username
                            .trim()
                            .trim_start_matches('@')
                            .to_ascii_lowercase();
                        if !k.is_empty() {
                            shell.miniapps.bot_profiles_cache.insert(k, prof.clone());
                        }
                        if prof.app_tos_accepted && prof.bot.bot_user_id > 0 {
                            shell
                                .miniapps
                                .accepted_app_tos_bots
                                .insert(prof.bot.bot_user_id as u64);
                        }
                        shell.show_bot_profile_box_entity(prof, cx);
                    }
                    Err(e) => {
                        if shell
                            .miniapps
                            .bot_profile_box
                            .as_ref()
                            .is_none_or(|b| !b.read(cx).is_visible)
                        {
                            shell.window_runtime.enqueue_toast(
                                format!("Failed to load bot info: {}", e),
                                2400,
                            );
                        }
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn show_bot_profile_box_entity(
        &mut self,
        prof: BotPublicProfileDto,
        cx: &mut Context<Self>,
    ) {
        let pal = self.theme_state.palette;
        let lang = self.active_lang_id().to_string();
        if let Some(ref existing) = self.miniapps.bot_profile_box {
            existing.update(cx, |bx, bx_cx| {
                bx.set_palette(pal);
                bx.set_language(lang);
                bx.open(prof);
                bx_cx.notify();
            });
            return;
        }
        let ent = cx.new(|_| {
            crate::views::boxes::peers::bot_profile_box::BotProfileBox::new(prof, pal, lang)
        });
        cx.observe(&ent, |shell, bx, cx| {
            shell.drain_bot_profile_box_actions(&bx, cx);
        })
        .detach();
        self.miniapps.bot_profile_box = Some(ent);
    }

    /// Opens an existing bot chat or displays a virtual pre-start bot conversation without creating a DB chat.
    pub fn open_bot_message_chat_or_prestart(
        &mut self,
        bot_id: u64,
        bot_username: &str,
        display_name: &str,
        description: &str,
        existing_chat_id: Option<i64>,
        has_started_chat: bool,
        cx: &mut Context<Self>,
    ) {
        activate_chats_workspace(self, cx);
        if let Some(cid) = existing_chat_id.filter(|&c| c > 0) {
            self.miniapps.pending_prestart_bot = None;
            self.open_or_select_chat(
                cid as u64,
                Some(display_name.to_string()),
                true,
                cx,
            );
            if !has_started_chat {
                self.sync_composer_and_inspector_miniapp_state(cx);
            }
            return;
        }
        let clean_un = bot_username.trim().trim_start_matches('@').to_string();
        let prof_opt = self
            .miniapps
            .bot_profiles_cache
            .get(&clean_un.to_ascii_lowercase());
        let is_owner_fabushi = prof_opt.is_some_and(|p| {
            p.bot.is_fabushi_bot && self.session_sync.user_id == Some(p.bot.owner_user_id as u64)
        });
        let title_str = if display_name.trim().is_empty() {
            clean_un.clone()
        } else {
            display_name.to_string()
        };
        let initials = {
            let i = derive_initials(&title_str);
            if i.is_empty() { "BT".to_string() } else { i }
        };
        let color = peer_avatar_color(bot_id.max(1));
        let lang = self.active_lang_id().to_string();
        let subtitle = tr_str(&lang, "机器人", "bot").to_string();

        self.miniapps.pending_prestart_bot =
            Some(crate::views::app_shell::shell_state::PrestartBotTarget {
                bot_id,
                bot_username: clean_un.clone(),
                display_name: title_str.clone(),
                description: description.to_string(),
                is_viewer_owner_fabushi: is_owner_fabushi,
            });
        self.sidebar.update(cx, |sb, sb_cx| {
            sb.active_chat_id = None;
            sb.search_focused = false;
            sb_cx.notify();
        });
        self.stage.update(cx, |st, st_cx| {
            st.active_chat_id = None;
            st.messages.clear();
            st.row_height_cache.rebuild_prefix_sums(&st.messages);
            st.chat_title = title_str.clone();
            st.chat_subtitle = subtitle.clone();
            st.chat_avatar_initials = initials.clone();
            st.chat_avatar_color = color;
            st.is_group_chat = false;
            st.is_saved_messages = false;
            st.pinned_message = None;
            st.set_bot_prestart_state(
                Some(description.to_string()),
                true,
                is_owner_fabushi,
            );
            st_cx.notify();
        });
        self.inspector.update(cx, |insp, insp_cx| {
            insp.set_peer(&title_str, &subtitle, &initials, color);
            let at_un = if clean_un.is_empty() {
                String::new()
            } else {
                format!("@{clean_un}")
            };
            insp.sync_profile_details(
                at_un,
                String::new(),
                tr_str(&lang, "机器人", "bot").to_string(),
                None,
                Vec::new(),
            );
            insp.profile.bio = description.to_string();
            insp.sync_members(Vec::new(), String::new(), false);
            insp.sync_shared_content(Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
            insp_cx.notify();
        });
        self.sync_composer_and_inspector_miniapp_state(cx);
        cx.notify();
    }

    /// Materializes a pending virtual pre-start bot conversation into a real DB chat and sends `/start`.
    pub fn materialize_prestart_bot_and_start(&mut self, cx: &mut Context<Self>) {
        if let Some(cid) = resolve_active_target_chat_id(
            self.stage.read(cx).active_chat_id,
            self.sidebar.read(cx).active_chat_id,
        ) {
            self.stage.update(cx, |st, st_cx| {
                st.pending_bot_start = false;
                st.show_bot_start_bar = false;
                st_cx.notify();
            });
            self.append_outgoing_text("/start".to_string(), cx);
            if let Some(rc) = self.session_sync.rich_chats_by_id.get(&cid) {
                if let Some(ref un) = rc.peer_username {
                    if let Some(prof) = self
                        .miniapps
                        .bot_profiles_cache
                        .get_mut(&un.trim().trim_start_matches('@').to_ascii_lowercase())
                    {
                        prof.has_started_chat = true;
                    }
                }
            }
            return;
        }
        let Some(target) = self.miniapps.pending_prestart_bot.clone() else {
            return;
        };
        let Some(fence) = session_fence(self) else {
            return;
        };
        let token = fence.token.clone();
        let client = self.client.clone();
        let req = AddContactHttpRequest {
            first_name: target.display_name.clone(),
            last_name: None,
            username: Some(target.bot_username.clone()),
            phone: None,
            is_bot: true,
            bot_system_prompt: None,
        };
        self.stage.update(cx, |st, st_cx| {
            st.pending_bot_start = false;
            st.show_bot_start_bar = false;
            st_cx.notify();
        });
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.add_contact(&token, req).await;
            let _ = weak.update(app, |shell, cx| {
                if !fence_matches(&fence, shell) {
                    return;
                }
                if let Ok(resp) = res {
                    shell.miniapps.pending_prestart_bot = None;
                    let key = target
                        .bot_username
                        .trim()
                        .trim_start_matches('@')
                        .to_ascii_lowercase();
                    if let Some(prof) = shell.miniapps.bot_profiles_cache.get_mut(&key) {
                        prof.existing_chat_id = Some(resp.chat_id as i64);
                        prof.has_started_chat = true;
                    }
                    shell.open_or_select_chat(resp.chat_id, Some(resp.title), true, cx);
                    shell.append_outgoing_text("/start".to_string(), cx);
                    shell.refresh_chats(cx);
                }
            });
        })
        .detach();
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
