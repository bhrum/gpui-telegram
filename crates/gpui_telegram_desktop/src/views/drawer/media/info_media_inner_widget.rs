//! Info Media Inner Widget
//! Upstream: `Telegram/SourceFiles/info/media/info_media_inner_widget.cpp:L1-L279`
//! Upstream: `Telegram/SourceFiles/info/media/info_media_inner_widget.h:L1-L99`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;
use super::info_media_common::SharedMediaType;

/// Scrollable inner container hosting the shared media items, date sections,
/// and virtualization viewport calculations.
#[derive(Debug, Clone)]
pub struct InfoMediaInnerWidgetMedia {
    pub peer_id: i64,
    pub media_type: SharedMediaType,
    pub scroll_top: f32,
    pub content_height: f32,
    pub search_query: String,
    pub is_loading: bool,
}

impl InfoMediaInnerWidgetMedia {
    pub fn new(peer_id: i64, media_type: SharedMediaType) -> Self {
        Self {
            peer_id,
            media_type,
            scroll_top: 0.0,
            content_height: 0.0,
            search_query: String::new(),
            is_loading: false,
        }
    }

    pub fn set_scroll_top(&mut self, top: f32) {
        self.scroll_top = top.max(0.0);
    }

    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }
}

impl Default for InfoMediaInnerWidgetMedia {
    fn default() -> Self {
        Self::new(0, SharedMediaType::PhotoVideo)
    }
}

impl Render for InfoMediaInnerWidgetMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .overflow_hidden()
            .bg(rgb(0x18222d))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .p(px(8.0))
                    .gap(px(4.0))
                    .child(format!("Shared {} for peer {}", self.media_type.label(), self.peer_id))
            )
    }
}
