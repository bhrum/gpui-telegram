//! Authentic GPUI theme stylesheet for ShareBox (Forward / Share Message Picker).
//! Upstream reference: `Telegram/SourceFiles/boxes/share_box.style:L1-L35`.

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct ShareBoxStyleSheet {
    pub rows_top_margin: Pixels,
    pub row_height: Pixels,
    pub photo_top_offset: Pixels,
    pub userpic_size: Pixels,
    pub comment_input_height: Pixels,
    pub box_width: Pixels,
    pub peer_name_fg: Rgba,
    pub comment_bg: Rgba,
    pub send_button_bg: Rgba,
}

impl Default for ShareBoxStyleSheet {
    fn default() -> Self {
        Self {
            rows_top_margin: px(12.0),
            row_height: px(108.0),
            photo_top_offset: px(6.0),
            userpic_size: px(52.0),
            comment_input_height: px(48.0),
            box_width: px(380.0),
            peer_name_fg: rgb(0x1E293B),
            comment_bg: rgb(0xF8FAFC),
            send_button_bg: rgb(0x3B82F6),
        }
    }
}

impl ShareBoxStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn grid_row_height(&self) -> Pixels {
        self.row_height
    }
}
