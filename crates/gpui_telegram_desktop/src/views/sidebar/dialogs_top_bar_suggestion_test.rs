//! Test harness helpers for top-bar suggestions.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/dialogs_top_bar_suggestion_test.cpp:L1-138`
//! Subsystem: dialogs, Feature: TCG-N03

#[derive(Debug, Clone, Default)]
pub struct DialogsTopBarSuggestionTest {
    pub pass_count: usize,
}

impl DialogsTopBarSuggestionTest {
    pub fn new() -> Self {
        Self::default()
    }
}
