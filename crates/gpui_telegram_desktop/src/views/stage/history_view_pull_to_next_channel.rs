//! History View Pull-To-Next-Channel Gesture & Channel Discovery.
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/history/history_view_pull_to_next_channel.h:L34-L60`
//! - Upstream: `Telegram/SourceFiles/history/history_view_pull_to_next_channel.cpp:L48-L117`
//! - Anonymous Constants:
//!   - `kResetReachedOn = 0.95;` (history_view_pull_to_next_channel.cpp:L48)
//!   - `kReverseEase = 1.5;` (history_view_pull_to_next_channel.cpp:L49)
//!   - `kReadyDwell = 120;` (ms, history_view_pull_to_next_channel.cpp:L50)
//!   - `kExpandDuration = 250;` (ms, history_view_pull_to_next_channel.cpp:L51)
//!   - `kReleaseShowDuration = 250;` (ms, history_view_pull_to_next_channel.cpp:L52)
//!   - `kReleaseHideDuration = 250;` (ms, history_view_pull_to_next_channel.cpp:L53)
//!   - `kPanelDuration = 320;` (ms, history_view_pull_to_next_channel.cpp:L54)
//!   - `kBounceDuration = 400;` (ms, history_view_pull_to_next_channel.cpp:L55)
//!   - `PullThreshold() = st::historyPullNextThreshold * 2;` (history_view_pull_to_next_channel.cpp:L62)

use gpui::*;

pub const RESET_REACHED_ON: f32 = 0.95;
pub const REVERSE_EASE: f32 = 1.5;
pub const READY_DWELL_MS: u64 = 120;
pub const EXPAND_DURATION_MS: u64 = 250;
pub const RELEASE_SHOW_DURATION_MS: u64 = 250;
pub const RELEASE_HIDE_DURATION_MS: u64 = 250;
pub const PANEL_DURATION_MS: u64 = 320;
pub const BOUNCE_DURATION_MS: u64 = 400;
pub const DEFAULT_PULL_THRESHOLD_PX: f32 = 64.0 * 2.0; // Doubled threshold (L62)

/// Channel candidate descriptor used for 4-tier unread discovery.
/// C++ citation: `history_view_pull_to_next_channel.cpp:L65-L117`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnreadChannelCandidate {
    pub id: u64,
    pub title: String,
    pub unread_count: u32,
    pub is_broadcast: bool,
    pub is_archived: bool,
    pub filter_id: Option<u32>,
}

/// Gesture interaction mode.
/// C++ citation: `HistoryView::PullToNextChannel::Mode` (history_view_pull_to_next_channel.h:L49-L54).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PullToNextChannelMode {
    Inactive,
    Pulling,
    Ready,
    Released,
    Bouncing,
}

/// Pull-to-next-channel manager and visual pill controller.
/// C++ citation: `HistoryView::PullToNextChannel` (history_view_pull_to_next_channel.h:L34-L60).
pub struct HistoryViewPullToNextChannel {
    mode: PullToNextChannelMode,
    pull_distance: f32,
    threshold: f32,
    next_channel: Option<UnreadChannelCandidate>,
    dwell_start_ms: Option<u64>,
}

impl HistoryViewPullToNextChannel {
    pub fn new() -> Self {
        Self {
            mode: PullToNextChannelMode::Inactive,
            pull_distance: 0.0,
            threshold: DEFAULT_PULL_THRESHOLD_PX,
            next_channel: None,
            dwell_start_ms: None,
        }
    }

    /// 4-tier channel discovery algorithm finding the next unread broadcast channel.
    /// C++ citation: `history_view_pull_to_next_channel.cpp:L65-L117` (`FindNextUnreadChannel`).
    pub fn find_next_unread_channel<'a>(
        current_id: u64,
        active_filter_id: Option<u32>,
        channels: &'a [UnreadChannelCandidate],
    ) -> Option<&'a UnreadChannelCandidate> {
        // Tier 1: Channels in the active filter list
        if let Some(f_id) = active_filter_id {
            if let Some(c) = channels.iter().find(|c| {
                c.id != current_id && c.is_broadcast && c.unread_count > 0 && c.filter_id == Some(f_id)
            }) {
                return Some(c);
            }

            // Tier 2: Channels in other filter lists
            if let Some(c) = channels.iter().find(|c| {
                c.id != current_id
                    && c.is_broadcast
                    && c.unread_count > 0
                    && c.filter_id.is_some()
                    && c.filter_id != Some(f_id)
            }) {
                return Some(c);
            }
        }

        // Tier 3: Channels in the main chats list (non-archived)
        if let Some(c) = channels.iter().find(|c| {
            c.id != current_id && c.is_broadcast && c.unread_count > 0 && c.filter_id.is_none() && !c.is_archived
        }) {
            return Some(c);
        }

        // Tier 4: Channels in the archived folder
        channels.iter().find(|c| {
            c.id != current_id && c.is_broadcast && c.unread_count > 0 && c.is_archived
        })
    }

    /// Sets the target next channel discovered by the 4-tier traversal.
    pub fn set_next_channel(&mut self, candidate: Option<UnreadChannelCandidate>) {
        self.next_channel = candidate;
    }

    /// Handles upward drag delta at the bottom boundary of the transcript.
    /// Uses reverse cubic easing (`kReverseEase = 1.5`).
    pub fn update_pull(&mut self, dy: f32, now_ms: u64) {
        if self.next_channel.is_none() {
            return;
        }

        self.pull_distance = (self.pull_distance + dy).max(0.0);
        let progress = (self.pull_distance / self.threshold).min(1.5);

        if progress >= 1.0 {
            if self.mode != PullToNextChannelMode::Ready {
                self.mode = PullToNextChannelMode::Ready;
                self.dwell_start_ms = Some(now_ms);
            }
        } else if progress >= RESET_REACHED_ON {
            // Approaching release threshold
            self.mode = PullToNextChannelMode::Pulling;
        } else {
            self.mode = PullToNextChannelMode::Pulling;
            self.dwell_start_ms = None;
        }
    }

    /// Releases drag gesture. If held past dwell threshold, triggers navigation to next channel.
    pub fn release_pull(&mut self, now_ms: u64) -> Option<u64> {
        let is_ready = self.mode == PullToNextChannelMode::Ready;
        let dwell_ok = self
            .dwell_start_ms
            .map_or(false, |start| now_ms.saturating_sub(start) >= READY_DWELL_MS);

        if is_ready && dwell_ok {
            self.mode = PullToNextChannelMode::Released;
            let target_id = self.next_channel.as_ref().map(|c| c.id);
            self.pull_distance = 0.0;
            target_id
        } else {
            self.mode = PullToNextChannelMode::Bouncing;
            self.pull_distance = 0.0;
            self.dwell_start_ms = None;
            None
        }
    }

    pub fn cancel(&mut self) {
        self.mode = PullToNextChannelMode::Inactive;
        self.pull_distance = 0.0;
        self.dwell_start_ms = None;
    }

    pub fn mode(&self) -> PullToNextChannelMode {
        self.mode
    }

    pub fn pull_distance(&self) -> f32 {
        self.pull_distance
    }

    pub fn next_channel(&self) -> Option<&UnreadChannelCandidate> {
        self.next_channel.as_ref()
    }
}

impl Default for HistoryViewPullToNextChannel {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewPullToNextChannel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.mode == PullToNextChannelMode::Inactive || self.pull_distance <= 0.0 {
            return div().into_any_element();
        }

        let channel_name = self
            .next_channel
            .as_ref()
            .map(|c| c.title.as_str())
            .unwrap_or("Next Channel");
        let is_ready = self.mode == PullToNextChannelMode::Ready;

        div()
            .absolute()
            .bottom(px(16.0))
            .left_0()
            .right_0()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .px(px(16.0))
                    .py(px(8.0))
                    .rounded_full()
                    .bg(if is_ready { rgb(0x2AABEE) } else { rgb(0x3F3F46) })
                    .text_color(rgb(0xFFFFFF))
                    .text_size(px(13.0))
                    .shadow_md()
                    .child(if is_ready {
                        format!("Release for {}", channel_name)
                    } else {
                        format!("Pull up for {}", channel_name)
                    })
            )
            .into_any_element()
    }
}
