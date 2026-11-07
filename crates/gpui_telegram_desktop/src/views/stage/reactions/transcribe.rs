//! Voice & Video Message Transcription Button State Machine
//! Faithful upstream C++ extraction:
//! - `history_view_transcribe_button.cpp:L30-L75`

use gpui::*;

pub const K_IN_NON_CHOSEN_OPACITY: f32 = 0.12;
pub const K_OUT_NON_CHOSEN_OPACITY: f32 = 0.18;
pub const K_ARROW_PIVOT_NEAR: f32 = 0.349;
pub const K_ARROW_PIVOT_FAR: f32 = 1.0 - K_ARROW_PIVOT_NEAR; // 0.651

pub const TRANSCRIBE_SIZE_AUDIO: f32 = 30.0;
pub const TRANSCRIBE_SIZE_ROUND_VIDEO: f32 = 36.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TranscribeState {
    #[default]
    Idle,
    Loading,
    Transcribed,
    Failed,
    PremiumLocked,
}

#[derive(Debug, Clone)]
pub struct TranscribeButton {
    pub message_id: u64,
    pub is_round_video: bool,
    pub state: TranscribeState,
    pub is_opened: bool,
    pub transcript_text: Option<String>,
    pub loading_progress: f32,
    pub has_premium: bool,
}

impl TranscribeButton {
    pub fn new(message_id: u64, is_round_video: bool, has_premium: bool) -> Self {
        Self {
            message_id,
            is_round_video,
            state: TranscribeState::Idle,
            is_opened: false,
            transcript_text: None,
            loading_progress: 0.0,
            has_premium,
        }
    }

    pub fn button_size(&self) -> f32 {
        if self.is_round_video {
            TRANSCRIBE_SIZE_ROUND_VIDEO
        } else {
            TRANSCRIBE_SIZE_AUDIO
        }
    }

    pub fn trigger_click(&mut self) {
        match self.state {
            TranscribeState::Idle => {
                if !self.has_premium {
                    self.state = TranscribeState::PremiumLocked;
                } else {
                    self.state = TranscribeState::Loading;
                    self.loading_progress = 0.1;
                }
            }
            TranscribeState::Loading => {
                // Cancel
                self.state = TranscribeState::Idle;
            }
            TranscribeState::Transcribed => {
                // Toggle expand/collapse
                self.is_opened = !self.is_opened;
            }
            TranscribeState::Failed => {
                // Retry
                self.state = TranscribeState::Loading;
            }
            TranscribeState::PremiumLocked => {
                // Prompt premium preview
            }
        }
    }

    pub fn set_transcribed(&mut self, text: String) {
        self.transcript_text = Some(text);
        self.state = TranscribeState::Transcribed;
        self.is_opened = true;
    }

    pub fn set_failed(&mut self) {
        self.state = TranscribeState::Failed;
    }
}

impl Default for TranscribeButton {
    fn default() -> Self {
        Self::new(0, false, true)
    }
}

impl Render for TranscribeButton {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let size = self.button_size();
        let icon = match self.state {
            TranscribeState::Idle => "A->文",
            TranscribeState::Loading => "⏳",
            TranscribeState::Transcribed => if self.is_opened { "▲" } else { "▼" },
            TranscribeState::Failed => "⚠️",
            TranscribeState::PremiumLocked => "🔒",
        };

        let mut root = div()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(size))
                    .h(px(size))
                    .rounded_full()
                    .bg(match self.state {
                        TranscribeState::Loading => rgba(0x2AABEE33),
                        TranscribeState::Transcribed => rgba(0x2AABEE22),
                        TranscribeState::PremiumLocked => rgba(0x88888822),
                        _ => rgba(0x00000010),
                    })
                    .hover(|s| s.bg(rgba(0x00000020)))
                    .text_size(px(11.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(if self.state == TranscribeState::Transcribed {
                        rgb(0x2AABEE)
                    } else {
                        rgb(0x555555)
                    })
                    .child(icon),
            );

        if self.is_opened {
            if let Some(text) = &self.transcript_text {
                root = root.child(
                    div()
                        .mt(px(4.0))
                        .p(px(8.0))
                        .bg(rgba(0x0000000A))
                        .rounded(px(8.0))
                        .text_size(px(13.0))
                        .text_color(rgb(0x222222))
                        .child(text.clone()),
                );
            }
        }

        root
    }
}
