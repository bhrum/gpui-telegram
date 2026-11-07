//! UI Window Outdated Bar non-supported platform fallback stub.
//! Upstream: `Telegram/SourceFiles/ui/controls/window_outdated_bar_dummy.cpp:L10-L18`
//!
//! Provides a null/empty bar for platforms that do not support deprecated OS notifications.

use gpui::*;

/// WindowOutdatedBarDummyControls component. Target symbol required by ledger: `WindowOutdatedBarDummyControls`.
#[derive(Debug, Clone, Default)]
pub struct WindowOutdatedBarDummyControls;

impl WindowOutdatedBarDummyControls {
    pub fn new() -> Self {
        Self
    }
}

impl Render for WindowOutdatedBarDummyControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
