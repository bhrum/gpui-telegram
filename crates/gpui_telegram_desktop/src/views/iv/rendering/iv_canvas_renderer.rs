//! Canvas renderer for Instant View
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, Default)]
pub struct IvCanvasRenderer {
    pub scale_factor: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
}

impl IvCanvasRenderer {
    pub fn new() -> Self {
        Self {
            scale_factor: 1.0,
            viewport_width: 800.0,
            viewport_height: 600.0,
        }
    }

    pub fn set_viewport(&mut self, width: f32, height: f32, scale: f32) {
        self.viewport_width = width;
        self.viewport_height = height;
        self.scale_factor = scale;
    }
}

impl Render for IvCanvasRenderer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .bg(rgb(0x181a1d))
            .child(
                div()
                    .p_4()
                    .text_xs()
                    .text_color(rgb(0x888888))
                    .child(format!("Canvas: {:.0}x{:.0} @ {:.1}x", self.viewport_width, self.viewport_height, self.scale_factor)),
            )
    }
}
