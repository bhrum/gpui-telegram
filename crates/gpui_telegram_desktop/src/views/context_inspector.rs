//! ContextInspectorDrawer 320px Right Drawer (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.5).

use crate::components::ui::{TextInputController, delete_prev_word, pop_last_char};
use crate::i18n::tr_str;
use crate::theme::assets::{IconName, tg_icon};
use crate::theme::palette::TdesktopThemePalette;
use crate::views::drawer::bot_computer_inspector::{
    render_bot_computers_subpage, render_bot_config_editor, render_bot_profile_main,
};
use crate::views::drawer::inspector_panels::{
    render_members_panel, render_settings_and_stats_panel, render_shared_media_panel,
};
use crate::views::drawer::inspector_tabs::*;
use crate::views::drawer::statistics::info_statistics_inner_widget::ChartDataPoint;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_proto::{BotComputerProfileDto, ComputerNodeDto, ComputerRunPolicy};

pub const DRAWER_WIDTH: f32 = 320.0;
pub const CONTEXT_INSPECTOR_WIDTH: f32 = DRAWER_WIDTH;
pub const HEADER_HEIGHT: f32 = 54.0;
pub const CONTEXT_INSPECTOR_HEADER_HEIGHT: f32 = HEADER_HEIGHT;
pub const AVATAR_SIZE: f32 = 80.0;
pub const CONTEXT_INSPECTOR_AVATAR_SIZE: f32 = AVATAR_SIZE;

pub struct ContextInspector {
    pub is_open: bool,
    pub active_tab: InspectorTab,
    pub subpage: InspectorSubpage,
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
    pub computers: Vec<ComputerNodeDto>,
    pub bot_computer_profile: Option<BotComputerProfileDto>,
    pub bot_config_focus: BotConfigFocusField,
    pub bot_draft_persona: String,
    pub bot_draft_model: String,
    pub bot_draft_approval_mode: ComputerRunPolicy,
    pub bot_draft_allow_cross_device: bool,
    pub bot_draft_allow_cloud_self_config: bool,
    pub persona_input: Option<Entity<TextInputController>>,
    pub model_input: Option<Entity<TextInputController>>,
    pub active_fabushi_bot: Option<gpui_telegram_proto::miniapp_contracts::FabushiBotSummaryDto>,
}

impl ContextInspector {
    pub fn new() -> Self {
        Self {
            is_open: false,
            active_tab: InspectorTab::Profile,
            subpage: InspectorSubpage::Main,
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
            palette: TdesktopThemePalette::default(),
            language_id: "en".to_string(),
            computers: Vec::new(),
            bot_computer_profile: None,
            bot_config_focus: BotConfigFocusField::None,
            bot_draft_persona: String::new(),
            bot_draft_model: String::new(),
            bot_draft_approval_mode: ComputerRunPolicy::AskEveryTime,
            bot_draft_allow_cross_device: true,
            bot_draft_allow_cloud_self_config: true,
            persona_input: None,
            model_input: None,
            active_fabushi_bot: None,
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
            is_bot: false,
            bot_id: 0,
            bot_persona_prompt: String::new(),
            bot_commands: Vec::new(),
            miniapp_title: None,
            miniapp_url: None,
            in_attach_menu: false,
            bot_mau_count: 0,
            bot_has_main_app: false,
            bot_privacy_policy_url: String::new(),
            bot_has_settings: false,
            bot_can_join_groups: false,
            is_viewer_owner: false,
            is_verified: false,
            is_blocked: false,
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
            is_bot: false, bot_id: 0, bot_persona_prompt: String::new(), bot_commands: Vec::new(),
            miniapp_title: None, miniapp_url: None, in_attach_menu: false,
            bot_mau_count: 0, bot_has_main_app: false, bot_privacy_policy_url: String::new(),
            bot_has_settings: false, bot_can_join_groups: false, is_viewer_owner: false,
            is_verified: false, is_blocked: false,
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
        self.subpage = InspectorSubpage::Main;
        self.bot_computer_profile = None;
        self.bot_config_focus = BotConfigFocusField::None;
        self.bot_draft_persona.clear();
        self.bot_draft_model.clear();
        self.bot_draft_approval_mode = ComputerRunPolicy::AskEveryTime;
        self.bot_draft_allow_cross_device = true;
        self.bot_draft_allow_cloud_self_config = true;
        self.active_fabushi_bot = None;
    }

    pub fn clear_peer(&mut self) { self.is_open = false; self.active_tab = InspectorTab::Profile; self.clear_peer_facts(); }
    pub fn configure_for_cloud_workspace(&mut self, _server_url: &str) { self.clear_peer(); }
    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, lang: impl Into<String>) { self.language_id = lang.into(); }
    pub fn take_action(&mut self) -> Option<InspectorAction> { self.pending_action.take() }
    pub fn set_active_fabushi_bot(&mut self, bot: Option<gpui_telegram_proto::miniapp_contracts::FabushiBotSummaryDto>) { self.active_fabushi_bot = bot; }
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

    #[allow(clippy::too_many_arguments)]
    pub fn sync_bot_details(
        &mut self,
        is_bot: bool,
        bot_id: u64,
        persona_prompt: String,
        commands: Vec<(String, String)>,
        miniapp_title: Option<String>,
        miniapp_url: Option<String>,
        in_attach_menu: bool,
    ) {
        self.profile.is_bot = is_bot;
        self.profile.bot_id = bot_id;
        self.profile.bot_persona_prompt = persona_prompt;
        self.profile.bot_commands = commands;
        self.profile.miniapp_title = miniapp_title;
        self.profile.miniapp_url = miniapp_url;
        self.profile.in_attach_menu = in_attach_menu;
        if !is_bot {
            self.profile.bot_mau_count = 0;
            self.profile.bot_has_main_app = false;
            self.profile.bot_privacy_policy_url.clear();
            self.profile.bot_has_settings = false;
            self.profile.bot_can_join_groups = false;
            self.profile.is_viewer_owner = false;
            self.profile.is_verified = false;
            self.profile.is_blocked = false;
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn sync_bot_profile_extra(
        &mut self,
        mau_count: i64,
        has_main_app: bool,
        privacy_policy_url: String,
        has_settings: bool,
        can_join_groups: bool,
        is_viewer_owner: bool,
        is_verified: bool,
        is_blocked: bool,
    ) {
        self.profile.bot_mau_count = mau_count;
        self.profile.bot_has_main_app = has_main_app;
        self.profile.bot_privacy_policy_url = privacy_policy_url;
        self.profile.bot_has_settings = has_settings;
        self.profile.bot_can_join_groups = can_join_groups;
        self.profile.is_viewer_owner = is_viewer_owner;
        self.profile.is_verified = is_verified;
        self.profile.is_blocked = is_blocked;
        self.is_contact_blocked = is_blocked;
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

    pub fn ensure_bot_inputs(&mut self, cx: &mut Context<Self>) {
        if self.persona_input.is_none() {
            self.persona_input = Some(cx.new(|_| TextInputController::new()));
        }
        if self.model_input.is_none() {
            self.model_input = Some(cx.new(|_| TextInputController::new()));
        }
    }

    pub fn sync_computers(&mut self, computers: Vec<ComputerNodeDto>) {
        self.computers = computers;
    }

    pub fn sync_bot_computer_profile(&mut self, profile: Option<BotComputerProfileDto>) {
        let same_bot = self
            .bot_computer_profile
            .as_ref()
            .zip(profile.as_ref())
            .is_some_and(|(a, b)| a.bot_id == b.bot_id);
        let editing_now = self.subpage == InspectorSubpage::BotConfigEditor && same_bot;
        self.bot_computer_profile = profile.clone();
        if !editing_now {
            if let Some(p) = profile {
                self.bot_draft_persona = p.persona_prompt;
                self.bot_draft_model = p.preferred_model;
                self.bot_draft_approval_mode = p.approval_mode;
                self.bot_draft_allow_cross_device = p.allow_cross_device_control;
                self.bot_draft_allow_cloud_self_config = p.allow_cloud_self_config;
            } else {
                self.bot_draft_persona = self.profile.bot_persona_prompt.clone();
                if self.bot_draft_model.is_empty() {
                    self.bot_draft_model = "gpt-5.4".to_string();
                }
            }
        }
    }

    pub fn open_bot_config_editor(&mut self, cx: &mut Context<Self>) {
        self.ensure_bot_inputs(cx);
        if let Some(ref p) = self.bot_computer_profile {
            self.bot_draft_persona = p.persona_prompt.clone();
            self.bot_draft_model = p.preferred_model.clone();
            self.bot_draft_approval_mode = p.approval_mode;
            self.bot_draft_allow_cross_device = p.allow_cross_device_control;
            self.bot_draft_allow_cloud_self_config = p.allow_cloud_self_config;
        } else {
            self.bot_draft_persona = self.profile.bot_persona_prompt.clone();
            if self.bot_draft_model.is_empty() {
                self.bot_draft_model = "gpt-5.4".to_string();
            }
        }
        self.subpage = InspectorSubpage::BotConfigEditor;
        self.bot_config_focus = BotConfigFocusField::Persona;
        cx.notify();
    }

    pub fn is_bot_input_active(&self) -> bool {
        self.is_open
            && self.active_tab == InspectorTab::Profile
            && self.profile.is_bot
            && self.subpage == InspectorSubpage::BotConfigEditor
            && self.bot_config_focus != BotConfigFocusField::None
    }

    fn active_bot_controller(&self) -> Option<&Entity<TextInputController>> {
        match self.bot_config_focus {
            BotConfigFocusField::Persona => self.persona_input.as_ref(),
            BotConfigFocusField::Model => self.model_input.as_ref(),
            BotConfigFocusField::None => None,
        }
    }

    fn active_bot_draft_text(&self) -> &str {
        match self.bot_config_focus {
            BotConfigFocusField::Persona => &self.bot_draft_persona,
            BotConfigFocusField::Model => &self.bot_draft_model,
            BotConfigFocusField::None => "",
        }
    }

    fn active_bot_draft_mut(&mut self) -> Option<&mut String> {
        match self.bot_config_focus {
            BotConfigFocusField::Persona => Some(&mut self.bot_draft_persona),
            BotConfigFocusField::Model => Some(&mut self.bot_draft_model),
            BotConfigFocusField::None => None,
        }
    }

    pub fn active_bot_input_text_and_preedit(&self, cx: &App) -> (String, Option<String>) {
        let committed = self.active_bot_draft_text().to_string();
        if let Some(ctrl) = self.active_bot_controller() {
            ctrl.read(cx).text_and_preedit(committed)
        } else {
            (committed, None)
        }
    }

    pub fn push_bot_input_text(&mut self, text: &str, cx: &mut Context<Self>) -> bool {
        if !self.is_bot_input_active() {
            return false;
        }
        if let Some(ctrl) = self.active_bot_controller().cloned() {
            ctrl.update(cx, |c, _| c.clear_preedit());
        }
        if let Some(dst) = self.active_bot_draft_mut() {
            dst.push_str(text);
            cx.notify();
            return true;
        }
        false
    }

    pub fn clear_active_bot_input(&mut self, cx: &mut Context<Self>) -> bool {
        if !self.is_bot_input_active() {
            return false;
        }
        if let Some(ctrl) = self.active_bot_controller().cloned() {
            ctrl.update(cx, |c, _| c.clear_preedit());
        }
        if let Some(dst) = self.active_bot_draft_mut() {
            dst.clear();
            cx.notify();
            return true;
        }
        false
    }

    pub fn handle_bot_non_char_key(
        &mut self,
        key: &str,
        is_alt: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.is_bot_input_active() {
            return false;
        }
        match key {
            "backspace" | "delete" => {
                if let Some(dst) = self.active_bot_draft_mut() {
                    let changed = if is_alt {
                        delete_prev_word(dst)
                    } else {
                        pop_last_char(dst)
                    };
                    if changed {
                        cx.notify();
                    }
                }
                true
            }
            "tab" => {
                self.bot_config_focus = match self.bot_config_focus {
                    BotConfigFocusField::Persona => BotConfigFocusField::Model,
                    BotConfigFocusField::Model => BotConfigFocusField::Persona,
                    BotConfigFocusField::None => BotConfigFocusField::None,
                };
                cx.notify();
                true
            }
            "enter" => {
                if self.bot_config_focus == BotConfigFocusField::Persona {
                    self.bot_draft_persona.push('\n');
                    cx.notify();
                } else {
                    self.bot_config_focus = BotConfigFocusField::None;
                    cx.notify();
                }
                true
            }
            _ => false,
        }
    }

    pub fn replace_and_mark_bot_input(
        &mut self,
        new_text: &str,
        cx: &mut Context<Self>,
    ) -> Option<String> {
        self.ensure_bot_inputs(cx);
        let commit = self
            .active_bot_controller()
            .cloned()
            .and_then(|c| c.update(cx, |ctrl, _| ctrl.handle_replace_and_mark(new_text)));
        if let Some(txt) = commit.as_ref() {
            if let Some(dst) = self.active_bot_draft_mut() {
                dst.push_str(txt);
            }
        }
        cx.notify();
        commit
    }

    pub fn set_bot_ime_preedit(&mut self, preedit: Option<&str>, cx: &mut Context<Self>) {
        self.ensure_bot_inputs(cx);
        if let Some(ctrl) = self.active_bot_controller().cloned() {
            ctrl.update(cx, |c, _| {
                c.ime_preedit = preedit.filter(|s| !s.is_empty()).map(str::to_string);
            });
            cx.notify();
        }
    }
}

impl Default for ContextInspector { fn default() -> Self { Self::new() } }

impl Render for ContextInspector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            return div().w(px(0.0)).h_full().overflow_hidden().into_any_element();
        }
        let pal = self.palette;
        if !self.members.is_group_or_channel && self.active_tab == InspectorTab::Members {
            self.active_tab = InspectorTab::Profile;
        }
        let has_bot_media = self.shared_media.media_count > 0
            || !self.shared_media.files.is_empty()
            || !self.shared_media.links.is_empty();
        if self.profile.is_bot {
            if matches!(self.active_tab, InspectorTab::Members | InspectorTab::Settings)
                || (self.active_tab == InspectorTab::SharedMedia && !has_bot_media)
            {
                self.active_tab = InspectorTab::Profile;
            }
        }
        let active_tab = self.active_tab;
        let is_bot_profile_tab = active_tab == InspectorTab::Profile && self.profile.is_bot;
        let allow_owner_subpages =
            self.profile.is_viewer_owner && self.active_fabushi_bot.is_some();
        let subpage = if is_bot_profile_tab && allow_owner_subpages {
            self.subpage
        } else {
            InspectorSubpage::Main
        };
        if is_bot_profile_tab && subpage == InspectorSubpage::BotConfigEditor {
            self.ensure_bot_inputs(cx);
        }
        let lang = self.language_id.as_str();

        let body = match active_tab {
            InspectorTab::Profile => {
                if self.profile.is_bot {
                    match subpage {
                        InspectorSubpage::Main => render_bot_profile_main(self, pal, lang, cx),
                        InspectorSubpage::ComputersList => {
                            render_bot_computers_subpage(self, pal, lang, cx)
                        }
                        InspectorSubpage::BotConfigEditor => {
                            render_bot_config_editor(self, pal, lang, cx)
                        }
                    }
                } else {
                    InspectorTabsView::render_profile(
                        &self.profile,
                        self.is_secret_chat_active,
                        self.is_contact_blocked,
                        self.copied_feedback.as_ref(),
                        pal,
                        lang,
                        cx,
                    )
                }
            }
            InspectorTab::Members => render_members_panel(&self.members, pal, lang, cx),
            InspectorTab::SharedMedia => render_shared_media_panel(&self.shared_media, pal, lang, cx),
            InspectorTab::Settings => render_settings_and_stats_panel(&self.settings, self.channel_boost_level, self.channel_boosts_current, self.channel_boosts_target, &self.channel_stats, pal, lang, cx),
        };
        let title = if self.profile.is_bot {
            tr_str(lang, "机器人信息", "Bot Info")
        } else {
            match active_tab {
                InspectorTab::Profile => tr_str(lang, "个人资料", "Profile"),
                InspectorTab::Members => tr_str(lang, "成员列表", "Members"),
                InspectorTab::SharedMedia => tr_str(lang, "共享内容", "Shared"),
                InspectorTab::Settings => tr_str(lang, "更多设置", "Settings"),
            }
        };

        let header_left = if subpage != InspectorSubpage::Main {
            let sub_label = match subpage {
                InspectorSubpage::ComputersList => tr_str(lang, "电脑", "Computers"),
                InspectorSubpage::BotConfigEditor => tr_str(lang, "AI Bot 配置", "Bot Config"),
                InspectorSubpage::Main => "",
            };
            div()
                .flex()
                .items_center()
                .gap(px(4.0))
                .child(
                    div()
                        .id("inspector-breadcrumb-root")
                        .cursor_pointer()
                        .text_size(px(14.5))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(pal.text_secondary))
                        .hover(|s| s.text_color(rgb(pal.text_primary)))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                this.subpage = InspectorSubpage::Main;
                                this.bot_config_focus = BotConfigFocusField::None;
                                this.pending_action =
                                    Some(InspectorAction::CloseBotComputersSubpage);
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "机器人信息", "Bot Info").to_string()),
                )
                .child(tg_icon(IconName::ChevronRight, 13.0, pal.text_secondary))
                .child(
                    div()
                        .text_size(px(14.5))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .child(sub_label.to_string()),
                )
                .into_any_element()
        } else {
            div()
                .text_size(px(15.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(pal.text_primary))
                .child(title.to_string())
                .into_any_element()
        };

        let show_tabs = subpage == InspectorSubpage::Main && (!self.profile.is_bot || has_bot_media);
        let mut root = div()
            .w(px(DRAWER_WIDTH))
            .flex_shrink_0()
            .h_full()
            .bg(rgb(pal.dialogs_bg))
            .border_l_1()
            .border_color(rgb(pal.border_fg))
            .flex()
            .flex_col()
            .child(
                div()
                    .h(px(HEADER_HEIGHT))
                    .flex_shrink_0()
                    .px(px(16.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(rgb(pal.border_fg))
                    .child(header_left)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .when(!self.profile.is_bot || allow_owner_subpages, |row| {
                                row.child(
                                    div()
                                        .id("inspector-header-more")
                                        .cursor_pointer()
                                        .w(px(28.0))
                                        .h(px(28.0))
                                        .rounded_full()
                                        .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                                if this.profile.is_bot
                                                    && this.profile.is_viewer_owner
                                                    && this.active_fabushi_bot.is_some()
                                                {
                                                    if this.subpage == InspectorSubpage::Main {
                                                        this.subpage =
                                                            InspectorSubpage::ComputersList;
                                                        this.pending_action = Some(
                                                            InspectorAction::OpenBotComputersSubpage,
                                                        );
                                                    } else {
                                                        this.subpage = InspectorSubpage::Main;
                                                        this.bot_config_focus =
                                                            BotConfigFocusField::None;
                                                    }
                                                    cx.notify();
                                                }
                                            }),
                                        )
                                        .child(tg_icon(IconName::MoreVertical, 15.0, pal.icon_fg)),
                                )
                            })
                            .child(
                                div()
                                    .id("inspector-header-close")
                                    .cursor_pointer()
                                    .w(px(28.0))
                                    .h(px(28.0))
                                    .rounded_full()
                                    .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.bot_config_focus = BotConfigFocusField::None;
                                            this.close();
                                            cx.notify();
                                        }),
                                    )
                                    .child(tg_icon(IconName::Close, 15.0, pal.icon_fg)),
                            ),
                    ),
            );

        if show_tabs {
            let mut tabs_row = div()
                .h(px(40.0))
                .flex_shrink_0()
                .px(px(8.0))
                .flex()
                .items_center()
                .justify_around()
                .border_b_1()
                .border_color(rgb(pal.border_fg))
                .child(Self::tab_btn(
                    tr_str(lang, "资料", "Profile"),
                    InspectorTab::Profile,
                    active_tab,
                    pal,
                    cx,
                ));
            if self.profile.is_bot {
                tabs_row = tabs_row.child(Self::tab_btn(
                    tr_str(lang, "媒体", "Media"),
                    InspectorTab::SharedMedia,
                    active_tab,
                    pal,
                    cx,
                ));
            } else {
                if self.members.is_group_or_channel {
                    tabs_row = tabs_row.child(Self::tab_btn(
                        tr_str(lang, "成员", "Members"),
                        InspectorTab::Members,
                        active_tab,
                        pal,
                        cx,
                    ));
                }
                tabs_row = tabs_row
                    .child(Self::tab_btn(
                        tr_str(lang, "媒体", "Media"),
                        InspectorTab::SharedMedia,
                        active_tab,
                        pal,
                        cx,
                    ))
                    .child(Self::tab_btn(
                        tr_str(lang, "设置", "Settings"),
                        InspectorTab::Settings,
                        active_tab,
                        pal,
                        cx,
                    ));
            }
            root = root.child(tabs_row);
        }

        root.child(
            div()
                .id("inspector-tab-scroll")
                .flex_1()
                .min_h(px(0.0))
                .p(px(14.0))
                .overflow_y_scroll()
                .child(body),
        )
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
