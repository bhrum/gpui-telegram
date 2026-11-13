//! Target Symbol: IvDelegate
//! Module 13: Instant View Host Delegate Interface & Geometry Persistence
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Default for WindowGeometry {
    fn default() -> Self {
        Self {
            x: 100,
            y: 100,
            width: 860,
            height: 900,
        }
    }
}

pub trait IvDelegateInterface {
    fn iv_geometry(&self) -> WindowGeometry;
    fn iv_save_geometry(&mut self, geometry: WindowGeometry);
    fn iv_zoom(&self) -> i32;
    fn iv_set_zoom(&mut self, zoom: i32);
    fn iv_activated(&mut self);
}

#[derive(Debug, Clone)]
pub struct IvDelegate {
    pub geometry: WindowGeometry,
    pub zoom: i32,
    pub is_activated: bool,
}

impl Default for IvDelegate {
    fn default() -> Self {
        Self {
            geometry: WindowGeometry::default(),
            zoom: 100,
            is_activated: false,
        }
    }
}

impl IvDelegate {
    pub fn new() -> Self {
        Self::default()
    }
}

impl IvDelegateInterface for IvDelegate {
    fn iv_geometry(&self) -> WindowGeometry {
        self.geometry
    }

    fn iv_save_geometry(&mut self, geometry: WindowGeometry) {
        self.geometry = geometry;
    }

    fn iv_zoom(&self) -> i32 {
        self.zoom
    }

    fn iv_set_zoom(&mut self, zoom: i32) {
        self.zoom = zoom.clamp(50, 300);
    }

    fn iv_activated(&mut self) {
        self.is_activated = true;
    }
}

impl Render for IvDelegate {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p(px(8.0))
            .bg(rgb(0x1E222A))
            .rounded(px(6.0))
            .text_size(px(12.0))
            .text_color(rgb(0xCCD2DE))
            .child(format!(
                "IV Delegate: {}x{} @ ({}, {}) | Zoom: {}%",
                self.geometry.width, self.geometry.height, self.geometry.x, self.geometry.y, self.zoom
            ))
    }
}
