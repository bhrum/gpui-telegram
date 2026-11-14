//! ContextInspectorDrawer 320px Right Drawer (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.5).

use crate::i18n::tr_str;
use crate::theme::assets::{IconName, tg_icon};
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::drawer::inspector_panels::{
    render_members_panel, render_settings_and_stats_panel, render_shared_media_panel,
};
use crate::views::drawer::inspector_tabs::*;
use crate::views::drawer::statistics::info_statistics_inner_widget::ChartDataPoint;
use gpui::*;

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
    pub media_items: Vec<SharedMediaItem>,
    pub channel_stats: Vec<ChartDataPoint>,
    pub is_secret_chat_active: bool,
    pub is_contact_blocked: bool,
    pub pending_media_open: Option<(String, bool)>,
    pub pending_action: Option<InspectorAction>,
    pub copied_feedback: Option<String>,
    pub channel_boost_level: u32,
    pub channel_boosts_current: u32,
    pub channel_boosts_target: u32,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl ContextInspector {
    pub fn new() -> Self {
        Self {
            is_open: false,
            active_tab: InspectorTab::Profile,
            profile: Self::empty_profile(),
            members: Self::empty_members(),
            shared_media: Self::empty_media(),
            settings: Self::empty_settings(),
            media_items: Vec::new(),
            channel_stats: Vec::new(),
            is_secret_chat_active: false,
            is_contact_blocked: false,
            pending_media_open: None,
            pending_action: None,
            copied_feedback: None,
            channel_boost_level: 0,
            channel_boosts_current: 0,
            channel_boosts_target: 30,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".to_string(),
        }
    }

    pub fn new_demo() -> Self {
        let mut s = Self::new();
        s.is_open = true;
        s.channel_boost_level = 8;
        s.profile = ProfileTabModel {
            name: "Alice Smith".into(),
            username: "@alice_smith".into(),
            phone: "+1 (555) 019-2834".into(),
            bio: "Rust & Systems engineer. Building GPUI Telegram 1:1 desktop client.".into(),
            status: "online".into(),
            avatar_initials: "AS".into(),
            avatar_color: 0x2AABEE,
            avatar_path: None,
            facts: Vec::new(),
            is_notifications_enabled: true,
        };
        s.members.members = vec![
            MemberItem { id: 1, name: "Alice Smith".into(), initials: "AS".into(), color: 0x2AABEE, role: "Owner".into(), is_online: true },
            MemberItem { id: 2, name: "Bob Builder".into(), initials: "BB".into(), color: 0x9B59B6, role: "Admin".into(), is_online: false },
        ];
        s.settings.e2ee_fingerprint = "🔐 🌟 🚀 ⚡".into();
        s.channel_boosts_current = 24;
        s
    }

    fn empty_profile() -> ProfileTabModel {
        ProfileTabModel {
            name: String::new(), username: String::new(), phone: String::new(), bio: String::new(),
            status: String::new(), avatar_initials: String::new(), avatar_color: 0, avatar_path: None,
            facts: Vec::new(), is_notifications_enabled: true,
        }
    }
    fn empty_members() -> MembersTabModel {
        MembersTabModel { members: Vec::new(), search_query: String::new(), invite_link: String::new(), is_group_or_channel: false }
    }
    fn empty_media() -> SharedMediaTabModel {
        SharedMediaTabModel {
            active_subtab: SharedMediaSubTab::Media, media_count: 0, media_items: Vec::new(),
            files: Vec::new(), links: Vec::new(), voice_notes: Vec::new(), music_items: Vec::new(),
        }
    }
    fn empty_settings() -> SettingsTabModel {
        SettingsTabModel { e2ee_fingerprint: String::new(), auto_delete_period: "Off".into(), notification_sound: "Default (Aurora)".into() }
    }

    fn clear_peer_facts(&mut self) {
        self.profile = Self::empty_profile();
        self.members = Self::empty_members();
        self.shared_media = Self::empty_media();
        self.settings = Self::empty_settings();
        self.media_items.clear();
        self.channel_stats.clear();
        self.is_secret_chat_active = false;
        self.is_contact_blocked = false;
        self.pending_media_open = None;
        self.pending_action = None;
        self.copied_feedback = None;
    }

    pub fn clear_peer(&mut self) { self.is_open = false; self.active_tab = InspectorTab::Profile; self.clear_peer_facts(); }
    pub fn configure_for_cloud_workspace(&mut self, _server_url: &str) { self.clear_peer(); }
    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, lang: impl Into<String>) { self.language_id = lang.into(); }
    pub fn take_action(&mut self) -> Option<InspectorAction> { self.pending_action.take() }
    pub fn set_tab(&mut self, tab: InspectorTab) { self.active_tab = tab; }
    pub fn toggle_open(&mut self) { self.is_open = !self.is_open; }
    pub fn open(&mut self) { self.is_open = true; }
    pub fn close(&mut self) { self.is_open = false; }
    pub fn set_auto_delete_period(&mut self, period: impl Into<String>) { self.settings.auto_delete_period = period.into(); }

    pub fn set_peer(&mut self, name: &str, subtitle: &str, initials: &str, color: u32) {
        if name.trim().is_empty() { self.clear_peer(); return; }
        let same = !self.profile.name.is_empty() && self.profile.name == name && self.profile.avatar_initials == initials && self.profile.avatar_color == color;
        if !same { self.clear_peer_facts(); }
        self.profile.name = name.to_string();
        self.profile.bio = subtitle.to_string();
        self.profile.avatar_initials = initials.to_string();
        self.profile.avatar_color = color;
    }

    pub fn sync_profile_details(&mut self, username: String, phone: String, status: String, avatar_path: Option<String>, facts: Vec<(String, String)>) {
        self.profile.username = username; self.profile.phone = phone; self.profile.status = status;
        self.profile.avatar_path = avatar_path; self.profile.facts = facts;
    }

    pub fn sync_shared_content(&mut self, media: Vec<SharedMediaItem>, files: Vec<SharedFileItem>, links: Vec<SharedLinkItem>, voice_notes: Vec<SharedVoiceItem>, music_items: Vec<SharedFileItem>) {
        self.shared_media.media_count = media.len() as u32;
        self.media_items = media.clone();
        self.shared_media.media_items = media;
        self.shared_media.files = files; self.shared_media.links = links;
        self.shared_media.voice_notes = voice_notes; self.shared_media.music_items = music_items;
    }

    pub fn sync_members(&mut self, members: Vec<MemberItem>, invite_link: String, is_group_or_channel: bool) {
        self.members.members = members; self.members.invite_link = invite_link; self.members.is_group_or_channel = is_group_or_channel;
    }

    pub fn sync_channel_statistics(&mut self, boost_level: u32, current: u32, target: u32, bars: Vec<ChartDataPoint>) {
        self.channel_boost_level = boost_level; self.channel_boosts_current = current; self.channel_boosts_target = target; self.channel_stats = bars;
    }
}

impl Default for ContextInspector { fn default() -> Self { Self::new() } }

impl Render for ContextInspector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            return div().w(px(0.0)).h_full().overflow_hidden().into_any_element();
        }
        let pal = self.palette;
        let lang = self.language_id.as_str();
        let active_tab = self.active_tab;
        let body = match active_tab {
            InspectorTab::Profile => InspectorTabsView::render_profile(&self.profile, self.is_secret_chat_active, self.is_contact_blocked, self.copied_feedback.as_ref(), pal, lang, cx),
            InspectorTab::Members => render_members_panel(&self.members, pal, lang, cx),
            InspectorTab::SharedMedia => render_shared_media_panel(&self.shared_media, pal, lang, cx),
            InspectorTab::Settings => render_settings_and_stats_panel(&self.settings, self.channel_boost_level, self.channel_boosts_current, self.channel_boosts_target, &self.channel_stats, pal, lang, cx),
        };
        let title = match active_tab {
            InspectorTab::Profile => tr_str(lang, "个人资料", "Info"),
            InspectorTab::Members => tr_str(lang, "成员列表", "Members"),
            InspectorTab::SharedMedia => tr_str(lang, "共享内容", "Shared"),
            InspectorTab::Settings => tr_str(lang, "更多设置", "Settings"),
        };

        div().w(px(DRAWER_WIDTH)).flex_shrink_0().h_full().bg(rgb(pal.dialogs_bg)).border_l_1().border_color(rgb(pal.border_fg)).flex().flex_col()
            .child(div().h(px(HEADER_HEIGHT)).flex_shrink_0().px(px(16.0)).flex().items_center().justify_between().border_b_1().border_color(rgb(pal.border_fg))
                .child(div().text_size(px(15.0)).font_weight(FontWeight::BOLD).text_color(rgb(pal.text_primary)).child(title.to_string()))
                .child(div().cursor_pointer().w(px(28.0)).h(px(28.0)).rounded_full().hover(move |s| s.bg(rgb(pal.dialogs_hover_bg))).flex().items_center().justify_center()
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| { this.close(); cx.notify(); })).child(tg_icon(IconName::Close, 15.0, pal.icon_fg))))
            .child(div().h(px(40.0)).flex_shrink_0().px(px(8.0)).flex().items_center().justify_around().border_b_1().border_color(rgb(pal.border_fg))
                .child(Self::tab_btn(tr_str(lang, "资料", "Profile"), InspectorTab::Profile, active_tab, pal, cx))
                .child(Self::tab_btn(tr_str(lang, "成员", "Members"), InspectorTab::Members, active_tab, pal, cx))
                .child(Self::tab_btn(tr_str(lang, "媒体", "Media"), InspectorTab::SharedMedia, active_tab, pal, cx))
                .child(Self::tab_btn(tr_str(lang, "设置", "Settings"), InspectorTab::Settings, active_tab, pal, cx)))
            .child(div().id("inspector-tab-scroll").flex_1().min_h(px(0.0)).p(px(14.0)).overflow_y_scroll().child(body))
            .into_any_element()
    }
}

impl ContextInspector {
    fn tab_btn(label: &str, tab: InspectorTab, active: InspectorTab, pal: TdesktopThemePalette, cx: &mut Context<Self>) -> impl IntoElement {
        let sel = tab == active;
        div().cursor_pointer().h_full().px(px(8.0)).flex().items_center().border_b_2()
            .border_color(if sel { rgba((pal.accent << 8) | 0xFF) } else { rgba(0x00000000) })
            .text_size(px(12.0)).font_weight(if sel { FontWeight::BOLD } else { FontWeight::NORMAL })
            .text_color(rgb(if sel { pal.accent } else { pal.text_secondary }))
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| { this.set_tab(tab); cx.notify(); }))
            .child(label.to_string())
    }
}
