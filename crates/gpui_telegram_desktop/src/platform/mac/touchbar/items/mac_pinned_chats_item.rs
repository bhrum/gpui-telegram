//! macOS TouchBar Pinned Chats Strip Item
//! Upstream Source: Telegram/SourceFiles/platform/mac/touchbar/items/mac_pinned_chats_item.h, Telegram/SourceFiles/platform/mac/touchbar/items/mac_pinned_chats_item.mm
//! Subsystem: platform | Feature ID: TCG-N17 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// Pinned chat entry displayed on the macOS TouchBar strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinnedChatEntry {
    pub peer_id: u64,
    pub title: String,
    pub is_self: bool,
    pub is_replies: bool,
    pub is_online: bool,
    pub unread_count: u32,
}

/// Interactive horizontal pinned chats panel and GPUI Render view for TouchBar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacPinnedChatsItemMacTouchbarItems {
    pub chats: Vec<PinnedChatEntry>,
    pub selected_peer_id: Option<u64>,
    pub circle_diameter: i32,
    pub button_spacing: i32,
    pub online_circle_size: i32,
    pub unread_badge_size: i32,
}

impl MacPinnedChatsItemMacTouchbarItems {
    pub const CIRCLE_DIAMETER: i32 = 30;
    pub const PINNED_BUTTONS_SPACE: i32 = 30;
    pub const ONLINE_CIRCLE_SIZE: i32 = 8;
    pub const UNREAD_BADGE_SIZE: i32 = 15;

    pub fn new() -> Self {
        Self {
            chats: Vec::new(),
            selected_peer_id: None,
            circle_diameter: Self::CIRCLE_DIAMETER,
            button_spacing: Self::PINNED_BUTTONS_SPACE,
            online_circle_size: Self::ONLINE_CIRCLE_SIZE,
            unread_badge_size: Self::UNREAD_BADGE_SIZE,
        }
    }

    pub fn add_pinned_chat(&mut self, chat: PinnedChatEntry) -> Result<()> {
        self.chats.retain(|c| c.peer_id != chat.peer_id);
        self.chats.push(chat);
        Ok(())
    }

    pub fn select_chat(&mut self, peer_id: u64) -> Result<bool> {
        if self.chats.iter().any(|c| c.peer_id == peer_id) {
            self.selected_peer_id = Some(peer_id);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn remove_chat(&mut self, peer_id: u64) -> Result<()> {
        self.chats.retain(|c| c.peer_id != peer_id);
        if self.selected_peer_id == Some(peer_id) {
            self.selected_peer_id = None;
        }
        Ok(())
    }
}

impl Default for MacPinnedChatsItemMacTouchbarItems {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MacPinnedChatsItemMacTouchbarItems {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.selected_peer_id;

        div()
            .h(px(30.0))
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .px_2()
            .bg(rgb(0x1e1e1e))
            .rounded_md()
            .children(self.chats.iter().map(|chat| {
                let is_sel = selected == Some(chat.peer_id);
                let bg_color = if is_sel {
                    rgb(0x2b5278)
                } else if chat.is_self {
                    rgb(0x5288c1)
                } else {
                    rgb(0x3a3a3a)
                };

                let initial = chat.title.chars().next().unwrap_or('?').to_string();

                let mut bubble = div()
                    .relative()
                    .w(px(30.0))
                    .h(px(30.0))
                    .rounded_full()
                    .bg(bg_color)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_xs()
                    .text_color(rgb(0xffffff))
                    .font_weight(FontWeight::BOLD)
                    .child(initial);

                if chat.is_online {
                    bubble = bubble.child(
                        div()
                            .absolute()
                            .bottom_0()
                            .right_0()
                            .w(px(8.0))
                            .h(px(8.0))
                            .rounded_full()
                            .bg(rgb(0x40a7e3))
                            .border_1()
                            .border_color(rgb(0x1e1e1e)),
                    );
                }

                if chat.unread_count > 0 {
                    bubble = bubble.child(
                        div()
                            .absolute()
                            .top_0()
                            .right_0()
                            .px_1()
                            .h(px(12.0))
                            .rounded_full()
                            .bg(rgb(0xe53935))
                            .text_color(rgb(0xffffff))
                            .text_xs()
                            .child(chat.unread_count.to_string()),
                    );
                }
                bubble
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinned_chats_strip() {
        let mut strip = MacPinnedChatsItemMacTouchbarItems::new();
        assert_eq!(strip.circle_diameter, 30);

        let chat1 = PinnedChatEntry {
            peer_id: 1001,
            title: "Saved Messages".into(),
            is_self: true,
            is_replies: false,
            is_online: false,
            unread_count: 0,
        };
        let chat2 = PinnedChatEntry {
            peer_id: 1002,
            title: "Bob".into(),
            is_self: false,
            is_replies: false,
            is_online: true,
            unread_count: 3,
        };

        strip.add_pinned_chat(chat1).unwrap();
        strip.add_pinned_chat(chat2).unwrap();
        assert_eq!(strip.chats.len(), 2);

        assert!(strip.select_chat(1002).unwrap());
        assert_eq!(strip.selected_peer_id, Some(1002));
    }
}
