//! Authentic GPUI Message Lifetime TTL Picker Component.
//! Reference: Telegram/SourceFiles/ui/boxes/time_picker_box.cpp:L25-L132
//! Reference: Telegram/SourceFiles/ui/boxes/time_picker_box.h:L15-L22

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Standard 16 discrete TTL intervals from 15 minutes to 3 months.
/// Reference: `time_picker_box.cpp:L27-L46`
pub fn default_time_picker_values() -> Vec<i64> {
    vec![
        60 * 15,          // 15m
        60 * 30,          // 30m
        3600 * 1,         // 1h
        3600 * 2,         // 2h
        3600 * 3,         // 3h
        3600 * 4,         // 4h
        3600 * 8,         // 8h
        3600 * 12,        // 12h
        86400 * 1,        // 1d
        86400 * 2,        // 2d
        86400 * 3,        // 3d
        86400 * 7 * 1,    // 1w
        86400 * 7 * 2,    // 2w
        86400 * 31 * 1,   // 1mo
        86400 * 31 * 2,   // 2mo
        86400 * 31 * 3,   // 3mo
    ]
}

/// Target symbol for message TTL cylindrical drum picker.
/// Reference: `time_picker_box.h:L18-L22`
#[derive(Debug, Clone)]
pub struct TimePickerBoxBoxes {
    pub values: Vec<i64>,
    pub selected_index: usize,
    pub is_confirmed: bool,
}

impl Default for TimePickerBoxBoxes {
    fn default() -> Self {
        Self::new(86400) // 1 day default
    }
}

impl TimePickerBoxBoxes {
    pub fn new(start_value: i64) -> Self {
        let values = default_time_picker_values();
        let idx = Self::find_nearest_index(&values, start_value);
        Self {
            values,
            selected_index: idx,
            is_confirmed: false,
        }
    }

    /// Finds nearest interval index with lower-bound distance tie-breaking.
    /// Reference: `time_picker_box.cpp:L50-L60`
    pub fn find_nearest_index(values: &[i64], target: i64) -> usize {
        if values.is_empty() {
            return 0;
        }
        match values.binary_search(&target) {
            Ok(idx) => idx,
            Err(0) => 0,
            Err(idx) if idx >= values.len() => values.len() - 1,
            Err(idx) => {
                let left = values[idx - 1];
                let right = values[idx];
                if (target - left).abs() < (target - right).abs() {
                    idx - 1
                } else {
                    idx
                }
            }
        }
    }

    pub fn selected_value(&self) -> i64 {
        self.values.get(self.selected_index).copied().unwrap_or(0)
    }

    pub fn set_selected_index(&mut self, idx: usize) {
        if idx < self.values.len() {
            self.selected_index = idx;
        }
    }

    pub fn confirm(&mut self) -> i64 {
        self.is_confirmed = true;
        self.selected_value()
    }
}
