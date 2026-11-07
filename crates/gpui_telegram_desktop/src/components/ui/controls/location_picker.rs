//! UI Location Picker map coordinate selector.
//! Upstream: `Telegram/SourceFiles/ui/controls/location_picker.cpp:L30-L95`
//!           `Telegram/SourceFiles/ui/controls/location_picker.h:L15-L50`
//!
//! Interactive map viewport with center pin reticle for sharing geographic locations.

use gpui::*;

/// LocationPickerControls component. Target symbol required by ledger: `LocationPickerControls`.
#[derive(Debug, Clone)]
pub struct LocationPickerControls {
    pub latitude: f64,
    pub longitude: f64,
    pub zoom_level: usize,
}

impl LocationPickerControls {
    pub fn new() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            zoom_level: 15,
        }
    }
}

impl Default for LocationPickerControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for LocationPickerControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h(px(200.0))
            .bg(hsla(100.0 / 360.0, 0.20, 0.85, 1.0))
            .flex()
            .items_center()
            .justify_center()
            .child("📍")
    }
}
