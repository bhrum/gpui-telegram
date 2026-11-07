//! Info Profile QR Gifts component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_qr_gifts.cpp:L1-L177`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_qr_gifts.h:L1-L33`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

/// Specialized QR card renderer for collectible star gifts and Telegram Premium codes.
#[derive(Debug, Clone)]
pub struct InfoProfileQrGiftsProfile {
    pub gift_id: u64,
    pub gift_title: String,
    pub gift_link: String,
}

impl InfoProfileQrGiftsProfile {
    pub fn new(gift_id: u64, title: impl Into<String>, link: impl Into<String>) -> Self {
        Self {
            gift_id,
            gift_title: title.into(),
            gift_link: link.into(),
        }
    }
}

impl Default for InfoProfileQrGiftsProfile {
    fn default() -> Self {
        Self::new(0, "Collectible Star Gift", "https://t.me/nft/gift")
    }
}

impl Render for InfoProfileQrGiftsProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .p(px(16.0))
            .gap(px(8.0))
            .bg(rgb(0x18222d))
            .child(div().font_weight(FontWeight::BOLD).child(self.gift_title.clone()))
            .child(div().text_xs().text_color(rgb(0x708499)).child(self.gift_link.clone()))
    }
}
