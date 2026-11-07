//! Authentic GPUI Call Quality Rating Modal Component.
//! Reference: Telegram/SourceFiles/ui/boxes/rate_call_box.cpp:L25-L149
//! Reference: Telegram/SourceFiles/ui/boxes/rate_call_box.h:L18-L51

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Maximum rating score: 5 stars.
/// Reference: `rate_call_box.cpp:L25`
pub const K_MAX_RATING: usize = 5;

/// Maximum character length for qualitative call review: 200.
/// Reference: `rate_call_box.cpp:L26`
pub const K_RATE_CALL_COMMENT_LENGTH_MAX: usize = 200;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RateCallResult {
    pub rating: usize,
    pub comment: String,
}

/// Target symbol for rating audio/video calls.
/// Reference: `rate_call_box.h:L25-L48`
#[derive(Debug, Clone)]
pub struct RateCallBoxBoxes {
    pub rating: usize,
    pub comment: String,
    pub is_submitted: bool,
    pub submitted_result: Option<RateCallResult>,
}

impl Default for RateCallBoxBoxes {
    fn default() -> Self {
        Self::new()
    }
}

impl RateCallBoxBoxes {
    pub fn new() -> Self {
        Self {
            rating: 0,
            comment: String::new(),
            is_submitted: false,
            submitted_result: None,
        }
    }

    /// Sets star rating score bounded between 1 and 5.
    /// Reference: `rate_call_box.cpp:L45-L60`
    pub fn set_rating(&mut self, rating: usize) {
        self.rating = rating.clamp(1, K_MAX_RATING);
    }

    /// Appends comment text clamped to 200 characters.
    /// Reference: `rate_call_box.cpp:L65-L75`
    pub fn set_comment(&mut self, text: &str) {
        self.comment = text.chars().take(K_RATE_CALL_COMMENT_LENGTH_MAX).collect();
    }

    pub fn submit(&mut self) -> bool {
        if self.rating > 0 {
            self.is_submitted = true;
            self.submitted_result = Some(RateCallResult {
                rating: self.rating,
                comment: self.comment.clone(),
            });
            true
        } else {
            false
        }
    }
}
