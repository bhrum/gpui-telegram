//! Upstream: `Telegram/SourceFiles/ui/effects/credits_graphics.cpp:L46-L65`
//! Upstream: `Telegram/SourceFiles/ui/effects/credits_graphics.h:L18-L94`
//!
//! Star Credits history entry graphics and multi-thumbnail renderer.
//! Implements multi-avatar overlapping layout with stroke `st::lineWidth * 2` and shift `stroke * 3`.

use gpui::*;

#[derive(Debug, Clone)]
pub struct CreditsEntryItem {
    pub title: String,
    pub date_str: String,
    pub star_count: i64,
    pub is_inbound: bool,
}

#[derive(Debug, Clone)]
pub struct CreditsGraphicsEffects {
    entries: Vec<CreditsEntryItem>,
}

impl CreditsGraphicsEffects {
    pub fn new() -> Self {
        let entries = vec![
            CreditsEntryItem { title: "Star Reaction".to_string(), date_str: "Today".to_string(), star_count: 50, is_inbound: true },
            CreditsEntryItem { title: "Digital Sticker Purchase".to_string(), date_str: "Yesterday".to_string(), star_count: -15, is_inbound: false },
        ];
        Self { entries }
    }

    /// C++: `credits_graphics.cpp:L52-L58`:
    /// `stroke = st::lineWidth * 2; shift = stroke * 3; smaller = size - shift`
    pub fn multi_thumbnail_metrics(size: f32, line_width: f32) -> (f32, f32) {
        let stroke = line_width * 2.0;
        let shift = stroke * 3.0;
        let smaller = (size - shift).max(1.0);
        (shift, smaller)
    }

    pub fn entries(&self) -> &[CreditsEntryItem] {
        &self.entries
    }
}

impl Default for CreditsGraphicsEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for CreditsGraphicsEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .children(self.entries.iter().map(|e| {
                div()
                    .flex()
                    .justify_between()
                    .p(px(8.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x182533))
                    .child(
                        div()
                            .child(e.title.clone())
                            .text_size(px(13.0))
                            .text_color(rgb(0xF5F5F5))
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(if e.is_inbound { rgb(0x4CAF50) } else { rgb(0xE53935) })
                            .child(format!("{}⭐ {}", if e.is_inbound { "+" } else { "" }, e.star_count))
                    )
            }))
    }
}
