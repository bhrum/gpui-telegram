//! History View Empty List Bubble View Component (`history_view_empty_list_bubble`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_empty_list_bubble.h` (L1-L42)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_empty_list_bubble.cpp` (L1-L71)

/// Geometry calculation for centered bubble (1/3 from top)
/// Upstream: `history_view_empty_list_bubble.cpp:L40`
#[inline]
pub fn calculate_empty_bubble_position(parent_w: f32, parent_h: f32, bubble_w: f32, bubble_h: f32) -> (f32, f32) {
    let x = ((parent_w - bubble_w) / 2.0).max(0.0);
    let y = ((parent_h - bubble_h) / 3.0).max(0.0);
    (x, y)
}

/// GPUI view component for empty chat service bubble
#[derive(Debug, Clone)]
pub struct HistoryViewEmptyListBubbleView {
    pub text: String,
    pub width: f32,
    pub height: f32,
    pub is_visible: bool,
}

impl HistoryViewEmptyListBubbleView {
    pub fn new(text: String) -> Self {
        Self {
            text,
            width: 280.0,
            height: 80.0,
            is_visible: true,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn compute_position(&self, container_w: f32, container_h: f32) -> (f32, f32) {
        calculate_empty_bubble_position(container_w, container_h, self.width, self.height)
    }
}
