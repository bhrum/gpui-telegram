//! History View Scheduled Section & Schedule Box Component (`history_view_scheduled_section`)
//!
//! Dedicated view section for scheduled messages and scheduling configuration modal
//! upstream: `Telegram/SourceFiles/history/view/history_view_scheduled_section.h` (L1-L362)
//! upstream: `Telegram/SourceFiles/history/view/history_view_scheduled_section.cpp` (L1-L1882)
//! upstream: `Telegram/SourceFiles/history/view/history_view_schedule_box.h` (L1-L96)
//! upstream: `Telegram/SourceFiles/history/view/history_view_schedule_box.cpp` (L1-L436)

use std::time::Duration;

/// Anonymous namespace constants
pub const K_VIDEO_PROCESSING_INFO_DURATION: Duration = Duration::from_millis(4000);
pub const K_NOTIFY_REST_FRAME: u32 = 40;
pub const K_NOTIFY_LAST_FRAME: u32 = 80;
pub const K_NOTIFY_TOOLTIP_DURATION: Duration = Duration::from_millis(5000);

/// Magic timestamp signifying "Send When User Comes Online"
/// Upstream: `Api::kScheduledUntilOnlineTimestamp`
pub const API_SCHEDULED_UNTIL_ONLINE_TIMESTAMP: i64 = 0x7FFFFFFF;

/// Modal type for schedule configuration dialog
/// Upstream: `history_view_schedule_box.h`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleBoxType {
    Disabled,
    SilentOnly,
    ScheduledToUser,
}

/// Recurring schedule interval (Telegram Premium feature)
/// Upstream: `history_view_schedule_box.cpp:L382-L392`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleRepeatPeriod {
    None,
    Daily,
    Weekly,
    Monthly,
}

/// Single scheduled message entity
#[derive(Debug, Clone, PartialEq)]
pub struct ScheduledMessageItem {
    pub id: i64,
    pub text: String,
    pub scheduled_at: i64,
    pub is_until_online: bool,
    pub is_silent: bool,
    pub repeat: ScheduleRepeatPeriod,
    pub has_video: bool,
}

/// Scheduled Section Memento for section state preservation
/// Upstream: `HistoryView::ScheduledMemento`
#[derive(Debug, Clone, PartialEq)]
pub struct ScheduledMemento {
    pub peer_id: i64,
    pub topic_id: Option<i64>,
    pub sent_to_scheduled_id: Option<i64>,
}

impl ScheduledMemento {
    pub fn new(peer_id: i64, topic_id: Option<i64>, sent_to_scheduled_id: Option<i64>) -> Self {
        Self {
            peer_id,
            topic_id,
            sent_to_scheduled_id,
        }
    }
}

/// Schedule configuration box modal state
/// Upstream: `HistoryView::ScheduleBox`
#[derive(Debug, Clone)]
pub struct HistoryViewScheduleBoxState {
    pub box_type: ScheduleBoxType,
    pub scheduled_timestamp: i64,
    pub is_until_online: bool,
    pub is_silent: bool,
    pub repeat: ScheduleRepeatPeriod,
    pub is_premium: bool,
    pub show_tooltip: bool,
}

impl HistoryViewScheduleBoxState {
    pub fn new(box_type: ScheduleBoxType, is_premium: bool) -> Self {
        Self {
            box_type,
            scheduled_timestamp: 0,
            is_until_online: false,
            is_silent: false,
            repeat: ScheduleRepeatPeriod::None,
            is_premium,
            show_tooltip: false,
        }
    }

    pub fn set_until_online(&mut self) -> bool {
        if self.box_type == ScheduleBoxType::ScheduledToUser {
            self.is_until_online = true;
            self.scheduled_timestamp = API_SCHEDULED_UNTIL_ONLINE_TIMESTAMP;
            true
        } else {
            false
        }
    }

    pub fn set_repeat_period(&mut self, period: ScheduleRepeatPeriod) -> bool {
        if self.is_premium || period == ScheduleRepeatPeriod::None {
            self.repeat = period;
            true
        } else {
            false
        }
    }

    pub fn toggle_silent(&mut self) -> bool {
        self.is_silent = !self.is_silent;
        self.is_silent
    }
}

/// Primary Scheduled Section view state
/// Upstream: `HistoryView::ScheduledWidget`
#[derive(Debug, Clone)]
pub struct HistoryViewScheduledSectionState {
    pub memento: ScheduledMemento,
    pub items: Vec<ScheduledMessageItem>,
    pub is_loading: bool,
    pub video_processing_toast_visible: bool,
}

impl HistoryViewScheduledSectionState {
    pub fn new(peer_id: i64, topic_id: Option<i64>) -> Self {
        Self {
            memento: ScheduledMemento::new(peer_id, topic_id, None),
            items: Vec::new(),
            is_loading: false,
            video_processing_toast_visible: false,
        }
    }

    pub fn send_now(&mut self, id: i64) -> Option<ScheduledMessageItem> {
        if let Some(idx) = self.items.iter().position(|it| it.id == id) {
            Some(self.items.remove(idx))
        } else {
            None
        }
    }

    pub fn reschedule(&mut self, id: i64, new_timestamp: i64) -> bool {
        if let Some(it) = self.items.iter_mut().find(|it| it.id == id) {
            it.scheduled_at = new_timestamp;
            it.is_until_online = new_timestamp == API_SCHEDULED_UNTIL_ONLINE_TIMESTAMP;
            true
        } else {
            false
        }
    }
}
