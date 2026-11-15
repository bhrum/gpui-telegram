//! Upstream Source: Telegram/SourceFiles/settings/sections/settings_global_ttl.cpp

use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_action_row, settings_card, settings_container, settings_section_header,
};
use gpui::prelude::FluentBuilder;
use gpui::*;

pub const TTL_OFF: u32 = 0;
pub const TTL_ONE_DAY: u32 = 86400;
pub const TTL_ONE_WEEK: u32 = 604800;
pub const TTL_ONE_MONTH: u32 = 2678400;

#[derive(Debug, Clone)]
pub struct SettingsGlobalTtlSections {
    pub current_ttl: u32,
    pub custom_ttl: Option<u32>,
    pub pending_ttl: Option<u32>,
    pub pending_save_global_ttl: Option<u32>,
    pub pending_apply_to_existing: Option<u32>,
    pub is_confirming: bool,
    pub existing_chats_count: usize,
    pub status_message: Option<String>,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
}

impl SettingsGlobalTtlSections {
    pub fn new() -> Self {
        Self {
            current_ttl: TTL_OFF,
            custom_ttl: None,
            pending_ttl: None,
            pending_save_global_ttl: None,
            pending_apply_to_existing: None,
            is_confirming: false,
            existing_chats_count: 42,
            status_message: None,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12),
            language_id: "en".into(),
        }
    }

    pub fn new_live() -> Self {
        let mut s = Self::new();
        s.existing_chats_count = 0;
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, lang: impl Into<String>) { self.language_id = lang.into(); }

    pub fn format_ttl(ttl: u32) -> &'static str {
        match ttl {
            TTL_OFF => "Off",
            TTL_ONE_DAY => "1 Day",
            TTL_ONE_WEEK => "1 Week",
            TTL_ONE_MONTH => "1 Month",
            _ => "Custom",
        }
    }

    pub fn format_ttl_localized(ttl: u32, lang: &str) -> &'static str {
        match ttl {
            TTL_OFF => tr_str(lang, "关闭", "Off"),
            TTL_ONE_DAY => tr_str(lang, "1 天", "1 Day"),
            TTL_ONE_WEEK => tr_str(lang, "1 周", "1 Week"),
            TTL_ONE_MONTH => tr_str(lang, "1 个月", "1 Month"),
            _ => tr_str(lang, "自定义", "Custom"),
        }
    }

    pub fn set_ttl(&mut self, ttl: u32) {
        if self.current_ttl == TTL_OFF && ttl != TTL_OFF {
            self.pending_ttl = Some(ttl);
            self.is_confirming = true;
        } else {
            self.current_ttl = ttl;
            self.pending_save_global_ttl = Some(ttl);
            let lbl = Self::format_ttl_localized(ttl, &self.language_id);
            self.status_message = Some(if self.language_id.starts_with("zh") {
                format!("自动删除已设置为 {}", lbl)
            } else {
                format!("Auto-delete set to {}", lbl)
            });
        }
    }

    pub fn confirm_pending(&mut self) {
        if let Some(ttl) = self.pending_ttl.take() {
            self.current_ttl = ttl;
            self.pending_save_global_ttl = Some(ttl);
            let lbl = Self::format_ttl_localized(ttl, &self.language_id);
            self.status_message = Some(if self.language_id.starts_with("zh") {
                format!("已开启全局自动删除: {}", lbl)
            } else {
                format!("Auto-delete enabled: {}", lbl)
            });
        }
        self.is_confirming = false;
    }

    pub fn cancel_confirm(&mut self) {
        self.pending_ttl = None;
        self.is_confirming = false;
    }

    pub fn apply_to_existing(&mut self) {
        self.pending_apply_to_existing = Some(self.current_ttl);
        let lbl = Self::format_ttl_localized(self.current_ttl, &self.language_id);
        self.status_message = Some(if self.language_id.starts_with("zh") {
            format!("已将自动删除计时器（{}）同步至现有私聊对话", lbl)
        } else {
            format!(
                "Applied auto-delete timer ({}) to {} existing chats",
                lbl,
                self.existing_chats_count
            )
        });
    }
}

impl Default for SettingsGlobalTtlSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsGlobalTtlSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let lang = &self.language_id;
        let opts = [TTL_OFF, TTL_ONE_DAY, TTL_ONE_WEEK, TTL_ONE_MONTH];

        settings_container(pal)
            .child(settings_card(pal)
                .child(settings_section_header(IconName::ClockSending, tr_str(lang, "全局自动删除消息", "Auto-Delete Messages"), Some(tr_str(lang, "新发起的所有对话将自动套用此自动删除期限", "Automatically delete messages in all new chats")), pal))
                .children(opts.into_iter().map(|ttl| {
                    let mark = if self.current_ttl == ttl { "✓" } else { "" };
                    let label = Self::format_ttl_localized(ttl, lang);
                    settings_action_row(IconName::ClockSending, label, mark, pal, false, move |this| {
                        this.set_ttl(ttl);
                    }, cx)
                }))
                .child(settings_action_row(
                    IconName::Forward,
                    tr_str(lang, "应用到现有私聊对话", "Apply to Existing Private Chats"),
                    Self::format_ttl_localized(self.current_ttl, lang),
                    pal,
                    false,
                    Self::apply_to_existing,
                    cx,
                ))
                .when(self.is_confirming, |el| {
                    let pending = self.pending_ttl.unwrap_or(TTL_ONE_DAY);
                    let confirm_title = if lang.starts_with("zh") {
                        format!("确认开启 {} 自动删除？", Self::format_ttl_localized(pending, lang))
                    } else {
                        format!("Enable {} auto-delete?", Self::format_ttl_localized(pending, lang))
                    };
                    el.child(settings_action_row(IconName::SelectCheck, &confirm_title, tr_str(lang, "确认", "Confirm"), pal, false, Self::confirm_pending, cx))
                        .child(settings_action_row(IconName::Close, tr_str(lang, "取消", "Cancel"), "", pal, true, Self::cancel_confirm, cx))
                })
                .when_some(self.status_message.clone(), |el, msg| {
                    el.child(div().text_size(px(pal.scaled_px(12.0))).text_color(rgb(pal.success)).child(msg))
                }))
            .into_any_element()
    }
}
