//! Authentic GPUI Scheduled Date-Time Picker Component.
//! Reference: Telegram/SourceFiles/ui/boxes/choose_date_time.cpp:L30-L424
//! Reference: Telegram/SourceFiles/ui/boxes/choose_date_time.h:L25-L68

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Minimum forward schedule threshold: 10 seconds.
/// Reference: `choose_date_time.cpp:L38`
pub const K_MINIMAL_SCHEDULE: i64 = 10;

/// Standard widget height: 95px.
/// Reference: `choose_date_time.style:L26`
pub const SCHEDULE_HEIGHT: f32 = 95.0;

/// Date input field width: 136px.
/// Reference: `choose_date_time.style:L35`
pub const SCHEDULE_DATE_WIDTH: f32 = 136.0;

/// Time input field width: 72px.
/// Reference: `choose_date_time.style:L36`
pub const SCHEDULE_TIME_WIDTH: f32 = 72.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleRepeat {
    None,
    Daily,
    Weekly,
    Monthly,
}

/// Target symbol for scheduled message date-time selection dialog.
/// Reference: `choose_date_time.h:L35-L65`
#[derive(Debug, Clone)]
pub struct ChooseDateTimeBoxes {
    pub scheduled_timestamp: i64,
    pub repeat_option: ScheduleRepeat,
    pub submit_enabled: bool,
    pub hours: u32,
    pub minutes: u32,
    pub title: String,
}

impl Default for ChooseDateTimeBoxes {
    fn default() -> Self {
        Self::new(1794025000)
    }
}

impl ChooseDateTimeBoxes {
    pub fn new(current_timestamp: i64) -> Self {
        let default_schedule = current_timestamp + 600; // 10 minutes ahead
        Self {
            scheduled_timestamp: default_schedule,
            repeat_option: ScheduleRepeat::None,
            submit_enabled: true,
            hours: 12,
            minutes: 0,
            title: "Schedule Message".to_string(),
        }
    }

    /// Validates whether chosen schedule is at least 10 seconds ahead.
    /// Reference: `choose_date_time.cpp:L140-L150`
    pub fn is_valid_schedule(&self, now: i64) -> bool {
        self.scheduled_timestamp >= now + K_MINIMAL_SCHEDULE
    }

    pub fn set_time(&mut self, hours: u32, minutes: u32, now: i64) {
        self.hours = hours.clamp(0, 23);
        self.minutes = minutes.clamp(0, 59);
        self.submit_enabled = self.is_valid_schedule(now);
    }

    pub fn set_repeat(&mut self, repeat: ScheduleRepeat) {
        self.repeat_option = repeat;
    }
}
