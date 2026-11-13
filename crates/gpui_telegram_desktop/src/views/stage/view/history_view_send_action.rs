//! History View Typing & Send Action Indicator
//! Faithful upstream C++ extraction:
//! - `history_view_send_action.cpp:L30-L100`
//! - `history_view_send_action.h:L20-L45`

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SendActionType {
    #[default]
    Typing,
    RecordVoice,
    UploadPhoto,
    UploadVideo,
    UploadFile,
    ChooseSticker,
}

#[derive(Debug, Clone)]
pub struct HistoryViewSendActionView {
    pub peer_id: u64,
    pub user_name: String,
    pub action_type: SendActionType,
    pub is_active: bool,
    pub animation_frame: usize,
}

impl HistoryViewSendActionView {
    pub fn new(peer_id: u64, user_name: String, action_type: SendActionType) -> Self {
        Self {
            peer_id,
            user_name,
            action_type,
            is_active: true,
            animation_frame: 0,
        }
    }

    pub fn description(&self) -> String {
        let action = match self.action_type {
            SendActionType::Typing => "typing",
            SendActionType::RecordVoice => "recording voice note",
            SendActionType::UploadPhoto => "sending photo",
            SendActionType::UploadVideo => "sending video",
            SendActionType::UploadFile => "sending document",
            SendActionType::ChooseSticker => "choosing sticker",
        };
        format!("{} is {}...", self.user_name, action)
    }

    pub fn tick_animation(&mut self) {
        self.animation_frame = (self.animation_frame + 1) % 4;
    }
}

impl Default for HistoryViewSendActionView {
    fn default() -> Self {
        Self::new(0, "User".into(), SendActionType::Typing)
    }
}

impl Render for HistoryViewSendActionView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_active {
            return div().into_any_element();
        }

        let desc = self.description();
        let dots = match self.animation_frame {
            0 => ".  ",
            1 => ".. ",
            2 => "...",
            _ => "   ",
        };

        div()
            .flex()
            .items_center()
            .gap(px(4.0))
            .text_size(px(12.0))
            .text_color(rgb(0x2AABEE))
            .child(desc)
            .child(dots)
            .into_any_element()
    }
}
