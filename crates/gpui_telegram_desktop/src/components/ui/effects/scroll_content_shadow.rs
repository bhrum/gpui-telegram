//! Upstream: `Telegram/SourceFiles/ui/effects/scroll_content_shadow.cpp:L17-L46`
//! Upstream: `Telegram/SourceFiles/ui/effects/scroll_content_shadow.h:L12-L21`
//!
//! Dynamic top/bottom content edge shadows for scroll areas. Shows top shadow when
//! scrolled away from top, and bottom shadow when scrolled above bottom.

use gpui::*;
use gpui::prelude::FluentBuilder;

#[derive(Debug, Clone, Default)]
pub struct ScrollContentShadowEffects {
    scroll_top: f32,
    viewport_height: f32,
    inner_height: f32,
    shadow_height: f32,
}

impl ScrollContentShadowEffects {
    pub fn new() -> Self {
        Self {
            scroll_top: 0.0,
            viewport_height: 400.0,
            inner_height: 400.0,
            shadow_height: 4.0,
        }
    }

    pub fn update_geometry(&mut self, scroll_top: f32, viewport_height: f32, inner_height: f32) {
        self.scroll_top = scroll_top.max(0.0);
        self.viewport_height = viewport_height.max(0.0);
        self.inner_height = inner_height.max(0.0);
    }

    /// C++: `scroll_content_shadow.cpp:L40`: `topShadow->toggleOn(scroll->scrollTopValue() | rpl::map(_1 > 0));`
    pub fn is_top_shadow_visible(&self) -> bool {
        self.scroll_top > 0.5
    }

    /// C++: `scroll_content_shadow.cpp:L41-L45`: `bottomShadow->toggleOn(_1 + _2 < _3);`
    pub fn is_bottom_shadow_visible(&self) -> bool {
        self.scroll_top + self.viewport_height + 0.5 < self.inner_height
    }

    pub fn shadow_height(&self) -> f32 {
        self.shadow_height
    }
}

impl Render for ScrollContentShadowEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let top_visible = self.is_top_shadow_visible();
        let bottom_visible = self.is_bottom_shadow_visible();

        div()
            .relative()
            .size_full()
            .when(top_visible, |el| {
                el.child(
                    div()
                        .absolute()
                        .top_0()
                        .w_full()
                        .h(px(self.shadow_height))
                        .bg(rgba(0x00000044))
                )
            })
            .when(bottom_visible, |el| {
                el.child(
                    div()
                        .absolute()
                        .bottom_0()
                        .w_full()
                        .h(px(self.shadow_height))
                        .bg(rgba(0x00000044))
                )
            })
    }
}
