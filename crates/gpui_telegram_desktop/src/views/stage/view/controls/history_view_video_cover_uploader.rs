//! Video cover custom thumbnail uploader with radial progress animation.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_video_cover_uploader.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_video_cover_uploader.h`

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoCoverUploadState {
    pub file_path: String,
    pub generation: u64,
    pub is_uploading: bool,
    pub upload_progress_percent: u32,
    pub is_complete: bool,
}

impl Default for VideoCoverUploadState {
    fn default() -> Self {
        Self {
            file_path: String::new(),
            generation: 0,
            is_uploading: false,
            upload_progress_percent: 0,
            is_complete: false,
        }
    }
}

pub struct HistoryViewVideoCoverUploaderViewControls {
    pub state: VideoCoverUploadState,
}

impl HistoryViewVideoCoverUploaderViewControls {
    pub fn new() -> Self {
        Self {
            state: VideoCoverUploadState::default(),
        }
    }

    pub fn start_upload(&mut self, path: impl Into<String>) {
        self.state.generation += 1;
        self.state.file_path = path.into();
        self.state.is_uploading = true;
        self.state.upload_progress_percent = 0;
        self.state.is_complete = false;
    }

    pub fn set_progress(&mut self, percent: u32) {
        self.state.upload_progress_percent = percent.min(100);
        if percent >= 100 {
            self.state.is_uploading = false;
            self.state.is_complete = true;
        }
    }

    pub fn cancel(&mut self) {
        self.state.is_uploading = false;
        self.state.upload_progress_percent = 0;
    }
}

impl Default for HistoryViewVideoCoverUploaderViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewVideoCoverUploaderViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.state.is_uploading && !self.state.is_complete {
            return div().into_any_element();
        }

        let progress = self.state.upload_progress_percent;
        let is_complete = self.state.is_complete;

        div()
            .flex()
            .items_center()
            .gap_2()
            .px_3()
            .py_1()
            .bg(rgb(0x18222d))
            .text_xs()
            .child(
                div()
                    .text_color(rgb(0x2481cc))
                    .child(if is_complete {
                        "Cover Upload Complete".to_string()
                    } else {
                        format!("Uploading Cover... {}%", progress)
                    }),
            )
            .into_any_element()
    }
}
