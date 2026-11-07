//! UI Peer List Dummy skeleton loader.
//! Upstream: `Telegram/SourceFiles/ui/controls/peer_list_dummy.cpp:L10-L35`
//!           `Telegram/SourceFiles/ui/controls/peer_list_dummy.h:L10-L25`
//!
//! Provides placeholder shimmer rows while loading contact and chat lists.

use gpui::*;

/// PeerListDummyControls component. Target symbol required by ledger: `PeerListDummyControls`.
#[derive(Debug, Clone)]
pub struct PeerListDummyControls {
    pub row_count: usize,
    pub row_height: f32,
}

impl PeerListDummyControls {
    pub fn new(row_count: usize) -> Self {
        Self {
            row_count,
            row_height: 56.0,
        }
    }
}

impl Default for PeerListDummyControls {
    fn default() -> Self {
        Self::new(5)
    }
}

impl Render for PeerListDummyControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .children((0..self.row_count).map(|_| {
                div()
                    .h(px(self.row_height))
                    .w_full()
                    .flex()
                    .items_center()
                    .px(px(12.0))
                    .gap(px(10.0))
                    .child(div().w(px(40.0)).h(px(40.0)).rounded(px(20.0)).bg(hsla(0.0, 0.0, 0.90, 1.0)))
                    .child(div().w(px(120.0)).h(px(12.0)).rounded(px(4.0)).bg(hsla(0.0, 0.0, 0.90, 1.0)))
            }))
    }
}
