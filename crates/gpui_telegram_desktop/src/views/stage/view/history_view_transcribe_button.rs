//! History View Voice Transcription Button View
//! Faithful upstream C++ extraction:
//! - `history_view_transcribe_button.cpp:L30-L75`
//! - `history_view_transcribe_button.h:L20-L60`

use gpui::*;
use crate::views::stage::reactions::transcribe::{TranscribeButton, TranscribeState, TRANSCRIBE_SIZE_AUDIO};

#[derive(Debug, Clone)]
pub struct HistoryViewTranscribeButtonView {
    pub inner: TranscribeButton,
}

impl HistoryViewTranscribeButtonView {
    pub fn new(message_id: u64, is_round_video: bool, has_premium: bool) -> Self {
        Self {
            inner: TranscribeButton::new(message_id, is_round_video, has_premium),
        }
    }

    pub fn click(&mut self) {
        self.inner.trigger_click();
    }

    pub fn set_text(&mut self, text: String) {
        self.inner.set_transcribed(text);
    }
}

impl Default for HistoryViewTranscribeButtonView {
    fn default() -> Self {
        Self::new(0, false, true)
    }
}

impl Render for HistoryViewTranscribeButtonView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let size = self.inner.button_size();
        let icon = match self.inner.state {
            TranscribeState::Idle => "A->文",
            TranscribeState::Loading => "⏳",
            TranscribeState::Transcribed => if self.inner.is_opened { "▲" } else { "▼" },
            TranscribeState::Failed => "⚠️",
            TranscribeState::PremiumLocked => "🔒",
        };

        div()
            .flex()
            .items_center()
            .justify_center()
            .w(px(size))
            .h(px(size))
            .rounded_full()
            .bg(rgba(0x0000000C))
            .text_size(px(11.0))
            .child(icon)
    }
}
