//! Upstream: `Telegram/SourceFiles/ui/effects/ttl_icon.cpp:L16-L45`
//! Upstream: `Telegram/SourceFiles/ui/effects/ttl_icon.h:L10-L19`
//!
//! TTL (Time-To-Live) countdown self-destructing message timer icon.
//! Renders countdown badge with 2-character limit and circular arc indicator.

use gpui::*;

pub const TTL_PEN_WIDTH: f32 = 1.5;

#[derive(Debug, Clone)]
pub struct TtlIconEffects {
    duration_text: String,
    seconds_left: u32,
    size: f32,
}

impl TtlIconEffects {
    pub fn new(seconds_left: u32) -> Self {
        let text = Self::format_seconds(seconds_left);
        Self {
            duration_text: text,
            seconds_left,
            size: 18.0,
        }
    }

    /// C++: `ttl_icon.cpp:L25`: `(text.size() > 2) ? text.mid(0, 2) : text`
    pub fn display_badge_text(&self) -> &str {
        if self.duration_text.len() > 2 {
            &self.duration_text[..2]
        } else {
            &self.duration_text
        }
    }

    pub fn set_seconds(&mut self, seconds: u32) {
        self.seconds_left = seconds;
        self.duration_text = Self::format_seconds(seconds);
    }

    fn format_seconds(secs: u32) -> String {
        if secs < 60 {
            format!("{}s", secs)
        } else if secs < 3600 {
            format!("{}m", secs / 60)
        } else if secs < 86400 {
            format!("{}h", secs / 3600)
        } else {
            format!("{}d", secs / 86400)
        }
    }

    pub fn seconds_left(&self) -> u32 {
        self.seconds_left
    }
}

impl Default for TtlIconEffects {
    fn default() -> Self {
        Self::new(30)
    }
}

impl Render for TtlIconEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let label = self.display_badge_text().to_string();
        div()
            .size(px(self.size))
            .rounded(px(self.size / 2.0))
            .border_1()
            .border_color(rgb(0xE53935))
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(9.0))
            .font_weight(FontWeight::BOLD)
            .text_color(rgb(0xE53935))
            .child(label)
    }
}
