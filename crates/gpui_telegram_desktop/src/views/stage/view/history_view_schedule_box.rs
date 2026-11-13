//! History View Schedule Box View Component (`history_view_schedule_box`)
//!
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_schedule_box.h` (L1-L96)
//! Reconstructs upstream: `Telegram/SourceFiles/history/view/history_view_schedule_box.cpp` (L1-L436)

use crate::views::stage::scheduled_section::{
    HistoryViewScheduleBoxState, ScheduleBoxType, ScheduleRepeatPeriod,
    API_SCHEDULED_UNTIL_ONLINE_TIMESTAMP, K_NOTIFY_LAST_FRAME, K_NOTIFY_REST_FRAME,
    K_NOTIFY_TOOLTIP_DURATION,
};

/// GPUI view component for message scheduling modal
#[derive(Debug, Clone)]
pub struct HistoryViewScheduleBoxView {
    pub inner: HistoryViewScheduleBoxState,
    pub is_open: bool,
}

impl HistoryViewScheduleBoxView {
    pub fn new(box_type: ScheduleBoxType, is_premium: bool) -> Self {
        Self {
            inner: HistoryViewScheduleBoxState::new(box_type, is_premium),
            is_open: true,
        }
    }

    pub fn schedule_until_online(&mut self) -> bool {
        self.inner.set_until_online()
    }

    pub fn set_repeat(&mut self, period: ScheduleRepeatPeriod) -> bool {
        self.inner.set_repeat_period(period)
    }

    pub fn toggle_silent(&mut self) -> bool {
        self.inner.toggle_silent()
    }

    pub fn online_timestamp_constant(&self) -> i64 {
        API_SCHEDULED_UNTIL_ONLINE_TIMESTAMP
    }

    pub fn notify_rest_frame(&self) -> u32 {
        K_NOTIFY_REST_FRAME
    }

    pub fn notify_last_frame(&self) -> u32 {
        K_NOTIFY_LAST_FRAME
    }

    pub fn notify_tooltip_duration(&self) -> std::time::Duration {
        K_NOTIFY_TOOLTIP_DURATION
    }
}
