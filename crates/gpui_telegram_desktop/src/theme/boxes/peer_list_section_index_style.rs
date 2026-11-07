//! Authentic GPUI theme stylesheet for PeerListSectionIndex (A-Z alphabet scrubber).
//! Upstream reference: `Telegram/SourceFiles/boxes/peer_list_section_index.style:L1-L17`.

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct PeerListSectionIndexStyleSheet {
    pub index_width: Pixels,
    pub index_approach_distance: Pixels,
    pub slot_height: Pixels,
    pub min_slot_height: Pixels,
    pub font_size: Pixels,
    pub text_fg: Rgba,
    pub active_fg: Rgba,
    pub background_hover: Rgba,
}

impl Default for PeerListSectionIndexStyleSheet {
    fn default() -> Self {
        Self {
            index_width: px(22.0),
            index_approach_distance: px(32.0),
            slot_height: px(15.0),
            min_slot_height: px(12.0),
            font_size: px(10.0),
            text_fg: rgb(0x94A3B8),
            active_fg: rgb(0x2563EB),
            background_hover: gpui::rgba(0x0000000A),
        }
    }
}

impl PeerListSectionIndexStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(&self) -> Pixels {
        self.index_width
    }
}
