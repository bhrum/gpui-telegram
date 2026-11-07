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
    CallOverlay, CanonicalComposer, ChatSidebar, ComputerWorkbench, ContextInspector,
    ConversationStage, MediaViewer, ModalContainer, NavRail,
    nav_rail::NavDestination,
    stage::cards::{
        AlbumItem, DeliveryStatus, InlineButtonType, InlineKeyboardButton, MediaType,
        MessageContent, MessageDirection, MessageItemModel, PollOption, PollType,
    },
};
use gpui::*;
use gpui::prelude::FluentBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginWizardStep {
    Start,
    Phone,
    QrCode,
    Code,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveWorkspacePanel {
    Chats,
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
    pub focus_handle: FocusHandle,
    pub next_message_id: u64,
}

impl AppShell {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let nav_rail = cx.new(|_| NavRail::new());
        let sidebar = cx.new(|_| ChatSidebar::new());
        let stage = cx.new(|_| ConversationStage::new());
        let composer = cx.new(|_| CanonicalComposer::new());
        let inspector = cx.new(|_| ContextInspector::new());
        let modal_container = cx.new(|_| ModalContainer::new());
        let focus_handle = cx.focus_handle();

        // 1. Observe NavRail clicks (Destinations, Overlays, Logout)
        cx.observe(&nav_rail, |shell: &mut AppShell, rail: Entity<NavRail>, cx: &mut Context<AppShell>| {
            let (dest, logout_req) = {
                let r = rail.read(cx);
                (r.active_destination, r.logout_requested)
            };
            if logout_req {
                rail.update(cx, |r, _| r.logout_requested = false);
                shell.is_authenticated = false;
                shell.login_step = LoginWizardStep::Start;
                shell.verification_code.clear();
                cx.notify();
                return;
            }
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
                NavDestination::Calls | NavDestination::TalkMode => {
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
                    shell.active_panel = ActiveWorkspacePanel::Settings;
                }
            }
            cx.notify();
        }).detach();

        // 2. Observe ChatSidebar clicks (Dialog selection, Story click, Search focus)
        cx.observe(&sidebar, |shell: &mut AppShell, sb: Entity<ChatSidebar>, cx: &mut Context<AppShell>| {
            let (selected_dialog, story_peer, search_focused) = {
                let s = sb.read(cx);
                (s.selected_dialog().cloned(), s.story_clicked_peer.clone(), s.search_focused)
            };
            if search_focused {
                shell.composer.update(cx, |c, _| c.is_input_focused = false);
            }
            if let Some(peer) = story_peer {
                sb.update(cx, |s, _| s.story_clicked_peer = None);
                shell.show_media_viewer(format!("https://t.me/stories/{}", peer), false, cx);
            }
            if let Some(d) = selected_dialog {
                shell.active_panel = ActiveWorkspacePanel::Chats;
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
                    "View Task Matrix" => shell.active_panel = ActiveWorkspacePanel::TaskCenter,
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

        Self {
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
            is_chinese_locale: true,
            active_panel: ActiveWorkspacePanel::Chats,
            focus_handle,
            next_message_id: 900,
        }
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
                shell.hide_talk_mode();
                cx.notify();
            }
        }).detach();
        self.talk_mode_overlay = Some(overlay);
    }

    pub fn hide_talk_mode(&mut self) {
        self.talk_mode_overlay = None;
    }

    pub fn show_remote_computer(&mut self, cx: &mut Context<Self>) {
        let overlay = cx.new(|_| ComputerWorkbench::new("Dev-Workstation-01".to_string()));
        cx.observe(&overlay, |shell: &mut AppShell, ov: Entity<ComputerWorkbench>, cx: &mut Context<AppShell>| {
            if ov.read(cx).should_close {
                shell.hide_remote_computer();
                cx.notify();
            }
        }).detach();
        self.computer_overlay = Some(overlay);
    }

    pub fn hide_remote_computer(&mut self) {
        self.computer_overlay = None;
    }

    pub fn show_media_viewer(&mut self, url: impl Into<String>, is_video: bool, cx: &mut Context<Self>) {
        let overlay = cx.new(|_| MediaViewer::new(url.into(), is_video));
        cx.observe(&overlay, |shell: &mut AppShell, ov: Entity<MediaViewer>, cx: &mut Context<AppShell>| {
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
        let reply_id = self.next_message_id;
        self.next_message_id += 1;

        let active_chat_id = self.stage.read(cx).active_chat_id.unwrap_or(1);
        let peer_name = self.stage.read(cx).chat_title.clone();
        let peer_initials = self.stage.read(cx).chat_avatar_initials.clone();
        let peer_color = self.stage.read(cx).chat_avatar_color;

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
                content: MessageContent::Text {
                    text: text.clone(),
                    is_edited: false,
                },
                bot_keyboard: None,
                is_pinned: false,
            });

            // Automatic realistic reply from peer / AI agent
            if active_chat_id != 3 {
                st.add_message(MessageItemModel {
                    id: reply_id,
                    sender_id: active_chat_id,
                    sender_name: peer_name.clone(),
                    avatar_initials: peer_initials,
                    avatar_color: peer_color,
                    direction: MessageDirection::Incoming,
                    timestamp: "Just now".to_string(),
                    delivery_status: DeliveryStatus::Read,
                    reply_to: None,
                    reactions: vec![],
                    content: MessageContent::Text {
                        text: format!("Received: \"{}\" — synced via local Rust server (SQLite WAL + MTProto 2.0).", text),
                        is_edited: false,
                    },
                    bot_keyboard: if active_chat_id == 4 {
                        Some(vec![vec![
                            InlineKeyboardButton {
                                text: "Launch Talk Mode".to_string(),
                                button_type: InlineButtonType::Callback("action_talk_mode".to_string()),
                            },
                            InlineKeyboardButton {
                                text: "Remote Computer".to_string(),
                                button_type: InlineButtonType::Callback("action_remote_computer".to_string()),
                            },
                        ]])
                    } else {
                        None
                    },
                    is_pinned: false,
                });
            }
            st_cx.notify();
        });

        self.sidebar.update(cx, |sb, sb_cx| {
            if let Some(d) = sb.dialogs.iter_mut().find(|d| d.id == active_chat_id) {
                d.last_message = format!("You: {}", text);
                d.timestamp = "Just now".to_string();
            }
            sb_cx.notify();
        });
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

    fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
        let key = event.keystroke.key.as_str();

        // Escape closes any active modal or overlay
        if key == "escape" {
            if self.modal_container.read(cx).is_modal_active() {
                self.modal_container.update(cx, |mc, mc_cx| {
                    mc.dismiss_top();
                    mc_cx.notify();
                });
                cx.notify();
                return;
            }
            if self.media_viewer_overlay.is_some() {
                self.hide_media_viewer();
                cx.notify();
                return;
            }
            if self.talk_mode_overlay.is_some() {
                self.hide_talk_mode();
                cx.notify();
                return;
            }
            if self.computer_overlay.is_some() {
                self.hide_remote_computer();
                cx.notify();
                return;
            }
        }

        // If on Login screen, handle keyboard typing for Phone / Verification Code
        if !self.is_authenticated {
            match self.login_step {
                LoginWizardStep::Start | LoginWizardStep::QrCode => {
                    if key == "enter" {
                        self.is_authenticated = true;
                        cx.notify();
                    }
                }
                LoginWizardStep::Phone => {
                    if key == "enter" {
                        if !self.phone_number.trim().is_empty() {
                            self.login_step = LoginWizardStep::Code;
                            self.login_error = None;
                        }
                        cx.notify();
                    } else if key == "backspace" {
                        self.phone_number.pop();
                        cx.notify();
                    } else if let Some(ref ch) = event.keystroke.key_char {
                        if ch.chars().all(|c| c.is_ascii_digit() || c == ' ' || c == '-') {
                            self.phone_number.push_str(ch);
                            cx.notify();
                        }
                    }
                }
                LoginWizardStep::Code => {
                    if key == "enter" {
                        self.is_authenticated = true;
                        cx.notify();
                    } else if key == "backspace" {
                        self.verification_code.pop();
                        cx.notify();
                    } else if let Some(ref ch) = event.keystroke.key_char {
                        if ch.chars().all(|c| c.is_ascii_digit()) && self.verification_code.len() < 6 {
                            self.verification_code.push_str(ch);
                            if self.verification_code.len() >= 5 {
                                self.is_authenticated = true;
                            }
                            cx.notify();
                        }
                    }
                }
            }
            return;
        }

        // Authenticated workspace keyboard routing: Sidebar Search vs Message Composer
        let is_search_focused = self.sidebar.read(cx).search_focused;
        if is_search_focused {
            if key == "backspace" {
                self.sidebar.update(cx, |sb, sb_cx| {
                    let mut q = sb.search_query.clone();
                    q.pop();
                    sb.set_search_query(q);
                    sb_cx.notify();
                });
            } else if key == "enter" || key == "escape" {
                self.sidebar.update(cx, |sb, sb_cx| {
                    sb.search_focused = false;
                    sb_cx.notify();
                });
            } else if let Some(ref ch) = event.keystroke.key_char {
                if !event.keystroke.modifiers.platform && !event.keystroke.modifiers.control {
                    self.sidebar.update(cx, |sb, sb_cx| {
                        let mut q = sb.search_query.clone();
                        q.push_str(ch);
                        sb.set_search_query(q);
                        sb_cx.notify();
                    });
                }
            }
            return;
        }

        // Default: route keyboard typing to CanonicalComposer
        if key == "enter" {
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
        } else if key == "backspace" {
            self.composer.update(cx, |c, c_cx| {
                let mut t = c.text().to_string();
                t.pop();
                c.set_text(t);
                c_cx.notify();
            });
        } else if key == "space" {
            self.composer.update(cx, |c, c_cx| {
                c.ime_machine.insert_text(" ");
                c_cx.notify();
            });
        } else if let Some(ref ch) = event.keystroke.key_char {
            if !event.keystroke.modifiers.platform && !event.keystroke.modifiers.control {
                self.composer.update(cx, |c, c_cx| {
                    c.ime_machine.insert_text(ch);
                    c_cx.notify();
                });
            }
        }
    }
}

impl Render for AppShell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.focus_handle.is_focused(window) {
            self.focus_handle.focus(window);
        }

        if !self.is_authenticated {
            return div()
                .id("app-shell-login-root")
                .track_focus(&self.focus_handle)
                .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                    this.handle_key_down(event, cx);
                }))
                .size_full()
                .bg(rgb(0x0E1621))
                .flex()
                .items_center()
                .justify_center()
                .child(self.render_login_wizard(cx))
                .into_any_element();
        }

        div()
            .id("app-shell-authenticated-root")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                this.handle_key_down(event, cx);
            }))
            .relative()
            .size_full()
            .flex()
            .bg(rgb(0x0E1621))
            // 1. Left Primary Nav Rail (52px)
            .child(self.nav_rail.clone())
            // 2. Chat Sidebar (288px)
            .child(self.sidebar.clone())
            // 3. Main Stage + Composer Column (or TaskCenter / McpTools / Settings Panel)
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
                LoginWizardStep::Start => div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(14.0))
                    .child(
                        div()
                            .text_size(px(22.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(if zh { "Telegram Desktop (GPUI 原生版)" } else { "Telegram Desktop" }),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(if zh {
                                "基于 Rust + GPUI 120fps 架构与独立自托管服务端"
                            } else {
                                "Welcome to the official GPUI Telegram Desktop client."
                            }),
                    )
                    // Primary Button: Start Messaging (Phone Login)
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
                                this.login_step = LoginWizardStep::Phone;
                                cx.notify();
                            }))
                            .child(if zh { "📱 手机号登录 (START MESSAGING)" } else { "START MESSAGING" }),
                    )
                    // Secondary Button: QR Code Login
                    .child(
                        div()
                            .cursor_pointer()
                            .w_full()
                            .h(px(40.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x242F3D))
                            .hover(|s| s.bg(rgb(0x2B5278)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x2AABEE))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.login_step = LoginWizardStep::QrCode;
                                cx.notify();
                            }))
                            .child(if zh { "📷 扫码登录 (Log in by QR Code)" } else { "Log in by QR Code" }),
                    )
                    // Instant 1-Click Demo Login Button
                    .child(
                        div()
                            .cursor_pointer()
                            .w_full()
                            .h(px(42.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x1ABC9C))
                            .hover(|s| s.bg(rgb(0x16A085)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.is_authenticated = true;
                                cx.notify();
                            }))
                            .child(if zh {
                                "⚡ 一键体验登录 (直接进入主界面)"
                            } else {
                                "⚡ Instant Demo Sign In (1-Click)"
                            }),
                    )
                    .into_any_element(),

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
                    // Simulated QR Box (Clickable to simulate scan)
                    .child(
                        div()
                            .cursor_pointer()
                            .w(px(160.0))
                            .h(px(160.0))
                            .rounded(px(12.0))
                            .bg(rgb(0xFFFFFF))
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .gap(px(6.0))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.is_authenticated = true;
                                cx.notify();
                            }))
                            .child(div().text_size(px(44.0)).child("🔳"))
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x17212B))
                                    .child(if zh { "点击二维码模拟扫码成功" } else { "Click QR to Simulate Scan" }),
                            ),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .w_full()
                            .h(px(42.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x1ABC9C))
                            .hover(|s| s.bg(rgb(0x16A085)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.is_authenticated = true;
                                cx.notify();
                            }))
                            .child(if zh { "📱 模拟手机确认登录" } else { "📱 Simulate Phone Scan & Sign In" }),
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
                                } else {
                                    this.is_authenticated = true;
                                }
                                cx.notify();
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
                                                this.is_authenticated = true;
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
                                this.is_authenticated = true;
                                cx.notify();
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
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.active_panel = ActiveWorkspacePanel::Chats;
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
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.active_panel = ActiveWorkspacePanel::Chats;
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

    fn render_settings_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
                            .child("⚙️ Telegram Desktop Settings & Account"),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(rgb(0x2AABEE))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.active_panel = ActiveWorkspacePanel::Chats;
                                cx.notify();
                            }))
                            .child("← Back to Chats"),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .p(px(16.0))
                    .rounded(px(10.0))
                    .bg(rgb(0x17212B))
                    .border_1()
                    .border_color(rgb(0x242F3D))
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(format!("Signed in Phone: {} {}", self.country_code, self.phone_number)),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .child("Server: http://127.0.0.1:8080 • Session: Authenticated"),
                            ),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(14.0))
                            .py(px(8.0))
                            .rounded(px(6.0))
                            .bg(rgb(0xE53935))
                            .hover(|s| s.bg(rgb(0xC62828)))
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.is_authenticated = false;
                                this.login_step = LoginWizardStep::Start;
                                this.verification_code.clear();
                                cx.notify();
                            }))
                            .child("🚪 Log Out (退出登录)"),
                    ),
            )
            .child(
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
                        this.is_chinese_locale = !this.is_chinese_locale;
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child("🌐 Interface Language (界面语言切换)"),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x2AABEE))
                            .child(if self.is_chinese_locale { "中文 (Simplified Chinese)" } else { "English (US)" }),
                    ),
            )
    }
}
