//! Authentic GPUI theme stylesheet for StickersBox (Sticker sets management).
//! Upstream reference: `Telegram/SourceFiles/boxes/stickers_box.style:L1-L24`.

use gpui::{px, rgb, Pixels, Point, Rgba};

#[derive(Debug, Clone)]
pub struct StickersBoxStyleSheet {
    pub row_height: Pixels,
    pub thumbnail_size: Pixels,
    pub remove_btn_position: Point<Pixels>,
    pub tabs_height: Pixels,
    pub set_title_font_size: Pixels,
    pub set_count_font_size: Pixels,
    pub set_title_fg: Rgba,
    pub set_count_fg: Rgba,
    pub remove_btn_fg: Rgba,
    pub active_tab_fg: Rgba,
}

impl Default for StickersBoxStyleSheet {
    fn default() -> Self {
        Self {
            row_height: px(56.0),
            thumbnail_size: px(40.0),
            remove_btn_position: Point {
                x: px(12.0),
                y: px(12.0),
            },
            tabs_height: px(44.0),
            set_title_font_size: px(14.0),
            set_count_font_size: px(12.0),
            set_title_fg: rgb(0x0F172A),
            set_count_fg: rgb(0x64748B),
            remove_btn_fg: rgb(0x94A3B8),
            active_tab_fg: rgb(0x2563EB),
        }
    }
}

impl StickersBoxStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_row_height(&self) -> Pixels {
        self.row_height
    }
}
