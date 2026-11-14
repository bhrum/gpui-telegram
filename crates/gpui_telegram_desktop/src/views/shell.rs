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

use crate::views::{
    composer::KeyAction,
    CallOverlay, CanonicalComposer, ChatSidebar, ComputerWorkbench, ContextInspector,
    ConversationStage, MediaViewer, ModalContainer, NavRail,
    nav_rail::NavDestination,
    chat_sidebar::{
        CategorizedSearchResult, CreationInputField, DialogDeliveryStatus, DialogItemModel,
        PendingAddContactRequest, PendingCreateGroupRequest,
    },
    stage::cards::{
        AlbumItem, DeliveryStatus, InlineButtonType, InlineKeyboardButton, MediaType,
        MessageContent, MessageDirection, MessageItemModel, PollOption, PollType,
    },
};
use crate::app::preferences::AppPreferences;
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
    net::http_client::{
        AddContactHttpRequest, AlipayPollResponse, AlipayStartResponse, EmailRegisterRequest,
        EmailSendCodeRequest, EmailSendCodeResponse,
    },
    AgentRunRequest, AuthResponse, ChatSummary, ClientConfig, ConnectionStatus, HttpClientError,
    LoginRequest, RegisterRequest, SendMessageHttpRequest, StoredSession, SyncOutcome,
    TelegramClient,
};
use gpui_telegram_core::types::ChatId;
use gpui_telegram_proto::sync::{
    PtsUpdate, SyncGetDifferenceRequest, SyncGetDifferenceResponse, UpdateEvent,
};
use gpui_telegram_proto::ws::{WsClientFrame, WsServerFrame};
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

        // 1. Observe NavRail clicks (Destinations, Overlays, Logout)
        cx.observe(&nav_rail, |shell: &mut AppShell, rail: Entity<NavRail>, cx: &mut Context<AppShell>| {
            let (dest, logout_req, trigger) = {
                let r = rail.read(cx);
                (r.active_destination, r.logout_requested, r.trigger_counter)
            };
            if logout_req {
                rail.update(cx, |r, _| {
                    r.logout_requested = false;
                    r.is_connected = false;
                });
                shell.logout(cx);
                return;
            }

            // Click-intent & Change-detection Guard:
            // Background notifications (e.g. WebSocket connection updates in line 913,
            // unread count updates, or badge updates) notify `nav_rail` but DO NOT
            // increment `trigger_counter`.
            // Ignore if trigger hasn't changed AND destination hasn't changed.
            if shell.last_handled_nav_trigger == trigger && shell.last_handled_nav_dest == Some(dest) {
                return;
            }
            shell.last_handled_nav_trigger = trigger;
            shell.last_handled_nav_dest = Some(dest);

            match dest {
                NavDestination::Chats | NavDestination::Folders | NavDestination::SavedMessages => {
                    shell.active_panel = ActiveWorkspacePanel::Chats;
                    shell.sidebar.update(cx, |sb, sb_cx| {
                        match dest {
                            NavDestination::Chats => sb.select_folder("All"),
                            NavDestination::Folders => sb.select_folder("Work"),
                            NavDestination::SavedMessages => sb.select_dialog(3),
                            _ => {}
                        }
                        sb_cx.notify();
                    });
                }
                NavDestination::Calls => {
                    shell.active_panel = ActiveWorkspacePanel::Calls;
                }
                NavDestination::TalkMode => {
                    shell.show_talk_mode(cx);
                }
                NavDestination::RemoteComputer => {
                    shell.show_remote_computer(cx);
                }
                NavDestination::TaskCenter => {
                    shell.active_panel = ActiveWorkspacePanel::TaskCenter;
                }
                NavDestination::McpTools => {
                    shell.active_panel = ActiveWorkspacePanel::McpTools;
                }
                NavDestination::Settings => {
                    if shell.active_panel == ActiveWorkspacePanel::Settings {
                        shell.settings_section = SettingsSectionType::Main;
                        shell.settings_stack.clear();
                    } else {
                        shell.active_panel = ActiveWorkspacePanel::Settings;
                    }
                }
            }
            cx.notify();
        }).detach();


        // 2. Observe ChatSidebar clicks (Dialog selection, Story click, Search focus, + Creation Modal, Categorized Search)
        cx.observe(&sidebar, |shell: &mut AppShell, sb: Entity<ChatSidebar>, cx: &mut Context<AppShell>| {
            let (
                selected_dialog,
                story_peer,
                search_focused,
                has_creation_modal,
                pending_contact,
                pending_group,
                search_query,
                search_cat,
                search_seq,
            ) = sb.update(cx, |s, _| {
                (
                    s.selected_dialog().cloned(),
                    s.story_clicked_peer.take(),
                    s.search_focused,
                    s.active_creation_modal.is_some(),
                    s.pending_add_contact_request.take(),
                    s.pending_create_group_request.take(),
                    s.search_query.clone(),
                    s.active_search_category.clone(),
                    s.search_request_seq,
                )
            });
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
                shell.show_media_viewer(format!("https://t.me/stories/{}", peer), false, cx);
            }
            if let Some(d) = selected_dialog {
                shell.active_panel = ActiveWorkspacePanel::Chats;
                shell.last_handled_nav_dest = Some(NavDestination::Chats);
                shell.nav_rail.update(cx, |r, _| {
                    r.set_active_destination(NavDestination::Chats);
                });
                let subtitle = if d.is_online { "online" } else { "last seen recently" };
                shell.stage.update(cx, |st, st_cx| {
                    if st.active_chat_id != Some(d.id) {
                        st.set_chat(d.id, &d.peer_name, subtitle, &d.avatar_initials, d.avatar_color);
                        st_cx.notify();
                    }
                });
                shell.inspector.update(cx, |insp, insp_cx| {
                    insp.set_peer(&d.peer_name, subtitle, &d.avatar_initials, d.avatar_color);
                    insp_cx.notify();
                });
                shell.load_historical_messages(d.id, cx);
            }
            cx.notify();
        }).detach();

        // 3. Observe ConversationStage clicks (Call button, Inspector toggle, Media lightbox, Bot Keyboard)
        cx.observe(&stage, |shell: &mut AppShell, st: Entity<ConversationStage>, cx: &mut Context<AppShell>| {
            let (call_req, media_req, bot_act, insp_open) = {
                let s = st.read(cx);
                (s.pending_call_request, s.pending_media_url.clone(), s.pending_bot_action.clone(), s.is_inspector_open)
            };
            if call_req {
                st.update(cx, |s, _| s.pending_call_request = false);
                shell.show_talk_mode(cx);
            }
            if let Some((url, is_video)) = media_req {
                st.update(cx, |s, _| s.pending_media_url = None);
                shell.show_media_viewer(url, is_video, cx);
            }
            if let Some(action) = bot_act {
                st.update(cx, |s, _| s.pending_bot_action = None);
                match action.as_str() {
                    "Launch Talk Mode" => shell.show_talk_mode(cx),
                    "Remote Computer" => shell.show_remote_computer(cx),
                    "View Task Matrix" => {
                        shell.active_panel = ActiveWorkspacePanel::TaskCenter;
                        shell.last_handled_nav_dest = Some(NavDestination::TaskCenter);
                        shell.nav_rail.update(cx, |r, _| {
                            r.set_active_destination(NavDestination::TaskCenter);
                        });
                    }
                    other => {
                        let modal_title = format!("{} • GPUI Telegram Docs", other);
                        shell.modal_container.update(cx, |mc, mc_cx| {
                            mc.show_modal(
                                modal_title,
                                440.0,
                                240.0,
                                crate::components::ui::layers::LayerStackOption::CloseOther,
                            );
                            mc_cx.notify();
                        });
                    }
                }
            }
            shell.inspector.update(cx, |insp, insp_cx| {
                if insp.is_open != insp_open {
                    insp.is_open = insp_open;
                    insp_cx.notify();
                }
            });
            cx.notify();
        }).detach();

        // 4. Observe CanonicalComposer (Send message, Send attachment/voice, Input focus)
        cx.observe(&composer, |shell: &mut AppShell, comp: Entity<CanonicalComposer>, cx: &mut Context<AppShell>| {
            let (msg_opt, att_opt, focused) = {
                let c = comp.read(cx);
                (c.submitted_message.clone(), c.submitted_attachment.clone(), c.is_input_focused)
            };
            if focused {
                shell.sidebar.update(cx, |sb, _| sb.search_focused = false);
            }
            if let Some(text) = msg_opt {
                comp.update(cx, |c, _| c.submitted_message = None);
                shell.append_outgoing_text(text, cx);
            }
            if let Some(kind) = att_opt {
                comp.update(cx, |c, _| c.submitted_attachment = None);
                shell.append_outgoing_attachment(&kind, cx);
            }
            cx.notify();
        }).detach();

        // 5. Observe ContextInspector close button & shared media item clicks
        cx.observe(&inspector, |shell: &mut AppShell, insp: Entity<ContextInspector>, cx: &mut Context<AppShell>| {
            let (is_open, media_open) = {
                let i = insp.read(cx);
                (i.is_open, i.pending_media_open.clone())
            };
            if let Some((url, is_video)) = media_open {
                insp.update(cx, |i, _| i.pending_media_open = None);
                shell.show_media_viewer(url, is_video, cx);
            }
            shell.stage.update(cx, |st, _| {
                st.is_inspector_open = is_open;
            });
            cx.notify();
        }).detach();

        let shell_instance = Self {
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
            phone_number: "138 0013 8000".to_string(),
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
        };


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
                            shell.is_authenticated = true;
                            shell.session_token = Some(token.clone());
                            shell.session_id = Some(session_id);
                            shell.user_id = Some(me.user_id);
                            shell.username = resolved_user.clone();
                            shell.client.ws_transport.set_token(Some(token));
                            shell.client.ws_transport.start();
                            shell.start_ws_inbound_stream(cx);
                            shell.refresh_chats(cx);
                            shell.sidebar.update(cx, |sb, sb_cx| {
                                sb.configure_for_cloud_workspace(&srv_url);
                                sb_cx.notify();
                            });
                            shell.stage.update(cx, |st, st_cx| {
                                st.configure_for_cloud_workspace(&srv_url, resolved_user.as_deref());
                                st_cx.notify();
                            });
                            shell.inspector.update(cx, |insp, insp_cx| {
                                insp.configure_for_cloud_workspace(&srv_url);
                                insp_cx.notify();
                            });
                            cx.notify();
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
        self.is_authenticated = true;
        self.session_token = Some(auth.token.clone());
        self.session_id = Some(auth.session_id.clone());
        self.user_id = Some(auth.user_id);
        self.username = Some(auth.username.clone());
        self.ime_preedit = None;
        self.client.ws_transport.set_token(Some(auth.token.clone()));
        self.client.ws_transport.start();
        self.start_ws_inbound_stream(cx);
        self.refresh_chats(cx);
        self.sidebar.update(cx, |sb, sb_cx| {
            sb.configure_for_cloud_workspace(&srv_url);
            sb_cx.notify();
        });
        self.stage.update(cx, |st, st_cx| {
            st.configure_for_cloud_workspace(&srv_url, Some(&auth.username));
            st_cx.notify();
        });
        self.inspector.update(cx, |insp, insp_cx| {
            insp.configure_for_cloud_workspace(&srv_url);
            insp_cx.notify();
        });

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
        self.user_id = None;
        self.username = None;
        self.is_authenticated = false;
        self.ws_stream_started = false;
        self.login_step = LoginWizardStep::Start;
        self.verification_code.clear();
        self.client.ws_transport.stop();
        self.client.ws_transport.set_token(None);
        self.nav_rail.update(cx, |r, _| r.is_connected = false);
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

    fn append_outgoing_text(&mut self, text: String, cx: &mut Context<Self>) {
        let msg_id = self.next_message_id;
        self.next_message_id += 1;

        let active_chat_id = self.stage.read(cx).active_chat_id.unwrap_or(1);
        let peer_name = self.stage.read(cx).chat_title.clone();
        let peer_initials = self.stage.read(cx).chat_avatar_initials.clone();
        let peer_color = self.stage.read(cx).chat_avatar_color;

        // 1. Add user's outgoing message bubble
        self.stage.update(cx, |st, st_cx| {
            st.add_message(MessageItemModel {
                id: msg_id,
                sender_id: self.user_id.unwrap_or(1),
                sender_name: "You".to_string(),
                avatar_initials: "ME".to_string(),
                avatar_color: 0x3498DB,
                direction: MessageDirection::Outgoing,
                timestamp: "Just now".to_string(),
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
        self.sidebar.update(cx, |sb, sb_cx| {
            if let Some(d) = sb.dialogs.iter_mut().find(|d| d.id == active_chat_id) {
                d.last_message = format!("You: {}", text);
                d.timestamp = "Just now".to_string();
                d.delivery_status = DialogDeliveryStatus::Sending;
            }
            sb_cx.notify();
        });

        // 3. Dispatch real HTTP POST /api/v1/messages/send
        if let Some(token) = self.session_token.clone() {
            let client = self.client.clone();
            let http_req = SendMessageHttpRequest {
                chat_id: active_chat_id,
                text: text.clone(),
                reply_to: None,
            };
            let sent_msg_id = msg_id;

            cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
                let res = client.http_client.send_message(&token, &http_req).await;
                let _ = weak_shell.update(app, |shell, cx| {
                    match res {
                        Ok(resp) => {
                            shell.client.sync_coordinator.set_pts(
                                ChatId(active_chat_id as i64),
                                resp.pts,
                            );

                            shell.stage.update(cx, |st, st_cx| {
                                if let Some(m) = st.messages.iter_mut().find(|m| m.id == sent_msg_id) {
                                    m.id = resp.message_id;
                                    m.delivery_status = DeliveryStatus::Sent;
                                }
                                if let Some(hist) = st.chat_histories.get_mut(&active_chat_id) {
                                    if let Some(m) = hist.iter_mut().find(|m| m.id == sent_msg_id) {
                                        m.id = resp.message_id;
                                        m.delivery_status = DeliveryStatus::Sent;
                                    }
                                }
                                st_cx.notify();
                            });
                            shell.sidebar.update(cx, |sb, sb_cx| {
                                if let Some(d) = sb.dialogs.iter_mut().find(|d| d.id == active_chat_id) {
                                    d.delivery_status = DialogDeliveryStatus::Sent;
                                }
                                sb_cx.notify();
                            });
                        }
                        Err(err) => {
                            eprintln!("[AppShell] Message send HTTP failure: {}. Preserving local message", err);
                        }
                    }
                    cx.notify();
                });
            })
            .detach();
        }

        // 4. Also dispatch WebSocket frame
        let ws_req = gpui_telegram_proto::ws::SendMessageRequest::new(
            ChatId(active_chat_id as i64),
            msg_id,
            gpui_telegram_proto::message::MessageContent::plain_text(text.clone()),
        );
        let _ = self.client.ws_channel.send(WsClientFrame::SendMessage(ws_req));

        // 5. Check if target chat is an AI Bot (Chat 4 or contains "Autonomous Agent" / "Bot")
        let is_ai_bot = active_chat_id == 4
            || peer_name.contains("Autonomous Agent")
            || peer_name.contains("Bot");

        if is_ai_bot {
            let reply_id = self.next_message_id;
            self.next_message_id += 1;

            // Insert placeholder thinking bubble
            self.stage.update(cx, |st, st_cx| {
                st.add_message(MessageItemModel {
                    id: reply_id,
                    sender_id: active_chat_id,
                    sender_name: peer_name.clone(),
                    avatar_initials: peer_initials.clone(),
                    avatar_color: peer_color,
                    direction: MessageDirection::Incoming,
                    timestamp: "Just now".to_string(),
                    delivery_status: DeliveryStatus::Read,
                    reply_to: None,
                    reactions: vec![],
                    content: MessageContent::Text {
                        text: "Thinking... 💭".to_string(),
                        is_edited: false,
                    },
                    bot_keyboard: None,
                    is_pinned: false,
                });
                st_cx.notify();
            });

            // Set typing indicator in ChatSidebar
            self.sidebar.update(cx, |sb, sb_cx| {
                sb.set_typing(
                    active_chat_id,
                    Some("Autonomous Agent Bot is thinking...".to_string()),
                );
                sb_cx.notify();
            });

            let client = self.client.clone();
            let token = self.session_token.clone().unwrap_or_default();
            let session_id = self
                .session_id
                .clone()
                .unwrap_or_else(|| format!("sess_guest_{}", active_chat_id));
            let prompt = text.clone();

            cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
                let agent_req = AgentRunRequest {
                    prompt,
                    session_id,
                    model_id: Some("grok-3".to_string()),
                    thinking_budget_tokens: Some(1024),
                };
                let result = client.http_client.agent_runs(&token, &agent_req).await;

                let _ = weak_shell.update(app, |shell, cx| {
                    // Clear typing indicator in ChatSidebar
                    shell.sidebar.update(cx, |sb, sb_cx| {
                        sb.set_typing(active_chat_id, None);
                        sb_cx.notify();
                    });

                    match result {
                        Ok(run_resp) => {
                            let response_text = run_resp.response;
                            let snippet = response_text
                                .lines()
                                .next()
                                .unwrap_or(&response_text)
                                .to_string();

                            let is_current =
                                shell.stage.read(cx).active_chat_id == Some(active_chat_id);
                            if is_current {
                                shell.stage.update(cx, |st, st_cx| {
                                    if let Some(msg) =
                                        st.messages.iter_mut().find(|m| m.id == reply_id)
                                    {
                                        msg.content = MessageContent::Text {
                                            text: response_text.clone(),
                                            is_edited: false,
                                        };
                                        msg.bot_keyboard = Some(vec![vec![
                                            InlineKeyboardButton {
                                                text: "Launch Talk Mode".to_string(),
                                                button_type: InlineButtonType::Callback(
                                                    "action_talk_mode".to_string(),
                                                ),
                                            },
                                            InlineKeyboardButton {
                                                text: "Remote Computer".to_string(),
                                                button_type: InlineButtonType::Callback(
                                                    "action_remote_computer".to_string(),
                                                ),
                                            },
                                        ]]);
                                    }
                                    st_cx.notify();
                                });
                            } else {
                                shell.stage.update(cx, |st, _| {
                                    if let Some(history) =
                                        st.chat_histories.get_mut(&active_chat_id)
                                    {
                                        if let Some(msg) =
                                            history.iter_mut().find(|m| m.id == reply_id)
                                        {
                                            msg.content = MessageContent::Text {
                                                text: response_text.clone(),
                                                is_edited: false,
                                            };
                                            msg.bot_keyboard = Some(vec![vec![
                                                InlineKeyboardButton {
                                                    text: "Launch Talk Mode".to_string(),
                                                    button_type: InlineButtonType::Callback(
                                                        "action_talk_mode".to_string(),
                                                    ),
                                                },
                                                InlineKeyboardButton {
                                                    text: "Remote Computer".to_string(),
                                                    button_type: InlineButtonType::Callback(
                                                        "action_remote_computer".to_string(),
                                                    ),
                                                },
                                            ]]);
                                        }
                                    }
                                });
                            }

                            shell.sidebar.update(cx, |sb, sb_cx| {
                                if let Some(d) =
                                    sb.dialogs.iter_mut().find(|d| d.id == active_chat_id)
                                {
                                    d.last_message = snippet;
                                    d.timestamp = "Just now".to_string();
                                }
                                sb_cx.notify();
                            });
                        }
                        Err(err) => {
                            let err_msg = format!("⚠️ AI Agent Error: {}", err);
                            shell.stage.update(cx, |st, st_cx| {
                                if let Some(msg) =
                                    st.messages.iter_mut().find(|m| m.id == reply_id)
                                {
                                    msg.content = MessageContent::Text {
                                        text: err_msg,
                                        is_edited: false,
                                    };
                                }
                                st_cx.notify();
                            });
                        }
                    }
                    cx.notify();
                });
            })
            .detach();
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
                let active_chat = self.stage.read(cx).active_chat_id.unwrap_or(1);
                self.apply_update_event(ChatId(active_chat as i64), update_event, cx);
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

    fn handle_ws_ack(&mut self, client_nonce: u64, message_id: i64, pts: u64, cx: &mut Context<Self>) {
        let active_chat = self.stage.read(cx).active_chat_id;
        if let Some(cid) = active_chat {
            if pts > 0 {
                self.client.sync_coordinator.set_pts(ChatId(cid as i64), pts);
            }
        }

        self.stage.update(cx, |st, st_cx| {
            if let Some(msg) = st.messages.iter_mut().find(|m| m.id == client_nonce) {
                if message_id > 0 {
                    msg.id = message_id as u64;
                }
                msg.delivery_status = DeliveryStatus::Sent;
                st_cx.notify();
            }
            if let Some(cid) = active_chat {
                if let Some(history) = st.chat_histories.get_mut(&cid) {
                    if let Some(msg) = history.iter_mut().find(|m| m.id == client_nonce) {
                        if message_id > 0 {
                            msg.id = message_id as u64;
                        }
                        msg.delivery_status = DeliveryStatus::Sent;
                    }
                }
            }
        });

        if let Some(cid) = active_chat {
            self.sidebar.update(cx, |sb, sb_cx| {
                if let Some(d) = sb.dialogs.iter_mut().find(|d| d.id == cid) {
                    d.delivery_status = DialogDeliveryStatus::Sent;
                }
                sb_cx.notify();
            });
        }
        cx.notify();
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

    fn apply_update_event(&mut self, chat_id: ChatId, event: UpdateEvent, cx: &mut Context<Self>) {
        match event {
            UpdateEvent::NewMessage(msg) => {
                let current_user_id = self.user_id.unwrap_or(0);
                let is_outgoing = msg.sender_id.0 == current_user_id;

                if is_outgoing {
                    if let Some(nonce) = msg.client_nonce {
                        let found = self.stage.read(cx).messages.iter().any(|m| m.id == nonce);
                        if found {
                            self.handle_ws_ack(nonce, msg.id.0, 0, cx);
                            return;
                        }
                    }
                }

                let preview_text = match &msg.content {
                    gpui_telegram_proto::message::MessageContent::Text { text, .. } => text.clone(),
                    _ => "[Media attachment]".to_string(),
                };

                let model = MessageItemModel {
                    id: msg.id.0 as u64,
                    sender_id: msg.sender_id.0,
                    sender_name: if is_outgoing { "You".to_string() } else { "Peer".to_string() },
                    avatar_initials: if is_outgoing { "ME".to_string() } else { "PR".to_string() },
                    avatar_color: if is_outgoing { 0x3498DB } else { 0x9B59B6 },
                    direction: if is_outgoing { MessageDirection::Outgoing } else { MessageDirection::Incoming },
                    timestamp: msg.date.format("%H:%M").to_string(),
                    delivery_status: DeliveryStatus::Read,
                    reply_to: None,
                    reactions: vec![],
                    content: MessageContent::Text {
                        text: preview_text.clone(),
                        is_edited: false,
                    },
                    bot_keyboard: None,
                    is_pinned: false,
                };

                let chat_u64 = chat_id.0 as u64;
                let is_current_chat = self.stage.read(cx).active_chat_id == Some(chat_u64);

                if is_current_chat {
                    self.stage.update(cx, |st, st_cx| {
                        st.add_message(model.clone());
                        st_cx.notify();
                    });
                } else {
                    self.stage.update(cx, |st, _| {
                        st.chat_histories.entry(chat_u64).or_default().push(model);
                    });
                }

                self.sidebar.update(cx, |sb, sb_cx| {
                    if let Some(dialog) = sb.dialogs.iter_mut().find(|d| d.id == chat_u64) {
                        dialog.last_message = preview_text;
                        dialog.timestamp = msg.date.format("%H:%M").to_string();
                        if !is_current_chat && !is_outgoing {
                            dialog.unread_count += 1;
                        }
                    }
                    sb_cx.notify();
                });
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
        let token = match self.session_token.clone() {
            Some(t) => t,
            None => return,
        };
        let client = self.client.clone();

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.get_chats(&token).await;
            let _ = weak_shell.update(app, |shell, cx| {
                match res {
                    Ok(chats) if !chats.is_empty() => {
                        let mut auto_selected_welcome: Option<DialogItemModel> = None;
                        shell.sidebar.update(cx, |sb, sb_cx| {
                            // Remove any fabricated local bootstrap dialogs (id <= 5) not present on the server
                            sb.dialogs
                                .retain(|d| d.id > 5 || chats.iter().any(|c| c.chat_id == d.id));

                            for server_chat in chats {
                                let is_welcome_bot = (server_chat.chat_id >= 700_000_000
                                    && server_chat.chat_id < 750_000_000)
                                    || server_chat.title.contains("Telegram Official Bot");
                                let is_bot = is_welcome_bot || server_chat.is_bot.unwrap_or(false);
                                if let Some(existing) = sb.dialogs.iter_mut().find(|d| d.id == server_chat.chat_id) {
                                    existing.peer_name = server_chat.title;
                                    if let Some(msg) = server_chat.last_message {
                                        existing.last_message = msg;
                                    }
                                    existing.unread_count = server_chat.unread_count;
                                    if is_welcome_bot && (sb.active_chat_id == Some(4) || sb.active_chat_id.is_none()) {
                                        auto_selected_welcome = Some(existing.clone());
                                    }
                                } else {
                                    let initials = if is_welcome_bot {
                                        "TG".to_string()
                                    } else if is_bot {
                                        "🤖".to_string()
                                    } else {
                                        server_chat
                                            .title
                                            .chars()
                                            .take(2)
                                            .collect::<String>()
                                            .to_uppercase()
                                    };
                                    let folder = if is_bot {
                                        "Bots".to_string()
                                    } else if server_chat.chat_type != 0 {
                                        "Work".to_string()
                                    } else {
                                        "Personal".to_string()
                                    };
                                    let dialog_model = DialogItemModel {
                                        id: server_chat.chat_id,
                                        peer_name: server_chat.title,
                                        avatar_initials: initials,
                                        avatar_color: server_chat.avatar_color.unwrap_or(0x2AABEE),
                                        last_message: server_chat
                                            .last_message
                                            .unwrap_or_else(|| "No messages yet".to_string()),
                                        timestamp: "刚刚".to_string(),
                                        unread_count: server_chat.unread_count,
                                        is_muted: false,
                                        is_pinned: is_welcome_bot,
                                        is_online: is_bot || server_chat.chat_type == 0,
                                        typing_indicator: None,
                                        delivery_status: DialogDeliveryStatus::Read,
                                        folder,
                                    };
                                    if is_welcome_bot {
                                        sb.dialogs.insert(0, dialog_model.clone());
                                        if sb.active_chat_id == Some(4) || sb.active_chat_id.is_none() {
                                            sb.active_chat_id = Some(dialog_model.id);
                                            auto_selected_welcome = Some(dialog_model);
                                        }
                                    } else {
                                        sb.dialogs.push(dialog_model);
                                    }
                                }
                            }

                            // Ensure an active dialog is selected if the previous active_chat_id was a removed placeholder
                            if auto_selected_welcome.is_none() {
                                let current_valid = sb
                                    .active_chat_id
                                    .map(|id| sb.dialogs.iter().any(|d| d.id == id))
                                    .unwrap_or(false);
                                if !current_valid {
                                    if let Some(first) = sb.dialogs.first().cloned() {
                                        sb.active_chat_id = Some(first.id);
                                        auto_selected_welcome = Some(first);
                                    }
                                }
                            }

                            sb.recompute_folder_badges();
                            sb_cx.notify();
                        });
                        if let Some(welcome) = auto_selected_welcome {
                            shell.stage.update(cx, |st, st_cx| {
                                st.set_chat(
                                    welcome.id,
                                    &welcome.peer_name,
                                    "official bot · online",
                                    &welcome.avatar_initials,
                                    welcome.avatar_color,
                                );
                                st_cx.notify();
                            });
                            shell.inspector.update(cx, |insp, insp_cx| {
                                insp.set_peer(
                                    &welcome.peer_name,
                                    "official bot · online",
                                    &welcome.avatar_initials,
                                    welcome.avatar_color,
                                );
                                insp_cx.notify();
                            });
                            shell.load_historical_messages(welcome.id, cx);
                        }
                    }
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("[AppShell] Failed to fetch /api/v1/chats: {}. Retaining existing dialogs", err);
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    /// Fetches historical messages from the backend for the selected chat
    pub fn load_historical_messages(&mut self, chat_id: u64, cx: &mut Context<Self>) {
        let token = match self.session_token.clone() {
            Some(t) => t,
            None => return,
        };
        let client = self.client.clone();
        let my_user_id = self.user_id.unwrap_or(0);
        let (peer_name, peer_initials, peer_color) = {
            let sb = self.sidebar.read(cx);
            sb.dialogs
                .iter()
                .find(|d| d.id == chat_id)
                .map(|d| (d.peer_name.clone(), d.avatar_initials.clone(), d.avatar_color))
                .unwrap_or_else(|| ("Telegram Official Bot".to_string(), "TG".to_string(), 0x2AABEE))
        };

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.get_chat_messages(&token, chat_id, Some(50)).await;
            let _ = weak_shell.update(app, |shell, cx| {
                match res {
                    Ok(msgs) if !msgs.is_empty() => {
                        let models: Vec<MessageItemModel> = msgs
                            .into_iter()
                            .map(|hm| {
                                let is_outgoing = hm.is_outgoing.unwrap_or(hm.sender_id == my_user_id);
                                let incoming_name = if hm.sender_id == 777_000 {
                                    "Telegram Official Bot".to_string()
                                } else {
                                    peer_name.clone()
                                };
                                let incoming_initials = if hm.sender_id == 777_000 {
                                    "TG".to_string()
                                } else {
                                    peer_initials.clone()
                                };
                                MessageItemModel {
                                    id: hm.message_id,
                                    sender_id: hm.sender_id,
                                    sender_name: if is_outgoing { "You".to_string() } else { incoming_name },
                                    avatar_initials: if is_outgoing { "ME".to_string() } else { incoming_initials },
                                    avatar_color: if is_outgoing { 0x3498DB } else { peer_color },
                                    direction: if is_outgoing { MessageDirection::Outgoing } else { MessageDirection::Incoming },
                                    timestamp: hm.date.map(|d| {
                                        chrono::DateTime::from_timestamp_millis(d)
                                            .map(|dt| dt.format("%H:%M").to_string())
                                            .unwrap_or_else(|| "Earlier".to_string())
                                    }).unwrap_or_else(|| "Earlier".to_string()),
                                    delivery_status: DeliveryStatus::Read,
                                    reply_to: None,
                                    reactions: vec![],
                                    content: MessageContent::Text {
                                        text: hm.text,
                                        is_edited: false,
                                    },
                                    bot_keyboard: None,
                                    is_pinned: false,
                                }
                            })
                            .collect();

                        shell.stage.update(cx, |st, st_cx| {
                            if st.active_chat_id == Some(chat_id) {
                                st.merge_historical_messages(models.clone());
                                st.row_height_cache.rebuild_prefix_sums(&st.messages);
                                st_cx.notify();
                            }
                            st.chat_histories.insert(chat_id, models);
                        });
                    }
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("[AppShell] Failed to fetch history for chat {}: {}. Retaining cached messages", chat_id, err);
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn append_outgoing_attachment(&mut self, kind: &str, cx: &mut Context<Self>) {
        let msg_id = self.next_message_id;
        self.next_message_id += 1;

        let content = match kind {
            "voice" => MessageContent::Voice {
                duration_secs: 8,
                waveform_5bit: vec![5, 12, 24, 30, 28, 19, 14, 22, 31, 25, 16, 10, 18, 27, 21, 9],
                playback_position_secs: 0.0,
                is_playing: false,
                playback_speed: 1.0,
            },
            "photo" => MessageContent::Album {
                items: vec![
                    AlbumItem { id: 1, media_type: MediaType::Photo, url: "".to_string(), aspect_ratio: 1.33, preview_color: 0x2980B9, caption: None },
                ],
                caption: Some("📷 High-resolution screenshot sent from GPUI Desktop".to_string()),
            },
            "video" => MessageContent::Media {
                media_type: MediaType::Video,
                url: "https://example.com/capture.mp4".to_string(),
                width: 1920,
                height: 1080,
                duration_secs: Some(45),
                file_size_bytes: 8_400_000,
                caption: Some("🎬 120fps GPUI Metal Capture".to_string()),
                is_streaming: true,
            },
            "location" => MessageContent::Location {
                latitude: 31.2304,
                longitude: 121.4737,
                title: "Shared Live Location".to_string(),
                address: "GPUI Telegram Desktop Client".to_string(),
            },
            _ => MessageContent::Poll {
                question: "Ready to ship GPUI Telegram v1.0?".to_string(),
                poll_type: PollType::Regular,
                options: vec![
                    PollOption { id: 1, text: "Yes, ship it! 🚀".to_string(), votes: 1, percentage: 100.0, is_selected: true, is_correct: true },
                    PollOption { id: 2, text: "Run one more benchmark".to_string(), votes: 0, percentage: 0.0, is_selected: false, is_correct: false },
                ],
                total_votes: 1,
                is_closed: false,
                selected_option_id: Some(1),
            },
        };

        self.stage.update(cx, |st, st_cx| {
            st.add_message(MessageItemModel {
                id: msg_id,
                sender_id: 1,
                sender_name: "You".to_string(),
                avatar_initials: "ME".to_string(),
                avatar_color: 0x3498DB,
                direction: MessageDirection::Outgoing,
                timestamp: "Just now".to_string(),
                delivery_status: DeliveryStatus::Read,
                reply_to: None,
                reactions: vec![],
                content,
                bot_keyboard: None,
                is_pinned: false,
            });
            st_cx.notify();
        });
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

        self.composer.update(cx, |c, c_cx| {
            c.is_input_focused = true;
            if c.ime_machine.is_composing() {
                c.ime_machine.on_ime_commit(&clean);
            } else {
                c.ime_machine.insert_text(&clean);
            }
            c_cx.notify();
        });
        cx.notify();
    }

    fn handle_non_char_key(&mut self, key: &str, is_shift: bool, cx: &mut Context<Self>) -> bool {
        // Escape closes any active modal, overlay, or search focus
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
                    c_cx.notify();
                });
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
            if self.media_viewer_overlay.is_some() {
                self.hide_media_viewer();
                cx.notify();
                return true;
            }
            if self.talk_mode_overlay.is_some() {
                self.hide_talk_mode(cx);
                cx.notify();
                return true;
            }
            if self.computer_overlay.is_some() {
                self.hide_remote_computer(cx);
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
            if self.is_authenticated && self.sidebar.read(cx).search_focused {
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
            if self.active_panel == ActiveWorkspacePanel::Settings {
                self.navigate_settings_back(cx);
                cx.notify();
                return true;
            }
            return false;
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
                        let text_to_send = {
                            let c = self.composer.read(cx);
                            c.text().trim().to_string()
                        };
                        if !text_to_send.is_empty() {
                            self.composer.update(cx, |c, c_cx| {
                                c.clear();
                                c_cx.notify();
                            });
                            self.append_outgoing_text(text_to_send, cx);
                        }
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
                    c_cx.notify();
                });
                cx.notify();
                true
            }
            _ => false,
        }
    }

    fn handle_window_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let key = event.keystroke.key.as_str();

        // 1. Handle Cmd / Ctrl shortcuts (Paste, Clear, Search focus)
        if event.keystroke.modifiers.platform || event.keystroke.modifiers.control {
            match key {
                "v" => {
                    if let Some(clip_text) = cx.read_from_clipboard().and_then(|item| item.text()) {
                        self.insert_committed_text(&clip_text, cx);
                    }
                    cx.stop_propagation();
                }
                "backspace" | "a" | "u" => {
                    self.clear_active_field(cx);
                    cx.stop_propagation();
                }
                "k" if self.is_authenticated => {
                    self.sidebar.update(cx, |sb, sb_cx| {
                        sb.focus_search();
                        sb_cx.notify();
                    });
                    self.composer.update(cx, |c, c_cx| {
                        c.is_input_focused = false;
                        c_cx.notify();
                    });
                    cx.notify();
                    cx.stop_propagation();
                }
                "," if self.is_authenticated => {
                    self.navigate_settings_to(SettingsSectionType::Main, cx);
                    cx.stop_propagation();
                }
                "[" if self.active_panel == ActiveWorkspacePanel::Settings => {
                    self.navigate_settings_back(cx);
                    cx.stop_propagation();
                }
                _ => {}
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
            match key {
                "v" => {
                    if let Some(clip_text) = cx.read_from_clipboard().and_then(|item| item.text()) {
                        self.insert_committed_text(&clip_text, cx);
                    }
                }
                "backspace" | "a" | "u" => {
                    self.clear_active_field(cx);
                }
                "," if self.is_authenticated => {
                    self.navigate_settings_to(SettingsSectionType::Main, cx);
                }
                "[" if self.active_panel == ActiveWorkspacePanel::Settings => {
                    self.navigate_settings_back(cx);
                }
                _ => {}
            }
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
        };
        if new_text.is_empty() {
            self.ime_preedit = None;
            if self.is_authenticated && !sidebar_capturing {
                self.composer.update(cx, |c, c_cx| {
                    c.ime_machine.on_ime_cancel();
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
                .bg(rgb(0x0E1621))
                .flex()
                .items_center()
                .justify_center()
                .child(ime_canvas)
                .child(self.render_login_wizard(cx))
                .into_any_element();
        }

        div()
            .id("app-shell-authenticated-root")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                this.handle_window_key_down(event, window, cx);
            }))
            .relative()
            .size_full()
            .flex()
            .bg(rgb(0x0E1621))
            .child(ime_canvas)
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
            .child(self.inspector.clone())
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
                el.child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .occlude()
                        .bg(rgba(0x00000088))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(overlay),
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
            // 5b. Centered Telegram-style Contacts Modal Overlay (`media_1794178846290_753d90ef.jpg`)
            .when(self.sidebar.read(cx).is_contacts_modal_open, |el| {
                let sb_ref = self.sidebar.read(cx);
                let contact_q = sb_ref.contacts_search_query.clone();
                let q_lower = contact_q.trim().to_lowercase();
                let contact_dialogs: Vec<DialogItemModel> = sb_ref
                    .dialogs
                    .iter()
                    .filter(|d| {
                        (d.folder == "Personal" || d.folder == "Bots")
                            && !d.peer_name.contains("Telegram Official Bot")
                            && (q_lower.is_empty()
                                || d.peer_name.to_lowercase().contains(&q_lower)
                                || d.last_message.to_lowercase().contains(&q_lower))
                    })
                    .cloned()
                    .collect();

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
                                .bg(rgb(0x17212B))
                                .border_1()
                                .border_color(rgb(0x242F3D))
                                .flex()
                                .flex_col()
                                .overflow_hidden()
                                // Header: "联系人" + Sort button "⇅A"
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
                                                .text_color(rgb(0xFFFFFF))
                                                .child("联系人"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.0))
                                                .text_color(rgb(0x8A9AA8))
                                                .child("⇅A"),
                                        ),
                                )
                                // Search Bar: "🔍 搜索"
                                .child(
                                    div()
                                        .w_full()
                                        .px(px(16.0))
                                        .py(px(8.0))
                                        .border_b_1()
                                        .border_color(rgb(0x0E1621))
                                        .flex()
                                        .items_center()
                                        .gap(px(8.0))
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .text_color(rgb(0x6B808D))
                                                .child("🔍"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(13.5))
                                                .text_color(if contact_q.is_empty() {
                                                    rgb(0x6B808D)
                                                } else {
                                                    rgb(0xFFFFFF)
                                                })
                                                .child(if contact_q.is_empty() {
                                                    "搜索".to_string()
                                                } else {
                                                    contact_q
                                                }),
                                        ),
                                )
                                // Body: Contact rows or "未找到相关联系人" empty state
                                .child(
                                    div()
                                        .id("contacts-modal-scroll")
                                        .flex_1()
                                        .w_full()
                                        .overflow_y_scroll()
                                        .flex()
                                        .flex_col()
                                        .when(contact_dialogs.is_empty(), |body| {
                                            body.items_center().justify_center().child(
                                                div()
                                                    .text_size(px(13.5))
                                                    .text_color(rgb(0x6B808D))
                                                    .child("未找到相关联系人"),
                                            )
                                        })
                                        .when(!contact_dialogs.is_empty(), |body| {
                                            body.children(contact_dialogs.into_iter().map(|d| {
                                                let cid = d.id;
                                                div()
                                                    .cursor_pointer()
                                                    .w_full()
                                                    .px(px(16.0))
                                                    .py(px(10.0))
                                                    .hover(|s| s.bg(rgb(0x202B36)))
                                                    .flex()
                                                    .items_center()
                                                    .gap(px(12.0))
                                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                                        this.sidebar.update(cx, |sb, sb_cx| {
                                                            sb.close_contacts_modal();
                                                            sb.select_dialog(cid);
                                                            sb_cx.notify();
                                                        });
                                                        cx.notify();
                                                    }))
                                                    .child(
                                                        div()
                                                            .w(px(40.0))
                                                            .h(px(40.0))
                                                            .rounded_full()
                                                            .bg(rgb(d.avatar_color))
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .text_size(px(14.0))
                                                            .font_weight(FontWeight::BOLD)
                                                            .text_color(rgb(0xFFFFFF))
                                                            .child(d.avatar_initials),
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .child(
                                                                div()
                                                                    .text_size(px(14.0))
                                                                    .font_weight(FontWeight::BOLD)
                                                                    .text_color(rgb(0xFFFFFF))
                                                                    .child(d.peer_name),
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_size(px(12.0))
                                                                    .text_color(rgb(0x5288C1))
                                                                    .child(if d.folder == "Bots" {
                                                                        "AI Bot · 在线"
                                                                    } else {
                                                                        "在线"
                                                                    }),
                                                            ),
                                                    )
                                            }))
                                        }),
                                )
                                // Bottom Bar: "添加联系人" (left) and "关闭" (right)
                                .child(
                                    div()
                                        .w_full()
                                        .px(px(16.0))
                                        .py(px(12.0))
                                        .border_t_1()
                                        .border_color(rgb(0x0E1621))
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
                                                .hover(|s| s.bg(rgb(0x202B36)))
                                                .text_size(px(13.5))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0x2AABEE))
                                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                    this.sidebar.update(cx, |sb, sb_cx| {
                                                        sb.open_add_contact_modal(crate::views::chat_sidebar::AddContactKind::AiBot);
                                                        sb_cx.notify();
                                                    });
                                                    cx.notify();
                                                }))
                                                .child("添加联系人"),
                                        )
                                        .child(
                                            div()
                                                .id("contacts-modal-close-btn")
                                                .cursor_pointer()
                                                .px(px(10.0))
                                                .py(px(6.0))
                                                .rounded(px(6.0))
                                                .hover(|s| s.bg(rgb(0x202B36)))
                                                .text_size(px(13.5))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0x2AABEE))
                                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                    this.sidebar.update(cx, |sb, sb_cx| {
                                                        sb.close_contacts_modal();
                                                        sb_cx.notify();
                                                    });
                                                    cx.notify();
                                                }))
                                                .child("关闭"),
                                        ),
                                ),
                        ),
                )
            })
            // 6. Modal Layer Stack Container
            .child(self.modal_container.clone())
            .into_any_element()
    }
}

impl AppShell {
    fn render_login_wizard(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let zh = self.is_chinese_locale;
        let step = self.login_step;
        let country = self.country_code.clone();
        let phone = self.phone_number.clone();
        let code = self.verification_code.clone();

        div()
            .w(px(440.0))
            .rounded(px(16.0))
            .bg(rgb(0x17212B))
            .border_1()
            .border_color(rgb(0x242F3D))
            .p(px(32.0))
            .flex()
            .flex_col()
            .items_center()
            .gap(px(18.0))
            // Top Language Switcher Row
            .child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x6B808D))
                            .child("GPUI Telegram Desktop • MTProto 2.0"),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(rgb(0x242F3D))
                            .hover(|s| s.bg(rgb(0x2AABEE)))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.is_chinese_locale = !this.is_chinese_locale;
                                cx.notify();
                            }))
                            .child(if zh { "🌐 中文 / EN" } else { "🌐 EN / 中文" }),
                    ),
            )
            // Telegram Plane Logo Circle
            .child(
                div()
                    .w(px(84.0))
                    .h(px(84.0))
                    .rounded_full()
                    .bg(rgb(0x2AABEE))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(40.0))
                    .text_color(rgb(0xFFFFFF))
                    .child("✈️"),
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
                            let dot_color = if has_net_err { rgb(0xEF5350) } else { rgb(0x4CAF50) };
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
                                            .bg(rgb(0x101921))
                                            .border_1()
                                            .border_color(rgb(0x2AABEE))
                                            .flex()
                                            .items_center()
                                            .text_size(px(11.5))
                                            .text_color(if self.server_url_draft.is_empty() {
                                                rgb(0x5B6B79)
                                            } else {
                                                rgb(0xFFFFFF)
                                            })
                                            .child(draft_display),
                                    )
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .h(px(30.0))
                                            .px(px(10.0))
                                            .rounded(px(6.0))
                                            .bg(rgb(0x2AABEE))
                                            .hover(|s| s.bg(rgb(0x229ED9)))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(11.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.apply_custom_server_url(cx);
                                            }))
                                            .child(if zh { "保存" } else { "Save" }),
                                    )
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .h(px(30.0))
                                            .px(px(8.0))
                                            .rounded(px(6.0))
                                            .bg(rgb(0x242F3D))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(11.0))
                                            .text_color(rgb(0x8A9AA8))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.editing_server_url = false;
                                                this.server_url_draft = this.client.config.server_url.clone();
                                                cx.notify();
                                            }))
                                            .child(if zh { "取消" } else { "Cancel" }),
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
                                    .bg(rgb(0x101921))
                                    .border_1()
                                    .border_color(if has_net_err { rgb(0x7F1D1D) } else { rgb(0x242F3D) })
                                    .hover(|s| s.border_color(rgb(0x2AABEE)))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                        this.server_url_draft = this.client.config.server_url.clone();
                                        this.editing_server_url = true;
                                        cx.notify();
                                    }))
                                    .child(div().w(px(6.0)).h(px(6.0)).rounded_full().bg(dot_color))
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(rgb(0x8A9AA8))
                                            .child(format!("Cloud VM: {}", server_url)),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(10.5))
                                            .text_color(rgb(0x2AABEE))
                                            .child(if zh { "修改" } else { "Edit" }),
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
                                .bg(rgb(0x101921))
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
                                        .bg(if is_pw { rgb(0x2AABEE) } else { rgb(0x101921) })
                                        .hover(|s| if !is_pw { s.bg(rgb(0x1E2C3A)) } else { s })
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xFFFFFF))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                            this.switch_auth_tab(AuthWizardTab::PasswordLogin, cx);
                                        }))
                                        .child(if zh { "🔑 密码登录" } else { "🔑 Password" })
                                })
                                .child({
                                    let is_alipay = self.auth_wizard_tab == AuthWizardTab::AlipayQr;
                                    div()
                                        .cursor_pointer()
                                        .flex_1()
                                        .h(px(32.0))
                                        .rounded(px(6.0))
                                        .bg(if is_alipay { rgb(0x1677FF) } else { rgb(0x101921) })
                                        .hover(|s| if !is_alipay { s.bg(rgb(0x1E2C3A)) } else { s })
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xFFFFFF))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                            this.switch_auth_tab(AuthWizardTab::AlipayQr, cx);
                                        }))
                                        .child(if zh { "💳 支付宝扫码" } else { "💳 Alipay QR" })
                                })
                                .child({
                                    let is_email = self.auth_wizard_tab == AuthWizardTab::EmailRegister;
                                    div()
                                        .cursor_pointer()
                                        .flex_1()
                                        .h(px(32.0))
                                        .rounded(px(6.0))
                                        .bg(if is_email { rgb(0x2AABEE) } else { rgb(0x101921) })
                                        .hover(|s| if !is_email { s.bg(rgb(0x1E2C3A)) } else { s })
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xFFFFFF))
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                            this.switch_auth_tab(AuthWizardTab::EmailRegister, cx);
                                        }))
                                        .child(if zh { "✉️ 邮箱注册" } else { "✉️ Email Register" })
                                }),
                        )
                        // Error Banner if present
                        .when_some(self.login_error.clone(), |el, err| {
                            el.child(
                                div()
                                    .w_full()
                                    .p(px(8.0))
                                    .rounded(px(6.0))
                                    .bg(rgba(0xE5393522))
                                    .border_1()
                                    .border_color(rgb(0xE53935))
                                    .text_size(px(12.0))
                                    .text_color(rgb(0xFF5252))
                                    .child(format!("⚠️ {}", err)),
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
                                            .text_color(rgb(0xFFFFFF))
                                            .child(if zh {
                                                if is_reg { "注册 Telegram 云端账号" } else { "登录 Telegram 云端服务" }
                                            } else {
                                                if is_reg { "Create Cloud Account" } else { "Sign In to Cloud Server" }
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
                                                .bg(rgb(0x242F3D))
                                                .border_1()
                                                .border_color(if fn_active { rgb(0x2AABEE) } else { rgb(0x242F3D) })
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
                                                            .text_color(rgb(0x6B808D))
                                                            .child(if fn_active {
                                                                "| Your Display Name (输入昵称)".to_string()
                                                            } else {
                                                                "Your Display Name (显示昵称)".to_string()
                                                            })
                                                    } else {
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .gap(px(2.0))
                                                            .child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(0xFFFFFF))
                                                                    .child(fn_val.clone()),
                                                            )
                                                            .when_some(fn_preedit, |p_el, p| {
                                                                p_el.child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .text_color(rgb(0x2AABEE))
                                                                        .border_b_1()
                                                                        .border_color(rgb(0x2AABEE))
                                                                        .child(p),
                                                                )
                                                            })
                                                            .when(fn_active, |c_el| {
                                                                c_el.child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .text_color(rgb(0x2AABEE))
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
                                                                    .text_size(px(12.0))
                                                                    .text_color(rgb(0x8A9AA8))
                                                                    .hover(|s| s.text_color(rgb(0xFFFFFF)))
                                                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                                                                        this.first_name_input.clear();
                                                                        this.ime_preedit = None;
                                                                        this.active_login_field = LoginInputField::FirstName;
                                                                        this.focus_handle.focus(window);
                                                                        cx.stop_propagation();
                                                                        cx.notify();
                                                                    }))
                                                                    .child("✕"),
                                                            )
                                                        })
                                                        .child(
                                                            div()
                                                                .text_size(px(11.0))
                                                                .text_color(rgb(0x6B808D))
                                                                .child("🪪"),
                                                        ),
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
                                            .bg(rgb(0x242F3D))
                                            .border_1()
                                            .border_color(if u_active { rgb(0x2AABEE) } else { rgb(0x242F3D) })
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
                                                        .text_color(rgb(0x6B808D))
                                                        .child(if u_active {
                                                            "| Username (请输入用户名)".to_string()
                                                        } else {
                                                            "Username (用户名)".to_string()
                                                        })
                                                } else {
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(2.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .text_color(rgb(0xFFFFFF))
                                                                .child(user_val.clone()),
                                                        )
                                                        .when_some(u_preedit, |p_el, p| {
                                                            p_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(0x2AABEE))
                                                                    .border_b_1()
                                                                    .border_color(rgb(0x2AABEE))
                                                                    .child(p),
                                                                )
                                                        })
                                                        .when(u_active, |c_el| {
                                                            c_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(0x2AABEE))
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
                                                                .text_size(px(12.0))
                                                                .text_color(rgb(0x8A9AA8))
                                                                .hover(|s| s.text_color(rgb(0xFFFFFF)))
                                                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                                                                    this.username_input.clear();
                                                                    this.ime_preedit = None;
                                                                    this.active_login_field = LoginInputField::Username;
                                                                    this.focus_handle.focus(window);
                                                                    cx.stop_propagation();
                                                                    cx.notify();
                                                                }))
                                                                .child("✕"),
                                                        )
                                                    })
                                                    .child(
                                                        div()
                                                            .text_size(px(11.0))
                                                            .text_color(rgb(0x6B808D))
                                                            .child("👤"),
                                                    ),
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
                                            .bg(rgb(0x242F3D))
                                            .border_1()
                                            .border_color(if p_active { rgb(0x2AABEE) } else { rgb(0x242F3D) })
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
                                                        .text_color(rgb(0x6B808D))
                                                        .child(if p_active {
                                                            "| Password (请输入密码)".to_string()
                                                        } else {
                                                            "Password (密码)".to_string()
                                                        })
                                                } else {
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(2.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .text_color(rgb(0xFFFFFF))
                                                                .child("•".repeat(pass_val.chars().count())),
                                                        )
                                                        .when_some(p_preedit, |p_el, p| {
                                                            p_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(0x2AABEE))
                                                                    .border_b_1()
                                                                    .border_color(rgb(0x2AABEE))
                                                                    .child(p),
                                                                )
                                                        })
                                                        .when(p_active, |c_el| {
                                                            c_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(0x2AABEE))
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
                                                                .text_size(px(12.0))
                                                                .text_color(rgb(0x8A9AA8))
                                                                .hover(|s| s.text_color(rgb(0xFFFFFF)))
                                                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                                                                    this.password_input.clear();
                                                                    this.ime_preedit = None;
                                                                    this.active_login_field = LoginInputField::Password;
                                                                    this.focus_handle.focus(window);
                                                                    cx.stop_propagation();
                                                                    cx.notify();
                                                                }))
                                                                .child("✕"),
                                                        )
                                                    })
                                                    .child(
                                                        div()
                                                            .text_size(px(11.0))
                                                            .text_color(rgb(0x6B808D))
                                                            .child("🔒"),
                                                    ),
                                            )
                                    })
                                    // Submit Button
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .w_full()
                                            .h(px(42.0))
                                            .rounded(px(8.0))
                                            .bg(if is_loading { rgb(0x1E96D8) } else { rgb(0x2AABEE) })
                                            .hover(|s| s.bg(rgb(0x1E96D8)))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                if this.is_register_mode {
                                                    this.submit_register(cx);
                                                } else {
                                                    this.submit_login(cx);
                                                }
                                            }))
                                            .child(if is_loading {
                                                "Authenticating with Cloud Server..."
                                            } else if is_reg {
                                                "Create Account & Sign In (注册并登录)"
                                            } else {
                                                "Sign In (登录到云服务器)"
                                            }),
                                    )
                                    // Toggle Mode: Sign In vs Register
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .text_size(px(12.0))
                                            .text_color(rgb(0x2AABEE))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.is_register_mode = !this.is_register_mode;
                                                this.login_error = None;
                                                cx.notify();
                                            }))
                                            .child(if is_reg {
                                                "← Already have an account? Sign In (已有账号？直接登录)"
                                            } else {
                                                "Don't have an account? Register (没有账号？点击注册)"
                                            }),
                                    )
                                    // Quick 1-Click Demo Login Button (Real API call to live backend)
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .w_full()
                                            .h(px(38.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(0x1ABC9C))
                                            .hover(|s| s.bg(rgb(0x16A085)))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(12.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.username_input = "cloud_probe_user".to_string();
                                                this.password_input = "probe_password_123".to_string();
                                                this.submit_login(cx);
                                            }))
                                            .child("⚡ 1-Click Cloud Demo Login (cloud_probe_user)"),
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
                                            .text_color(rgb(0xFFFFFF))
                                            .child(if zh { "使用支付宝扫码登录" } else { "Log in with Alipay QR" }),
                                    )
                                    .child(qr_card)
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(if is_loading { rgb(0x2AABEE) } else { rgb(0x8A9AA8) })
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
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.open_alipay_web_auth(cx);
                                            }))
                                            .child(if zh { "🌐 在浏览器打开支付宝授权" } else { "🌐 Open Alipay Authorization in Browser" }),
                                    )
                                    // Refresh QR Code Button
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .w_full()
                                            .h(px(36.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(0x242F3D))
                                            .hover(|s| s.bg(rgb(0x2B5278)))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(12.0))
                                            .text_color(rgb(0x8A9AA8))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.start_alipay_flow(cx);
                                            }))
                                            .child(if zh { "🔄 刷新二维码" } else { "🔄 Refresh QR Code" }),
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
                                            .text_color(rgb(0xFFFFFF))
                                            .child(if zh { "邮箱注册账号" } else { "Register with Email" }),
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
                                                    .bg(rgb(0x242F3D))
                                                    .border_1()
                                                    .border_color(if e_active { rgb(0x2AABEE) } else { rgb(0x242F3D) })
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
                                                                .text_color(rgb(0x6B808D))
                                                                .child(if e_active {
                                                                    "| Email (输入邮箱)".to_string()
                                                                } else {
                                                                    "Email (邮箱地址)".to_string()
                                                                })
                                                        } else {
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .gap(px(2.0))
                                                                .child(
                                                                    div()
                                                                        .text_size(px(13.0))
                                                                        .text_color(rgb(0xFFFFFF))
                                                                        .child(e_val.clone()),
                                                                )
                                                                .when_some(e_preedit, |p_el, p| {
                                                                    p_el.child(
                                                                        div()
                                                                            .text_size(px(13.0))
                                                                            .text_color(rgb(0x2AABEE))
                                                                            .border_b_1()
                                                                            .border_color(rgb(0x2AABEE))
                                                                            .child(p),
                                                                    )
                                                                })
                                                                .when(e_active, |c_el| {
                                                                    c_el.child(
                                                                        div()
                                                                            .text_size(px(13.0))
                                                                            .text_color(rgb(0x2AABEE))
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
                                                    .bg(if cooldown > 0 || is_sending { rgb(0x242F3D) } else { rgb(0x2AABEE) })
                                                    .hover(|s| if cooldown == 0 && !is_sending { s.bg(rgb(0x1E96D8)) } else { s })
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .text_size(px(12.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(if cooldown > 0 || is_sending { rgb(0x8A9AA8) } else { rgb(0xFFFFFF) })
                                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                        this.send_email_code(cx);
                                                    }))
                                                    .child(if is_sending {
                                                        "发送中...".to_string()
                                                    } else if cooldown > 0 {
                                                        format!("重新获取 ({}s)", cooldown)
                                                    } else if zh {
                                                        "获取验证码".to_string()
                                                    } else {
                                                        "Send Code".to_string()
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
                                            .bg(rgb(0x242F3D))
                                            .border_1()
                                            .border_color(if ec_active { rgb(0x2AABEE) } else { rgb(0x242F3D) })
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
                                                        .text_color(rgb(0x6B808D))
                                                        .child(if ec_active {
                                                            "| 6-digit Code (输入6位验证码)".to_string()
                                                        } else {
                                                            "Verification Code (验证码)".to_string()
                                                        })
                                                } else {
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(2.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .text_color(rgb(0xFFFFFF))
                                                                .child(ec_val.clone()),
                                                        )
                                                        .when(ec_active, |c_el| {
                                                            c_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(0x2AABEE))
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
                                            .bg(rgb(0x242F3D))
                                            .border_1()
                                            .border_color(if u_active { rgb(0x2AABEE) } else { rgb(0x242F3D) })
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
                                                        .text_color(rgb(0x6B808D))
                                                        .child(if u_active {
                                                            "| Username (输入用户名)".to_string()
                                                        } else {
                                                            "Username (用户名)".to_string()
                                                        })
                                                } else {
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(2.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .text_color(rgb(0xFFFFFF))
                                                                .child(u_val.clone()),
                                                        )
                                                        .when(u_active, |c_el| {
                                                            c_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(0x2AABEE))
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
                                            .bg(rgb(0x242F3D))
                                            .border_1()
                                            .border_color(if p_active { rgb(0x2AABEE) } else { rgb(0x242F3D) })
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
                                                        .text_color(rgb(0x6B808D))
                                                        .child(if p_active {
                                                            "| Password (输入密码)".to_string()
                                                        } else {
                                                            "Password (密码)".to_string()
                                                        })
                                                } else {
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(2.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .text_color(rgb(0xFFFFFF))
                                                                .child("•".repeat(p_val.chars().count())),
                                                        )
                                                        .when(p_active, |c_el| {
                                                            c_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(0x2AABEE))
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
                                            .bg(rgb(0x242F3D))
                                            .border_1()
                                            .border_color(if fn_active { rgb(0x2AABEE) } else { rgb(0x242F3D) })
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
                                                        .text_color(rgb(0x6B808D))
                                                        .child(if fn_active {
                                                            "| Display Name (输入昵称，可选)".to_string()
                                                        } else {
                                                            "Display Name (显示昵称，可选)".to_string()
                                                        })
                                                } else {
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(px(2.0))
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .text_color(rgb(0xFFFFFF))
                                                                .child(fn_val.clone()),
                                                        )
                                                        .when(fn_active, |c_el| {
                                                            c_el.child(
                                                                div()
                                                                    .text_size(px(13.0))
                                                                    .text_color(rgb(0x2AABEE))
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
                                            .bg(if is_loading { rgb(0x1E96D8) } else { rgb(0x2AABEE) })
                                            .hover(|s| s.bg(rgb(0x1E96D8)))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.submit_email_register(cx);
                                            }))
                                            .child(if is_loading {
                                                "Authenticating..."
                                            } else if zh {
                                                "立即注册并登录"
                                            } else {
                                                "Register & Sign In"
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
                            .text_color(rgb(0xFFFFFF))
                            .child(if zh { "输入您的手机号码" } else { "Your Phone Number" }),
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
                                    .bg(if is_sel { rgb(0x2AABEE) } else { rgb(0x242F3D) })
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
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
                            .bg(rgb(0x242F3D))
                            .border_1()
                            .border_color(rgb(0x2AABEE))
                            .px(px(12.0))
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(format!("{} {}", country, if phone.is_empty() { "___ ____ ____" } else { &phone })),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .text_size(px(12.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.phone_number.clear();
                                        cx.notify();
                                    }))
                                    .child("✕"),
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
                                    .bg(rgb(0x242F3D))
                                    .hover(|s| s.bg(rgb(0x2B5278)))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
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
                                    .bg(rgb(0x242F3D))
                                    .hover(|s| s.bg(rgb(0x2B5278)))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(13.0))
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.login_step = LoginWizardStep::Start;
                                        cx.notify();
                                    }))
                                    .child(if zh { "← 返回" } else { "← Back" }),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .flex_1()
                                    .h(px(42.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(0x2AABEE))
                                    .hover(|s| s.bg(rgb(0x1E96D8)))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.login_step = LoginWizardStep::Code;
                                        cx.notify();
                                    }))
                                    .child(if zh { "下一步 (获取验证码) →" } else { "NEXT →" }),
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
                            .text_color(rgb(0xFFFFFF))
                            .child(if zh { "使用 Telegram 扫码登录" } else { "Scan QR Code with Telegram" }),
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
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.login_step = LoginWizardStep::Start;
                                this.switch_auth_tab(AuthWizardTab::AlipayQr, cx);
                            }))
                            .child(if zh { "💳 切换至支付宝真实扫码登录" } else { "💳 Switch to Alipay QR Login" }),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(13.0))
                            .text_color(rgb(0x2AABEE))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.login_step = LoginWizardStep::Phone;
                                cx.notify();
                            }))
                            .child(if zh { "改用手机号登录" } else { "Log in by phone number" }),
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
                            .text_color(rgb(0xFFFFFF))
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
                            .bg(rgb(0x242F3D))
                            .border_1()
                            .border_color(rgb(0x2AABEE))
                            .px(px(14.0))
                            .flex()
                            .items_center()
                            .justify_between()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                if this.verification_code.is_empty() {
                                    this.verification_code = "12345".to_string();
                                    cx.notify();
                                } else {
                                    this.submit_login(cx);
                                }
                            }))
                            .child(
                                div()
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(if code.is_empty() { "• • • • •".to_string() } else { code.clone() }),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .text_size(px(12.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.verification_code.clear();
                                        cx.notify();
                                    }))
                                    .child("✕"),
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
                                    .bg(rgb(0x242F3D))
                                    .hover(|s| s.bg(rgb(0x2B5278)))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
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
                    // Auto-Fill Code & Sign In button
                    .child(
                        div()
                            .cursor_pointer()
                            .w_full()
                            .h(px(44.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x2AABEE))
                            .hover(|s| s.bg(rgb(0x1E96D8)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.verification_code = "12345".to_string();
                                this.submit_login(cx);
                            }))
                            .child(if zh {
                                "✅ 自动填入验证码 12345 并登录"
                            } else {
                                "✅ Auto-Fill Code 12345 & Sign In"
                            }),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(12.0))
                            .text_color(rgb(0x8A9AA8))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.login_step = LoginWizardStep::Phone;
                                cx.notify();
                            }))
                            .child(if zh { "← 修改手机号" } else { "← Change Phone Number" }),
                    )
                    .into_any_element(),
            })
    }

    fn render_calls_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let zh = self.is_chinese_locale;
        div()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .overflow_hidden()
            .bg(rgb(0x0E1621))
            .p(px(24.0))
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(if zh { "📞 通话记录" } else { "📞 Recent Calls" }),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(rgb(0x2AABEE))
                            .hover(|s| s.bg(rgb(0x2492CA)))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.active_panel = ActiveWorkspacePanel::Chats;
                                this.last_handled_nav_dest = Some(NavDestination::Chats);
                                this.nav_rail.update(cx, |r, _| {
                                    r.set_active_destination(NavDestination::Chats);
                                });
                                cx.notify();
                            }))
                            .child(if zh { "← 返回对话" } else { "← Back to Chats" }),
                    ),
            )
            // Authentic Centered Empty State matching Telegram Desktop calls_box_controller.cpp
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .w(px(72.0))
                            .h(px(72.0))
                            .rounded_full()
                            .bg(rgb(0x2B5278))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(32.0))
                            .child("📞"),
                    )
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(if zh { "暂无通话记录" } else { "No recent calls" }),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(if zh {
                                "您发起或接收的通话将显示在这里。"
                            } else {
                                "Calls you make or receive will appear here."
                            }),
                    ),
            )
    }

    fn render_task_center_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .overflow_hidden()
            .bg(rgb(0x0E1621))
            .p(px(24.0))
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child("⚡ Autonomous Agent Task Center (CAP-02)"),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(rgb(0x2AABEE))
                            .hover(|s| s.bg(rgb(0x2492CA)))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.active_panel = ActiveWorkspacePanel::Chats;
                                this.last_handled_nav_dest = Some(NavDestination::Chats);
                                this.nav_rail.update(cx, |r, _| {
                                    r.set_active_destination(NavDestination::Chats);
                                });
                                cx.notify();
                            }))
                            .child("← Back to Chats"),
                    ),
            )
            .children([
                ("Task #101 • 13-Module Upstream C++ Parity Audit", "COMPLETED • 3,441 / 3,441 files verified", 0x4CAF50, false),
                ("Task #102 • Local SQLite WAL + FTS5 + Tantivy Indexer", "RUNNING • 0ms query latency", 0x2AABEE, false),
                ("Task #103 • WebRTC Full-Duplex Talk Mode Audio Pipeline", "READY • 18ms P2P RTT", 0x1ABC9C, true),
            ].into_iter().map(|(title, status, color, is_talk_task)| {
                div()
                    .cursor_pointer()
                    .w_full()
                    .p(px(16.0))
                    .rounded(px(10.0))
                    .bg(rgb(0x17212B))
                    .border_1()
                    .border_color(rgb(0x242F3D))
                    .hover(|s| s.border_color(rgb(0x2AABEE)).bg(rgb(0x202B36)))
                    .flex()
                    .items_center()
                    .justify_between()
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                        if is_talk_task {
                            this.show_talk_mode(cx);
                        } else {
                            this.show_remote_computer(cx);
                        }
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(title),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(color))
                            .child(status),
                    )
            }))
    }

    fn render_mcp_tools_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .overflow_hidden()
            .bg(rgb(0x0E1621))
            .p(px(24.0))
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child("🔧 MCP Tools & Permission Dock (CAP-03)"),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(rgb(0x2AABEE))
                            .hover(|s| s.bg(rgb(0x2492CA)))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.active_panel = ActiveWorkspacePanel::Chats;
                                this.last_handled_nav_dest = Some(NavDestination::Chats);
                                this.nav_rail.update(cx, |r, _| {
                                    r.set_active_destination(NavDestination::Chats);
                                });
                                cx.notify();
                            }))
                            .child("← Back to Chats"),
                    ),
            )
            .children([
                ("filesystem.read_workspace", "Allowed (Read-Only Sandbox)", 0x4CAF50),
                ("terminal.execute_command", "Requires Approval per Command", 0xE67E22),
                ("remote_computer.webrtc_stream", "Encrypted E2EE Tunnel Active", 0x2AABEE),
            ].into_iter().map(|(tool, policy, color)| {
                div()
                    .cursor_pointer()
                    .w_full()
                    .p(px(16.0))
                    .rounded(px(10.0))
                    .bg(rgb(0x17212B))
                    .border_1()
                    .border_color(rgb(0x242F3D))
                    .hover(|s| s.border_color(rgb(0x2AABEE)).bg(rgb(0x202B36)))
                    .flex()
                    .items_center()
                    .justify_between()
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                        this.show_remote_computer(cx);
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(tool),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(color))
                            .child(policy),
                    )
            }))
    }

    pub fn navigate_settings_to(&mut self, target: SettingsSectionType, cx: &mut Context<Self>) {
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
        cx.notify();
    }

    fn render_language_settings(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_zh = self.is_chinese_locale;
        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .p(px(24.0))
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child(if is_zh { "🌐 界面语言设置" } else { "🌐 Interface Language Settings" }),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0x8A9AA8))
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
                    .child(
                        // Chinese Option
                        div()
                            .cursor_pointer()
                            .p(px(16.0))
                            .rounded(px(10.0))
                            .bg(rgb(0x17212B))
                            .border_1()
                            .border_color(if is_zh { rgb(0x2AABEE) } else { rgb(0x242F3D) })
                            .hover(|s| s.bg(rgb(0x202B36)))
                            .flex()
                            .items_center()
                            .justify_between()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.update_preferences(cx, |p| {
                                    p.localization.language_code = "zh-CN".to_string();
                                });
                            }))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(12.0))
                                    .child(div().text_size(px(22.0)).child("🇨🇳"))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_size(px(15.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0xFFFFFF))
                                                    .child("简体中文 (Simplified Chinese)"),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(rgb(0x8A9AA8))
                                                    .child("zh-CN • 官方翻译完整支持"),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(if is_zh { rgb(0x2AABEE) } else { rgb(0x56626E) })
                                    .child(if is_zh { "✓ 已启用" } else { "选择" }),
                            ),
                    )
                    .child(
                        // English Option
                        div()
                            .cursor_pointer()
                            .p(px(16.0))
                            .rounded(px(10.0))
                            .bg(rgb(0x17212B))
                            .border_1()
                            .border_color(if !is_zh { rgb(0x2AABEE) } else { rgb(0x242F3D) })
                            .hover(|s| s.bg(rgb(0x202B36)))
                            .flex()
                            .items_center()
                            .justify_between()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.update_preferences(cx, |p| {
                                    p.localization.language_code = "en-US".to_string();
                                });
                            }))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(12.0))
                                    .child(div().text_size(px(22.0)).child("🇺🇸"))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_size(px(15.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0xFFFFFF))
                                                    .child("English (US)"),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(12.0))
                                                    .text_color(rgb(0x8A9AA8))
                                                    .child("en-US • Default Telegram Desktop English"),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(if !is_zh { rgb(0x2AABEE) } else { rgb(0x56626E) })
                                    .child(if !is_zh { "✓ Active" } else { "Select" }),
                            ),
                    ),
            )
    }

    fn render_settings_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_root = self.settings_section.is_root();
        let zh = self.is_chinese_locale;
        let title = self.settings_section.display_title_localized(zh);
        let icon = self.settings_section.icon();

        let back_label = if is_root {
            if zh { "← 返回对话".to_string() } else { "← Back to Chats".to_string() }
        } else if let Some(&prev) = self.settings_stack.last() {
            if zh {
                format!("← 返回 {}", prev.display_title_localized(true))
            } else {
                format!("← {}", prev.display_title_localized(false))
            }
        } else if zh {
            "← 返回".to_string()
        } else {
            "← Back".to_string()
        };

        let crumbs = self.settings_section.breadcrumb_path();

        div()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .overflow_hidden()
            .bg(rgb(0x0E1621))
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
                    .bg(rgb(0x17212B))
                    .border_b_1()
                    .border_color(rgb(0x1F2A38))
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
                                    .bg(if is_root { rgb(0x2AABEE) } else { rgb(0x232E3C) })
                                    .hover(|s| s.bg(if is_root { rgb(0x2492CA) } else { rgb(0x2B3846) }))
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                        this.navigate_settings_back(cx);
                                    }))
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
                                                                    .text_color(rgb(0x6C7883))
                                                                    .child("›"),
                                                            )
                                                        })
                                                        .child(
                                                            div()
                                                                .text_size(px(11.0))
                                                                .text_color(if is_current {
                                                                    rgb(0x2AABEE)
                                                                } else {
                                                                    rgb(0x8A9AA8)
                                                                })
                                                                .cursor_pointer()
                                                                .hover(|s| s.text_color(rgb(0xFFFFFF)))
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
                                            .child(div().text_size(px(18.0)).child(icon))
                                            .child(
                                                div()
                                                    .text_size(px(16.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0xFFFFFF))
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
                                    .bg(rgb(0x232E3C))
                                    .hover(|s| s.bg(rgb(0x2B3846)))
                                    .text_size(px(13.0))
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                        this.navigate_settings_to(SettingsSectionType::Search, cx);
                                    }))
                                    .child(if zh { "🔍 搜索" } else { "🔍 Search" }),
                            )
                            .child(
                                // Language Toggle
                                div()
                                    .cursor_pointer()
                                    .px(px(10.0))
                                    .py(px(6.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(0x232E3C))
                                    .hover(|s| s.bg(rgb(0x2B3846)))
                                    .text_size(px(13.0))
                                    .text_color(rgb(0x2AABEE))
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
                                    .child(if zh { "🌐 中文" } else { "🌐 English" }),
                            )
                            .child(
                                // Close Button
                                div()
                                    .cursor_pointer()
                                    .px(px(10.0))
                                    .py(px(6.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(0x232E3C))
                                    .hover(|s| s.bg(rgb(0xE53935)))
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                        this.close_settings(cx);
                                    }))
                                    .child(if zh { "✕ 关闭" } else { "✕ Close" }),
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
