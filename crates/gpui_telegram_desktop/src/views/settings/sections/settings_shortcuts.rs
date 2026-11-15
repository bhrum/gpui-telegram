use gpui::*;
use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::settings::settings_theme_ui::{
    settings_card, settings_container, settings_section_header, settings_subsection_title,
    settings_toggle_row,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutEntry {
    pub command_id: String,
    pub description: String,
    pub key_sequence: String,
}

pub struct SettingsShortcutsSections {
    pub shortcuts: Vec<ShortcutEntry>,
    pub shortcuts_enabled: bool,
    pub search_query: String,
    pub is_visible: bool,
    pub pending_sync: bool,
    pub palette: TdesktopThemePalette,
    pub language_id: String,
    pub selected_filter: Option<&'static str>,
}

impl SettingsShortcutsSections {
    pub fn new() -> Self {
        Self {
            shortcuts: vec![
                ShortcutEntry { command_id: "chat_first".into(), description: "Jump to First Chat".into(), key_sequence: "Cmd+1 / Ctrl+1".into() },
                ShortcutEntry { command_id: "chat_last".into(), description: "Jump to Last Chat".into(), key_sequence: "Cmd+9 / Ctrl+9".into() },
                ShortcutEntry { command_id: "search".into(), description: "Search Messages".into(), key_sequence: "Cmd+F / Ctrl+F".into() },
                ShortcutEntry { command_id: "lock".into(), description: "Lock App".into(), key_sequence: "Cmd+Shift+L".into() },
                ShortcutEntry { command_id: "settings".into(), description: "Open Settings".into(), key_sequence: "Cmd+, / Ctrl+,".into() },
                ShortcutEntry { command_id: "close".into(), description: "Close Modal / Exit Select".into(), key_sequence: "Esc".into() },
            ],
            shortcuts_enabled: true,
            search_query: String::new(),
            is_visible: false,
            pending_sync: false,
            palette: TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x3390ec, 14, 12),
            language_id: "en".into(),
            selected_filter: None,
        }
    }

    pub fn new_live(palette: TdesktopThemePalette, language_id: &str) -> Self {
        let mut s = Self::new();
        s.palette = palette;
        s.language_id = language_id.into();
        s.is_visible = true;
        s
    }

    pub fn set_palette(&mut self, palette: TdesktopThemePalette) { self.palette = palette; }
    pub fn set_language(&mut self, language_id: impl Into<String>) { self.language_id = language_id.into(); }
    pub fn toggle_visibility(&mut self) { self.is_visible = !self.is_visible; }
    pub fn toggle_shortcuts_enabled(&mut self) { self.shortcuts_enabled = !self.shortcuts_enabled; self.pending_sync = true; }
    pub fn lng_shortcuts_minimize(&self) -> &'static str { "Minimize Window" }
    pub fn lng_shortcuts_chat_last(&self) -> &'static str { "Jump to Last Chat" }
    pub fn lng_shortcuts_lock(&self) -> &'static str { "Lock App" }
    pub fn lng_shortcuts_chat_first(&self) -> &'static str { "Jump to First Chat" }
    pub fn lng_shortcuts_close(&self) -> &'static str { "Close Window" }
    pub fn lng_shortcuts_search(&self) -> &'static str { "Search" }

    fn localized_description<'a>(sc: &'a ShortcutEntry, lang: &str) -> &'a str {
        if !lang.starts_with("zh") {
            return &sc.description;
        }
        match sc.command_id.as_str() {
            "chat_first" => "跳转到第一个对话",
            "chat_last" => "跳转到最后一个对话",
            "search" => "搜索消息",
            "lock" => "立即锁定应用",
            "settings" => "打开设置弹窗",
            "minimize" => "最小化窗口",
            "close" => "关闭弹窗 / 退出多选",
            _ => &sc.description,
        }
    }
}

impl Default for SettingsShortcutsSections {
    fn default() -> Self { Self::new() }
}

impl Render for SettingsShortcutsSections {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette.clone();
        let lang = self.language_id.clone();
        let q = self.search_query.to_lowercase();
        let shortcuts: Vec<ShortcutEntry> = self.shortcuts.iter()
            .filter(|s| {
                q.is_empty()
                    || s.command_id.to_lowercase().contains(&q)
                    || s.description.to_lowercase().contains(&q)
                    || s.key_sequence.to_lowercase().contains(&q)
            })
            .cloned()
            .collect();

        let filters = [
            ("", tr_str(&lang, "全部", "All"), "All"),
            ("chat", tr_str(&lang, "对话跳转", "Chats"), "Chats"),
            ("cmd", tr_str(&lang, "组合键", "Cmd/Ctrl"), "Ctrl"),
        ];

        settings_container(pal.clone())
            .child(settings_section_header(IconName::SettingsGear, tr_str(&lang, "键盘快捷键", "Keyboard Shortcuts"), Some(tr_str(&lang, "快速对话跳转、全局搜索与锁屏快捷键", "Quick navigation and window bindings")), pal.clone()))
            .child(settings_card(pal.clone()).child(settings_toggle_row(
                IconName::SettingsGear,
                tr_str(&lang, "启用自定义工作区快捷键", "Enable Workspace Shortcuts"),
                None,
                self.shortcuts_enabled,
                pal.clone(),
                Self::toggle_shortcuts_enabled,
                cx,
            )))
            .child(div().flex().flex_row().gap(px(6.0)).children(filters.into_iter().map(|(flt, label, tag)| {
                let active = self.search_query == flt;
                div().px(px(10.0)).py(px(5.0)).rounded(px(6.0)).bg(rgb(if active { pal.accent } else { pal.card_bg })).border_1().border_color(rgb(if active { pal.accent } else { pal.border_fg }))
                    .text_xs().text_color(rgb(if active { pal.active_button_fg } else { pal.text_primary })).cursor_pointer()
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                        this.search_query = flt.to_string();
                        this.selected_filter = if flt.is_empty() { None } else { Some(tag) };
                        this.pending_sync = true;
                        cx.notify();
                    }))
                    .child(label)
            })))
            .child(settings_subsection_title(tr_str(&lang, "已启用快捷键", "Active Bindings"), pal.clone()))
            .child(settings_card(pal.clone()).children(shortcuts.into_iter().map(|sc| {
                let desc = Self::localized_description(&sc, &lang).to_string();
                div().flex().flex_row().justify_between().items_center().px(px(10.0)).py(px(6.0))
                    .child(div().text_sm().text_color(rgb(if self.shortcuts_enabled { pal.text_primary } else { pal.text_secondary })).child(desc))
                    .child(div().px(px(8.0)).py(px(3.0)).rounded(px(5.0)).bg(rgb(pal.input_bg)).border_1().border_color(rgb(pal.border_fg)).text_xs().font_family("monospace").text_color(rgb(pal.text_primary)).child(sc.key_sequence))
            })))
    }
}
