//! Info Profile Status Label component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_status_label.cpp:L1-L157`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_status_label.h:L1-L36`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserOnlineStatus {
    Online,
    Offline(i64), // timestamp
    Recently,
    WithinWeek,
    WithinMonth,
    LongTimeAgo,
}

/// Dynamic status label rendering online status, last seen text, or member counts.
#[derive(Debug, Clone)]
pub struct InfoProfileStatusLabelProfile {
    pub status_text: String,
    pub is_online: bool,
}

impl InfoProfileStatusLabelProfile {
    pub fn new(text: impl Into<String>, is_online: bool) -> Self {
        Self {
            status_text: text.into(),
            is_online,
        }
    }

    pub fn from_status(status: UserOnlineStatus) -> Self {
        let (text, online) = match status {
            UserOnlineStatus::Online => ("online", true),
            UserOnlineStatus::Offline(_) => ("last seen recently", false),
            UserOnlineStatus::Recently => ("last seen recently", false),
            UserOnlineStatus::WithinWeek => ("last seen within a week", false),
            UserOnlineStatus::WithinMonth => ("last seen within a month", false),
            UserOnlineStatus::LongTimeAgo => ("last seen a long time ago", false),
        };
        Self::new(text, online)
    }
}

impl Default for InfoProfileStatusLabelProfile {
    fn default() -> Self {
        Self::new("offline", false)
    }
}

impl Render for InfoProfileStatusLabelProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let color = if self.is_online { rgb(0x2ea6ff) } else { rgb(0x708499) };
        div()
            .text_xs()
            .text_color(color)
            .child(self.status_text.clone())
    }
}
