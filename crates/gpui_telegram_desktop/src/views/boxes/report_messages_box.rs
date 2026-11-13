//! Report Messages and Channel Content Modal Box
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/report_messages_box.h:L1-L43`
//! - `Telegram/SourceFiles/boxes/report_messages_box.cpp:L1-L246`
//! - `Telegram/SourceFiles/boxes/report_messages_box.cpp:L25-L110` (Reason definitions)
//!
//! Subsystem: boxes, Feature: TCG-N03

use gpui::{px, Pixels};

/// Predefined Telegram content report reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportReason {
    Spam,
    Violence,
    ChildAbuse,
    Pornography,
    Copyright,
    IllegalDrugs,
    PersonalDetails,
    Other,
}

impl ReportReason {
    pub fn title(&self) -> &'static str {
        match self {
            Self::Spam => "Spam",
            Self::Violence => "Violence",
            Self::ChildAbuse => "Child Abuse",
            Self::Pornography => "Pornography",
            Self::Copyright => "Copyright",
            Self::IllegalDrugs => "Illegal Drugs",
            Self::PersonalDetails => "Personal Details",
            Self::Other => "Other",
        }
    }
}

/// Modal dialog box for reporting abusive messages or channels to Telegram moderation.
/// Target symbol in ledger: `ReportMessagesBox`.
#[derive(Debug, Clone)]
pub struct ReportMessagesBox {
    pub is_visible: bool,
    pub width: Pixels,
    pub height: Pixels,
    pub peer_id: u64,
    pub message_ids: Vec<i32>,
    pub selected_reason: Option<ReportReason>,
    pub comment: String,
    pub is_submitting: bool,
}

impl Default for ReportMessagesBox {
    fn default() -> Self {
        Self::new(0, Vec::new())
    }
}

impl ReportMessagesBox {
    pub fn new(peer_id: u64, message_ids: Vec<i32>) -> Self {
        Self {
            is_visible: true,
            width: px(380.0),
            height: px(440.0),
            peer_id,
            message_ids,
            selected_reason: None,
            comment: String::new(),
            is_submitting: false,
        }
    }

    pub fn select_reason(&mut self, reason: ReportReason) {
        self.selected_reason = Some(reason);
    }

    pub fn set_comment(&mut self, text: &str) {
        self.comment = text.trim().to_string();
    }

    pub fn can_submit(&self) -> bool {
        if self.is_submitting {
            return false;
        }
        match self.selected_reason {
            Some(ReportReason::Other) => !self.comment.is_empty(),
            Some(_) => true,
            None => false,
        }
    }
}
