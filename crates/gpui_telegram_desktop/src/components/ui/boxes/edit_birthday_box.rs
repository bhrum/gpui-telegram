//! Authentic GPUI User Birthday Editor Component.
//! Reference: Telegram/SourceFiles/ui/boxes/edit_birthday_box.cpp:L30-L221
//! Reference: Telegram/SourceFiles/ui/boxes/edit_birthday_box.h:L18-L30

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditBirthdayType {
    Edit,
    Suggest,
    ConfirmSuggestion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BirthdayData {
    pub day: u32,
    pub month: u32,
    pub year: Option<i32>,
}

/// Target symbol for birthday drum picker modal dialog.
/// Reference: `edit_birthday_box.h:L18-L30`
#[derive(Debug, Clone)]
pub struct EditBirthdayBoxBoxes {
    pub birthday: BirthdayData,
    pub box_type: EditBirthdayType,
    pub current_year: i32,
    pub is_saved: bool,
}

impl Default for EditBirthdayBoxBoxes {
    fn default() -> Self {
        Self::new(BirthdayData { day: 15, month: 6, year: Some(1995) }, EditBirthdayType::Edit)
    }
}

impl EditBirthdayBoxBoxes {
    pub fn new(initial: BirthdayData, box_type: EditBirthdayType) -> Self {
        let current_year = 2026;
        let mut b = Self {
            birthday: initial,
            box_type,
            current_year,
            is_saved: false,
        };
        b.clamp_day();
        b
    }

    /// Evaluates maximum days in month considering leap year status.
    /// Reference: `edit_birthday_box.cpp:L60-L75`
    pub fn max_days_in_month(month: u32, year: Option<i32>) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                let is_leap = year.map_or(true, |y| (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0));
                if is_leap { 29 } else { 28 }
            }
            _ => 30,
        }
    }

    pub fn clamp_day(&mut self) {
        let max = Self::max_days_in_month(self.birthday.month, self.birthday.year);
        if self.birthday.day > max {
            self.birthday.day = max;
        }
        if self.birthday.day < 1 {
            self.birthday.day = 1;
        }
    }

    pub fn set_date(&mut self, day: u32, month: u32, year: Option<i32>) {
        self.birthday.month = month.clamp(1, 12);
        self.birthday.year = year.map(|y| y.clamp(1900, self.current_year));
        self.birthday.day = day;
        self.clamp_day();
    }

    pub fn save(&mut self) -> BirthdayData {
        self.is_saved = true;
        self.birthday
    }
}
