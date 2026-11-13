//! ContextInspectorDrawer 320px Right Drawer (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.5).
//!
//! Sub-components:
//! 1. Drawer Header (Title, Close Button).
//! 2. Four Main Tabs:
//!    - Profile Tab (80px avatar, info, notifications switch, actions)
//!    - Members Tab (member count, admin badges, add action)
//!    - Shared Media Tab (Media, Files, Audio, Links sub-tabs)
//!    - Settings Tab (E2EE fingerprint, auto-delete, sounds)

use gpui::*;
use crate::views::drawer::inspector_tabs::*;

pub const DRAWER_WIDTH: f32 = 320.0;
pub const CONTEXT_INSPECTOR_WIDTH: f32 = DRAWER_WIDTH;
pub const HEADER_HEIGHT: f32 = 54.0;
pub const CONTEXT_INSPECTOR_HEADER_HEIGHT: f32 = HEADER_HEIGHT;
pub const AVATAR_SIZE: f32 = 80.0;
pub const CONTEXT_INSPECTOR_AVATAR_SIZE: f32 = AVATAR_SIZE;

pub struct ContextInspector {
    pub is_open: bool,
    pub active_tab: InspectorTab,
    pub profile: ProfileTabModel,
    pub members: MembersTabModel,
    pub shared_media: SharedMediaTabModel,
    pub settings: SettingsTabModel,
    pub is_secret_chat_active: bool,
    pub is_contact_blocked: bool,
    pub pending_media_open: Option<(String, bool)>,
}

impl ContextInspector {
    pub fn new() -> Self {
        let profile = ProfileTabModel {
            name: String::new(),
            username: String::new(),
            phone: String::new(),
            bio: String::new(),
            avatar_initials: String::new(),
            avatar_color: 0,
            is_notifications_enabled: true,
        };

        let members = MembersTabModel {
            members: Vec::new(),
            search_query: String::new(),
        };

        let shared_media = SharedMediaTabModel {
            active_subtab: SharedMediaSubTab::Media,
            media_count: 0,
            files: Vec::new(),
            links: Vec::new(),
        };

        let settings = SettingsTabModel {
            e2ee_fingerprint: String::new(),
            auto_delete_period: "Off".to_string(),
            notification_sound: "Default (Aurora)".to_string(),
        };

        Self {
            is_open: false,
            active_tab: InspectorTab::Profile,
            profile,
            members,
            shared_media,
            settings,
            is_secret_chat_active: false,
            is_contact_blocked: false,
            pending_media_open: None,
        }
    }

    pub fn new_demo() -> Self {
        let profile = ProfileTabModel {
            name: "Alice Smith".to_string(),
            username: "@alice_smith".to_string(),
            phone: "+1 (555) 019-2834".to_string(),
            bio: "Rust & Systems engineer. Building GPUI Telegram 1:1 desktop client.".to_string(),
            avatar_initials: "AS".to_string(),
            avatar_color: 0x2AABEE,
            is_notifications_enabled: true,
        };

        let members = MembersTabModel {
            members: vec![
                MemberItem {
                    id: 1,
                    name: "Alice Smith".to_string(),
                    initials: "AS".to_string(),
                    color: 0x2AABEE,
                    role: "Owner".to_string(),
                    is_online: true,
                },
                MemberItem {
                    id: 2,
                    name: "Bob Builder".to_string(),
                    initials: "BB".to_string(),
                    color: 0x9B59B6,
                    role: "Admin".to_string(),
                    is_online: false,
                },
                MemberItem {
                    id: 3,
                    name: "Autonomous Agent Bot".to_string(),
                    initials: "🤖".to_string(),
                    color: 0x1ABC9C,
                    role: "Bot".to_string(),
                    is_online: true,
                },
            ],
            search_query: String::new(),
        };

        let shared_media = SharedMediaTabModel {
            active_subtab: SharedMediaSubTab::Media,
            media_count: 48,
            files: vec![
                SharedFileItem {
                    name: "gpui_telegram_spec_v2.pdf".to_string(),
                    size_text: "4.2 MB".to_string(),
                    extension: "PDF".to_string(),
                    date: "Today at 14:10".to_string(),
                },
                SharedFileItem {
                    name: "desktop_views_mockup.fig".to_string(),
                    size_text: "18.9 MB".to_string(),
                    extension: "FIG".to_string(),
                    date: "Yesterday".to_string(),
                },
            ],
            links: vec![
                SharedLinkItem {
                    title: "GPUI Documentation & High-Performance UI Guide".to_string(),
                    url: "https://gpui.rs/docs".to_string(),
                    domain: "gpui.rs".to_string(),
                },
                SharedLinkItem {
                    title: "Telegram Desktop Source Repository".to_string(),
                    url: "https://github.com/telegramdesktop/tdesktop".to_string(),
                    domain: "github.com".to_string(),
                },
            ],
        };

        let settings = SettingsTabModel {
            e2ee_fingerprint: "🔐 🌟 🚀 ⚡ 🛡️".to_string(),
            auto_delete_period: "Off".to_string(),
            notification_sound: "Default (Aurora)".to_string(),
        };

        Self {
            is_open: true,
            active_tab: InspectorTab::Profile,
            profile,
            members,
            shared_media,
            settings,
            is_secret_chat_active: false,
            is_contact_blocked: false,
            pending_media_open: None,
        }
    }

    fn clear_peer_facts(&mut self) {
        self.profile = ProfileTabModel {
            name: String::new(),
            username: String::new(),
            phone: String::new(),
            bio: String::new(),
            avatar_initials: String::new(),
            avatar_color: 0,
            is_notifications_enabled: true,
        };
        self.members = MembersTabModel {
            members: Vec::new(),
            search_query: String::new(),
        };
        self.shared_media = SharedMediaTabModel {
            active_subtab: SharedMediaSubTab::Media,
            media_count: 0,
            files: Vec::new(),
            links: Vec::new(),
        };
        self.settings = SettingsTabModel {
            e2ee_fingerprint: String::new(),
            auto_delete_period: "Off".to_string(),
            notification_sound: "Default (Aurora)".to_string(),
        };
        self.is_secret_chat_active = false;
        self.is_contact_blocked = false;
        self.pending_media_open = None;
    }

    pub fn clear_peer(&mut self) {
        self.is_open = false;
        self.active_tab = InspectorTab::Profile;
        self.clear_peer_facts();
    }

    pub fn configure_for_cloud_workspace(&mut self, _server_url: &str) {
        self.clear_peer();
    }

    pub fn set_peer(&mut self, name: &str, subtitle: &str, initials: &str, color: u32) {
        if name.trim().is_empty() {
            self.clear_peer();
            return;
        }

        let is_same_peer = !self.profile.name.is_empty()
            && self.profile.name == name
            && self.profile.avatar_initials == initials
            && self.profile.avatar_color == color;

        if !is_same_peer {
            self.clear_peer_facts();
        }

        self.profile.name = name.to_string();
        self.profile.bio = subtitle.to_string();
        self.profile.avatar_initials = initials.to_string();
        self.profile.avatar_color = color;
    }

    pub fn set_tab(&mut self, tab: InspectorTab) {
        self.active_tab = tab;
    }

    pub fn toggle_open(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn set_auto_delete_period(&mut self, period: impl Into<String>) {
        self.settings.auto_delete_period = period.into();
    }
}

impl Default for ContextInspector {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ContextInspector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            return div().w(px(0.0)).h_full().overflow_hidden().into_any_element();
        }

        let active_tab = self.active_tab;
        let notif_enabled = self.profile.is_notifications_enabled;
        let auto_del = self.settings.auto_delete_period.clone();

        div()
            .w(px(DRAWER_WIDTH)) // Fixed 320px width per spec
            .flex_shrink_0()
            .h_full()
            .overflow_hidden()
            .bg(rgb(0x17212B)) // Upstream inspector background
            .border_l_1()
            .border_color(rgb(0x0E1621))
            .flex()
            .flex_col()
            // 1. Header (height 54px per upstream info.style:41 & SPEC-02)
            .child(
                div()
                    .h(px(HEADER_HEIGHT))
                    .flex_shrink_0()
                    .w_full()
                    .px(px(16.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(rgb(0x0E1621))
                    .child(
                        div()
                            .text_size(px(15.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(match active_tab {
                                InspectorTab::Profile => "User Info",
                                InspectorTab::Members => "Chat Members",
                                InspectorTab::SharedMedia => "Shared Content",
                                InspectorTab::Settings => "Chat Settings",
                            }),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded(px(4.0))
                            .hover(|s| s.bg(rgb(0x242F3D)))
                            .text_size(px(16.0))
                            .text_color(rgb(0x8A9AA8))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.close();
                                cx.notify();
                            }))
                            .child("✕"),
                    ),
            )
            // 2. Tab Navigation Strip (Profile, Members, Shared, Settings)
            .child(
                div()
                    .h(px(40.0))
                    .flex_shrink_0()
                    .w_full()
                    .px(px(8.0))
                    .flex()
                    .items_center()
                    .justify_around()
                    .border_b_1()
                    .border_color(rgb(0x0E1621))
                    .child(Self::render_tab_button("Profile", InspectorTab::Profile, active_tab, cx))
                    .child(Self::render_tab_button("Members", InspectorTab::Members, active_tab, cx))
                    .child(Self::render_tab_button("Media", InspectorTab::SharedMedia, active_tab, cx))
                    .child(Self::render_tab_button("Settings", InspectorTab::Settings, active_tab, cx)),
            )
            // 3. Tab Body
            .child(
                div()
                    .id("inspector-tab-scroll")
                    .flex_1()
                    .min_h(px(0.0))
                    .w_full()
                    .p(px(16.0))
                    .overflow_y_scroll()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(match active_tab {
                        InspectorTab::Profile => InspectorTabsView::render_profile(
                            &self.profile,
                            self.is_secret_chat_active,
                            self.is_contact_blocked,
                            cx,
                        ).into_any_element(),
                        InspectorTab::Members => InspectorTabsView::render_members(&self.members, cx).into_any_element(),
                        InspectorTab::SharedMedia => InspectorTabsView::render_shared_media(&self.shared_media, cx).into_any_element(),
                        InspectorTab::Settings => InspectorTabsView::render_settings(&self.settings, cx).into_any_element(),
                    })
                    .child(
                        div()
                            .pt(px(8.0))
                            .border_t_1()
                            .border_color(rgb(0x242F3D))
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .cursor_pointer()
                                    .w_full()
                                    .py(px(8.0))
                                    .px(px(12.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(0x242F3D))
                                    .hover(|s| s.bg(rgb(0x2B5278)))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.profile.is_notifications_enabled = !this.profile.is_notifications_enabled;
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .child("🔔 Notifications"),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(if notif_enabled { rgb(0x4CAF50) } else { rgb(0xE53935) })
                                            .child(if notif_enabled { "ON" } else { "MUTED" }),
                                    ),
                            )
                            .child(
                                div()
                                    .cursor_pointer()
                                    .w_full()
                                    .py(px(8.0))
                                    .px(px(12.0))
                                    .rounded(px(8.0))
                                    .bg(rgb(0x242F3D))
                                    .hover(|s| s.bg(rgb(0x2B5278)))
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        let next = match this.settings.auto_delete_period.as_str() {
                                            "Off" => "24 Hours",
                                            "24 Hours" => "7 Days",
                                            _ => "Off",
                                        };
                                        this.set_auto_delete_period(next);
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .child("⏱️ Auto-Delete Timer"),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(rgb(0x2AABEE))
                                            .child(auto_del),
                                    ),
                            ),
                    ),
            )
            .into_any_element()
    }
}

impl ContextInspector {
    fn render_tab_button(label: &str, tab: InspectorTab, active: InspectorTab, cx: &mut Context<Self>) -> impl IntoElement {
        let is_selected = tab == active;
        div()
            .cursor_pointer()
            .h_full()
            .px(px(8.0))
            .flex()
            .items_center()
            .justify_center()
            .border_b_2()
            .border_color(if is_selected { rgb(0x2AABEE) } else { rgba(0x00000000) })
            .text_size(px(12.0))
            .font_weight(if is_selected { FontWeight::BOLD } else { FontWeight::NORMAL })
            .text_color(if is_selected { rgb(0x2AABEE) } else { rgb(0x8A9AA8) })
            .hover(|s| s.text_color(rgb(0xFFFFFF)))
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                this.set_tab(tab);
                cx.notify();
            }))
            .child(label.to_string())
    }
}
