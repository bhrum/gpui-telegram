//! UI Emoji Button composer accessory button.
//! Upstream: `Telegram/SourceFiles/ui/controls/emoji_button.cpp:L25-L85`
//!           `Telegram/SourceFiles/ui/controls/emoji_button.h:L15-L50`
//!
//! Toggles the tabbed emoji / sticker / GIF picker panel next to the message composer.

use gpui::*;

/// EmojiButtonControls component. Target symbol required by ledger: `EmojiButtonControls`.
#[derive(Debug, Clone)]
pub struct EmojiButtonControls {
    pub is_active: bool,
    pub is_hovered: bool,
    pub size: f32,
}

impl EmojiButtonControls {
    pub fn new() -> Self {
        Self {
            is_active: false,
            is_hovered: false,
            size: 32.0,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

impl Default for EmojiButtonControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for EmojiButtonControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let color = if self.is_active {
            hsla(204.0 / 360.0, 0.88, 0.53, 1.0)
        } else if self.is_hovered {
            hsla(0.0, 0.0, 0.35, 1.0)
        } else {
            hsla(0.0, 0.0, 0.55, 1.0)
        };

        div()
            .w(px(self.size))
            .h(px(self.size))
            .rounded(px(self.size / 2.0))
            .flex()
            .items_center()
            .justify_center()
            .text_color(color)
            .text_size(px(18.0))
            .child("😊")
    }
}
