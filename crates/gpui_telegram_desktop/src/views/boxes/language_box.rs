//! Authentic GPUI implementation of Telegram Desktop's Language Selection Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/language_box.h:L26-L58`
//! Upstream implementation: `Telegram/SourceFiles/boxes/language_box.cpp:L62-L120`, `L1500-L1539`, and `ui/boxes/choose_language_box.cpp:L24-L65`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

pub const K_LANGUAGE_NAME_PREFIX: &str = "cloud_lng_language_";
pub const K_TRANSLATE_TO_PREFIX: &str = "cloud_lng_translate_to_";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageItem {
    pub id: String,
    pub name: String,
    pub native_name: String,
    pub is_official: bool,
    pub translated_percent: u32,
}

#[derive(Clone)]
pub struct LanguageBox {
    pub current_language_id: String,
    pub search_query: String,
    pub languages: Vec<LanguageItem>,
    pub show_translate_button: bool,
    pub translate_entire_chats: bool,
    pub do_not_translate: Vec<String>,
    pub is_closed: bool,
    pub on_switch: Option<Arc<dyn Fn(String) + Send + Sync>>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl LanguageBox {
    pub fn new(current_id: impl Into<String>) -> Self {
        Self {
            current_language_id: current_id.into(),
            search_query: String::new(),
            languages: Self::default_languages(),
            show_translate_button: true,
            translate_entire_chats: false,
            do_not_translate: vec!["en".into()],
            is_closed: false,
            on_switch: None,
            on_close: None,
        }
    }

    pub fn default_languages() -> Vec<LanguageItem> {
        vec![
            LanguageItem { id: "en".into(), name: "English".into(), native_name: "English".into(), is_official: true, translated_percent: 100 },
            LanguageItem { id: "ru".into(), name: "Russian".into(), native_name: "Русский".into(), is_official: true, translated_percent: 100 },
            LanguageItem { id: "zh-CN".into(), name: "Chinese (Simplified)".into(), native_name: "简体中文".into(), is_official: true, translated_percent: 99 },
            LanguageItem { id: "es".into(), name: "Spanish".into(), native_name: "Español".into(), is_official: true, translated_percent: 100 },
            LanguageItem { id: "de".into(), name: "German".into(), native_name: "Deutsch".into(), is_official: true, translated_percent: 100 },
            LanguageItem { id: "fr".into(), name: "French".into(), native_name: "Français".into(), is_official: true, translated_percent: 100 },
            LanguageItem { id: "it".into(), name: "Italian".into(), native_name: "Italiano".into(), is_official: true, translated_percent: 100 },
            LanguageItem { id: "ja".into(), name: "Japanese".into(), native_name: "日本語".into(), is_official: true, translated_percent: 98 },
            LanguageItem { id: "ko".into(), name: "Korean".into(), native_name: "한국어".into(), is_official: true, translated_percent: 97 },
        ]
    }

    pub fn filter_languages(&self) -> Vec<LanguageItem> {
        if self.search_query.is_empty() {
            self.languages.clone()
        } else {
            let q = self.search_query.to_lowercase();
            self.languages.iter().filter(|l| l.name.to_lowercase().contains(&q) || l.native_name.to_lowercase().contains(&q) || l.id.to_lowercase().contains(&q)).cloned().collect()
        }
    }

    pub fn switch_language(&mut self, id: String) {
        self.current_language_id = id.clone();
        if let Some(ref cb) = self.on_switch {
            cb(id);
        }
        self.close();
    }

    pub fn toggle_translate_button(&mut self) {
        self.show_translate_button = !self.show_translate_button;
    }

    pub fn close(&mut self) {
        self.is_closed = true;
        if let Some(ref cb) = self.on_close {
            cb();
        }
    }
}

impl Default for LanguageBox {
    fn default() -> Self {
        Self::new("en")
    }
}

impl Render for LanguageBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        let filtered = self.filter_languages();

        div()
            .id("language_box")
            .w(px(380.0))
            .p(px(20.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(12.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x222222))
                            .child("Language"),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .text_size(px(14.0))
                            .text_color(rgb(0x888888))
                            .child("✕"),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .p(px(8.0))
                    .rounded(px(6.0))
                    .bg(rgb(0xF1F3F5))
                    .text_size(px(13.0))
                    .text_color(rgb(0x888888))
                    .child(if self.search_query.is_empty() { "Search languages...".to_string() } else { self.search_query.clone() }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .max_h(px(220.0))
                    .overflow_hidden()
                    .children(filtered.iter().map(|lang| {
                        let is_cur = lang.id == self.current_language_id;
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .p(px(8.0))
                            .rounded(px(6.0))
                            .bg(if is_cur { rgb(0xEEF8FF) } else { rgb(0xFFFFFF) })
                            .cursor_pointer()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().text_size(px(13.0)).font_weight(FontWeight::MEDIUM).text_color(rgb(0x222222)).child(lang.native_name.clone()))
                                    .child(div().text_size(px(11.0)).text_color(rgb(0x888888)).child(lang.name.clone())),
                            )
                            .child(if is_cur {
                                div().text_size(px(14.0)).text_color(rgb(0x2AABEE)).child("✓")
                            } else {
                                div()
                            })
                    })),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .pt(px(8.0))
                    .border_t_1()
                    .border_color(rgb(0xEEEEEE))
                    .child(div().text_size(px(12.0)).text_color(rgb(0x444444)).child("Show Translate Button in Chats"))
                    .child(
                        div()
                            .size(px(16.0))
                            .rounded(px(4.0))
                            .border_1()
                            .border_color(rgb(0x2AABEE))
                            .bg(if self.show_translate_button { rgb(0x2AABEE) } else { rgb(0xFFFFFF) })
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(if self.show_translate_button { div().text_size(px(11.0)).text_color(rgb(0xFFFFFF)).child("✓") } else { div() }),
                    ),
            )
            .into_any_element()
    }
}
