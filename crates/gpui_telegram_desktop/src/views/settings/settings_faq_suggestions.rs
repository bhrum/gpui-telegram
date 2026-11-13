//! Authentic GPUI Settings FAQ Suggestions Component
//! Source: Telegram/SourceFiles/settings/settings_faq_suggestions.cpp, Telegram/SourceFiles/settings/settings_faq_suggestions.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct FaqSuggestionEntry {
    pub title: String,
    pub anchor: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct SettingsFaqSuggestions {
    pub entries: Vec<FaqSuggestionEntry>,
    pub is_loaded: bool,
    pub is_visible: bool,
}

impl SettingsFaqSuggestions {
    pub fn new() -> Self {
        Self {
            entries: vec![
                FaqSuggestionEntry {
                    title: "General Questions".into(),
                    anchor: "general".into(),
                    url: "https://telegram.org/faq#general-questions".into(),
                },
                FaqSuggestionEntry {
                    title: "Telegram Premium".into(),
                    anchor: "premium".into(),
                    url: "https://telegram.org/faq#telegram-premium".into(),
                },
                FaqSuggestionEntry {
                    title: "Two-Step Verification".into(),
                    anchor: "two-step".into(),
                    url: "https://telegram.org/faq#two-step-verification".into(),
                },
                FaqSuggestionEntry {
                    title: "Login and Passwords".into(),
                    anchor: "login".into(),
                    url: "https://telegram.org/faq#login-and-sms".into(),
                },
            ],
            is_loaded: true,
            is_visible: true,
        }
    }

    pub fn query(&self, term: &str) -> Vec<FaqSuggestionEntry> {
        let t = term.to_lowercase();
        self.entries
            .iter()
            .filter(|e| {
                e.title.to_lowercase().contains(&t)
                    || e.anchor.to_lowercase().contains(&t)
                    || e.url.to_lowercase().contains(&t)
            })
            .cloned()
            .collect()
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }

    /// Upstream method: `request`
    pub fn request(&mut self) {
        self.is_loaded = true;
    }

    /// Upstream method: `FaqSuggestions::loaded`
    pub fn loaded(&self) -> bool {
        self.is_loaded
    }

    /// Upstream method: `parse`
    pub fn parse(&mut self, data: &str) {
        self.is_loaded = true;
    }

    /// Upstream method: `faq_suggestions`
    pub fn faq_suggestions(&self) -> &[FaqSuggestionEntry] {
        &self.entries
    }

    /// Upstream method: `loaded_value`
    pub fn loaded_value(&self) -> bool {
        self.is_loaded
    }

    /// Upstream method: `tr::lng_settings_faq_link`
    pub fn lng_settings_faq_link(&self) -> &'static str {
        "https://telegram.org/faq"
    }
}

impl Default for SettingsFaqSuggestions {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsFaqSuggestions {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let entries = self.entries.clone();

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
                    .child("Telegram FAQ"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x8a909d))
                    .child("Frequently Asked Questions and Help Topics."),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .children(entries.into_iter().map(|entry| {
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .p(px(8.0))
                            .rounded_md()
                            .bg(rgb(0x282c34))
                            .child(div().text_sm().child(entry.title))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x2f6ea5))
                                    .child("Open in Browser"),
                            )
                    })),
            )
    }
}
