//! InlineBotDownloads: File download manager for Telegram Mini Apps.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/inline_bots/inline_bot_downloads.h:L24-L60`
//! - `Telegram/SourceFiles/inline_bots/inline_bot_downloads.cpp:L34-L80`
#![allow(dead_code)]

use gpui::*;
use std::collections::HashMap;

/// Downloads storage version (upstream: `kDownloadsVersion = 1`).
pub const K_DOWNLOADS_VERSION: u32 = 1;

/// Maximum number of bots with tracked downloads (upstream: `kMaxDownloadsBots = 4096`).
pub const K_MAX_DOWNLOADS_BOTS: usize = 4096;

/// Maximum downloads tracked per bot (upstream: `kMaxDownloadsPerBot = 16384`).
pub const K_MAX_DOWNLOADS_PER_BOT: usize = 16384;

/// Download entry state.
#[derive(Debug, Clone, PartialEq)]
pub enum DownloadState {
    Pending,
    Loading(f32),
    Paused,
    Completed,
    Failed(String),
}

/// Download action command (upstream: `DownloadsAction`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadAction {
    Pause,
    Resume,
    Cancel,
    Open,
}

/// Single download task metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct DownloadEntry {
    pub id: u32,
    pub bot_id: u64,
    pub url: String,
    pub path: String,
    pub state: DownloadState,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
}

/// Authentic InlineBotDownloads component state.
pub struct InlineBotDownloads {
    pub downloads: HashMap<u32, DownloadEntry>,
    pub next_id: u32,
    pub updated_at: i64,
}

impl InlineBotDownloads {
    pub fn new() -> Self {
        Self {
            downloads: HashMap::new(),
            next_id: 1,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn start_download(
        &mut self,
        bot_id: u64,
        url: impl Into<String>,
        path: impl Into<String>,
    ) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.downloads.insert(
            id,
            DownloadEntry {
                id,
                bot_id,
                url: url.into(),
                path: path.into(),
                state: DownloadState::Pending,
                downloaded_bytes: 0,
                total_bytes: 0,
            },
        );
        self.updated_at = chrono::Utc::now().timestamp();
        id
    }

    pub fn update_progress(&mut self, id: u32, downloaded: u64, total: u64) {
        if let Some(entry) = self.downloads.get_mut(&id) {
            entry.downloaded_bytes = downloaded;
            entry.total_bytes = total;
            let frac = if total > 0 {
                (downloaded as f32) / (total as f32)
            } else {
                0.0
            };
            if downloaded >= total && total > 0 {
                entry.state = DownloadState::Completed;
            } else {
                entry.state = DownloadState::Loading(frac);
            }
            self.updated_at = chrono::Utc::now().timestamp();
        }
    }

    pub fn apply_action(&mut self, id: u32, action: DownloadAction) {
        if let Some(entry) = self.downloads.get_mut(&id) {
            match action {
                DownloadAction::Pause => entry.state = DownloadState::Paused,
                DownloadAction::Resume => {
                    let frac = if entry.total_bytes > 0 {
                        (entry.downloaded_bytes as f32) / (entry.total_bytes as f32)
                    } else {
                        0.0
                    };
                    entry.state = DownloadState::Loading(frac);
                }
                DownloadAction::Cancel => {
                    entry.state = DownloadState::Failed("Cancelled".to_string());
                }
                DownloadAction::Open => {}
            }
            self.updated_at = chrono::Utc::now().timestamp();
        }
    }

    pub fn list_for_bot(&self, bot_id: u64) -> Vec<&DownloadEntry> {
        self.downloads.values().filter(|e| e.bot_id == bot_id).collect()
    }
}

impl Default for InlineBotDownloads {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InlineBotDownloads {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.downloads.len();

        div()
            .flex()
            .items_center()
            .p(px(8.0))
            .bg(rgb(0xFFFFFF))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x333333))
                    .child(format!("Active downloads: {}", count))
            )
    }
}
