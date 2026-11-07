//! History View Read Metrics Tracker View Component (`history_view_read_metrics_tracker`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_read_metrics_tracker.h` (L1-L79)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_read_metrics_tracker.cpp` (L1-L346)

use std::collections::HashMap;
use std::time::Duration;

/// Anonymous namespace constants from `history_view_read_metrics_tracker.cpp:L21-L25`
pub const K_READ_METRICS_GRACE_PERIOD: Duration = Duration::from_millis(300);
pub const K_READ_METRICS_ACTIVITY_TIMEOUT: Duration = Duration::from_millis(15000);
pub const K_READ_METRICS_MAX_TRACKING_DURATION: Duration = Duration::from_millis(300000);
pub const K_READ_METRICS_MIN_REPORT_THRESHOLD: Duration = Duration::from_millis(300);

/// Tracked reading duration per message
#[derive(Debug, Clone, PartialEq)]
pub struct TrackedMessageReading {
    pub message_id: i64,
    pub cumulative_visible_duration: Duration,
    pub is_reported: bool,
}

/// GPUI view component for time-in-viewport reading metrics telemetry
#[derive(Debug, Clone)]
pub struct HistoryViewReadMetricsTrackerView {
    pub peer_id: i64,
    pub is_app_active: bool,
    pub tracked_messages: HashMap<i64, TrackedMessageReading>,
}

impl HistoryViewReadMetricsTrackerView {
    pub fn new(peer_id: i64) -> Self {
        Self {
            peer_id,
            is_app_active: true,
            tracked_messages: HashMap::new(),
        }
    }

    pub fn set_app_active(&mut self, active: bool) {
        self.is_app_active = active;
    }

    pub fn record_visibility(&mut self, message_id: i64, delta: Duration) {
        if !self.is_app_active {
            return;
        }
        let entry = self.tracked_messages.entry(message_id).or_insert(TrackedMessageReading {
            message_id,
            cumulative_visible_duration: Duration::ZERO,
            is_reported: false,
        });

        entry.cumulative_visible_duration = (entry.cumulative_visible_duration + delta)
            .min(K_READ_METRICS_MAX_TRACKING_DURATION);
    }

    pub fn is_ready_for_report(&self, message_id: i64) -> bool {
        if let Some(entry) = self.tracked_messages.get(&message_id) {
            !entry.is_reported && entry.cumulative_visible_duration >= K_READ_METRICS_MIN_REPORT_THRESHOLD
        } else {
            false
        }
    }

    pub fn mark_reported(&mut self, message_id: i64) {
        if let Some(entry) = self.tracked_messages.get_mut(&message_id) {
            entry.is_reported = true;
        }
    }

    pub fn grace_period(&self) -> Duration {
        K_READ_METRICS_GRACE_PERIOD
    }

    pub fn activity_timeout(&self) -> Duration {
        K_READ_METRICS_ACTIVITY_TIMEOUT
    }

    pub fn max_tracking_duration(&self) -> Duration {
        K_READ_METRICS_MAX_TRACKING_DURATION
    }
}
