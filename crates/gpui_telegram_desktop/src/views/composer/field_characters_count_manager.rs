//! Field Characters Count Manager and Over-Limit Indicator
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/field_characters_count_manager.h:L1-L25`
//! - `Telegram/SourceFiles/chat_helpers/field_characters_count_manager.cpp:L1-L37`
//! - `Telegram/SourceFiles/chat_helpers/field_characters_count_manager.cpp:L16` (`kMax` threshold)
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

/// Characters count manager tracking composer text length against limits
/// (e.g. 4096 chars for standard messages, 1024 for media captions).
/// Target symbol in ledger: `FieldCharactersCountManager`.
#[derive(Debug, Clone)]
pub struct FieldCharactersCountManager {
    pub current_count: usize,
    pub previous_count: usize,
    pub max_limit: usize,
    pub is_limit_exceeded: bool,
    pub limit_crossed: bool,
}

impl Default for FieldCharactersCountManager {
    fn default() -> Self {
        Self {
            current_count: 0,
            previous_count: 0,
            max_limit: 4096,
            is_limit_exceeded: false,
            limit_crossed: false,
        }
    }
}

impl FieldCharactersCountManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_limit(max_limit: usize) -> Self {
        Self {
            max_limit,
            ..Self::default()
        }
    }

    /// Update text length count and check boundary transitions per `field_characters_count_manager.cpp:L12-L24`.
    pub fn set_count(&mut self, count: usize) {
        self.previous_count = self.current_count;
        self.current_count = count;

        let was = self.previous_count > self.max_limit;
        let now = self.current_count > self.max_limit;
        self.is_limit_exceeded = now;
        self.limit_crossed = was != now;
    }

    pub fn count(&self) -> usize {
        self.current_count
    }

    pub fn remaining(&self) -> i64 {
        (self.max_limit as i64) - (self.current_count as i64)
    }

    pub fn is_exceeded(&self) -> bool {
        self.is_limit_exceeded
    }
}
