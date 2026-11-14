//! AppShell: Root Desktop View Orchestration (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.1).
//!
//! Layout Structure:
//! - Interactive Telegram Desktop Login / Intro Wizard (`Start -> Phone / QR Code -> Code -> Authenticated`)
//! - Authenticated 4-Column Workspace:
//!   `[ NavRail (52px) ] [ ChatSidebar (288px) ] [ ConversationStage (flex-1) + CanonicalComposer ] [ ContextInspector (320px) ]`
//! - Overlays:
//!   - Full-Duplex Talk Mode Orb Overlay (`CallOverlay`)
//!   - Remote Computer Workbench Overlay (`ComputerWorkbench`)
//!   - Media Viewer Lightbox Overlay (`MediaViewer`)

pub use crate::views::app_shell::shell_truth::{
    apply_authoritative_chat_list, apply_fenced_chat_list_response,
    apply_fenced_history_response, derive_initials, format_authoritative_timestamp,
    historical_message_to_model, overlay_rich_message_metadata, peer_subtitle_for_dialog,
    reset_account_conversation_views, select_authoritative_dialog, try_consume_composer_for_send,
    validate_selected_send_target, SessionRequestFence, VerifiedSendTarget,
};
use crate::app::{preferences::AppPreferences, TelegramApp};
use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemeMode, TdesktopThemePalette};
use crate::views::{
    app_shell::{
        build_account_record, compute_shell_palette, render_empty_panel_notice,
        render_panel_header, toggle_desktop_theme_mode,
        window_connecting_widget::ConnectionStatus as BannerConnectionStatus, ShellPanelState,
    },
    boxes::modal_types::ModalDialogKind,
    chat_sidebar::{
        CategorizedSearchResult, CreationInputField, DialogDeliveryStatus, DialogItemModel,
        PendingAddContactRequest, PendingCreateGroupRequest,
    },
    composer::KeyAction,
    global_tasks_workspace::TasksWorkspaceTab,
    iv::IvViewer,
    nav_rail::NavDestination,
    stage::cards::{
        DeliveryStatus, MessageContent, MessageDirection, MessageItemModel,
    },
    CallOverlay, CanonicalComposer, ChatSidebar, ComputerWorkbench, ContextInspector,
    ConversationStage, GlobalTasksWorkspace, MainMenuDrawer, MediaViewer, ModalContainer, NavRail,
};
use crate::window::WindowSubsystemRuntime;
use gpui_telegram_agent::TalkCoordinator;
use gpui_telegram_media::{AudioPlaybackEngine, VoiceRecorder};
use gpui_telegram_proto::contracts::auth::ContactDto;
use gpui_telegram_webrtc::WebRtcCallEngine;
use crate::views::settings::sections::{
    SettingsActiveSessionsSections, SettingsAdvancedSections, SettingsBlockedPeersSections,
    SettingsBusinessSections, SettingsCallsSections, SettingsChatSections,
    SettingsCreditsSections, SettingsFoldersSections, SettingsGlobalTtlSections,
    SettingsInformationSections, SettingsLocalPasscodeSections, SettingsLocalStorageSections,
    SettingsMainSections, SettingsNotificationsReactionsSections, SettingsNotificationsSections,
    SettingsNotificationsTypeSections, SettingsPasskeysSections, SettingsPremiumSections,
    SettingsPrivacySecuritySections, SettingsShortcutsSections, SettingsWebsitesSections,
};
use crate::views::settings::settings_power_saving::SettingsPowerSaving;
use crate::views::settings::settings_search::SettingsSearch;
use crate::views::settings::settings_type::SettingsSectionType;
use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_telegram_client::storage::Localstorage;

use gpui_telegram_client::{
    net::{
        hermes_client::HermesRichChatDto,
        http_client::{
            AddContactHttpRequest, AlipayPollResponse, AlipayStartResponse, EmailRegisterRequest,
            EmailSendCodeRequest,
        },
        RichSendMessageRequest,
    },
    AuthResponse, ClientConfig, ConnectionStatus, HttpClientError, LoginRequest,
    RegisterRequest, StoredSession, SyncOutcome, TelegramClient,
};
use gpui_telegram_core::types::ChatId;
use gpui_telegram_proto::sync::{
    PtsUpdate, SyncGetDifferenceRequest, SyncGetDifferenceResponse, UpdateEvent,
};
use gpui_telegram_proto::ws::{WsClientFrame, WsServerFrame};
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;

fn utf16_len(s: &str) -> usize {
    s.encode_utf16().count()
}

fn utf16_offset_to_utf8(s: &str, utf16_offset: usize) -> usize {
    let mut current_utf16 = 0;
    for (byte_idx, ch) in s.char_indices() {
        if current_utf16 >= utf16_offset {
            return byte_idx;
        }
        current_utf16 += ch.len_utf16();
    }
    s.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginWizardStep {
    Start,
    Phone,
    QrCode,
    Code,
}

/// Multi-tab navigation modes for the Login Wizard card (Milestone 2 F8).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AuthWizardTab {
    #[default]
    PasswordLogin,
    AlipayQr,
    EmailRegister,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginInputField {
    Username,
    Password,
    FirstName,
    Email,
    EmailCode,
}

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
    if current.starts_with("zh") {
        "en-US"
    } else {
        "zh-CN"
    }
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

pub struct AppShell {
    pub nav_rail: Entity<NavRail>,
    pub sidebar: Entity<ChatSidebar>,
    pub stage: Entity<ConversationStage>,
    pub composer: Entity<CanonicalComposer>,
    pub inspector: Entity<ContextInspector>,
    pub modal_container: Entity<ModalContainer>,
    pub talk_mode_overlay: Option<Entity<CallOverlay>>,
    pub computer_overlay: Option<Entity<ComputerWorkbench>>,
    pub media_viewer_overlay: Option<Entity<MediaViewer>>,
    pub is_authenticated: bool,
    pub login_step: LoginWizardStep,
    pub country_code: String,
    pub phone_number: String,
    pub verification_code: String,
    pub login_error: Option<String>,
    pub is_chinese_locale: bool,
    pub active_panel: ActiveWorkspacePanel,
    pub last_handled_nav_dest: Option<NavDestination>,
    pub last_handled_nav_trigger: u64,
    pub focus_handle: FocusHandle,
    pub next_message_id: u64,
    pub client: Arc<TelegramClient>,
    pub session_token: Option<String>,
    pub session_id: Option<String>,
    pub user_id: Option<u64>,
    pub username: Option<String>,
    pub is_authenticating: bool,
    pub username_input: String,
    pub password_input: String,
    pub first_name_input: String,
    pub is_register_mode: bool,
    pub active_login_field: LoginInputField,
    pub ime_preedit: Option<String>,
    pub just_committed_ime: bool,
    pub ws_stream_started: bool,
    pub last_dispatched_search_seq: u64,

    // --- Milestone 2 (F8) Login Wizard Additions ---
    pub auth_wizard_tab: AuthWizardTab,

    pub editing_server_url: bool,
    pub server_url_draft: String,

    // Alipay QR Login State
    pub alipay_attempt_id: Option<String>,
    pub alipay_auth_url: Option<String>,
    pub alipay_qr_payload: Option<String>,
    pub alipay_status_text: String,
    pub alipay_is_loading: bool,
    pub alipay_poll_generation: u64,

    // Email Registration State
    pub email_input: String,
    pub email_code_input: String,
    pub email_code_cooldown: u32,
    pub email_cooldown_generation: u64,
    pub is_sending_email_code: bool,

    // Milestone 3: Feature F9, F10, F11 Settings Router, Materialization & Preferences
    pub settings_section: SettingsSectionType,
    pub settings_stack: Vec<SettingsSectionType>,
    pub settings_main: Entity<SettingsMainSections>,
    pub settings_information: Entity<SettingsInformationSections>,
    pub settings_notifications: Entity<SettingsNotificationsSections>,
    pub settings_notif_reactions: Entity<SettingsNotificationsReactionsSections>,
    pub settings_notif_type: Entity<SettingsNotificationsTypeSections>,
    pub settings_privacy: Entity<SettingsPrivacySecuritySections>,
    pub settings_blocked_peers: Entity<SettingsBlockedPeersSections>,
    pub settings_active_sessions: Entity<SettingsActiveSessionsSections>,
    pub settings_local_passcode: Entity<SettingsLocalPasscodeSections>,
    pub settings_passkeys: Entity<SettingsPasskeysSections>,
    pub settings_global_ttl: Entity<SettingsGlobalTtlSections>,
    pub settings_websites: Entity<SettingsWebsitesSections>,
    pub settings_chat: Entity<SettingsChatSections>,
    pub settings_folders: Entity<SettingsFoldersSections>,
    pub settings_advanced: Entity<SettingsAdvancedSections>,
    pub settings_local_storage: Entity<SettingsLocalStorageSections>,
    pub settings_power_saving: Entity<SettingsPowerSaving>,
    pub settings_calls: Entity<SettingsCallsSections>,
    pub settings_credits: Entity<SettingsCreditsSections>,
    pub settings_premium: Entity<SettingsPremiumSections>,
    pub settings_shortcuts: Entity<SettingsShortcutsSections>,
    pub settings_business: Entity<SettingsBusinessSections>,
    pub settings_search: Entity<SettingsSearch>,
    pub preferences: AppPreferences,
    pub local_storage: Localstorage,
    pub main_menu_drawer: Entity<MainMenuDrawer>,
    pub iv_viewer_overlay: Option<Entity<IvViewer>>,
    pub tasks_workspace: Entity<GlobalTasksWorkspace>,
    pub app: TelegramApp,
    pub window_runtime: WindowSubsystemRuntime,
    pub palette: TdesktopThemePalette,
    pub theme_mode: TdesktopThemeMode,
    pub accent_color: Option<u32>,
    pub bubble_radius_px: u32,
    pub wallpaper_id: Option<String>,
    pub panel_state: ShellPanelState,
    pub auth_generation: u64,
    pub chat_list_request_generation: u64,
    pub history_request_generation: u64,
    pub audio_player: AudioPlaybackEngine,
    pub voice_recorder: VoiceRecorder,
    pub webrtc_engine: WebRtcCallEngine,
    pub active_call_id: Option<String>,
    pub contacts_list: Vec<ContactDto>,
    pub rich_chats_by_id: HashMap<u64, HermesRichChatDto>,
    pub chat_peer_user_ids: HashMap<u64, u64>,
    pub talk_coordinator: TalkCoordinator,
    pub active_talk_session_is_ai: bool,
    pub active_talk_bot_chat_id: Option<u64>,
    pub active_talk_session_id: Option<String>,
    pub last_viewport_width: f32,
    pub last_sidebar_selection_revision: u64,
    pub pending_outgoing_nonces: HashMap<u64, u64>,
    pub resolved_outgoing_nonces: HashMap<u64, u64>,
}

fn should_switch_to_chats_on_sidebar_selection(
    current_revision: u64,
    last_seen_revision: &mut u64,
    has_selected_dialog: bool,
) -> bool {
    if current_revision == *last_seen_revision {
        return false;
    }
    *last_seen_revision = current_revision;
    has_selected_dialog
}

fn delivery_status_rank(status: DeliveryStatus) -> u8 {
    match status {
        DeliveryStatus::Sending => 0,
        DeliveryStatus::Sent => 1,
        DeliveryStatus::Read => 2,
    }
}

fn max_delivery_status(a: DeliveryStatus, b: DeliveryStatus) -> DeliveryStatus {
    if delivery_status_rank(a) >= delivery_status_rank(b) {
        a
    } else {
        b
    }
}

/// Promotes an optimistic message (`local_id`) to authoritative `server_id` in place,
/// preserving the highest delivery status and collapsing any duplicate with `server_id`.
fn reconcile_stage_message_list(
    list: &mut Vec<MessageItemModel>,
    local_id: u64,
    server_id: u64,
    min_status: DeliveryStatus,
) -> bool {
    let existing_server_idx = (server_id > 0)
        .then(|| list.iter().position(|m| m.id == server_id))
        .flatten();
    let local_idx = list.iter().position(|m| m.id == local_id);

    match (local_idx, existing_server_idx) {
        (Some(l_idx), Some(s_idx)) if l_idx != s_idx => {
            let local_status = list[l_idx].delivery_status;
            list[s_idx].delivery_status = max_delivery_status(
                max_delivery_status(list[s_idx].delivery_status, local_status),
                min_status,
            );
            list.remove(l_idx);
            true
        }
        (Some(l_idx), _) => {
            if server_id > 0 {
                list[l_idx].id = server_id;
            }
            list[l_idx].delivery_status =
                max_delivery_status(list[l_idx].delivery_status, min_status);
            true
        }
        (None, Some(s_idx)) => {
            list[s_idx].delivery_status =
                max_delivery_status(list[s_idx].delivery_status, min_status);
            true
        }
        (None, None) => false,
    }
}


impl AppShell {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let config = ClientConfig::resolve();
        let client = Arc::new(TelegramClient::new(config));
        Self::new_with_client(client, cx)
    }

    pub fn new_with_client(client: Arc<TelegramClient>, cx: &mut Context<Self>) -> Self {
        client
            .ws_transport
            .set_max_reconnect_attempts(client.config.max_reconnect_attempts);
        let server_url_init = client.config.server_url.clone();
        let nav_rail = cx.new(|_| NavRail::new());
        let sidebar = cx.new(|_| {
            let mut sb = ChatSidebar::new();
            sb.configure_for_cloud_workspace(&server_url_init);
            sb
        });
        let stage = cx.new(|_| {
            let mut st = ConversationStage::new();
            st.configure_for_cloud_workspace(&server_url_init, None);
            st
        });
        let composer = cx.new(|_| CanonicalComposer::new());
        let inspector = cx.new(|_| {
            let mut insp = ContextInspector::new();
            insp.configure_for_cloud_workspace(&server_url_init);
            insp
        });
        let modal_container = cx.new(|_| ModalContainer::new());
        let focus_handle = cx.focus_handle();

        let settings_main = cx.new(|_| SettingsMainSections::new());
        let settings_information = cx.new(|_| SettingsInformationSections::new());
        let settings_notifications = cx.new(|_| SettingsNotificationsSections::new());
        let settings_notif_reactions = cx.new(|_| SettingsNotificationsReactionsSections::new());
        let settings_notif_type = cx.new(|_| SettingsNotificationsTypeSections::new("All Chats"));
        let settings_privacy = cx.new(|_| SettingsPrivacySecuritySections::new());
        let settings_blocked_peers = cx.new(|_| SettingsBlockedPeersSections::new());
        let settings_active_sessions = cx.new(|_| SettingsActiveSessionsSections::new());
        let settings_local_passcode = cx.new(|_| SettingsLocalPasscodeSections::new());
        let settings_passkeys = cx.new(|_| SettingsPasskeysSections::new());
        let settings_global_ttl = cx.new(|_| SettingsGlobalTtlSections::new());
        let settings_websites = cx.new(|_| SettingsWebsitesSections::new());
        let settings_chat = cx.new(|_| SettingsChatSections::new());
        let settings_folders = cx.new(|_| SettingsFoldersSections::new());
        let settings_advanced = cx.new(|_| SettingsAdvancedSections::new());
        let settings_local_storage = cx.new(|_| SettingsLocalStorageSections::new());
        let settings_power_saving = cx.new(|_| SettingsPowerSaving::new());
        let settings_calls = cx.new(|_| SettingsCallsSections::new());
        let settings_credits = cx.new(|_| SettingsCreditsSections::new());
        let settings_premium = cx.new(|_| SettingsPremiumSections::new());
        let settings_shortcuts = cx.new(|_| SettingsShortcutsSections::new());
        let settings_business = cx.new(|_| SettingsBusinessSections::new());
        let settings_search = cx.new(|_| SettingsSearch::with_default_entries());

        // Observe SettingsMainSections navigation clicks
        cx.observe(&settings_main, |shell: &mut AppShell, main: Entity<SettingsMainSections>, cx: &mut Context<AppShell>| {
            if let Some(target) = main.update(cx, |m, _| m.active_section.take()) {
                shell.navigate_settings_to(target, cx);
            }
        }).detach();

        // Hydrate preferences from disk & Localstorage
        let app_dir = ClientConfig::get_app_dir();
        let mut local_storage = Localstorage::new(&app_dir);
        let _ = local_storage.start();
        let preferences = AppPreferences::load(&app_dir, &local_storage);
        let is_chinese_locale = preferences.localization.language_code.starts_with("zh");
        let (theme_mode, init_palette) = compute_shell_palette(&preferences, None, None, 12, None);
        let init_lang = if is_chinese_locale { "zh-CN" } else { "en" };

        let main_menu_drawer = cx.new(|_| {
            let mut m = MainMenuDrawer::new(Vec::new(), 0);
            m.set_palette(init_palette);
            m.set_language(init_lang);
            m
        });
        let tasks_workspace = cx.new(|_| GlobalTasksWorkspace::new_live(init_palette, init_lang));

        cx.observe(&main_menu_drawer, |shell: &mut AppShell, drawer: Entity<MainMenuDrawer>, cx: &mut Context<AppShell>| {
            if let Some(cmd) = drawer.update(cx, |d, _| d.take_command()) {
                shell.handle_main_menu_command(cmd, cx);
            }
        }).detach();
        cx.observe(&modal_container, |shell: &mut AppShell, modal: Entity<ModalContainer>, cx: &mut Context<AppShell>| {
            if let Some(outcome) = modal.update(cx, |m, _| m.take_outcome()) {
                shell.handle_modal_outcome(outcome, cx);
            }
        }).detach();
        cx.observe(&tasks_workspace, |shell: &mut AppShell, tw: Entity<GlobalTasksWorkspace>, cx: &mut Context<AppShell>| {
            if let Some(act) = tw.update(cx, |w, _| w.take_action()) {
                shell.handle_tasks_workspace_action(act, cx);
            }
        }).detach();
        cx.observe(&settings_chat, |shell: &mut AppShell, sc: Entity<SettingsChatSections>, cx: &mut Context<AppShell>| {
            if let Some((mode, accent, wallpaper)) = sc.update(cx, |s, _| s.take_theme_change()) {
                shell.apply_theme_selection(mode, Some(accent), Some(wallpaper), cx);
            }
        }).detach();
        cx.observe(&settings_search, |shell: &mut AppShell, ss: Entity<SettingsSearch>, cx: &mut Context<AppShell>| {
            if let Some(target) = ss.update(cx, |s, _| s.take_selected_section()) {
                shell.navigate_settings_to(target, cx);
            }
        }).detach();

        // 1. Observe NavRail clicks (Destinations, Overlays, Logout, MainMenu, Theme, Folders)
        cx.observe(&nav_rail, |shell: &mut AppShell, rail: Entity<NavRail>, cx: &mut Context<AppShell>| {
            let (dest, logout_req, trigger, menu_req, archive_req, theme_req, folder_req) = rail.update(cx, |r, _| {
                let f = r.selected_folder_requested.take().and_then(|fid| {
                    r.custom_folders.iter().find(|item| item.folder_id == fid).map(|item| item.title.clone())
                });
                let mr = if r.menu_open_requested { r.menu_open_requested = false; true } else { false };
                let ar = if r.archive_open_requested { r.archive_open_requested = false; true } else { false };
                let tr = if r.theme_toggle_requested { r.theme_toggle_requested = false; true } else { false };
                (r.active_destination, r.logout_requested, r.trigger_counter, mr, ar, tr, f)
            });
            if menu_req {
                shell.main_menu_drawer.update(cx, |d, _| d.toggle());
                cx.notify();
            }
            if archive_req {
                shell.active_panel = ActiveWorkspacePanel::Chats;
                shell.sidebar.update(cx, |sb, _| { sb.is_archived_expanded = !sb.is_archived_expanded; });
                cx.notify();
            }
            if theme_req {
                let next = toggle_desktop_theme_mode(shell.theme_mode);
                shell.apply_theme_selection(next, None, None, cx);
            }
            if let Some(folder_title) = folder_req {
                shell.active_panel = ActiveWorkspacePanel::Chats;
                shell.sidebar.update(cx, |sb, sb_cx| { sb.select_folder(folder_title); sb_cx.notify(); });
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

            // Click-intent & Change-detection Guard:
            // Background notifications (e.g. WebSocket connection updates,
            // unread count updates, or badge updates) notify `nav_rail` but DO NOT
            // increment `trigger_counter`.
            // Ignore if trigger hasn't changed AND destination hasn't changed.
            if shell.last_handled_nav_trigger == trigger && shell.last_handled_nav_dest == Some(dest) {
                return;
            }
            shell.last_handled_nav_trigger = trigger;
            shell.activate_nav_destination(dest, cx);
        }).detach();


        // 2. Observe ChatSidebar clicks (Dialog selection, Story click, Search focus, + Creation Modal, Categorized Search, MainMenu, Context Actions)
        cx.observe(&sidebar, |shell: &mut AppShell, sb: Entity<ChatSidebar>, cx: &mut Context<AppShell>| {
            let (
                selected_dialog,
                selection_rev,
                story_peer,
                search_focused,
                has_creation_modal,
                pending_contact,
                pending_group,
                search_query,
                search_cat,
                search_seq,
                open_menu,
                context_action,
            ) = sb.update(cx, |s, _| {
                (
                    s.selected_dialog().cloned(),
                    s.selection_revision,
                    s.story_clicked_peer.take(),
                    s.search_focused,
                    s.active_creation_modal.is_some(),
                    s.pending_add_contact_request.take(),
                    s.pending_create_group_request.take(),
                    s.search_query.clone(),
                    s.active_search_category.clone(),
                    s.search_request_seq,
                    s.take_open_main_menu_requested(),
                    s.take_pending_context_action(),
                )
            });
            if open_menu {
                shell.main_menu_drawer.update(cx, |d, _| d.toggle());
            }
            if let Some(action) = context_action {
                shell.handle_sidebar_context_action(action, cx);
            }
            if search_focused || has_creation_modal {
                shell.composer.update(cx, |c, _| c.is_input_focused = false);
            }
            if let Some(contact_req) = pending_contact {
                shell.dispatch_add_contact(contact_req, cx);
            }
            if let Some(group_req) = pending_group {
                shell.dispatch_create_group(group_req, cx);
            }
            if search_focused && search_seq > 0 && shell.last_dispatched_search_seq != search_seq {
                shell.dispatch_categorized_search(search_query, search_cat, search_seq, cx);
            }
            if let Some(peer) = story_peer {
                let title = peer.clone();
                shell.open_modal_dialog(title, ModalDialogKind::StoriesViewer { peer_name: peer }, 420.0, 560.0, cx);
            }
            if should_switch_to_chats_on_sidebar_selection(
                selection_rev,
                &mut shell.last_sidebar_selection_revision,
                selected_dialog.is_some(),
            ) {
                if let Some(d) = selected_dialog {
                    shell.active_panel = ActiveWorkspacePanel::Chats;
                    shell.last_handled_nav_dest = Some(NavDestination::Chats);
                    shell.nav_rail.update(cx, |r, _| {
                        r.set_active_destination(NavDestination::Chats);
                    });
                    let needs_sync = shell.stage.read(cx).active_chat_id != Some(d.id)
                        || shell.stage.read(cx).chat_title != d.peer_name;
                    if needs_sync {
                        shell.sync_selected_chat_inspector_and_stage(d.id, cx);
                    }
                }
            }
            cx.notify();
        }).detach();

        // 3. Observe ConversationStage clicks (Call, Media, IV, Edit, Forward, Delete, Pin, Poll, Reactions, Voice, Files, Inspector)
        cx.observe(&stage, |shell: &mut AppShell, st: Entity<ConversationStage>, cx: &mut Context<AppShell>| {
            shell.handle_stage_pending_actions(cx);
            let insp_open = st.read(cx).is_inspector_open;
            shell.inspector.update(cx, |insp, insp_cx| {
                if insp.is_open != insp_open {
                    insp.is_open = insp_open;
                    insp_cx.notify();
                }
            });
            cx.notify();
        }).detach();

        // 4. Observe CanonicalComposer (Send message, Silent/Scheduled, Edit, Attachments, Stickers, Polls, Voice, Input focus)
        cx.observe(&composer, |shell: &mut AppShell, comp: Entity<CanonicalComposer>, cx: &mut Context<AppShell>| {
            let (att_opt, focused) = {
                let c = comp.read(cx);
                (c.submitted_attachment.clone(), c.is_input_focused)
            };
            if focused {
                shell.sidebar.update(cx, |sb, _| sb.search_focused = false);
            }
            if let Some(kind) = att_opt {
                comp.update(cx, |c, _| c.submitted_attachment = None);
                shell.append_outgoing_attachment(&kind, cx);
            }
            shell.handle_composer_pending_actions(cx);
            cx.notify();
        }).detach();

        // 5. Observe ContextInspector close button, shared media clicks, and InspectorActions
        cx.observe(&inspector, |shell: &mut AppShell, insp: Entity<ContextInspector>, cx: &mut Context<AppShell>| {
            let (is_open, media_open, action) = insp.update(cx, |i, _| {
                (i.is_open, i.pending_media_open.take(), i.take_action())
            });
            if let Some((url, is_video)) = media_open {
                shell.show_media_viewer(url, is_video, cx);
            }
            if let Some(act) = action {
                shell.handle_inspector_action(act, cx);
            }
            shell.stage.update(cx, |st, _| {
                st.is_inspector_open = is_open;
            });
            cx.notify();
        }).detach();

        let mut shell_instance = Self {
            nav_rail,
            sidebar,
            stage,
            composer,
            inspector,
            modal_container,
            talk_mode_overlay: None,
            computer_overlay: None,
            media_viewer_overlay: None,
            is_authenticated: false,
            login_step: LoginWizardStep::Start,
            country_code: "+86".to_string(),
            phone_number: String::new(),
            verification_code: String::new(),
            login_error: None,
            is_chinese_locale,
            active_panel: ActiveWorkspacePanel::Chats,
            last_handled_nav_dest: Some(NavDestination::Chats),
            last_handled_nav_trigger: 0,
            focus_handle,
            next_message_id: 900,
            client: client.clone(),
            session_token: None,
            session_id: None,
            user_id: None,
            username: None,
            is_authenticating: false,
            username_input: String::new(),
            password_input: String::new(),
            first_name_input: String::new(),
            is_register_mode: false,
            active_login_field: LoginInputField::Username,
            ime_preedit: None,
            just_committed_ime: false,
            ws_stream_started: false,
            last_dispatched_search_seq: 0,
            auth_wizard_tab: AuthWizardTab::PasswordLogin,
            editing_server_url: false,
            server_url_draft: client.config.server_url.clone(),
            alipay_attempt_id: None,
            alipay_auth_url: None,
            alipay_qr_payload: None,
            alipay_status_text: String::new(),
            alipay_is_loading: false,
            alipay_poll_generation: 0,
            email_input: String::new(),
            email_code_input: String::new(),
            email_code_cooldown: 0,
            email_cooldown_generation: 0,
            is_sending_email_code: false,
            settings_section: SettingsSectionType::Main,
            settings_stack: Vec::new(),
            settings_main,
            settings_information,
            settings_notifications,
            settings_notif_reactions,
            settings_notif_type,
            settings_privacy,
            settings_blocked_peers,
            settings_active_sessions,
            settings_local_passcode,
            settings_passkeys,
            settings_global_ttl,
            settings_websites,
            settings_chat,
            settings_folders,
            settings_advanced,
            settings_local_storage,
            settings_power_saving,
            settings_calls,
            settings_credits,
            settings_premium,
            settings_shortcuts,
            settings_business,
            settings_search,
            preferences,
            local_storage,
            main_menu_drawer,
            iv_viewer_overlay: None,
            tasks_workspace,
            app: TelegramApp::new(),
            window_runtime: WindowSubsystemRuntime::new(),
            palette: init_palette,
            theme_mode,
            accent_color: None,
            bubble_radius_px: 12,
            wallpaper_id: None,
            panel_state: ShellPanelState::default(),
            auth_generation: 0,
            chat_list_request_generation: 0,
            history_request_generation: 0,
            audio_player: AudioPlaybackEngine::new(),
            voice_recorder: VoiceRecorder::new(16_000),
            webrtc_engine: WebRtcCallEngine::default(),
            active_call_id: None,
            contacts_list: Vec::new(),
            rich_chats_by_id: HashMap::new(),
            chat_peer_user_ids: HashMap::new(),
            talk_coordinator: TalkCoordinator::new(),
            active_talk_session_is_ai: false,
            active_talk_bot_chat_id: None,
            active_talk_session_id: None,
            last_viewport_width: 1280.0,
            last_sidebar_selection_revision: 0,
            pending_outgoing_nonces: HashMap::new(),
            resolved_outgoing_nonces: HashMap::new(),
        };
        shell_instance.sync_theme_and_locale_to_views(cx);

        // Hydrate stored session on startup if present
        if let Some(stored) = ClientConfig::load_session() {
            let client_clone = client.clone();
            let token = stored.token.clone();
            let session_id = stored.session_id.clone();
            let username = stored.username.clone();

            cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
                let verify_res = client_clone.http_client.get_me(&token).await;
                let _ = weak_shell.update(app, |shell, cx| {
                    match verify_res {
                        Ok(me) => {
                            let resolved_user = me.username.or(Some(username));
                            let srv_url = shell.client.config.server_url.clone();
                            let auth = AuthResponse {
                                token: token.clone(),
                                session_id: session_id.clone(),
                                user_id: me.user_id,
                                username: resolved_user.clone().unwrap_or_default(),
                                first_name: me.first_name.clone(),
                                created_at: me.created_at.clone(),
                                expires_at: me.expires_at.clone(),
                            };
                            shell.apply_authenticated_session(auth, cx);
                            let _ = srv_url;
                        }
                        Err(_) => {
                            ClientConfig::clear_session();
                            shell.login_error =
                                Some("Saved session expired. Please sign in again.".to_string());
                            cx.notify();
                        }
                    }
                });
            })
            .detach();
        }

        shell_instance
    }

    pub fn apply_authenticated_session(&mut self, auth: AuthResponse, cx: &mut Context<Self>) {
        let srv_url = self.client.config.server_url.clone();
        self.auth_generation = self.auth_generation.wrapping_add(1);
        self.chat_list_request_generation = self.chat_list_request_generation.wrapping_add(1);
        self.history_request_generation = self.history_request_generation.wrapping_add(1);
        self.is_authenticated = true;
        self.session_token = Some(auth.token.clone());
        self.session_id = Some(auth.session_id.clone());
        self.user_id = Some(auth.user_id);
        self.username = Some(auth.username.clone());
        self.ime_preedit = None;
        self.panel_state.reset();
        self.client.set_auth_session(auth.clone());
        self.client.connect_ws();
        self.start_ws_inbound_stream(cx);

        let full_name = match auth.first_name.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            Some(name) => name.to_string(),
            None => auth.username.clone(),
        };
        let acc = build_account_record(auth.user_id, &full_name, &auth.username, "", "");
        self.app.bind_authenticated_session(auth.user_id, &full_name, "");
        self.window_runtime.bind_session_user(auth.user_id, &full_name, "");
        self.window_runtime.set_connection_status(BannerConnectionStatus::Connecting, None);
        self.nav_rail.update(cx, |r, _| r.set_account(&acc.avatar_initials, &acc.display_name));
        self.main_menu_drawer.update(cx, |m, _| {
            m.accounts = vec![acc];
            m.active_account_id = auth.user_id as u32;
        });
        self.pending_outgoing_nonces.clear();
        self.resolved_outgoing_nonces.clear();
        self.sidebar.update(cx, |sb, sb_cx| {
            self.stage.update(sb_cx, |st, st_cx| {
                self.inspector.update(st_cx, |insp, _| {
                    reset_account_conversation_views(sb, st, insp, &srv_url, Some(&auth.username));
                });
            });
        });
        self.last_sidebar_selection_revision = self.sidebar.read(cx).selection_revision;

        self.refresh_chats(cx);
        self.refresh_folders(cx);
        self.refresh_contacts(cx);
        self.refresh_wallet(cx);
        self.refresh_tasks_and_mcp(cx);
        self.refresh_hermes_commands(cx);

        let stored = StoredSession {
            server_url: srv_url,
            user_id: auth.user_id,
            username: auth.username,
            token: auth.token,
            session_id: auth.session_id,
            created_at_ms: chrono::Utc::now().timestamp_millis(),
        };
        let _ = ClientConfig::save_session(&stored);
        cx.notify();
    }

    pub fn submit_login(&mut self, cx: &mut Context<Self>) {
        let username = self.username_input.trim().to_string();
        let password = self.password_input.clone();
        if username.is_empty() || password.is_empty() {
            self.login_error = Some("Username and password cannot be empty".to_string());
            cx.notify();
            return;
        }

        self.is_authenticating = true;
        self.login_error = None;
        cx.notify();

        let client = self.client.clone();
        let req = LoginRequest {
            username: username.clone(),
            password,
            device_name: "GPUI Desktop Client".to_string(),
        };

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.login(&req).await;
            let _ = weak_shell.update(app, |shell, cx| {
                shell.is_authenticating = false;
                match res {
                    Ok(auth) => {
                        shell.apply_authenticated_session(auth, cx);
                    }
                    Err(err) => {
                        shell.login_error = Some(match err {
                            HttpClientError::Unauthorized(_) => {
                                "Invalid username or password (401)".to_string()
                            }
                            other => other.to_string(),
                        });
                        cx.notify();
                    }
                }
            });
        })
        .detach();
    }

    pub fn submit_register(&mut self, cx: &mut Context<Self>) {
        let username = self.username_input.trim().to_string();
        let password = self.password_input.clone();
        let first_name = if self.first_name_input.trim().is_empty() {
            username.clone()
        } else {
            self.first_name_input.trim().to_string()
        };

        if username.is_empty() || password.is_empty() {
            self.login_error = Some("Username and password cannot be empty".to_string());
            cx.notify();
            return;
        }

        self.is_authenticating = true;
        self.login_error = None;
        cx.notify();

        let client = self.client.clone();
        let req = RegisterRequest {
            username: username.clone(),
            password,
            first_name,
            device_name: "GPUI Desktop Client".to_string(),
        };

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.register(&req).await;
            let _ = weak_shell.update(app, |shell, cx| {
                shell.is_authenticating = false;
                match res {
                    Ok(auth) => {
                        shell.apply_authenticated_session(auth, cx);
                    }
                    Err(err) => {
                        shell.login_error = Some(match err {
                            HttpClientError::Conflict(_) => {
                                format!("Username '{}' is already registered (409)", username)
                            }
                            other => other.to_string(),
                        });
                        cx.notify();
                    }
                }
            });
        })
        .detach();
    }

    pub fn switch_auth_tab(&mut self, tab: AuthWizardTab, cx: &mut Context<Self>) {
        if self.auth_wizard_tab == tab {
            return;
        }
        self.auth_wizard_tab = tab;
        self.login_error = None;
        self.ime_preedit = None;

        match tab {
            AuthWizardTab::PasswordLogin => {
                self.active_login_field = LoginInputField::Username;
            }
            AuthWizardTab::AlipayQr => {
                self.start_alipay_flow(cx);
            }
            AuthWizardTab::EmailRegister => {
                self.active_login_field = LoginInputField::Email;
            }
        }
        cx.notify();
    }

    pub fn apply_custom_server_url(&mut self, cx: &mut Context<Self>) {
        let raw = self.server_url_draft.trim().to_string();
        if raw.is_empty() {
            self.editing_server_url = false;
            cx.notify();
            return;
        }
        let normalized = gpui_telegram_client::config::ClientConfig::save_server_url(&raw)
            .unwrap_or_else(|_| gpui_telegram_client::config::ClientConfig::normalize_url(&raw));
        let mut new_cfg = self.client.config.clone();
        new_cfg.server_url = normalized.clone();
        self.client = Arc::new(TelegramClient::new(new_cfg));
        self.server_url_draft = normalized;
        self.editing_server_url = false;
        self.login_error = None;
        if self.auth_wizard_tab == AuthWizardTab::AlipayQr {
            self.start_alipay_flow(cx);
        } else {
            cx.notify();
        }
    }

    fn render_real_alipay_qr_matrix(
        payload: Option<&str>,
        is_loading: bool,
        zh: bool,
    ) -> AnyElement {
        if let Some(url) = payload.filter(|s| !s.trim().is_empty()) {
            if let Ok(code) = qrcode::QrCode::with_error_correction_level(
                url.as_bytes(),
                qrcode::EcLevel::Q,
            ) {
                let n = code.width();
                let colors = code.into_colors();
                let module_px: f32 = if n <= 45 {
                    4.0
                } else if n <= 65 {
                    3.0
                } else {
                    2.5
                };
                let grid_px = (n as f32) * module_px;
                let quiet_px = 12.0_f32;
                let card_px = grid_px + quiet_px * 2.0;

                let mut matrix_layer = div()
                    .relative()
                    .w(px(grid_px))
                    .h(px(grid_px))
                    .bg(rgb(0xFFFFFF));

                for y in 0..n {
                    let mut x = 0;
                    while x < n {
                        if matches!(colors[y * n + x], qrcode::Color::Dark) {
                            let start_x = x;
                            while x < n && matches!(colors[y * n + x], qrcode::Color::Dark) {
                                x += 1;
                            }
                            let span_w = ((x - start_x) as f32) * module_px;
                            matrix_layer = matrix_layer.child(
                                div()
                                    .absolute()
                                    .left(px((start_x as f32) * module_px))
                                    .top(px((y as f32) * module_px))
                                    .w(px(span_w))
                                    .h(px(module_px))
                                    .bg(rgb(0x000000)),
                            );
                        } else {
                            x += 1;
                        }
                    }
                }

                // Compact center Alipay badge (~2.5% area, safely covered by EcLevel::Q 25% ECC)
                let badge_outer = 28.0_f32;
                let badge_offset = (grid_px - badge_outer) * 0.5;
                matrix_layer = matrix_layer.child(
                    div()
                        .absolute()
                        .left(px(badge_offset))
                        .top(px(badge_offset))
                        .w(px(badge_outer))
                        .h(px(badge_outer))
                        .rounded(px(6.0))
                        .bg(rgb(0xFFFFFF))
                        .p(px(2.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .w_full()
                                .h_full()
                                .rounded(px(4.0))
                                .bg(rgb(0x1677FF))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xFFFFFF))
                                .child("支"),
                        ),
                );

                return div()
                    .w(px(card_px))
                    .h(px(card_px))
                    .rounded(px(12.0))
                    .bg(rgb(0xFFFFFF))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(matrix_layer)
                    .into_any_element();
            }
        }

        div()
            .w(px(200.0))
            .h(px(200.0))
            .rounded(px(12.0))
            .bg(rgb(0xFFFFFF))
            .p(px(16.0))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(8.0))
            .child(
                div()
                    .size(px(48.0))
                    .rounded(px(10.0))
                    .bg(rgb(0x1677FF))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(24.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child("支"),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x475569))
                    .child(if is_loading {
                        if zh { "正在生成支付宝二维码..." } else { "Generating Alipay QR..." }
                    } else if zh {
                        "请点击下方刷新获取二维码"
                    } else {
                        "Click Refresh below to load QR"
                    }),
            )
            .into_any_element()
    }

    pub fn open_alipay_browser_url(url: &str) -> bool {
        if !(url.starts_with("https://") || url.starts_with("http://")) {
            return false;
        }
        #[cfg(target_os = "macos")]
        let cmd = std::process::Command::new("open").arg(url).spawn();
        #[cfg(target_os = "linux")]
        let cmd = std::process::Command::new("xdg-open").arg(url).spawn();
        #[cfg(target_os = "windows")]
        let cmd = std::process::Command::new("cmd").args(["/C", "start", "", url]).spawn();
        cmd.is_ok()
    }

    pub fn open_alipay_web_auth(&mut self, cx: &mut Context<Self>) {
        if self.alipay_is_loading {
            return;
        }
        if let Some(url) = self.alipay_auth_url.clone() {
            let _ = Self::open_alipay_browser_url(&url);
            cx.notify();
        } else {
            self.start_alipay_flow(cx);
        }
    }

    pub fn start_alipay_flow(&mut self, cx: &mut Context<Self>) {
        self.alipay_is_loading = true;
        self.alipay_status_text = if self.is_chinese_locale {
            "正在获取支付宝登录二维码...".to_string()
        } else {
            "Fetching Alipay login QR code...".to_string()
        };
        self.alipay_attempt_id = None;
        self.alipay_auth_url = None;
        self.alipay_qr_payload = None;
        self.alipay_poll_generation += 1;
        let poll_gen = self.alipay_poll_generation;

        let client = self.client.clone();
        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let start_res = client.http_client.alipay_start().await;

            let mut launched_attempt_id: Option<String> = None;
            let ok = weak_shell.update(app, |shell, cx| {
                if shell.alipay_poll_generation != poll_gen || shell.auth_wizard_tab != AuthWizardTab::AlipayQr {
                    return false;
                }
                shell.alipay_is_loading = false;
                match start_res {
                    Ok(resp) => {
                        let att_id = resp.attempt_id.clone();
                        launched_attempt_id = Some(att_id.clone());
                        shell.alipay_attempt_id = Some(att_id);
                        shell.alipay_auth_url = Some(resp.login_url.clone());
                        shell.alipay_qr_payload = Some(resp.login_url.clone());
                        shell.alipay_status_text = if shell.is_chinese_locale {
                            "请使用手机支付宝 App 扫码登录".to_string()
                        } else {
                            "Please scan with Alipay mobile app".to_string()
                        };
                        cx.notify();
                        true
                    }
                    Err(err) => {
                        shell.alipay_status_text = format!("启动支付宝登录失败: {}", err);
                        shell.login_error = Some(err.to_string());
                        cx.notify();
                        false
                    }
                }
            }).unwrap_or(false);

            if ok {
                if let Some(att_id) = launched_attempt_id {
                    loop {
                        let _ = client.runtime().spawn(async {
                            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
                        }).await;

                        let should_continue = weak_shell.update(app, |shell, _cx| {
                            shell.alipay_poll_generation == poll_gen
                                && shell.auth_wizard_tab == AuthWizardTab::AlipayQr
                                && !shell.is_authenticated
                                && shell.alipay_attempt_id.as_deref() == Some(&att_id)
                        }).unwrap_or(false);
                        if !should_continue {
                            break;
                        }

                        let poll_res = client.http_client.alipay_poll(&att_id).await;
                        let done = weak_shell.update(app, |shell, cx| {
                            if shell.alipay_poll_generation != poll_gen || shell.auth_wizard_tab != AuthWizardTab::AlipayQr {
                                return true;
                            }
                            match poll_res {
                                Ok(poll) => {
                                    if poll.is_completed() {
                                        if let Some(session) = poll.session {
                                            shell.apply_authenticated_session(session, cx);
                                            return true;
                                        } else if let Some(token) = poll.token {
                                            let auth = AuthResponse {
                                                user_id: poll.user_id.or_else(|| poll.user.as_ref().map(|u| u.user_id)).unwrap_or(100003),
                                                username: poll.username.or_else(|| poll.user.as_ref().and_then(|u| u.username.clone())).unwrap_or_else(|| "alipay_user".to_string()),
                                                session_id: format!("sess_alipay_{}", att_id),
                                                token,
                                                first_name: poll.user.as_ref().map(|u| u.first_name.clone()).or_else(|| Some("支付宝用户".to_string())),
                                                created_at: None,
                                                expires_at: None,
                                            };
                                            shell.apply_authenticated_session(auth, cx);
                                            return true;
                                        }
                                    } else if poll.is_expired() {
                                        shell.alipay_status_text = if shell.is_chinese_locale {
                                            "二维码已失效，请点击刷新".to_string()
                                        } else {
                                            "QR Code expired, please refresh".to_string()
                                        };
                                        cx.notify();
                                        return true;
                                    }
                                    false
                                }
                                Err(_) => false,
                            }
                        }).unwrap_or(true);

                        if done {
                            break;
                        }
                    }
                }
            }
        }).detach();
    }

    pub fn simulate_alipay_scan(&mut self, cx: &mut Context<Self>) {
        let attempt_id = match &self.alipay_attempt_id {
            Some(id) => id.clone(),
            None => return,
        };
        self.alipay_status_text = if self.is_chinese_locale {
            "正在模拟扫码授权...".to_string()
        } else {
            "Simulating scan authorization...".to_string()
        };
        cx.notify();

        let client = self.client.clone();
        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let sim_res = client.http_client.alipay_simulate_scan(&attempt_id).await;
            let _ = weak_shell.update(app, |shell, cx| {
                match sim_res {
                    Ok(_) => {
                        shell.alipay_status_text = if shell.is_chinese_locale {
                            "模拟授权成功，正在同步会话...".to_string()
                        } else {
                            "Authorization approved, completing login...".to_string()
                        };
                        cx.notify();
                    }
                    Err(err) => {
                        shell.login_error = Some(format!("模拟授权失败: {}", err));
                        cx.notify();
                    }
                }
            });
        }).detach();
    }

    pub fn send_email_code(&mut self, cx: &mut Context<Self>) {
        let email = self.email_input.trim().to_string();
        if email.is_empty() || !email.contains('@') || !email.contains('.') {
            self.login_error = Some(if self.is_chinese_locale {
                "请输入有效的邮箱地址".to_string()
            } else {
                "Please enter a valid email address".to_string()
            });
            cx.notify();
            return;
        }

        if self.email_code_cooldown > 0 || self.is_sending_email_code {
            return;
        }

        self.is_sending_email_code = true;
        self.login_error = None;
        self.email_cooldown_generation += 1;
        let cd_gen = self.email_cooldown_generation;
        cx.notify();

        let client = self.client.clone();
        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.email_send_code(&email).await;
            let _ = weak_shell.update(app, |shell, cx| {
                shell.is_sending_email_code = false;
                match res {
                    Ok(resp) => {
                        shell.email_code_cooldown = if resp.cooldown_seconds > 0 { resp.cooldown_seconds } else { 60 };
                        shell.active_login_field = LoginInputField::EmailCode;
                        shell.login_error = None;
                        cx.notify();

                        let cooldown = shell.email_code_cooldown;
                        // Start 1-second interval cooldown ticker
                        cx.spawn(async move |weak_timer: WeakEntity<Self>, timer_app: &mut AsyncApp| {
                            for _ in 0..cooldown {
                                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                                let should_continue = weak_timer.update(timer_app, |s, s_cx| {
                                    if s.email_cooldown_generation != cd_gen {
                                        return false;
                                    }
                                    if s.email_code_cooldown > 0 {
                                        s.email_code_cooldown -= 1;
                                        s_cx.notify();
                                        s.email_code_cooldown > 0
                                    } else {
                                        false
                                    }
                                }).unwrap_or(false);
                                if !should_continue {
                                    break;
                                }
                            }
                        }).detach();
                    }
                    Err(err) => {
                        shell.login_error = Some(match err {
                            HttpClientError::Api { message, .. } => message,
                            other => other.to_string(),
                        });
                        cx.notify();
                    }
                }
            });
        }).detach();
    }

    pub fn submit_email_register(&mut self, cx: &mut Context<Self>) {
        let email = self.email_input.trim().to_string();
        let code = self.email_code_input.trim().to_string();
        let username = self.username_input.trim().to_string();
        let password = self.password_input.clone();
        let first_name = if self.first_name_input.trim().is_empty() {
            username.clone()
        } else {
            self.first_name_input.trim().to_string()
        };

        if email.is_empty() || !email.contains('@') {
            self.login_error = Some(if self.is_chinese_locale {
                "请输入有效的邮箱地址".to_string()
            } else {
                "Please enter a valid email address".to_string()
            });
            cx.notify();
            return;
        }

        if code.len() < 4 {
            self.login_error = Some(if self.is_chinese_locale {
                "请输入收到的验证码".to_string()
            } else {
                "Please enter the verification code".to_string()
            });
            cx.notify();
            return;
        }

        if username.is_empty() || password.is_empty() {
            self.login_error = Some(if self.is_chinese_locale {
                "用户名和密码不能为空".to_string()
            } else {
                "Username and password cannot be empty".to_string()
            });
            cx.notify();
            return;
        }

        self.is_authenticating = true;
        self.login_error = None;
        cx.notify();

        let client = self.client.clone();
        let req = EmailRegisterRequest {
            email,
            code,
            username,
            password,
            first_name,
        };

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.email_register(&req).await;
            let _ = weak_shell.update(app, |shell, cx| {
                shell.is_authenticating = false;
                match res {
                    Ok(auth) => {
                        shell.apply_authenticated_session(auth, cx);
                    }
                    Err(err) => {
                        shell.login_error = Some(match err {
                            HttpClientError::Api { message, .. } => message,
                            other => other.to_string(),
                        });
                        cx.notify();
                    }
                }
            });
        }).detach();
    }

    pub fn logout(&mut self, cx: &mut Context<Self>) {
        let token = self.session_token.take();
        let session_id = self.session_id.take();
        let client = self.client.clone();
        let srv_url = self.client.config.server_url.clone();
        self.auth_generation = self.auth_generation.wrapping_add(1);
        self.chat_list_request_generation = self.chat_list_request_generation.wrapping_add(1);
        self.history_request_generation = self.history_request_generation.wrapping_add(1);
        self.user_id = None;
        self.username = None;
        self.is_authenticated = false;
        self.ws_stream_started = false;
        self.login_step = LoginWizardStep::Start;
        self.verification_code.clear();
        self.ime_preedit = None;
        self.contacts_list.clear();
        self.active_call_id = None;
        self.webrtc_engine.hangup_local("logout");
        let _ = self.webrtc_engine.drain_outbound_frames();
        let _ = self.voice_recorder.stop();
        self.audio_player.stop();
        self.talk_mode_overlay = None;
        self.computer_overlay = None;
        self.media_viewer_overlay = None;
        self.iv_viewer_overlay = None;
        self.active_panel = ActiveWorkspacePanel::Chats;
        self.client.clear_session();
        self.app.reset_session();
        self.window_runtime.clear_session_user();
        self.window_runtime
            .set_connection_status(BannerConnectionStatus::Connecting, None);
        self.panel_state.reset();
        self.pending_outgoing_nonces.clear();
        self.resolved_outgoing_nonces.clear();
        self.sidebar.update(cx, |sb, sb_cx| {
            self.stage.update(sb_cx, |st, st_cx| {
                self.inspector.update(st_cx, |insp, _| {
                    reset_account_conversation_views(sb, st, insp, &srv_url, None);
                });
            });
        });
        self.last_sidebar_selection_revision = self.sidebar.read(cx).selection_revision;
        self.main_menu_drawer.update(cx, |m, _| {
            m.close();
            m.accounts.clear();
            m.active_account_id = 0;
        });
        self.modal_container.update(cx, |m, _| m.hide_all());
        self.nav_rail.update(cx, |r, _| {
            r.is_connected = false;
            r.set_account("?", "");
            r.active_destination = NavDestination::Chats;
        });
        ClientConfig::clear_session();

        if let (Some(token), Some(session_id)) = (token, session_id) {
            let http = client.http_client.clone();
            client.runtime().spawn(async move {
                let _ = http.revoke_session(&token, &session_id).await;
            });
        }
        cx.notify();
    }

    pub fn nav_rail(&self) -> &Entity<NavRail> {
        &self.nav_rail
    }

    pub fn sidebar(&self) -> &Entity<ChatSidebar> {
        &self.sidebar
    }

    pub fn stage(&self) -> &Entity<ConversationStage> {
        &self.stage
    }

    pub fn composer(&self) -> &Entity<CanonicalComposer> {
        &self.composer
    }

    pub fn inspector(&self) -> &Entity<ContextInspector> {
        &self.inspector
    }

    pub fn show_talk_mode(&mut self, cx: &mut Context<Self>) {
        let peer = self.stage.read(cx).chat_title.clone();
        let overlay = cx.new(|_| CallOverlay::new(peer));
        cx.observe(&overlay, |shell: &mut AppShell, ov: Entity<CallOverlay>, cx: &mut Context<AppShell>| {
            if ov.read(cx).should_close {
                shell.hide_talk_mode(cx);
                cx.notify();
            }
        }).detach();
        self.talk_mode_overlay = Some(overlay);
    }

    pub fn hide_talk_mode(&mut self, cx: &mut Context<Self>) {
        self.talk_mode_overlay = None;
        self.last_handled_nav_dest = Some(NavDestination::Chats);
        self.nav_rail.update(cx, |r, _| {
            if r.active_destination == NavDestination::TalkMode || r.active_destination == NavDestination::Calls {
                r.set_active_destination(NavDestination::Chats);
            }
        });
    }

    pub fn show_remote_computer(&mut self, cx: &mut Context<Self>) {
        let overlay = cx.new(|_| ComputerWorkbench::new("Dev-Workstation-01".to_string()));
        cx.observe(&overlay, |shell: &mut AppShell, ov: Entity<ComputerWorkbench>, cx: &mut Context<AppShell>| {
            if ov.read(cx).should_close {
                shell.hide_remote_computer(cx);
                cx.notify();
            }
        }).detach();
        self.computer_overlay = Some(overlay);
    }

    pub fn hide_remote_computer(&mut self, cx: &mut Context<Self>) {
        self.computer_overlay = None;
        self.last_handled_nav_dest = Some(NavDestination::Chats);
        self.nav_rail.update(cx, |r, _| {
            if r.active_destination == NavDestination::RemoteComputer {
                r.set_active_destination(NavDestination::Chats);
            }
        });
    }

    pub fn show_media_viewer(&mut self, url: impl Into<String>, is_video: bool, cx: &mut Context<Self>) {
        let overlay = cx.new(|_| MediaViewer::new(url.into(), is_video));
        cx.observe(&overlay, move |shell: &mut AppShell, ov: Entity<MediaViewer>, cx: &mut Context<AppShell>| {
            if ov.read(cx).should_close {
                shell.hide_media_viewer();
                cx.notify();
            }
        }).detach();
        self.media_viewer_overlay = Some(overlay);
    }

    pub fn hide_media_viewer(&mut self) {
        self.media_viewer_overlay = None;
    }

    pub fn append_outgoing_text(&mut self, text: String, cx: &mut Context<Self>) {
        let verified_target = {
            let sb = self.sidebar.read(cx);
            let st = self.stage.read(cx);
            validate_selected_send_target(
                self.is_authenticated,
                self.session_token.as_deref(),
                self.user_id,
                sb,
                st,
            )
        };
        let Some(target) = verified_target else {
            return;
        };
        let active_chat_id = target.chat_id;
        let my_user_id = target.user_id;
        let Some(token) = self.session_token.clone() else {
            return;
        };

        let msg_id = self.client.next_client_nonce();
        self.pending_outgoing_nonces.insert(msg_id, active_chat_id);

        let peer_name = self.stage.read(cx).chat_title.clone();
        let my_name = self.username.clone().unwrap_or_else(|| "Me".to_string());
        let my_initials = crate::views::app_shell::shell_truth::derive_initials(&my_name);
        let my_color = crate::views::app_shell::peer_avatar_color(my_user_id);
        let now_ts = format_authoritative_timestamp(Some(chrono::Utc::now().timestamp_millis()));
        let fence = SessionRequestFence {
            auth_generation: self.auth_generation,
            request_generation: 0,
            token: token.clone(),
            user_id: my_user_id,
        };

        // 1. Add user's outgoing message bubble
        self.stage.update(cx, |st, st_cx| {
            st.add_message(MessageItemModel {
                id: msg_id,
                sender_id: my_user_id,
                sender_name: my_name.clone(),
                avatar_initials: my_initials,
                avatar_color: my_color,
                direction: MessageDirection::Outgoing,
                timestamp: now_ts.clone(),
                delivery_status: DeliveryStatus::Sending,
                reply_to: None,
                reactions: vec![],
                content: MessageContent::Text {
                    text: text.clone(),
                    is_edited: false,
                },
                bot_keyboard: None,
                is_pinned: false,
            });
            st_cx.notify();
        });

        // 2. Update sidebar preview
        let is_bot_dialog = {
            let sb = self.sidebar.read(cx);
            sb.bot_ids.contains(&active_chat_id)
                || sb
                    .dialogs
                    .iter()
                    .any(|d| d.id == active_chat_id && d.folder == "Bots")
        };
        self.sidebar.update(cx, |sb, sb_cx| {
            if let Some(d) = sb.dialogs.iter_mut().find(|d| d.id == active_chat_id) {
                d.last_message = text.clone();
                d.timestamp = now_ts.clone();
                d.delivery_status = DialogDeliveryStatus::Sending;
            }
            sb_cx.notify();
        });

        // 3. Dispatch real HTTP POST /api/v1/messages/send (fenced by session, carrying client_nonce)
        {
            let client = self.client.clone();
            let http_req = RichSendMessageRequest {
                chat_id: active_chat_id as i64,
                text: Some(text.clone()),
                content: Some(gpui_telegram_proto::message::MessageContent::plain_text(text.clone())),
                entities: Vec::new(),
                reply_to: None,
                quote_text: None,
                topic_id: None,
                client_nonce: Some(msg_id),
                forward_from_chat_id: None,
                forward_from_msg_id: None,
                silent: false,
                scheduled_at_ms: None,
                inline_keyboard: None,
            };
            let sent_msg_id = msg_id;
            let http_fence = fence.clone();

            cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
                let res = client.http_client.send_message_rich(&token, &http_req).await;
                let _ = weak_shell.update(app, |shell, cx| {
                    if !http_fence.matches(
                        shell.is_authenticated,
                        shell.auth_generation,
                        0,
                        shell.session_token.as_deref(),
                        shell.user_id,
                    ) {
                        return;
                    }
                    shell.pending_outgoing_nonces.remove(&sent_msg_id);
                    match res {
                        Ok(resp) => {
                            let server_id = u64::try_from(resp.message_id).unwrap_or(0);
                            if server_id > 0 {
                                shell.resolved_outgoing_nonces.insert(sent_msg_id, server_id);
                            }
                            shell.reconcile_outgoing_message(
                                active_chat_id,
                                sent_msg_id,
                                server_id,
                                DeliveryStatus::Sent,
                                resp.pts,
                                cx,
                            );
                        }
                        Err(err) => {
                            let err_text = format!(
                                "{}: {}",
                                tr_str(shell.active_lang_id(), "发送失败", "Message send failed"),
                                err
                            );
                            shell.window_runtime.enqueue_toast(err_text, 3500);
                        }
                    }
                    cx.notify();
                });
            })
            .detach();
        }

        // 4. For AI Bot chats, the server's `POST /api/v1/messages/send` triggers `maybe_trigger_bot_reply`
        //    and pushes the single authoritative reply over WebSocket. Do NOT spawn a duplicate `agent_runs` call.
        let is_ai_bot = is_bot_dialog
            || self.is_chat_bot(active_chat_id, cx)
            || peer_name.contains("Autonomous Agent")
            || peer_name.contains("Bot");

        if is_ai_bot {
            let lang = self.active_lang_id();
            self.sidebar.update(cx, |sb, sb_cx| {
                sb.set_typing(
                    active_chat_id,
                    Some(tr_str(lang, "AI 助手正在思考...", "Assistant is thinking...").to_string()),
                );
                sb_cx.notify();
            });
        }
    }

    /// Launches the long-lived GPUI background tasks for WebSocket frame streaming
    /// and connection health monitoring upon successful authentication or session hydration.
    pub fn start_ws_inbound_stream(&mut self, cx: &mut Context<Self>) {
        if self.ws_stream_started {
            return;
        }
        self.ws_stream_started = true;

        // 1. Observe Connection Status changes for NavRail UI indicator
        let mut status_rx = self.client.ws_transport.subscribe_status();
        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            while status_rx.changed().await.is_ok() {
                let status = status_rx.borrow().clone();
                let is_connected = status.is_connected();
                let update_res = weak_shell.update(app, |shell, cx| {
                    let was_connected = shell.nav_rail.read(cx).is_connected;
                    shell.nav_rail.update(cx, |r, _| {
                        r.set_connected(is_connected);
                    });
                    let banner_status = match &status {
                        ConnectionStatus::Connected => BannerConnectionStatus::Connected,
                        ConnectionStatus::Connecting => BannerConnectionStatus::Connecting,
                        ConnectionStatus::Reconnecting { .. } => BannerConnectionStatus::Updating,
                        ConnectionStatus::Offline { .. } => BannerConnectionStatus::WaitingForNetwork,
                    };
                    shell.window_runtime.set_connection_status(banner_status, None);
                    if is_connected && !was_connected {
                        // When WebSocket establishes or re-establishes connection,
                        // trigger catch-up sync (refresh_chats and refresh historical messages
                        // for the active chat if selected) so that any messages or PTS changes
                        // that occurred while offline are seamlessly reconciled.
                        shell.refresh_chats(cx);
                        let active_chat = shell.stage.read(cx).active_chat_id;
                        if let Some(active_chat_id) = active_chat {
                            shell.load_historical_messages(active_chat_id, cx);
                        }
                    }
                    if let ConnectionStatus::Offline { ref reason } = status {
                        if reason.contains("401") || reason.contains("unauthorized") {
                            shell.logout(cx);
                            shell.login_error = Some(
                                "Session expired or revoked from another device. Please sign in again.".to_string(),
                            );
                        }
                    }
                    cx.notify();
                });
                if update_res.is_err() {
                    break;
                }
            }
        })
        .detach();

        // 2. Consume real-time inbound WebSocket frames
        let mut frame_rx = self.client.ws_channel.subscribe();
        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            while let Ok(frame) = frame_rx.recv().await {
                let update_res = weak_shell.update(app, |shell, cx| {
                    shell.handle_ws_server_frame(frame, cx);
                });
                if update_res.is_err() {
                    break;
                }
            }
        })
        .detach();
    }

    /// Master router for incoming WsServerFrame variants dispatched from the WebSocket thread.
    pub fn handle_ws_server_frame(&mut self, frame: WsServerFrame, cx: &mut Context<Self>) {
        match frame {
            WsServerFrame::Ack { client_nonce, message_id, pts, .. } => {
                self.handle_ws_ack(client_nonce, message_id, pts, cx);
            }
            WsServerFrame::PtsPush(pts_update) => {
                self.handle_ws_pts_push(pts_update, cx);
            }
            WsServerFrame::SyncDiff(diff_response) => {
                self.handle_ws_sync_diff(diff_response, cx);
            }
            WsServerFrame::Update(update_event) => {
                if let Some(active_chat) = self.stage.read(cx).active_chat_id {
                    self.apply_update_event(ChatId(active_chat as i64), update_event, cx);
                } else {
                    self.apply_update_event(ChatId(0), update_event, cx);
                }
            }
            WsServerFrame::Pong { .. } => {
                // Heartbeat roundtrip successful; connection alive
            }
            WsServerFrame::Error { code, .. } => {
                if code == 401 {
                    self.logout(cx);
                    self.login_error = Some("Session expired (401). Please sign in again.".to_string());
                }
            }
        }
    }

    fn has_stage_or_history_message(&self, chat_id: u64, msg_id: u64, cx: &App) -> bool {
        let st = self.stage.read(cx);
        let in_active = st.active_chat_id == Some(chat_id)
            && st.messages.iter().any(|m| m.id == msg_id);
        in_active
            || st
                .chat_histories
                .get(&chat_id)
                .is_some_and(|hist| hist.iter().any(|m| m.id == msg_id))
    }

    fn reconcile_outgoing_message(
        &mut self,
        chat_id: u64,
        local_id: u64,
        server_id: u64,
        min_status: DeliveryStatus,
        pts: u64,
        cx: &mut Context<Self>,
    ) {
        if pts > 0 {
            self.client.sync_coordinator.set_pts(ChatId(chat_id as i64), pts);
        }
        self.stage.update(cx, |st, st_cx| {
            let mut changed = false;
            if st.active_chat_id == Some(chat_id) {
                if reconcile_stage_message_list(&mut st.messages, local_id, server_id, min_status) {
                    st.row_height_cache.rebuild_prefix_sums(&st.messages);
                    changed = true;
                }
            }
            if let Some(history) = st.chat_histories.get_mut(&chat_id) {
                reconcile_stage_message_list(history, local_id, server_id, min_status);
            }
            if changed {
                st_cx.notify();
            }
        });
        self.sidebar.update(cx, |sb, sb_cx| {
            if let Some(d) = sb.dialogs.iter_mut().find(|d| d.id == chat_id) {
                d.delivery_status = if matches!(min_status, DeliveryStatus::Read) {
                    DialogDeliveryStatus::Read
                } else {
                    DialogDeliveryStatus::Sent
                };
            }
            sb_cx.notify();
        });
        cx.notify();
    }

    fn handle_ws_ack(&mut self, client_nonce: u64, message_id: i64, pts: u64, cx: &mut Context<Self>) {
        let tracked_chat = self.pending_outgoing_nonces.remove(&client_nonce);
        let server_id = u64::try_from(message_id).unwrap_or(0);
        if server_id > 0 {
            self.resolved_outgoing_nonces.insert(client_nonce, server_id);
        }
        if let Some(cid) = tracked_chat.or(self.stage.read(cx).active_chat_id) {
            self.reconcile_outgoing_message(cid, client_nonce, server_id, DeliveryStatus::Sent, pts, cx);
        }
    }

    fn handle_ws_pts_push(&mut self, pts_update: PtsUpdate, cx: &mut Context<Self>) {
        let chat_id = pts_update.chat_id;
        match self.client.sync_coordinator.evaluate_update(&pts_update) {
            SyncOutcome::Contiguous => {
                self.apply_update_event(chat_id, pts_update.event, cx);
                let drained = self.client.sync_coordinator.drain_contiguous_buffer(chat_id);
                for buffered in drained {
                    self.apply_update_event(chat_id, buffered.event, cx);
                }
            }
            SyncOutcome::Duplicate => {}
            SyncOutcome::GapDetected { last_seen, .. } => {
                self.client.sync_coordinator.buffer_gap_update(pts_update);
                let req = SyncGetDifferenceRequest::new(chat_id, last_seen, 100);
                self.client.ws_channel.send(WsClientFrame::Sync(req));
            }
        }
    }

    fn resolve_incoming_sender_identity(
        &self,
        chat_u64: u64,
        sender_id: u64,
        cx: &gpui::App,
    ) -> (String, String, u32) {
        let color = crate::views::app_shell::peer_avatar_color(sender_id);
        if let Some(c) = self.contacts_list.iter().find(|c| c.user_id == sender_id) {
            let mut full = c.first_name.trim().to_string();
            if let Some(ref last) = c.last_name {
                let l = last.trim();
                if !l.is_empty() {
                    if !full.is_empty() {
                        full.push(' ');
                    }
                    full.push_str(l);
                }
            }
            if full.is_empty() {
                if let Some(ref u) = c.username {
                    full = u.trim().to_string();
                }
            }
            if !full.is_empty() {
                let init = derive_initials(&full);
                return (
                    full,
                    if init.is_empty() { "TG".to_string() } else { init },
                    color,
                );
            }
        }
        if let Some(rc) = self.rich_chats_by_id.get(&chat_u64) {
            if rc.peer_user_id == Some(sender_id)
                || rc.is_bot
                || (!rc.is_group && !rc.is_channel)
            {
                let candidate = rc
                    .peer_display_name
                    .as_deref()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .or_else(|| {
                        let t = rc.title.trim();
                        if t.is_empty() { None } else { Some(t) }
                    });
                if let Some(name) = candidate {
                    let init = derive_initials(name);
                    return (
                        name.to_string(),
                        if init.is_empty() { "TG".to_string() } else { init },
                        color,
                    );
                }
            }
        }
        let sb = self.sidebar.read(cx);
        if let Some(d) = sb.dialogs.iter().find(|d| d.id == chat_u64) {
            let is_group_or_channel = d.folder == "Groups" || d.folder == "Channels";
            if (!is_group_or_channel || sender_id == chat_u64) && !d.peer_name.trim().is_empty() {
                let init = derive_initials(&d.peer_name);
                return (
                    d.peer_name.clone(),
                    if init.is_empty() { "TG".to_string() } else { init },
                    d.avatar_color,
                );
            }
        }
        let st = self.stage.read(cx);
        if st.active_chat_id == Some(chat_u64) && !st.chat_title.trim().is_empty() {
            let init = derive_initials(&st.chat_title);
            return (
                st.chat_title.clone(),
                if init.is_empty() { "TG".to_string() } else { init },
                st.chat_avatar_color,
            );
        }
        let fallback = format!("User {}", sender_id);
        let init = derive_initials(&fallback);
        (
            fallback,
            if init.is_empty() { "TG".to_string() } else { init },
            color,
        )
    }

    fn apply_update_event(&mut self, chat_id: ChatId, event: UpdateEvent, cx: &mut Context<Self>) {
        match event {
            UpdateEvent::NewMessage(msg) => {
                let current_user_id = self.user_id.unwrap_or(0);
                let is_outgoing = msg.sender_id.0 == current_user_id;
                let chat_u64 = chat_id.0 as u64;
                let server_id = u64::try_from(msg.id.0).unwrap_or(0);

                if is_outgoing {
                    if let Some(nonce) = msg.client_nonce {
                        let was_tracked = self.pending_outgoing_nonces.remove(&nonce).is_some()
                            || self.resolved_outgoing_nonces.contains_key(&nonce)
                            || self.has_stage_or_history_message(chat_u64, nonce, cx)
                            || (server_id > 0 && self.has_stage_or_history_message(chat_u64, server_id, cx));
                        if was_tracked {
                            if server_id > 0 {
                                self.resolved_outgoing_nonces.insert(nonce, server_id);
                            }
                            self.reconcile_outgoing_message(
                                chat_u64,
                                nonce,
                                server_id,
                                DeliveryStatus::Read,
                                0,
                                cx,
                            );
                            return;
                        }
                    }
                    if server_id > 0 && self.has_stage_or_history_message(chat_u64, server_id, cx) {
                        self.reconcile_outgoing_message(
                            chat_u64,
                            server_id,
                            server_id,
                            DeliveryStatus::Read,
                            0,
                            cx,
                        );
                        return;
                    }
                }

                let preview_text = match &msg.content {
                    gpui_telegram_proto::message::MessageContent::Text { text, .. } => text.clone(),
                    _ => "[Media attachment]".to_string(),
                };
                let inline_kb = serde_json::to_value(&msg.content)
                    .ok()
                    .and_then(|val| crate::views::app_shell::shell_truth::decode_inline_keyboard_json(&val));

                let (sender_name, avatar_initials, avatar_color) = if is_outgoing {
                    let my_name = self.username.clone().unwrap_or_else(|| "You".to_string());
                    let init = derive_initials(&my_name);
                    (
                        my_name,
                        if init.is_empty() { "ME".to_string() } else { init },
                        crate::views::app_shell::peer_avatar_color(current_user_id),
                    )
                } else {
                    self.resolve_incoming_sender_identity(chat_u64, msg.sender_id.0, cx)
                };

                let model = MessageItemModel {
                    id: server_id,
                    sender_id: msg.sender_id.0,
                    sender_name: sender_name.clone(),
                    avatar_initials,
                    avatar_color,
                    direction: if is_outgoing { MessageDirection::Outgoing } else { MessageDirection::Incoming },
                    timestamp: msg.date.format("%H:%M").to_string(),
                    delivery_status: DeliveryStatus::Read,
                    reply_to: None,
                    reactions: vec![],
                    content: MessageContent::Text {
                        text: preview_text.clone(),
                        is_edited: false,
                    },
                    bot_keyboard: inline_kb.clone(),
                    is_pinned: false,
                };

                let is_current_chat = self.stage.read(cx).active_chat_id == Some(chat_u64);

                if is_current_chat {
                    self.stage.update(cx, |st, st_cx| {
                        st.add_message(model.clone());
                        st_cx.notify();
                    });
                } else {
                    self.stage.update(cx, |st, _| {
                        let hist = st.chat_histories.entry(chat_u64).or_default();
                        if !reconcile_stage_message_list(hist, model.id, model.id, DeliveryStatus::Read) {
                            hist.push(model);
                        }
                    });
                }

                let is_bot_chat = self.is_chat_bot(chat_u64, cx)
                    || self.sidebar.read(cx).bot_ids.contains(&msg.sender_id.0);

                if !is_outgoing
                    && self.talk_mode_overlay.is_some()
                    && self.active_talk_session_is_ai
                    && self.active_talk_bot_chat_id == Some(chat_u64)
                    && !preview_text.trim().is_empty()
                {
                    if let Some(ov) = self.talk_mode_overlay.clone() {
                        ov.update(cx, |o, _| {
                            o.add_transcript(&sender_name, &preview_text, false);
                        });
                    }
                }

                self.sidebar.update(cx, |sb, sb_cx| {
                    if !is_outgoing {
                        sb.set_typing(chat_u64, None);
                    }
                    if let Some(dialog) = sb.dialogs.iter_mut().find(|d| d.id == chat_u64) {
                        dialog.last_message = preview_text;
                        dialog.timestamp = msg.date.format("%H:%M").to_string();
                        if !is_current_chat && !is_outgoing {
                            dialog.unread_count += 1;
                        }
                    }
                    sb_cx.notify();
                });

                if !is_outgoing && is_bot_chat && inline_kb.is_none() && server_id > 0 {
                    if let (Some(token), Some(uid)) = (self.session_token.clone(), self.user_id) {
                        let fence = SessionRequestFence {
                            auth_generation: self.auth_generation,
                            request_generation: 0,
                            token,
                            user_id: uid,
                        };
                        let client = self.client.clone();
                        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                            if let Ok(rich) = client.hermes_messages_history_rich(chat_u64 as i64, Some(10)).await {
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
                                    shell.stage.update(cx, |st, st_cx| {
                                        overlay_rich_message_metadata(&mut st.messages, &rich);
                                        if let Some(hist) = st.chat_histories.get_mut(&chat_u64) {
                                            overlay_rich_message_metadata(hist, &rich);
                                        }
                                        st_cx.notify();
                                    });
                                    cx.notify();
                                });
                            }
                        })
                        .detach();
                    }
                }
                cx.notify();
            }
            UpdateEvent::EditMessage(msg) => {
                let chat_u64 = chat_id.0 as u64;
                let target_id = msg.id.0 as u64;
                if let gpui_telegram_proto::message::MessageContent::Text { text, .. } = msg.content {
                    self.stage.update(cx, |st, st_cx| {
                        if let Some(m) = st.messages.iter_mut().find(|m| m.id == target_id) {
                            m.content = MessageContent::Text {
                                text: text.clone(),
                                is_edited: true,
                            };
                            st.row_height_cache.mark_dirty();
                        }
                        if let Some(history) = st.chat_histories.get_mut(&chat_u64) {
                            if let Some(m) = history.iter_mut().find(|m| m.id == target_id) {
                                m.content = MessageContent::Text {
                                    text: text.clone(),
                                    is_edited: true,
                                };
                            }
                        }
                        st_cx.notify();
                    });
                }
                cx.notify();
            }
            UpdateEvent::DeleteMessages(tombstone) => {
                let chat_u64 = chat_id.0 as u64;
                let ids_to_delete: std::collections::HashSet<u64> =
                    tombstone.message_ids.iter().map(|id| id.0 as u64).collect();

                self.stage.update(cx, |st, st_cx| {
                    st.messages.retain(|m| !ids_to_delete.contains(&m.id));
                    if let Some(history) = st.chat_histories.get_mut(&chat_u64) {
                        history.retain(|m| !ids_to_delete.contains(&m.id));
                    }
                    st.row_height_cache.mark_dirty();
                    st_cx.notify();
                });
                cx.notify();
            }
            UpdateEvent::UserTyping(typing_event) => {
                self.sidebar.update(cx, |sb, sb_cx| {
                    sb.set_typing(
                        typing_event.chat_id.0 as u64,
                        Some(format!("User #{} is typing...", typing_event.user_id.0)),
                    );
                    sb_cx.notify();
                });
                cx.notify();
            }
            UpdateEvent::MessageReactions(ev) | UpdateEvent::ReactionUpdated(ev) => {
                let chat_u64 = ev.chat_id.0 as u64;
                let msg_u64 = ev.message_id.0 as u64;
                let mapped: Vec<crate::views::stage::cards::ReactionItem> = ev
                    .reactions
                    .into_iter()
                    .map(|r| crate::views::stage::cards::ReactionItem {
                        emoji: r.emoji,
                        count: r.count,
                        user_reacted: r.is_chosen_by_me,
                    })
                    .collect();
                self.stage.update(cx, |st, st_cx| {
                    if let Some(m) = st.messages.iter_mut().find(|m| m.id == msg_u64) {
                        m.reactions = mapped.clone();
                    }
                    if let Some(history) = st.chat_histories.get_mut(&chat_u64) {
                        if let Some(m) = history.iter_mut().find(|m| m.id == msg_u64) {
                            m.reactions = mapped;
                        }
                    }
                    st_cx.notify();
                });
                cx.notify();
            }
            UpdateEvent::PinnedMessagesChanged {
                chat_id: ev_chat,
                pinned_ids,
            } => {
                let chat_u64 = ev_chat.0 as u64;
                let ids: Vec<i64> = pinned_ids.iter().map(|id| id.0).collect();
                self.stage.update(cx, |st, st_cx| {
                    if let Some(history) = st.chat_histories.get_mut(&chat_u64) {
                        for m in history.iter_mut() {
                            m.is_pinned = ids.contains(&(m.id as i64));
                        }
                    }
                    if st.active_chat_id == Some(chat_u64) {
                        st.pinned_tracker.set_pinned_ids(ids.clone());
                        for m in &mut st.messages {
                            m.is_pinned = ids.contains(&(m.id as i64));
                        }
                    }
                    st_cx.notify();
                });
                cx.notify();
            }
            UpdateEvent::PinnedMessageChanged { message_id } => {
                let pin_u64 = message_id.map(|id| id.0 as u64);
                self.stage.update(cx, |st, st_cx| {
                    if let Some(id) = pin_u64 {
                        st.pinned_tracker.set_pinned_ids(vec![id as i64]);
                    } else {
                        st.pinned_tracker.set_pinned_ids(Vec::new());
                    }
                    for m in &mut st.messages {
                        m.is_pinned = Some(m.id) == pin_u64;
                    }
                    st_cx.notify();
                });
                cx.notify();
            }
            UpdateEvent::DraftUpdated {
                chat_id: ev_chat,
                draft,
                ..
            } => {
                let text = draft.map(|d| d.text).unwrap_or_default();
                self.sidebar.update(cx, |sb, sb_cx| {
                    sb.set_draft(ev_chat.0 as u64, text);
                    sb_cx.notify();
                });
                cx.notify();
            }
            UpdateEvent::FolderChanged { .. } => {
                self.refresh_folders(cx);
            }
            UpdateEvent::PollUpdated {
                chat_id: ev_chat, ..
            } => {
                if self.stage.read(cx).active_chat_id == Some(ev_chat.0 as u64) {
                    self.load_historical_messages(ev_chat.0 as u64, cx);
                }
            }
            UpdateEvent::CallUpdated {
                call_id,
                state,
                signal,
            } => {
                if let Some(sig) = signal {
                    let frame_opt = match sig.signal_type.as_str() {
                        "offer" => sig.sdp.map(|sdp| gpui_telegram_webrtc::SignalingFrame::Offer {
                            call_id: call_id.clone(),
                            sdp,
                        }),
                        "answer" => sig.sdp.map(|sdp| gpui_telegram_webrtc::SignalingFrame::Answer {
                            call_id: call_id.clone(),
                            sdp,
                        }),
                        "ice_candidate" => sig.candidate.map(|candidate| {
                            gpui_telegram_webrtc::SignalingFrame::IceCandidate {
                                call_id: call_id.clone(),
                                candidate,
                                sdp_mid: sig.sdp_mid,
                                sdp_mline_index: sig.sdp_mline_index.map(|v| v as u16),
                            }
                        }),
                        "ringing" => Some(gpui_telegram_webrtc::SignalingFrame::Ringing {
                            call_id: call_id.clone(),
                        }),
                        "accept" => Some(gpui_telegram_webrtc::SignalingFrame::Accept {
                            call_id: call_id.clone(),
                        }),
                        "hangup" => Some(gpui_telegram_webrtc::SignalingFrame::Hangup {
                            call_id: call_id.clone(),
                            reason: sig.reason.unwrap_or_else(|| "remote_hangup".to_string()),
                        }),
                        _ => None,
                    };
                    if let Some(frame) = frame_opt {
                        self.webrtc_engine.apply_signaling_frame(frame);
                    }
                }
                let emojis = self
                    .webrtc_engine
                    .emoji_fingerprint()
                    .map(|f| f.emoji_chars.join(" "));
                match state.as_str() {
                    "ended" | "declined" | "missed" | "cancelled" => {
                        if self.active_call_id.as_deref() == Some(call_id.as_str()) {
                            self.active_call_id = None;
                            self.talk_mode_overlay = None;
                        }
                        self.sync_calls_history(cx);
                    }
                    "accepted" | "active" | "connected" => {
                        if let Some(ov) = self.talk_mode_overlay.clone() {
                            ov.update(cx, |o, _| {
                                if let Some(ref e) = emojis {
                                    o.e2ee_emojis = e.clone();
                                }
                                o.is_connected = true;
                                o.set_state(crate::views::call_overlay::TalkModeState::Listening);
                            });
                        }
                    }
                    "ringing" | "initiated" => {
                        if self.talk_mode_overlay.is_none() && !call_id.is_empty() {
                            self.active_call_id = Some(call_id.clone());
                            let pal = self.palette;
                            let lang = self.active_lang_id();
                            let ov = cx.new(|_| {
                                let mut o = CallOverlay::new_live(
                                    tr_str(lang, "来电通话", "Incoming Call").to_string(),
                                    None,
                                    None,
                                    pal,
                                    lang,
                                );
                                o.is_connected = false;
                                o.set_state(crate::views::call_overlay::TalkModeState::Connecting);
                                if let Some(ref e) = emojis {
                                    o.e2ee_emojis = e.clone();
                                }
                                o
                            });
                            self.talk_mode_overlay = Some(ov);
                        }
                        self.sync_calls_history(cx);
                    }
                    _ => {}
                }
                cx.notify();
            }
            UpdateEvent::StoryUpdated { .. } | UpdateEvent::ForumTopicChanged { .. } => {
                cx.notify();
            }
            _ => {}
        }
    }

    fn handle_ws_sync_diff(&mut self, diff: SyncGetDifferenceResponse, cx: &mut Context<Self>) {
        let chat_id = diff.chat_id;

        for msg in diff.new_messages {
            self.apply_update_event(chat_id, UpdateEvent::NewMessage(msg), cx);
        }

        for update in diff.other_updates {
            self.apply_update_event(chat_id, update, cx);
        }

        self.client.sync_coordinator.set_pts(chat_id, diff.pts);

        if diff.has_more {
            let req = SyncGetDifferenceRequest::new(chat_id, diff.pts, 100);
            self.client.ws_channel.send(WsClientFrame::Sync(req));
        } else {
            let drained = self.client.sync_coordinator.drain_contiguous_buffer(chat_id);
            for buffered in drained {
                self.apply_update_event(chat_id, buffered.event, cx);
            }
        }
        cx.notify();
    }

    /// Dispatches POST /api/v1/contacts/add for either Create AI Bot or Add Friend
    pub fn dispatch_add_contact(&mut self, req: PendingAddContactRequest, cx: &mut Context<Self>) {
        let token = match self.session_token.clone() {
            Some(t) => t,
            None => return,
        };
        let client = self.client.clone();
        let http_req = AddContactHttpRequest {
            first_name: req.first_name.clone(),
            last_name: req.last_name.clone(),
            username: req.username.clone(),
            phone: req.phone.clone(),
            is_bot: req.is_bot,
            bot_system_prompt: req.system_prompt.clone(),
        };
        let local_dialog_id = req.local_dialog_id;

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.add_contact(&token, http_req).await;
            let _ = weak_shell.update(app, |shell, cx| {
                if let Ok(resp) = res {
                    let server_chat_id = resp.chat_id;
                    let mut selected_peer: Option<DialogItemModel> = None;
                    shell.sidebar.update(cx, |sb, sb_cx| {
                        if let Some(d) = sb.dialogs.iter_mut().find(|d| d.id == local_dialog_id) {
                            d.id = server_chat_id;
                            d.peer_name = resp.title.clone();
                            d.avatar_color = resp.avatar_color.unwrap_or(0x2AABEE);
                            selected_peer = Some(d.clone());
                        }
                        if sb.active_chat_id == Some(local_dialog_id) {
                            sb.active_chat_id = Some(server_chat_id);
                        }
                        sb_cx.notify();
                    });
                    if let Some(d) = selected_peer {
                        shell.stage.update(cx, |st, st_cx| {
                            st.set_chat(server_chat_id, &d.peer_name, "online", &d.avatar_initials, d.avatar_color);
                            st_cx.notify();
                        });
                    }
                    shell.load_historical_messages(server_chat_id, cx);
                    cx.notify();
                }
            });
        })
        .detach();
    }

    /// Dispatches POST /api/v1/chats/create for Create Group
    pub fn dispatch_create_group(&mut self, req: PendingCreateGroupRequest, cx: &mut Context<Self>) {
        let token = match self.session_token.clone() {
            Some(t) => t,
            None => return,
        };
        let client = self.client.clone();
        let title = req.title.clone();
        let member_ids = req.member_ids.clone();
        let member_usernames = req.member_usernames.clone();
        let about = req.about.clone();
        let local_dialog_id = req.local_dialog_id;

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client
                .http_client
                .create_group_with_token(
                    &token,
                    &title,
                    member_ids,
                    member_usernames,
                    about,
                )
                .await;
            let _ = weak_shell.update(app, |shell, cx| {
                if let Ok(resp) = res {
                    let server_chat_id = resp.chat_id.max(0) as u64;
                    let mut selected_peer: Option<DialogItemModel> = None;
                    shell.sidebar.update(cx, |sb, sb_cx| {
                        if let Some(d) = sb.dialogs.iter_mut().find(|d| d.id == local_dialog_id) {
                            d.id = server_chat_id;
                            selected_peer = Some(d.clone());
                        }
                        if sb.active_chat_id == Some(local_dialog_id) {
                            sb.active_chat_id = Some(server_chat_id);
                        }
                        sb_cx.notify();
                    });
                    if let Some(d) = selected_peer {
                        shell.stage.update(cx, |st, st_cx| {
                            st.set_chat(server_chat_id, &d.peer_name, "group", &d.avatar_initials, d.avatar_color);
                            st_cx.notify();
                        });
                    }
                    shell.load_historical_messages(server_chat_id, cx);
                    cx.notify();
                }
            });
        })
        .detach();
    }

    /// Dispatches GET /api/v1/search/messages?q=...&category=... for Categorized Search
    pub fn dispatch_categorized_search(
        &mut self,
        query: String,
        category: String,
        search_seq: u64,
        cx: &mut Context<Self>,
    ) {
        self.last_dispatched_search_seq = search_seq;
        let token = match self.session_token.clone() {
            Some(t) => t,
            None => return,
        };
        let client = self.client.clone();

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client
                .http_client
                .search_global(&token, &query, &category)
                .await;
            let _ = weak_shell.update(app, |shell, cx| {
                if shell.sidebar.read(cx).search_request_seq != search_seq {
                    return;
                }
                if let Ok(hits) = res {
                    let mapped: Vec<CategorizedSearchResult> = hits
                        .into_iter()
                        .map(|hit| {
                            let title = if hit.title.is_empty() {
                                format!("Chat #{}", hit.chat_id)
                            } else {
                                hit.title
                            };
                            let initials = title
                                .chars()
                                .take(2)
                                .collect::<String>()
                                .to_uppercase();
                            let cat = if hit.category.is_empty() {
                                category.clone()
                            } else {
                                hit.category
                            };
                            CategorizedSearchResult {
                                chat_id: hit.chat_id,
                                message_id: hit.message_id,
                                category: cat,
                                title,
                                subtitle: hit.snippet,
                                timestamp: "刚刚".to_string(),
                                avatar_initials: if initials.is_empty() {
                                    "TG".to_string()
                                } else {
                                    initials
                                },
                                avatar_color: hit.avatar_color.filter(|&c| c != 0).unwrap_or(0x2AABEE),
                                is_bot: hit.is_bot,
                            }
                        })
                        .collect();
                    shell.sidebar.update(cx, |sb, sb_cx| {
                        sb.remote_search_results = mapped;
                        sb_cx.notify();
                    });
                    cx.notify();
                }
            });
        })
        .detach();
    }

    /// Refreshes authentic chat list from GET /api/v1/chats
    pub fn refresh_chats(&mut self, cx: &mut Context<Self>) {
        let (Some(token), Some(uid)) = (self.session_token.clone(), self.user_id) else {
            return;
        };
        if !self.is_authenticated {
            return;
        }
        self.chat_list_request_generation = self.chat_list_request_generation.wrapping_add(1);
        let fence = SessionRequestFence {
            auth_generation: self.auth_generation,
            request_generation: self.chat_list_request_generation,
            token: token.clone(),
            user_id: uid,
        };
        let client = self.client.clone();

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.get_chats(&token).await;
            let rich_chats = client.hermes_list_chats_rich().await.ok();
            let _ = weak_shell.update(app, |shell, cx| {
                match res {
                    Ok(chats) => {
                        if let Some(ref rich_list) = rich_chats {
                            for rc in rich_list {
                                if rc.chat_id > 0 {
                                    let cid = rc.chat_id as u64;
                                    shell.rich_chats_by_id.insert(cid, rc.clone());
                                    if let Some(pid) = rc.peer_user_id {
                                        if pid > 0 {
                                            shell.chat_peer_user_ids.insert(cid, pid);
                                        }
                                    }
                                }
                            }
                        }
                        let applied = shell.sidebar.update(cx, |sb, sb_cx| {
                            let ok = shell.stage.update(sb_cx, |st, st_cx| {
                                let applied = shell.inspector.update(st_cx, |insp, insp_cx| {
                                    let applied = apply_fenced_chat_list_response(
                                        &fence,
                                        shell.is_authenticated,
                                        shell.auth_generation,
                                        shell.chat_list_request_generation,
                                        shell.session_token.as_deref(),
                                        shell.user_id,
                                        sb,
                                        st,
                                        insp,
                                        &chats,
                                    );
                                    if applied {
                                        insp_cx.notify();
                                    }
                                    applied
                                });
                                if applied {
                                    st_cx.notify();
                                }
                                applied
                            });
                            if ok {
                                for c in &chats {
                                    if c.is_bot.unwrap_or(false) {
                                        sb.bot_ids.insert(c.chat_id);
                                    }
                                }
                                if let Some(ref rich_list) = rich_chats {
                                    for rc in rich_list {
                                        if rc.is_bot && rc.chat_id > 0 {
                                            sb.bot_ids.insert(rc.chat_id as u64);
                                        }
                                    }
                                }
                                for d in &mut sb.dialogs {
                                    if d.avatar_initials.is_empty() || d.avatar_initials == "?" {
                                        let fixed = derive_initials(&d.peer_name);
                                        if !fixed.is_empty() {
                                            d.avatar_initials = fixed;
                                        }
                                    }
                                }
                                sb_cx.notify();
                            }
                            ok
                        });
                        if applied {
                            if let Some(selected_id) = shell.sidebar.read(cx).active_chat_id {
                                shell.sync_selected_chat_inspector_and_stage(selected_id, cx);
                            }
                        }
                    }
                    Err(err) => {
                        if fence.matches(
                            shell.is_authenticated,
                            shell.auth_generation,
                            shell.chat_list_request_generation,
                            shell.session_token.as_deref(),
                            shell.user_id,
                        ) {
                            shell.window_runtime.enqueue_toast(
                                format!(
                                    "{}: {}",
                                    tr_str(
                                        shell.active_lang_id(),
                                        "刷新会话列表失败",
                                        "Failed to refresh chats"
                                    ),
                                    err
                                ),
                                3200,
                            );
                        }
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    /// Fetches historical messages from the backend for the selected chat
    pub fn load_historical_messages(&mut self, chat_id: u64, cx: &mut Context<Self>) {
        let (Some(token), Some(uid)) = (self.session_token.clone(), self.user_id) else {
            return;
        };
        if !self.is_authenticated {
            return;
        }
        self.history_request_generation = self.history_request_generation.wrapping_add(1);
        let fence = SessionRequestFence {
            auth_generation: self.auth_generation,
            request_generation: self.history_request_generation,
            token: token.clone(),
            user_id: uid,
        };
        let client = self.client.clone();

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.get_chat_messages(&token, chat_id, Some(50)).await;
            let rich_msgs = client.hermes_messages_history_rich(chat_id as i64, Some(50)).await.ok();
            let _ = weak_shell.update(app, |shell, cx| {
                match res {
                    Ok(msgs) => {
                        let applied = shell.sidebar.read_with(cx, |sb, _| sb.dialogs.clone());
                        let _ = applied;
                        let ok = shell.sidebar.update(cx, |sb, sb_cx| {
                            shell.stage.update(sb_cx, |st, st_cx| {
                                let res = apply_fenced_history_response(
                                    &fence,
                                    shell.is_authenticated,
                                    shell.auth_generation,
                                    shell.history_request_generation,
                                    shell.session_token.as_deref(),
                                    shell.user_id,
                                    sb,
                                    st,
                                    chat_id,
                                    msgs,
                                );
                                if res {
                                    if let Some(ref rich) = rich_msgs {
                                        overlay_rich_message_metadata(&mut st.messages, rich);
                                        if let Some(hist) = st.chat_histories.get_mut(&chat_id) {
                                            overlay_rich_message_metadata(hist, rich);
                                        }
                                    }
                                    st_cx.notify();
                                }
                                res
                            })
                        });
                        if ok && shell.stage.read(cx).active_chat_id == Some(chat_id) {
                            let (_, media, files, links, voices) =
                                crate::views::app_shell::map_stage_media_to_gallery(
                                    &shell.stage.read(cx).messages,
                                );
                            shell.inspector.update(cx, |insp, insp_cx| {
                                insp.sync_shared_content(media, files, links, voices, Vec::new());
                                insp_cx.notify();
                            });
                        }
                    }
                    Err(err) => {
                        if fence.matches(
                            shell.is_authenticated,
                            shell.auth_generation,
                            shell.history_request_generation,
                            shell.session_token.as_deref(),
                            shell.user_id,
                        ) {
                            shell.window_runtime.enqueue_toast(
                                format!(
                                    "{}: {}",
                                    tr_str(
                                        shell.active_lang_id(),
                                        "加载聊天记录失败",
                                        "Failed to load history"
                                    ),
                                    err
                                ),
                                3200,
                            );
                        }
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn append_outgoing_attachment(&mut self, kind: &str, cx: &mut Context<Self>) {
        let lang = self.active_lang_id();
        match kind {
            "poll" => {
                self.open_modal_dialog(
                    tr_str(lang, "创建投票", "Create Poll"),
                    ModalDialogKind::CreatePoll,
                    420.0,
                    480.0,
                    cx,
                );
            }
            "voice" => {
                let start = !self.voice_recorder.is_recording();
                self.toggle_voice_recording(start, cx);
            }
            "photo" | "video" | "file" => {
                self.window_runtime.enqueue_toast(
                    tr_str(
                        lang,
                        "请将文件拖拽至聊天窗口上传发送",
                        "Drop a local file into the chat window to upload and send",
                    ),
                    2600,
                );
                cx.notify();
            }
            "location" => {
                self.window_runtime.enqueue_toast(
                    tr_str(
                        lang,
                        "位置共享需要选择真实位置后发送",
                        "Location sharing requires selecting a location",
                    ),
                    2600,
                );
                cx.notify();
            }
            _ => {}
        }
    }

    pub fn active_text_and_preedit(&self, cx: &App) -> (String, Option<String>) {
        if !self.is_authenticated {
            if self.editing_server_url {
                return (self.server_url_draft.clone(), self.ime_preedit.clone());
            }
            match self.login_step {
                LoginWizardStep::Start => {
                    let committed = match self.active_login_field {
                        LoginInputField::Username => self.username_input.clone(),
                        LoginInputField::Password => self.password_input.clone(),
                        LoginInputField::FirstName => self.first_name_input.clone(),
                        LoginInputField::Email => self.email_input.clone(),
                        LoginInputField::EmailCode => self.email_code_input.clone(),
                    };
                    (committed, self.ime_preedit.clone())
                }
                LoginWizardStep::Phone => (self.phone_number.clone(), None),
                LoginWizardStep::Code => (self.verification_code.clone(), None),
                LoginWizardStep::QrCode => (String::new(), None),
            }
        } else if self.sidebar.read(cx).active_creation_modal.is_some() {
            let sb = self.sidebar.read(cx);
            let committed = match sb.active_creation_field {
                CreationInputField::PrimaryName => sb.creation_primary_input.clone(),
                CreationInputField::SecondaryHandle => sb.creation_secondary_input.clone(),
                CreationInputField::TertiaryPromptOrPhone => sb.creation_tertiary_input.clone(),
            };
            (committed, self.ime_preedit.clone())
        } else if self.sidebar.read(cx).search_focused {
            (
                self.sidebar.read(cx).search_query.clone(),
                self.ime_preedit.clone(),
            )
        } else {
            let c = self.composer.read(cx);
            (
                c.text().to_string(),
                c.ime_machine.preedit_text().map(|s| s.to_string()),
            )
        }
    }

    pub fn has_active_ime_preedit(&self, cx: &App) -> bool {
        let (_, preedit) = self.active_text_and_preedit(cx);
        preedit.as_ref().is_some_and(|p| !p.is_empty())
    }

    pub fn clear_active_field(&mut self, cx: &mut Context<Self>) {
        self.ime_preedit = None;
        if !self.is_authenticated {
            if self.editing_server_url {
                self.server_url_draft.clear();
                cx.notify();
                return;
            }
            match self.login_step {
                LoginWizardStep::Start => match self.active_login_field {
                    LoginInputField::Username => self.username_input.clear(),
                    LoginInputField::Password => self.password_input.clear(),
                    LoginInputField::FirstName => self.first_name_input.clear(),
                    LoginInputField::Email => self.email_input.clear(),
                    LoginInputField::EmailCode => self.email_code_input.clear(),
                },
                LoginWizardStep::Phone => self.phone_number.clear(),
                LoginWizardStep::Code => self.verification_code.clear(),
                LoginWizardStep::QrCode => {}
            }
        } else if self.sidebar.read(cx).active_creation_modal.is_some() {
            self.sidebar.update(cx, |sb, sb_cx| {
                sb.clear_creation_text();
                sb_cx.notify();
            });
        } else if self.sidebar.read(cx).search_focused {
            self.sidebar.update(cx, |sb, sb_cx| {
                sb.set_search_query("");
                sb_cx.notify();
            });
        } else {
            self.composer.update(cx, |c, c_cx| {
                c.clear();
                c_cx.notify();
            });
        }
        cx.notify();
    }

    fn is_studio_input_active(&self, cx: &gpui::App) -> bool {
        (self.active_panel == ActiveWorkspacePanel::TaskCenter
            || self.active_panel == ActiveWorkspacePanel::McpTools)
            && self.tasks_workspace.read(cx).active_input
                != crate::views::global_tasks_workspace::StudioInputTarget::None
    }

    pub fn insert_committed_text(&mut self, raw_text: &str, cx: &mut Context<Self>) {
        self.ime_preedit = None;
        let clean: String = raw_text
            .chars()
            .filter(|&c| c != '\r' && c != '\n')
            .collect();
        if clean.is_empty() {
            cx.notify();
            return;
        }

        if !self.is_authenticated {
            if self.editing_server_url {
                self.server_url_draft.push_str(&clean);
                cx.notify();
                return;
            }
            match self.login_step {
                LoginWizardStep::Start => {
                    match self.active_login_field {
                        LoginInputField::Username => self.username_input.push_str(&clean),
                        LoginInputField::Password => self.password_input.push_str(&clean),
                        LoginInputField::FirstName => self.first_name_input.push_str(&clean),
                        LoginInputField::Email => self.email_input.push_str(&clean),
                        LoginInputField::EmailCode => {
                            for ch in clean.chars().filter(|c| c.is_ascii_digit()) {
                                if self.email_code_input.len() < 6 {
                                    self.email_code_input.push(ch);
                                }
                            }
                        }
                    }
                    cx.notify();
                }
                LoginWizardStep::Phone => {
                    let digits: String = clean
                        .chars()
                        .filter(|c| c.is_ascii_digit() || *c == ' ' || *c == '-')
                        .collect();
                    if !digits.is_empty() {
                        self.phone_number.push_str(&digits);
                        cx.notify();
                    }
                }
                LoginWizardStep::QrCode => {}
                LoginWizardStep::Code => {
                    for ch in clean.chars().filter(|c| c.is_ascii_digit()) {
                        if self.verification_code.len() < 6 {
                            self.verification_code.push(ch);
                        }
                    }
                    if self.verification_code.len() >= 5 {
                        self.submit_login(cx);
                    }
                    cx.notify();
                }
            }
            return;
        }

        if self.modal_container.read(cx).is_modal_active() {
            self.modal_container.update(cx, |mc, mc_cx| {
                mc.handle_key_input("", Some(&clean));
                mc_cx.notify();
            });
            cx.notify();
            return;
        }

        if self.sidebar.read(cx).active_creation_modal.is_some() {
            self.sidebar.update(cx, |sb, sb_cx| {
                sb.push_creation_text(&clean);
                sb_cx.notify();
            });
            cx.notify();
            return;
        }

        if self.sidebar.read(cx).search_focused {
            self.sidebar.update(cx, |sb, sb_cx| {
                let mut q = sb.search_query.clone();
                q.push_str(&clean);
                sb.set_search_query(q);
                sb_cx.notify();
            });
            cx.notify();
            return;
        }

        if self.is_studio_input_active(cx) {
            self.tasks_workspace.update(cx, |w, w_cx| {
                w.apply_text_key_to_active_input("", Some(&clean));
                w_cx.notify();
            });
            cx.notify();
            return;
        }

        self.composer.update(cx, |c, c_cx| {
            c.is_input_focused = true;
            if c.ime_machine.is_composing() {
                c.ime_machine.on_ime_commit(&clean);
            } else {
                c.ime_machine.insert_text(&clean);
            }
            c.sync_autocomplete();
            c_cx.notify();
        });
        cx.notify();
    }

    fn handle_non_char_key(&mut self, key: &str, is_shift: bool, cx: &mut Context<Self>) -> bool {
        // Escape dismisses topmost active layer first (viewer overlays -> modals -> drawer -> inspector -> settings -> search/reply)
        if key == "escape" {
            if !self.is_authenticated && self.editing_server_url {
                self.ime_preedit = None;
                self.editing_server_url = false;
                self.server_url_draft = self.client.config.server_url.clone();
                cx.notify();
                return true;
            }
            if self.ime_preedit.take().is_some() {
                cx.notify();
                return true;
            }
            if self.composer.read(cx).ime_machine.is_composing() {
                self.composer.update(cx, |c, c_cx| {
                    c.ime_machine.on_ime_cancel();
                    c.sync_autocomplete();
                    c_cx.notify();
                });
                cx.notify();
                return true;
            }
            if self.is_studio_input_active(cx) {
                self.tasks_workspace.update(cx, |w, w_cx| {
                    w.active_input = crate::views::global_tasks_workspace::StudioInputTarget::None;
                    w_cx.notify();
                });
                cx.notify();
                return true;
            }
            let ac_dismissed = self.composer.update(cx, |c, c_cx| {
                let ok = c.handle_hermes_autocomplete_nav("escape", is_shift);
                if ok {
                    c_cx.notify();
                }
                ok
            });
            if ac_dismissed {
                cx.notify();
                return true;
            }
            if self.iv_viewer_overlay.is_some() {
                self.iv_viewer_overlay = None;
                cx.notify();
                return true;
            }
            if self.media_viewer_overlay.is_some() {
                self.hide_media_viewer();
                cx.notify();
                return true;
            }
            if self.talk_mode_overlay.is_some() {
                self.hangup_active_call(cx);
                return true;
            }
            if self.computer_overlay.is_some() {
                self.hide_remote_computer(cx);
                cx.notify();
                return true;
            }
            if self.modal_container.read(cx).is_modal_active() {
                self.modal_container.update(cx, |mc, mc_cx| {
                    mc.dismiss_top();
                    mc_cx.notify();
                });
                cx.notify();
                return true;
            }
            if self.is_authenticated && self.sidebar.read(cx).is_contacts_modal_open {
                self.sidebar.update(cx, |sb, sb_cx| {
                    sb.close_contacts_modal();
                    sb_cx.notify();
                });
                cx.notify();
                return true;
            }
            if self.is_authenticated
                && (self.sidebar.read(cx).active_creation_modal.is_some()
                    || self.sidebar.read(cx).is_plus_menu_open)
            {
                self.sidebar.update(cx, |sb, sb_cx| {
                    sb.close_creation_modal();
                    sb_cx.notify();
                });
                self.composer.update(cx, |c, c_cx| {
                    c.is_input_focused = true;
                    c_cx.notify();
                });
                cx.notify();
                return true;
            }
            if self.main_menu_drawer.read(cx).is_open {
                self.main_menu_drawer.update(cx, |m, m_cx| {
                    m.close();
                    m_cx.notify();
                });
                cx.notify();
                return true;
            }
            if self.is_authenticated && self.inspector.read(cx).is_open {
                self.set_inspector_open(false, cx);
                return true;
            }
            if self.is_authenticated && self.active_panel == ActiveWorkspacePanel::Settings {
                self.navigate_settings_back(cx);
                return true;
            }
            if self.is_authenticated && self.stage.read(cx).search_open {
                self.stage.update(cx, |st, st_cx| {
                    st.search_open = false;
                    st.search_query.clear();
                    st.search_match_ids.clear();
                    st.search_match_cursor = 0;
                    st_cx.notify();
                });
                self.composer.update(cx, |c, c_cx| {
                    c.is_input_focused = true;
                    c_cx.notify();
                });
                cx.notify();
                return true;
            }
            if self.is_authenticated
                && (self.sidebar.read(cx).search_focused
                    || !self.sidebar.read(cx).search_query.is_empty())
            {
                self.sidebar.update(cx, |sb, sb_cx| {
                    sb.exit_search();
                    sb_cx.notify();
                });
                self.composer.update(cx, |c, c_cx| {
                    c.is_input_focused = true;
                    c_cx.notify();
                });
                cx.notify();
                return true;
            }
            if self.is_authenticated
                && (self.composer.read(cx).replying_to.is_some()
                    || self.stage.read(cx).pending_reply.is_some())
            {
                self.composer.update(cx, |c, c_cx| {
                    c.cancel_reply_or_edit();
                    c_cx.notify();
                });
                self.stage.update(cx, |st, st_cx| {
                    st.pending_reply = None;
                    st_cx.notify();
                });
                cx.notify();
                return true;
            }
            return false;
        }

        if self.is_authenticated && self.modal_container.read(cx).is_modal_active() {
            let handled = self.modal_container.update(cx, |mc, mc_cx| {
                let ok = mc.handle_key_input(key, None);
                if ok {
                    mc_cx.notify();
                }
                ok
            });
            if let Some(outcome) = self.modal_container.update(cx, |mc, _| mc.take_outcome()) {
                self.handle_modal_outcome(outcome, cx);
            }
            if handled {
                cx.notify();
                return true;
            }
        }

        if !self.is_authenticated && self.editing_server_url {
            match key {
                "enter" => {
                    if let Some(p) = self.ime_preedit.take() {
                        self.server_url_draft.push_str(&p);
                        cx.notify();
                    } else {
                        self.apply_custom_server_url(cx);
                    }
                    return true;
                }
                "escape" => {
                    self.ime_preedit = None;
                    self.editing_server_url = false;
                    self.server_url_draft = self.client.config.server_url.clone();
                    cx.notify();
                    return true;
                }
                "backspace" => {
                    if self.ime_preedit.take().is_none() {
                        self.server_url_draft.pop();
                    }
                    cx.notify();
                    return true;
                }
                _ => {}
            }
        }

        if !self.is_authenticated {
            match self.login_step {
                LoginWizardStep::Start => match key {
                    "enter" => {
                        if let Some(p) = self.ime_preedit.take() {
                            self.insert_committed_text(&p, cx);
                        } else if self.auth_wizard_tab == AuthWizardTab::AlipayQr {
                            self.open_alipay_web_auth(cx);
                        } else if self.auth_wizard_tab == AuthWizardTab::EmailRegister {
                            self.submit_email_register(cx);
                        } else if self.is_register_mode {
                            self.submit_register(cx);
                        } else {
                            self.submit_login(cx);
                        }
                        return true;
                    }
                    "tab" => {
                        self.ime_preedit = None;
                        self.active_login_field = if self.auth_wizard_tab == AuthWizardTab::EmailRegister {
                            match self.active_login_field {
                                LoginInputField::Email => LoginInputField::EmailCode,
                                LoginInputField::EmailCode => LoginInputField::Username,
                                LoginInputField::Username => LoginInputField::Password,
                                LoginInputField::Password => LoginInputField::FirstName,
                                LoginInputField::FirstName => LoginInputField::Email,
                            }
                        } else {
                            match self.active_login_field {
                                LoginInputField::Username => LoginInputField::Password,
                                LoginInputField::Password => {
                                    if self.is_register_mode {
                                        LoginInputField::FirstName
                                    } else {
                                        LoginInputField::Username
                                    }
                                }
                                LoginInputField::FirstName => LoginInputField::Username,
                                _ => LoginInputField::Username,
                            }
                        };
                        cx.notify();
                        return true;
                    }
                    "backspace" => {
                        if self.ime_preedit.take().is_none() {
                            match self.active_login_field {
                                LoginInputField::Username => {
                                    self.username_input.pop();
                                }
                                LoginInputField::Password => {
                                    self.password_input.pop();
                                }
                                LoginInputField::FirstName => {
                                    self.first_name_input.pop();
                                }
                                LoginInputField::Email => {
                                    self.email_input.pop();
                                }
                                LoginInputField::EmailCode => {
                                    self.email_code_input.pop();
                                }
                            }
                        }
                        cx.notify();
                        return true;
                    }
                    _ => return false,
                },
                LoginWizardStep::Phone => match key {
                    "enter" => {
                        if !self.phone_number.trim().is_empty() {
                            self.login_step = LoginWizardStep::Code;
                            self.login_error = None;
                        }
                        cx.notify();
                        return true;
                    }
                    "backspace" => {
                        self.phone_number.pop();
                        cx.notify();
                        return true;
                    }
                    _ => return false,
                },
                LoginWizardStep::QrCode => {
                    if key == "enter" {
                        self.submit_login(cx);
                        return true;
                    }
                    return false;
                }
                LoginWizardStep::Code => match key {
                    "enter" => {
                        self.submit_login(cx);
                        return true;
                    }
                    "backspace" => {
                        self.verification_code.pop();
                        cx.notify();
                        return true;
                    }
                    _ => return false,
                },
            }
        }

        if self.sidebar.read(cx).active_creation_modal.is_some() {
            match key {
                "tab" => {
                    self.ime_preedit = None;
                    self.sidebar.update(cx, |sb, sb_cx| {
                        sb.cycle_creation_field();
                        sb_cx.notify();
                    });
                    cx.notify();
                    return true;
                }
                "backspace" => {
                    if self.ime_preedit.take().is_none() {
                        self.sidebar.update(cx, |sb, sb_cx| {
                            sb.backspace_creation_text();
                            sb_cx.notify();
                        });
                    }
                    cx.notify();
                    return true;
                }
                "enter" => {
                    if let Some(p) = self.ime_preedit.take() {
                        self.insert_committed_text(&p, cx);
                    } else {
                        self.sidebar.update(cx, |sb, sb_cx| {
                            sb.submit_creation_modal();
                            sb_cx.notify();
                        });
                        cx.notify();
                    }
                    return true;
                }
                _ => return false,
            }
        }

        // Authenticated workspace non-char keys
        if self.is_studio_input_active(cx) {
            let handled = self.tasks_workspace.update(cx, |w, w_cx| {
                let ok = w.apply_text_key_to_active_input(key, None);
                if ok {
                    w_cx.notify();
                }
                ok
            });
            if handled {
                cx.notify();
                return true;
            }
        }

        if self.active_panel == ActiveWorkspacePanel::Chats
            && !self.sidebar.read(cx).search_focused
            && self.sidebar.read(cx).active_creation_modal.is_none()
            && matches!(key, "up" | "down" | "tab" | "enter")
        {
            let ac_handled = self.composer.update(cx, |c, c_cx| {
                let ok = c.handle_hermes_autocomplete_nav(key, is_shift);
                if ok {
                    c_cx.notify();
                }
                ok
            });
            if ac_handled {
                cx.notify();
                return true;
            }
        }

        if key == "tab" {
            let currently_search = self.sidebar.read(cx).search_focused;
            self.sidebar.update(cx, |sb, sb_cx| {
                if currently_search {
                    sb.exit_search();
                } else {
                    sb.focus_search();
                }
                sb_cx.notify();
            });
            self.composer.update(cx, |c, c_cx| {
                c.is_input_focused = currently_search;
                c_cx.notify();
            });
            cx.notify();
            return true;
        }

        let is_search_focused = self.sidebar.read(cx).search_focused;
        if is_search_focused {
            match key {
                "backspace" => {
                    if self.ime_preedit.take().is_none() {
                        self.sidebar.update(cx, |sb, sb_cx| {
                            let mut q = sb.search_query.clone();
                            q.pop();
                            sb.set_search_query(q);
                            sb_cx.notify();
                        });
                    }
                    cx.notify();
                    return true;
                }
                "enter" => {
                    if let Some(p) = self.ime_preedit.take() {
                        self.insert_committed_text(&p, cx);
                    } else {
                        self.sidebar.update(cx, |sb, sb_cx| {
                            sb.search_focused = false;
                            sb_cx.notify();
                        });
                        self.composer.update(cx, |c, c_cx| {
                            c.is_input_focused = true;
                            c_cx.notify();
                        });
                        cx.notify();
                    }
                    return true;
                }
                _ => return false,
            }
        }

        match key {
            "enter" => {
                // Shell-level IME commit fence check
                if self.just_committed_ime {
                    self.just_committed_ime = false;
                    self.composer.update(cx, |c, c_cx| {
                        c.ime_machine.reset_commit_fence();
                        c_cx.notify();
                    });
                    cx.notify();
                    return true;
                }

                // Delegate Enter handling to CanonicalComposer
                let action = self.composer.update(cx, |c, c_cx| {
                    let act = c.handle_enter(is_shift);
                    c_cx.notify();
                    act
                });

                match action {
                    KeyAction::SendMessage => {
                        self.composer.update(cx, |c, c_cx| {
                            c.submit_primary();
                            c_cx.notify();
                        });
                        self.handle_composer_pending_actions(cx);
                    }
                    KeyAction::InsertNewline | KeyAction::ImeCandidateChosen | KeyAction::DefaultEdit | KeyAction::None => {
                        // Suppressed: strictly NO message dispatch!
                    }
                }
                cx.notify();
                true
            }
            "backspace" => {
                self.composer.update(cx, |c, c_cx| {
                    if c.ime_machine.is_composing() {
                        c.ime_machine.on_ime_cancel();
                    } else {
                        c.ime_machine.backspace();
                    }
                    c.sync_autocomplete();
                    c_cx.notify();
                });
                cx.notify();
                true
            }
            _ => false,
        }
    }

    /// Synchronizes `ContextInspector` open state with `ConversationStage::is_inspector_open`.
    pub fn set_inspector_open(&mut self, open: bool, cx: &mut Context<Self>) {
        self.inspector.update(cx, |insp, insp_cx| {
            if open {
                insp.open();
            } else {
                insp.close();
            }
            insp_cx.notify();
        });
        self.stage.update(cx, |st, st_cx| {
            st.is_inspector_open = open;
            st_cx.notify();
        });
        cx.notify();
    }

    /// Toggles the active interface language (`zh-CN` <-> `en-US`) and persists preferences.
    pub fn toggle_language(&mut self, cx: &mut Context<Self>) {
        let next_lang = toggled_language_code(&self.preferences.localization.language_code);
        self.update_preferences(cx, |p| {
            p.localization.language_code = next_lang.to_string();
        });
    }

    /// Switches to a `NavDestination` panel and synchronizes `NavRail` + workspace data cleanly.
    pub fn activate_nav_destination(&mut self, dest: NavDestination, cx: &mut Context<Self>) {
        self.last_handled_nav_dest = Some(dest);
        self.nav_rail.update(cx, |r, r_cx| {
            r.set_active_destination(dest);
            r_cx.notify();
        });
        match dest {
            NavDestination::Chats | NavDestination::Folders | NavDestination::SavedMessages => {
                self.active_panel = ActiveWorkspacePanel::Chats;
                let target_saved = if dest == NavDestination::SavedMessages {
                    let sb = self.sidebar.read(cx);
                    sb.saved_messages_id.or_else(|| {
                        let uid = self.user_id?;
                        sb.dialogs.iter().find(|d| d.id == uid).map(|d| d.id)
                    })
                } else {
                    None
                };
                self.sidebar.update(cx, |sb, sb_cx| {
                    match dest {
                        NavDestination::Chats => sb.select_folder("All"),
                        NavDestination::Folders => {
                            if let Some(first_folder) = sb
                                .folders
                                .iter()
                                .find(|(f, _)| f != "All")
                                .map(|(f, _)| f.clone())
                            {
                                sb.select_folder(first_folder);
                            }
                        }
                        NavDestination::SavedMessages => {
                            if let Some(saved_id) = target_saved {
                                sb.select_dialog(saved_id);
                            }
                        }
                        _ => {}
                    }
                    sb_cx.notify();
                });
                if let Some(saved_id) = target_saved {
                    self.sync_selected_chat_inspector_and_stage(saved_id, cx);
                }
            }
            NavDestination::Calls => {
                let shell = self;
                shell.active_panel = ActiveWorkspacePanel::Calls;
                shell.sync_calls_history(cx);
            }
            NavDestination::TalkMode => {
                if self.is_active_chat_bot(cx) {
                    self.start_ai_talk_mode_session(false, cx);
                } else if let Some(pid) = self.current_peer_user_id(cx) {
                    let name = self.stage.read(cx).chat_title.clone();
                    self.start_webrtc_call(pid, name, false, cx);
                } else {
                    self.show_talk_mode(cx);
                }
            }
            NavDestination::RemoteComputer => {
                self.open_computer_workbench_live(cx);
            }
            NavDestination::TaskCenter => {
                self.active_panel = ActiveWorkspacePanel::TaskCenter;
                self.tasks_workspace
                    .update(cx, |w, _| w.set_tab(TasksWorkspaceTab::Tasks));
                self.refresh_tasks_and_mcp(cx);
            }
            NavDestination::McpTools => {
                self.active_panel = ActiveWorkspacePanel::McpTools;
                self.tasks_workspace
                    .update(cx, |w, _| w.set_tab(TasksWorkspaceTab::McpTools));
                self.refresh_tasks_and_mcp(cx);
            }
            NavDestination::Settings => {
                if self.active_panel == ActiveWorkspacePanel::Settings {
                    self.settings_section = SettingsSectionType::Main;
                    self.settings_stack.clear();
                } else {
                    self.active_panel = ActiveWorkspacePanel::Settings;
                    self.refresh_wallet(cx);
                }
            }
        }
        cx.notify();
    }

    fn handle_shortcut_key(&mut self, key: &str, is_shift: bool, cx: &mut Context<Self>) -> bool {
        let key_lower = key.to_ascii_lowercase();
        match key_lower.as_str() {
            "v" => {
                if let Some(clip_text) = cx.read_from_clipboard().and_then(|item| item.text()) {
                    self.insert_committed_text(&clip_text, cx);
                }
                return true;
            }
            "backspace" | "a" | "u" => {
                self.clear_active_field(cx);
                return true;
            }
            "d" => {
                let next = next_tdesktop_theme_cycle(self.theme_mode);
                self.apply_theme_selection(next, None, None, cx);
                return true;
            }
            "l" => {
                self.toggle_language(cx);
                return true;
            }
            _ => {}
        }

        if !self.is_authenticated {
            return false;
        }

        let lang = self.active_lang_id();
        match key_lower.as_str() {
            "k" => {
                self.sidebar.update(cx, |sb, sb_cx| {
                    sb.focus_search();
                    sb_cx.notify();
                });
                self.composer.update(cx, |c, c_cx| {
                    c.is_input_focused = false;
                    c_cx.notify();
                });
                cx.notify();
                true
            }
            "," if self.is_authenticated => {
                self.navigate_settings_to(SettingsSectionType::Main, cx);
                true
            }
            "[" if self.active_panel == ActiveWorkspacePanel::Settings => {
                self.navigate_settings_back(cx);
                true
            }
            "i" => {
                let next_open = !self.inspector.read(cx).is_open;
                self.set_inspector_open(next_open, cx);
                true
            }
            "e" => {
                self.main_menu_drawer.update(cx, |d, d_cx| {
                    d.toggle();
                    d_cx.notify();
                });
                cx.notify();
                true
            }
            "j" => {
                self.refresh_contacts(cx);
                self.open_modal_dialog(
                    tr_str(lang, "联系人", "Contacts"),
                    ModalDialogKind::Contacts,
                    420.0,
                    500.0,
                    cx,
                );
                true
            }
            "n" => {
                if is_shift {
                    self.open_modal_dialog(
                        tr_str(lang, "新建频道", "New Channel"),
                        ModalDialogKind::NewChannel,
                        420.0,
                        380.0,
                        cx,
                    );
                } else {
                    self.open_modal_dialog(
                        tr_str(lang, "新建群组", "New Group"),
                        ModalDialogKind::NewGroup,
                        420.0,
                        380.0,
                        cx,
                    );
                }
                true
            }
            "p" => {
                self.open_modal_dialog(
                    tr_str(lang, "创建投票", "Create Poll"),
                    ModalDialogKind::CreatePoll,
                    420.0,
                    480.0,
                    cx,
                );
                true
            }
            digit => {
                if let Some(dest) = nav_destination_for_shortcut_digit(digit) {
                    self.activate_nav_destination(dest, cx);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn handle_window_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let key = event.keystroke.key.as_str();

        // 1. Handle Cmd / Ctrl shortcuts (Theme, Language, Navigation, Modals, Search, Paste)
        if event.keystroke.modifiers.platform || event.keystroke.modifiers.control {
            if self.handle_shortcut_key(key, event.keystroke.modifiers.shift, cx) {
                cx.stop_propagation();
            }
            return;
        }

        // 2. Handle non-character control keys (Enter, Backspace, Tab, Escape)
        if self.handle_non_char_key(key, event.keystroke.modifiers.shift, cx) {
            cx.stop_propagation();
        }
        // 3. Printable characters are NOT stopped here so GPUI's macOS NSTextInputClient
        //    routes both English keys (`insertText:`) and Chinese Pinyin (`setMarkedText:`)
        //    through `EntityInputHandler` without duplication.
    }

    pub fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
        let key = event.keystroke.key.as_str();

        if event.keystroke.modifiers.platform || event.keystroke.modifiers.control {
            self.handle_shortcut_key(key, event.keystroke.modifiers.shift, cx);
            return;
        }

        if self.handle_non_char_key(key, event.keystroke.modifiers.shift, cx) {
            return;
        }

        if key == "space" {
            self.insert_committed_text(" ", cx);
        } else if let Some(ref ch) = event.keystroke.key_char {
            self.insert_committed_text(ch, cx);
        }
    }
}

impl EntityInputHandler for AppShell {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        _adjusted_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<String> {
        let (committed, preedit) = self.active_text_and_preedit(cx);
        let full = match preedit {
            Some(p) => format!("{}{}", committed, p),
            None => committed,
        };
        let start = utf16_offset_to_utf8(&full, range_utf16.start);
        let end = utf16_offset_to_utf8(&full, range_utf16.end).max(start);
        Some(full.get(start..end).unwrap_or("").to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let (committed, preedit) = self.active_text_and_preedit(cx);
        let total_utf16 =
            utf16_len(&committed) + preedit.as_deref().map_or(0, utf16_len);
        Some(UTF16Selection {
            range: total_utf16..total_utf16,
            reversed: false,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        let (committed, preedit) = self.active_text_and_preedit(cx);
        if let Some(p) = preedit {
            if !p.is_empty() {
                let start = utf16_len(&committed);
                let end = start + utf16_len(&p);
                return Some(start..end);
            }
        }
        None
    }

    fn unmark_text(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.just_committed_ime = true;
        if let Some(p) = self.ime_preedit.take() {
            if !p.is_empty() {
                self.insert_committed_text(&p, cx);
                return;
            }
        }
        let preedit_opt = self
            .composer
            .read(cx)
            .ime_machine
            .preedit_text()
            .map(|s| s.to_string());
        if let Some(p) = preedit_opt {
            self.composer.update(cx, |c, c_cx| {
                c.ime_machine.on_ime_commit(&p);
                c_cx.notify();
            });
        }
        cx.notify();
    }

    fn replace_text_in_range(
        &mut self,
        _range_utf16: Option<Range<usize>>,
        text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let was_composing = self.has_active_ime_preedit(cx) || self.composer.read(cx).ime_machine.is_composing();
        self.insert_committed_text(text, cx);
        if was_composing {
            self.just_committed_ime = true;
        }
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        _range_utf16: Option<Range<usize>>,
        new_text: &str,
        _new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let sidebar_capturing = {
            let sb = self.sidebar.read(cx);
            sb.search_focused || sb.active_creation_modal.is_some()
        } || self.is_studio_input_active(cx);
        if new_text.is_empty() {
            self.ime_preedit = None;
            if self.is_authenticated && !sidebar_capturing {
                self.composer.update(cx, |c, c_cx| {
                    c.ime_machine.on_ime_cancel();
                    c.sync_autocomplete();
                    c_cx.notify();
                });
            }
            cx.notify();
            return;
        }

        if !self.is_authenticated || sidebar_capturing {
            self.ime_preedit = Some(new_text.to_string());
        } else {
            self.composer.update(cx, |c, c_cx| {
                c.is_input_focused = true;
                c.ime_machine.on_ime_update(new_text, new_text.len());
                c.sync_autocomplete();
                c_cx.notify();
            });
        }
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: Range<usize>,
        element_bounds: Bounds<Pixels>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        if !self.is_authenticated {
            let center_x = element_bounds.origin.x + element_bounds.size.width / 2.0;
            let center_y = element_bounds.origin.y + element_bounds.size.height / 2.0;
            let (committed, preedit) = self.active_text_and_preedit(cx);
            let char_count = committed.chars().count() + preedit.as_deref().map_or(0, |p| p.chars().count());
            let caret_x = center_x - px(120.0) + px((char_count as f32 * 8.0).min(240.0));
            let caret_y = center_y + px(10.0);
            return Some(Bounds::new(point(caret_x, caret_y), size(px(2.0), px(20.0))));
        }

        if self.sidebar.read(cx).active_creation_modal.is_some() {
            let (committed, preedit) = self.active_text_and_preedit(cx);
            let char_count = committed.chars().count() + preedit.as_deref().map_or(0, |p| p.chars().count());
            let caret_x = element_bounds.origin.x + px(52.0 + 24.0) + px((char_count as f32 * 7.5).min(220.0));
            let caret_y = element_bounds.origin.y + px(160.0);
            return Some(Bounds::new(point(caret_x, caret_y), size(px(2.0), px(18.0))));
        }

        if self.sidebar.read(cx).search_focused {
            let q = self.sidebar.read(cx).search_query.clone();
            let char_count = q.chars().count();
            let caret_x = element_bounds.origin.x + px(52.0 + 16.0 + 28.0) + px((char_count as f32 * 7.5).min(200.0));
            let caret_y = element_bounds.origin.y + px(22.0);
            return Some(Bounds::new(point(caret_x, caret_y), size(px(2.0), px(18.0))));
        }

        // CanonicalComposer caret positioning at bottom of chat stage
        let (committed, preedit) = self.active_text_and_preedit(cx);
        let full_text = match &preedit {
            Some(p) => format!("{}{}", committed, p),
            None => committed,
        };

        let mut caret_advance_px = 0.0f32;
        for ch in full_text.chars() {
            if ch == '\n' {
                caret_advance_px = 0.0;
            } else if ch.is_ascii() {
                caret_advance_px += 8.2;
            } else {
                caret_advance_px += 14.0;
            }
        }

        let max_text_width = f32::from(element_bounds.size.width) - 458.0 - 360.0;
        let clamped_advance = caret_advance_px.clamp(0.0, max_text_width.max(100.0));
        let caret_x = element_bounds.origin.x + px(458.0 + clamped_advance);
        let caret_y = element_bounds.origin.y + element_bounds.size.height - px(36.0);

        Some(Bounds::new(point(caret_x, caret_y), size(px(2.0), px(20.0))))
    }

    fn character_index_for_point(
        &mut self,
        _point: Point<Pixels>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<usize> {
        let (committed, preedit) = self.active_text_and_preedit(cx);
        Some(utf16_len(&committed) + preedit.as_deref().map_or(0, utf16_len))
    }
}

impl Render for AppShell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.focus_handle.is_focused(window) {
            self.focus_handle.focus(window);
        }
        let vp = window.viewport_size();
        let vp_w = f32::from(vp.width);
        let vp_h = f32::from(vp.height);
        if (vp_w - self.last_viewport_width).abs() >= 1.0 && vp_w > 0.0 {
            self.last_viewport_width = vp_w;
            self.app.sync_viewport_size(vp_w, vp_h);
        }
        let pal = self.palette;
        let lang = self.active_lang_id();

        let focus_handle = self.focus_handle.clone();
        let view = cx.entity().clone();
        let ime_canvas = canvas(
            |bounds, _window, _cx| bounds,
            move |bounds, _, window, cx| {
                window.handle_input(
                    &focus_handle,
                    ElementInputHandler::new(bounds, view.clone()),
                    cx,
                );
            },
        )
        .size_0();

        if !self.is_authenticated {
            return div()
                .id("app-shell-login-root")
                .track_focus(&self.focus_handle)
                .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                    this.handle_window_key_down(event, window, cx);
                }))
                .size_full()
                .bg(rgb(pal.window_bg))
                .flex()
                .items_center()
                .justify_center()
                .child(ime_canvas)
                .child(self.render_login_wizard(cx))
                .into_any_element();
        }

        let show_conn_banner = self.window_runtime.connecting_widget.is_visible;
        let conn_banner_text = self.window_runtime.connecting_widget.status_text();
        let current_toast = self.window_runtime.controller.current_toast.clone();

        div()
            .id("app-shell-authenticated-root")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                this.handle_window_key_down(event, window, cx);
            }))
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(pal.window_bg))
            .child(ime_canvas)
            .when(show_conn_banner, |el| {
                el.child(
                    div()
                        .w_full()
                        .h(px(26.0))
                        .bg(rgb(pal.accent))
                        .text_color(rgb(pal.active_button_fg))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(conn_banner_text),
                )
            })
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .min_h(px(0.0))
                    .flex()
                    // 1. Left Primary Nav Rail (52px)
                    .child(self.nav_rail.clone())
                    // 2. Chat Sidebar (288px)
                    .child(self.sidebar.clone())
                    // 3. Main Stage + Composer Column (or Calls / TaskCenter / McpTools / Settings Panel)
                    .child(match self.active_panel {
                        ActiveWorkspacePanel::Chats => div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .min_w(px(0.0))
                            .h_full()
                            .overflow_hidden()
                            .child(self.stage.clone())
                            .child(self.composer.clone())
                            .into_any_element(),
                        ActiveWorkspacePanel::Calls => self.render_calls_panel(cx).into_any_element(),
                        ActiveWorkspacePanel::TaskCenter => self.render_task_center_panel(cx).into_any_element(),
                        ActiveWorkspacePanel::McpTools => self.render_mcp_tools_panel(cx).into_any_element(),
                        ActiveWorkspacePanel::Settings => self.render_settings_panel(cx).into_any_element(),
                    })
                    // 4. Right Context Inspector (320px)
                    .child(self.inspector.clone()),
            )
            // 5. Capability Overlays (Floating / Fullscreen layers)
            .when_some(self.computer_overlay.clone(), |el, overlay| {
                el.child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .occlude()
                        .child(overlay),
                )
            })
            .when_some(self.talk_mode_overlay.clone(), |el, overlay| {
                let is_ai_talk = self.active_talk_session_is_ai;
                let is_rec = self.voice_recorder.is_recording();
                let talk_lang = self.active_lang_id();
                let talk_pal = self.palette;
                el.child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .occlude()
                        .bg(rgba(0x00000088))
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap(px(12.0))
                        .child(overlay)
                        .when(is_ai_talk, |wrap| {
                            wrap.child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(10.0))
                                    .px(px(16.0))
                                    .py(px(10.0))
                                    .rounded(px(14.0))
                                    .bg(rgb(talk_pal.card_bg))
                                    .border_1()
                                    .border_color(rgb(talk_pal.border_fg))
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .px(px(12.0))
                                            .py(px(6.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(if is_rec { talk_pal.danger } else { talk_pal.accent }))
                                            .text_size(px(12.5))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _e, _w, cx| {
                                                    let next = !this.voice_recorder.is_recording();
                                                    this.toggle_voice_recording(next, cx);
                                                }),
                                            )
                                            .child(if is_rec {
                                                tr_str(talk_lang, "停止并发送语音", "Stop & Send Voice")
                                            } else {
                                                tr_str(talk_lang, "按住/点击说话", "Push to Talk")
                                            }),
                                    )
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .px(px(12.0))
                                            .py(px(6.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(talk_pal.input_bg))
                                            .border_1()
                                            .border_color(rgb(talk_pal.border_fg))
                                            .text_size(px(12.5))
                                            .text_color(rgb(talk_pal.text_primary))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _e, _w, cx| {
                                                    let draft = this.composer.read(cx).text().trim().to_string();
                                                    let prompt = if !draft.is_empty() {
                                                        this.composer.update(cx, |c, _| c.set_text(String::new()));
                                                        draft
                                                    } else {
                                                        tr_str(
                                                            this.active_lang_id(),
                                                            "请用简洁的语音总结当前状态",
                                                            "Give me a concise spoken briefing of my current status.",
                                                        )
                                                        .to_string()
                                                    };
                                                    this.send_ai_talk_voice_turn(prompt, true, cx);
                                                }),
                                            )
                                            .child(tr_str(talk_lang, "发送对话回合", "Send Voice Turn")),
                                    )
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .px(px(12.0))
                                            .py(px(6.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(talk_pal.input_bg))
                                            .border_1()
                                            .border_color(rgb(talk_pal.danger))
                                            .text_size(px(12.5))
                                            .text_color(rgb(talk_pal.danger))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _e, _w, cx| {
                                                    this.interrupt_ai_talk_barge_in(cx);
                                                }),
                                            )
                                            .child(tr_str(talk_lang, "打断播报 (<60ms)", "Barge-In (<60ms)")),
                                    ),
                            )
                        }),
                )
            })
            .when_some(self.media_viewer_overlay.clone(), |el, overlay| {
                el.child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .occlude()
                        .child(overlay),
                )
            })
            .when_some(self.iv_viewer_overlay.clone(), |el, overlay| {
                el.child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .occlude()
                        .child(overlay),
                )
            })
            // 5b. Contacts Modal Overlay backed by contacts_list + dialogs
            .when(self.sidebar.read(cx).is_contacts_modal_open, |el| {
                let sb_ref = self.sidebar.read(cx);
                let contact_q = sb_ref.contacts_search_query.clone();
                let q_lower = contact_q.trim().to_lowercase();
                let mut rows: Vec<(u64, Option<u64>, String, String, String, u32)> = Vec::new();
                for c in &self.contacts_list {
                    let full = format!(
                        "{} {}",
                        c.first_name,
                        c.last_name.as_deref().unwrap_or("")
                    )
                    .trim()
                    .to_string();
                    let name = if full.is_empty() {
                        c.username
                            .clone()
                            .unwrap_or_else(|| format!("User {}", c.user_id))
                    } else {
                        full
                    };
                    let sub = c
                        .username
                        .as_deref()
                        .map(|u| format!("@{u}"))
                        .or_else(|| c.phone.clone())
                        .unwrap_or_default();
                    let matched = q_lower.is_empty()
                        || name.to_lowercase().contains(&q_lower)
                        || sub.to_lowercase().contains(&q_lower);
                    if matched {
                        let resolved_chat = c.chat_id.map(|id| id as u64).or_else(|| {
                            sb_ref
                                .dialogs
                                .iter()
                                .find(|d| d.id == c.user_id || d.peer_name == name)
                                .map(|d| d.id)
                        });
                        rows.push((
                            c.user_id,
                            resolved_chat,
                            crate::views::app_shell::shell_truth::derive_initials(&name),
                            name,
                            sub,
                            crate::views::app_shell::peer_avatar_color(c.user_id),
                        ));
                    }
                }
                for d in &sb_ref.dialogs {
                    if d.folder != "Groups"
                        && !d.peer_name.contains("Telegram Official Bot")
                        && !rows.iter().any(|(_, cid, _, n, _, _)| *cid == Some(d.id) || n == &d.peer_name)
                        && (q_lower.is_empty()
                            || d.peer_name.to_lowercase().contains(&q_lower)
                            || d.last_message.to_lowercase().contains(&q_lower))
                    {
                        rows.push((
                            d.id,
                            Some(d.id),
                            d.avatar_initials.clone(),
                            d.peer_name.clone(),
                            peer_subtitle_for_dialog(d),
                            d.avatar_color,
                        ));
                    }
                }

                el.child(
                    div()
                        .id("telegram-contacts-modal-backdrop")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .occlude()
                        .bg(rgba(0x00000088))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .id("telegram-contacts-modal-box")
                                .w(px(360.0))
                                .h(px(480.0))
                                .rounded(px(12.0))
                                .bg(rgb(pal.modal_bg))
                                .border_1()
                                .border_color(rgb(pal.border_fg))
                                .flex()
                                .flex_col()
                                .overflow_hidden()
                                .child(
                                    div()
                                        .w_full()
                                        .px(px(20.0))
                                        .py(px(14.0))
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .text_size(px(16.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(pal.text_primary))
                                                .child(tr_str(lang, "联系人", "Contacts")),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(12.0))
                                                .text_color(rgb(pal.text_secondary))
                                                .child(format!("{} {}", rows.len(), tr_str(lang, "位联系人", "contacts"))),
                                        ),
                                )
                                .child(
                                    div()
                                        .w_full()
                                        .px(px(16.0))
                                        .py(px(8.0))
                                        .border_b_1()
                                        .border_color(rgb(pal.border_fg))
                                        .flex()
                                        .items_center()
                                        .gap(px(8.0))
                                        .child(tg_icon(IconName::Search, 14.0, pal.icon_fg))
                                        .child(
                                            div()
                                                .text_size(px(13.5))
                                                .text_color(if contact_q.is_empty() {
                                                    rgb(pal.text_secondary)
                                                } else {
                                                    rgb(pal.text_primary)
                                                })
                                                .child(if contact_q.is_empty() {
                                                    tr_str(lang, "搜索", "Search").to_string()
                                                } else {
                                                    contact_q
                                                }),
                                        ),
                                )
                                .child(
                                    div()
                                        .id("contacts-modal-scroll")
                                        .flex_1()
                                        .w_full()
                                        .overflow_y_scroll()
                                        .flex()
                                        .flex_col()
                                        .when(rows.is_empty(), |body| {
                                            body.items_center().justify_center().child(
                                                div()
                                                    .text_size(px(13.5))
                                                    .text_color(rgb(pal.text_secondary))
                                                    .child(tr_str(lang, "未找到相关联系人", "No matching contacts")),
                                            )
                                        })
                                        .when(!rows.is_empty(), |body| {
                                            body.children(rows.into_iter().map(|(uid, cid_opt, initials, name, sub, col)| {
                                                let click_name = name.clone();
                                                div()
                                                    .cursor_pointer()
                                                    .w_full()
                                                    .px(px(16.0))
                                                    .py(px(10.0))
                                                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(12.0))
                                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                                        this.sidebar.update(cx, |sb, sb_cx| {
                                                            sb.close_contacts_modal();
                                                            if let Some(cid) = cid_opt {
                                                                sb.select_dialog_from_user(cid);
                                                            }
                                                            sb_cx.notify();
                                                        });
                                                        if let Some(cid) = cid_opt {
                                                            this.sync_selected_chat_inspector_and_stage(cid, cx);
                                                        } else {
                                                            let req = gpui_telegram_proto::contracts::chats::CreateChatRequest {
                                                                title: click_name.clone(),
                                                                chat_type: Some(serde_json::json!(0)),
                                                                peer_user_id: Some(uid),
                                                                ..Default::default()
                                                            };
                                                            let client = this.client.clone();
                                                            cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
                                                                if let Ok(created) = client.create_chat(&req).await {
                                                                    let _ = weak.update(app, |s, cx| {
                                                                        s.sidebar.update(cx, |sb, _| sb.select_dialog_from_user(created.chat_id as u64));
                                                                        s.refresh_chats(cx);
                                                                    });
                                                                }
                                                            })
                                                            .detach();
                                                        }
                                                        cx.notify();
                                                    }))
                                                    .child(
                                                        div()
                                                            .w(px(40.0))
                                                            .h(px(40.0))
                                                            .rounded_full()
                                                            .bg(rgb(col))
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::BOLD)
                                                            .text_color(rgb(pal.active_button_fg))
                                                            .child(initials),
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .child(
                                                                div()
                                                                    .text_size(px(14.0))
                                                                    .font_weight(FontWeight::BOLD)
                                                                    .text_color(rgb(pal.text_primary))
                                                                    .child(name),
                                                            )
                                                            .when(!sub.is_empty(), |el| {
                                                                el.child(
                                                                    div()
                                                                        .text_size(px(12.0))
                                                                        .text_color(rgb(pal.accent))
                                                                        .child(sub),
                                                                )
                                                            }),
                                                    )
                                            }))
                                        }),
                                )
                                .child(
                                    div()
                                        .w_full()
                                        .px(px(16.0))
                                        .py(px(12.0))
                                        .border_t_1()
                                        .border_color(rgb(pal.border_fg))
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .id("contacts-modal-add-btn")
                                                .cursor_pointer()
                                                .px(px(10.0))
                                                .py(px(6.0))
                                                .rounded(px(6.0))
                                                .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                                .text_size(px(13.5))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(pal.accent))
                                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                    this.sidebar.update(cx, |sb, sb_cx| {
                                                        sb.close_contacts_modal();
                                                        sb_cx.notify();
                                                    });
                                                    let l = this.active_lang_id();
                                                    this.open_modal_dialog(
                                                        tr_str(l, "添加联系人", "Add Contact"),
                                                        ModalDialogKind::AddContact,
                                                        380.0,
                                                        360.0,
                                                        cx,
                                                    );
                                                }))
                                                .child(tr_str(lang, "添加联系人", "Add Contact")),
                                        )
                                        .child(
                                            div()
                                                .id("contacts-modal-close-btn")
                                                .cursor_pointer()
                                                .px(px(10.0))
                                                .py(px(6.0))
                                                .rounded(px(6.0))
                                                .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                                .text_size(px(13.5))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(pal.accent))
                                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                    this.sidebar.update(cx, |sb, sb_cx| {
                                                        sb.close_contacts_modal();
                                                        sb_cx.notify();
                                                    });
                                                    cx.notify();
                                                }))
                                                .child(tr_str(lang, "关闭", "Close")),
                                        ),
                                ),
                        ),
                )
            })
            // 5c. Slide-out Main Menu Drawer
            .child(self.main_menu_drawer.clone())
            // 6. Modal Layer Stack Container
            .child(self.modal_container.clone())
            // 7. Window Toast Banner
            .when_some(current_toast, |el, toast| {
                el.child(
                    div()
                        .id("window-toast-banner")
                        .cursor_pointer()
                        .absolute()
                        .bottom(px(28.0))
                        .right(px(28.0))
                        .px(px(16.0))
                        .py(px(9.0))
                        .rounded(px(16.0))
                        .bg(rgb(pal.card_bg))
                        .border_1()
                        .border_color(rgb(pal.border_fg))
                        .shadow_lg()
                        .text_size(px(12.5))
                        .text_color(rgb(pal.text_primary))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                            this.window_runtime.controller.dismiss_current_toast();
                            cx.notify();
                        }))
                        .child(toast.text),
                )
            })
            .into_any_element()
    }
}

impl AppShell {
    fn render_login_wizard(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let zh = self.is_chinese_locale;
        let pal = self.palette;
        let lang = self.active_lang_id();
        let step = self.login_step;
        let country = self.country_code.clone();
        let phone = self.phone_number.clone();
        let code = self.verification_code.clone();

        div()
            .w(px(440.0))
            .rounded(px(16.0))
            .bg(rgb(pal.modal_bg))
            .border_1()
            .border_color(rgb(pal.border_fg))
            .p(px(32.0))
            .flex()
            .flex_col()
            .items_center()
            .gap(px(18.0))
            // Top Controls Row (Language + Theme Switcher)
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(pal.text_secondary))
                            .child("GPUI Telegram Desktop • MTProto 2.0"),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .child({
                                let is_dark = self.theme_mode.is_dark();
                                div()
                                    .cursor_pointer()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.input_bg))
                                    .border_1()
                                    .border_color(rgb(pal.border_fg))
                                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                    .flex()
                                    .items_center()
                                    .gap(px(5.0))
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        let next = toggle_desktop_theme_mode(this.theme_mode);
                                        this.apply_theme_selection(next, None, None, cx);
                                    }))
                                    .child(tg_icon(
                                        if is_dark { IconName::SunDay } else { IconName::NightMoon },
                                        13.0,
                                        pal.icon_fg,
                                    ))
                                    .child(if is_dark {
                                        tr_str(lang, "日间", "Day")
                                    } else {
                                        tr_str(lang, "夜间", "Night")
                                    })
                            })
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.input_bg))
                                    .border_1()
                                    .border_color(rgb(pal.border_fg))
                                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                    .flex()
                                    .items_center()
                                    .gap(px(5.0))
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.toggle_language(cx);
                                    }))
                                    .child(tg_icon(IconName::Translate, 13.0, pal.icon_fg))
                                    .child(tr_str(lang, "中文 / EN", "EN / 中文")),
                            ),
                    ),
            )
            // Telegram Plane Logo Circle
            .child(
                div()
                    .w(px(84.0))
                    .h(px(84.0))
                    .rounded_full()
                    .bg(rgb(pal.accent))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(tg_icon(IconName::TelegramPlaneLogo, 44.0, pal.active_button_fg)),
            )
            // Step-specific Body
            .child(match step {
                LoginWizardStep::Start => {
                    let server_url = self.client.config.server_url.clone();

                    div()
                        .w_full()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(12.0))
                        // Cloud Server Status & Editable Endpoint Bar
                        .child({
                            let has_net_err = self
                                .login_error
                                .as_deref()
                                .is_some_and(|e| e.contains("Network connection error") || e.contains("timed out") || e.contains("Connection refused"));
                            let dot_color = if has_net_err { rgb(pal.danger) } else { rgb(pal.success) };
                            if self.editing_server_url {
                                let draft_display = if self.server_url_draft.is_empty() {
                                    "http://<Google-Cloud-IP>:8080".to_string()
                                } else {
                                    format!("{}|", self.server_url_draft)
                                };
                                div()
                                    .w_full()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .flex_1()
                                            .h(px(30.0))
                                            .px(px(10.0))
                                            .rounded(px(6.0))
                                            .bg(rgb(pal.input_bg))
                                            .border_1()
                                            .border_color(rgb(pal.accent))
                                            .flex()
                                            .items_center()
                                            .text_size(px(11.5))
                                            .text_color(if self.server_url_draft.is_empty() {
                                                rgb(pal.text_secondary)
                                            } else {
                                                rgb(pal.text_primary)
                                            })
                                            .child(draft_display),
                                    )
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .h(px(30.0))
                                            .px(px(10.0))
                                            .rounded(px(6.0))
                                            .bg(rgb(pal.accent))
                                            .hover(move |s| s.bg(rgb(pal.accent_hover)))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(11.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(pal.active_button_fg))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.apply_custom_server_url(cx);
                                            }))
                                            .child(tr_str(lang, "保存", "Save")),
                                    )
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .h(px(30.0))
                                            .px(px(8.0))
                                            .rounded(px(6.0))
                                            .bg(rgb(pal.input_bg))
                                            .border_1()
                                            .border_color(rgb(pal.border_fg))
                                            .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(11.0))
                                            .text_color(rgb(pal.text_secondary))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.editing_server_url = false;
                                                this.server_url_draft = this.client.config.server_url.clone();
                                                cx.notify();
                                            }))
                                            .child(tr_str(lang, "取消", "Cancel")),
                                    )
                                    .into_any_element()
                            } else {
                                div()
                                    .cursor_pointer()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .px(px(10.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.input_bg))
                                    .border_1()
                                    .border_color(if has_net_err { rgb(pal.danger) } else { rgb(pal.border_fg) })
                                    .hover(move |s| s.border_color(rgb(pal.accent)))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                        this.server_url_draft = this.client.config.server_url.clone();
                                        this.editing_server_url = true;
                                        cx.notify();
                                    }))
                                    .child(div().w(px(6.0)).h(px(6.0)).rounded_full().bg(dot_color))
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(rgb(pal.text_secondary))
                                            .child(format!("Cloud VM: {}", server_url)),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(10.5))
                                            .text_color(rgb(pal.accent))
                                            .child(tr_str(lang, "修改", "Edit")),
                                    )
                                    .into_any_element()
                            }
                        })
                        // Multi-Tab Switcher Bar (Milestone 2 F8)
                        .child(
                            div()
                                .w_full()
                                .flex()
                                .items_center()
                                .bg(rgb(pal.input_bg))
                                .border_1()
                                .border_color(rgb(pal.border_fg))
                                .rounded(px(8.0))
                                .p(px(3.0))
                                .gap(px(4.0))
                                .child({
                                    let is_pw = self.auth_wizard_tab == AuthWizardTab::PasswordLogin;
                                    div()
                                        .cursor_pointer()
                                        .flex_1()
                                        .h(px(32.0))
                                        .rounded(px(6.0))
                                        .bg(if is_pw { rgb(pal.accent) } else { rgb(pal.input_bg) })
                                        .hover(move |s| if !is_pw { s.bg(rgb(pal.hover_bg)) } else { s })
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .gap(px(5.0))
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if is_pw { rgb(pal.active_button_fg) } else { rgb(pal.text_primary) })
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                            this.switch_auth_tab(AuthWizardTab::PasswordLogin, cx);
                                        }))
                                        .child(tg_icon(IconName::Lock, 12.0, if is_pw { pal.active_button_fg } else { pal.icon_fg }))
                                        .child(tr_str(lang, "密码登录", "Password"))
                                })
                                .child({
                                    let is_alipay = self.auth_wizard_tab == AuthWizardTab::AlipayQr;
                                    div()
                                        .cursor_pointer()
                                        .flex_1()
                                        .h(px(32.0))
                                        .rounded(px(6.0))
                                        .bg(if is_alipay { rgb(0x1677FF) } else { rgb(pal.input_bg) })
                                        .hover(move |s| if !is_alipay { s.bg(rgb(pal.hover_bg)) } else { s })
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .gap(px(5.0))
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if is_alipay { rgb(pal.active_button_fg) } else { rgb(pal.text_primary) })
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                            this.switch_auth_tab(AuthWizardTab::AlipayQr, cx);
                                        }))
                                        .child(tg_icon(IconName::WalletStars, 12.0, if is_alipay { pal.active_button_fg } else { pal.icon_fg }))
                                        .child(tr_str(lang, "支付宝扫码", "Alipay QR"))
                                })
                                .child({
                                    let is_email = self.auth_wizard_tab == AuthWizardTab::EmailRegister;
                                    div()
                                        .cursor_pointer()
                                        .flex_1()
                                        .h(px(32.0))
                                        .rounded(px(6.0))
                                        .bg(if is_email { rgb(pal.accent) } else { rgb(pal.input_bg) })
                                        .hover(move |s| if !is_email { s.bg(rgb(pal.hover_bg)) } else { s })
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .gap(px(5.0))
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if is_email { rgb(pal.active_button_fg) } else { rgb(pal.text_primary) })
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                            this.switch_auth_tab(AuthWizardTab::EmailRegister, cx);
                                        }))
                                        .child(tg_icon(IconName::SendPlane, 12.0, if is_email { pal.active_button_fg } else { pal.icon_fg }))
                                        .child(tr_str(lang, "邮箱注册", "Email Register"))
                                }),
                        )
                        // Error Banner if present
                        .when_some(self.login_error.clone(), |el, err| {
                            el.child(
                                div()
                                    .w_full()
                                    .p(px(8.0))
                                    .rounded(px(6.0))
                                    .bg(TdesktopThemePalette::rgba_with_alpha(pal.danger, 0.14))
                                    .border_1()
                                    .border_color(rgb(pal.danger))
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .text_size(px(12.0))
                                    .text_color(rgb(pal.danger))
                                    .child(tg_icon(IconName::Close, 12.0, pal.danger))
                                    .child(err),
                            )
                        })
                        // Tab Content Body
                        .child(match self.auth_wizard_tab {
                            AuthWizardTab::PasswordLogin => {
                                let is_reg = self.is_register_mode;
                                let is_loading = self.is_authenticating;
                                let u_active = self.active_login_field == LoginInputField::Username;
                                let p_active = self.active_login_field == LoginInputField::Password;
                                let fn_active = self.active_login_field == LoginInputField::FirstName;
                                let user_val = self.username_input.clone();
                                let pass_val = self.password_input.clone();
                                let fn_val = self.first_name_input.clone();

                                div()
                                    .w_full()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap(px(12.0))
                                    .child(
                                        div()
                                            .text_size(px(18.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(pal.text_primary))
                                            .child(if is_reg {
                                                tr_str(lang, "注册 Telegram 云端账号", "Create Cloud Account")
                                            } else {
                                                tr_str(lang, "登录 Telegram 云端服务", "Sign In to Cloud Server")
                                            }),
                                    )
                                    // Register: First Name Input
                                    .when(is_reg, |el| {
                                        let fn_preedit = if fn_active { self.ime_preedit.clone() } else { None };
                                        let fn_empty = fn_val.is_empty() && fn_preedit.is_none();
                                        el.child(
                                            div()
                                                .cursor_pointer()
                                                .w_full()
                                                .h(px(40.0))
                                                .rounded(px(8.0))
                                                .bg(rgb(pal.input_bg))
                                                .border_1()
                                                .border_color(if fn_active { rgb(pal.accent) } else { rgb(pal.border_fg) })
                                                .px(px(12.0))
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                                                    this.ime_preedit = None;
                                                    this.active_login_field = LoginInputField::FirstName;
                                                    this.focus_handle.focus(window);
                                                    cx.notify();
                                                }))
                                                .child(
                                                    if fn_empty {
                                                        div()
                                                            .text_size(px(13.0))
                                                            .text_color(rgb(pal.text_secondary))
                                                            .child(if fn_active {
                                                                tr_str(lang, "| 请输入显示昵称", "| Your Display Name")
                                                            } else {
                                                                tr_str(lang, "显示昵称", "Your Display Name")
                                                            })
                                                    } else {
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .gap(px(2.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(pal.text_primary))
                                                                    .child(fn_val.clone()),
                                                            )
                                                            .when_some(fn_preedit, |p_el, p| {
                                                                p_el.child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .text_color(rgb(pal.accent))
                                                                        .border_b_1()
                                                                        .border_color(rgb(pal.accent))
                                                                        .child(p),
                                                                )
                                                            })
                                                            .when(fn_active, |c_el| {
                                                                c_el.child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .text_color(rgb(pal.accent))
                                                                        .child("|"),
                                                                )
                                                            })
                                                    },
                                                )
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(8.0))
                                                        .when(!fn_val.is_empty(), |c_el| {
                                                            c_el.child(
                                                                div()
                                                                    .cursor_pointer()
                                                                    .p(px(2.0))
                                                                    .rounded(px(4.0))
                                                                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                                                                        this.first_name_input.clear();
                                                                        this.ime_preedit = None;
                                                                        this.active_login_field = LoginInputField::FirstName;
                                                                        this.focus_handle.focus(window);
                                                                        cx.stop_propagation();
                                                                        cx.notify();
                                                                    }))
                                                                    .child(tg_icon(IconName::Close, 11.0, pal.icon_fg)),
                                                            )
                                                        })
                                                        .child(tg_icon(IconName::Verified, 13.0, pal.text_secondary)),
                                                ),
                                        )
                                    })
                                    // Username Input Box
                                    .child({
                                        let u_preedit = if u_active { self.ime_preedit.clone() } else { None };
                                        let u_empty = user_val.is_empty() && u_preedit.is_none();
                                        div()
                                            .cursor_pointer()
                                            .w_full()
                                            .h(px(40.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(pal.input_bg))
                                            .border_1()
                                            .border_color(if u_active { rgb(pal.accent) } else { rgb(pal.border_fg) })
                                            .px(px(12.0))
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                                                this.ime_preedit = None;
                                                this.active_login_field = LoginInputField::Username;
                                                this.focus_handle.focus(window);
                                                cx.notify();
                                            }))
                                            .child(
                                                if u_empty {
                                                    div()
                                                        .text_size(px(13.0))
                                                        .text_color(rgb(pal.text_secondary))
                                                        .child(if u_active {
                                                            tr_str(lang, "| 请输入用户名", "| Username")
                                                        } else {
                                                            tr_str(lang, "用户名", "Username")
                                                        })
                                                } else {
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(2.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .text_color(rgb(pal.text_primary))
                                                                .child(user_val.clone()),
                                                        )
                                                        .when_some(u_preedit, |p_el, p| {
                                                            p_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(pal.accent))
                                                                    .border_b_1()
                                                                    .border_color(rgb(pal.accent))
                                                                    .child(p),
                                                                )
                                                        })
                                                        .when(u_active, |c_el| {
                                                            c_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(pal.accent))
                                                                    .child("|"),
                                                            )
                                                        })
                                                },
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(8.0))
                                                    .when(!user_val.is_empty(), |c_el| {
                                                        c_el.child(
                                                            div()
                                                                .cursor_pointer()
                                                                .p(px(2.0))
                                                                .rounded(px(4.0))
                                                                .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                                                                    this.username_input.clear();
                                                                    this.ime_preedit = None;
                                                                    this.active_login_field = LoginInputField::Username;
                                                                    this.focus_handle.focus(window);
                                                                    cx.stop_propagation();
                                                                    cx.notify();
                                                                }))
                                                                .child(tg_icon(IconName::Close, 11.0, pal.icon_fg)),
                                                        )
                                                    })
                                                    .child(tg_icon(IconName::ContactsUsers, 13.0, pal.text_secondary)),
                                            )
                                    })
                                    // Password Input Box
                                    .child({
                                        let p_preedit = if p_active { self.ime_preedit.clone() } else { None };
                                        let p_empty = pass_val.is_empty() && p_preedit.is_none();
                                        div()
                                            .cursor_pointer()
                                            .w_full()
                                            .h(px(40.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(pal.input_bg))
                                            .border_1()
                                            .border_color(if p_active { rgb(pal.accent) } else { rgb(pal.border_fg) })
                                            .px(px(12.0))
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                                                this.ime_preedit = None;
                                                this.active_login_field = LoginInputField::Password;
                                                this.focus_handle.focus(window);
                                                cx.notify();
                                            }))
                                            .child(
                                                if p_empty {
                                                    div()
                                                        .text_size(px(13.0))
                                                        .text_color(rgb(pal.text_secondary))
                                                        .child(if p_active {
                                                            tr_str(lang, "| 请输入密码", "| Password")
                                                        } else {
                                                            tr_str(lang, "密码", "Password")
                                                        })
                                                } else {
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(2.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .text_color(rgb(pal.text_primary))
                                                                .child("•".repeat(pass_val.chars().count())),
                                                        )
                                                        .when_some(p_preedit, |p_el, p| {
                                                            p_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(pal.accent))
                                                                    .border_b_1()
                                                                    .border_color(rgb(pal.accent))
                                                                    .child(p),
                                                                )
                                                        })
                                                        .when(p_active, |c_el| {
                                                            c_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(pal.accent))
                                                                    .child("|"),
                                                            )
                                                        })
                                                },
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(8.0))
                                                    .when(!pass_val.is_empty(), |c_el| {
                                                        c_el.child(
                                                            div()
                                                                .cursor_pointer()
                                                                .p(px(2.0))
                                                                .rounded(px(4.0))
                                                                .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                                                                    this.password_input.clear();
                                                                    this.ime_preedit = None;
                                                                    this.active_login_field = LoginInputField::Password;
                                                                    this.focus_handle.focus(window);
                                                                    cx.stop_propagation();
                                                                    cx.notify();
                                                                }))
                                                                .child(tg_icon(IconName::Close, 11.0, pal.icon_fg)),
                                                        )
                                                    })
                                                    .child(tg_icon(IconName::Lock, 13.0, pal.text_secondary)),
                                            )
                                    })
                                    // Submit Button
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .w_full()
                                            .h(px(42.0))
                                            .rounded(px(8.0))
                                            .bg(if is_loading { rgb(pal.accent_hover) } else { rgb(pal.accent) })
                                            .hover(move |s| s.bg(rgb(pal.accent_hover)))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(pal.active_button_fg))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                if this.is_register_mode {
                                                    this.submit_register(cx);
                                                } else {
                                                    this.submit_login(cx);
                                                }
                                            }))
                                            .child(if is_loading {
                                                tr_str(lang, "正在连接云服务器登录...", "Authenticating with Cloud Server...")
                                            } else if is_reg {
                                                tr_str(lang, "注册并登录", "Create Account & Sign In")
                                            } else {
                                                tr_str(lang, "登录到云服务器", "Sign In")
                                            }),
                                    )
                                    // Toggle Mode: Sign In vs Register
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .text_size(px(12.0))
                                            .text_color(rgb(pal.accent))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.is_register_mode = !this.is_register_mode;
                                                this.login_error = None;
                                                cx.notify();
                                            }))
                                            .child(if is_reg {
                                                tr_str(lang, "← 已有账号？直接登录", "← Already have an account? Sign In")
                                            } else {
                                                tr_str(lang, "没有账号？点击注册", "Don't have an account? Register")
                                            }),
                                    )
                                    .into_any_element()
                            }

                            AuthWizardTab::AlipayQr => {
                                let status_text = if self.alipay_status_text.is_empty() {
                                    if zh {
                                        "请使用手机支付宝 App 扫码登录".to_string()
                                    } else {
                                        "Please scan with Alipay mobile app".to_string()
                                    }
                                } else {
                                    self.alipay_status_text.clone()
                                };
                                let is_loading = self.alipay_is_loading;
                                let qr_card = Self::render_real_alipay_qr_matrix(
                                    self.alipay_qr_payload.as_deref(),
                                    is_loading,
                                    zh,
                                );

                                div()
                                    .w_full()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap(px(12.0))
                                    .child(
                                        div()
                                            .text_size(px(18.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(pal.text_primary))
                                            .child(tr_str(lang, "使用支付宝扫码登录", "Log in with Alipay QR")),
                                    )
                                    .child(qr_card)
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(if is_loading { rgb(pal.accent) } else { rgb(pal.text_secondary) })
                                            .child(status_text),
                                    )
                                    // Open Real Alipay Web OAuth in Default Browser
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .w_full()
                                            .h(px(40.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(0x1677FF))
                                            .hover(|s| s.bg(rgb(0x0E5CD6)))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .gap(px(6.0))
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.open_alipay_web_auth(cx);
                                            }))
                                            .child(tg_icon(IconName::SendPlane, 13.0, 0xFFFFFF))
                                            .child(tr_str(lang, "在浏览器打开支付宝授权", "Open Alipay Authorization in Browser")),
                                    )
                                    // Refresh QR Code Button
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .w_full()
                                            .h(px(36.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(pal.input_bg))
                                            .border_1()
                                            .border_color(rgb(pal.border_fg))
                                            .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .gap(px(6.0))
                                            .text_size(px(12.0))
                                            .text_color(rgb(pal.text_primary))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.start_alipay_flow(cx);
                                            }))
                                            .child(tg_icon(IconName::ClockSending, 13.0, pal.icon_fg))
                                            .child(tr_str(lang, "刷新二维码", "Refresh QR Code")),
                                    )
                                    .into_any_element()
                            }

                            AuthWizardTab::EmailRegister => {
                                let is_loading = self.is_authenticating;
                                let e_active = self.active_login_field == LoginInputField::Email;
                                let ec_active = self.active_login_field == LoginInputField::EmailCode;
                                let u_active = self.active_login_field == LoginInputField::Username;
                                let p_active = self.active_login_field == LoginInputField::Password;
                                let fn_active = self.active_login_field == LoginInputField::FirstName;
                                let e_val = self.email_input.clone();
                                let ec_val = self.email_code_input.clone();
                                let u_val = self.username_input.clone();
                                let p_val = self.password_input.clone();
                                let fn_val = self.first_name_input.clone();
                                let cooldown = self.email_code_cooldown;
                                let is_sending = self.is_sending_email_code;

                                div()
                                    .w_full()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap(px(10.0))
                                    .child(
                                        div()
                                            .text_size(px(18.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(pal.text_primary))
                                            .child(tr_str(lang, "邮箱注册账号", "Register with Email")),
                                    )
                                    // Row: Email Input + Send Code Button
                                    .child(
                                        div()
                                            .w_full()
                                            .flex()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child({
                                                let e_preedit = if e_active { self.ime_preedit.clone() } else { None };
                                                let e_empty = e_val.is_empty() && e_preedit.is_none();
                                                div()
                                                    .cursor_pointer()
                                                    .flex_1()
                                                    .h(px(40.0))
                                                    .rounded(px(8.0))
                                                    .bg(rgb(pal.input_bg))
                                                    .border_1()
                                                    .border_color(if e_active { rgb(pal.accent) } else { rgb(pal.border_fg) })
                                                    .px(px(12.0))
                                                    .flex()
                                                    .items_center()
                                                    .justify_between()
                                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                                                        this.ime_preedit = None;
                                                        this.active_login_field = LoginInputField::Email;
                                                        this.focus_handle.focus(window);
                                                        cx.notify();
                                                    }))
                                                    .child(
                                                        if e_empty {
                                                            div()
                                                                .text_size(px(13.0))
                                                                .text_color(rgb(pal.text_secondary))
                                                                .child(if e_active {
                                                                    tr_str(lang, "| 请输入邮箱地址", "| Email")
                                                                } else {
                                                                    tr_str(lang, "邮箱地址", "Email")
                                                                })
                                                        } else {
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap(px(2.0))
                                                                .child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .text_color(rgb(pal.text_primary))
                                                                        .child(e_val.clone()),
                                                                )
                                                                .when_some(e_preedit, |p_el, p| {
                                                                    p_el.child(
                                                                        div()
                                                                            .text_size(px(13.0))
                                                                            .text_color(rgb(pal.accent))
                                                                            .border_b_1()
                                                                            .border_color(rgb(pal.accent))
                                                                            .child(p),
                                                                    )
                                                                })
                                                                .when(e_active, |c_el| {
                                                                    c_el.child(
                                                                        div()
                                                                            .text_size(px(13.0))
                                                                            .text_color(rgb(pal.accent))
                                                                            .child("|"),
                                                                    )
                                                                })
                                                        },
                                                    )
                                            })
                                            .child(
                                                div()
                                                    .cursor_pointer()
                                                    .px(px(12.0))
                                                    .h(px(40.0))
                                                    .rounded(px(8.0))
                                                    .bg(if cooldown > 0 || is_sending { rgb(pal.input_bg) } else { rgb(pal.accent) })
                                                    .border_1()
                                                    .border_color(if cooldown > 0 || is_sending { rgb(pal.border_fg) } else { rgb(pal.accent) })
                                                    .hover(move |s| if cooldown == 0 && !is_sending { s.bg(rgb(pal.accent_hover)) } else { s })
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_size(px(12.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(if cooldown > 0 || is_sending { rgb(pal.text_secondary) } else { rgb(pal.active_button_fg) })
                                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                        this.send_email_code(cx);
                                                    }))
                                                    .child(if is_sending {
                                                        tr_str(lang, "发送中...", "Sending...").to_string()
                                                    } else if cooldown > 0 {
                                                        if zh {
                                                            format!("重新获取 ({}s)", cooldown)
                                                        } else {
                                                            format!("Resend ({}s)", cooldown)
                                                        }
                                                    } else {
                                                        tr_str(lang, "获取验证码", "Send Code").to_string()
                                                    }),
                                            ),
                                    )
                                    // Code Input Box
                                    .child({
                                        let ec_preedit = if ec_active { self.ime_preedit.clone() } else { None };
                                        let ec_empty = ec_val.is_empty() && ec_preedit.is_none();
                                        div()
                                            .cursor_pointer()
                                            .w_full()
                                            .h(px(40.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(pal.input_bg))
                                            .border_1()
                                            .border_color(if ec_active { rgb(pal.accent) } else { rgb(pal.border_fg) })
                                            .px(px(12.0))
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                                                this.ime_preedit = None;
                                                this.active_login_field = LoginInputField::EmailCode;
                                                this.focus_handle.focus(window);
                                                cx.notify();
                                            }))
                                            .child(
                                                if ec_empty {
                                                    div()
                                                        .text_size(px(13.0))
                                                        .text_color(rgb(pal.text_secondary))
                                                        .child(if ec_active {
                                                            tr_str(lang, "| 请输入6位验证码", "| 6-digit Code")
                                                        } else {
                                                            tr_str(lang, "验证码", "Verification Code")
                                                        })
                                                } else {
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(2.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .text_color(rgb(pal.text_primary))
                                                                .child(ec_val.clone()),
                                                        )
                                                        .when(ec_active, |c_el| {
                                                            c_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(pal.accent))
                                                                    .child("|"),
                                                            )
                                                        })
                                                },
                                            )
                                    })
                                    // Username Input Box
                                    .child({
                                        let u_preedit = if u_active { self.ime_preedit.clone() } else { None };
                                        let u_empty = u_val.is_empty() && u_preedit.is_none();
                                        div()
                                            .cursor_pointer()
                                            .w_full()
                                            .h(px(40.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(pal.input_bg))
                                            .border_1()
                                            .border_color(if u_active { rgb(pal.accent) } else { rgb(pal.border_fg) })
                                            .px(px(12.0))
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                                                this.ime_preedit = None;
                                                this.active_login_field = LoginInputField::Username;
                                                this.focus_handle.focus(window);
                                                cx.notify();
                                            }))
                                            .child(
                                                if u_empty {
                                                    div()
                                                        .text_size(px(13.0))
                                                        .text_color(rgb(pal.text_secondary))
                                                        .child(if u_active {
                                                            tr_str(lang, "| 请输入用户名", "| Username")
                                                        } else {
                                                            tr_str(lang, "用户名", "Username")
                                                        })
                                                } else {
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(2.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .text_color(rgb(pal.text_primary))
                                                                .child(u_val.clone()),
                                                        )
                                                        .when(u_active, |c_el| {
                                                            c_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(pal.accent))
                                                                    .child("|"),
                                                            )
                                                        })
                                                },
                                            )
                                    })
                                    // Password Input Box
                                    .child({
                                        let p_preedit = if p_active { self.ime_preedit.clone() } else { None };
                                        let p_empty = p_val.is_empty() && p_preedit.is_none();
                                        div()
                                            .cursor_pointer()
                                            .w_full()
                                            .h(px(40.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(pal.input_bg))
                                            .border_1()
                                            .border_color(if p_active { rgb(pal.accent) } else { rgb(pal.border_fg) })
                                            .px(px(12.0))
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                                                this.ime_preedit = None;
                                                this.active_login_field = LoginInputField::Password;
                                                this.focus_handle.focus(window);
                                                cx.notify();
                                            }))
                                            .child(
                                                if p_empty {
                                                    div()
                                                        .text_size(px(13.0))
                                                        .text_color(rgb(pal.text_secondary))
                                                        .child(if p_active {
                                                            tr_str(lang, "| 请输入密码", "| Password")
                                                        } else {
                                                            tr_str(lang, "密码", "Password")
                                                        })
                                                } else {
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(2.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .text_color(rgb(pal.text_primary))
                                                                .child("•".repeat(p_val.chars().count())),
                                                        )
                                                        .when(p_active, |c_el| {
                                                            c_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(pal.accent))
                                                                    .child("|"),
                                                            )
                                                        })
                                                },
                                            )
                                    })
                                    // First Name Input Box
                                    .child({
                                        let fn_preedit = if fn_active { self.ime_preedit.clone() } else { None };
                                        let fn_empty = fn_val.is_empty() && fn_preedit.is_none();
                                        div()
                                            .cursor_pointer()
                                            .w_full()
                                            .h(px(40.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(pal.input_bg))
                                            .border_1()
                                            .border_color(if fn_active { rgb(pal.accent) } else { rgb(pal.border_fg) })
                                            .px(px(12.0))
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                                                this.ime_preedit = None;
                                                this.active_login_field = LoginInputField::FirstName;
                                                this.focus_handle.focus(window);
                                                cx.notify();
                                            }))
                                            .child(
                                                if fn_empty {
                                                    div()
                                                        .text_size(px(13.0))
                                                        .text_color(rgb(pal.text_secondary))
                                                        .child(if fn_active {
                                                            tr_str(lang, "| 请输入显示昵称（可选）", "| Display Name (optional)")
                                                        } else {
                                                            tr_str(lang, "显示昵称（可选）", "Display Name (optional)")
                                                        })
                                                } else {
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(2.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .text_color(rgb(pal.text_primary))
                                                                .child(fn_val.clone()),
                                                        )
                                                        .when(fn_active, |c_el| {
                                                            c_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(pal.accent))
                                                                    .child("|"),
                                                            )
                                                        })
                                                },
                                            )
                                    })
                                    // Submit Button
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .w_full()
                                            .h(px(42.0))
                                            .rounded(px(8.0))
                                            .bg(if is_loading { rgb(pal.accent_hover) } else { rgb(pal.accent) })
                                            .hover(move |s| s.bg(rgb(pal.accent_hover)))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(pal.active_button_fg))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.submit_email_register(cx);
                                            }))
                                            .child(if is_loading {
                                                tr_str(lang, "正在注册并登录...", "Authenticating...")
                                            } else {
                                                tr_str(lang, "立即注册并登录", "Register & Sign In")
                                            }),
                                    )
                                    .into_any_element()
                            }
                        })
                        .into_any_element()
                }

                LoginWizardStep::Phone => div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child(tr_str(lang, "输入您的手机号码", "Your Phone Number")),
                    )
                    // Country Code Selector Pills
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .children(["+86", "+1", "+44", "+81"].into_iter().map(|cc| {
                                let is_sel = country == cc;
                                let cc_str = cc.to_string();
                                div()
                                    .cursor_pointer()
                                    .px(px(10.0))
                                    .py(px(5.0))
                                    .rounded(px(6.0))
                                    .bg(if is_sel { rgb(pal.accent) } else { rgb(pal.input_bg) })
                                    .border_1()
                                    .border_color(if is_sel { rgb(pal.accent) } else { rgb(pal.border_fg) })
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(if is_sel { rgb(pal.active_button_fg) } else { rgb(pal.text_primary) })
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                        this.country_code = cc_str.clone();
                                        cx.notify();
                                    }))
                                    .child(cc)
                            })),
                    )
                    // Phone Input Box
                    .child(
                        div()
                            .w_full()
                            .h(px(42.0))
                            .rounded(px(8.0))
                            .bg(rgb(pal.input_bg))
                            .border_1()
                            .border_color(rgb(pal.accent))
                            .px(px(12.0))
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(format!("{} {}", country, if phone.is_empty() { "___ ____ ____" } else { &phone })),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .p(px(2.0))
                                    .rounded(px(4.0))
                                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.phone_number.clear();
                                        cx.notify();
                                    }))
                                    .child(tg_icon(IconName::Close, 11.0, pal.icon_fg)),
                            ),
                    )
                    // On-screen clickable numeric keypad (so mouse-only users can also dial)
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_wrap()
                            .gap(px(6.0))
                            .children(["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "⌫"].into_iter().map(|digit| {
                                let d_str = digit.to_string();
                                div()
                                    .cursor_pointer()
                                    .w(px(58.0))
                                    .h(px(32.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.input_bg))
                                    .border_1()
                                    .border_color(rgb(pal.border_fg))
                                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                        if d_str == "⌫" {
                                            this.phone_number.pop();
                                        } else {
                                            this.phone_number.push_str(&d_str);
                                        }
                                        cx.notify();
                                    }))
                                    .child(digit)
                            })),
                    )
                    // Next & Back buttons
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .cursor_pointer()
                                    .px(px(14.0))
                                    .h(px(42.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.input_bg))
                                    .border_1()
                                    .border_color(rgb(pal.border_fg))
                                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .gap(px(5.0))
                                    .text_size(px(13.0))
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.login_step = LoginWizardStep::Start;
                                        cx.notify();
                                    }))
                                    .child(tg_icon(IconName::BackArrow, 13.0, pal.icon_fg))
                                    .child(tr_str(lang, "返回", "Back")),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .flex_1()
                                    .h(px(42.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(pal.accent))
                                    .hover(move |s| s.bg(rgb(pal.accent_hover)))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.active_button_fg))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.login_step = LoginWizardStep::Code;
                                        cx.notify();
                                    }))
                                    .child(tr_str(lang, "下一步 (获取验证码) →", "NEXT →")),
                            ),
                    )
                    .into_any_element(),

                LoginWizardStep::QrCode => div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child(tr_str(lang, "使用 Telegram 扫码登录", "Scan QR Code with Telegram")),
                    )
                    .child(Self::render_real_alipay_qr_matrix(
                        self.alipay_qr_payload.as_deref(),
                        self.alipay_is_loading,
                        zh,
                    ))
                    .child(
                        div()
                            .cursor_pointer()
                            .w_full()
                            .h(px(42.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x1677FF))
                            .hover(|s| s.bg(rgb(0x0E5CD6)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(6.0))
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.login_step = LoginWizardStep::Start;
                                this.switch_auth_tab(AuthWizardTab::AlipayQr, cx);
                            }))
                            .child(tg_icon(IconName::WalletStars, 13.0, 0xFFFFFF))
                            .child(tr_str(lang, "切换至支付宝真实扫码登录", "Switch to Alipay QR Login")),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(13.0))
                            .text_color(rgb(pal.accent))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.login_step = LoginWizardStep::Phone;
                                cx.notify();
                            }))
                            .child(tr_str(lang, "改用手机号登录", "Log in by phone number")),
                    )
                    .into_any_element(),

                LoginWizardStep::Code => div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child(if zh {
                                format!("验证码已发送至 {} {}", country, phone)
                            } else {
                                format!("Code sent to {} {}", country, phone)
                            }),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .w_full()
                            .h(px(44.0))
                            .rounded(px(8.0))
                            .bg(rgb(pal.input_bg))
                            .border_1()
                            .border_color(rgb(pal.accent))
                            .px(px(14.0))
                            .flex()
                            .items_center()
                            .justify_between()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                if !this.verification_code.trim().is_empty() {
                                    this.submit_login(cx);
                                } else {
                                    cx.notify();
                                }
                            }))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(if code.is_empty() { "• • • • •".to_string() } else { code.clone() }),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .p(px(2.0))
                                    .rounded(px(4.0))
                                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.verification_code.clear();
                                        cx.notify();
                                    }))
                                    .child(tg_icon(IconName::Close, 11.0, pal.icon_fg)),
                            ),
                    )
                    // On-screen clickable numeric keypad for verification code
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_wrap()
                            .gap(px(6.0))
                            .children(["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "⌫"].into_iter().map(|digit| {
                                let d_str = digit.to_string();
                                div()
                                    .cursor_pointer()
                                    .w(px(58.0))
                                    .h(px(32.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.input_bg))
                                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                        if d_str == "⌫" {
                                            this.verification_code.pop();
                                        } else if this.verification_code.len() < 6 {
                                            this.verification_code.push_str(&d_str);
                                            if this.verification_code.len() >= 5 {
                                                this.submit_login(cx);
                                            }
                                        }
                                        cx.notify();
                                    }))
                                    .child(digit)
                            })),
                    )
                    // Submit Verification Code Button
                    .child(
                        div()
                            .cursor_pointer()
                            .w_full()
                            .h(px(44.0))
                            .rounded(px(8.0))
                            .bg(rgb(pal.accent))
                            .hover(move |s| s.bg(rgb(pal.accent_hover)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.active_button_fg))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                if this.verification_code.trim().is_empty() {
                                    this.login_error = Some(
                                        tr_str(
                                            this.active_lang_id(),
                                            "请输入您收到的验证码",
                                            "Please enter the verification code you received",
                                        )
                                        .to_string(),
                                    );
                                    cx.notify();
                                } else {
                                    this.submit_login(cx);
                                }
                            }))
                            .child(tr_str(lang, "验证并登录", "Verify & Sign In")),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(12.0))
                            .text_color(rgb(pal.text_secondary))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.login_step = LoginWizardStep::Phone;
                                cx.notify();
                            }))
                            .child(tr_str(lang, "← 修改手机号", "← Change Phone Number")),
                    )
                    .into_any_element(),
            })
    }

    fn render_calls_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let lang = self.active_lang_id();
        let (title_block, back_btn) = render_panel_header(
            IconName::PhoneCall,
            tr_str(lang, "通话记录", "Recent Calls"),
            tr_str(lang, "返回对话", "Back to Chats"),
            &pal,
        );
        let my_uid = self.user_id.unwrap_or(0);
        let calls = self.panel_state.calls.clone();
        let contacts = self.contacts_list.clone();

        div()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .overflow_hidden()
            .bg(rgb(pal.window_bg))
            .p(px(24.0))
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(title_block)
                    .child(back_btn.on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                        this.active_panel = ActiveWorkspacePanel::Chats;
                        this.last_handled_nav_dest = Some(NavDestination::Chats);
                        this.nav_rail.update(cx, |r, _| {
                            r.set_active_destination(NavDestination::Chats);
                        });
                        cx.notify();
                    }))),
            )
            .when(self.panel_state.calls_loading, |el| {
                el.child(render_empty_panel_notice(
                    IconName::PhoneCall,
                    tr_str(lang, "正在加载通话记录...", "Loading call history..."),
                    "",
                    None,
                    &pal,
                ))
            })
            .when(!self.panel_state.calls_loading && calls.is_empty(), |el| {
                el.child(render_empty_panel_notice(
                    IconName::PhoneCall,
                    tr_str(lang, "暂无通话记录", "No recent calls"),
                    tr_str(
                        lang,
                        "您发起或接收的通话将显示在这里。",
                        "Calls you make or receive will appear here.",
                    ),
                    self.panel_state.calls_error.as_deref(),
                    &pal,
                ))
            })
            .when(!calls.is_empty(), |el| {
                el.child(
                    div()
                        .id("calls-history-list")
                        .flex_1()
                        .w_full()
                        .overflow_y_scroll()
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .children(calls.into_iter().map(|call| {
                            let peer_id = if call.caller_id == my_uid {
                                call.callee_id
                            } else {
                                call.caller_id
                            };
                            let peer_name = contacts
                                .iter()
                                .find(|ct| ct.user_id == peer_id)
                                .map(|ct| {
                                    format!(
                                        "{} {}",
                                        ct.first_name,
                                        ct.last_name.as_deref().unwrap_or("")
                                    )
                                    .trim()
                                    .to_string()
                                })
                                .filter(|s| !s.is_empty())
                                .unwrap_or_else(|| format!("User {}", peer_id));
                            let ts = format_authoritative_timestamp(Some(call.started_at_ms));
                            let state_label = crate::views::app_shell::shell_actions::localize_call_state(
                                &call.state,
                                lang,
                            );
                            let sub = if call.duration_secs > 0 {
                                format!("{state_label} · {}s · {ts}", call.duration_secs)
                            } else if !ts.is_empty() {
                                format!("{state_label} · {ts}")
                            } else {
                                state_label
                            };
                            let is_missed = call.state.eq_ignore_ascii_case("missed")
                                || call.state.eq_ignore_ascii_case("declined");
                            let icon = if call.is_video {
                                IconName::VideoCall
                            } else {
                                IconName::PhoneCall
                            };
                            let redial_name = peer_name.clone();
                            let is_vid = call.is_video;

                            div()
                                .cursor_pointer()
                                .w_full()
                                .px(px(16.0))
                                .py(px(12.0))
                                .rounded(px(10.0))
                                .bg(rgb(pal.card_bg))
                                .border_1()
                                .border_color(rgb(pal.border_fg))
                                .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                .flex()
                                .items_center()
                                .justify_between()
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                    this.start_webrtc_call(peer_id, redial_name.clone(), is_vid, cx);
                                }))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(12.0))
                                        .child(tg_icon(
                                            icon,
                                            18.0,
                                            if is_missed { pal.danger } else { pal.accent },
                                        ))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(2.0))
                                                .child(
                                                    div()
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(rgb(pal.text_primary))
                                                        .child(peer_name),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(12.0))
                                                        .text_color(if is_missed {
                                                            rgb(pal.danger)
                                                        } else {
                                                            rgb(pal.text_secondary)
                                                        })
                                                        .child(sub),
                                                ),
                                        ),
                                )
                                .child(tg_icon(IconName::PhoneCall, 16.0, pal.accent))
                        })),
                )
            })
    }

    fn render_task_center_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let lang = self.active_lang_id();
        let (title_block, back_btn) = render_panel_header(
            IconName::Poll,
            tr_str(lang, "Hermes 智能体工作台", "Hermes Agent Studio"),
            tr_str(lang, "返回对话", "Back to Chats"),
            &pal,
        );
        div()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .overflow_hidden()
            .bg(rgb(pal.window_bg))
            .p(px(24.0))
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(title_block)
                    .child(back_btn.on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                        this.active_panel = ActiveWorkspacePanel::Chats;
                        this.last_handled_nav_dest = Some(NavDestination::Chats);
                        this.nav_rail.update(cx, |r, _| {
                            r.set_active_destination(NavDestination::Chats);
                        });
                        cx.notify();
                    }))),
            )
            .child(div().flex_1().min_h(px(0.0)).w_full().child(self.tasks_workspace.clone()))
    }

    fn render_mcp_tools_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let lang = self.active_lang_id();
        let (title_block, back_btn) = render_panel_header(
            IconName::TerminalMcp,
            tr_str(lang, "Hermes 工具集与终端工作台", "Hermes Toolsets & Terminal Studio"),
            tr_str(lang, "返回对话", "Back to Chats"),
            &pal,
        );
        div()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .overflow_hidden()
            .bg(rgb(pal.window_bg))
            .p(px(24.0))
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(title_block)
                    .child(back_btn.on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                        this.active_panel = ActiveWorkspacePanel::Chats;
                        this.last_handled_nav_dest = Some(NavDestination::Chats);
                        this.nav_rail.update(cx, |r, _| {
                            r.set_active_destination(NavDestination::Chats);
                        });
                        cx.notify();
                    }))),
            )
            .child(div().flex_1().min_h(px(0.0)).w_full().child(self.tasks_workspace.clone()))
    }

    pub fn navigate_settings_to(&mut self, target: SettingsSectionType, cx: &mut Context<Self>) {
        if self.active_panel != ActiveWorkspacePanel::Settings {
            self.settings_stack.clear();
            self.settings_section = target;
            self.active_panel = ActiveWorkspacePanel::Settings;
            self.last_handled_nav_dest = Some(NavDestination::Settings);
            self.nav_rail.update(cx, |r, _| {
                r.set_active_destination(NavDestination::Settings);
            });
            cx.notify();
            return;
        }
        if self.settings_section == target {
            return;
        }
        if self.settings_stack.last().copied() != Some(self.settings_section) {
            self.settings_stack.push(self.settings_section);
        }
        self.settings_section = target;
        self.active_panel = ActiveWorkspacePanel::Settings;
        self.last_handled_nav_dest = Some(NavDestination::Settings);
        self.nav_rail.update(cx, |r, _| {
            r.set_active_destination(NavDestination::Settings);
        });
        cx.notify();
    }

    pub fn navigate_settings_back(&mut self, cx: &mut Context<Self>) {
        if let Some(prev) = self.settings_stack.pop() {
            self.settings_section = prev;
            cx.notify();
        } else if !self.settings_section.is_root() {
            let parent = self.settings_section.parent_section().unwrap_or(SettingsSectionType::Main);
            self.settings_section = parent;
            cx.notify();
        } else {
            self.close_settings(cx);
        }
    }

    pub fn close_settings(&mut self, cx: &mut Context<Self>) {
        self.active_panel = ActiveWorkspacePanel::Chats;
        self.last_handled_nav_dest = Some(NavDestination::Chats);
        self.nav_rail.update(cx, |r, _| {
            r.set_active_destination(NavDestination::Chats);
        });
        self.settings_section = SettingsSectionType::Main;
        self.settings_stack.clear();
        cx.notify();
    }

    pub fn update_preferences<F>(&mut self, cx: &mut Context<Self>, f: F)
    where
        F: FnOnce(&mut AppPreferences),
    {
        f(&mut self.preferences);
        let app_dir = ClientConfig::get_app_dir();
        if let Err(e) = self.preferences.save_to_dir(&app_dir) {
            eprintln!("[AppShell] Failed to persist preferences.json: {e}");
        }
        if let Err(e) = self.preferences.sync_to_localstorage(&mut self.local_storage) {
            eprintln!("[AppShell] Failed to sync Localstorage TDF settings: {e}");
        }
        self.is_chinese_locale = self.preferences.localization.language_code.starts_with("zh");
        self.sync_theme_and_locale_to_views(cx);
        cx.notify();
    }

    fn render_language_option(
        &self,
        lang_code: &'static str,
        badge_code: &'static str,
        title: &'static str,
        subtitle: &'static str,
        is_selected: bool,
        status_label: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let pal = self.palette;
        let badge_fg = if is_selected {
            pal.active_button_fg
        } else {
            pal.text_primary
        };
        let badge_icon_fg = if is_selected {
            pal.active_button_fg
        } else {
            pal.accent
        };
        div()
            .cursor_pointer()
            .p(px(16.0))
            .rounded(px(10.0))
            .bg(rgb(pal.card_bg))
            .border_1()
            .border_color(if is_selected {
                rgb(pal.accent)
            } else {
                rgb(pal.border_fg)
            })
            .hover(move |s| s.bg(rgb(pal.hover_bg)))
            .flex()
            .items_center()
            .justify_between()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    this.update_preferences(cx, |p| {
                        p.localization.language_code = lang_code.to_string();
                    });
                }),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .px(px(10.0))
                            .py(px(7.0))
                            .rounded(px(8.0))
                            .bg(rgb(if is_selected { pal.accent } else { pal.input_bg }))
                            .border_1()
                            .border_color(rgb(if is_selected {
                                pal.accent
                            } else {
                                pal.border_fg
                            }))
                            .flex()
                            .items_center()
                            .gap(px(5.0))
                            .child(tg_icon(IconName::Translate, 13.0, badge_icon_fg))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(badge_fg))
                                    .child(badge_code),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(title),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(pal.text_secondary))
                                    .child(subtitle),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .when(is_selected, |el| {
                        el.child(tg_icon(IconName::SelectCheck, 14.0, pal.accent))
                    })
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(if is_selected {
                                rgb(pal.accent)
                            } else {
                                rgb(pal.text_secondary)
                            })
                            .child(status_label),
                    ),
            )
    }

    fn render_language_settings(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_zh = self.is_chinese_locale;
        let pal = self.palette;
        let lang = self.active_lang_id();
        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .p(px(24.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(tg_icon(IconName::Translate, 20.0, pal.accent))
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child(tr_str(lang, "界面语言设置", "Interface Language Settings")),
                    ),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(pal.text_secondary))
                    .child(if is_zh {
                        "选择 Telegram Desktop 的默认显示语言。修改后将即时生效并持久化。"
                    } else {
                        "Choose the default display language for Telegram Desktop. Changes apply and persist immediately."
                    }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(self.render_language_option(
                        "zh-CN",
                        "ZH",
                        "简体中文 (Simplified Chinese)",
                        "zh-CN • 官方翻译完整支持",
                        is_zh,
                        if is_zh { "已启用" } else { "选择" },
                        cx,
                    ))
                    .child(self.render_language_option(
                        "en-US",
                        "EN",
                        "English (US)",
                        "en-US • Default Telegram Desktop English",
                        !is_zh,
                        if !is_zh { "Active" } else { "Select" },
                        cx,
                    )),
            )
    }

    fn render_settings_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_root = self.settings_section.is_root();
        let zh = self.is_chinese_locale;
        let pal = self.palette;
        let lang = self.active_lang_id();
        let title = self.settings_section.display_title_localized(zh);
        let section_icon = self.settings_section.icon_name();

        let back_label = if is_root {
            tr_str(lang, "返回对话", "Back to Chats").to_string()
        } else if let Some(&prev) = self.settings_stack.last() {
            if zh {
                format!("返回 {}", prev.display_title_localized(true))
            } else {
                prev.display_title_localized(false).to_string()
            }
        } else {
            tr_str(lang, "返回", "Back").to_string()
        };

        let crumbs = self.settings_section.breadcrumb_path();

        div()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .overflow_hidden()
            .bg(rgb(pal.window_bg))
            .flex()
            .flex_col()
            // 1. Settings Navigation Bar
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(20.0))
                    .py(px(12.0))
                    .bg(rgb(pal.topbar_bg))
                    .border_b_1()
                    .border_color(rgb(pal.border_fg))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.0))
                            .child(
                                // Back Button
                                div()
                                    .cursor_pointer()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .px(px(12.0))
                                    .py(px(6.0))
                                    .rounded(px(6.0))
                                    .bg(if is_root { rgb(pal.accent) } else { rgb(pal.input_bg) })
                                    .hover(move |s| s.bg(if is_root { rgb(pal.accent_hover) } else { rgb(pal.hover_bg) }))
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(if is_root { rgb(pal.active_button_fg) } else { rgb(pal.text_primary) })
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                        this.navigate_settings_back(cx);
                                    }))
                                    .child(tg_icon(
                                        IconName::BackArrow,
                                        14.0,
                                        if is_root { pal.active_button_fg } else { pal.icon_fg },
                                    ))
                                    .child(back_label),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(2.0))
                                    .when(crumbs.len() > 1, |el| {
                                        el.child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(px(4.0))
                                                .children(crumbs.into_iter().enumerate().map(|(idx, sec)| {
                                                    let is_current = sec == self.settings_section;
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(4.0))
                                                        .when(idx > 0, |sep| {
                                                            sep.child(
                                                                div()
                                                                    .text_size(px(11.0))
                                                                    .text_color(rgb(pal.text_secondary))
                                                                    .child("›"),
                                                            )
                                                        })
                                                        .child(
                                                            div()
                                                                .text_size(px(11.0))
                                                                .text_color(if is_current {
                                                                    rgb(pal.accent)
                                                                } else {
                                                                    rgb(pal.text_secondary)
                                                                })
                                                                .cursor_pointer()
                                                                .hover(move |s| s.text_color(rgb(pal.text_primary)))
                                                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                                                    this.navigate_settings_to(sec, cx);
                                                                }))
                                                                .child(sec.display_title_localized(zh)),
                                                        )
                                                })),
                                        )
                                    })
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(tg_icon(section_icon, 18.0, pal.accent))
                                            .child(
                                                div()
                                                    .text_size(px(16.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(pal.text_primary))
                                                    .child(title),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        // Header Actions Cluster
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                // Search Button
                                div()
                                    .cursor_pointer()
                                    .px(px(10.0))
                                    .py(px(6.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.input_bg))
                                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                    .flex()
                                    .items_center()
                                    .gap(px(5.0))
                                    .text_size(px(13.0))
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                        this.navigate_settings_to(SettingsSectionType::Search, cx);
                                    }))
                                    .child(tg_icon(IconName::Search, 13.0, pal.icon_fg))
                                    .child(tr_str(lang, "搜索", "Search")),
                            )
                            .child(
                                // Language Toggle
                                div()
                                    .cursor_pointer()
                                    .px(px(10.0))
                                    .py(px(6.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.input_bg))
                                    .hover(move |s| s.bg(rgb(pal.hover_bg)))
                                    .flex()
                                    .items_center()
                                    .gap(px(5.0))
                                    .text_size(px(13.0))
                                    .text_color(rgb(pal.accent))
                                    .font_weight(FontWeight::BOLD)
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                        this.update_preferences(cx, |p| {
                                            if p.localization.language_code.starts_with("zh") {
                                                p.localization.language_code = "en-US".to_string();
                                            } else {
                                                p.localization.language_code = "zh-CN".to_string();
                                            }
                                        });
                                    }))
                                    .child(tg_icon(IconName::Translate, 13.0, pal.accent))
                                    .child(tr_str(lang, "中文", "English")),
                            )
                            .child(
                                // Close Button
                                div()
                                    .cursor_pointer()
                                    .px(px(10.0))
                                    .py(px(6.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.input_bg))
                                    .hover(move |s| s.bg(rgb(pal.danger)))
                                    .flex()
                                    .items_center()
                                    .gap(px(5.0))
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                        this.close_settings(cx);
                                    }))
                                    .child(tg_icon(IconName::Close, 13.0, pal.icon_fg))
                                    .child(tr_str(lang, "关闭", "Close")),
                            ),
                    ),
            )
            // 2. Active Section Scroll View
            .child(
                div()
                    .id("settings-scroll-view")
                    .flex_1()
                    .overflow_y_scroll()
                    .child(match self.settings_section {
                        SettingsSectionType::Main => self.settings_main.clone().into_any_element(),
                        SettingsSectionType::Information => self.settings_information.clone().into_any_element(),
                        SettingsSectionType::Notifications => self.settings_notifications.clone().into_any_element(),
                        SettingsSectionType::NotificationsReactions => self.settings_notif_reactions.clone().into_any_element(),
                        SettingsSectionType::NotificationsType => self.settings_notif_type.clone().into_any_element(),
                        SettingsSectionType::PrivacySecurity => self.settings_privacy.clone().into_any_element(),
                        SettingsSectionType::BlockedPeers => self.settings_blocked_peers.clone().into_any_element(),
                        SettingsSectionType::ActiveSessions => self.settings_active_sessions.clone().into_any_element(),
                        SettingsSectionType::LocalPasscode => self.settings_local_passcode.clone().into_any_element(),
                        SettingsSectionType::Passkeys => self.settings_passkeys.clone().into_any_element(),
                        SettingsSectionType::GlobalTtl => self.settings_global_ttl.clone().into_any_element(),
                        SettingsSectionType::Websites => self.settings_websites.clone().into_any_element(),
                        SettingsSectionType::Chat => self.settings_chat.clone().into_any_element(),
                        SettingsSectionType::Folders => self.settings_folders.clone().into_any_element(),
                        SettingsSectionType::Advanced => self.settings_advanced.clone().into_any_element(),
                        SettingsSectionType::LocalStorage => self.settings_local_storage.clone().into_any_element(),
                        SettingsSectionType::PowerSaving => self.settings_power_saving.clone().into_any_element(),
                        SettingsSectionType::Calls => self.settings_calls.clone().into_any_element(),
                        SettingsSectionType::Credits => self.settings_credits.clone().into_any_element(),
                        SettingsSectionType::Premium => self.settings_premium.clone().into_any_element(),
                        SettingsSectionType::Shortcuts => self.settings_shortcuts.clone().into_any_element(),
                        SettingsSectionType::Business => self.settings_business.clone().into_any_element(),
                        SettingsSectionType::Search => self.settings_search.clone().into_any_element(),
                        SettingsSectionType::Language => self.render_language_settings(cx).into_any_element(),
                    }),
            )
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[std::prelude::v1::test]
    fn test_theme_cycle_covers_all_four_tdesktop_modes() {
        let m1 = next_tdesktop_theme_cycle(TdesktopThemeMode::Night);
        let m2 = next_tdesktop_theme_cycle(m1);
        let m3 = next_tdesktop_theme_cycle(m2);
        let m4 = next_tdesktop_theme_cycle(m3);
        assert_eq!(
            (m1, m2, m3, m4),
            (
                TdesktopThemeMode::Tinted,
                TdesktopThemeMode::Day,
                TdesktopThemeMode::Classic,
                TdesktopThemeMode::Night,
            )
        );
    }

    #[std::prelude::v1::test]
    fn test_language_toggle_and_nav_shortcut_mapping() {
        assert_eq!(toggled_language_code("zh-CN"), "en-US");
        assert_eq!(toggled_language_code("zh-Hans"), "en-US");
        assert_eq!(toggled_language_code("en-US"), "zh-CN");
        assert_eq!(
            nav_destination_for_shortcut_digit("1"),
            Some(NavDestination::Chats)
        );
        assert_eq!(
            nav_destination_for_shortcut_digit("2"),
            Some(NavDestination::Folders)
        );
        assert_eq!(
            nav_destination_for_shortcut_digit("3"),
            Some(NavDestination::Calls)
        );
        assert_eq!(
            nav_destination_for_shortcut_digit("4"),
            Some(NavDestination::TaskCenter)
        );
        assert_eq!(
            nav_destination_for_shortcut_digit("5"),
            Some(NavDestination::McpTools)
        );
        assert_eq!(
            nav_destination_for_shortcut_digit("6"),
            Some(NavDestination::TalkMode)
        );
        assert_eq!(
            nav_destination_for_shortcut_digit("7"),
            Some(NavDestination::SavedMessages)
        );
        assert_eq!(
            nav_destination_for_shortcut_digit("8"),
            Some(NavDestination::Settings)
        );
        assert_eq!(nav_destination_for_shortcut_digit("9"), None);
    }

    #[std::prelude::v1::test]
    fn test_sidebar_selection_revision_gates_panel_switch_ignoring_hover() {
        let mut last_seen = 0u64;
        // Hover/notify with unchanged revision (0 == 0) must NOT switch away from Settings.
        assert!(!should_switch_to_chats_on_sidebar_selection(0, &mut last_seen, true));
        assert_eq!(last_seen, 0);

        // Explicit dialog selection bumps revision -> triggers switch and advances watermark.
        assert!(should_switch_to_chats_on_sidebar_selection(1, &mut last_seen, true));
        assert_eq!(last_seen, 1);

        // Subsequent hover notifications at revision 1 again do NOT switch panels.
        assert!(!should_switch_to_chats_on_sidebar_selection(1, &mut last_seen, true));
    }

    #[std::prelude::v1::test]
    fn test_reconcile_stage_message_list_handles_ws_first_http_first_and_duplicates() {
        let mk = |id: u64, status: DeliveryStatus| MessageItemModel {
            id,
            sender_id: 10,
            sender_name: "Me".to_string(),
            avatar_initials: "ME".to_string(),
            avatar_color: 0x3498DB,
            direction: MessageDirection::Outgoing,
            timestamp: "13:45".to_string(),
            delivery_status: status,
            reply_to: None,
            reactions: vec![],
            content: MessageContent::Text {
                text: "Hello".to_string(),
                is_edited: false,
            },
            bot_keyboard: None,
            is_pinned: false,
        };

        // 1. WS arrives first (Read), then HTTP arrives (Sent): stays 1 bubble at Read.
        let mut ws_first = vec![mk(1_700_000_001, DeliveryStatus::Sending)];
        assert!(reconcile_stage_message_list(
            &mut ws_first,
            1_700_000_001,
            5,
            DeliveryStatus::Read
        ));
        assert!(reconcile_stage_message_list(
            &mut ws_first,
            1_700_000_001,
            5,
            DeliveryStatus::Sent
        ));
        assert_eq!(ws_first.len(), 1);
        assert_eq!(ws_first[0].id, 5);
        assert_eq!(ws_first[0].delivery_status, DeliveryStatus::Read);

        // 2. HTTP arrives first (Sent), then WS arrives (Read): stays 1 bubble promoted to Read.
        let mut http_first = vec![mk(1_700_000_002, DeliveryStatus::Sending)];
        assert!(reconcile_stage_message_list(
            &mut http_first,
            1_700_000_002,
            6,
            DeliveryStatus::Sent
        ));
        assert!(reconcile_stage_message_list(
            &mut http_first,
            1_700_000_002,
            6,
            DeliveryStatus::Read
        ));
        assert_eq!(http_first.len(), 1);
        assert_eq!(http_first[0].id, 6);
        assert_eq!(http_first[0].delivery_status, DeliveryStatus::Read);

        // 3. Even if both optimistic and server bubbles exist simultaneously, collapses to 1.
        let mut both = vec![
            mk(7, DeliveryStatus::Read),
            mk(1_700_000_003, DeliveryStatus::Sending),
        ];
        assert!(reconcile_stage_message_list(
            &mut both,
            1_700_000_003,
            7,
            DeliveryStatus::Sent
        ));
        assert_eq!(both.len(), 1);
        assert_eq!(both[0].id, 7);
        assert_eq!(both[0].delivery_status, DeliveryStatus::Read);
    }
}
