//! History View Translate Tracker View Component (`history_view_translate_tracker`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_translate_tracker.h` (L1-L88)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_translate_tracker.cpp` (L1-L535)

use crate::views::stage::translate_bar::{
    TranslateTracker, K_ENOUGH_FOR_RECOGNITION, K_ENOUGH_FOR_TRANSLATION,
    K_MAX_CHECK_IN_BUNCH, K_REQUEST_COUNT_LIMIT, K_REQUEST_LENGTH_LIMIT,
};

/// GPUI view component for translation language tracker
#[derive(Debug, Clone)]
pub struct HistoryViewTranslateTrackerView {
    pub inner: TranslateTracker,
}

impl HistoryViewTranslateTrackerView {
    pub fn new(peer_id: i64, is_channel_auto: bool) -> Self {
        Self {
            inner: TranslateTracker::new(peer_id, is_channel_auto),
        }
    }

    pub fn feed(&mut self, text: &str) -> bool {
        self.inner.feed_text(text)
    }

    pub fn set_lang(&mut self, lang: String) {
        self.inner.set_detected_language(lang);
    }

    pub fn enough_recognition_threshold(&self) -> usize {
        K_ENOUGH_FOR_RECOGNITION
    }

    pub fn enough_translation_threshold(&self) -> usize {
        K_ENOUGH_FOR_TRANSLATION
    }

    pub fn max_check_in_bunch(&self) -> usize {
        K_MAX_CHECK_IN_BUNCH
    }

    pub fn request_length_limit(&self) -> usize {
        K_REQUEST_LENGTH_LIMIT
    }

    pub fn request_count_limit(&self) -> usize {
        K_REQUEST_COUNT_LIMIT
    }
}
