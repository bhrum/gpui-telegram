//! UI Button Labels multi-line text alignment helper.
//! Upstream: `Telegram/SourceFiles/ui/controls/button_labels.cpp:L15-L50`
//!           `Telegram/SourceFiles/ui/controls/button_labels.h:L10-L35`
//!
//! Aligns primary title and secondary caption labels within button controls.

use gpui::*;

/// ButtonLabelsControls component. Target symbol required by ledger: `ButtonLabelsControls`.
#[derive(Debug, Clone)]
pub struct ButtonLabelsControls {
    pub title: String,
    pub subtitle: Option<String>,
}

impl ButtonLabelsControls {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
        }
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }
}

impl Default for ButtonLabelsControls {
    fn default() -> Self {
        Self::new("Button")
    }
}

impl Render for ButtonLabelsControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut d = div()
            .flex()
            .flex_col()
            .items_center()
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child(self.title.clone())
            );

        if let Some(sub) = &self.subtitle {
            d = d.child(
                div()
                    .text_size(px(11.0))
                    .text_color(hsla(0.0, 0.0, 0.4, 1.0))
                    .child(sub.clone())
            );
        }
        d
    }
}
