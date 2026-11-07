//! History View Group Call Bar View Component (`history_view_group_call_bar`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_group_call_bar.h` (L1-L65)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_group_call_bar.cpp` (L1-L452)
//!
//! Manages active group call banner shown atop chat history, userpic avatar stacking with
//! speaker priority sorting, and call join/leave state transitions.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Maximum visible speaker userpics in group call bar.
/// Upstream: `history_view_group_call_bar.cpp:L132` (`constexpr auto kLimit = 3;`)
pub const K_GROUP_CALL_USERPICS_LIMIT: usize = 3;

/// Bit shift mask for speaking participant sorting priority.
/// Upstream: `history_view_group_call_bar.cpp:L119` (`uint64(0x100000000ULL)`)
pub const K_SORT_KEY_SPEAKING_FLAG: u64 = 0x1_0000_0000;

/// Represents an individual participant userpic in the stacked avatar row.
/// Upstream: `history_view_group_call_bar.h:L27-L32` (`struct UserpicInRow`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserpicInRow {
    pub peer_id: i64,
    pub is_speaking: bool,
    pub unique_key: u64,
}

/// Upstream avatar stack width calculation.
/// Upstream: `history_view_group_call_bar.cpp:L39` (`single + (limit - 1) * (single - shift)`)
#[inline]
pub fn calculate_avatar_stack_width(participant_num: usize, single_size: f32, shift: f32) -> f32 {
    if participant_num == 0 {
        0.0
    } else {
        let clamped = participant_num.min(K_GROUP_CALL_USERPICS_LIMIT);
        single_size + (clamped - 1) as f32 * (single_size - shift)
    }
}

/// Computes participant priority sort key (speaking first, then most recently active).
/// Upstream: `history_view_group_call_bar.cpp:L118-L122` (`SortKey`)
#[inline]
pub fn compute_participant_sort_key(is_speaking: bool, last_active: u64, date: u64) -> u64 {
    let speaking_weight = if is_speaking { K_SORT_KEY_SPEAKING_FLAG } else { 0 };
    let time_weight = last_active.max(date);
    speaking_weight | time_weight
}

/// Checks whether userpic row needs visual regeneration.
/// Upstream: `history_view_group_call_bar.cpp:L67-L83` (`NeedRegenerateUserpics`)
pub fn need_regenerate_userpics(current: &[UserpicInRow], incoming: &[UserpicInRow]) -> bool {
    if current.len() != incoming.len() {
        return true;
    }
    for (a, b) in current.iter().zip(incoming.iter()) {
        if a.peer_id != b.peer_id || a.is_speaking != b.is_speaking || a.unique_key != b.unique_key {
            return true;
        }
    }
    false
}

/// GPUI view component for active group call banner.
/// Upstream: `history_view_group_call_bar.h:L34-L64`, `history_view_group_call_bar.cpp:L104-L245`
#[derive(Debug, Clone)]
pub struct HistoryViewGroupCallBarView {
    pub call_id: i64,
    pub peer_id: i64,
    pub title: String,
    pub total_participants: usize,
    pub active_speaker_ids: Vec<i64>,
    pub stacked_userpics: Vec<UserpicInRow>,
    pub is_muted: bool,
    pub is_joined: bool,
    pub is_active_banner: bool,
}

impl HistoryViewGroupCallBarView {
    /// Creates a new group call bar view.
    /// Upstream: `history_view_group_call_bar.cpp:L104-L141`
    pub fn new(call_id: i64, peer_id: i64, title: String) -> Self {
        Self {
            call_id,
            peer_id,
            title,
            total_participants: 0,
            active_speaker_ids: Vec::new(),
            stacked_userpics: Vec::new(),
            is_muted: true,
            is_joined: false,
            is_active_banner: true,
        }
    }

    /// Sets the list of active speaker IDs.
    pub fn set_speakers(&mut self, speakers: Vec<i64>) {
        self.active_speaker_ids = speakers.clone();
        self.stacked_userpics = speakers
            .into_iter()
            .take(K_GROUP_CALL_USERPICS_LIMIT)
            .map(|id| UserpicInRow {
                peer_id: id,
                is_speaking: true,
                unique_key: id as u64,
            })
            .collect();
    }

    /// Calculates avatar stack width given single userpic diameter and overlap shift.
    /// Upstream: `history_view_group_call_bar.cpp:L39`
    pub fn avatar_stack_width(&self, size: f32, shift: f32) -> f32 {
        calculate_avatar_stack_width(self.active_speaker_ids.len(), size, shift)
    }

    /// Maximum speakers shown concurrently.
    pub fn speaker_limit(&self) -> usize {
        K_GROUP_CALL_USERPICS_LIMIT
    }

    /// Updates voice chat join/mute state.
    pub fn update_call_state(&mut self, joined: bool, muted: bool) {
        self.is_joined = joined;
        self.is_muted = muted;
    }
}

impl Default for HistoryViewGroupCallBarView {
    fn default() -> Self {
        Self::new(0, 0, "Group Call".to_string())
    }
}

impl Render for HistoryViewGroupCallBarView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let avatar_w = self.avatar_stack_width(28.0, 8.0);
        let joined = self.is_joined;

        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(40.0))
            .px(px(12.0))
            .bg(if joined { rgb(0x27A6E5) } else { rgb(0x3B424B) })
            .border_b_1()
            .border_color(rgb(0x2C323B))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    // Call icon / avatar stack
                    .child(
                        div()
                            .w(px(avatar_w.max(28.0)))
                            .h(px(28.0))
                            .flex()
                            .items_center()
                            .child(
                                if self.stacked_userpics.is_empty() {
                                    div()
                                        .w(px(28.0))
                                        .h(px(28.0))
                                        .rounded_full()
                                        .bg(rgb(0x4A90E2))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(14.0))
                                        .child("📞")
                                } else {
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(rgb(0xFFFFFF))
                                        .child(format!("🗣 {}", self.stacked_userpics.len()))
                                }
                            )
                    )
                    // Call title and participant count
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(self.title.clone())
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(rgb(0xD0E5F5))
                                    .child(format!("{} in call", self.total_participants))
                            )
                    )
            )
            // Join / Leave button
            .child(
                div()
                    .px(px(12.0))
                    .py(px(4.0))
                    .rounded(px(14.0))
                    .bg(if joined { rgb(0xEE4444) } else { rgb(0x40B34F) })
                    .text_color(rgb(0xFFFFFF))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(if joined { "Leave" } else { "Join" })
            )
    }
}
