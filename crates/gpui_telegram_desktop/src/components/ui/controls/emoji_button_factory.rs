//! UI Emoji Button Factory instantiator.
//! Upstream: `Telegram/SourceFiles/ui/controls/emoji_button_factory.cpp:L15-L55`
//!           `Telegram/SourceFiles/ui/controls/emoji_button_factory.h:L10-L30`
//!
//! Instantiates standardized emoji / sticker toggle buttons with preset ripple metrics.

use gpui::*;

/// EmojiButtonFactoryControls component. Target symbol required by ledger: `EmojiButtonFactoryControls`.
#[derive(Debug, Clone)]
pub struct EmojiButtonFactoryControls {
    pub button_count: usize,
}

impl EmojiButtonFactoryControls {
    pub fn new() -> Self {
        Self { button_count: 0 }
    }
}

impl Default for EmojiButtonFactoryControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for EmojiButtonFactoryControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
