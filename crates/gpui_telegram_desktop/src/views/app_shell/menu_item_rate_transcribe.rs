//! MenuItemRateTranscribe: Quality rating action for voice-to-text transcriptions
//! Upstream: Telegram/SourceFiles/menu/menu_item_rate_transcribe.h, menu_item_rate_transcribe.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct MenuItemRateTranscribe {
    pub message_id: i64,
    pub is_positive: Option<bool>,
}

impl MenuItemRateTranscribe {
    pub fn new(message_id: i64) -> Self {
        Self {
            message_id,
            is_positive: None,
        }
    }

    pub fn rate(&mut self, positive: bool) {
        self.is_positive = Some(positive);
    }
}

impl Default for MenuItemRateTranscribe {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for MenuItemRateTranscribe {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("RateTranscribe: msg={}, rating={:?}", self.message_id, self.is_positive))
    }
}
