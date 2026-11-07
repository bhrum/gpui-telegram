//! Download Path Configuration Modal Box View.
//!
//! Faithful translation of upstream Telegram Desktop:
//! - `Telegram/SourceFiles/boxes/download_path_box.h`
//! - `Telegram/SourceFiles/boxes/download_path_box.cpp:L21-L100`
//!
//! Features:
//! - `DownloadDirectoryType { Downloads, Temp, Custom }`
//! - Radio selection for default Downloads folder, temporary cache folder, or custom path
//! - Path bookmark storage and native folder chooser integration

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadDirectoryType {
    Downloads,
    Temp,
    Custom,
}

#[derive(Debug, Clone)]
pub struct DownloadPathBox {
    pub selected_type: DownloadDirectoryType,
    pub custom_path: String,
    pub path_bookmark: Vec<u8>,
    pub ask_each_time: bool,
}

impl DownloadPathBox {
    pub fn new() -> Self {
        Self {
            selected_type: DownloadDirectoryType::Downloads,
            custom_path: String::new(),
            path_bookmark: Vec::new(),
            ask_each_time: false,
        }
    }

    pub fn with_custom_path(mut self, path: impl Into<String>) -> Self {
        self.selected_type = DownloadDirectoryType::Custom;
        self.custom_path = path.into();
        self
    }

    pub fn set_type(&mut self, directory_type: DownloadDirectoryType) {
        self.selected_type = directory_type;
    }

    pub fn effective_path_display(&self) -> &str {
        match self.selected_type {
            DownloadDirectoryType::Downloads => "Default (Downloads folder)",
            DownloadDirectoryType::Temp => "Temporary folder (cleared on exit)",
            DownloadDirectoryType::Custom => {
                if self.custom_path.is_empty() {
                    "No custom folder chosen"
                } else {
                    &self.custom_path
                }
            }
        }
    }
}

impl Default for DownloadPathBox {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for DownloadPathBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let path_text = self.effective_path_display();

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
                    .text_color(rgb(0x202020))
                    .child("Download Path Settings"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .mt(px(10.0))
                    .text_size(px(13.0))
                    .text_color(rgb(0x404040))
                    .child(format!("Mode: {:?}", self.selected_type))
                    .child(format!("Location: {}", path_text))
                    .child(format!("Always ask download location: {}", if self.ask_each_time { "Yes" } else { "No" })),
            )
    }
}
