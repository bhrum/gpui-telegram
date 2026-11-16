//! Entity observer bindings, WebSocket connection status monitoring, and event handlers
//! for `AuthWizardView`, `SettingsRouter`, and workspace panels in `AppShell`.

use std::future::Future;
use std::sync::Arc;

use gpui::{AsyncApp, ClipboardItem, Context, WeakEntity};
use gpui_telegram_client::{ConnectionStatus, TelegramClient};
use gpui_telegram_proto::contracts::auth::{
    TwoFactorSetRequest, UpdateProfileRequest as ProtoUpdateProfileRequest,
};
use gpui_telegram_proto::contracts::chats::ChatFolderDto;
use gpui_telegram_proto::miniapp_contracts::MiniAppLaunchMode;

use crate::i18n::tr_str;
use crate::views::app_shell::shell_state::should_switch_to_chats_on_sidebar_selection;
use crate::views::app_shell::shell_theme::toggle_desktop_theme_mode;
use crate::views::app_shell::window_connecting_widget::ConnectionStatus as BannerConnectionStatus;
use crate::views::auth::AuthWizardEvent;
use crate::views::boxes::modal_types::ModalDialogKind;
use crate::views::chat_sidebar::CategorizedSearchResult;
use crate::views::nav_rail::NavDestination;
use crate::views::settings::{settings_type::SettingsSectionType, SettingsRouterEvent};
use crate::views::shell::{toggled_language_code, ActiveWorkspacePanel, AppShell};

impl AppShell {
    fn spawn_shell_fut<R: 'static, Fut: Future<Output = R> + 'static>(
        &self,
        cx: &mut Context<Self>,
        fut: Fut,
        apply: impl FnOnce(&mut Self, R, &mut Context<Self>) + 'static,
    ) {
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let out = fut.await;
            let _ = weak.update(app, |shell, cx| apply(shell, out, cx));
        })
        .detach();
    }

    fn sync_auth_wizard_visuals(&self, cx: &mut Context<Self>) {
        let (pal, is_zh) = (self.theme_state.palette, self.theme_state.is_chinese_locale);
        self.auth_wizard.update(cx, |w, wcx| {
            w.apply_theme_and_locale(pal, is_zh);
            wcx.notify();
        });
    }

    pub(crate) fn enqueue_localized_toast(
        &mut self,
        zh: &'static str,
        en: &'static str,
        duration_ms: u64,
        cx: &mut Context<Self>,
    ) {
        let msg = tr_str(self.active_lang_id(), zh, en);
        self.window_runtime.enqueue_toast(msg, duration_ms);
        cx.notify();
    }

    /// Wires all child view subscriptions, entity observers, and the WS status monitor.
    pub(crate) fn wire_workspace_observers(&self, cx: &mut Context<Self>) {
        cx.subscribe(&self.auth_wizard, |shell, _, ev, cx| shell.handle_auth_wizard_event(ev, cx))
            .detach();
        cx.subscribe(&self.settings_router, |shell, _, ev, cx| {
            shell.handle_settings_router_event(ev, cx);
        })
        .detach();

        self.spawn_ws_status_observer(cx);
        self.spawn_idle_auto_lock_observer(cx);
        self.observe_drawer_and_modal_entities(cx);
        self.observe_nav_rail_entity(cx);
        self.observe_sidebar_entity(cx);
        self.observe_stage_composer_inspector_entities(cx);
    }

    fn spawn_idle_auto_lock_observer(&self, cx: &mut Context<Self>) {
        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            loop {
                app.background_executor()
                    .timer(std::time::Duration::from_secs(5))
                    .await;
                let keep = weak_shell
                    .update(app, |shell, cx| {
                        let elapsed = shell.overlays.last_user_activity.elapsed().as_secs();
                        let _ = shell.check_idle_auto_lock(elapsed, cx);
                        true
                    })
                    .unwrap_or(false);
                if !keep {
                    break;
                }
            }
        })
        .detach();
    }

    pub(crate) fn stop_settings_mic_test_capture(&mut self, cx: &mut Context<Self>) {
        self.overlays.mic_test_loop_gen = self.overlays.mic_test_loop_gen.wrapping_add(1);
        if self.rtc_talk.active_call_id.is_none()
            && self.rtc_talk.talk_mode_overlay.is_none()
            && self.rtc_talk.voice_recorder.is_recording()
        {
            let _ = self.rtc_talk.voice_recorder.stop();
        }
        self.settings_router.update(cx, |sr, scx| {
            if let Some(ref sc) = sr.settings_calls {
                sc.update(scx, |sc, sc_cx| {
                    sc.stop_mic_test_silent();
                    sc_cx.notify();
                });
            }
        });
    }

    /// Spawns a background task monitoring `WsTransport` status changes for the current `self.client`.
    pub(crate) fn spawn_ws_status_observer(&self, cx: &mut Context<Self>) {
        let watched_client = self.client.clone();
        let mut status_rx = watched_client.ws_transport.subscribe_status();
        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            while status_rx.changed().await.is_ok() {
                let status = status_rx.borrow().clone();
                let is_connected = status.is_connected();
                let keep = weak_shell
                    .update(app, |shell, cx| {
                        if !Arc::ptr_eq(&shell.client, &watched_client) {
                            return false;
                        }
                        let was_connected = shell.nav_rail.read(cx).is_connected;
                        shell.nav_rail.update(cx, |r, _| r.set_connected(is_connected));
                        let banner = match &status {
                            ConnectionStatus::Connected => BannerConnectionStatus::Connected,
                            ConnectionStatus::Connecting => BannerConnectionStatus::Connecting,
                            ConnectionStatus::Reconnecting { .. } => BannerConnectionStatus::Updating,
                            ConnectionStatus::Offline { .. } => BannerConnectionStatus::WaitingForNetwork,
                        };
                        shell.window_runtime.set_connection_status(banner, None);
                        if is_connected && !was_connected {
                            shell.refresh_chats(cx);
                            if let Some(cid) = shell.stage.read(cx).active_chat_id {
                                shell.load_historical_messages(cid, cx);
                            }
                        }
                        if let ConnectionStatus::Offline { ref reason } = status {
                            if reason.contains("401") || reason.contains("unauthorized") {
                                shell.logout(cx);
                                shell.auth_wizard.update(cx, |w, wcx| {
                                    w.set_login_error(Some(
                                        "Session expired or revoked from another device. Please sign in again."
                                            .to_string(),
                                    ));
                                    wcx.notify();
                                });
                            }
                        }
                        cx.notify();
                        true
                    })
                    .unwrap_or(false);
                if !keep {
                    break;
                }
            }
        })
        .detach();
    }

    fn observe_drawer_and_modal_entities(&self, cx: &mut Context<Self>) {
        cx.observe(&self.main_menu_drawer, |shell, drawer, cx| {
            let (cmd, bf, ma) = drawer
                .update(cx, |d, _| (d.take_command(), d.take_botfather_open(), d.take_miniapps_center()));
            if let Some(cmd) = cmd {
                shell.handle_main_menu_command(cmd, cx);
            }
            if bf {
                shell.open_botfather_chat(cx);
            }
            if ma {
                shell.open_miniapps_center(cx);
            }
        })
        .detach();
        cx.observe(&self.modal_container, |shell, modal, cx| {
            if let Some(outcome) = modal.update(cx, |m, _| m.take_outcome()) {
                shell.handle_modal_outcome(outcome, cx);
            }
        })
        .detach();
        cx.observe(&self.tasks_workspace, |shell, tw, cx| {
            if let Some(act) = tw.update(cx, |w, _| w.take_action()) {
                shell.handle_tasks_workspace_action(act, cx);
            }
        })
        .detach();
    }

    fn observe_nav_rail_entity(&self, cx: &mut Context<Self>) {
        cx.observe(&self.nav_rail, |shell, rail, cx| {
            let (dest, logout_req, trigger, menu_req, archive_req, theme_req, folder_req) =
                rail.update(cx, |r, _| {
                    let f = r.selected_folder_requested.take().and_then(|fid| {
                        r.custom_folders.iter().find(|i| i.folder_id == fid).map(|i| i.title.clone())
                    });
                    (
                        r.active_destination,
                        r.logout_requested,
                        r.trigger_counter,
                        std::mem::take(&mut r.menu_open_requested),
                        std::mem::take(&mut r.archive_open_requested),
                        std::mem::take(&mut r.theme_toggle_requested),
                        f,
                    )
                });
            if menu_req {
                shell.sidebar.update(cx, |s, s_cx| {
                    s.is_account_popover_open = !s.is_account_popover_open;
                    s.is_plus_menu_open = false;
                    s_cx.notify();
                });
                cx.notify();
            }
            if archive_req {
                shell.active_panel = ActiveWorkspacePanel::Chats;
                shell.sidebar.update(cx, |sb, _| sb.is_archived_expanded = !sb.is_archived_expanded);
                cx.notify();
            }
            if theme_req {
                let next = toggle_desktop_theme_mode(shell.theme_state.theme_mode);
                shell.apply_theme_selection(next, None, None, cx);
                shell.sidebar.update(cx, |_, sb_cx| sb_cx.notify());
            }
            if let Some(folder_title) = folder_req {
                shell.active_panel = ActiveWorkspacePanel::Chats;
                shell.sidebar.update(cx, |sb, sb_cx| {
                    sb.select_folder(folder_title);
                    sb_cx.notify();
                });
                cx.notify();
            }
            if logout_req {
                rail.update(cx, |r, _| {
                    r.logout_requested = false;
                    r.is_connected = false;
                });
                shell.logout(cx);
                return;
            }
            if shell.last_handled_nav_trigger == trigger && shell.last_handled_nav_dest == Some(dest) {
                return;
            }
            shell.last_handled_nav_trigger = trigger;
            shell.activate_nav_destination(dest, cx);
        })
        .detach();
    }

    fn observe_sidebar_entity(&self, cx: &mut Context<Self>) {
        cx.observe(&self.sidebar, |shell, sb, cx| {
            let (
                selected_dialog, selection_rev, story_peer, search_focused, has_creation_modal,
                pending_contact, pending_group, search_query, search_cat, search_seq,
                open_menu, open_new_channel, context_action, is_collapsed, collapse_rev,
                open_botfather, install_open_app, open_app_only, open_bot_profile,
                launch_dialog_miniapp, open_search_hit,
            ) = sb.update(cx, |s, _| (
                s.selected_dialog().cloned(), s.selection_revision, s.story_clicked_peer.take(),
                s.search_focused, s.active_creation_modal.is_some(),
                s.pending_add_contact_request.take(), s.pending_create_group_request.take(),
                s.search_query.clone(), s.active_search_category.clone(), s.search_request_seq,
                s.take_open_main_menu_requested(), s.take_pending_open_new_channel(),
                s.take_pending_context_action(), s.is_collapsed, s.collapse_state_revision,
                std::mem::take(&mut s.pending_open_botfather),
                s.pending_install_open_miniapp.take(), s.pending_open_miniapp_only.take(),
                s.pending_open_bot_profile.take(), s.pending_launch_dialog_miniapp.take(),
                s.pending_open_search_hit.take(),
            ));
            if open_botfather {
                shell.open_botfather_chat(cx);
            }
            if let Some(app_item) = install_open_app {
                shell.install_and_launch_miniapp(app_item, cx);
            }
            if let Some(app_item) = open_app_only {
                if app_item.is_installed {
                    shell.launch_miniapp(
                        &app_item.bot_username, &app_item.short_name,
                        app_item.companion_chat_id, MiniAppLaunchMode::DirectLink, None, cx,
                    );
                } else {
                    shell.install_and_launch_miniapp(app_item, cx);
                }
            }
            if let Some(bot_un) = open_bot_profile {
                shell.open_bot_profile_popup(&bot_un, cx);
            }
            if let Some((cid, uri)) = launch_dialog_miniapp {
                let _ = shell.launch_miniapp_from_uri(
                    &uri,
                    Some(cid),
                    MiniAppLaunchMode::MainMiniApp,
                    cx,
                );
            }
            if let Some(hit) = open_search_hit {
                shell.handle_sidebar_search_hit(hit, cx);
            }
            if collapse_rev != shell.overlays.last_sidebar_collapse_revision {
                shell.overlays.last_sidebar_collapse_revision = collapse_rev;
                shell.update_preferences(cx, |p| p.appearance.sidebar_collapsed = is_collapsed);
            }
            if open_menu {
                shell.sidebar.update(cx, |s, s_cx| {
                    s.is_account_popover_open = !s.is_account_popover_open;
                    s.is_plus_menu_open = false;
                    s_cx.notify();
                });
            }
            let sb_popup = {
                let s = shell.sidebar.read(cx);
                s.is_plus_menu_open || s.is_account_popover_open || s.is_contacts_modal_open || s.active_creation_modal.is_some()
            };
            if sb_popup && shell.stage.read(cx).more_menu_open {
                shell.stage.update(cx, |st, st_cx| {
                    st.more_menu_open = false;
                    st.ttl_submenu_open = false;
                    st_cx.notify();
                });
            }
            if open_new_channel {
                let title = tr_str(shell.active_lang_id(), "新建频道", "New Channel");
                shell.open_modal_dialog(title, ModalDialogKind::NewChannel, 420.0, 380.0, cx);
            }
            if let Some(action) = context_action {
                shell.handle_sidebar_context_action(action, cx);
            }
            if search_focused || has_creation_modal {
                shell.composer.update(cx, |c, _| c.is_input_focused = false);
            }
            if let Some(req) = pending_contact {
                shell.dispatch_add_contact(req, cx);
            }
            if let Some(req) = pending_group {
                shell.dispatch_create_group(req, cx);
            }
            if search_focused && search_seq > 0 && shell.session_sync.last_dispatched_search_seq != search_seq {
                shell.dispatch_categorized_search(search_query, search_cat, search_seq, cx);
            }
            if let Some(peer) = story_peer {
                shell.open_modal_dialog(peer.clone(), ModalDialogKind::StoriesViewer { peer_name: peer }, 420.0, 560.0, cx);
            }
            if should_switch_to_chats_on_sidebar_selection(
                selection_rev, &mut shell.overlays.last_sidebar_selection_revision, selected_dialog.is_some(),
            ) {
                if let Some(d) = selected_dialog {
                    shell.active_panel = ActiveWorkspacePanel::Chats;
                    shell.last_handled_nav_dest = Some(NavDestination::Chats);
                    shell.nav_rail.update(cx, |r, _| r.set_active_destination(NavDestination::Chats));
                    let st = shell.stage.read(cx);
                    if st.active_chat_id != Some(d.id) || st.chat_title != d.peer_name {
                        shell.sync_selected_chat_inspector_and_stage(d.id, cx);
                    }
                }
            }
            cx.notify();
        })
        .detach();
    }

    fn handle_sidebar_search_hit(&mut self, hit: CategorizedSearchResult, cx: &mut Context<Self>) {
        if hit.category.eq_ignore_ascii_case("Apps") {
            let chat_opt = (hit.chat_id > 0).then_some(hit.chat_id);
            if !self.launch_miniapp_from_uri(&hit.subtitle, chat_opt, MiniAppLaunchMode::DirectLink, cx) {
                let found = self.miniapps.catalog_miniapps.iter().find(|a| {
                    a.title.eq_ignore_ascii_case(&hit.title)
                        || hit.subtitle.to_lowercase().contains(&a.bot_username.to_lowercase())
                }).cloned();
                if let Some(app) = found {
                    self.launch_miniapp(
                        &app.bot_username, &app.short_name,
                        app.direct_chat_id.filter(|&id| id > 0).map(|id| id as u64),
                        MiniAppLaunchMode::DirectLink, None, cx,
                    );
                }
            }
        } else if hit.is_bot {
            let candidate_un = hit.subtitle.split_whitespace().find(|w| w.starts_with('@')).map(|w| {
                w.trim_start_matches('@').trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_')
            }).filter(|s| !s.is_empty());
            if let Some(un) = candidate_un {
                self.open_bot_chat_by_username(un, cx);
            } else if hit.chat_id > 0 {
                self.open_or_select_chat(hit.chat_id, Some(hit.title), true, cx);
            } else {
                self.open_bot_chat_by_username(&hit.title, cx);
            }
        } else if hit.chat_id > 0 {
            self.open_or_select_chat(hit.chat_id, Some(hit.title), hit.is_bot, cx);
        }
    }

    fn observe_stage_composer_inspector_entities(&self, cx: &mut Context<Self>) {
        cx.observe(&self.stage, |shell, st, cx| {
            let bot_start_clicked = st.update(cx, |s, _| std::mem::take(&mut s.pending_bot_start));
            if bot_start_clicked {
                shell.materialize_prestart_bot_and_start(cx);
            }
            shell.handle_stage_pending_actions(cx);
            let insp_open = st.read(cx).is_inspector_open;
            shell.inspector.update(cx, |insp, insp_cx| {
                if insp.is_open != insp_open {
                    insp.is_open = insp_open;
                    insp_cx.notify();
                }
            });
            cx.notify();
        })
        .detach();
        cx.observe(&self.composer, |shell, comp, cx| {
            let (att_opt, focused, launch_miniapp) =
                comp.update(cx, |c, _| (c.submitted_attachment.take(), c.is_input_focused, c.take_launch_miniapp()));
            if focused {
                shell.sidebar.update(cx, |sb, _| sb.search_focused = false);
            }
            if let Some(kind) = att_opt {
                shell.append_outgoing_attachment(&kind, cx);
            }
            if let Some((bot_id, bot_un, short_name, url)) = launch_miniapp {
                shell.launch_miniapp_from_composer(bot_id, &bot_un, &short_name, &url, cx);
            }
            shell.handle_composer_pending_actions(cx);
            cx.notify();
        })
        .detach();
        cx.observe(&self.inspector, |shell, insp, cx| {
            let (is_open, media_open, action) =
                insp.update(cx, |i, _| (i.is_open, i.pending_media_open.take(), i.take_action()));
            if let Some((url, is_video)) = media_open {
                shell.show_media_viewer(url, is_video, cx);
            }
            if let Some(act) = action {
                shell.handle_inspector_action(act, cx);
            }
            shell.stage.update(cx, |st, _| st.is_inspector_open = is_open);
            cx.notify();
        })
        .detach();
    }

    /// Dispatches events emitted by `AuthWizardView` (authentication, server URL updates, theme/locale toggles, toasts).
    pub fn handle_auth_wizard_event(&mut self, ev: &AuthWizardEvent, cx: &mut Context<Self>) {
        match ev {
            AuthWizardEvent::Authenticated(auth) => self.apply_authenticated_session(auth.clone(), cx),
            AuthWizardEvent::ServerUrlApplied(url) => {
                let cache_dir = self.client.local_store.cache_dir().map(|p| p.to_path_buf());
                self.client.local_store.clear_all();
                let mut new_cfg = self.client.config.clone();
                new_cfg.server_url = url.clone();
                let next_client = Arc::new(match cache_dir {
                    Some(dir) => TelegramClient::open_with_dir(new_cfg.clone(), dir)
                        .unwrap_or_else(|_| TelegramClient::new(new_cfg)),
                    None => TelegramClient::new(new_cfg),
                });
                next_client.ws_transport.set_max_reconnect_attempts(next_client.config.max_reconnect_attempts);
                self.client = next_client;
                self.session_sync.ws_stream_started = false;
                self.sidebar.update(cx, |sb, _| sb.configure_for_cloud_workspace(url));
                self.stage.update(cx, |st, _| st.configure_for_cloud_workspace(url, None));
                self.inspector.update(cx, |insp, _| insp.configure_for_cloud_workspace(url));
                self.spawn_ws_status_observer(cx);
                cx.notify();
            }
            AuthWizardEvent::Toast { message, duration_ms } => {
                self.window_runtime.enqueue_toast(message.clone(), *duration_ms);
                cx.notify();
            }
            AuthWizardEvent::ToggleTheme => {
                let next = toggle_desktop_theme_mode(self.theme_state.theme_mode);
                self.apply_theme_selection(next, None, None, cx);
                self.sync_auth_wizard_visuals(cx);
            }
            AuthWizardEvent::ToggleLanguage => {
                self.toggle_language_pref(cx);
                self.sync_auth_wizard_visuals(cx);
            }
        }
    }

    fn toggle_language_pref(&mut self, cx: &mut Context<Self>) {
        let next = toggled_language_code(&self.preferences.localization.language_code).to_string();
        self.update_preferences(cx, |p| p.localization.language_code = next);
    }

    /// Dispatches high-level actions and preference mutations emitted by `SettingsRouter`.
    pub fn handle_settings_router_event(&mut self, ev: &SettingsRouterEvent, cx: &mut Context<Self>) {
        match ev {
            SettingsRouterEvent::CloseRequested => {
                self.stop_settings_mic_test_capture(cx);
                self.close_settings(cx);
            }
            SettingsRouterEvent::LogoutRequested => {
                self.stop_settings_mic_test_capture(cx);
                self.logout(cx);
            }
            SettingsRouterEvent::LockAppNow => {
                self.stop_settings_mic_test_capture(cx);
                self.lock_app_now(cx);
            }
            SettingsRouterEvent::CopyClipboard(txt) => {
                cx.write_to_clipboard(ClipboardItem::new_string(txt.clone()));
                self.enqueue_localized_toast("已复制到剪贴板", "Copied to clipboard", 1800, cx);
            }
            SettingsRouterEvent::UpdateInterfaceScale(pct) => {
                let pct = *pct;
                self.update_preferences(cx, |p| p.interface_scale_pct = pct);
            }
            SettingsRouterEvent::OpenModal { title, kind, width, height } =>
                self.open_modal_dialog(title.clone(), kind.clone(), *width, *height, cx),
            SettingsRouterEvent::ApplyTheme { mode, accent, wallpaper } =>
                self.apply_theme_selection(*mode, Some(*accent), Some(wallpaper.clone()), cx),
            SettingsRouterEvent::UpdateChatPrefs {
                font_size,
                bubble_radius,
                send_on_enter,
                suggest_stickers,
                loop_animated_stickers,
                blur_wallpaper,
            } => {
                self.theme_state.bubble_radius_px = *bubble_radius;
                self.composer.update(cx, |c, ccx| {
                    c.send_on_enter = *send_on_enter;
                    c.suggest_stickers_by_emoji = *suggest_stickers;
                    c.loop_animated_stickers = *loop_animated_stickers;
                    c.stickers_list.loop_animated = *loop_animated_stickers;
                    ccx.notify();
                });
                self.stage.update(cx, |st, stcx| {
                    st.loop_animated_stickers = *loop_animated_stickers;
                    stcx.notify();
                });
                self.update_preferences(cx, |p| {
                    p.appearance.font_size = *font_size;
                    p.appearance.bubble_radius = *bubble_radius;
                    p.appearance.send_on_enter = *send_on_enter;
                    p.appearance.suggest_stickers_by_emoji = *suggest_stickers;
                    p.appearance.loop_animated_stickers = *loop_animated_stickers;
                    p.appearance.blur_wallpaper = *blur_wallpaper;
                });
            }
            SettingsRouterEvent::SaveProfile { first_name, last_name, username, phone, bio } => {
                let c = self.client.clone();
                let req = ProtoUpdateProfileRequest {
                    first_name: Some(first_name.clone()), last_name: Some(last_name.clone()),
                    username: username.clone(), phone: Some(phone.clone()), bio: Some(bio.clone()),
                    avatar_media_id: None, emoji_status: None,
                };
                self.spawn_shell_fut(cx, async move { c.update_profile(&req).await }, |s, res, cx| match res {
                    Ok(_) => {
                        let msg = tr_str(s.active_lang_id(), "个人资料已保存", "Profile saved").to_string();
                        let info = s.settings_router.update(cx, |r, rcx| r.ensure_information(rcx));
                        info.update(cx, |is, icx| { is.status_message = Some(msg.clone()); icx.notify(); });
                        s.window_runtime.enqueue_toast(msg, 2200);
                        s.refresh_settings_truth(cx);
                    }
                    Err(e) => { s.window_runtime.enqueue_toast(format!("Save failed: {e}"), 2600); cx.notify(); }
                });
            }
            SettingsRouterEvent::TwoFactorMutation(pw_opt) => {
                let (c, pw_opt) = (self.client.clone(), pw_opt.clone());
                let enabled = pw_opt.is_some();
                let fut = async move {
                    match pw_opt {
                        Some(pw) => c.set_2fa(&TwoFactorSetRequest {
                            current_password: None, new_password: pw, hint: None, recovery_email: None,
                        }).await.is_ok(),
                        None => c.disable_2fa(None).await.is_ok(),
                    }
                };
                self.spawn_shell_fut(cx, fut, move |s, ok, cx| {
                    if !ok {
                        s.enqueue_localized_toast("两步验证操作失败", "Failed to update 2FA", 2400, cx);
                        return;
                    }
                    let pe = s.settings_router.update(cx, |r, rcx| r.ensure_privacy(rcx));
                    pe.update(cx, |ps, pcx| { ps.set_two_factor_enabled(enabled); pcx.notify(); });
                    let (zh, en) = if enabled {
                        ("两步验证密码已启用", "Two-Step Verification enabled")
                    } else {
                        ("两步验证已关闭", "Two-Step Verification disabled")
                    };
                    s.enqueue_localized_toast(zh, en, 2200, cx);
                });
            }
            SettingsRouterEvent::PushUserSettings => self.push_user_settings_to_backend(cx),
            SettingsRouterEvent::RevokeSession(sid) => {
                let (c, sid) = (self.client.clone(), sid.clone());
                self.spawn_shell_fut(cx, async move { let _ = c.revoke_session(&sid).await; }, |s, _, cx| {
                    s.enqueue_localized_toast("已注销该设备会话", "Session terminated", 2000, cx);
                    s.refresh_settings_truth(cx);
                });
            }
            SettingsRouterEvent::RevokeOtherSessions => {
                let c = self.client.clone();
                self.spawn_shell_fut(cx, async move { let _ = c.revoke_other_sessions().await; }, |s, _, cx| {
                    s.enqueue_localized_toast("已注销所有其他设备", "Terminated all other sessions", 2200, cx);
                    s.refresh_settings_truth(cx);
                });
            }
            SettingsRouterEvent::UnblockPeer { user_id, .. } => {
                let (c, uid) = (self.client.clone(), *user_id);
                self.spawn_shell_fut(cx, async move { let _ = c.unblock_user(uid).await; }, |s, _, cx| {
                    s.enqueue_localized_toast("已取消屏蔽该用户", "User unblocked", 2000, cx);
                    s.refresh_chats(cx);
                });
            }
            SettingsRouterEvent::UpdateLocalPasscode { disable, hash, auto_lock_seconds } => {
                if *disable {
                    self.overlays.is_passcode_locked = false;
                    self.overlays.passcode_unlock_input.clear();
                    self.overlays.passcode_unlock_error = None;
                    self.update_preferences(cx, |p| p.local_passcode_hash = None);
                    self.push_user_settings_to_backend(cx);
                    self.enqueue_localized_toast("本地密码锁已关闭", "Local passcode disabled", 2000, cx);
                } else {
                    self.update_preferences(cx, |p| {
                        p.local_passcode_hash = hash.clone();
                        p.auto_lock_seconds = u64::from(*auto_lock_seconds);
                    });
                }
            }
            SettingsRouterEvent::UpdateGlobalTtl(_) => { self.push_user_settings_to_backend(cx); cx.notify(); }
            SettingsRouterEvent::ApplyGlobalTtlToExistingChats(ttl) => {
                let ttl_secs = *ttl;
                let direct_chat_ids: Vec<u64> = {
                    let sb = self.sidebar.read(cx);
                    let saved_id = sb.saved_messages_id;
                    sb.dialogs
                        .iter()
                        .filter(|d| {
                            Some(d.id) != saved_id
                                && d.folder != "Groups"
                                && d.folder != "Channels"
                                && !self
                                    .session_sync
                                    .rich_chats_by_id
                                    .get(&d.id)
                                    .is_some_and(|rc| {
                                        rc.is_group
                                            || rc.is_channel
                                            || matches!(rc.chat_type, 1 | 2 | 3 | 5)
                                            || rc.kind.as_deref() == Some("saved")
                                    })
                        })
                        .map(|d| d.id)
                        .collect()
                };
                for &cid in &direct_chat_ids {
                    if let Some(rc) = self.session_sync.rich_chats_by_id.get_mut(&cid) {
                        rc.message_ttl_seconds = ttl_secs;
                    }
                }
                let active_cid = self.stage.read(cx).active_chat_id;
                if active_cid.is_some_and(|cid| direct_chat_ids.contains(&cid)) {
                    self.stage.update(cx, |st, st_cx| {
                        st.message_ttl_seconds = ttl_secs;
                        st_cx.notify();
                    });
                }
                if self.is_authenticated && self.session_sync.session_token.is_some() {
                    let c = self.client.clone();
                    let ids = direct_chat_ids.clone();
                    cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
                        for cid in ids {
                            let _ = c.set_chat_ttl(cid as i64, ttl_secs).await;
                        }
                    })
                    .detach();
                }
                let msg = if self.active_lang_id().starts_with("zh") {
                    format!("已将自动删除规则应用到 {} 个私聊会话", direct_chat_ids.len())
                } else {
                    format!(
                        "Applied auto-delete timer to {} private chats",
                        direct_chat_ids.len()
                    )
                };
                self.window_runtime.enqueue_toast(msg, 2200);
                cx.notify();
            }
            SettingsRouterEvent::UpdateNotificationPrefs {
                private_chats, groups, channels, sound_enabled, flash_taskbar,
                include_muted_in_badge, badge_count_messages,
            } => {
                self.sidebar.update(cx, |sb, sb_cx| {
                    sb.set_notification_badge_rules(*badge_count_messages, *include_muted_in_badge);
                    sb_cx.notify();
                });
                self.update_preferences(cx, |p| {
                    p.notifications.private_chats = *private_chats;
                    p.notifications.groups = *groups;
                    p.notifications.channels = *channels;
                    p.notifications.sound_enabled = *sound_enabled;
                    p.notifications.flash_taskbar = *flash_taskbar;
                    p.notifications.include_muted_in_badge = *include_muted_in_badge;
                    p.notifications.badge_count_messages = *badge_count_messages;
                });
                self.push_user_settings_to_backend(cx);
            }
            SettingsRouterEvent::UpdateNotificationReactionsPrefs {
                notify_reactions,
                notify_stories,
                contacts_only,
                sound_enabled,
            } => {
                self.update_preferences(cx, |p| {
                    p.notifications.reaction_notifications = *notify_reactions;
                    p.notifications.story_reaction_notifications = *notify_stories;
                    p.notifications.reaction_contacts_only = *contacts_only;
                    p.notifications.reaction_sound_enabled = *sound_enabled;
                });
                self.push_user_settings_to_backend(cx);
            }
            SettingsRouterEvent::UpdateNotificationTypePrefs {
                alerts_enabled,
                preview_enabled,
                sound_enabled,
            } => {
                self.update_preferences(cx, |p| {
                    p.notifications.type_alerts_enabled = *alerts_enabled;
                    p.notifications.type_preview_enabled = *preview_enabled;
                    p.notifications.type_sound_enabled = *sound_enabled;
                });
                self.push_user_settings_to_backend(cx);
            }
            SettingsRouterEvent::UpdateFolderTags(on) => {
                self.sidebar.update(cx, |sb, sb_cx| {
                    sb.folder_tags_enabled = *on;
                    sb_cx.notify();
                });
                self.update_preferences(cx, |p| p.folder_tags_enabled = *on);
            }
            SettingsRouterEvent::CreateFolder { id, title } => {
                let c = self.client.clone();
                let dto = ChatFolderDto {
                    id: *id as i64, title: title.clone(),
                    include_contacts: true, include_non_contacts: true,
                    include_groups: true, include_channels: true, include_bots: true,
                    ..Default::default()
                };
                self.spawn_shell_fut(cx, async move { let _ = c.save_folder(&dto).await; }, |s, _, cx| s.refresh_folders(cx));
            }
            SettingsRouterEvent::DeleteFolder(fid) => {
                let (c, fid) = (self.client.clone(), *fid as i64);
                self.spawn_shell_fut(cx, async move { let _ = c.delete_folder(fid).await; }, |s, _, cx| s.refresh_folders(cx));
            }
            SettingsRouterEvent::UpdateAdvancedPrefs {
                ask_download_path, opengl_enabled, native_window_frame, tray_icon_enabled, auto_update_enabled,
            } => self.update_preferences(cx, |p| {
                p.ask_download_path = *ask_download_path;
                p.opengl_enabled = *opengl_enabled;
                p.native_window_frame = *native_window_frame;
                p.tray_icon_enabled = *tray_icon_enabled;
                p.auto_update_enabled = *auto_update_enabled;
            }),
            SettingsRouterEvent::OpenExportDataModal => {
                let label = tr_str(self.active_lang_id(), "导出聊天数据", "Export Chat Data");
                let title = self.stage.read(cx).chat_title.clone();
                self.open_modal_dialog(label, ModalDialogKind::ExportData { chat_title: title }, 420.0, 360.0, cx);
            }
            SettingsRouterEvent::UpdateLocalStoragePrefs { max_cache_gb, retention_weeks } =>
                self.update_preferences(cx, |p| { p.max_cache_gb = u64::from(*max_cache_gb); p.retention_weeks = *retention_weeks; }),
            SettingsRouterEvent::ClearLocalCache => self.clear_local_cache_files(cx),
            SettingsRouterEvent::UpdateCallPrefs { input_device, output_device, camera_device } =>
                self.update_preferences(cx, |p| {
                    p.audio_input_device = input_device.clone();
                    p.audio_output_device = output_device.clone();
                    p.camera_device = camera_device.clone();
                }),
            SettingsRouterEvent::ToggleMicTest(active) => {
                self.overlays.mic_test_loop_gen = self.overlays.mic_test_loop_gen.wrapping_add(1);
                let expected_gen = self.overlays.mic_test_loop_gen;
                if *active {
                    let mut start_ok = true;
                    if self.rtc_talk.active_call_id.is_none()
                        && self.rtc_talk.talk_mode_overlay.is_none()
                        && !self.rtc_talk.voice_recorder.is_recording()
                    {
                        start_ok = self.rtc_talk.voice_recorder.start().is_ok();
                    }
                    if !start_ok && !self.rtc_talk.voice_recorder.is_recording() {
                        self.enqueue_localized_toast(
                            "当前环境未检测到可用麦克风输入流",
                            "Microphone capture unavailable in current environment",
                            2200,
                            cx,
                        );
                    }
                    cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
                        loop {
                            app.background_executor()
                                .timer(std::time::Duration::from_millis(80))
                                .await;
                            let keep = weak_shell
                                .update(app, |shell, cx| {
                                    if shell.overlays.mic_test_loop_gen != expected_gen {
                                        return false;
                                    }
                                    let raw = shell.rtc_talk.voice_recorder.current_level();
                                    let norm = (f32::from(raw) / 8192.0).clamp(0.0, 1.0);
                                    let mut still_testing = false;
                                    shell.settings_router.update(cx, |sr, scx| {
                                        if let Some(ref sc) = sr.settings_calls {
                                            sc.update(scx, |sc, sc_cx| {
                                                still_testing = sc.mic_test_active;
                                                if still_testing {
                                                    sc.set_mic_level_normalized(norm);
                                                    sc_cx.notify();
                                                }
                                            });
                                        }
                                    });
                                    cx.notify();
                                    still_testing
                                })
                                .unwrap_or(false);
                            if !keep {
                                break;
                            }
                        }
                    })
                    .detach();
                } else {
                    self.stop_settings_mic_test_capture(cx);
                }
                cx.notify();
            }
            SettingsRouterEvent::UpdatePowerSavingPrefs {
                enabled, threshold_pct, animate_stickers_chat, animate_stickers_panel,
                animate_emoji_panel, autoplay_gifs, autoplay_videos,
            } => self.update_preferences(cx, |p| {
                p.power_saving_enabled = *enabled;
                p.power_saving_threshold_pct = *threshold_pct;
                p.animate_stickers_chat = *animate_stickers_chat;
                p.animate_stickers_panel = *animate_stickers_panel;
                p.animate_emoji_panel = *animate_emoji_panel;
                p.autoplay_gifs = *autoplay_gifs;
                p.autoplay_videos = *autoplay_videos;
            }),
            SettingsRouterEvent::TopupStars(amount) => {
                if !self.is_authenticated || self.session_sync.session_token.is_none() {
                    return;
                }
                let (c, amount) = (self.client.clone(), i64::from(*amount));
                self.spawn_shell_fut(cx, async move { c.wallet_topup(amount).await }, |s, res, cx| {
                    match res {
                        Ok(_) => s.enqueue_localized_toast("星币充值已到账", "Stars top-up completed", 2200, cx),
                        Err(e) => {
                            let pfx = tr_str(s.active_lang_id(), "星币充值失败", "Top-up failed");
                            s.window_runtime.enqueue_toast(format!("{pfx}: {e}"), 2600);
                            cx.notify();
                        }
                    }
                    s.refresh_wallet(cx);
                });
            }
            SettingsRouterEvent::WithdrawStars => {
                self.enqueue_localized_toast(
                    "已提交创作者收益提现申请",
                    "Creator TON withdrawal requested",
                    2200,
                    cx,
                );
            }
            SettingsRouterEvent::ActivatePremium(months) => {
                self.push_user_settings_to_backend(cx);
                let msg = if self.active_lang_id().starts_with("zh") {
                    format!("Telegram 高级会员（{months} 个月）已生效")
                } else {
                    format!("Telegram Premium ({months} mo) activated")
                };
                self.window_runtime.enqueue_toast(msg, 2200);
                cx.notify();
            }
            SettingsRouterEvent::UpdatePasskeysPrefs(names) => {
                self.update_preferences(cx, |p| p.passkey_names = names.clone());
                self.enqueue_localized_toast("通行密钥设置已保存", "Passkeys updated", 1800, cx);
            }
            SettingsRouterEvent::UpdateWebsitesPrefs(domains) => {
                self.update_preferences(cx, |p| p.connected_website_domains = domains.clone());
                self.enqueue_localized_toast("已更新授权网站列表", "Connected websites updated", 1800, cx);
            }
            SettingsRouterEvent::UpdateShortcutsPrefs { enabled, filter } => {
                self.update_preferences(cx, |p| {
                    p.shortcuts_enabled = *enabled;
                    p.shortcuts_filter = filter.clone();
                });
            }
            SettingsRouterEvent::UpdateBusinessPrefs(tools) => {
                self.update_preferences(cx, |p| p.business_configured_tools = tools.clone());
            }
            SettingsRouterEvent::SelectLanguage(code) =>
                self.update_preferences(cx, |p| p.localization.language_code = code.to_string()),
            SettingsRouterEvent::ToggleLanguage => self.toggle_language_pref(cx),
            SettingsRouterEvent::Computers(ev) => self.handle_computer_settings_event(ev, cx),
            SettingsRouterEvent::Navigated(sec) => {
                if *sec != SettingsSectionType::Calls {
                    self.stop_settings_mic_test_capture(cx);
                }
                if *sec == SettingsSectionType::Computers {
                    self.refresh_computers(cx);
                    self.sync_computers_to_views(cx);
                }
                cx.notify();
            }
        }
    }
}
