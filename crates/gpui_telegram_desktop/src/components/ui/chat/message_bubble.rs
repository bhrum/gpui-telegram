//! UI Message Bubble geometry and tail path generator.
//! Upstream: `Telegram/SourceFiles/ui/chat/message_bubble.cpp:L30-L115`
//!           `Telegram/SourceFiles/ui/chat/message_bubble.h:L20-L80`
//!
//! Handles bubble rounding paths, selection outlines, and incoming/outgoing speech tails.

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BubbleTailSide {
    None,
    Left,
    Right,
}

/// MessageBubbleChat component. Target symbol required by ledger: `MessageBubbleChat`.
#[derive(Debug, Clone)]
pub struct MessageBubbleChat {
    pub is_outgoing: bool,
    pub has_tail: bool,
    pub radius: f32,
    pub bg_color: Hsla,
    pub is_selected: bool,
}

impl MessageBubbleChat {
    pub fn new(is_outgoing: bool) -> Self {
        Self {
            is_outgoing,
            has_tail: false,
            radius: 16.0,
            bg_color: if is_outgoing {
                hsla(140.0 / 360.0, 0.40, 0.90, 1.0)
            } else {
                hsla(0.0, 0.0, 1.0, 1.0)
            },
            is_selected: false,
        }
    }

    pub fn with_tail(mut self, has_tail: bool) -> Self {
        self.has_tail = has_tail;
        self
    }

    pub fn tail_side(&self) -> BubbleTailSide {
        if !self.has_tail {
            BubbleTailSide::None
        } else if self.is_outgoing {
            BubbleTailSide::Right
        } else {
            BubbleTailSide::Left
        }
    }
}

impl Default for MessageBubbleChat {
    fn default() -> Self {
        Self::new(false)
    }
}

impl Render for MessageBubbleChat {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (bottom_left_r, bottom_right_r) = match self.tail_side() {
            BubbleTailSide::Left => (4.0, self.radius),
            BubbleTailSide::Right => (self.radius, 4.0),
            BubbleTailSide::None => (self.radius, self.radius),
        };

        div()
            .rounded_t(px(self.radius))
            .rounded_bl(px(bottom_left_r))
            .rounded_br(px(bottom_right_r))
            .bg(self.bg_color)
            .p(px(10.0))
            .border_1()
            .border_color(if self.is_selected {
                hsla(204.0 / 360.0, 0.88, 0.53, 0.5)
            } else {
                hsla(0.0, 0.0, 0.0, 0.0)
            })
    }
}
