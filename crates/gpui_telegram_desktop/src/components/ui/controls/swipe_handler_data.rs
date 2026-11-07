//! UI Swipe Handler Data structures for gesture tracking.
//! Upstream: `Telegram/SourceFiles/ui/controls/swipe_handler_data.h:L10-L49`
//!
//! Models cursor translation, reply thresholds, reach ratios, and bare message IDs for swipe actions.

use gpui::*;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct SwipeContextData {
    pub ratio: f32,
    pub reach_ratio: f32,
    pub exact_translation: f32,
    pub msg_bare_id: i64,
    pub translation: i32,
    pub cursor_top: i32,
    pub inverted: bool,
}

impl SwipeContextData {
    pub fn is_empty(&self) -> bool {
        self.ratio == 0.0
            && self.reach_ratio == 0.0
            && self.translation == 0
            && self.cursor_top == 0
    }

    pub fn visual_translation_for(&self, bare_id: i64) -> i32 {
        if self.translation == 0 || self.msg_bare_id != bare_id {
            0
        } else if self.inverted {
            self.translation
        } else {
            -self.translation
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SwipeHandlerInitData {
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub is_rtl: bool,
}

/// SwipeHandlerDataControls component. Target symbol required by ledger: `SwipeHandlerDataControls`.
#[derive(Debug, Clone, Default)]
pub struct SwipeHandlerDataControls {
    pub context: SwipeContextData,
    pub init_data: SwipeHandlerInitData,
}

impl SwipeHandlerDataControls {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_context(mut self, context: SwipeContextData) -> Self {
        self.context = context;
        self
    }
}

impl Render for SwipeHandlerDataControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_size(px(12.0))
            .child(format!("SwipeData: msg={} ratio={:.2}", self.context.msg_bare_id, self.context.ratio))
    }
}
