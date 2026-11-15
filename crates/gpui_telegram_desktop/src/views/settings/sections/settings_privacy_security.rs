// Source: Telegram/SourceFiles/settings/sections/settings_privacy_security.cpp

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_container,
    settings_section_header, settings_subsection_title, settings_toggle_row,
};
use crate::views::settings::settings_type::SettingsSectionType;
use gpui::prelude::FluentBuilder;
use gpui::*;

#[derive(Debug, Clone)]
pub struct SettingsPrivacySecuritySections {
    pub blocked_count: usize,
    pub passcode_enabled: bool,
    pub two_factor_enabled: bool,
    pub active_sessions_count: usize,
    pub ttl_months: u32,
    pub global_ttl_seconds: u32,
    pub sensitive_content_enabled: bool,
    pub phone_privacy: String,
    pub last_seen_privacy: String,
    pub photo_privacy: String,
    pub calls_privacy: String,
    pub forwards_privacy: String,
    pub groups_privacy: String,
    pub two_factor_editing: bool,
    pub two_factor_password_draft: String,
    pub pending_nav_section: Option<SettingsSectionType>,
    pub pending_edit_privacy_category: Option<String>,
    pub pending_2fa_action: Option<Option<String>>,
    pub pending_save_settings: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl SettingsPrivacySecuritySections {
    pub fn new() -> Self {
        Self {
            blocked_count: 0,
            passcode_enabled: false,
            two_factor_enabled: true,
            active_sessions_count: 2,
            ttl_months: 6,
            global_ttl_seconds: 0,
            sensitive_content_enabled: false,
            phone_privacy: "Contacts".into(),
            last_seen_privacy: "Everybody".into(),
            photo_privacy: "Everybody".into(),
            calls_privacy: "Contacts".into(),
            forwards_privacy: "Everybody".into(),
            groups_privacy: "Contacts".into(),
            two_factor_editing: false,
            two_factor_password_draft: String::new(),
            pending_nav_section: None,
            pending_edit_privacy_category: None,
            pending_2fa_action: None,
            pending_save_settings: false,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".into(),
        }
    }

    pub fn new_live() -> Self {
        let mut s = Self::new();
        s.two_factor_enabled = false;
        s.active_sessions_count = 0;
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, lang: impl Into<String>) { self.language_id = lang.into(); }
    pub fn set_blocked_count(&mut self, count: usize) { self.blocked_count = count; }
    pub fn set_passcode_enabled(&mut self, enabled: bool) { self.passcode_enabled = enabled; }
    pub fn set_two_factor_enabled(&mut self, enabled: bool) { self.two_factor_enabled = enabled; }

    pub fn set_ttl_months(&mut self, months: u32) {
        self.ttl_months = match months {
            1 | 3 | 6 | 12 | 18 | 24 => months,
            _ => 6,
        };
    }

    pub fn toggle_sensitive_content(&mut self) {
        self.sensitive_content_enabled = !self.sensitive_content_enabled;
        self.pending_save_settings = true;
    }

    pub fn ttl_description(&self) -> String {
        match self.ttl_months {
            1 => "1 month".into(),
            3 => "3 months".into(),
            6 => "6 months".into(),
            12 => "1 year".into(),
            18 => "18 months".into(),
            24 => "2 years".into(),
            n => format!("{} months", n),
        }
    }

    pub fn ttl_description_localized(&self, lang: &str) -> String {
        if !lang.starts_with("zh") {
            return self.ttl_description();
        }
        match self.ttl_months {
            1 => "1 个月".into(),
            3 => "3 个月".into(),
            6 => "6 个月".into(),
            12 => "1 年".into(),
            18 => "18 个月".into(),
            24 => "2 年".into(),
            n => format!("{} 个月", n),
        }
    }

    pub fn global_ttl_label(&self) -> &'static str {
        match self.global_ttl_seconds {
            0 => "Off",
            86_400 => "1 Day",
            604_800 => "1 Week",
            2_678_400 | 2_592_000 => "1 Month",
            _ => "Custom",
        }
    }

    pub fn global_ttl_label_localized(&self, lang: &str) -> &'static str {
        match self.global_ttl_seconds {
            0 => tr_str(lang, "关闭", "Off"),
            86_400 => tr_str(lang, "1 天", "1 Day"),
            604_800 => tr_str(lang, "1 周", "1 Week"),
            2_678_400 | 2_592_000 => tr_str(lang, "1 个月", "1 Month"),
            _ => tr_str(lang, "自定义", "Custom"),
        }
    }

    fn localize_privacy_value<'a>(val: &'a str, lang: &str) -> &'a str {
        if !lang.starts_with("zh") {
            return val;
        }
        match val {
            "Everybody" => "所有人",
            "Contacts" | "My Contacts" => "我的联系人",
            "Nobody" => "没有人",
            "Premium" | "Premium Only" => "仅限高级会员",
            "Paid (Stars)" => "星币付费",
            _ => val,
        }
    }
}

impl Default for SettingsPrivacySecuritySections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsPrivacySecuritySections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let lang = self.language_id.clone();
        let on_lbl = tr_str(&lang, "已开启", "On");
        let off_lbl = tr_str(&lang, "已关闭", "Off");
        let blocked_val = format!("{}", self.blocked_count);
        let sessions_val = format!("{}", self.active_sessions_count);
        let passcode_val = if self.passcode_enabled { on_lbl } else { off_lbl };
        let twofa_val = if self.two_factor_enabled { on_lbl } else { off_lbl };
        let global_ttl_val = self.global_ttl_label_localized(&lang);
        let ttl_desc = self.ttl_description_localized(&lang);
        let is_2fa_editing = self.two_factor_editing;
        let two_factor_enabled = self.two_factor_enabled;
        let masked_pw: String = "•".repeat(self.two_factor_password_draft.chars().count());

        let privacy_rows = [
            ("Phone Number", tr_str(&lang, "手机号码", "Phone Number"), self.phone_privacy.clone()),
            ("Last Seen & Online", tr_str(&lang, "最后上线状态", "Last Seen & Online"), self.last_seen_privacy.clone()),
            ("Profile Photos", tr_str(&lang, "个人头像", "Profile Photos"), self.photo_privacy.clone()),
            ("Calls", tr_str(&lang, "语音与视频通话", "Calls"), self.calls_privacy.clone()),
            ("Forwarded Messages", tr_str(&lang, "转发消息链接", "Forwarded Messages"), self.forwards_privacy.clone()),
            ("Groups & Channels", tr_str(&lang, "群组与频道邀请", "Groups & Channels"), self.groups_privacy.clone()),
        ];

        settings_container(pal)
            .child(
                settings_card(pal)
                    .child(settings_section_header(
                        IconName::Lock,
                        tr_str(&lang, "隐私与安全", "Privacy and Security"),
                        Some(tr_str(&lang, "管理登录设备、两步验证密码与隐私可见范围", "Manage active devices, 2FA, and privacy visibility")),
                        pal,
                    ))
                    .child(settings_action_row(
                        IconName::Close,
                        tr_str(&lang, "已屏蔽用户", "Blocked Users"),
                        &blocked_val,
                        pal,
                        false,
                        |this| {
                            this.pending_nav_section = Some(SettingsSectionType::BlockedPeers);
                        },
                        cx,
                    ))
                    .child(settings_action_row(
                        IconName::ComputerDesktop,
                        tr_str(&lang, "活跃设备与会话", "Active Sessions"),
                        &sessions_val,
                        pal,
                        false,
                        |this| {
                            this.pending_nav_section = Some(SettingsSectionType::ActiveSessions);
                        },
                        cx,
                    ))
                    .child(settings_action_row(
                        IconName::Lock,
                        tr_str(&lang, "本地密码锁", "Local Passcode"),
                        passcode_val,
                        pal,
                        false,
                        |this| {
                            this.pending_nav_section = Some(SettingsSectionType::LocalPasscode);
                        },
                        cx,
                    ))
                    .child(settings_action_row(
                        IconName::Verified,
                        tr_str(&lang, "两步验证密码", "Two-Step Verification"),
                        twofa_val,
                        pal,
                        false,
                        |this| {
                            this.two_factor_editing = !this.two_factor_editing;
                        },
                        cx,
                    ))
                    .when(is_2fa_editing, |el| {
                        el.child(
                            div()
                                .w_full()
                                .p(px(10.0))
                                .rounded(px(8.0))
                                .bg(rgb(pal.input_bg))
                                .border_1()
                                .border_color(rgb(pal.accent))
                                .flex()
                                .flex_col()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(rgb(pal.text_primary))
                                        .child(if masked_pw.is_empty() {
                                            tr_str(&lang, "直接输入云端两步验证密码…", "Type cloud 2FA password...").to_string()
                                        } else {
                                            format!("{}|", masked_pw)
                                        }),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .justify_end()
                                        .gap(px(8.0))
                                        .when(two_factor_enabled, |row| {
                                            row.child(
                                                div()
                                                    .px(px(12.0))
                                                    .py(px(5.0))
                                                    .rounded(px(6.0))
                                                    .bg(rgb(pal.danger))
                                                    .text_size(px(12.0))
                                                    .text_color(rgb(pal.active_button_fg))
                                                    .cursor_pointer()
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        cx.listener(|this, _e, _w, cx| {
                                                            this.pending_2fa_action = Some(None);
                                                            this.two_factor_editing = false;
                                                            this.two_factor_password_draft.clear();
                                                            cx.notify();
                                                        }),
                                                    )
                                                    .child(tr_str(&lang, "关闭两步验证", "Disable 2FA")),
                                            )
                                        })
                                        .child(
                                            div()
                                                .px(px(12.0))
                                                .py(px(5.0))
                                                .rounded(px(6.0))
                                                .bg(rgb(pal.accent))
                                                .text_size(px(12.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(rgb(pal.active_button_fg))
                                                .cursor_pointer()
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _e, _w, cx| {
                                                        if !this.two_factor_password_draft.trim().is_empty() {
                                                            let pw = this.two_factor_password_draft.trim().to_string();
                                                            this.pending_2fa_action = Some(Some(pw));
                                                            this.two_factor_editing = false;
                                                            this.two_factor_password_draft.clear();
                                                            cx.notify();
                                                        }
                                                    }),
                                                )
                                                .child(tr_str(&lang, "保存两步验证密码", "Save 2FA Password")),
                                        ),
                                ),
                        )
                    })
                    .child(settings_action_row(
                        IconName::ClockSending,
                        tr_str(&lang, "全局自动删除消息", "Auto-Delete Messages"),
                        global_ttl_val,
                        pal,
                        false,
                        |this| {
                            this.pending_nav_section = Some(SettingsSectionType::GlobalTtl);
                        },
                        cx,
                    ))
                    .child(settings_action_row(
                        IconName::Lock,
                        tr_str(&lang, "通行密钥 (Passkeys)", "Passkeys"),
                        "›",
                        pal,
                        false,
                        |this| {
                            this.pending_nav_section = Some(SettingsSectionType::Passkeys);
                        },
                        cx,
                    ))
                    .child(settings_action_row(
                        IconName::BotRobot,
                        tr_str(&lang, "已授权网站", "Connected Websites"),
                        "›",
                        pal,
                        false,
                        |this| {
                            this.pending_nav_section = Some(SettingsSectionType::Websites);
                        },
                        cx,
                    )),
            )
            .child(
                settings_card(pal)
                    .child(settings_subsection_title(tr_str(&lang, "隐私权限", "Privacy"), pal))
                    .children(privacy_rows.into_iter().map(|(cat_key, label, val)| {
                        let cat = cat_key.to_string();
                        let val_loc = Self::localize_privacy_value(&val, &lang).to_string();
                        settings_action_row(
                            IconName::Lock,
                            label,
                            &val_loc,
                            pal,
                            false,
                            move |this| {
                                this.pending_edit_privacy_category = Some(cat.clone());
                            },
                            cx,
                        )
                    })),
            )
            .child(
                settings_card(pal)
                    .child(settings_subsection_title(tr_str(&lang, "账号与内容控制", "Account & Content"), pal))
                    .child(settings_action_row(
                        IconName::ClockSending,
                        tr_str(&lang, "账号自毁期限", "If Away For"),
                        &ttl_desc,
                        pal,
                        false,
                        |this| {
                            let next = match this.ttl_months { 1 => 3, 3 => 6, 6 => 12, 12 => 18, 18 => 24, _ => 1 };
                            this.set_ttl_months(next);
                            this.pending_save_settings = true;
                        },
                        cx,
                    ))
                    .child(settings_toggle_row(
                        IconName::Close,
                        tr_str(&lang, "关闭敏感内容过滤", "Disable Filtering"),
                        None,
                        self.sensitive_content_enabled,
                        pal,
                        Self::toggle_sensitive_content,
                        cx,
                    )),
            )
            .into_any_element()
    }
}
