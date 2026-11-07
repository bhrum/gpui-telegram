//! UI Group Call Bar top banner component.
//! Upstream: `Telegram/SourceFiles/ui/chat/group_call_bar.cpp:L67-L106`
//!           `Telegram/SourceFiles/ui/chat/group_call_bar.h:L24-L59`
//!
//! Renders voice chat / livestream banner with participant counts, live status,
//! and precise countdown timing for scheduled calls.

use gpui::*;

pub const K_DAY_SECONDS: i64 = 86400;

/// Formats scheduled call countdown per `ui/chat/group_call_bar.cpp:L73-L94`.
pub fn format_scheduled_left(seconds_left: i64) -> (String, bool) {
    let late = seconds_left < 0;
    let abs_left = seconds_left.abs();

    let text = if abs_left >= K_DAY_SECONDS {
        let days = abs_left / K_DAY_SECONDS;
        format!("{}d", days)
    } else {
        let hours = abs_left / 3600;
        let minutes = (abs_left % 3600) / 60;
        let seconds = abs_left % 60;
        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    };

    let display = if late {
        format!("\u{2212}{}", text)
    } else {
        text
    };

    (display, late)
}

/// GroupCallBarChat component. Target symbol required by ledger: `GroupCallBarChat`.
#[derive(Debug, Clone)]
pub struct GroupCallBarChat {
    pub title: String,
    pub participant_count: usize,
    pub is_live: bool,
    pub scheduled_seconds_left: Option<i64>,
}

impl GroupCallBarChat {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            participant_count: 0,
            is_live: true,
            scheduled_seconds_left: None,
        }
    }

    pub fn with_participants(mut self, count: usize) -> Self {
        self.participant_count = count;
        self
    }

    pub fn with_scheduled(mut self, seconds_left: i64) -> Self {
        self.scheduled_seconds_left = Some(seconds_left);
        self.is_live = false;
        self
    }
}

impl Default for GroupCallBarChat {
    fn default() -> Self {
        Self::new("Voice Chat")
    }
}

impl Render for GroupCallBarChat {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let status_str = if let Some(secs) = self.scheduled_seconds_left {
            let (formatted, _) = format_scheduled_left(secs);
            format!("Starts in {}", formatted)
        } else if self.is_live {
            format!("{} members", self.participant_count)
        } else {
            "Ended".to_string()
        };

        div()
            .h(px(40.0))
            .w_full()
            .px(px(12.0))
            .bg(hsla(140.0 / 360.0, 0.45, 0.95, 1.0))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .gap(px(8.0))
                    .items_center()
                    .child(self.title.clone())
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                            .child(status_str)
                    )
            )
    }
}
