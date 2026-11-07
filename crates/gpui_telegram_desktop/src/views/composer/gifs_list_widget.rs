//! Saved GIFs Grid and Inline Search Viewer
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/gifs_list_widget.h:L1-L227`
//! - `Telegram/SourceFiles/chat_helpers/gifs_list_widget.cpp:L1-L1040`
//! - `Telegram/SourceFiles/chat_helpers/gifs_list_widget.cpp:L52-L55` (`kSearchRequestDelay`, `kMinRepaintDelay`)
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

use std::time::Duration;

pub const K_SEARCH_REQUEST_DELAY: Duration = Duration::from_millis(400);
pub const K_MIN_REPAINT_DELAY: Duration = Duration::from_millis(33);
pub const K_MIN_AFTER_SCROLL_DELAY: Duration = Duration::from_millis(33);

#[derive(Debug, Clone, PartialEq)]
pub struct GifItem {
    pub document_id: u64,
    pub width: u32,
    pub height: u32,
    pub duration_seconds: f32,
    pub is_saved: bool,
    pub thumbnail_url: String,
    pub stream_url: String,
}

/// Gifs List Widget managing saved GIFs and inline search results.
/// Target symbol in ledger: `GifsListWidget`.
#[derive(Debug, Clone)]
pub struct GifsListWidget {
    pub saved_gifs: Vec<GifItem>,
    pub search_results: Vec<GifItem>,
    pub is_searching: bool,
    pub search_query: String,
    pub hovered_document_id: Option<u64>,
    pub selected_document_id: Option<u64>,
}

impl Default for GifsListWidget {
    fn default() -> Self {
        Self {
            saved_gifs: Vec::new(),
            search_results: Vec::new(),
            is_searching: false,
            search_query: String::new(),
            hovered_document_id: None,
            selected_document_id: None,
        }
    }
}

impl GifsListWidget {
    pub fn new() -> Self {
        Self::default()
    }

    /// Toggle saved state for GIF item per `gifs_list_widget.cpp:L58-L89`
    pub fn toggle_saved_gif(&mut self, document_id: u64) -> bool {
        if let Some(pos) = self.saved_gifs.iter().position(|g| g.document_id == document_id) {
            self.saved_gifs.remove(pos);
            false
        } else if let Some(item) = self.search_results.iter().find(|g| g.document_id == document_id) {
            let mut saved = item.clone();
            saved.is_saved = true;
            self.saved_gifs.insert(0, saved);
            true
        } else {
            false
        }
    }

    pub fn set_search_query(&mut self, query: &str) {
        self.search_query = query.trim().to_string();
        self.is_searching = !self.search_query.is_empty();
    }

    pub fn visible_items(&self) -> &[GifItem] {
        if self.is_searching {
            &self.search_results
        } else {
            &self.saved_gifs
        }
    }
}
