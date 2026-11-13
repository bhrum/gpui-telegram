//! Bottom controls bar when composer input field is disabled or replaced with action buttons.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_bottom_controls.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_bottom_controls.h`

use gpui::*;
use serde::{Deserialize, Serialize};

/// Operating mode of bottom controls
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BottomControlsMode {
    History,
    Replies,
    Sublist,
}

impl Default for BottomControlsMode {
    fn default() -> Self {
        Self::History
    }
}

/// Action triggered by clicking bottom buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BottomControlsAction {
    Unblock,
    BotStart,
    JoinChannel,
    JoinGroup,
    MuteUnmute,
    Report,
}

/// Authentic Bottom Controls Component
pub struct HistoryViewBottomControlsViewControls {
    pub mode: BottomControlsMode,
    pub can_send_messages: bool,
    pub is_blocked: bool,
    pub is_bot_start: bool,
    pub is_join_channel: bool,
    pub is_muted: bool,
    pub in_report_mode: bool,
    pub report_count: i32,
    pub content_height: i32,
}

impl HistoryViewBottomControlsViewControls {
    pub fn new() -> Self {
        Self {
            mode: BottomControlsMode::History,
            can_send_messages: true,
            is_blocked: false,
            is_bot_start: false,
            is_join_channel: false,
            is_muted: false,
            in_report_mode: false,
            report_count: 0,
            content_height: 48,
        }
    }

    /// Check if an action button is currently active
    pub fn is_button_active(&self) -> bool {
        self.is_blocked
            || self.is_bot_start
            || self.is_join_channel
            || self.in_report_mode
    }

    pub fn set_report_mode(&mut self, report_mode: bool, selected_count: i32) {
        self.in_report_mode = report_mode;
        self.report_count = selected_count;
    }

    pub fn active_action(&self) -> Option<BottomControlsAction> {
        if self.in_report_mode {
            Some(BottomControlsAction::Report)
        } else if self.is_blocked {
            Some(BottomControlsAction::Unblock)
        } else if self.is_bot_start {
            Some(BottomControlsAction::BotStart)
        } else if self.is_join_channel {
            Some(BottomControlsAction::JoinChannel)
        } else if !self.can_send_messages {
            Some(BottomControlsAction::MuteUnmute)
        } else {
            None
        }
    }
}

impl Default for HistoryViewBottomControlsViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewBottomControlsViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let action = self.active_action();
        let label = match action {
            Some(BottomControlsAction::Report) => {
                format!("Report {} Messages", self.report_count)
            }
            Some(BottomControlsAction::Unblock) => "Unblock User".to_string(),
            Some(BottomControlsAction::BotStart) => "START".to_string(),
            Some(BottomControlsAction::JoinChannel) => "JOIN CHANNEL".to_string(),
            Some(BottomControlsAction::JoinGroup) => "JOIN GROUP".to_string(),
            Some(BottomControlsAction::MuteUnmute) => {
                if self.is_muted {
                    "UNMUTE".to_string()
                } else {
                    "MUTE".to_string()
                }
            }
            None => "Chat Input Ready".to_string(),
        };

        div()
            .flex()
            .items_center()
            .justify_center()
            .h_12()
            .w_full()
            .bg(rgb(0x18222d))
            .border_t_1()
            .border_color(rgb(0x2a394a))
            .child(
                div()
                    .px_6()
                    .py_2()
                    .rounded_md()
                    .bg(rgb(0x2481cc))
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .hover(|s| s.bg(rgb(0x1e72b4)))
                    .child(label),
            )
    }
}
