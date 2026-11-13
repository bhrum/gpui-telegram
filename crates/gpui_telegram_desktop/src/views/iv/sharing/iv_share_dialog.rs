//! Share article modal dialog for Instant View
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, Default)]
pub struct IvShareDialog {
    pub is_open: bool,
    pub article_url: String,
    pub article_title: String,
    pub link_copied: bool,
}

impl IvShareDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open_share(&mut self, url: impl Into<String>, title: impl Into<String>) {
        self.article_url = url.into();
        self.article_title = title.into();
        self.is_open = true;
        self.link_copied = false;
    }

    pub fn copy_link(&mut self) {
        self.link_copied = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }
}

impl Render for IvShareDialog {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_open {
            return div();
        }

        let title = self.article_title.clone();
        let url = self.article_url.clone();
        let copy_status = if self.link_copied { "Copied!" } else { "Copy Link" };

        div()
            .flex()
            .flex_col()
            .p_4()
            .bg(rgb(0x212529))
            .text_color(rgb(0xffffff))
            .rounded_lg()
            .child(div().font_weight(FontWeight::BOLD).child(format!("Share: {}", title)))
            .child(div().my_2().text_xs().text_color(rgb(0xaaaaaa)).child(url))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .mt_3()
                    .child(div().p_2().bg(rgb(0x2b5278)).rounded_md().child(copy_status))
                    .child(div().p_2().bg(rgb(0x333333)).rounded_md().child("Close")),
            )
    }
}
