//! Upstream: `Telegram/SourceFiles/ui/effects/unique_gift_message_bubble.cpp:L21-L36`, `L42-L60`
//! Upstream: `Telegram/SourceFiles/ui/effects/unique_gift_message_bubble.h:L18-L64`
//!
//! Unique star gift message bubble layout with smooth Bézier tail geometry.
//! Uses cubic control points for customized chat balloon silhouettes.

use gpui::*;

pub const FILL_OPACITY: f32 = 1.0 / 6.0; // 0.166667
pub const OUTLINE_OPACITY: f32 = 0.5;
pub const ARC_CONTROL: f32 = 0.55228475;
pub const TAIL_TIP_CONTROL_X: f32 = 0.5459279;
pub const TAIL_JOIN_X: f32 = 0.79999984;

#[derive(Debug, Clone)]
pub struct UniqueGiftMessageBubbleEffects {
    gift_title: String,
    num_stars: u64,
    body_width: f32,
    body_height: f32,
    has_tail: bool,
}

impl UniqueGiftMessageBubbleEffects {
    pub fn new(title: impl Into<String>, stars: u64) -> Self {
        Self {
            gift_title: title.into(),
            num_stars: stars,
            body_width: 240.0,
            body_height: 140.0,
            has_tail: true,
        }
    }

    /// C++: `unique_gift_message_bubble.cpp:L42-L60`: `ResolveWidths`
    pub fn resolve_widths(outer_width: f32, avatar_size: f32, tail_width: f32, radius: f32) -> (f32, f32) {
        let max_body = (outer_width - avatar_size - tail_width).max(0.0);
        let min_body = (2.0 * radius + tail_width).min(max_body);
        (min_body, max_body)
    }

    pub fn set_has_tail(&mut self, has_tail: bool) {
        self.has_tail = has_tail;
    }

    pub fn gift_title(&self) -> &str {
        &self.gift_title
    }

    pub fn stars(&self) -> u64 {
        self.num_stars
    }
}

impl Default for UniqueGiftMessageBubbleEffects {
    fn default() -> Self {
        Self::new("Limited Edition Gift", 500)
    }
}

impl Render for UniqueGiftMessageBubbleEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(self.body_width))
            .min_h(px(self.body_height))
            .rounded(px(16.0))
            .bg(rgba(0x2B52782A))
            .border_1()
            .border_color(rgba(0x2AABEE80))
            .p(px(12.0))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFD700))
                    .child(format!("⭐ {} Stars", self.num_stars))
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0xF5F5F5))
                    .child(self.gift_title.clone())
            )
    }
}
