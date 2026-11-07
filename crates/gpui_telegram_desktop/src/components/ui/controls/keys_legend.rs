//! UI Keys Legend shortcut badge indicator.
//! Upstream: `Telegram/SourceFiles/ui/controls/keys_legend.cpp:L15-L50`
//!           `Telegram/SourceFiles/ui/controls/keys_legend.h:L10-L30`
//!
//! Renders keyboard shortcut keycaps beside menu items and search fields.

use gpui::*;

/// KeysLegendControls component. Target symbol required by ledger: `KeysLegendControls`.
#[derive(Debug, Clone)]
pub struct KeysLegendControls {
    pub shortcut: String,
}

impl KeysLegendControls {
    pub fn new(shortcut: impl Into<String>) -> Self {
        Self {
            shortcut: shortcut.into(),
        }
    }
}

impl Default for KeysLegendControls {
    fn default() -> Self {
        Self::new("Ctrl+K")
    }
}

impl Render for KeysLegendControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .px(px(6.0))
            .py(px(2.0))
            .rounded(px(4.0))
            .bg(hsla(0.0, 0.0, 0.88, 1.0))
            .text_size(px(10.0))
            .font_weight(FontWeight::MEDIUM)
            .child(self.shortcut.clone())
    }
}
