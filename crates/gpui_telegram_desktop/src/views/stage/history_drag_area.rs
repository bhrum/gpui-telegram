//! History Drag Area (File Drop Targets for Photos and Documents).
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/history/history_drag_area.h:L17-L60`
//! - Upstream: `Telegram/SourceFiles/history/history_drag_area.cpp:L36-L50`
//! - Constants:
//!   - `MinimalHeight = 120;` (px, history_drag_area.h:L42)
//!   - `DragAreaEvents`: DragEnter, DragMove, DragLeave, Drop, MouseButtonRelease, Leave (history_drag_area.cpp:L38)

use gpui::*;

pub const DRAG_AREA_MINIMAL_HEIGHT: f32 = 120.0;

/// Drop zone compartment in the history chat area.
/// C++ citation: `DragArea::Areas` (history_drag_area.h:L21-L24).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragTargetZone {
    Photo,
    Document,
}

/// History drag area overlay handling file drag-and-drop operations into chat.
/// C++ citation: `DragArea` (history_drag_area.h:L17-L60).
pub struct HistoryDragArea {
    is_drag_active: bool,
    hovered_zone: Option<DragTargetZone>,
    dropped_files: Vec<String>,
    is_archive_drop: bool,
    height: f32,
}

impl HistoryDragArea {
    pub fn new() -> Self {
        Self {
            is_drag_active: false,
            hovered_zone: None,
            dropped_files: Vec::new(),
            is_archive_drop: false,
            height: DRAG_AREA_MINIMAL_HEIGHT * 2.0,
        }
    }

    /// Minimal height required for rendering the dual drop target zones.
    /// C++ citation: `DragArea::MinimalHeight` (history_drag_area.h:L42).
    pub fn minimal_height() -> f32 {
        DRAG_AREA_MINIMAL_HEIGHT
    }

    /// Handles drag enter event, inspecting dragged file payloads.
    pub fn drag_enter(&mut self, is_image_detected: bool) {
        self.is_drag_active = true;
        self.hovered_zone = if is_image_detected {
            Some(DragTargetZone::Photo)
        } else {
            Some(DragTargetZone::Document)
        };
    }

    /// Updates current hover coordinate within the container.
    /// In C++: top half is photo drop, bottom half is document drop.
    pub fn drag_move(&mut self, local_y: f32, container_height: f32) {
        if !self.is_drag_active {
            return;
        }
        let half = (container_height / 2.0).max(DRAG_AREA_MINIMAL_HEIGHT);
        self.hovered_zone = Some(if local_y < half {
            DragTargetZone::Photo
        } else {
            DragTargetZone::Document
        });
    }

    pub fn drag_leave(&mut self) {
        self.is_drag_active = false;
        self.hovered_zone = None;
    }

    /// Finalizes file drop with target zone classification.
    pub fn drop_files(&mut self, files: Vec<String>, as_archive: bool) -> (DragTargetZone, Vec<String>) {
        let zone = self.hovered_zone.unwrap_or(DragTargetZone::Document);
        self.is_archive_drop = as_archive;
        self.is_drag_active = false;
        self.hovered_zone = None;
        (zone, files)
    }

    pub fn is_active(&self) -> bool {
        self.is_drag_active
    }

    pub fn hovered_zone(&self) -> Option<DragTargetZone> {
        self.hovered_zone
    }
}

impl Default for HistoryDragArea {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryDragArea {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_drag_active {
            return div().into_any_element();
        }

        let is_photo_hovered = self.hovered_zone == Some(DragTargetZone::Photo);
        let is_doc_hovered = self.hovered_zone == Some(DragTargetZone::Document);

        div()
            .absolute()
            .inset_0()
            .flex()
            .flex_col()
            .bg(rgba(0x00000088))
            .p(px(16.0))
            .gap(px(16.0))
            .child(
                // Photo drop zone (top)
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .rounded(px(8.0))
                    .border_2()
                    .border_dashed()
                    .border_color(if is_photo_hovered { rgb(0x2AABEE) } else { rgb(0xCBD5E1) })
                    .bg(if is_photo_hovered { rgba(0x2AABEE33) } else { rgba(0xFFFFFF11) })
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child("Drop images here to send without compression")
                    )
            )
            .child(
                // Document drop zone (bottom)
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .rounded(px(8.0))
                    .border_2()
                    .border_dashed()
                    .border_color(if is_doc_hovered { rgb(0x2AABEE) } else { rgb(0xCBD5E1) })
                    .bg(if is_doc_hovered { rgba(0x2AABEE33) } else { rgba(0xFFFFFF11) })
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child("Drop files here to send as document attachments")
                    )
            )
            .into_any_element()
    }
}
