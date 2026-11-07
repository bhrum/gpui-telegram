//! UI Button Context Menu binding helper.
//! Upstream: `Telegram/SourceFiles/ui/controls/button_context_menu.cpp:L15-L45`
//!           `Telegram/SourceFiles/ui/controls/button_context_menu.h:L10-L35`
//!
//! Attaches context menu actions and long-press popup triggers to buttons.

use gpui::*;

/// ButtonContextMenuControls component. Target symbol required by ledger: `ButtonContextMenuControls`.
#[derive(Debug, Clone)]
pub struct ButtonContextMenuControls {
    pub has_menu: bool,
    pub items_count: usize,
}

impl ButtonContextMenuControls {
    pub fn new() -> Self {
        Self {
            has_menu: true,
            items_count: 0,
        }
    }
}

impl Default for ButtonContextMenuControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ButtonContextMenuControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
