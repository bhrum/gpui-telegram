//! Saved Messages Self-Forwards Tag Manager View
//! Faithful upstream C++ extraction:
//! - `history_view_self_forwards_tagger.cpp:L20-L80`
//! - `history_view_self_forwards_tagger.h:L20-L45`

use gpui::*;

#[derive(Debug, Clone)]
pub struct HistoryViewSelfForwardsTaggerView {
    pub message_id: u64,
    pub active_tags: Vec<String>,
    pub is_tag_menu_open: bool,
}

impl HistoryViewSelfForwardsTaggerView {
    pub fn new(message_id: u64) -> Self {
        Self {
            message_id,
            active_tags: Vec::new(),
            is_tag_menu_open: false,
        }
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.active_tags.contains(&tag) {
            self.active_tags.push(tag);
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.active_tags.retain(|t| t != tag);
    }
}

impl Default for HistoryViewSelfForwardsTaggerView {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for HistoryViewSelfForwardsTaggerView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let tags = &self.active_tags;

        div()
            .flex()
            .items_center()
            .gap(px(4.0))
            .children(tags.iter().map(|tag| {
                div()
                    .px(px(6.0))
                    .py(px(2.0))
                    .rounded(px(10.0))
                    .bg(rgba(0x2AABEE20))
                    .text_size(px(11.0))
                    .text_color(rgb(0x2AABEE))
                    .child(format!("#{}", tag))
            }))
            .child(
                div()
                    .px(px(6.0))
                    .py(px(2.0))
                    .rounded(px(10.0))
                    .bg(rgba(0x00000008))
                    .hover(|s| s.bg(rgba(0x00000015)))
                    .text_size(px(11.0))
                    .text_color(rgb(0x666666))
                    .child("+ tag")
            )
    }
}
