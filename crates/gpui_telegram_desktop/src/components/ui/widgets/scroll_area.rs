//! ScrollArea Reusable Widget
//! Upstream: Telegram/SourceFiles/ui/widgets/selecting_scroll.cpp:L18-L80
//! Upstream: Telegram/SourceFiles/ui/widgets/middle_click_autoscroll.cpp:L16-L23
//! Upstream: Telegram/SourceFiles/ui/rows_scroll_cache.cpp:L11-L42
//! Implements smooth scrolling container with scrollbar thumb geometry and boundary clamping.

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_DEFAULT_SCROLLBAR_WIDTH: f32 = 6.0;
pub const K_MIN_SCROLLBAR_THUMB_HEIGHT: f32 = 24.0;

#[derive(Debug, Clone)]
pub struct ScrollArea {
    pub scroll_top: f32,
    pub content_height: f32,
    pub viewport_height: f32,
    pub is_dragging_thumb: bool,
    pub thumb_opacity: f32,
    pub scrollbar_width: f32,
}

impl ScrollArea {
    pub fn new() -> Self {
        Self {
            scroll_top: 0.0,
            content_height: 1000.0,
            viewport_height: 400.0,
            is_dragging_thumb: false,
            thumb_opacity: 0.6,
            scrollbar_width: K_DEFAULT_SCROLLBAR_WIDTH,
        }
    }

    pub fn set_dimensions(&mut self, content_h: f32, viewport_h: f32) {
        self.content_height = content_h;
        self.viewport_height = viewport_h;
        self.clamp_scroll();
    }

    /// Scrolls by pixel delta, strictly clamping within [0, max_scroll].
    /// Upstream: ui/widgets/selecting_scroll.cpp:L73-L80
    pub fn scroll_by(&mut self, delta: f32) {
        self.scroll_top += delta;
        self.clamp_scroll();
    }

    pub fn scroll_to_top(&mut self) {
        self.scroll_top = 0.0;
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_top = self.max_scroll();
    }

    pub fn max_scroll(&self) -> f32 {
        (self.content_height - self.viewport_height).max(0.0)
    }

    fn clamp_scroll(&mut self) {
        let max = self.max_scroll();
        self.scroll_top = self.scroll_top.clamp(0.0, max);
    }

    /// Computes scrollbar thumb height and Y position.
    pub fn compute_thumb_geometry(&self) -> (f32, f32) {
        if self.content_height <= self.viewport_height || self.content_height <= 0.0 {
            return (0.0, 0.0);
        }
        let ratio = self.viewport_height / self.content_height;
        let thumb_height = (self.viewport_height * ratio).max(K_MIN_SCROLLBAR_THUMB_HEIGHT);
        let track_available = self.viewport_height - thumb_height;
        let scroll_ratio = if self.max_scroll() > 0.0 {
            self.scroll_top / self.max_scroll()
        } else {
            0.0
        };
        let thumb_top = track_available * scroll_ratio;
        (thumb_top, thumb_height)
    }
}

impl Default for ScrollArea {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ScrollArea {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (thumb_top, thumb_height) = self.compute_thumb_geometry();
        let mut root = div()
            .relative()
            .w_full()
            .h(px(self.viewport_height))
            .overflow_hidden();

        if thumb_height > 0.0 {
            root = root.child(
                div()
                    .absolute()
                    .right(px(2.0))
                    .top(px(thumb_top))
                    .w(px(self.scrollbar_width))
                    .h(px(thumb_height))
                    .rounded(px(self.scrollbar_width * 0.5))
                    .bg(rgba(0x00000044)),
            );
        }
        root
    }
}
