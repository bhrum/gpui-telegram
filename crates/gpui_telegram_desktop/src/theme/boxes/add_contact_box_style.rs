//! Authentic GPUI theme stylesheet for AddContactBox.
//! Upstream reference: `Telegram/SourceFiles/boxes/add_contact_box.style:L1-L28`.

use gpui::{px, rgb, Pixels, Point, Rgba};

#[derive(Debug, Clone)]
pub struct AddContactBoxStyleSheet {
    pub contact_icon_position: Point<Pixels>,
    pub contact_padding: (Pixels, Pixels, Pixels, Pixels),
    pub contact_row_height: Pixels,
    pub contact_user_icon_size: Pixels,
    pub contact_phone_icon_size: Pixels,
    pub user_icon_color: Rgba,
    pub phone_icon_color: Rgba,
    pub error_color: Rgba,
    pub background_color: Rgba,
}

impl Default for AddContactBoxStyleSheet {
    fn default() -> Self {
        Self {
            contact_icon_position: Point {
                x: px(17.0),
                y: px(12.0),
            },
            contact_padding: (px(56.0), px(16.0), px(16.0), px(16.0)),
            contact_row_height: px(48.0),
            contact_user_icon_size: px(24.0),
            contact_phone_icon_size: px(20.0),
            user_icon_color: rgb(0x707579),
            phone_icon_color: rgb(0x707579),
            error_color: rgb(0xDF3F40),
            background_color: rgb(0xFFFFFF),
        }
    }
}

impl AddContactBoxStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn row_height(&self) -> Pixels {
        self.contact_row_height
    }

    pub fn icon_offset(&self) -> Point<Pixels> {
        self.contact_icon_position
    }
}
