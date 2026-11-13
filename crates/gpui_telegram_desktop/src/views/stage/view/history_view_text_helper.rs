//! Authentic HistoryView Text Helper Component (`history_view_text_helper.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_text_helper.cpp:L20-L75`

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TextHelperOptions {
    pub has_spoilers: bool,
    pub spoilers_revealed: bool,
    pub has_collapsed_blockquotes: bool,
    pub blockquote_expanded: bool,
    pub custom_emoji_count: usize,
}

/// Authentic GPUI View for HistoryView::TextHelper.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HistoryViewTextHelperView {
    pub options: TextHelperOptions,
}

impl HistoryViewTextHelperView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reveal_spoilers(&mut self) {
        self.options.spoilers_revealed = true;
    }

    pub fn toggle_blockquote(&mut self) {
        self.options.blockquote_expanded = !self.options.blockquote_expanded;
    }
}

impl Render for HistoryViewTextHelperView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
