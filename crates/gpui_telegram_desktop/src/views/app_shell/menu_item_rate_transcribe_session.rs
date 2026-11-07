//! MenuItemRateTranscribeSession: Session state container for submitted transcription feedback
//! Upstream: Telegram/SourceFiles/menu/menu_item_rate_transcribe_session.h, menu_item_rate_transcribe_session.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct MenuItemRateTranscribeSession {
    pub session_id: String,
    pub rated_messages: Vec<(i64, bool)>,
}

impl MenuItemRateTranscribeSession {
    pub fn new() -> Self {
        Self {
            session_id: "transcribe_session".to_string(),
            rated_messages: Vec::new(),
        }
    }

    pub fn record(&mut self, msg_id: i64, rating: bool) {
        self.rated_messages.push((msg_id, rating));
    }

    pub fn is_rated(&self, msg_id: i64) -> bool {
        self.rated_messages.iter().any(|(id, _)| *id == msg_id)
    }
}

impl Default for MenuItemRateTranscribeSession {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MenuItemRateTranscribeSession {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("RateSession: {} feedback entries", self.rated_messages.len()))
    }
}
