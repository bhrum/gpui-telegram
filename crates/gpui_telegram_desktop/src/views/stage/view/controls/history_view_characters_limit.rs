//! Characters limitation indicator label with unicode minus sign and logarithmic digit count logic.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_characters_limit.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_characters_limit.h`

use gpui::*;
use serde::{Deserialize, Serialize};

/// Mathematical unicode minus sign
/// Upstream: `constexpr auto kMinus = QChar(0x2212);` (`history_view_characters_limit.cpp:L15`)
pub const UNICODE_MINUS: char = '\u{2212}';

/// Over-limit cap
/// Upstream: `constexpr auto kLimit = int(999);` (`history_view_characters_limit.cpp:L16`)
pub const LIMIT_CAP: i32 = 999;

/// Count decimal digits for ordering changes
/// Upstream: `[[nodiscard]] int CountDigits(int n)` (`history_view_characters_limit.cpp:L18-20`)
pub fn count_digits(n: i32) -> usize {
    if n == 0 {
        1
    } else {
        ((n.abs() as f64).log10().floor() as usize) + 1
    }
}

/// Authentic characters limit label view and model
pub struct HistoryViewCharactersLimitViewControls {
    pub current_value: i32,
    pub max_limit: i32,
    pub is_visible: bool,
}

impl HistoryViewCharactersLimitViewControls {
    pub fn new() -> Self {
        Self {
            current_value: 0,
            max_limit: 4096,
            is_visible: false,
        }
    }

    /// Set value: negative means remaining characters, positive means exceeded count
    /// Upstream: `CharactersLimitLabel::setLeft(int value)` (`history_view_characters_limit.cpp:L53-69`)
    pub fn set_left(&mut self, value: i32) {
        self.current_value = value;
        self.is_visible = value > 0 || value > -100;
    }

    /// Formatted display string and error flag
    pub fn display_info(&self) -> (String, bool) {
        if self.current_value > 0 {
            // Exceeded limit: prefix with unicode minus, red warning
            let capped = self.current_value.min(LIMIT_CAP);
            (format!("{}{}", UNICODE_MINUS, capped), true)
        } else {
            // Characters remaining
            ((-self.current_value).to_string(), false)
        }
    }
}

impl Default for HistoryViewCharactersLimitViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewCharactersLimitViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let (text, is_exceeded) = self.display_info();

        div()
            .flex()
            .items_center()
            .justify_center()
            .px_1p5()
            .py_0p5()
            .rounded_sm()
            .text_xs()
            .font_weight(FontWeight::BOLD)
            .text_color(if is_exceeded {
                rgb(0xff3b30) // Red error
            } else {
                rgb(0x8e8e93) // Muted subtext
            })
            .child(text)
            .into_any_element()
    }
}
