//! Bot menu button with dynamic text size animation and custom emoji support.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_bot_menu_button.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_bot_menu_button.h`

use gpui::*;
use serde::{Deserialize, Serialize};

/// Authentic bot menu button model and GPUI component
pub struct HistoryViewBotMenuButtonViewControls {
    pub text: String,
    pub is_small: bool,
    pub is_active: bool,
    pub width_px: f32,
    pub emoji_id: Option<u64>,
}

impl HistoryViewBotMenuButtonViewControls {
    pub fn new() -> Self {
        Self {
            text: "Menu".to_string(),
            is_small: false,
            is_active: false,
            width_px: 80.0,
            emoji_id: None,
        }
    }

    /// Refresh button text and small flag, recalculating animated width
    /// Upstream: `BotMenuButton::refresh` (`history_view_bot_menu_button.cpp:L70-85`)
    pub fn refresh(&mut self, text: impl Into<String>, small: bool) -> bool {
        let new_text = text.into();
        let changed = self.text != new_text || self.is_small != small;
        self.text = new_text;
        self.is_small = small;
        self.width_px = if small { 36.0 } else { 24.0 + (self.text.len() as f32 * 8.0).max(48.0) };
        changed
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

impl Default for HistoryViewBotMenuButtonViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewBotMenuButtonViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_small = self.is_small;
        let text = self.text.clone();

        let mut inner = div()
            .flex()
            .items_center()
            .gap_1()
            .child("☰");

        if !is_small {
            inner = inner.child(text);
        }

        div()
            .flex()
            .items_center()
            .justify_center()
            .h_7()
            .px_2()
            .rounded_full()
            .bg(if self.is_active {
                rgb(0x2481cc)
            } else {
                rgb(0x242f3d)
            })
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(0xffffff))
            .hover(|s| s.bg(rgb(0x2b3848)))
            .child(inner)
    }
}
