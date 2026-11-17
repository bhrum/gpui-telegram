//! AppShell: Root Desktop View Orchestration (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.1).
//!
//! Unauthenticated onboarding/login delegates to `AuthWizardView`; authenticated workspace mounts
//! Sidebar + Stage/Composer + Inspector, lazy `SettingsRouter`, and `shell_overlays_render` overlays.

pub use crate::views::app_shell::shell_state::{
    reconcile_stage_message_list, should_switch_to_chats_on_sidebar_selection, ShellComputersState,
    ShellMiniAppState, ShellOverlayState, ShellRtcTalkState, ShellSessionSyncState,
    ShellThemeVisualState,
};
pub use crate::views::app_shell::shell_truth::*;
pub use crate::views::auth::{
    AuthWizardEvent, AuthWizardTab, AuthWizardView, LoginInputField, LoginWizardStep,
};

use std::path::PathBuf;
use std::sync::Arc;

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_client::storage::Localstorage;
use gpui_telegram_client::{AuthResponse, ClientConfig, StoredSession, TelegramClient};

use crate::app::{preferences::AppPreferences, TelegramApp};
use crate::components::ui::TextInputController;
use crate::i18n::tr_str;
use crate::theme::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::app_shell::{
    build_account_record, compute_shell_palette,
    window_connecting_widget::ConnectionStatus as BannerConnectionStatus, ModalContainer,
    ShellPanelState,
};
use crate::views::nav_rail::NavDestination;
use crate::views::settings::{
    sections::SettingsLocalPasscodeSections, settings_type::SettingsSectionType, SettingsRouter,
};
use crate::views::{
    CallOverlay, CanonicalComposer, ChatSidebar, ComputerWorkbench, ContextInspector,
    ConversationStage, GlobalTasksWorkspace, MainMenuDrawer, MediaViewer, NavRail,
};
use crate::window::WindowSubsystemRuntime;

/// Top-level workspace panel state when authenticated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveWorkspacePanel {
    Chats,
    Calls,
    TaskCenter,
    McpTools,
    Settings,
}

/// Cycles the 4 built-in Telegram Desktop theme palettes (`Night -> Tinted -> Day -> Classic`).
pub fn next_tdesktop_theme_cycle(current: TdesktopThemeMode) -> TdesktopThemeMode {
    match current {
        TdesktopThemeMode::Night => TdesktopThemeMode::Tinted,
        TdesktopThemeMode::Tinted => TdesktopThemeMode::Day,
        TdesktopThemeMode::Day => TdesktopThemeMode::Classic,
        TdesktopThemeMode::Classic => TdesktopThemeMode::Night,
    }
}

/// Toggles the UI language code between `zh-CN` and `en-US`.
pub fn toggled_language_code(current: &str) -> &'static str {
    if current.starts_with("zh") { "en-US" } else { "zh-CN" }
}

/// Maps `Cmd+1..8` keystrokes to primary `NavDestination` tabs when authenticated.
pub fn nav_destination_for_shortcut_digit(key: &str) -> Option<NavDestination> {
    match key {
        "1" => Some(NavDestination::Chats),
        "2" => Some(NavDestination::Folders),
        "3" => Some(NavDestination::Calls),
        "4" => Some(NavDestination::TaskCenter),
        "5" => Some(NavDestination::McpTools),
        "6" => Some(NavDestination::TalkMode),
        "7" => Some(NavDestination::SavedMessages),
        "8" => Some(NavDestination::Settings),
        _ => None,
    }
}

/// Root GPUI desktop view orchestrating workspace views, session lifecycle, routing, and overlays.
pub struct AppShell {
    pub nav_rail: Entity<NavRail>,
    pub sidebar: Entity<ChatSidebar>,
    pub stage: Entity<ConversationStage>,
    pub composer: Entity<CanonicalComposer>,
    pub inspector: Entity<ContextInspector>,
    pub modal_container: Entity<ModalContainer>,
    pub main_menu_drawer: Entity<MainMenuDrawer>,
    pub tasks_workspace: Entity<GlobalTasksWorkspace>,
    pub auth_wizard: Entity<AuthWizardView>,
    pub settings_router: Entity<SettingsRouter>,
    pub client: Arc<TelegramClient>,
    pub preferences_dir: PathBuf,
    pub preferences: AppPreferences,
    pub local_storage: Localstorage,
    pub app: TelegramApp,
    pub window_runtime: WindowSubsystemRuntime,
    pub panel_state: ShellPanelState,
    pub is_authenticated: bool,
    pub active_panel: ActiveWorkspacePanel,
    pub last_handled_nav_dest: Option<NavDestination>,
    pub last_handled_nav_trigger: u64,
    pub focus_handle: FocusHandle,
    pub ime: TextInputController,
    pub session_sync: ShellSessionSyncState,
    pub miniapps: ShellMiniAppState,
    pub rtc_talk: ShellRtcTalkState,
    pub overlays: ShellOverlayState,
    pub computers: ShellComputersState,
    pub theme_state: ShellThemeVisualState,
}

impl AppShell {
    /// Creates an `AppShell` using the resolved default `ClientConfig`.
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self::new_with_client(Arc::new(TelegramClient::new(ClientConfig::resolve())), cx)
    }

    /// Creates an `AppShell` bound to `client`, loads persisted preferences, wires observers,
    /// and attempts startup session hydration if a stored session is present.
    pub fn new_with_client(client: Arc<TelegramClient>, cx: &mut Context<Self>) -> Self {
        client.ws_transport.set_max_reconnect_attempts(client.config.max_reconnect_attempts);
        let srv = client.config.server_url.clone();

        let nav_rail = cx.new(|_| NavRail::new());
        let sidebar = cx.new(|_| { let mut sb = ChatSidebar::new(); sb.configure_for_cloud_workspace(&srv); sb });
        let stage = cx.new(|_| { let mut st = ConversationStage::new(); st.configure_for_cloud_workspace(&srv, None); st });
        let composer = cx.new(|_| CanonicalComposer::new());
        let inspector = cx.new(|_| { let mut insp = ContextInspector::new(); insp.configure_for_cloud_workspace(&srv); insp });
        let modal_container = cx.new(|_| ModalContainer::new());
        let focus_handle = cx.focus_handle();

        let app_dir = if std::env::var_os("GPUI_TELEGRAM_CONFIG_DIR").is_some() {
            ClientConfig::get_app_dir()
        } else if let Some(cache_dir) = client.local_store.cache_dir() {
            if cache_dir.file_name().and_then(|s| s.to_str()) == Some("cache") {
                cache_dir.parent().unwrap_or(&cache_dir).to_path_buf()
            } else {
                cache_dir.to_path_buf()
            }
        } else {
            ClientConfig::get_app_dir()
        };
        let mut local_storage = Localstorage::new(&app_dir);
        let _ = local_storage.start();
        let preferences = AppPreferences::load(&app_dir, &local_storage);
        let is_chinese_locale = preferences.localization.language_code.starts_with("zh");
        let init_radius = preferences.appearance.bubble_radius;
        let (theme_mode, init_palette) =
            compute_shell_palette(&preferences, None, None, init_radius, None);
        let init_lang = if is_chinese_locale { "zh-CN" } else { "en" };

        composer.update(cx, |comp, _| {
            comp.send_on_enter = preferences.appearance.send_on_enter;
            comp.suggest_stickers_by_emoji = preferences.appearance.suggest_stickers_by_emoji;
            comp.loop_animated_stickers = preferences.appearance.loop_animated_stickers;
            comp.stickers_list.loop_animated = preferences.appearance.loop_animated_stickers;
        });
        stage.update(cx, |st, _| {
            st.loop_animated_stickers = preferences.appearance.loop_animated_stickers;
        });
        sidebar.update(cx, |sb, sb_cx| {
            sb.attach_nav_rail(nav_rail.clone(), sb_cx);
            sb.set_collapsed(preferences.appearance.sidebar_collapsed);
            sb.folder_tags_enabled = preferences.folder_tags_enabled;
            sb.set_notification_badge_rules(
                preferences.notifications.badge_count_messages,
                preferences.notifications.include_muted_in_badge,
            );
        });

        let main_menu_drawer = cx.new(|_| {
            let mut m = MainMenuDrawer::new(Vec::new(), 0);
            m.set_palette(init_palette);
            m.set_language(init_lang);
            m
        });
        let tasks_workspace = cx.new(|_| GlobalTasksWorkspace::new_live(init_palette, init_lang));
        let auth_wizard = cx.new(|cx| AuthWizardView::new(client.clone(), init_palette, is_chinese_locale, cx));
        let settings_router = cx.new(|_| SettingsRouter::new(init_palette, is_chinese_locale, preferences.clone()));

        let mut init_theme_state =
            ShellThemeVisualState::new(init_palette, theme_mode, is_chinese_locale);
        init_theme_state.bubble_radius_px = init_radius;

        let mut shell_instance = Self {
            nav_rail,
            sidebar,
            stage,
            composer,
            inspector,
            modal_container,
            main_menu_drawer,
            tasks_workspace,
            auth_wizard,
            settings_router,
            client: client.clone(),
            preferences_dir: app_dir,
            preferences,
            local_storage,
            app: TelegramApp::new(),
            window_runtime: WindowSubsystemRuntime::new(),
            panel_state: ShellPanelState::default(),
            is_authenticated: false,
            active_panel: ActiveWorkspacePanel::Chats,
            last_handled_nav_dest: Some(NavDestination::Chats),
            last_handled_nav_trigger: 0,
            focus_handle,
            ime: TextInputController::new(),
            session_sync: ShellSessionSyncState::new(),
            miniapps: ShellMiniAppState::new(),
            rtc_talk: ShellRtcTalkState::new(),
            overlays: ShellOverlayState::new(),
            computers: ShellComputersState::new(),
            theme_state: init_theme_state,
        };

        shell_instance.overlays.is_passcode_locked = shell_instance.has_local_passcode_configured();
        shell_instance.wire_workspace_observers(cx);
        shell_instance.sync_theme_and_locale_to_views(cx);
        shell_instance.hydrate_settings_from_preferences(cx);
        shell_instance.refresh_settings_truth(cx);

        if let Some(stored) = ClientConfig::load_session() {
            cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
                let verify_res = client.http_client.get_me(&stored.token).await;
                let _ = weak_shell.update(app, |shell, cx| match verify_res {
                    Ok(me) => {
                        let auth = AuthResponse {
                            token: stored.token,
                            session_id: stored.session_id,
                            user_id: me.user_id,
                            username: me.username.unwrap_or(stored.username),
                            first_name: me.first_name,
                            created_at: me.created_at,
                            expires_at: me.expires_at,
                        };
                        shell.apply_authenticated_session(auth, cx);
                    }
                    Err(_) => {
                        shell.client.clear_session();
                        shell.client.local_store.clear_all();
                        ClientConfig::clear_session();
                        shell.auth_wizard.update(cx, |w, wcx| {
                            w.set_login_error(Some("Saved session expired. Please sign in again.".to_string()));
                            wcx.notify();
                        });
                        cx.notify();
                    }
                });
            })
            .detach();
        }

        shell_instance
    }

    fn reset_conversation_views(&mut self, srv_url: &str, username: Option<&str>, cx: &mut Context<Self>) {
        self.sidebar.update(cx, |sb, sb_cx| {
            self.stage.update(sb_cx, |st, st_cx| {
                self.inspector.update(st_cx, |insp, _| {
                    reset_account_conversation_views(sb, st, insp, srv_url, username);
                });
            });
        });
        self.overlays.last_sidebar_selection_revision = self.sidebar.read(cx).selection_revision;
    }

    /// Binds an authenticated `AuthResponse`, opens WebSocket streaming, resets peer views,
    /// triggers initial server data syncs, and persists the session to disk.
    pub fn apply_authenticated_session(&mut self, auth: AuthResponse, cx: &mut Context<Self>) {
        let srv_url = self.client.config.server_url.clone();
        self.session_sync.begin_session(
            auth.token.clone(),
            auth.session_id.clone(),
            auth.user_id,
            auth.username.clone(),
        );
        self.is_authenticated = true;
        self.ime.clear_preedit();
        self.ime.just_committed_ime = false;
        self.panel_state.reset();
        self.auth_wizard.update(cx, |w, wcx| {
            w.is_authenticated = true;
            w.is_authenticating = false;
            w.set_login_error(None);
            wcx.notify();
        });
        self.client.set_auth_session(auth.clone());
        self.client.connect_ws();
        self.start_ws_inbound_stream(cx);

        let full_name = auth
            .first_name
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or(auth.username.as_str())
            .to_string();
        let acc = build_account_record(auth.user_id, &full_name, &auth.username, "", "");
        self.app.bind_authenticated_session(auth.user_id, &full_name, "");
        self.window_runtime.bind_session_user(auth.user_id, &full_name, "");
        self.window_runtime.set_connection_status(BannerConnectionStatus::Connecting, None);
        self.nav_rail.update(cx, |r, _| r.set_account(&acc.avatar_initials, &acc.display_name));
        self.main_menu_drawer.update(cx, |m, _| {
            m.accounts = vec![acc];
            m.active_account_id = auth.user_id as u32;
        });
        self.session_sync.clear_message_tracking();
        self.reset_conversation_views(&srv_url, Some(&auth.username), cx);

        let cached_folders = self.client.local_store.get_folders();
        if !cached_folders.is_empty() {
            crate::views::app_shell::shell_settings_sync::apply_folders_to_shell(
                self,
                &cached_folders,
                cx,
            );
        }

        self.refresh_chats(cx);
        self.refresh_folders(cx);
        self.refresh_contacts(cx);
        self.refresh_wallet(cx);
        self.refresh_settings_truth(cx);
        self.refresh_tasks_and_mcp(cx);
        self.refresh_hermes_commands(cx);
        self.register_current_host_and_start_relay(cx);

        let _ = ClientConfig::save_session(&StoredSession {
            server_url: srv_url,
            user_id: auth.user_id,
            username: auth.username,
            token: auth.token,
            session_id: auth.session_id,
            created_at_ms: chrono::Utc::now().timestamp_millis(),
        });
        cx.notify();
    }

    /// Clears session credentials, tears down calls/overlays/Mini Apps, resets views,
    /// returns to `AuthWizardView`, and revokes the remote session token asynchronously.
    pub fn logout(&mut self, cx: &mut Context<Self>) {
        let token = self.session_sync.session_token.clone();
        let session_id = self.session_sync.session_id.clone();
        let client = self.client.clone();
        let srv_url = self.client.config.server_url.clone();

        self.session_sync.reset_on_logout();
        self.is_authenticated = false;
        self.ime.clear_preedit();
        self.ime.just_committed_ime = false;
        self.rtc_talk.reset_on_logout();
        self.overlays.clear_overlays();
        self.miniapps.reset();
        self.computers.reset_on_logout();
        self.active_panel = ActiveWorkspacePanel::Chats;
        self.last_handled_nav_dest = Some(NavDestination::Chats);

        self.auth_wizard.update(cx, |w, wcx| { w.reset_after_logout(srv_url.clone()); wcx.notify(); });
        self.settings_router.update(cx, |r, rcx| r.close(rcx));

        self.client.local_store.clear_all();
        self.client.clear_session();
        self.app.reset_session();
        self.window_runtime.clear_session_user();
        self.window_runtime.set_connection_status(BannerConnectionStatus::Connecting, None);
        self.panel_state.reset();
        self.reset_conversation_views(&srv_url, None, cx);

        self.main_menu_drawer.update(cx, |m, _| { m.close(); m.accounts.clear(); m.active_account_id = 0; });
        self.modal_container.update(cx, |m, _| m.hide_all());
        self.nav_rail.update(cx, |r, _| {
            r.is_connected = false;
            r.set_account("?", "");
            r.set_active_destination(NavDestination::Chats);
        });
        ClientConfig::clear_session();

        if let (Some(token), Some(session_id)) = (token, session_id) {
            let http = client.http_client.clone();
            client.runtime().spawn(async move { let _ = http.revoke_session(&token, &session_id).await; });
        }
        cx.notify();
    }

    /// Returns `"zh-CN"` if Chinese locale is active, otherwise `"en"`.
    pub fn active_lang_id(&self) -> &'static str {
        if self.theme_state.is_chinese_locale || self.preferences.localization.language_code.starts_with("zh") {
            "zh-CN"
        } else {
            "en"
        }
    }

    pub fn nav_rail(&self) -> &Entity<NavRail> { &self.nav_rail }
    pub fn sidebar(&self) -> &Entity<ChatSidebar> { &self.sidebar }
    pub fn stage(&self) -> &Entity<ConversationStage> { &self.stage }
    pub fn composer(&self) -> &Entity<CanonicalComposer> { &self.composer }
    pub fn inspector(&self) -> &Entity<ContextInspector> { &self.inspector }

    fn attach_overlay_close_observer<V: 'static>(
        &self,
        overlay: &Entity<V>,
        is_closed: impl Fn(&V) -> bool + 'static,
        on_close: impl Fn(&mut Self, &mut Context<Self>) + 'static,
        cx: &mut Context<Self>,
    ) {
        cx.observe(overlay, move |shell: &mut AppShell, ov: Entity<V>, cx| {
            if is_closed(ov.read(cx)) {
                on_close(shell, cx);
                cx.notify();
            }
        })
        .detach();
    }

    fn reset_nav_destination_if(&mut self, matching: &[NavDestination], cx: &mut Context<Self>) {
        self.last_handled_nav_dest = Some(NavDestination::Chats);
        self.nav_rail.update(cx, |r, _| {
            if matching.contains(&r.active_destination) {
                r.set_active_destination(NavDestination::Chats);
            }
        });
    }

    pub fn show_talk_mode(&mut self, cx: &mut Context<Self>) {
        let peer = self.stage.read(cx).chat_title.clone();
        let overlay = cx.new(|_| CallOverlay::new(peer));
        self.attach_overlay_close_observer(&overlay, |ov| ov.should_close, Self::hide_talk_mode, cx);
        self.rtc_talk.talk_mode_overlay = Some(overlay);
    }

    pub fn hide_talk_mode(&mut self, cx: &mut Context<Self>) {
        self.rtc_talk.talk_mode_overlay = None;
        self.reset_nav_destination_if(&[NavDestination::TalkMode, NavDestination::Calls], cx);
    }

    pub fn show_remote_computer(&mut self, cx: &mut Context<Self>) {
        let overlay = cx.new(|_| ComputerWorkbench::new("Dev-Workstation-01".to_string()));
        self.attach_overlay_close_observer(&overlay, |ov| ov.should_close, Self::hide_remote_computer, cx);
        self.overlays.computer_overlay = Some(overlay);
    }

    pub fn hide_remote_computer(&mut self, cx: &mut Context<Self>) {
        self.overlays.computer_overlay = None;
        self.reset_nav_destination_if(&[NavDestination::RemoteComputer], cx);
    }

    pub fn show_media_viewer(&mut self, url: impl Into<String>, is_video: bool, cx: &mut Context<Self>) {
        let overlay = cx.new(|_| MediaViewer::new(url.into(), is_video));
        self.attach_overlay_close_observer(&overlay, |ov| ov.should_close, |shell, _| shell.hide_media_viewer(), cx);
        self.overlays.media_viewer_overlay = Some(overlay);
    }

    pub fn hide_media_viewer(&mut self) { self.overlays.media_viewer_overlay = None; }

    /// Returns the active section currently displayed inside `SettingsRouter`.
    pub fn current_settings_section(&self, cx: &App) -> SettingsSectionType {
        self.settings_router.read(cx).settings_section
    }

    /// Opens or navigates within the lazy `SettingsRouter` modal and dismisses conflicting popovers.
    pub fn navigate_settings_to(&mut self, target: SettingsSectionType, cx: &mut Context<Self>) {
        self.sidebar.update(cx, |sb, sb_cx| { sb.is_plus_menu_open = false; sb.is_account_popover_open = false; sb_cx.notify(); });
        self.stage.update(cx, |st, st_cx| { st.more_menu_open = false; st.ttl_submenu_open = false; st_cx.notify(); });

        let was_outside = self.active_panel != ActiveWorkspacePanel::Settings;
        self.settings_router.update(cx, |r, rcx| r.navigate_to(target, was_outside, rcx));
        self.active_panel = ActiveWorkspacePanel::Settings;
        self.last_handled_nav_dest = Some(NavDestination::Settings);
        self.nav_rail.update(cx, |r, _| r.set_active_destination(NavDestination::Settings));
        if was_outside {
            self.refresh_settings_truth(cx);
        }
        cx.notify();
    }

    /// Navigates back one step inside `SettingsRouter`, closing Settings when already at root.
    pub fn navigate_settings_back(&mut self, cx: &mut Context<Self>) {
        if !self.settings_router.update(cx, |r, rcx| r.navigate_back(rcx)) {
            self.close_settings(cx);
        } else {
            cx.notify();
        }
    }

    /// Closes the Settings modal, restores `ActiveWorkspacePanel::Chats`, and resets `SettingsRouter`.
    pub fn close_settings(&mut self, cx: &mut Context<Self>) {
        self.stop_settings_mic_test_capture(cx);
        let was_in_settings = self.active_panel == ActiveWorkspacePanel::Settings;
        self.active_panel = ActiveWorkspacePanel::Chats;
        self.last_handled_nav_dest = Some(NavDestination::Chats);
        self.nav_rail.update(cx, |r, _| r.set_active_destination(NavDestination::Chats));
        let needs_router_close = {
            let r = self.settings_router.read(cx);
            r.settings_section != SettingsSectionType::Main || !r.settings_stack.is_empty()
        };
        if needs_router_close {
            self.settings_router.update(cx, |r, rcx| r.close(rcx));
        }
        if was_in_settings || needs_router_close {
            cx.notify();
        }
    }

    /// Mutates `AppPreferences`, persists to disk + `Localstorage`, syncs `SettingsRouter`,
    /// updates locale flag, and propagates active theme/locale across child views.
    pub fn update_preferences<F>(&mut self, cx: &mut Context<Self>, f: F)
    where
        F: FnOnce(&mut AppPreferences),
    {
        f(&mut self.preferences);
        if let Err(e) = self.preferences.save_to_dir(&self.preferences_dir) {
            eprintln!("[AppShell] Failed to persist preferences.json: {e}");
        }
        if let Err(e) = self.preferences.sync_to_localstorage(&mut self.local_storage) {
            eprintln!("[AppShell] Failed to sync Localstorage TDF settings: {e}");
        }
        let prefs_snapshot = self.preferences.clone();
        self.theme_state.is_chinese_locale = prefs_snapshot.localization.language_code.starts_with("zh");
        self.theme_state.bubble_radius_px = prefs_snapshot.appearance.bubble_radius;
        self.settings_router.update(cx, |r, rcx| r.sync_preferences(prefs_snapshot, rcx));
        self.sync_theme_and_locale_to_views(cx);
        cx.notify();
    }

    /// Returns `true` when a non-empty local passcode hash is configured.
    pub fn has_local_passcode_configured(&self) -> bool {
        self.preferences
            .local_passcode_hash
            .as_deref()
            .is_some_and(|h| !h.is_empty())
    }

    /// Refreshes the user-activity timestamp used for idle auto-lock checks.
    pub fn record_user_activity(&mut self) {
        self.overlays.last_user_activity = std::time::Instant::now();
    }

    /// Activates the blocking passcode lockscreen if a local passcode is configured.
    pub fn lock_app_now(&mut self, cx: &mut Context<Self>) -> bool {
        if !self.has_local_passcode_configured() {
            return false;
        }
        self.overlays.is_passcode_locked = true;
        self.overlays.passcode_unlock_input.clear();
        self.overlays.passcode_unlock_error = None;
        self.ime.clear_preedit();
        self.settings_router.update(cx, |sr, scx| {
            if let Some(ref lp) = sr.settings_local_passcode {
                lp.update(scx, |lp, lpcx| {
                    lp.is_locked = true;
                    lpcx.notify();
                });
            }
        });
        cx.notify();
        true
    }

    /// Verifies `pin` against `preferences.local_passcode_hash` and unlocks on match.
    pub fn try_unlock_with_passcode(&mut self, pin: &str, cx: &mut Context<Self>) -> bool {
        let Some(expected) = self.preferences.local_passcode_hash.as_deref() else {
            self.overlays.is_passcode_locked = false;
            self.overlays.passcode_unlock_input.clear();
            self.overlays.passcode_unlock_error = None;
            cx.notify();
            return true;
        };
        if SettingsLocalPasscodeSections::verify_passcode_against_hash(pin, expected) {
            self.overlays.is_passcode_locked = false;
            self.overlays.passcode_unlock_input.clear();
            self.overlays.passcode_unlock_error = None;
            self.record_user_activity();
            self.settings_router.update(cx, |sr, scx| {
                if let Some(ref lp) = sr.settings_local_passcode {
                    lp.update(scx, |lp, lpcx| {
                        lp.is_locked = false;
                        lp.bad_tries = 0;
                        lpcx.notify();
                    });
                }
            });
            cx.notify();
            true
        } else {
            let err = tr_str(
                self.active_lang_id(),
                "本地锁定密码不正确，请重试",
                "Incorrect passcode. Please try again.",
            )
            .to_string();
            self.overlays.passcode_unlock_input.clear();
            self.overlays.passcode_unlock_error = Some(err);
            self.settings_router.update(cx, |sr, scx| {
                if let Some(ref lp) = sr.settings_local_passcode {
                    lp.update(scx, |lp, lpcx| {
                        lp.bad_tries = lp.bad_tries.saturating_add(1);
                        lpcx.notify();
                    });
                }
            });
            cx.notify();
            false
        }
    }

    /// Submits the currently typed PIN on the passcode lockscreen.
    pub fn submit_passcode_unlock_input(&mut self, cx: &mut Context<Self>) -> bool {
        let pin = self.overlays.passcode_unlock_input.clone();
        self.try_unlock_with_passcode(&pin, cx)
    }

    /// Locks the app when `auto_lock_seconds > 0` and idle duration meets/exceeds it.
    pub fn check_idle_auto_lock(&mut self, elapsed_secs: u64, cx: &mut Context<Self>) -> bool {
        if self.is_authenticated
            && !self.overlays.is_passcode_locked
            && self.has_local_passcode_configured()
            && self.preferences.auto_lock_seconds > 0
            && elapsed_secs >= self.preferences.auto_lock_seconds
        {
            return self.lock_app_now(cx);
        }
        false
    }

    fn render_toast_banner(&self, text: String, pal: TdesktopThemePalette, cx: &mut Context<Self>) -> Stateful<Div> {
        div()
            .id("window-toast-banner").cursor_pointer()
            .absolute().bottom(px(pal.scaled_px(28.0))).right(px(pal.scaled_px(28.0)))
            .px(px(pal.scaled_px(16.0))).py(px(pal.scaled_px(9.0))).rounded(px(pal.scaled_px(16.0)))
            .bg(rgb(pal.card_bg)).border_1().border_color(rgb(pal.border_fg)).shadow_lg()
            .text_size(px(pal.scaled_px(12.5))).text_color(rgb(pal.text_primary))
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                this.window_runtime.controller.dismiss_current_toast();
                cx.notify();
            }))
            .child(text)
    }
}

impl Render for AppShell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let scale_factor = (self.preferences.interface_scale_pct.clamp(50, 300) as f32) / 100.0;
        window.set_rem_size(px(16.0 * scale_factor));

        if self.is_authenticated && !self.focus_handle.is_focused(window) {
            self.focus_handle.focus(window);
        }
        let vp = window.viewport_size();
        let (vp_w, vp_h) = (f32::from(vp.width), f32::from(vp.height));
        if (vp_w - self.overlays.last_viewport_width).abs() >= 1.0 && vp_w > 0.0 {
            self.overlays.last_viewport_width = vp_w;
            self.app.sync_viewport_size(vp_w, vp_h);
        }

        let pal = self.theme_state.palette;
        let current_toast = self.window_runtime.controller.current_toast.clone();
        let update_banner = self.render_in_app_update_banner(cx);

        if !self.is_authenticated {
            return div()
                .id("app-shell-login-root")
                .size_full()
                .bg(rgb(pal.window_bg))
                .flex()
                .flex_col()
                .relative()
                .when_some(update_banner, |el, banner| el.child(banner))
                .child(
                    div()
                        .flex_1()
                        .w_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(self.auth_wizard.clone()),
                )
                .when_some(current_toast, |el, toast| el.child(self.render_toast_banner(toast.text, pal, cx)))
                .into_any_element();
        }

        let focus_handle = self.focus_handle.clone();
        let view = cx.entity().clone();
        let ime_canvas = canvas(
            |bounds, _, _| bounds,
            move |bounds, _, window, cx| {
                window.handle_input(&focus_handle, ElementInputHandler::new(bounds, view.clone()), cx);
            },
        ).size_0();

        let show_conn_banner = self.window_runtime.connecting_widget.is_visible;
        let conn_banner_text = self.window_runtime.connecting_widget.status_text();
        let (contacts_open, creation_modal, account_popover_open) = {
            let sb = self.sidebar.read(cx);
            (sb.is_contacts_modal_open, sb.active_creation_modal, sb.is_account_popover_open)
        };

        div()
            .id("app-shell-authenticated-root")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                this.handle_window_key_down(event, window, cx);
            }))
            .relative().size_full().flex().flex_col().bg(rgb(pal.window_bg))
            .child(ime_canvas)
            .when(show_conn_banner, |el| {
                el.child(
                    div().w_full().h(px(pal.scaled_px(26.0))).bg(rgb(pal.accent)).text_color(rgb(pal.active_button_fg))
                        .text_size(px(pal.scaled_px(12.0))).font_weight(FontWeight::MEDIUM)
                        .flex().items_center().justify_center().child(conn_banner_text),
                )
            })
            .when_some(update_banner, |el, banner| el.child(banner))
            .child(
                div().flex_1().w_full().min_h(px(0.0)).flex()
                    .child(self.sidebar.clone())
                    .child(
                        div().flex().flex_col().flex_1().min_w(px(0.0)).h_full().overflow_hidden()
                            .child(self.stage.clone()).child(self.composer.clone()).into_any_element(),
                    )
                    .child(self.inspector.clone()),
            )
            .when_some(self.overlays.computer_overlay.clone(), |el, ov| el.child(self.render_remote_computer_overlay(ov)))
            .when_some(self.rtc_talk.talk_mode_overlay.clone(), |el, ov| el.child(self.render_talk_mode_overlay(ov, cx)))
            .when_some(self.overlays.media_viewer_overlay.clone(), |el, ov| el.child(self.render_media_viewer_overlay(ov)))
            .when_some(self.overlays.iv_viewer_overlay.clone(), |el, ov| el.child(self.render_iv_viewer_overlay(ov)))
            .children(self.render_miniapp_overlays(cx))
            .when(contacts_open, |el| el.child(self.render_contacts_modal_overlay(cx)))
            .when_some(creation_modal, |el, kind| el.child(self.render_sidebar_creation_modal_overlay(kind, cx)))
            .when(self.active_panel == ActiveWorkspacePanel::Calls, |el| el.child(self.render_calls_modal_overlay(cx)))
            .when(self.active_panel == ActiveWorkspacePanel::Settings, |el| el.child(self.settings_router.clone().into_any_element()))
            .when(self.active_panel == ActiveWorkspacePanel::TaskCenter, |el| el.child(self.render_task_center_modal_overlay(cx)))
            .when(self.active_panel == ActiveWorkspacePanel::McpTools, |el| el.child(self.render_mcp_tools_modal_overlay(cx)))
            .when(account_popover_open, |el| el.child(self.render_account_popover_overlay(cx)))
            .child(self.modal_container.clone())
            .when(self.overlays.is_passcode_locked, |el| el.child(self.render_passcode_lock_overlay(cx)))
            .when_some(
                self.computers.pending_approvals.first().cloned(),
                |el, pending| el.child(self.render_pending_computer_approval_banner(&pending, cx)),
            )
            .when_some(current_toast, |el, toast| el.child(self.render_toast_banner(toast.text, pal, cx)))
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::stage::cards::{DeliveryStatus, MessageContent, MessageDirection, MessageItemModel};

    #[std::prelude::v1::test]
    fn test_theme_cycle_covers_all_four_tdesktop_modes() {
        let m1 = next_tdesktop_theme_cycle(TdesktopThemeMode::Night);
        let m2 = next_tdesktop_theme_cycle(m1);
        let m3 = next_tdesktop_theme_cycle(m2);
        let m4 = next_tdesktop_theme_cycle(m3);
        assert_eq!((m1, m2, m3, m4), (TdesktopThemeMode::Tinted, TdesktopThemeMode::Day, TdesktopThemeMode::Classic, TdesktopThemeMode::Night));
    }

    #[std::prelude::v1::test]
    fn test_language_toggle_and_nav_shortcut_mapping() {
        assert_eq!((toggled_language_code("zh-CN"), toggled_language_code("zh-Hans"), toggled_language_code("en-US")), ("en-US", "en-US", "zh-CN"));
        for (key, expected) in [
            ("1", Some(NavDestination::Chats)), ("2", Some(NavDestination::Folders)),
            ("3", Some(NavDestination::Calls)), ("4", Some(NavDestination::TaskCenter)),
            ("5", Some(NavDestination::McpTools)), ("6", Some(NavDestination::TalkMode)),
            ("7", Some(NavDestination::SavedMessages)), ("8", Some(NavDestination::Settings)),
            ("9", None),
        ] {
            assert_eq!(nav_destination_for_shortcut_digit(key), expected);
        }
    }

    #[std::prelude::v1::test]
    fn test_sidebar_selection_revision_gates_panel_switch_ignoring_hover() {
        let mut last_seen = 0u64;
        assert!(!should_switch_to_chats_on_sidebar_selection(0, &mut last_seen, true));
        assert_eq!(last_seen, 0);
        assert!(should_switch_to_chats_on_sidebar_selection(1, &mut last_seen, true));
        assert_eq!(last_seen, 1);
        assert!(!should_switch_to_chats_on_sidebar_selection(1, &mut last_seen, true));
    }

    #[std::prelude::v1::test]
    fn test_reconcile_stage_message_list_handles_ws_first_http_first_and_duplicates() {
        let mk = |id: u64, delivery_status: DeliveryStatus| MessageItemModel {
            id, sender_id: 10, sender_name: "Me".into(), avatar_initials: "ME".into(),
            avatar_color: 0x3498DB, direction: MessageDirection::Outgoing, timestamp: "13:45".into(),
            delivery_status, reply_to: None, reactions: vec![], bot_keyboard: None, is_pinned: false,
            content: MessageContent::Text { text: "Hello".into(), is_edited: false },
        };

        let mut ws_first = vec![mk(1_700_000_001, DeliveryStatus::Sending)];
        assert!(reconcile_stage_message_list(&mut ws_first, 1_700_000_001, 5, DeliveryStatus::Read));
        assert!(reconcile_stage_message_list(&mut ws_first, 1_700_000_001, 5, DeliveryStatus::Sent));
        assert_eq!((ws_first.len(), ws_first[0].id, ws_first[0].delivery_status), (1, 5, DeliveryStatus::Read));

        let mut http_first = vec![mk(1_700_000_002, DeliveryStatus::Sending)];
        assert!(reconcile_stage_message_list(&mut http_first, 1_700_000_002, 6, DeliveryStatus::Sent));
        assert!(reconcile_stage_message_list(&mut http_first, 1_700_000_002, 6, DeliveryStatus::Read));
        assert_eq!((http_first.len(), http_first[0].id, http_first[0].delivery_status), (1, 6, DeliveryStatus::Read));

        let mut both = vec![mk(7, DeliveryStatus::Read), mk(1_700_000_003, DeliveryStatus::Sending)];
        assert!(reconcile_stage_message_list(&mut both, 1_700_000_003, 7, DeliveryStatus::Sent));
        assert_eq!((both.len(), both[0].id, both[0].delivery_status), (1, 7, DeliveryStatus::Read));
    }
}
