//! Authentic GPUI Abuse & Content Report Flow Modal Component.
//! Reference: Telegram/SourceFiles/ui/boxes/report_box_graphics.cpp:L30-L224
//! Reference: Telegram/SourceFiles/ui/boxes/report_box_graphics.h:L20-L64

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Maximum length for detailed abuse explanation: 512.
/// Reference: `report_box_graphics.cpp:L32`
pub const K_REPORT_REASON_LENGTH_MAX: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportSource {
    Message,
    Channel,
    Group,
    Bot,
    ProfilePhoto,
    ProfileVideo,
    GroupPhoto,
    GroupVideo,
    ChannelPhoto,
    ChannelVideo,
    Story,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportReason {
    Spam,
    Fake,
    Violence,
    ChildAbuse,
    Pornography,
    Copyright,
    IllegalDrugs,
    PersonalDetails,
    Other,
}

/// Target symbol for abuse reporting dialogs.
/// Reference: `report_box_graphics.h:L35-L64`
#[derive(Debug, Clone)]
pub struct ReportBoxGraphicsBoxes {
    pub source: ReportSource,
    pub selected_reason: Option<ReportReason>,
    pub details_comment: String,
    pub is_submitted: bool,
}

impl Default for ReportBoxGraphicsBoxes {
    fn default() -> Self {
        Self::new(ReportSource::Message)
    }
}

impl ReportBoxGraphicsBoxes {
    pub fn new(source: ReportSource) -> Self {
        Self {
            source,
            selected_reason: None,
            details_comment: String::new(),
            is_submitted: false,
        }
    }

    /// Selects reason category.
    /// Reference: `report_box_graphics.cpp:L60-L75`
    pub fn select_reason(&mut self, reason: ReportReason) {
        self.selected_reason = Some(reason);
    }

    pub fn set_details(&mut self, comment: &str) {
        self.details_comment = comment.chars().take(K_REPORT_REASON_LENGTH_MAX).collect();
    }

    pub fn submit(&mut self) -> bool {
        if self.selected_reason.is_some() {
            self.is_submitted = true;
            true
        } else {
            false
        }
    }
}
