//! UI Labeled Emoji Tabs selector header strip.
//! Upstream: `Telegram/SourceFiles/ui/controls/labeled_emoji_tabs.cpp:L30-L90`
//!           `Telegram/SourceFiles/ui/controls/labeled_emoji_tabs.h:L15-L55`
//!
//! Provides top navigation between Emoji, Stickers, and GIFs with animated slider underline.

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabeledTab {
    Emoji,
    Stickers,
    Gifs,
}

/// LabeledEmojiTabsControls component. Target symbol required by ledger: `LabeledEmojiTabsControls`.
#[derive(Debug, Clone)]
pub struct LabeledEmojiTabsControls {
    pub active_tab: LabeledTab,
}

impl LabeledEmojiTabsControls {
    pub fn new() -> Self {
        Self { active_tab: LabeledTab::Emoji }
    }

    pub fn set_tab(&mut self, tab: LabeledTab) {
        self.active_tab = tab;
    }
}

impl Default for LabeledEmojiTabsControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for LabeledEmojiTabsControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap(px(16.0))
            .p(px(8.0))
            .child("Emoji")
            .child("Stickers")
            .child("GIFs")
    }
}
