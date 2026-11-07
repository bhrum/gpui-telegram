//! History View Swipe Back Session Gesture Handler.
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/history/history_view_swipe_back_session.h:L18-L30`
//! - Upstream: `Telegram/SourceFiles/history/history_view_swipe_back_session.cpp:L18-L63`
//! - Invariant: `dontStart = list->touchMaybeSelectingValue()` (history_view_swipe_back_session.cpp:L59)
//! - Direction: `Qt::RightToLeft` swipe-back trigger calling `showBackFromStack()` (history_view_swipe_back_session.cpp:L47-L52)

use gpui::*;

/// Swipe back session state machine.
/// C++ citation: `Window::SetupSwipeBackSection` (history_view_swipe_back_session.cpp:L18-L63).
pub struct HistoryViewSwipeBackSession {
    is_active: bool,
    translation_x: f32,
    completion_threshold_ratio: f32,
    is_selecting_text: bool,
    target_width: f32,
}

impl HistoryViewSwipeBackSession {
    pub fn new() -> Self {
        Self {
            is_active: false,
            translation_x: 0.0,
            completion_threshold_ratio: 0.35,
            is_selecting_text: false,
            target_width: 800.0,
        }
    }

    /// Sets whether the user is actively selecting text.
    /// C++ citation: `history_view_swipe_back_session.cpp:L59` (`dontStart = list->touchMaybeSelectingValue()`).
    pub fn set_selecting_text(&mut self, selecting: bool) {
        self.is_selecting_text = selecting;
        if selecting && self.is_active {
            self.cancel();
        }
    }

    /// Starts a swipe-back gesture if not selecting text.
    pub fn start_swipe(&mut self, width: f32) -> bool {
        if self.is_selecting_text {
            return false;
        }
        self.is_active = true;
        self.target_width = width.max(100.0);
        self.translation_x = 0.0;
        true
    }

    /// Updates current horizontal swipe delta.
    /// C++ citation: `history_view_swipe_back_session.cpp:L24-L45`.
    pub fn update_translation(&mut self, dx: f32) {
        if !self.is_active {
            return;
        }
        // Only allow positive translation (moving current section to the right)
        self.translation_x = dx.max(0.0);
    }

    /// Progress fraction from 0.0 to 1.0.
    pub fn progress(&self) -> f32 {
        (self.translation_x / self.target_width).clamp(0.0, 1.0)
    }

    /// Whether current swipe distance exceeds the pop threshold.
    pub fn should_pop(&self) -> bool {
        self.progress() >= self.completion_threshold_ratio
    }

    /// Cancels or finishes the session.
    pub fn finish(&mut self) -> bool {
        let pop = self.is_active && self.should_pop();
        self.cancel();
        pop
    }

    pub fn cancel(&mut self) {
        self.is_active = false;
        self.translation_x = 0.0;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn translation_x(&self) -> f32 {
        self.translation_x
    }
}

impl Default for HistoryViewSwipeBackSession {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewSwipeBackSession {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_active || self.translation_x <= 0.0 {
            return div().into_any_element();
        }

        let p = self.progress();
        let bg_opacity = (1.0 - p) * 0.4;

        div()
            .absolute()
            .inset_0()
            .bg(rgba((0x000000 as u32) << 8 | (bg_opacity * 255.0) as u32))
            .into_any_element()
    }
}
