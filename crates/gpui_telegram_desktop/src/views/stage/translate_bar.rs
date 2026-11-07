//! History View Translate Bar & Tracker Component (`history_view_translate_bar`)
//!
//! Real-time chat translation banner and background language recognition engine
//! upstream: `Telegram/SourceFiles/history/view/history_view_translate_bar.h` (L1-L79)
//! upstream: `Telegram/SourceFiles/history/view/history_view_translate_bar.cpp` (L1-L669)
//! upstream: `Telegram/SourceFiles/history/view/history_view_translate_tracker.h` (L1-L88)
//! upstream: `Telegram/SourceFiles/history/view/history_view_translate_tracker.cpp` (L1-L535)

use std::collections::HashSet;
use std::time::Duration;

/// Anonymous namespace constants from `history_view_translate_tracker.cpp:L32-L36`
pub const K_ENOUGH_FOR_RECOGNITION: usize = 10;
pub const K_ENOUGH_FOR_TRANSLATION: usize = 6;
pub const K_MAX_CHECK_IN_BUNCH: usize = 100;
pub const K_REQUEST_LENGTH_LIMIT: usize = 24 * 1024; // 24,576 bytes
pub const K_REQUEST_COUNT_LIMIT: usize = 20;

/// Toast notification duration from `history_view_translate_bar.cpp:L45`
pub const K_TOAST_DURATION: Duration = Duration::from_millis(4000);

/// Background language recognition and batching tracker
/// Upstream: `HistoryView::TranslateTracker`
#[derive(Debug, Clone)]
pub struct TranslateTracker {
    pub history_peer_id: i64,
    pub is_auto_translation_channel: bool,
    pub detected_language: Option<String>,
    pub accumulated_characters: usize,
    pub is_tracking_language: bool,
}

impl TranslateTracker {
    pub fn new(peer_id: i64, is_channel_auto: bool) -> Self {
        Self {
            history_peer_id: peer_id,
            is_auto_translation_channel: is_channel_auto,
            detected_language: None,
            accumulated_characters: 0,
            is_tracking_language: true,
        }
    }

    /// Feed incoming message text to the recognition buffer
    pub fn feed_text(&mut self, text: &str) -> bool {
        self.accumulated_characters += text.chars().count();
        if self.detected_language.is_none() && self.accumulated_characters >= K_ENOUGH_FOR_RECOGNITION {
            // Language recognition threshold reached
            true
        } else {
            false
        }
    }

    pub fn set_detected_language(&mut self, lang: String) {
        self.detected_language = Some(lang);
    }

    /// Determine if translation bar should be displayed for this chat
    pub fn should_offer_translation(
        &self,
        target_lang: &str,
        skip_languages: &HashSet<String>,
        translation_disabled_for_peer: bool,
    ) -> bool {
        if translation_disabled_for_peer {
            return false;
        }
        if let Some(ref detected) = self.detected_language {
            if detected.eq_ignore_ascii_case(target_lang) {
                return false;
            }
            if skip_languages.contains(detected) {
                return false;
            }
            true
        } else {
            false
        }
    }
}

/// Translation Banner state
/// Upstream: `HistoryView::TranslateBar`
#[derive(Debug, Clone)]
pub struct HistoryViewTranslateBarState {
    pub is_visible: bool,
    pub from_lang: Option<String>,
    pub to_lang: String,
    pub skip_languages: HashSet<String>,
    pub is_translating: bool,
    pub is_translated: bool,
    pub show_toast: bool,
}

impl HistoryViewTranslateBarState {
    pub fn new(target_lang: String) -> Self {
        Self {
            is_visible: false,
            from_lang: None,
            to_lang: target_lang,
            skip_languages: HashSet::new(),
            is_translating: false,
            is_translated: false,
            show_toast: false,
        }
    }

    pub fn update_from_tracker(&mut self, tracker: &TranslateTracker, peer_disabled: bool) {
        if tracker.should_offer_translation(&self.to_lang, &self.skip_languages, peer_disabled) {
            self.from_lang = tracker.detected_language.clone();
            self.is_visible = true;
        } else {
            self.is_visible = false;
        }
    }

    pub fn trigger_translation(&mut self) {
        self.is_translating = true;
    }

    pub fn complete_translation(&mut self) {
        self.is_translating = false;
        self.is_translated = true;
        self.show_toast = true;
    }

    pub fn revert_to_original(&mut self) {
        self.is_translated = false;
    }

    pub fn add_skip_language(&mut self, lang: String) {
        self.skip_languages.insert(lang);
        if let Some(ref from) = self.from_lang {
            if self.skip_languages.contains(from) {
                self.is_visible = false;
            }
        }
    }
}
