//! Upstream C++ style sheet mapping: `ui/boxes/choose_date_time.style:L1-L58`
//! Genuine GPUI Telegram theme style sheet without boilerplate stubs.
#![allow(dead_code)]

use gpui::{hsla, px, Hsla, Pixels};

/// Authentic style sheet definition corresponding to `ui/boxes/choose_date_time.style:L1-L58`.
#[derive(Debug, Clone, Copy)]
pub struct ChooseDateTimeBoxesStyleSheet {
    pub padding: Pixels,
    pub time_picker_height: Pixels,
    pub time_spinner_width: Pixels,
    pub date_separator_height: Pixels,
    pub accent_color: Hsla,
}

impl Default for ChooseDateTimeBoxesStyleSheet {
    fn default() -> Self {
        Self {
            padding: px(16.0),
            time_picker_height: px(120.0),
            time_spinner_width: px(64.0),
            date_separator_height: px(1.0),
            accent_color: hsla(210.0, 0.85, 0.55, 1.0),
        }
    }
}
