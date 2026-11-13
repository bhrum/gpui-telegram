//! Autodelete / TTL countdown button with tiny duration formatter.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_ttl_button.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_ttl_button.h`

use gpui::*;
use serde::{Deserialize, Serialize};

/// Format tiny TTL badge text (e.g. "1d", "1w", "1m")
/// Upstream: `Ui::FormatTTLTiny` (`history_view_ttl_button.cpp:L40`)
pub fn format_ttl_tiny(seconds: i32) -> String {
    if seconds <= 0 {
        return String::new();
    }
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m", seconds / 60)
    } else if seconds < 86400 {
        format!("{}h", seconds / 3600)
    } else if seconds < 604800 {
        format!("{}d", seconds / 86400)
    } else if seconds < 2592000 {
        format!("{}w", seconds / 604800)
    } else {
        format!("{}m", seconds / 2592000)
    }
}

pub struct HistoryViewTtlButtonViewControls {
    pub ttl_seconds: i32,
    pub is_visible: bool,
}

impl HistoryViewTtlButtonViewControls {
    pub fn new() -> Self {
        Self {
            ttl_seconds: 0,
            is_visible: false,
        }
    }

    pub fn set_ttl(&mut self, seconds: i32) {
        self.ttl_seconds = seconds;
        self.is_visible = seconds > 0;
    }

    pub fn badge_text(&self) -> String {
        format_ttl_tiny(self.ttl_seconds)
    }
}

impl Default for HistoryViewTtlButtonViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewTtlButtonViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible || self.ttl_seconds <= 0 {
            return div().into_any_element();
        }

        let badge = self.badge_text();

        div()
            .flex()
            .items_center()
            .justify_center()
            .px_2()
            .py_0p5()
            .rounded_full()
            .bg(rgb(0x242f3d))
            .text_xs()
            .font_weight(FontWeight::BOLD)
            .text_color(rgb(0x2481cc))
            .child(format!("⏱ {}", badge))
            .into_any_element()
    }
}
