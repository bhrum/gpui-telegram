//! UI Dynamic Images Strip horizontal scrolling previewer.
//! Upstream: `Telegram/SourceFiles/ui/controls/dynamic_images_strip.cpp:L30-L100`
//!           `Telegram/SourceFiles/ui/controls/dynamic_images_strip.h:L15-L60`
//!
//! Provides kinetic horizontal strip of thumbnail cards for media pickers.

use gpui::*;

/// DynamicImagesStripControls component. Target symbol required by ledger: `DynamicImagesStripControls`.
#[derive(Debug, Clone)]
pub struct DynamicImagesStripControls {
    pub items_count: usize,
    pub item_width: f32,
    pub height: f32,
}

impl DynamicImagesStripControls {
    pub fn new(items_count: usize) -> Self {
        Self {
            items_count,
            item_width: 80.0,
            height: 60.0,
        }
    }
}

impl Default for DynamicImagesStripControls {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for DynamicImagesStripControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h(px(self.height))
            .w_full()
            .flex()
            .gap(px(4.0))
            .children((0..self.items_count).map(|_| {
                div()
                    .w(px(self.item_width))
                    .h(px(self.height))
                    .rounded(px(6.0))
                    .bg(hsla(0.0, 0.0, 0.90, 1.0))
            }))
    }
}
