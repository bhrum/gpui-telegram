//! MenuMarkAsRead: Context action to mark a chat, channel, or folder as read / unread
//! Upstream: Telegram/SourceFiles/menu/menu_mark_as_read.h, menu_mark_as_read.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct MenuMarkAsRead {
    pub peer_id: u64,
    pub mark_as_unread: bool,
}

impl MenuMarkAsRead {
    pub fn new(peer_id: u64, mark_as_unread: bool) -> Self {
        Self {
            peer_id,
            mark_as_unread,
        }
    }

    pub fn action_label(&self) -> &'static str {
        if self.mark_as_unread {
            "Mark as unread"
        } else {
            "Mark as read"
        }
    }
}

impl Default for MenuMarkAsRead {
    fn default() -> Self {
        Self::new(0, false)
    }
}

impl Render for MenuMarkAsRead {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p(px(8.0))
            .text_xs()
            .text_color(rgb(0xFFFFFF))
            .child(self.action_label())
    }
}
