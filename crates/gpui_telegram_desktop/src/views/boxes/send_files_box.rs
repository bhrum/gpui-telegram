//! Send Files and Media Dispatch Modal Box View.
//!
//! Faithful translation of upstream Telegram Desktop:
//! - `Telegram/SourceFiles/boxes/send_files_box.h`
//! - `Telegram/SourceFiles/boxes/send_files_box.cpp:L89-L150`
//!
//! Features:
//! - `kMaxMessageLength = 4096`
//! - `kMaxDisplayNameLength = 64`
//! - `kStashFilesLimit = 100`, `kStashFilesMemoryLimit = 256MB`
//! - Grouped media album vs individual files sending mode
//! - Compress images toggle, silent send, schedule send, and caption input

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_MAX_MESSAGE_LENGTH: usize = 4096;
pub const K_MAX_DISPLAY_NAME_LENGTH: usize = 64;
pub const K_STASH_FILES_LIMIT: usize = 100;
pub const K_STASH_FILES_MEMORY_LIMIT_BYTES: i64 = 256 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct PreparedMediaFile {
    pub path: String,
    pub display_name: String,
    pub size_bytes: u64,
    pub is_image: bool,
    pub is_video: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendFilesWay {
    Album,
    SeparateFiles,
}

#[derive(Debug, Clone)]
pub struct SendFilesBox {
    pub files: Vec<PreparedMediaFile>,
    pub caption: String,
    pub send_way: SendFilesWay,
    pub compress_images: bool,
    pub is_silent: bool,
    pub scheduled_timestamp: Option<i64>,
}

impl SendFilesBox {
    pub fn new(files: Vec<PreparedMediaFile>) -> Self {
        Self {
            files,
            caption: String::new(),
            send_way: SendFilesWay::Album,
            compress_images: true,
            is_silent: false,
            scheduled_timestamp: None,
        }
    }

    pub fn set_caption(&mut self, text: impl Into<String>) {
        self.caption = text.into().chars().take(K_MAX_MESSAGE_LENGTH).collect();
    }

    pub fn toggle_compress(&mut self) {
        self.compress_images = !self.compress_images;
    }

    pub fn set_send_way(&mut self, way: SendFilesWay) {
        self.send_way = way;
    }

    pub fn total_bytes(&self) -> u64 {
        self.files.iter().map(|f| f.size_bytes).sum()
    }
}

impl Default for SendFilesBox {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl Render for SendFilesBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.files.len();
        let total_mb = self.total_bytes() / (1024 * 1024);

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(16.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xD0D0D0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(15.0))
                    .text_color(rgb(0x1B1B1B))
                    .child(format!("Send {} Files", count)),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .text_size(px(13.0))
                    .text_color(rgb(0x404040))
                    .child(format!("Total size: ~{} MB | Mode: {:?}", total_mb, self.send_way)),
            )
            .child(
                div()
                    .flex()
                    .gap(px(12.0))
                    .mt(px(8.0))
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .child(format!("Compress: {}", if self.compress_images { "Yes" } else { "No" }))
                    .child(format!("Silent: {}", if self.is_silent { "Yes" } else { "No" })),
            )
    }
}
