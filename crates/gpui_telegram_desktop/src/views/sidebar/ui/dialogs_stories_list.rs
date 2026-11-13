//! Horizontal stories preview list above conversation list.
//!
//! Based on upstream Telegram Desktop:
//! - `Telegram/SourceFiles/dialogs/ui/dialogs_stories_list.h:L34-75`
//! - `Telegram/SourceFiles/dialogs/ui/dialogs_stories_list.cpp:L37-75`
//!
//! Subsystem: dialogs, Feature: TCG-N03

use serde::{Deserialize, Serialize};

/// Upstream constants from `dialogs_stories_list.cpp:L37-43`.
pub const K_SMALL_THUMBS_SHOWN: usize = 3;
pub const K_PRELOAD_PAGES: usize = 2;
pub const K_EXPAND_AFTER_RATIO: f32 = 0.72;
pub const K_COLLAPSE_AFTER_RATIO: f32 = 0.68;
pub const K_FRICTION_RATIO: f32 = 0.15;
pub const K_EXPAND_CATCH_UP_DURATION_MS: u64 = 200;
pub const K_MAX_TOOLTIP_NAMES: usize = 3;

/// Single story element in the carousel (`dialogs_stories_list.h:L34-46`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryElement {
    pub id: u64,
    pub peer_name: String,
    pub avatar_initials: String,
    pub avatar_color: u32,
    pub story_count: u32,
    pub unread_count: u32,
    pub has_video_stream: bool,
}

/// Stories list state and layout coordinator (`dialogs_stories_list.cpp:L54-75`).
#[derive(Debug, Clone)]
pub struct DialogsStoriesListUi {
    pub elements: Vec<StoryElement>,
    pub is_expanded: bool,
    pub expand_ratio: f32,
    pub active_story_id: Option<u64>,
}

impl Default for DialogsStoriesListUi {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
            is_expanded: false,
            expand_ratio: 0.0,
            active_story_id: None,
        }
    }
}

impl DialogsStoriesListUi {
    pub fn new() -> Self {
        Self::default()
    }

    /// Determines if drag expansion ratio crosses the threshold to snap expanded (`dialogs_stories_list.cpp:L39`).
    pub fn should_expand(drag_ratio: f32) -> bool {
        drag_ratio >= K_EXPAND_AFTER_RATIO
    }

    /// Determines if drag collapse ratio crosses the threshold to snap collapsed (`dialogs_stories_list.cpp:L40`).
    pub fn should_collapse(drag_ratio: f32) -> bool {
        drag_ratio <= K_COLLAPSE_AFTER_RATIO
    }

    /// Number of small preview thumbnails displayed when collapsed (`dialogs_stories_list.cpp:L37`).
    pub fn collapsed_preview_count(&self) -> usize {
        self.elements.len().min(K_SMALL_THUMBS_SHOWN)
    }

    pub fn set_elements(&mut self, items: Vec<StoryElement>) {
        self.elements = items;
    }

    pub fn toggle_expanded(&mut self) {
        self.is_expanded = !self.is_expanded;
        self.expand_ratio = if self.is_expanded { 1.0 } else { 0.0 };
    }
}
