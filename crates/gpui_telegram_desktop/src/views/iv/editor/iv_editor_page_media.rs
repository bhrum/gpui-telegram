//! Instant View Media Block Editor & Album Aggregator (`iv_editor_page_media.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

/// Specific category of media attached to an article block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MediaKind {
    #[default]
    Photo,
    Video,
    Animation,
    Audio,
}

/// Description of an individual media asset in an article or album.
#[derive(Debug, Clone, Default)]
pub struct GroupedMediaItem {
    pub media_id: u64,
    pub kind: MediaKind,
    pub caption: String,
    pub has_spoiler: bool,
    pub width: u32,
    pub height: u32,
}

pub fn media_block_supports_spoiler(kind: MediaKind) -> bool {
    matches!(kind, MediaKind::Photo | MediaKind::Video | MediaKind::Animation)
}

pub fn is_groupable_media(kind: MediaKind) -> bool {
    matches!(kind, MediaKind::Photo | MediaKind::Video)
}

/// Groups consecutive photo and video blocks into multi-item albums (max 10 items).
pub fn group_media_items(items: &[GroupedMediaItem]) -> Vec<Vec<GroupedMediaItem>> {
    let mut albums = Vec::new();
    let mut current_album = Vec::new();

    for item in items {
        if is_groupable_media(item.kind) {
            current_album.push(item.clone());
            if current_album.len() == 10 {
                albums.push(current_album);
                current_album = Vec::new();
            }
        } else {
            if !current_album.is_empty() {
                albums.push(current_album);
                current_album = Vec::new();
            }
            albums.push(vec![item.clone()]);
        }
    }

    if !current_album.is_empty() {
        albums.push(current_album);
    }
    albums
}

/// Media block editor managing spoilers, dimensions, captions, and album creation.
#[derive(Debug, Clone)]
pub struct IvEditorPageMediaEditor {
    pub items: Vec<GroupedMediaItem>,
    pub selected_index: usize,
    pub is_active: bool,
}

impl Default for IvEditorPageMediaEditor {
    fn default() -> Self {
        Self {
            items: vec![GroupedMediaItem {
                media_id: 1001,
                kind: MediaKind::Photo,
                caption: "Sample photograph".to_string(),
                has_spoiler: false,
                width: 1280,
                height: 720,
            }],
            selected_index: 0,
            is_active: false,
        }
    }
}

impl IvEditorPageMediaEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_media(&mut self, kind: MediaKind, width: u32, height: u32, caption: impl Into<String>) {
        let media_id = 1000 + (self.items.len() as u64) + 1;
        self.items.push(GroupedMediaItem {
            media_id,
            kind,
            caption: caption.into(),
            has_spoiler: false,
            width,
            height,
        });
        self.selected_index = self.items.len() - 1;
    }

    pub fn toggle_spoiler(&mut self, index: usize) -> bool {
        if let Some(item) = self.items.get_mut(index) {
            if media_block_supports_spoiler(item.kind) {
                item.has_spoiler = !item.has_spoiler;
                return true;
            }
        }
        false
    }

    pub fn set_caption(&mut self, index: usize, caption: impl Into<String>) {
        if let Some(item) = self.items.get_mut(index) {
            item.caption = caption.into();
        }
    }
}

impl Render for IvEditorPageMediaEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let albums = group_media_items(&self.items);
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(12.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child("Instant View Media Editor")
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .child(format!(
                        "Total media assets: {} | Grouped albums: {}",
                        self.items.len(), albums.len()
                    ))
            )
            .child(
                div()
                    .mt(px(8.0))
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .children(self.items.iter().enumerate().map(|(idx, item)| {
                        div()
                            .text_size(px(12.0))
                            .child(format!(
                                "[#{}] {:?} ({}x{}) — Spoiler: {} — \"{}\"",
                                idx + 1, item.kind, item.width, item.height, item.has_spoiler, item.caption
                            ))
                    }))
            )
    }
}
