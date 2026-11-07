//! Target Symbol: IvZoomControls
//! Module 13: Instant View Zoom Controls & Menu Action Coordinator
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const ZOOM_STEP_DEFAULT: f32 = 0.10;
pub const ZOOM_STEP_SMALL: f32 = 0.05;
pub const ZOOM_STEP_TINY: f32 = 0.02;
pub const ZOOM_MIN: f32 = 0.50;
pub const ZOOM_MAX: f32 = 3.00;
pub const ZOOM_DEFAULT: f32 = 1.00;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ZoomStepKind {
    #[default]
    Normal,
    Small,
    Tiny,
}

impl ZoomStepKind {
    pub fn delta(self) -> f32 {
        match self {
            Self::Normal => ZOOM_STEP_DEFAULT,
            Self::Small => ZOOM_STEP_SMALL,
            Self::Tiny => ZOOM_STEP_TINY,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZoomMenuAction {
    ZoomIn,
    ZoomOut,
    Reset,
    Set(f32),
}

#[derive(Debug, Clone)]
pub struct IvZoomControls {
    pub zoom_factor: f32,
    pub is_menu_open: bool,
    pub min_zoom: f32,
    pub max_zoom: f32,
}

impl Default for IvZoomControls {
    fn default() -> Self {
        Self {
            zoom_factor: ZOOM_DEFAULT,
            is_menu_open: false,
            min_zoom: ZOOM_MIN,
            max_zoom: ZOOM_MAX,
        }
    }
}

impl IvZoomControls {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_zoom(mut self, zoom: f32) -> Self {
        self.set_zoom(zoom);
        self
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_factor = zoom.clamp(self.min_zoom, self.max_zoom);
    }

    pub fn zoom_in(&mut self, step: ZoomStepKind) -> f32 {
        let new_zoom = (self.zoom_factor + step.delta()).min(self.max_zoom);
        self.zoom_factor = (new_zoom * 100.0).round() / 100.0;
        self.zoom_factor
    }

    pub fn zoom_out(&mut self, step: ZoomStepKind) -> f32 {
        let new_zoom = (self.zoom_factor - step.delta()).max(self.min_zoom);
        self.zoom_factor = (new_zoom * 100.0).round() / 100.0;
        self.zoom_factor
    }

    pub fn reset_zoom(&mut self) -> f32 {
        self.zoom_factor = ZOOM_DEFAULT;
        self.zoom_factor
    }

    pub fn zoom_percentage(&self) -> i32 {
        (self.zoom_factor * 100.0).round() as i32
    }

    pub fn can_zoom_in(&self) -> bool {
        self.zoom_factor < self.max_zoom - 0.001
    }

    pub fn can_zoom_out(&self) -> bool {
        self.zoom_factor > self.min_zoom + 0.001
    }

    pub fn toggle_menu(&mut self) {
        self.is_menu_open = !self.is_menu_open;
    }
}

impl Render for IvZoomControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let zoom_str = format!("{}%", self.zoom_percentage());
        let can_in = self.can_zoom_in();
        let can_out = self.can_zoom_out();

        div()
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py_1()
            .rounded_md()
            .bg(rgb(0x2d3238))
            .text_color(rgb(0xe0e4e8))
            .child(
                div()
                    .px_1()
                    .cursor_pointer()
                    .opacity(if can_out { 1.0 } else { 0.4 })
                    .child("-"),
            )
            .child(
                div()
                    .px_2()
                    .text_sm()
                    .cursor_pointer()
                    .child(zoom_str),
            )
            .child(
                div()
                    .px_1()
                    .cursor_pointer()
                    .opacity(if can_in { 1.0 } else { 0.4 })
                    .child("+"),
            )
    }
}
