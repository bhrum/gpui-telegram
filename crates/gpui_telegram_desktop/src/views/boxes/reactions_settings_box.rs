//! Authentic GPUI implementation of Telegram Desktop's Reactions Settings Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/reactions_settings_box.h:L23-L43`
//! Upstream implementation: `Telegram/SourceFiles/boxes/reactions_settings_box.cpp:L44-L60`, `L200-L350`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::sync::Arc;

pub const K_CENTER_SIZE_MULTIPLIER: f64 = 1.6;
pub const K_MAX_LANDING_WAIT_MS: u64 = 3000;

#[derive(Clone)]
pub struct ReactionsSettingsBox {
    pub quick_reaction: String,
    pub available_reactions: Vec<String>,
    pub enable_animations: bool,
    pub is_closed: bool,
    pub on_save: Option<Arc<dyn Fn(String, bool) + Send + Sync>>,
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl ReactionsSettingsBox {
    pub fn new() -> Self {
        Self {
            quick_reaction: "👍".into(),
            available_reactions: vec![
                "👍".into(), "👎".into(), "❤️".into(), "🔥".into(),
                "🎉".into(), "🤩".into(), "😱".into(), "😁".into(),
                "😢".into(), "💩".into(), "🤮".into(), "🙏".into(),
            ],
            enable_animations: true,
            is_closed: false,
            on_save: None,
            on_close: None,
        }
    }

    pub fn set_quick_reaction(&mut self, emoji: impl Into<String>) {
        self.quick_reaction = emoji.into();
    }

    pub fn toggle_animations(&mut self) {
        self.enable_animations = !self.enable_animations;
    }

    pub fn save(&mut self) {
        if let Some(ref cb) = self.on_save {
            cb(self.quick_reaction.clone(), self.enable_animations);
        }
        self.close();
    }

    pub fn close(&mut self) {
        self.is_closed = true;
        if let Some(ref cb) = self.on_close {
            cb();
        }
    }
}

impl Default for ReactionsSettingsBox {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ReactionsSettingsBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        div()
            .id("reactions_settings_box")
            .w(px(380.0))
            .p(px(20.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(12.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(14.0))
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
                            .child("Quick Reaction"),
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
                    .text_size(px(13.0))
                    .text_color(rgb(0x666666))
                    .child("Double-tap or click a message to react with your preferred emoji."),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(8.0))
                    .p(px(8.0))
                    .bg(rgb(0xFAFAFA))
                    .rounded(px(10.0))
                    .children(self.available_reactions.iter().map(|emoji| {
                        let is_quick = emoji == &self.quick_reaction;
                        div()
                            .size(px(46.0))
                            .rounded(px(8.0))
                            .border_1()
                            .border_color(if is_quick { rgb(0x2AABEE) } else { rgb(0xEEEEEE) })
                            .bg(if is_quick { rgb(0xEEF8FF) } else { rgb(0xFFFFFF) })
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(div().text_size(px(24.0)).child(emoji.clone()))
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
                    .child(div().text_size(px(13.0)).text_color(rgb(0x444444)).child("Animated Reaction Effects"))
                    .child(
                        div()
                            .size(px(16.0))
                            .rounded(px(4.0))
                            .border_1()
                            .border_color(rgb(0x2AABEE))
                            .bg(if self.enable_animations { rgb(0x2AABEE) } else { rgb(0xFFFFFF) })
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(if self.enable_animations { div().text_size(px(11.0)).text_color(rgb(0xFFFFFF)).child("✓") } else { div() }),
                    ),
            )
            .into_any_element()
    }
}
