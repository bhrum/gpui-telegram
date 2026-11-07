//! UI Call Button rounded action button.
//! Upstream: `Telegram/SourceFiles/ui/controls/call_button.cpp:L25-L80`
//!           `Telegram/SourceFiles/ui/controls/call_button.h:L15-L65`
//!
//! Round button for accepting, hanging up, or muting audio/video calls.

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallButtonType {
    Accept,
    Decline,
    Mute,
    Camera,
}

/// CallButtonControls component. Target symbol required by ledger: `CallButtonControls`.
#[derive(Debug, Clone)]
pub struct CallButtonControls {
    pub button_type: CallButtonType,
    pub size: f32,
    pub is_active: bool,
}

impl CallButtonControls {
    pub fn new(button_type: CallButtonType) -> Self {
        Self {
            button_type,
            size: 52.0,
            is_active: false,
        }
    }
}

impl Default for CallButtonControls {
    fn default() -> Self {
        Self::new(CallButtonType::Accept)
    }
}

impl Render for CallButtonControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (bg, icon) = match self.button_type {
            CallButtonType::Accept => (hsla(140.0 / 360.0, 0.70, 0.45, 1.0), "📞"),
            CallButtonType::Decline => (hsla(0.0 / 360.0, 0.75, 0.50, 1.0), "📴"),
            CallButtonType::Mute => (hsla(0.0, 0.0, 0.35, 1.0), "🎤"),
            CallButtonType::Camera => (hsla(0.0, 0.0, 0.35, 1.0), "📹"),
        };

        div()
            .w(px(self.size))
            .h(px(self.size))
            .rounded(px(self.size / 2.0))
            .bg(bg)
            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
            .flex()
            .items_center()
            .justify_center()
            .child(icon)
    }
}
