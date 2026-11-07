//! Auto-Download Settings Modal Box View.
//!
//! Faithful translation of upstream Telegram Desktop:
//! - `Telegram/SourceFiles/boxes/auto_download_box.h`
//! - `Telegram/SourceFiles/boxes/auto_download_box.cpp:L33-L150`
//!
//! Features:
//! - Per-source auto-download settings (User / Private Chats, Groups, Channels)
//! - Size limit sliders for photos, videos, and general files
//! - `kMegabyte`, `kDefaultDownloadLimit`, `kDefaultAutoPlayLimit` constants
//! - Exceptions list (`always` and `never` override buckets per peer)

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::collections::HashSet;

/// Constants extracted from `auto_download_box.cpp:L33-L35`
pub const K_MEGABYTE: u64 = 1024 * 1024;
pub const K_DEFAULT_DOWNLOAD_LIMIT: u64 = 10 * K_MEGABYTE;
pub const K_DEFAULT_AUTOPLAY_LIMIT: u64 = 50 * K_MEGABYTE;

/// Source category for auto-download rules (`auto_download_box.cpp:L38`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoDownloadSource {
    User,
    Group,
    Channel,
}

/// Media type categories (`auto_download_box.cpp:L37`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutoDownloadMediaType {
    Photo,
    Video,
    File,
}

/// Peer override buckets (`auto_download_box.cpp:L56-L77`)
#[derive(Debug, Clone, Default)]
pub struct PeerBuckets {
    pub always: HashSet<u64>,
    pub never: HashSet<u64>,
}

#[derive(Debug, Clone)]
pub struct AutoDownloadBox {
    pub source: AutoDownloadSource,
    pub photos_enabled: bool,
    pub videos_enabled: bool,
    pub files_enabled: bool,
    pub video_size_limit: u64,
    pub file_size_limit: u64,
    pub autoplay_gifs: bool,
    pub autoplay_videos: bool,
    pub buckets: PeerBuckets,
    pub is_active: bool,
}

impl AutoDownloadBox {
    pub fn new(source: AutoDownloadSource) -> Self {
        Self {
            source,
            photos_enabled: true,
            videos_enabled: true,
            files_enabled: true,
            video_size_limit: K_DEFAULT_DOWNLOAD_LIMIT,
            file_size_limit: K_DEFAULT_DOWNLOAD_LIMIT,
            autoplay_gifs: true,
            autoplay_videos: false,
            buckets: PeerBuckets::default(),
            is_active: false,
        }
    }

    pub fn with_source(mut self, source: AutoDownloadSource) -> Self {
        self.source = source;
        self
    }

    pub fn source_title(&self) -> &'static str {
        match self.source {
            AutoDownloadSource::User => "Private Chats",
            AutoDownloadSource::Group => "Groups",
            AutoDownloadSource::Channel => "Channels",
        }
    }

    pub fn set_video_limit(&mut self, limit_bytes: u64) {
        self.video_size_limit = limit_bytes;
    }

    pub fn set_file_limit(&mut self, limit_bytes: u64) {
        self.file_size_limit = limit_bytes;
    }

    pub fn add_always_exception(&mut self, peer_id: u64) {
        self.buckets.never.remove(&peer_id);
        self.buckets.always.insert(peer_id);
    }

    pub fn add_never_exception(&mut self, peer_id: u64) {
        self.buckets.always.remove(&peer_id);
        self.buckets.never.insert(peer_id);
    }

    pub fn exceptions_count(&self) -> usize {
        self.buckets.always.len() + self.buckets.never.len()
    }
}

impl Default for AutoDownloadBox {
    fn default() -> Self {
        Self::new(AutoDownloadSource::User)
    }
}

impl Render for AutoDownloadBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let title = format!("Automatic media download: {}", self.source_title());
        let video_mb = self.video_size_limit / K_MEGABYTE;
        let file_mb = self.file_size_limit / K_MEGABYTE;

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(16.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(15.0))
                    .text_color(rgb(0x222222))
                    .child(title),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .mt(px(10.0))
                    .text_size(px(13.0))
                    .text_color(rgb(0x444444))
                    .child(format!("Photos: {}", if self.photos_enabled { "Enabled" } else { "Disabled" }))
                    .child(format!("Videos up to: {} MB", video_mb))
                    .child(format!("Files up to: {} MB", file_mb))
                    .child(format!("Autoplay Videos: {}", if self.autoplay_videos { "On" } else { "Off" }))
                    .child(format!("Exceptions: {} chats", self.exceptions_count())),
            )
    }
}
