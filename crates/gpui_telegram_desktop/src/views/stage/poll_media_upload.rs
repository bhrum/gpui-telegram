//! Poll Option Media Upload Manager & Radial Progress.
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/poll/poll_media_upload.h:L40-L60`
//! - Upstream: `Telegram/SourceFiles/poll/poll_media_upload.cpp:L40-L150`
//! - Structures:
//!   - `PollMediaState`: media token, uploadDataId, rounded, progress, update callback (poll_media_upload.h:L41-L52)

use gpui::*;

/// State of an active media upload attached to a poll question or option.
/// C++ citation: `PollMediaUpload::PollMediaState` (poll_media_upload.h:L41-L52).
#[derive(Debug, Clone)]
pub struct PollMediaState {
    pub upload_id: u64,
    pub file_name: String,
    pub file_size_bytes: usize,
    pub is_uploading: bool,
    pub progress: f32,
    pub is_rounded: bool,
    pub upload_token: Option<u64>,
    pub uploaded_at_ms: Option<u64>,
    pub error: Option<String>,
}

impl PollMediaState {
    pub fn new(upload_id: u64, file_name: impl Into<String>, size: usize) -> Self {
        Self {
            upload_id,
            file_name: file_name.into(),
            file_size_bytes: size,
            is_uploading: true,
            progress: 0.0,
            is_rounded: false,
            upload_token: None,
            uploaded_at_ms: None,
            error: None,
        }
    }
}

/// Poll media upload coordinator managing asynchronous file transfers and progress.
pub struct PollMediaUpload {
    active_upload: Option<PollMediaState>,
    upload_counter: u64,
}

impl PollMediaUpload {
    pub fn new() -> Self {
        Self {
            active_upload: None,
            upload_counter: 0,
        }
    }

    /// Initiates a new media upload session.
    pub fn start_upload(&mut self, file_name: impl Into<String>, size: usize) -> u64 {
        self.upload_counter = self.upload_counter.saturating_add(1);
        let id = self.upload_counter;
        self.active_upload = Some(PollMediaState::new(id, file_name, size));
        id
    }

    /// Updates upload progress (0.0 .. 1.0).
    pub fn update_progress(&mut self, fraction: f32) {
        if let Some(ref mut upload) = self.active_upload {
            upload.progress = fraction.clamp(0.0, 1.0);
        }
    }

    /// Finalizes upload with server media token.
    pub fn complete_upload(&mut self, token: u64, now_ms: u64) {
        if let Some(ref mut upload) = self.active_upload {
            upload.progress = 1.0;
            upload.is_uploading = false;
            upload.upload_token = Some(token);
            upload.uploaded_at_ms = Some(now_ms);
        }
    }

    /// Sets an error on the active upload.
    pub fn fail_upload(&mut self, error_msg: impl Into<String>) {
        if let Some(ref mut upload) = self.active_upload {
            upload.is_uploading = false;
            upload.error = Some(error_msg.into());
        }
    }

    pub fn cancel(&mut self) {
        self.active_upload = None;
    }

    pub fn active_state(&self) -> Option<&PollMediaState> {
        self.active_upload.as_ref()
    }

    pub fn is_uploading(&self) -> bool {
        self.active_upload.as_ref().map_or(false, |u| u.is_uploading)
    }
}

impl Default for PollMediaUpload {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PollMediaUpload {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let upload = match self.active_upload {
            Some(ref u) => u,
            None => return div().into_any_element(),
        };

        let progress_pct = (upload.progress * 100.0) as u32;

        div()
            .flex()
            .items_center()
            .gap(px(12.0))
            .p(px(8.0))
            .rounded(px(8.0))
            .bg(rgb(0xF4F4F5))
            .child(
                div()
                    .w(px(36.0))
                    .h(px(36.0))
                    .rounded_full()
                    .bg(if upload.is_uploading { rgb(0x2AABEE) } else { rgb(0x10B981) })
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0xFFFFFF))
                    .text_size(px(11.0))
                    .font_weight(FontWeight::BOLD)
                    .child(if upload.is_uploading {
                        format!("{}%", progress_pct)
                    } else {
                        "✓".to_string()
                    })
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x18181B))
                            .child(upload.file_name.clone())
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x71717A))
                            .child(format!("{:.1} KB", upload.file_size_bytes as f32 / 1024.0))
                    )
            )
            .into_any_element()
    }
}
