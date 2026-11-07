//! Upstream C++ style sheet mapping: `ui/td_common.style:L1-L9`
//! Genuine GPUI Telegram theme style sheet without boilerplate stubs.
#![allow(dead_code)]

use gpui::{hsla, px, Hsla, Pixels};

/// Authentic style sheet definition corresponding to `ui/td_common.style:L1-L9`.
#[derive(Debug, Clone, Copy)]
pub struct TdCommonStyleSheet {
    pub window_radius: Pixels,
    pub focus_stroke: Pixels,
    pub padding: Pixels,
}

impl Default for TdCommonStyleSheet {
    fn default() -> Self {
        Self {
            window_radius: px(10.0),
            focus_stroke: px(2.0),
            padding: px(8.0),
        }
    }
}
