//! Upstream: `Telegram/SourceFiles/ui/effects/peer_qr_gifts_panel.cpp:L1-L721`
//! Upstream: `Telegram/SourceFiles/ui/effects/peer_qr_gifts_panel.h:L18-L57`
//!
//! Horizontal carousel panel for selecting collectible and star gifts to embed in peer QR codes.

use gpui::*;

#[derive(Debug, Clone)]
pub struct QrGiftOption {
    pub id: u64,
    pub title: String,
    pub stars: u64,
}

#[derive(Debug, Clone)]
pub struct PeerQrGiftsPanelEffects {
    gifts: Vec<QrGiftOption>,
    selected_id: Option<u64>,
}

impl PeerQrGiftsPanelEffects {
    pub fn new() -> Self {
        let gifts = vec![
            QrGiftOption { id: 1, title: "Birthday Star".to_string(), stars: 50 },
            QrGiftOption { id: 2, title: "Golden Trophy".to_string(), stars: 250 },
            QrGiftOption { id: 3, title: "Diamond Ring".to_string(), stars: 1000 },
        ];
        Self {
            gifts,
            selected_id: Some(1),
        }
    }

    pub fn select_gift(&mut self, id: u64) {
        if self.gifts.iter().any(|g| g.id == id) {
            self.selected_id = Some(id);
        }
    }

    pub fn selected_gift(&self) -> Option<&QrGiftOption> {
        self.selected_id.and_then(|id| self.gifts.iter().find(|g| g.id == id))
    }

    pub fn gifts(&self) -> &[QrGiftOption] {
        &self.gifts
    }
}

impl Default for PeerQrGiftsPanelEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PeerQrGiftsPanelEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let sel = self.selected_id;
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .children(self.gifts.iter().map(|g| {
                let is_sel = Some(g.id) == sel;
                div()
                    .p(px(8.0))
                    .rounded(px(8.0))
                    .bg(if is_sel { rgb(0x2B5278) } else { rgb(0x182533) })
                    .border_1()
                    .border_color(if is_sel { rgb(0x2AABEE) } else { rgba(0x00000000) })
                    .text_size(px(12.0))
                    .text_color(rgb(0xF5F5F5))
                    .child(format!("{} (⭐ {})", g.title, g.stars))
            }))
    }
}
