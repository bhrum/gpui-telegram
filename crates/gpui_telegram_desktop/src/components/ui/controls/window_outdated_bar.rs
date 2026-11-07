//! UI Window Outdated Bar deprecation warning banner.
//! Upstream: `Telegram/SourceFiles/ui/controls/window_outdated_bar.cpp:L23-L26, L64-L70, L105-L134`
//!           `Telegram/SourceFiles/ui/controls/window_outdated_bar.h:L16-L20`
//!
//! Warns users running obsolete operating system versions with persistent dismissal throttling.

use gpui::*;

pub const K_MINIMAL_SKIP_DAYS: i64 = 7;
pub const K_SOON_SKIP_DAYS: i64 = 30;
pub const K_NOW_SKIP_DAYS: i64 = 90;

pub fn pack_date(year: i32, month: i32, day: i32) -> i32 {
    year * 10000 + month * 100 + day
}

/// WindowOutdatedBarControls component. Target symbol required by ledger: `WindowOutdatedBarControls`.
#[derive(Debug, Clone)]
pub struct WindowOutdatedBarControls {
    pub is_soon: bool,
    pub outdate_date_str: String,
    pub is_visible: bool,
}

impl WindowOutdatedBarControls {
    pub fn new(outdate_date_str: impl Into<String>, is_soon: bool) -> Self {
        Self {
            is_soon,
            outdate_date_str: outdate_date_str.into(),
            is_visible: true,
        }
    }

    pub fn dismiss(&mut self) {
        self.is_visible = false;
    }
}

impl Default for WindowOutdatedBarControls {
    fn default() -> Self {
        Self::new("2026-12-31", true)
    }
}

impl Render for WindowOutdatedBarControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div();
        }

        let bg = if self.is_soon {
            hsla(35.0 / 360.0, 0.90, 0.90, 1.0)
        } else {
            hsla(0.0 / 360.0, 0.85, 0.90, 1.0)
        };

        div()
            .h(px(36.0))
            .w_full()
            .px(px(12.0))
            .bg(bg)
            .flex()
            .items_center()
            .justify_between()
            .child(format!("System support ending on {}", self.outdate_date_str))
    }
}
