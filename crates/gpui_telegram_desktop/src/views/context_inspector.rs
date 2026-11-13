//! ContextInspectorDrawer 320px Right Drawer (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.5).
//!
//! Full 1:1 Telegram Desktop (`tdesktop`) Right Info Panel (`info/*`):
//! - Profile Tab (Avatar, Name, @username, Phone, Bio, Notifications switch, Send Gift 🎁,
//!   Export Chat 📤, Secret Chat E2EE 🔐, Block Contact)
//! - Members Tab (Member count, Owner/Admin/Bot badges, + Add Member)
//! - Shared Media Tab (Media, Files, Voice, Links sub-tabs)
//! - Settings & Analytics Tab (333-Emoji E2EE Key Fingerprint, Auto-Delete Timer,
//!   Notification Sound, plus Channel Growth & Boost Level 8 Progress Analytics!)

use crate::views::drawer::inspector_tabs::*;
use crate::views::tdesktop_theme::TdesktopThemePalette;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_webrtc::CallsEmojiFingerprint;

#[derive(Debug, Clone)]
pub enum InspectorQuickAction {
    OpenStarGift,
    OpenExportChat,
    StartAudioCall,
}

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

    // 1:1 Extended Channel Statistics & Boosts + Theme
    pub channel_boost_level: u32,
    pub channel_boosts_current: u32,
    pub channel_boosts_target: u32,
    pub pending_quick_action: Option<InspectorQuickAction>,
    pub palette: TdesktopThemePalette,
}

impl ContextInspector {
    pub fn new() -> Self {
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

        let fp = CallsEmojiFingerprint::from_sha256([
            0x4A, 0x91, 0x2C, 0x8F, 0x11, 0x7E, 0x33, 0xD2, 0x9B, 0x05, 0x64, 0xE1, 0x77, 0x19,
            0xA8, 0x5C, 0x3E, 0x82, 0xF0, 0x14, 0x6D, 0xB3, 0x29, 0x71, 0xC6, 0x48, 0x95, 0x0E,
            0x52, 0xDF, 0x83, 0x1A,
        ]);

        let settings = SettingsTabModel {
            e2ee_fingerprint: fp.as_emoji_string(),
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
            channel_boost_level: 8,
            channel_boosts_current: 34,
            channel_boosts_target: 40,
            pending_quick_action: None,
            palette: TdesktopThemePalette::default(),
        }
    }

    pub fn set_peer(&mut self, name: &str, subtitle: &str, initials: &str, color: u32) {
        self.profile.name = name.to_string();
        let slug = name
            .to_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect::<String>();
        self.profile.username = format!("@{}", slug.trim_matches('_'));
        self.profile.bio = format!("Status: {} • GPUI Telegram Desktop", subtitle);
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

        let pal = self.palette;
        let active_tab = self.active_tab;
        let boost_lvl = self.channel_boost_level;
        let boost_cur = self.channel_boosts_current;
        let boost_tgt = self.channel_boosts_target;

        div()
            .w(px(320.0))
            .flex_shrink_0()
            .h_full()
            .overflow_hidden()
            .bg(rgb(pal.dialogs_bg))
            .border_l_1()
            .border_color(rgb(pal.border_fg))
            .flex()
            .flex_col()
            // 1. Header (52px)
            .child(
                div()
                    .h(px(52.0))
                    .flex_shrink_0()
                    .w_full()
                    .px(px(16.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(rgb(pal.border_fg))
                    .child(
                        div()
                            .text_size(px(15.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.text_primary))
                            .child(match active_tab {
                                InspectorTab::Profile => "Profile Info",
                                InspectorTab::Members => "Members & Admins",
                                InspectorTab::SharedMedia => "Shared Media",
                                InspectorTab::Settings => "Security & Stats",
                            }),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded(px(4.0))
                            .hover(move |s| s.bg(rgb(pal.input_bg)))
                            .text_size(px(15.0))
                            .text_color(rgb(pal.text_secondary))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _event, _window, cx| {
                                    this.close();
                                    cx.notify();
                                }),
                            )
                            .child("✕"),
                    ),
            )
            // 2. Tab Navigation Strip (Profile, Members, Media, Stats/Settings)
            .child(
                div()
                    .h(px(40.0))
                    .flex_shrink_0()
                    .w_full()
                    .px(px(8.0))
                    .border_b_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .items_center()
                    .justify_between()
                    .children(
                        [
                            (InspectorTab::Profile, "资料"),
                            (InspectorTab::SharedMedia, "媒体"),
                            (InspectorTab::Members, "成员"),
                            (InspectorTab::Settings, "统计/加密"),
                        ]
                        .into_iter()
                        .map(|(tab, label)| {
                            let is_sel = active_tab == tab;
                            div()
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(5.0))
                                .rounded(px(6.0))
                                .bg(if is_sel {
                                    rgb(pal.accent)
                                } else {
                                    rgba(0x00000000)
                                })
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(if is_sel {
                                    rgb(0xFFFFFF)
                                } else {
                                    rgb(pal.text_secondary)
                                })
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _event, _window, cx| {
                                        this.set_tab(tab);
                                        cx.notify();
                                    }),
                                )
                                .child(label)
                        }),
                    ),
            )
            // 3. Scrollable Tab Body + 1:1 Telegram Action Shortcuts
            .child(
                div()
                    .id("inspector-drawer-body-scroll")
                    .flex_1()
                    .min_h(px(0.0))
                    .w_full()
                    .p(px(14.0))
                    .overflow_y_scroll()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(match active_tab {
                        InspectorTab::Profile => div()
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .child(InspectorTabsView::render_profile(
                                &self.profile,
                                self.is_secret_chat_active,
                                self.is_contact_blocked,
                                cx,
                            ))
                            // Extra 1:1 Quick Action Buttons: Gift, Call, Export
                            .child(
                                div()
                                    .flex()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .flex_1()
                                            .py(px(7.0))
                                            .rounded(px(8.0))
                                            .bg(rgba(0xF1C40F22))
                                            .flex()
                                            .justify_center()
                                            .text_size(px(11.5))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xF1C40F))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _e, _w, cx| {
                                                    this.pending_quick_action =
                                                        Some(InspectorQuickAction::OpenStarGift);
                                                    cx.notify();
                                                }),
                                            )
                                            .child("🎁 赠送礼物"),
                                    )
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .flex_1()
                                            .py(px(7.0))
                                            .rounded(px(8.0))
                                            .bg(rgb(pal.input_bg))
                                            .flex()
                                            .justify_center()
                                            .text_size(px(11.5))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(pal.accent))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _e, _w, cx| {
                                                    this.pending_quick_action =
                                                        Some(InspectorQuickAction::OpenExportChat);
                                                    cx.notify();
                                                }),
                                            )
                                            .child("📤 导出记录"),
                                    ),
                            )
                            .into_any_element(),
                        InspectorTab::Members => {
                            InspectorTabsView::render_members(&self.members, cx).into_any_element()
                        }
                        InspectorTab::SharedMedia => {
                            InspectorTabsView::render_shared_media(&self.shared_media, cx)
                                .into_any_element()
                        }
                        InspectorTab::Settings => div()
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .child(InspectorTabsView::render_settings(&self.settings, cx))
                            // Channel / Group Statistics & Boosts Card (`info/channel_statistics`)
                            .child(
                                div()
                                    .p(px(12.0))
                                    .rounded(px(10.0))
                                    .bg(rgb(0x1B2836))
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_size(px(12.5))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x2AABEE))
                                            .child(format!(
                                                "📈 频道数据与助力等级 • Level {}",
                                                boost_lvl
                                            )),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.5))
                                            .text_color(rgb(0x8A9AA8))
                                            .child(format!(
                                                "当前助力: {} / {} Boosts • 7日阅读增长 +24.8%",
                                                boost_cur, boost_tgt
                                            )),
                                    )
                                    .child(
                                        div()
                                            .cursor_pointer()
                                            .py(px(6.0))
                                            .rounded(px(6.0))
                                            .bg(rgb(0x2AABEE))
                                            .flex()
                                            .justify_center()
                                            .text_size(px(12.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xFFFFFF))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _e, _w, cx| {
                                                    this.channel_boosts_current += 1;
                                                    if this.channel_boosts_current
                                                        >= this.channel_boosts_target
                                                    {
                                                        this.channel_boost_level += 1;
                                                        this.channel_boosts_target += 10;
                                                    }
                                                    cx.notify();
                                                }),
                                            )
                                            .child("⚡ 立即助力频道 (Boost Channel)"),
                                    ),
                            )
                            .into_any_element(),
                    }),
            )
            .into_any_element()
    }
}
