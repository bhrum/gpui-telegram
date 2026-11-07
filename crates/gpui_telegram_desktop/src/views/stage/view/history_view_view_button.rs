//! History View "View in Chat" Button Overlay
//! Faithful upstream C++ extraction:
//! - `history_view_view_button.cpp:L30-L100`
//! - `history_view_view_button.h:L20-L50`

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewButtonKind {
    #[default]
    Giveaway,
    RichMessage,
}

#[derive(Debug, Clone)]
pub struct HistoryViewViewButtonView {
    pub message_id: u64,
    pub kind: ViewButtonKind,
    pub text: String,
    pub is_visible: bool,
}

impl HistoryViewViewButtonView {
    pub fn new(message_id: u64, kind: ViewButtonKind) -> Self {
        let text = match kind {
            ViewButtonKind::Giveaway => "View Giveaway in Chat".to_string(),
            ViewButtonKind::RichMessage => "View in Chat".to_string(),
        };
        Self {
            message_id,
            kind,
            text,
            is_visible: true,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl Default for HistoryViewViewButtonView {
    fn default() -> Self {
        Self::new(0, ViewButtonKind::RichMessage)
    }
}

impl Render for HistoryViewViewButtonView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let label = &self.text;

        div()
            .flex()
            .items_center()
            .justify_center()
            .h(px(28.0))
            .px(px(12.0))
            .bg(rgba(0x0000000D))
            .hover(|s| s.bg(rgba(0x00000018)))
            .rounded(px(14.0))
            .text_size(px(12.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgb(0x2AABEE))
            .child(label.clone())
            .into_any_element()
    }
}
