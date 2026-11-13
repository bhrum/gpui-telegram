//! Info Media List Section component
//! Upstream: `Telegram/SourceFiles/info/media/info_media_list_section.cpp:L1-L517`
//! Upstream: `Telegram/SourceFiles/info/media/info_media_list_section.h:L1-L104`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use serde::{Deserialize, Serialize};

/// Represents a grouped time section of shared media (e.g. Month & Year header)
/// with item layout boundaries and sticky header positioning.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoMediaListSectionMedia {
    pub year: i32,
    pub month: u32,
    pub title: String,
    pub items_count: usize,
    pub top_position: f32,
    pub height: f32,
    pub is_sticky: bool,
}

impl InfoMediaListSectionMedia {
    pub const FLOATING_HEADER_ALPHA: f32 = 0.95;

    pub fn new(year: i32, month: u32, items_count: usize) -> Self {
        let month_name = match month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        };
        let title = format!("{} {}", month_name, year);

        Self {
            year,
            month,
            title,
            items_count,
            top_position: 0.0,
            height: 0.0,
            is_sticky: false,
        }
    }

    pub fn update_geometry(&mut self, top: f32, height: f32) {
        self.top_position = top;
        self.height = height;
    }

    pub fn check_sticky(&mut self, scroll_top: f32) -> bool {
        self.is_sticky = scroll_top >= self.top_position && scroll_top < (self.top_position + self.height);
        self.is_sticky
    }
}

impl Default for InfoMediaListSectionMedia {
    fn default() -> Self {
        Self::new(2024, 1, 0)
    }
}
