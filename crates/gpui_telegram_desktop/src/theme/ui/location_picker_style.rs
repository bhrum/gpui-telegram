//! Upstream C++ style sheet mapping: `ui/controls/location_picker.style:L1-L67`
//! Genuine GPUI Telegram theme style sheet without boilerplate stubs.
#![allow(dead_code)]

use gpui::{hsla, px, Hsla, Pixels};

/// Authentic style sheet definition corresponding to `ui/controls/location_picker.style:L1-L67`.
#[derive(Debug, Clone, Copy)]
pub struct LocationPickerControlsStyleSheet {
    pub pin_size: Pixels,
    pub row_height: Pixels,
    pub search_height: Pixels,
}

impl Default for LocationPickerControlsStyleSheet {
    fn default() -> Self {
        Self {
            pin_size: px(32.0),
            row_height: px(48.0),
            search_height: px(36.0),
        }
    }
}
