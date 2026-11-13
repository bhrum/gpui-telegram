//! Target Symbol: IvCachedMedia
//! Module 13: Instant View Cached Media & Content Runtime Subsystem
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;
use std::collections::HashMap;

pub const GEO_POINT_SCALE: f32 = 2.0;
pub const GEO_POINT_ZOOM_MIN: u8 = 13;
pub const DEFAULT_MAP_WIDTH: u32 = 640;
pub const DEFAULT_MAP_HEIGHT: u32 = 360;

#[derive(Debug, Clone, PartialEq)]
pub enum MediaKind {
    Photo { width: u32, height: u32 },
    Video { duration_sec: u32, is_looping: bool },
    Audio { duration_sec: u32, performer: String, title: String },
    Document { filename: String, size_bytes: u64 },
    GeoMap { lat: f64, lon: f64, zoom: u8 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum MediaLoadingState {
    Idle,
    Downloading { progress: f32 },
    Ready,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct CachedMediaItem {
    pub id: u64,
    pub kind: MediaKind,
    pub state: MediaLoadingState,
    pub url: String,
    pub cache_path: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct IvCachedMedia {
    pub items: HashMap<u64, CachedMediaItem>,
    pub selected_item_id: Option<u64>,
}

impl IvCachedMedia {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_photo(&mut self, id: u64, url: impl Into<String>, width: u32, height: u32) {
        self.items.insert(
            id,
            CachedMediaItem {
                id,
                kind: MediaKind::Photo { width, height },
                state: MediaLoadingState::Idle,
                url: url.into(),
                cache_path: None,
            },
        );
    }

    pub fn register_map(&mut self, id: u64, lat: f64, lon: f64, zoom: u8) {
        let zoom = zoom.max(GEO_POINT_ZOOM_MIN);
        self.items.insert(
            id,
            CachedMediaItem {
                id,
                kind: MediaKind::GeoMap { lat, lon, zoom },
                state: MediaLoadingState::Ready,
                url: format!("geo:{},{}?z={}", lat, lon, zoom),
                cache_path: None,
            },
        );
    }

    pub fn register_document(
        &mut self,
        id: u64,
        url: impl Into<String>,
        filename: impl Into<String>,
        size_bytes: u64,
    ) {
        self.items.insert(
            id,
            CachedMediaItem {
                id,
                kind: MediaKind::Document {
                    filename: filename.into(),
                    size_bytes,
                },
                state: MediaLoadingState::Idle,
                url: url.into(),
                cache_path: None,
            },
        );
    }

    pub fn update_progress(&mut self, id: u64, progress: f32) {
        if let Some(item) = self.items.get_mut(&id) {
            item.state = MediaLoadingState::Downloading {
                progress: progress.clamp(0.0, 1.0),
            };
        }
    }

    pub fn mark_ready(&mut self, id: u64, path: impl Into<String>) {
        if let Some(item) = self.items.get_mut(&id) {
            item.state = MediaLoadingState::Ready;
            item.cache_path = Some(path.into());
        }
    }

    pub fn get_item(&self, id: u64) -> Option<&CachedMediaItem> {
        self.items.get(&id)
    }
}

impl Render for IvCachedMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut list = div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(12.0))
            .bg(rgb(0x1B1F26))
            .rounded(px(8.0))
            .gap(px(8.0));

        let header = div()
            .flex()
            .flex_row()
            .justify_between()
            .text_size(px(13.0))
            .font_weight(FontWeight::BOLD)
            .text_color(rgb(0xCCD2DE))
            .child(format!("Cached Media Resources ({})", self.items.len()));

        list = list.child(header);

        for (id, item) in &self.items {
            let desc = match &item.kind {
                MediaKind::Photo { width, height } => format!("Photo {}x{}", width, height),
                MediaKind::Video { duration_sec, .. } => format!("Video {}s", duration_sec),
                MediaKind::Audio { title, performer, .. } => format!("Audio {} - {}", performer, title),
                MediaKind::Document { filename, size_bytes } => {
                    format!("Doc {} ({} KB)", filename, size_bytes / 1024)
                }
                MediaKind::GeoMap { lat, lon, zoom } => {
                    format!("Map ({:.3}, {:.3}) zoom {}", lat, lon, zoom)
                }
            };

            let status_color = match &item.state {
                MediaLoadingState::Ready => rgb(0x61C554),
                MediaLoadingState::Downloading { .. } => rgb(0x2AABEE),
                MediaLoadingState::Failed(_) => rgb(0xED6A5E),
                MediaLoadingState::Idle => rgb(0x8D95A3),
            };

            list = list.child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .p(px(8.0))
                    .rounded(px(4.0))
                    .bg(rgb(0x232832))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(0xF0F3F8))
                                    .child(desc),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(rgb(0x7D8594))
                                    .child(item.url.clone()),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(status_color)
                            .child(match &item.state {
                                MediaLoadingState::Ready => "READY".to_string(),
                                MediaLoadingState::Downloading { progress } => {
                                    format!("{:.0}%", progress * 100.0)
                                }
                                MediaLoadingState::Failed(err) => err.clone(),
                                MediaLoadingState::Idle => "PENDING".to_string(),
                            }),
                    ),
            );
        }

        list
    }
}
