//! History Streamed Drafts Engine (AI Agent and Bot Real-Time Token Streaming).
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/history/history_streamed_drafts.h:L24-L75`
//! - Upstream: `Telegram/SourceFiles/history/history_streamed_drafts.cpp:L25-L45`
//! - Anonymous Constants:
//!   - `kClearTimeout = 30 * crl::time(1000);` (30s idle expiry, history_streamed_drafts.cpp:L27)

use std::collections::HashMap;
use gpui::*;

pub const STREAMED_DRAFT_CLEAR_TIMEOUT_MS: u64 = 30_000;

/// Kind of streamed draft payload.
/// C++ citation: `HistoryStreamedDrafts::DraftKind` (history_streamed_drafts.h:L51-L54).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryStreamedDraftKind {
    Text,
    Rich,
}

/// Active streamed draft item state.
/// C++ citation: `HistoryStreamedDrafts::Draft` (history_streamed_drafts.h:L65-L75).
#[derive(Debug, Clone)]
pub struct StreamedDraftItem {
    pub root_id: u64,
    pub from_id: u64,
    pub text: String,
    pub kind: HistoryStreamedDraftKind,
    pub can_stop: bool,
    pub keep_on_stop: bool,
    pub updated_at_ms: u64,
    pub top_msg_id: u64,
    pub is_stopped: bool,
}

/// History streamed drafts manager maintaining live token-generation previews.
/// C++ citation: `HistoryStreamedDrafts` (history_streamed_drafts.h:L24-L50).
pub struct HistoryStreamedDrafts {
    drafts: HashMap<u64, StreamedDraftItem>, // root_id -> draft
}

impl HistoryStreamedDrafts {
    pub fn new() -> Self {
        Self {
            drafts: HashMap::new(),
        }
    }

    /// Applies newly streamed text chunk or updates draft state.
    /// C++ citation: `history_streamed_drafts.cpp:L78-L120`.
    pub fn apply(
        &mut self,
        root_id: u64,
        from_id: u64,
        text: impl Into<String>,
        kind: HistoryStreamedDraftKind,
        can_stop: bool,
        keep_on_stop: bool,
        now_ms: u64,
    ) {
        let entry = self.drafts.entry(root_id).or_insert_with(|| StreamedDraftItem {
            root_id,
            from_id,
            text: String::new(),
            kind,
            can_stop,
            keep_on_stop,
            updated_at_ms: now_ms,
            top_msg_id: 0,
            is_stopped: false,
        });

        entry.text = text.into();
        entry.kind = kind;
        entry.can_stop = can_stop;
        entry.keep_on_stop = keep_on_stop;
        entry.updated_at_ms = now_ms;
    }

    /// Checks if a streamed draft can be interrupted by the user.
    /// C++ citation: `history_streamed_drafts.h:L43`.
    pub fn stoppable_for(&self, root_id: u64) -> bool {
        self.drafts.get(&root_id).map_or(false, |d| d.can_stop && !d.is_stopped)
    }

    /// User requested stopping generation.
    /// C++ citation: `history_streamed_drafts.h:L44`.
    pub fn request_stop(&mut self, root_id: u64) {
        if let Some(draft) = self.drafts.get_mut(&root_id) {
            draft.is_stopped = true;
            if !draft.keep_on_stop {
                self.drafts.remove(&root_id);
            }
        }
    }

    /// Adopts newly finalized incoming message, clearing the corresponding draft.
    /// C++ citation: `history_streamed_drafts.h:L48`.
    pub fn adopt_incoming(&mut self, root_id: u64) -> Option<StreamedDraftItem> {
        self.drafts.remove(&root_id)
    }

    /// Purges abandoned drafts older than `STREAMED_DRAFT_CLEAR_TIMEOUT_MS`.
    /// C++ citation: `history_streamed_drafts.cpp:L27` (`kClearTimeout`).
    pub fn purge_expired(&mut self, now_ms: u64) {
        self.drafts.retain(|_, d| {
            now_ms.saturating_sub(d.updated_at_ms) < STREAMED_DRAFT_CLEAR_TIMEOUT_MS
        });
    }

    pub fn get_draft(&self, root_id: u64) -> Option<&StreamedDraftItem> {
        self.drafts.get(&root_id)
    }

    pub fn has_any(&self) -> bool {
        !self.drafts.is_empty()
    }
}

impl Default for HistoryStreamedDrafts {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryStreamedDrafts {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.drafts.is_empty() {
            return div().into_any_element();
        }

        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .children(self.drafts.values().map(|d| {
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .px(px(8.0))
                    .py(px(4.0))
                    .rounded(px(4.0))
                    .bg(rgb(0xF4F4F5))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x71717A))
                            .child(format!("Generating: {}...", d.text.chars().take(40).collect::<String>()))
                    )
            }))
            .into_any_element()
    }
}
