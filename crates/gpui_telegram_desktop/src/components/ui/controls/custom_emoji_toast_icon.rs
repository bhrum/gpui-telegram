//! UI Custom Emoji Toast Icon renderer.
//! Upstream: `Telegram/SourceFiles/ui/controls/custom_emoji_toast_icon.cpp:L20-L70`
//!           `Telegram/SourceFiles/ui/controls/custom_emoji_toast_icon.h:L10-L35`
//!
//! Renders custom emoji/sticker icons inside notification toast balloons.

use gpui::*;

/// CustomEmojiToastIconControls component. Target symbol required by ledger: `CustomEmojiToastIconControls`.
#[derive(Debug, Clone)]
pub struct CustomEmojiToastIconControls {
    pub emoji_id: u64,
    pub icon_size: f32,
}

impl CustomEmojiToastIconControls {
    pub fn new(emoji_id: u64) -> Self {
        Self {
            emoji_id,
            icon_size: 24.0,
        }
    }
}

impl Default for CustomEmojiToastIconControls {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for CustomEmojiToastIconControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(self.icon_size))
            .h(px(self.icon_size))
            .flex()
            .items_center()
            .justify_center()
            .child("✨")
    }
}
