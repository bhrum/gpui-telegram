//! Authentic GPUI implementation of Telegram Downloads Drawer Container Widget.
//! Source: `Telegram/SourceFiles/info/downloads/info_downloads_widget.cpp:L1-L143`, `info_downloads_widget.h:L1-L76`
//! Subsystem: info | Feature ID: TCG-N38 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};

use super::info_downloads_inner_widget::InfoDownloadsInnerWidgetDownloads;
use super::info_downloads_provider::InfoDownloadsProviderDownloads;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadsWidgetMemento {
    pub scroll_top: f32,
    pub is_searching: bool,
}

#[derive(Debug, Clone)]
pub struct InfoDownloadsWidgetDownloads {
    pub inner: InfoDownloadsInnerWidgetDownloads,
    pub provider: InfoDownloadsProviderDownloads,
    pub search_query: String,
    pub scroll_top: f32,
}

impl InfoDownloadsWidgetDownloads {
    pub fn new() -> Self {
        Self {
            inner: InfoDownloadsInnerWidgetDownloads::new(),
            provider: InfoDownloadsProviderDownloads::new(),
            search_query: String::new(),
            scroll_top: 0.0,
        }
    }

    pub fn pause_all(&mut self) {
        self.provider.pause_all();
    }

    pub fn resume_all(&mut self) {
        self.provider.resume_all();
    }

    pub fn clear_completed(&mut self) {
        self.inner.clear_completed();
    }

    pub fn save_memento(&self) -> DownloadsWidgetMemento {
        DownloadsWidgetMemento {
            scroll_top: self.scroll_top,
            is_searching: !self.search_query.is_empty(),
        }
    }

    pub fn restore_memento(&mut self, memento: &DownloadsWidgetMemento) {
        self.scroll_top = memento.scroll_top;
    }
}

impl Default for InfoDownloadsWidgetDownloads {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoDownloadsWidgetDownloads {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let active_count = self.provider.active_count();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0e1621))
            .p(px(8.0))
            .gap(px(8.0))
            // Header Controls Toolbar
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .pb(px(8.0))
                    .border_b(px(1.0))
                    .border_color(rgb(0x17212b))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .child("Downloads")
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x5288c1))
                                    .child(if active_count > 0 { "Pause All" } else { "Resume All" })
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x707579))
                                    .child("Clear Finished")
                            )
                    )
            )
            // Task list container
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x707579))
                            .child(format!("{} items in queue", self.inner.tasks.len()))
                    )
            )
    }
}
