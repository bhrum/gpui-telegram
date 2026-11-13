//! UI Silent Toggle channel broadcast bell button.
//! Upstream: `Telegram/SourceFiles/ui/controls/silent_toggle.cpp:L20-L38, L40-L54, L82-L86`
//!           `Telegram/SourceFiles/ui/controls/silent_toggle.h:L15-L50`
//!
//! Toggles silent notifications for messages posted to broadcast channels.

use gpui::*;

/// SilentToggleControls component. Target symbol required by ledger: `SilentToggleControls`.
#[derive(Debug, Clone)]
pub struct SilentToggleControls {
    pub is_silent: bool,
    pub is_hovered: bool,
    pub size: f32,
}

impl SilentToggleControls {
    pub fn new() -> Self {
        Self {
            is_silent: false,
            is_hovered: false,
            size: 32.0,
        }
    }

    pub fn toggle(&mut self) {
        self.is_silent = !self.is_silent;
    }

    pub fn tooltip_text(&self) -> &'static str {
        if self.is_silent {
            "Subscribers will not be notified"
        } else {
            "Subscribers will be notified"
        }
    }
}

impl Default for SilentToggleControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SilentToggleControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let icon = if self.is_silent { "🔕" } else { "🔔" };
        div()
            .w(px(self.size))
            .h(px(self.size))
            .rounded(px(self.size / 2.0))
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(16.0))
            .child(icon)
    }
}
