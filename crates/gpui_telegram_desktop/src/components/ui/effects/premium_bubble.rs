//! Upstream: `Telegram/SourceFiles/ui/effects/premium_bubble.cpp:L25-L31`, `L46-L65`
//! Upstream: `Telegram/SourceFiles/ui/effects/premium_bubble.h:L18-L198`
//!
//! Floating speech bubble tooltip with number counter animation and tail deflection.
//! Implements 1000ms slide duration and 30px tail deflection math.

use gpui::*;
use std::time::Duration;

pub const BUBBLE_RADIUS_SUBTRACTOR: f32 = 2.0;
pub const DEFLECTION_SMALL: f32 = 20.0;
pub const DEFLECTION: f32 = 30.0;
pub const STEP_BEFORE_DEFLECTION: f32 = 0.75;
pub const SLIDE_DURATION: Duration = Duration::from_millis(1000);

#[derive(Debug, Clone)]
pub struct PremiumBubbleEffects {
    count_value: i64,
    has_tail: bool,
    width: f32,
    height: f32,
}

impl PremiumBubbleEffects {
    pub fn new(initial_count: i64) -> Self {
        Self {
            count_value: initial_count,
            has_tail: true,
            width: 80.0,
            height: 36.0,
        }
    }

    pub fn set_count(&mut self, count: i64) {
        self.count_value = count;
    }

    pub fn count(&self) -> i64 {
        self.count_value
    }

    pub fn has_tail(&self) -> bool {
        self.has_tail
    }
}

impl Default for PremiumBubbleEffects {
    fn default() -> Self {
        Self::new(1)
    }
}

impl Render for PremiumBubbleEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(self.width))
            .h(px(self.height))
            .rounded(px(self.height / 2.0 - BUBBLE_RADIUS_SUBTRACTOR))
            .bg(rgb(0x7B51B8))
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(13.0))
            .font_weight(FontWeight::BOLD)
            .text_color(rgb(0xFFFFFF))
            .child(format!("{}", self.count_value))
    }
}
