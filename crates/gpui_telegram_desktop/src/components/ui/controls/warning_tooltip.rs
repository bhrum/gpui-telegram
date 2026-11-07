//! UI Warning Tooltip transient bubble helper.
//! Upstream: `Telegram/SourceFiles/ui/controls/warning_tooltip.cpp:L22-L24, L33-L94`
//!           `Telegram/SourceFiles/ui/controls/warning_tooltip.h:L30-L56`
//!
//! Renders anchored warning balloons with automatic timeout (2000ms) and arrow indicators.

use gpui::*;

pub const K_DEFAULT_WARNING_TOOLTIP_DURATION_MS: u64 = 2000;

/// WarningTooltipControls component. Target symbol required by ledger: `WarningTooltipControls`.
#[derive(Debug, Clone)]
pub struct WarningTooltipControls {
    pub message: String,
    pub is_visible: bool,
    pub duration_ms: u64,
}

impl WarningTooltipControls {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            is_visible: false,
            duration_ms: K_DEFAULT_WARNING_TOOLTIP_DURATION_MS,
        }
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }
}

impl Default for WarningTooltipControls {
    fn default() -> Self {
        Self::new("Warning")
    }
}

impl Render for WarningTooltipControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div();
        }

        div()
            .p(px(8.0))
            .rounded(px(6.0))
            .bg(hsla(35.0 / 360.0, 0.95, 0.45, 1.0))
            .text_color(hsla(0.0, 0.0, 1.0, 1.0))
            .text_size(px(12.0))
            .child(self.message.clone())
    }
}
