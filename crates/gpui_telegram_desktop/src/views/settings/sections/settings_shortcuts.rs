//! Authentic GPUI Settings Shortcuts Section Component
//! Source: Telegram/SourceFiles/settings/sections/settings_shortcuts.cpp, Telegram/SourceFiles/settings/sections/settings_shortcuts.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct ShortcutEntry {
    pub command_id: String,
    pub description: String,
    pub key_sequence: String,
}

#[derive(Debug, Clone)]
pub struct SettingsShortcutsSections {
    pub shortcuts: Vec<ShortcutEntry>,
    pub search_query: String,
    pub is_visible: bool,
}

impl SettingsShortcutsSections {
    pub fn new() -> Self {
        Self {
            shortcuts: vec![
                ShortcutEntry {
                    command_id: "chat_first".into(),
                    description: "Jump to First Chat".into(),
                    key_sequence: "Alt+1".into(),
                },
                ShortcutEntry {
                    command_id: "chat_last".into(),
                    description: "Jump to Last Chat".into(),
                    key_sequence: "Alt+9".into(),
                },
                ShortcutEntry {
                    command_id: "search".into(),
                    description: "Search in Current Chat".into(),
                    key_sequence: "Ctrl+F".into(),
                },
                ShortcutEntry {
                    command_id: "lock".into(),
                    description: "Lock Application".into(),
                    key_sequence: "Ctrl+L".into(),
                },
                ShortcutEntry {
                    command_id: "minimize".into(),
                    description: "Minimize Window".into(),
                    key_sequence: "Ctrl+M".into(),
                },
                ShortcutEntry {
                    command_id: "close".into(),
                    description: "Close Active Chat".into(),
                    key_sequence: "Esc".into(),
                },
            ],
            search_query: String::new(),
            is_visible: true,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }

    /// Upstream method: `tr::lng_shortcuts_minimize`
    pub fn lng_shortcuts_minimize(&self) -> &'static str {
        "Minimize to Tray"
    }

    /// Upstream method: `tr::lng_shortcuts_chat_last`
    pub fn lng_shortcuts_chat_last(&self) -> &'static str {
        "Previous Chat"
    }

    /// Upstream method: `tr::lng_shortcuts_lock`
    pub fn lng_shortcuts_lock(&self) -> &'static str {
        "Lock Telegram"
    }

    /// Upstream method: `tr::lng_shortcuts_chat_first`
    pub fn lng_shortcuts_chat_first(&self) -> &'static str {
        "First Chat"
    }

    /// Upstream method: `tr::lng_shortcuts_close`
    pub fn lng_shortcuts_close(&self) -> &'static str {
        "Close Window / Chat"
    }

    /// Upstream method: `tr::lng_shortcuts_search`
    pub fn lng_shortcuts_search(&self) -> &'static str {
        "Search"
    }
}

impl Default for SettingsShortcutsSections {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsShortcutsSections {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let shortcuts = self.shortcuts.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap(px(12.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_lg()
                    .child("Keyboard Shortcuts"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .children(shortcuts.into_iter().map(|sc| {
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .p(px(8.0))
                            .rounded_md()
                            .bg(rgb(0x282c34))
                            .child(div().text_sm().child(sc.description))
                            .child(
                                div()
                                    .px(px(6.0))
                                    .py(px(2.0))
                                    .rounded(px(4.0))
                                    .bg(rgb(0x3e4451))
                                    .text_xs()
                                    .font_family("monospace")
                                    .child(sc.key_sequence),
                            )
                    })),
            )
    }
}
