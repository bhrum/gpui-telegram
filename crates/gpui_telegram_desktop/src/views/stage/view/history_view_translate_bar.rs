//! History View Translate Bar View Component (`history_view_translate_bar`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_translate_bar.h` (L1-L79)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_translate_bar.cpp` (L1-L669)

use crate::views::stage::translate_bar::{
    HistoryViewTranslateBarState, TranslateTracker, K_TOAST_DURATION,
};

/// GPUI view component for real-time translation banner
#[derive(Debug, Clone)]
pub struct HistoryViewTranslateBarView {
    pub inner: HistoryViewTranslateBarState,
    pub height: f32,
}

impl HistoryViewTranslateBarView {
    pub fn new(target_lang: String) -> Self {
        Self {
            inner: HistoryViewTranslateBarState::new(target_lang),
            height: 38.0,
        }
    }

    pub fn update_with_tracker(&mut self, tracker: &TranslateTracker, peer_disabled: bool) {
        self.inner.update_from_tracker(tracker, peer_disabled);
    }

    pub fn translate(&mut self) {
        self.inner.trigger_translation();
    }

    pub fn complete(&mut self) {
        self.inner.complete_translation();
    }

    pub fn revert(&mut self) {
        self.inner.revert_to_original();
    }

    pub fn add_skip(&mut self, lang: String) {
        self.inner.add_skip_language(lang);
    }

    pub fn toast_duration(&self) -> std::time::Duration {
        K_TOAST_DURATION
    }
}
