//! Target Symbol: IvMarkdownViewWidgetMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_view_widget.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const K_WHEEL_DELTA_STEP: i32 = 120;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DragAction {
    #[default]
    None,
    Selecting,
    Dragging,
}

#[derive(Debug, Clone, Default)]
pub struct PopupMenu {
    pub is_open: bool,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Default)]
pub struct QuotePaintCache {
    pub quote_id: u64,
    pub cached_height: f32,
}

#[derive(Debug, Clone, Default)]
pub struct QTouchEvent {
    pub touch_points: usize,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Default)]
pub struct QWheelEvent {
    pub delta_y: i32,
    pub is_horizontal: bool,
}

#[derive(Debug, Clone, Default)]
pub struct MarkdownDocumentWidget {
    pub content_height: f32,
    pub zoom_level: i32,
    pub drag_action: DragAction,
    pub is_placeholder_loading: bool,
    pub active_popup: Option<PopupMenu>,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownViewWidgetMarkdown {
    pub widget: MarkdownDocumentWidget,
    pub paint_cache: Vec<QuotePaintCache>,
    pub needs_relayout: bool,
    pub needs_repaint: bool,
}

impl Default for IvMarkdownViewWidgetMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownViewWidgetMarkdown {
    pub fn new() -> Self {
        Self {
            widget: MarkdownDocumentWidget {
                content_height: 1200.0,
                zoom_level: 100,
                drag_action: DragAction::None,
                is_placeholder_loading: false,
                active_popup: None,
            },
            paint_cache: Vec::new(),
            needs_relayout: false,
            needs_repaint: false,
        }
    }

    pub fn set_zoom(&mut self, zoom: i32) {
        self.widget.zoom_level = zoom;
        self.needs_relayout = true;
    }

    pub fn set_placeholder_loading(&mut self, loading: bool) {
        self.widget.is_placeholder_loading = loading;
        self.needs_repaint = true;
    }

    pub fn clear_placeholder_loading(&mut self) {
        self.widget.is_placeholder_loading = false;
        self.needs_repaint = true;
    }

    pub fn invalidate_raster_cache(&mut self) {
        self.paint_cache.clear();
        self.needs_repaint = true;
    }

    pub fn toggle_details(&mut self) {
        self.needs_relayout = true;
        self.needs_repaint = true;
    }

    pub fn toggle_blockquote(&mut self) {
        self.needs_relayout = true;
        self.needs_repaint = true;
    }

    pub fn resize_get_height(&mut self, width: f32) -> f32 {
        self.widget.content_height = (width * 1.5).max(400.0);
        self.widget.content_height
    }

    pub fn handle_wheel(&mut self, event: &QWheelEvent) {
        let steps = event.delta_y / K_WHEEL_DELTA_STEP;
        // Adjust scroll offset by steps
        self.needs_repaint = true;
    }
}

impl Render for IvMarkdownViewWidgetMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let height = self.widget.content_height;
        let zoom = self.widget.zoom_level;
        let loading = self.widget.is_placeholder_loading;
        let cache_entries = self.paint_cache.len();

        div()
            .flex()
            .flex_col()
            .gap_2()
            .p_3()
            .bg(rgb(0x1e2124))
            .text_color(rgb(0xf0f0f0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child("Instant View Markdown Document Widget"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Height: {:.0}px", height))
                    .child(format!("Zoom: {}%", zoom))
                    .child(format!("Loading: {}", loading))
                    .child(format!("Paint Cache: {}", cache_entries)),
            )
    }
}
