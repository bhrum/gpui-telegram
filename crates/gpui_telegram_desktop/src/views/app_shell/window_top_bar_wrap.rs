//! WindowTopBarWrap: Header bar wrapper with dynamic height, shadows, and adaptive metrics
//! Upstream: Telegram/SourceFiles/window/window_top_bar_wrap.h

use gpui::*;

#[derive(Debug, Clone)]
pub struct WindowTopBarWrap {
    pub content_height: f32,
    pub has_shadow: bool,
    pub is_compact: bool,
}

impl WindowTopBarWrap {
    pub fn new() -> Self {
        Self {
            content_height: 54.0,
            has_shadow: true,
            is_compact: false,
        }
    }

    pub fn show_shadow(&mut self) {
        self.has_shadow = true;
    }

    pub fn hide_shadow(&mut self) {
        self.has_shadow = false;
    }

    pub fn update_adaptive_layout(&mut self, is_compact: bool) {
        self.is_compact = is_compact;
        self.content_height = if is_compact { 44.0 } else { 54.0 };
    }
}

impl Default for WindowTopBarWrap {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowTopBarWrap {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .h(px(self.content_height))
            .w_full()
            .bg(rgb(0x17212B))
            .border_b_1()
            .border_color(if self.has_shadow { rgb(0x242F3D) } else { rgba(0x00000000) })
    }
}
