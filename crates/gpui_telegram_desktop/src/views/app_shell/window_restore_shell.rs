//! WindowRestoreShell: Window recovery shell displayed during session restore or restart
//! Upstream: Telegram/SourceFiles/window/window_restore_shell.h, window_restore_shell.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestoreState {
    Ready,
    Restoring { progress: u8 },
    Unavailable,
    Completed,
}

#[derive(Debug, Clone)]
pub struct WindowRestoreShell {
    pub state: RestoreState,
    pub attempts: u32,
    pub error_message: Option<String>,
}

impl WindowRestoreShell {
    pub fn new() -> Self {
        Self {
            state: RestoreState::Ready,
            attempts: 0,
            error_message: None,
        }
    }

    pub fn start_restore(&mut self) {
        self.state = RestoreState::Restoring { progress: 0 };
        self.attempts += 1;
    }

    pub fn set_progress(&mut self, progress: u8) {
        if matches!(self.state, RestoreState::Restoring { .. }) {
            self.state = RestoreState::Restoring {
                progress: progress.min(100),
            };
        }
    }

    pub fn show_unavailable(&mut self, reason: impl Into<String>) {
        self.state = RestoreState::Unavailable;
        self.error_message = Some(reason.into());
    }

    pub fn complete(&mut self) {
        self.state = RestoreState::Completed;
        self.error_message = None;
    }

    pub fn is_active(&self) -> bool {
        matches!(self.state, RestoreState::Restoring { .. })
    }
}

impl Default for WindowRestoreShell {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowRestoreShell {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0x0E1621))
            .text_color(rgb(0xFFFFFF))
            .child(format!("Restore Shell: {:?}", self.state))
    }
}
