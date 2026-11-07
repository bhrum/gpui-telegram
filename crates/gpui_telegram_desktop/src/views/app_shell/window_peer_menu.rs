//! WindowPeerMenu: Context menu generator for chats, channels, users, and bots
//! Upstream: Telegram/SourceFiles/window/window_peer_menu.h, window_peer_menu.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerType {
    User,
    Bot,
    Group,
    Channel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerMenuAction {
    ViewProfile,
    MuteNotifications,
    UnmuteNotifications,
    MarkAsRead,
    PinChat,
    UnpinChat,
    AddToFolder,
    ClearHistory,
    DeleteChat,
    BlockUser,
    Report,
}

#[derive(Debug, Clone)]
pub struct WindowPeerMenu {
    pub peer_id: u64,
    pub peer_type: PeerType,
    pub is_muted: bool,
    pub is_pinned: bool,
    pub is_visible: bool,
}

impl WindowPeerMenu {
    pub fn new() -> Self {
        Self {
            peer_id: 0,
            peer_type: PeerType::User,
            is_muted: false,
            is_pinned: false,
            is_visible: false,
        }
    }

    pub fn for_peer(peer_id: u64, peer_type: PeerType, is_muted: bool, is_pinned: bool) -> Self {
        Self {
            peer_id,
            peer_type,
            is_muted,
            is_pinned,
            is_visible: true,
        }
    }

    pub fn available_actions(&self) -> Vec<PeerMenuAction> {
        let mut actions = vec![PeerMenuAction::ViewProfile];
        if self.is_muted {
            actions.push(PeerMenuAction::UnmuteNotifications);
        } else {
            actions.push(PeerMenuAction::MuteNotifications);
        }
        actions.push(PeerMenuAction::MarkAsRead);
        if self.is_pinned {
            actions.push(PeerMenuAction::UnpinChat);
        } else {
            actions.push(PeerMenuAction::PinChat);
        }
        actions.push(PeerMenuAction::AddToFolder);
        actions.push(PeerMenuAction::ClearHistory);
        actions.push(PeerMenuAction::DeleteChat);
        actions
    }

    pub fn dismiss(&mut self) {
        self.is_visible = false;
    }
}

impl Default for WindowPeerMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowPeerMenu {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }
        div()
            .flex()
            .flex_col()
            .w(px(200.0))
            .bg(rgb(0x17212B))
            .rounded(px(6.0))
            .shadow_md()
            .border_1()
            .border_color(rgb(0x242F3D))
            .children(self.available_actions().iter().map(|act| {
                div()
                    .px(px(12.0))
                    .py(px(8.0))
                    .text_xs()
                    .text_color(rgb(0xFFFFFF))
                    .child(format!("{:?}", act))
            }))
            .into_any_element()
    }
}
