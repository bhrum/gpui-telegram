//! History Real-time Translation Manager.
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/history/history_translation.h:L14-L32`
//! - Upstream: `Telegram/SourceFiles/history/history_translation.cpp:L14-L51`
//! - Flags: `Data::HistoryUpdate::Flag::TranslateFrom`, `Flag::TranslatedTo` (history_translation.cpp:L16)

use gpui::*;

/// Update flag corresponding to upstream `Data::HistoryUpdate::Flag`.
/// C++ citation: `history_translation.cpp:L16`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryTranslationUpdateFlag {
    TranslateFrom,
    TranslatedTo,
}

/// History real-time translation state and language routing.
/// C++ equivalent: `HistoryTranslation` (history_translation.h:L14-L32).
pub struct HistoryTranslation {
    offer_from: Option<String>,
    translated_to: Option<String>,
    is_auto_translate_enabled: bool,
    show_translation_bar: bool,
    pending_flags: Vec<HistoryTranslationUpdateFlag>,
}

impl HistoryTranslation {
    pub fn new(offer_from: Option<String>) -> Self {
        let mut s = Self {
            offer_from: None,
            translated_to: None,
            is_auto_translate_enabled: true,
            show_translation_bar: offer_from.is_some(),
            pending_flags: Vec::new(),
        };
        if let Some(lang) = offer_from {
            s.offer_from(lang);
        }
        s
    }

    /// Sets the source language offered for translation.
    /// C++ citation: `history_translation.cpp:L27-L34`.
    pub fn offer_from(&mut self, id: impl Into<String>) {
        let lang = id.into();
        if self.offer_from.as_ref() == Some(&lang) {
            return;
        }
        self.offer_from = Some(lang);
        self.show_translation_bar = true;
        self.pending_flags.push(HistoryTranslationUpdateFlag::TranslateFrom);
    }

    /// Returns the currently offered source language.
    /// C++ citation: `history_translation.cpp:L36-L38`.
    pub fn offered_from(&self) -> Option<&str> {
        self.offer_from.as_deref()
    }

    /// Sets target translation language.
    /// C++ citation: `history_translation.cpp:L40-L47`.
    pub fn translate_to(&mut self, id: impl Into<String>) {
        let lang = id.into();
        if self.translated_to.as_ref() == Some(&lang) {
            return;
        }
        self.translated_to = Some(lang);
        self.pending_flags.push(HistoryTranslationUpdateFlag::TranslatedTo);
    }

    /// Returns the current target translation language.
    /// C++ citation: `history_translation.cpp:L49-L51`.
    pub fn translated_to(&self) -> Option<&str> {
        self.translated_to.as_deref()
    }

    /// Drains any pending update flags dispatched to subscribers.
    pub fn take_pending_flags(&mut self) -> Vec<HistoryTranslationUpdateFlag> {
        std::mem::take(&mut self.pending_flags)
    }

    pub fn set_show_bar(&mut self, show: bool) {
        self.show_translation_bar = show;
    }

    pub fn is_bar_visible(&self) -> bool {
        self.show_translation_bar && self.offer_from.is_some()
    }
}

impl Default for HistoryTranslation {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Render for HistoryTranslation {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_bar_visible() {
            return div().into_any_element();
        }

        let from_text = self.offer_from.as_deref().unwrap_or("Detected language");
        let to_text = self.translated_to.as_deref().unwrap_or("English");

        div()
            .flex()
            .items_center()
            .justify_between()
            .h(px(36.0))
            .px(px(16.0))
            .bg(rgb(0xEEF2FF))
            .border_b_1()
            .border_color(rgb(0xC7D2FE))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .text_size(px(13.0))
                    .text_color(rgb(0x3730A3))
                    .child(format!("Translate from {} to {}", from_text, to_text))
            )
            .child(
                div()
                    .flex()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x4338CA))
                            .cursor_pointer()
                            .child("Show original")
                    )
            )
            .into_any_element()
    }
}
