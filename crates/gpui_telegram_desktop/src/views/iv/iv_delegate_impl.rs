//! Target Symbol: IvDelegateImpl
//! Module 13: Instant View Host Delegate Platform Implementation
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;
use std::collections::HashMap;
use super::iv_delegate::{IvDelegateInterface, WindowGeometry};

#[derive(Debug, Clone)]
pub struct IvDelegateImpl {
    pub geometry: WindowGeometry,
    pub zoom: i32,
    pub monitor_crc: u32,
    pub saved_geometries: HashMap<u32, WindowGeometry>,
    pub is_active: bool,
}

impl Default for IvDelegateImpl {
    fn default() -> Self {
        Self {
            geometry: WindowGeometry {
                x: 120,
                y: 80,
                width: 860,
                height: 920,
            },
            zoom: 100,
            monitor_crc: 0x9A4B12F0,
            saved_geometries: HashMap::new(),
            is_active: false,
        }
    }
}

impl IvDelegateImpl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count_initial_geometry(&self, screen_w: u32, screen_h: u32) -> WindowGeometry {
        let width = (screen_w.min(860)).max(480);
        let height = (screen_h.saturating_sub(100)).max(600);
        let x = (screen_w.saturating_sub(width) / 2) as i32;
        let y = 60;
        WindowGeometry {
            x,
            y,
            width,
            height,
        }
    }

    pub fn clamp_to_screen(&mut self, screen_w: u32, screen_h: u32) {
        if self.geometry.x < 0 {
            self.geometry.x = 0;
        }
        if self.geometry.y < 0 {
            self.geometry.y = 0;
        }
        if (self.geometry.x as u32) + self.geometry.width > screen_w {
            self.geometry.x = (screen_w.saturating_sub(self.geometry.width)) as i32;
        }
        if (self.geometry.y as u32) + self.geometry.height > screen_h {
            self.geometry.y = (screen_h.saturating_sub(self.geometry.height)) as i32;
        }
    }
}

impl IvDelegateInterface for IvDelegateImpl {
    fn iv_geometry(&self) -> WindowGeometry {
        if let Some(saved) = self.saved_geometries.get(&self.monitor_crc) {
            *saved
        } else {
            self.geometry
        }
    }

    fn iv_save_geometry(&mut self, geometry: WindowGeometry) {
        self.geometry = geometry;
        self.saved_geometries.insert(self.monitor_crc, geometry);
    }

    fn iv_zoom(&self) -> i32 {
        self.zoom
    }

    fn iv_set_zoom(&mut self, zoom: i32) {
        self.zoom = zoom.clamp(50, 300);
    }

    fn iv_activated(&mut self) {
        self.is_active = true;
    }
}

impl Render for IvDelegateImpl {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p(px(8.0))
            .bg(rgb(0x1B2028))
            .rounded(px(6.0))
            .text_size(px(12.0))
            .text_color(rgb(0xD8DEE9))
            .child(format!(
                "Platform Delegate: {}x{} @ ({}, {}) | Zoom: {}% | Saved Screens: {}",
                self.geometry.width,
                self.geometry.height,
                self.geometry.x,
                self.geometry.y,
                self.zoom,
                self.saved_geometries.len(),
            ))
    }
}
