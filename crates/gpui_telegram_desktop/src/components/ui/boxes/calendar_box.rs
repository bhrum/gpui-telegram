//! Authentic GPUI Calendar Box Component.
//! Reference: Telegram/SourceFiles/ui/boxes/calendar_box.cpp:L45-L1591
//! Reference: Telegram/SourceFiles/ui/boxes/calendar_box.h:L35-L131

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Days in a Gregorian calendar week: 7.
/// Reference: `calendar_box.cpp:L35`
pub const K_DAYS_IN_WEEK: usize = 7;

/// Maximum thumbnails processed per paint pass: 4.
/// Reference: `calendar_box.cpp:L36`
pub const K_THUMBNAILS_PER_PAINT: usize = 4;

/// Cache count of adjacent months kept loaded: 2.
/// Reference: `calendar_box.cpp:L37`
pub const K_KEEP_MONTHS_AROUND: usize = 2;

/// Maximum week rows in a single monthly calendar grid: 6.
/// Reference: `calendar_box.cpp:L41`
pub const K_MAX_ROWS: usize = 6;

/// Months in a standard calendar year: 12.
/// Reference: `calendar_box.cpp:L42`
pub const K_MONTHS_COUNT: usize = 12;

/// Sentinel index for an unselected calendar date: INT_MIN / 2.
/// Reference: `calendar_box.cpp:L43`
pub const K_EMPTY_SELECTION: i32 = i32::MIN / 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Default for CalendarDate {
    fn default() -> Self {
        Self { year: 2026, month: 11, day: 7 }
    }
}

/// Target symbol for interactive monthly calendar box.
/// Reference: `calendar_box.h:L40-L120`
#[derive(Debug, Clone)]
pub struct CalendarBoxBoxes {
    pub current_month: CalendarDate,
    pub selected_date: Option<CalendarDate>,
    pub highlighted_date: Option<CalendarDate>,
    pub min_date: Option<CalendarDate>,
    pub max_date: Option<CalendarDate>,
    pub is_range_selection: bool,
    pub range_start: Option<CalendarDate>,
    pub range_end: Option<CalendarDate>,
}

impl Default for CalendarBoxBoxes {
    fn default() -> Self {
        Self::new()
    }
}

impl CalendarBoxBoxes {
    pub fn new() -> Self {
        Self {
            current_month: CalendarDate { year: 2026, month: 11, day: 1 },
            selected_date: Some(CalendarDate { year: 2026, month: 11, day: 7 }),
            highlighted_date: None,
            min_date: None,
            max_date: None,
            is_range_selection: false,
            range_start: None,
            range_end: None,
        }
    }

    /// Computes number of days in the specified month accounting for leap years.
    /// Reference: `calendar_box.cpp:L120-L135`
    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
                if is_leap { 29 } else { 28 }
            }
            _ => 30,
        }
    }

    pub fn next_month(&mut self) {
        if self.current_month.month == 12 {
            self.current_month.month = 1;
            self.current_month.year += 1;
        } else {
            self.current_month.month += 1;
        }
    }

    pub fn prev_month(&mut self) {
        if self.current_month.month == 1 {
            self.current_month.month = 12;
            self.current_month.year -= 1;
        } else {
            self.current_month.month -= 1;
        }
    }

    pub fn select_day(&mut self, day: u32) {
        let max_days = Self::days_in_month(self.current_month.year, self.current_month.month);
        let clamped_day = day.clamp(1, max_days);
        self.selected_date = Some(CalendarDate {
            year: self.current_month.year,
            month: self.current_month.month,
            day: clamped_day,
        });
    }
}
