//! UI Feature List bullet view.
//! Upstream: `Telegram/SourceFiles/ui/controls/feature_list.cpp:L25-L75`
//!           `Telegram/SourceFiles/ui/controls/feature_list.h:L10-L45`
//!
//! Renders stylized bullet checklists for Telegram Premium and channel boost feature highlights.

use gpui::*;

#[derive(Debug, Clone)]
pub struct FeatureListItem {
    pub title: String,
    pub description: String,
}

/// FeatureListControls component. Target symbol required by ledger: `FeatureListControls`.
#[derive(Debug, Clone)]
pub struct FeatureListControls {
    pub items: Vec<FeatureListItem>,
}

impl FeatureListControls {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn with_item(mut self, title: impl Into<String>, desc: impl Into<String>) -> Self {
        self.items.push(FeatureListItem {
            title: title.into(),
            description: desc.into(),
        });
        self
    }
}

impl Default for FeatureListControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for FeatureListControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .children(self.items.iter().map(|item| {
                div()
                    .flex()
                    .gap(px(8.0))
                    .items_start()
                    .child("✓")
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().font_weight(FontWeight::BOLD).child(item.title.clone()))
                            .child(div().text_size(px(12.0)).child(item.description.clone()))
                    )
            }))
    }
}
