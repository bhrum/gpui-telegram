//! Info Media Widget
//! Upstream: `Telegram/SourceFiles/info/media/info_media_widget.cpp:L1-L261`
//! Upstream: `Telegram/SourceFiles/info/media/info_media_widget.h:L1-L143`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;
use super::info_media_common::SharedMediaType;

/// Root container widget for the shared media drawer panel.
/// Coordinates the header bar, section buttons, search field, and inner content list.
#[derive(Debug, Clone)]
pub struct InfoMediaWidgetMedia {
    pub peer_id: i64,
    pub current_type: SharedMediaType,
    pub is_searching: bool,
    pub search_query: String,
}

impl InfoMediaWidgetMedia {
    pub fn new(peer_id: i64) -> Self {
        Self {
            peer_id,
            current_type: SharedMediaType::PhotoVideo,
            is_searching: false,
            search_query: String::new(),
        }
    }

    pub fn set_type(&mut self, media_type: SharedMediaType) {
        self.current_type = media_type;
    }

    pub fn toggle_search(&mut self) {
        self.is_searching = !self.is_searching;
        if !self.is_searching {
            self.search_query.clear();
        }
    }

    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
    }
}

impl Default for InfoMediaWidgetMedia {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for InfoMediaWidgetMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_searching = self.is_searching;
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x18222d))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .h(px(48.0))
                    .px(px(12.0))
                    .bg(rgb(0x202b36))
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child(format!("Shared {}", self.current_type.label()))
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x5288c1))
                            .child(if is_searching { "Close Search" } else { "Search" })
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .p(px(8.0))
                    .child(format!("Viewing shared items for peer {}", self.peer_id))
            )
    }
}
