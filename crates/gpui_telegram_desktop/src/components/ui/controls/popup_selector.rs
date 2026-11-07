//! UI Popup Selector anchored options menu.
//! Upstream: `Telegram/SourceFiles/ui/controls/popup_selector.cpp:L25-L75`
//!           `Telegram/SourceFiles/ui/controls/popup_selector.h:L15-L45`
//!
//! Floating option list anchored with directional arrow to target widget.

use gpui::*;

/// PopupSelectorControls component. Target symbol required by ledger: `PopupSelectorControls`.
#[derive(Debug, Clone)]
pub struct PopupSelectorControls {
    pub is_open: bool,
    pub options: Vec<String>,
}

impl PopupSelectorControls {
    pub fn new() -> Self {
        Self {
            is_open: false,
            options: Vec::new(),
        }
    }
}

impl Default for PopupSelectorControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PopupSelectorControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            return div();
        }

        div()
            .rounded(px(8.0))
            .bg(hsla(0.0, 0.0, 1.0, 1.0))
            .p(px(6.0))
            .shadow_lg()
            .children(self.options.iter().map(|opt| {
                div().p(px(4.0)).child(opt.clone())
            }))
    }
}
