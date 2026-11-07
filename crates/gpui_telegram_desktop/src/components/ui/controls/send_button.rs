//! UI Send Button multi-mode composer action button.
//! Upstream: `Telegram/SourceFiles/ui/controls/send_button.cpp:L24-L28, L66-L83, L91-L111`
//!           `Telegram/SourceFiles/ui/controls/send_button.h:L26-L120`
//!
//! Handles transitions between message send, voice record, video note, scheduled post,
//! and slowmode cooldown delay timers.

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendButtonType {
    Send,
    Stop,
    Schedule,
    Save,
    Record,
    Round,
    Cancel,
    Slowmode,
    EditPrice,
}

pub fn format_slowmode_delay(seconds: i32) -> String {
    let minutes = seconds / 60;
    let secs = seconds % 60;
    format!("{}:{:02}", minutes, secs)
}

/// SendButtonControls component. Target symbol required by ledger: `SendButtonControls`.
#[derive(Debug, Clone)]
pub struct SendButtonControls {
    pub button_type: SendButtonType,
    pub slowmode_delay_seconds: i32,
    pub stars_to_send: i32,
    pub is_forbidden: bool,
    pub size: f32,
}

impl SendButtonControls {
    pub fn new() -> Self {
        Self {
            button_type: SendButtonType::Send,
            slowmode_delay_seconds: 0,
            stars_to_send: 0,
            is_forbidden: false,
            size: 36.0,
        }
    }

    pub fn with_type(mut self, bt: SendButtonType) -> Self {
        self.button_type = bt;
        self
    }

    pub fn with_slowmode(mut self, seconds: i32) -> Self {
        self.slowmode_delay_seconds = seconds;
        self.button_type = if seconds > 0 { SendButtonType::Slowmode } else { SendButtonType::Send };
        self
    }

    pub fn icon_text(&self) -> String {
        match self.button_type {
            SendButtonType::Send => "➤".to_string(),
            SendButtonType::Record => "🎤".to_string(),
            SendButtonType::Round => "📹".to_string(),
            SendButtonType::Stop => "⏹".to_string(),
            SendButtonType::Schedule => "📅".to_string(),
            SendButtonType::Save => "💾".to_string(),
            SendButtonType::Cancel => "✕".to_string(),
            SendButtonType::Slowmode => format_slowmode_delay(self.slowmode_delay_seconds),
            SendButtonType::EditPrice => "🏷".to_string(),
        }
    }
}

impl Default for SendButtonControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SendButtonControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_slowmode = self.button_type == SendButtonType::Slowmode;
        let bg = if is_slowmode {
            hsla(0.0, 0.0, 0.75, 1.0)
        } else {
            hsla(204.0 / 360.0, 0.88, 0.53, 1.0)
        };

        div()
            .w(px(self.size))
            .h(px(self.size))
            .rounded(px(self.size / 2.0))
            .bg(bg)
            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
            .text_size(if is_slowmode { px(11.0) } else { px(16.0) })
            .flex()
            .items_center()
            .justify_center()
            .child(self.icon_text())
    }
}
