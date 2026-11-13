//! History Widget Main Conversation Stage Controller.
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/history/history_widget.h:L151-L220`
//! - Upstream: `Telegram/SourceFiles/history/history_widget.cpp:L224-L242`
//! - Anonymous Constants:
//!   - `kMessagesPerPageFirst = 30;` (history_widget.cpp:L226)
//!   - `kMessagesPerPage = 50;` (history_widget.cpp:L227)
//!   - `kPreloadHeightsCount = 3;` (history_widget.cpp:L228)
//!   - `kScrollToVoiceAfterScrolledMs = 1000;` (history_widget.cpp:L229)
//!   - `kSkipRepaintWhileScrollMs = 100;` (history_widget.cpp:L230)
//!   - `kShowMembersDropdownTimeoutMs = 300;` (history_widget.cpp:L231)
//!   - `kDisplayEditTimeWarningMs = 300 * 1000;` (5m edit warning, history_widget.cpp:L232)
//!   - `kFullDayInMs = 86400 * 1000;` (history_widget.cpp:L233)
//!   - `kSaveDraftTimeout = 1000;` (1s debounce, history_widget.cpp:L234)
//!   - `kSaveDraftAnywayTimeout = 5000;` (5s max wait, history_widget.cpp:L235)
//!   - `kSaveCloudDraftIdleTimeout = 14000;` (14s cloud sync, history_widget.cpp:L236)
//!   - `kRefreshSlowmodeLabelTimeout = 200;` (200ms slowmode countdown, history_widget.cpp:L237)
//!   - `kPsaAboutPrefix = "cloud_lng_about_psa_";` (history_widget.cpp:L242)

use gpui::*;

use super::history_inner_widget::{HistoryInnerWidget, MessageItemGeometry};
use super::history_drag_area::HistoryDragArea;
use super::history_view_pull_to_next_channel::HistoryViewPullToNextChannel;
use super::history_view_highlight_manager::{HistoryViewHighlightManager, SelectedQuote};
use super::history_view_top_toast::HistoryViewTopToast;

pub const MESSAGES_PER_PAGE_FIRST: usize = 30;
pub const MESSAGES_PER_PAGE: usize = 50;
pub const PRELOAD_HEIGHTS_COUNT: f32 = 3.0;
pub const DISPLAY_EDIT_TIME_WARNING_MS: u64 = 300_000; // 5 minutes
pub const SAVE_DRAFT_TIMEOUT_MS: u64 = 1000;
pub const SAVE_DRAFT_ANYWAY_TIMEOUT_MS: u64 = 5000;
pub const SAVE_CLOUD_DRAFT_IDLE_TIMEOUT_MS: u64 = 14_000;
pub const REFRESH_SLOWMODE_LABEL_TIMEOUT_MS: u64 = 200;

/// Main conversation stage container widget.
/// C++ citation: `HistoryWidget` (history_widget.h:L151-L220).
pub struct HistoryWidget {
    pub peer_id: u64,
    pub title: String,
    pub subtitle: String,
    pub pinned_text: Option<String>,
    pub is_loaded_at_top: bool,
    pub is_loaded_at_bottom: bool,
    pub unread_count_below: u32,
    pub is_at_bottom: bool,
    pub slowmode_seconds_left: u32,
    pub inner: HistoryInnerWidget,
    pub drag_area: HistoryDragArea,
    pub pull_channel: HistoryViewPullToNextChannel,
    pub highlight_manager: HistoryViewHighlightManager,
    pub top_toast: HistoryViewTopToast,
    last_draft_save_ms: u64,
}

impl HistoryWidget {
    pub fn new(peer_id: u64, title: impl Into<String>) -> Self {
        Self {
            peer_id,
            title: title.into(),
            subtitle: "online".to_string(),
            pinned_text: None,
            is_loaded_at_top: false,
            is_loaded_at_bottom: true,
            unread_count_below: 0,
            is_at_bottom: true,
            slowmode_seconds_left: 0,
            inner: HistoryInnerWidget::new(),
            drag_area: HistoryDragArea::new(),
            pull_channel: HistoryViewPullToNextChannel::new(),
            highlight_manager: HistoryViewHighlightManager::new(),
            top_toast: HistoryViewTopToast::new(),
            last_draft_save_ms: 0,
        }
    }

    /// Sets pinned message banner snippet.
    pub fn set_pinned_message(&mut self, text: Option<String>) {
        self.pinned_text = text;
    }

    /// Whether history is loaded to the earliest available message.
    /// C++ citation: `history_widget.h:L194` (`historyLoadedAtTop`).
    pub fn history_loaded_at_top(&self) -> bool {
        self.is_loaded_at_top
    }

    /// Whether history is loaded to the latest available message.
    /// C++ citation: `history_widget.h:L195` (`historyLoadedAtBottom`).
    pub fn history_loaded_at_bottom(&self) -> bool {
        self.is_loaded_at_bottom
    }

    /// Initial load of first batch of messages (`kMessagesPerPageFirst = 30`).
    /// C++ citation: `history_widget.cpp:L226`.
    pub fn first_load_messages(&mut self, count: usize) -> usize {
        let batch = count.min(MESSAGES_PER_PAGE_FIRST);
        self.is_loaded_at_bottom = true;
        batch
    }

    /// Paginates upward into earlier history (`kMessagesPerPage = 50`).
    /// C++ citation: `history_widget.cpp:L227`.
    pub fn load_messages(&mut self, count: usize) -> usize {
        if self.is_loaded_at_top {
            return 0;
        }
        let batch = count.min(MESSAGES_PER_PAGE);
        batch
    }

    /// Preload trigger check: whether remaining scroll distance is within `kPreloadHeightsCount` screens.
    /// C++ citation: `history_widget.cpp:L228`.
    pub fn should_preload_upward(&self, scroll_top: f32, viewport_height: f32) -> bool {
        if self.is_loaded_at_top {
            return false;
        }
        scroll_top <= (viewport_height * PRELOAD_HEIGHTS_COUNT)
    }

    /// Preload trigger check for downward scrolling.
    pub fn should_preload_downward(&self, scroll_top: f32, viewport_height: f32, total_height: f32) -> bool {
        if self.is_loaded_at_bottom {
            return false;
        }
        let remaining = total_height - (scroll_top + viewport_height);
        remaining <= (viewport_height * PRELOAD_HEIGHTS_COUNT)
    }

    /// Jump to message quote with highlight animation.
    pub fn jump_to_message(&mut self, msg_id: u64, now_ms: u64) {
        self.highlight_manager.enqueue(
            SelectedQuote {
                message_id: msg_id,
                ..Default::default()
            },
            now_ms,
        );
    }

    /// Debounced draft autosave scheduler.
    /// C++ citations: `kSaveDraftTimeout` (1s), `kSaveDraftAnywayTimeout` (5s).
    pub fn should_flush_draft(&self, now_ms: u64, is_dirty: bool) -> bool {
        if !is_dirty {
            return false;
        }
        let elapsed = now_ms.saturating_sub(self.last_draft_save_ms);
        elapsed >= SAVE_DRAFT_TIMEOUT_MS || elapsed >= SAVE_DRAFT_ANYWAY_TIMEOUT_MS
    }

    pub fn record_draft_saved(&mut self, now_ms: u64) {
        self.last_draft_save_ms = now_ms;
    }
}

impl Default for HistoryWidget {
    fn default() -> Self {
        Self::new(0, "Chat")
    }
}

impl Render for HistoryWidget {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let pinned = self.pinned_text.clone();
        let peer_id = self.peer_id;
        let title = self.title.clone();

        div()
            .relative()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(0xFFFFFF))
            // Pinned message banner
            .child(
                if let Some(ref pin) = pinned {
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .h(px(40.0))
                        .px(px(16.0))
                        .bg(rgb(0xF8FAFC))
                        .border_b_1()
                        .border_color(rgb(0xE2E8F0))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .w(px(2.0))
                                        .h(px(20.0))
                                        .bg(rgb(0x2AABEE))
                                        .rounded_full()
                                )
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rgb(0x1E293B))
                                        .child(format!("Pinned: {}", pin))
                                )
                        )
                } else {
                    div()
                }
            )
            // Inner transcript scroll area placeholder
            .child(
                div()
                    .flex_1()
                    .relative()
                    .overflow_hidden()
                    .p(px(16.0))
                    .child(format!("Chat stage for Peer #{} ({})", peer_id, title))
            )
    }
}
