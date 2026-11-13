//! Rich draft article preview for instant view / markdown articles inside the composer.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_rich_draft_preview.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_rich_draft_preview.h`

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RichDraftArticle {
    pub title: String,
    pub description: String,
    pub url: String,
    pub full_article_height: i32,
    pub is_clipped_bottom: bool,
}

pub struct HistoryViewRichDraftPreviewViewControls {
    pub article: Option<RichDraftArticle>,
    pub is_visible: bool,
}

impl HistoryViewRichDraftPreviewViewControls {
    pub fn new() -> Self {
        Self {
            article: None,
            is_visible: false,
        }
    }

    pub fn set_article(&mut self, title: impl Into<String>, desc: impl Into<String>, url: impl Into<String>) {
        self.article = Some(RichDraftArticle {
            title: title.into(),
            description: desc.into(),
            url: url.into(),
            full_article_height: 120,
            is_clipped_bottom: false,
        });
        self.is_visible = true;
    }

    pub fn clear(&mut self) {
        self.article = None;
        self.is_visible = false;
    }
}

impl Default for HistoryViewRichDraftPreviewViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewRichDraftPreviewViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible || self.article.is_none() {
            return div().into_any_element();
        }

        let art = self.article.as_ref().unwrap();
        let title = art.title.clone();
        let desc = art.description.clone();

        div()
            .flex()
            .flex_col()
            .gap_1()
            .p_2()
            .bg(rgb(0x18222d))
            .border_l_2()
            .border_color(rgb(0x2481cc))
            .rounded_r_md()
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x2481cc))
                    .child(title),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0xffffff))
                    .child(desc),
            )
            .into_any_element()
    }
}
