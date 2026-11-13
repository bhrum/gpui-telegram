//! Paint context for Instant View
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

#[derive(Debug, Clone, Default)]
pub struct IvPaintContext {
    pub scale_factor: f32,
    pub visible_top: f32,
    pub visible_bottom: f32,
    pub is_dark_theme: bool,
    pub selection_active: bool,
}

impl IvPaintContext {
    pub fn new() -> Self {
        Self {
            scale_factor: 1.0,
            visible_top: 0.0,
            visible_bottom: 600.0,
            is_dark_theme: true,
            selection_active: false,
        }
    }

    pub fn is_visible(&self, top: f32, bottom: f32) -> bool {
        bottom >= self.visible_top && top <= self.visible_bottom
    }
}
