//! History View Pinned Bar, Tracker & Section Component (`history_view_pinned_bar`)
//!
//! Pinned message banner, multi-pinned cycling tracker, and pinned section view
//! upstream: `Telegram/SourceFiles/history/view/history_view_pinned_bar.h` (L1-L67)
//! upstream: `Telegram/SourceFiles/history/view/history_view_pinned_bar.cpp` (L1-L409)
//! upstream: `Telegram/SourceFiles/history/view/history_view_pinned_tracker.h` (L1-L63)
//! upstream: `Telegram/SourceFiles/history/view/history_view_pinned_tracker.cpp` (L1-L186)
//! upstream: `Telegram/SourceFiles/history/view/history_view_pinned_section.h` (L1-L246)
//! upstream: `Telegram/SourceFiles/history/view/history_view_pinned_section.cpp` (L1-L853)

/// Preview media loading states
/// Upstream: `history_view_pinned_bar.cpp:L136-L146`
pub const K_FULL_LOADED: usize = 2;
pub const K_SOME_LOADED: usize = 1;
pub const K_NOT_LOADED: usize = 0;

/// Tracker limits
/// Upstream: `history_view_pinned_tracker.cpp:L25-L26`
pub const K_LOADED_LIMIT: usize = 5;
pub const K_CHANGE_VIEWER_LIMIT: usize = 2;

/// Server message ID boundary for universal projection
pub const SERVER_MAX_MSG_ID: i64 = 0x3FFFFFFF;

/// Format pinned message banner title
/// Upstream: `history_view_pinned_bar.cpp:L201-L210`: `WithPinnedTitle`
pub fn format_pinned_title(index: usize, count: usize) -> String {
    let count = count.max(1);
    let clamped_index = index.min(count - 1);
    if clamped_index + 1 >= count {
        "Pinned Message".to_string()
    } else if count == 2 {
        "Previous Pinned Message".to_string()
    } else {
        format!("Pinned Message #{}", clamped_index + 1)
    }
}

/// Project message ID for universal non-colliding history access
/// Upstream: `history_view_pinned_tracker.cpp:L76-L85`: `nextPinnedId` projections
pub fn project_universal_msg_id(is_channel: bool, msg_id: i64) -> i64 {
    if is_channel {
        msg_id
    } else {
        msg_id - SERVER_MAX_MSG_ID
    }
}

/// Multi-pinned message cyclical tracker
/// Upstream: `HistoryView::PinnedTracker`
#[derive(Debug, Clone)]
pub struct PinnedTracker {
    pub is_channel: bool,
    pub pinned_ids: Vec<i64>,
    pub current_index: usize,
}

impl PinnedTracker {
    pub fn new(is_channel: bool) -> Self {
        Self {
            is_channel,
            pinned_ids: Vec::new(),
            current_index: 0,
        }
    }

    pub fn set_pinned_ids(&mut self, ids: Vec<i64>) {
        self.pinned_ids = ids;
        if self.current_index >= self.pinned_ids.len() {
            self.current_index = 0;
        }
    }

    pub fn current_msg_id(&self) -> Option<i64> {
        self.pinned_ids.get(self.current_index).copied()
    }

    pub fn count(&self) -> usize {
        self.pinned_ids.len()
    }

    /// Advance to the next pinned message, wrapping around cyclically
    pub fn next_pinned_id(&mut self) -> Option<i64> {
        if self.pinned_ids.is_empty() {
            return None;
        }
        self.current_index = (self.current_index + 1) % self.pinned_ids.len();
        self.current_msg_id()
    }

    pub fn reset_to_latest(&mut self) {
        if !self.pinned_ids.is_empty() {
            self.current_index = self.pinned_ids.len() - 1;
        } else {
            self.current_index = 0;
        }
    }
}

/// Pinned Message Bar Widget state
/// Upstream: `HistoryView::PinnedBar`
#[derive(Debug, Clone)]
pub struct HistoryViewPinnedBarState {
    pub is_visible: bool,
    pub tracker: PinnedTracker,
    pub title: String,
    pub preview_text: String,
    pub has_media_preview: bool,
    pub is_spoiler: bool,
    pub load_level: usize,
}

impl HistoryViewPinnedBarState {
    pub fn new(is_channel: bool) -> Self {
        Self {
            is_visible: false,
            tracker: PinnedTracker::new(is_channel),
            title: "Pinned Message".to_string(),
            preview_text: String::new(),
            has_media_preview: false,
            is_spoiler: false,
            load_level: K_NOT_LOADED,
        }
    }

    pub fn update_content(&mut self, text: String, has_media: bool, spoiler: bool) {
        self.preview_text = text;
        self.has_media_preview = has_media;
        self.is_spoiler = spoiler;
        self.title = format_pinned_title(self.tracker.current_index, self.tracker.count());
    }

    pub fn on_bar_clicked(&mut self) -> Option<i64> {
        let next_id = self.tracker.next_pinned_id();
        if let Some(_) = next_id {
            self.title = format_pinned_title(self.tracker.current_index, self.tracker.count());
        }
        next_id
    }
}

/// Separate view section listing all pinned messages
/// Upstream: `HistoryView::PinnedMemento` & `HistoryView::PinnedWidget`
#[derive(Debug, Clone)]
pub struct PinnedSectionMemento {
    pub peer_id: i64,
    pub topic_id: Option<i64>,
    pub highlight_msg_id: Option<i64>,
}

impl PinnedSectionMemento {
    pub fn new(peer_id: i64, topic_id: Option<i64>, highlight_msg_id: Option<i64>) -> Self {
        Self {
            peer_id,
            topic_id,
            highlight_msg_id,
        }
    }
}
