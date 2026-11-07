//! UI Sub Tabs secondary tab strip.
//! Upstream: `Telegram/SourceFiles/ui/controls/sub_tabs.cpp:L25-L80`
//!           `Telegram/SourceFiles/ui/controls/sub_tabs.h:L15-L45`
//!
//! Compact segmented tab control for filtering sub-categories.

use gpui::*;

/// SubTabsControls component. Target symbol required by ledger: `SubTabsControls`.
#[derive(Debug, Clone)]
pub struct SubTabsControls {
    pub tabs: Vec<String>,
    pub active_index: usize,
}

impl SubTabsControls {
    pub fn new(tabs: Vec<String>) -> Self {
        Self {
            tabs,
            active_index: 0,
        }
    }
}

impl Default for SubTabsControls {
    fn default() -> Self {
        Self::new(vec!["Tab 1".into(), "Tab 2".into()])
    }
}

impl Render for SubTabsControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap(px(6.0))
            .p(px(2.0))
            .children(self.tabs.iter().enumerate().map(|(idx, name)| {
                let is_active = idx == self.active_index;
                div()
                    .px(px(8.0))
                    .py(px(4.0))
                    .rounded(px(12.0))
                    .bg(if is_active { hsla(204.0 / 360.0, 0.88, 0.53, 1.0) } else { hsla(0.0, 0.0, 0.92, 1.0) })
                    .text_color(if is_active { hsla(0.0, 0.0, 1.0, 1.0) } else { hsla(0.0, 0.0, 0.3, 1.0) })
                    .child(name.clone())
            }))
    }
}
