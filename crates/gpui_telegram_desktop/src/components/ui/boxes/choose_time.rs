//! Authentic GPUI Segmented Time Duration Editor Component.
//! Reference: Telegram/SourceFiles/ui/boxes/choose_time.cpp:L20-L156
//! Reference: Telegram/SourceFiles/ui/boxes/choose_time.h:L15-L26

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Target symbol for duration time selection widget.
/// Reference: `choose_time.h:L15-L26`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChooseTimeBoxes {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub hidden_days_input: bool,
}

impl Default for ChooseTimeBoxes {
    fn default() -> Self {
        Self::new(3600, false)
    }
}

impl ChooseTimeBoxes {
    pub fn new(initial_seconds: u32, hidden_days: bool) -> Self {
        let (d, h, m, s) = Self::seconds_to_components(initial_seconds, hidden_days);
        Self {
            days: d,
            hours: h,
            minutes: m,
            seconds: s,
            hidden_days_input: hidden_days,
        }
    }

    /// Decomposes total seconds into days, hours, minutes, and seconds.
    /// Reference: `choose_time.cpp:L45-L65`
    pub fn seconds_to_components(total_secs: u32, hidden_days: bool) -> (u32, u32, u32, u32) {
        if hidden_days {
            let s = total_secs % 60;
            let m = (total_secs / 60) % 60;
            let h = total_secs / 3600;
            (0, h, m, s)
        } else {
            let s = total_secs % 60;
            let rem1 = total_secs / 60;
            let m = rem1 % 60;
            let rem2 = rem1 / 60;
            let h = rem2 % 24;
            let d = rem2 / 24;
            (d, h, m, s)
        }
    }

    /// Aggregates segmented inputs into composite total seconds.
    /// Reference: `choose_time.cpp:L70-L80`
    pub fn total_seconds(&self) -> u32 {
        self.days * 86400 + self.hours * 3600 + self.minutes * 60 + self.seconds
    }

    pub fn set_values(&mut self, days: u32, hours: u32, minutes: u32, seconds: u32) {
        self.days = if self.hidden_days_input { 0 } else { days };
        self.hours = if self.hidden_days_input { hours } else { hours.clamp(0, 23) };
        self.minutes = minutes.clamp(0, 59);
        self.seconds = seconds.clamp(0, 59);
    }
}
