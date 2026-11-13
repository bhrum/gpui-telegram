//! UI More Chats bar banner for forum topics and folders.
//! Upstream: `Telegram/SourceFiles/ui/chat/more_chats_bar.cpp:L20-L65`
//!           `Telegram/SourceFiles/ui/chat/more_chats_bar.h:L15-L40`
//!
//! Indicates unread mentions and messages in other channels / topics.

use gpui::*;

/// MoreChatsBarChat component. Target symbol required by ledger: `MoreChatsBarChat`.
#[derive(Debug, Clone)]
pub struct MoreChatsBarChat {
    pub unread_count: usize,
    pub has_mentions: bool,
    pub is_visible: bool,
}

impl MoreChatsBarChat {
    pub fn new(unread_count: usize) -> Self {
        Self {
            unread_count,
            has_mentions: false,
            is_visible: true,
        }
    }

    pub fn with_mentions(mut self, has_mentions: bool) -> Self {
        self.has_mentions = has_mentions;
        self
    }
}

impl Default for MoreChatsBarChat {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for MoreChatsBarChat {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible || self.unread_count == 0 {
            return div();
        }

        div()
            .h(px(32.0))
            .w_full()
            .px(px(12.0))
            .bg(hsla(204.0 / 360.0, 0.85, 0.95, 1.0))
            .flex()
            .items_center()
            .justify_between()
            .text_size(px(12.0))
            .text_color(hsla(204.0 / 360.0, 0.88, 0.40, 1.0))
            .child(format!("{} more chats", self.unread_count))
    }
}
