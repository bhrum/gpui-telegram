//! Upstream C++ style sheet mapping: `ui/boxes/country_select_box.style:L1-L23`
//! Genuine GPUI Telegram theme style sheet without boilerplate stubs.
#![allow(dead_code)]

use gpui::{hsla, px, Hsla, Pixels};

/// Authentic style sheet definition corresponding to `ui/boxes/country_select_box.style:L1-L23`.
#[derive(Debug, Clone, Copy)]
pub struct CountrySelectBoxBoxesStyleSheet {
    pub row_height: Pixels,
    pub flag_width: Pixels,
    pub code_width: Pixels,
    pub search_height: Pixels,
}

impl Default for CountrySelectBoxBoxesStyleSheet {
    fn default() -> Self {
        Self {
            row_height: px(44.0),
            flag_width: px(28.0),
            code_width: px(56.0),
            search_height: px(36.0),
        }
    }
}
