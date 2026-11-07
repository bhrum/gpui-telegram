//! Authentic GPUI implementation of Telegram Downloads Inner List & Tasks Widget.
//! Source: `Telegram/SourceFiles/info/downloads/info_downloads_inner_widget.cpp:L1-L203`, `info_downloads_inner_widget.h:L1-L79`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DownloadTaskStatus {
    Active,
    Paused,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadTaskItem {
    pub file_id: u64,
    pub filename: String,
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
    pub speed_bytes_per_sec: u64,
    pub status: DownloadTaskStatus,
}

impl DownloadTaskItem {
    pub fn progress(&self) -> f32 {
        if self.total_bytes > 0 {
            (self.downloaded_bytes as f32) / (self.total_bytes as f32)
        } else {
            0.0
        }
    }

    pub fn format_size(&self) -> String {
        let mb_total = (self.total_bytes as f64) / (1024.0 * 1024.0);
        let mb_done = (self.downloaded_bytes as f64) / (1024.0 * 1024.0);
        format!("{:.1} / {:.1} MB", mb_done, mb_total)
    }
}

#[derive(Debug, Clone)]
pub struct InfoDownloadsInnerWidgetDownloads {
    pub tasks: Vec<DownloadTaskItem>,
    pub selected_file_ids: HashSet<u64>,
    pub is_selecting: bool,
}

impl InfoDownloadsInnerWidgetDownloads {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            selected_file_ids: HashSet::new(),
            is_selecting: false,
        }
    }

    pub fn add_task(&mut self, task: DownloadTaskItem) {
        if let Some(pos) = self.tasks.iter().position(|t| t.file_id == task.file_id) {
            self.tasks[pos] = task;
        } else {
            self.tasks.push(task);
        }
    }

    pub fn toggle_pause(&mut self, file_id: u64) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.file_id == file_id) {
            task.status = match task.status {
                DownloadTaskStatus::Active => DownloadTaskStatus::Paused,
                DownloadTaskStatus::Paused => DownloadTaskStatus::Active,
                other => other,
            };
        }
    }

    pub fn cancel_task(&mut self, file_id: u64) {
        self.tasks.retain(|t| t.file_id != file_id);
        self.selected_file_ids.remove(&file_id);
    }

    pub fn clear_completed(&mut self) {
        self.tasks.retain(|t| t.status != DownloadTaskStatus::Completed);
    }
}

impl Default for InfoDownloadsInnerWidgetDownloads {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoDownloadsInnerWidgetDownloads {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.tasks.is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(8.0))
            .when(is_empty, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .py(px(40.0))
                        .text_xs()
                        .text_color(rgb(0x707579))
                        .child("No downloads in queue")
                )
            })
            .when(!is_empty, |this| {
                let mut col = div().flex().flex_col().gap(px(4.0));
                for task in &self.tasks {
                    let progress = task.progress();
                    col = col.child(
                        div()
                            .flex()
                            .flex_col()
                            .p(px(8.0))
                            .rounded(px(6.0))
                            .bg(rgb(0x17212b))
                            .gap(px(4.0))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(task.filename.clone())
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x707579))
                                            .child(task.format_size())
                                    )
                            )
                            // Progress bar
                            .child(
                                div()
                                    .h(px(3.0))
                                    .w_full()
                                    .rounded_full()
                                    .bg(rgb(0x0e1621))
                                    .child(
                                        div()
                                            .h(px(3.0))
                                            .w(px(progress * 200.0))
                                            .rounded_full()
                                            .bg(rgb(0x5288c1))
                                    )
                            )
                    );
                }
                this.child(col)
            })
    }
}
