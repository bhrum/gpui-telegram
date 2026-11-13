//! Upstream: `Telegram/SourceFiles/ui/effects/message_sending_animation_controller.cpp:L34-L39`, `L40-L60`
//! Upstream: `Telegram/SourceFiles/ui/effects/message_sending_animation_controller.h:L18-L58`
//!
//! Message sending flight controller. Orchestrates animated trajectory from composer
//! field into the chat history view with midpoint surrounding scaling.

use gpui::*;
use gpui::prelude::FluentBuilder;

pub const SURROUNDING_PROGRESS: f32 = 0.5;

#[derive(Debug, Clone)]
pub struct FlightBounds {
    pub from_x: f32,
    pub from_y: f32,
    pub to_x: f32,
    pub to_y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct MessageSendingAnimationControllerEffects {
    progress: f32,
    bounds: Option<FlightBounds>,
    is_animating: bool,
}

impl MessageSendingAnimationControllerEffects {
    pub fn new() -> Self {
        Self {
            progress: 1.0,
            bounds: None,
            is_animating: false,
        }
    }

    pub fn start_flight(&mut self, bounds: FlightBounds) {
        self.bounds = Some(bounds);
        self.progress = 0.0;
        self.is_animating = true;
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
        if self.progress >= 1.0 {
            self.is_animating = false;
        }
    }

    /// C++: `message_sending_animation_controller.cpp:L36-L38`:
    /// `OffsetMid(value, min, max) = ((value * max) - (value * min)) / 2.`
    pub fn offset_mid(value: f32, min: f32, max: f32) -> f32 {
        ((value * max) - (value * min)) / 2.0
    }

    /// Computes current interpolated flight position `(x, y)`
    pub fn current_position(&self) -> Option<(f32, f32)> {
        self.bounds.as_ref().map(|b| {
            let t = self.progress;
            let ease = t * (2.0 - t); // Quadratic ease-out
            let x = b.from_x + (b.to_x - b.from_x) * ease;
            let y = b.from_y + (b.to_y - b.from_y) * ease;
            (x, y)
        })
    }

    pub fn is_animating(&self) -> bool {
        self.is_animating
    }
}

impl Default for MessageSendingAnimationControllerEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MessageSendingAnimationControllerEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let pos = self.current_position();
        div()
            .relative()
            .size_full()
            .when(self.is_animating && pos.is_some(), |el| {
                let (x, y) = pos.unwrap();
                let b = self.bounds.as_ref().unwrap();
                el.child(
                    div()
                        .absolute()
                        .left(px(x))
                        .top(px(y))
                        .w(px(b.width))
                        .h(px(b.height))
                        .bg(rgb(0x2B5278))
                        .rounded(px(8.0))
                )
            })
    }
}
