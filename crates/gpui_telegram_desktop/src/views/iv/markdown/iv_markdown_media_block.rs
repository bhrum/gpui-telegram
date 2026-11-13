//! Target Symbol: IvMarkdownMediaBlockMarkdown
//! Module 13: Instant View Structured Media Block Presentation (Channel, Document, Collage)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const CHANNEL_AVATAR_SIZE: f32 = 40.0;
pub const GROUPED_MEDIA_GAP: f32 = 4.0;

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelBlockData {
    pub channel_title: String,
    pub channel_username: String,
    pub message_text: String,
    pub members_count: usize,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DocumentBlockData {
    pub filename: String,
    pub file_size_bytes: u64,
    pub mime_type: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupedMediaItem {
    pub media_id: u64,
    pub thumbnail_url: String,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MediaBlockPresentation {
    Channel(ChannelBlockData),
    Document(DocumentBlockData),
    Collage(Vec<GroupedMediaItem>),
}

#[derive(Debug, Clone)]
pub struct IvMarkdownMediaBlockMarkdown {
    pub block_id: u64,
    pub presentation: MediaBlockPresentation,
}

impl Default for IvMarkdownMediaBlockMarkdown {
    fn default() -> Self {
        Self {
            block_id: 0,
            presentation: MediaBlockPresentation::Collage(Vec::new()),
        }
    }
}

impl IvMarkdownMediaBlockMarkdown {
    pub fn new_channel(block_id: u64, data: ChannelBlockData) -> Self {
        Self {
            block_id,
            presentation: MediaBlockPresentation::Channel(data),
        }
    }

    pub fn new_document(block_id: u64, data: DocumentBlockData) -> Self {
        Self {
            block_id,
            presentation: MediaBlockPresentation::Document(data),
        }
    }

    pub fn new_collage(block_id: u64, items: Vec<GroupedMediaItem>) -> Self {
        Self {
            block_id,
            presentation: MediaBlockPresentation::Collage(items),
        }
    }
}

impl Render for IvMarkdownMediaBlockMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match &self.presentation {
            MediaBlockPresentation::Channel(channel) => {
                let title = channel.channel_title.clone();
                let username = channel.channel_username.clone();
                let text = channel.message_text.clone();

                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .p_3()
                    .my_2()
                    .rounded_lg()
                    .bg(rgb(0x232a32))
                    .border_1()
                    .border_color(rgb(0x353f4b))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w(px(CHANNEL_AVATAR_SIZE))
                                    .h(px(CHANNEL_AVATAR_SIZE))
                                    .rounded_full()
                                    .bg(rgb(0x3e88c7)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().text_sm().font_weight(FontWeight::BOLD).child(title))
                                    .child(div().text_xs().text_color(rgb(0x7c8a99)).child(username)),
                            ),
                    )
                    .child(div().text_sm().child(text))
            }
            MediaBlockPresentation::Document(doc) => {
                let name = doc.filename.clone();
                let size_kb = (doc.file_size_bytes as f64 / 1024.0).round() as u64;

                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .p_3()
                    .my_2()
                    .rounded_lg()
                    .bg(rgb(0x232a32))
                    .child(div().text_xl().child("📎"))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().text_sm().font_weight(FontWeight::MEDIUM).child(name))
                            .child(div().text_xs().text_color(rgb(0x7c8a99)).child(format!("{} KB", size_kb))),
                    )
            }
            MediaBlockPresentation::Collage(items) => {
                let count = items.len();
                div()
                    .flex()
                    .gap(px(GROUPED_MEDIA_GAP))
                    .my_2()
                    .p_2()
                    .rounded_lg()
                    .bg(rgb(0x1e2329))
                    .child(div().text_xs().text_color(rgb(0x7c8a99)).child(format!("Media Collage: {} items", count)))
            }
        }
    }
}
