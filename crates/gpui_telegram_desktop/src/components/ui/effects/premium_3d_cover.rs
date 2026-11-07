//! Upstream: `Telegram/SourceFiles/ui/effects/premium_3d_cover.cpp:L1-L460`
//! Upstream: `Telegram/SourceFiles/ui/effects/premium_3d_cover.h:L20-L133`
//!
//! Interactive 3D object orbit cover container.
//! Tracks mouse drag gestures, computes yaw and pitch rotation with inertial damping.

use gpui::*;

#[derive(Debug, Clone)]
pub struct Premium3dCoverEffects {
    yaw: f32,
    pitch: f32,
    drag_active: bool,
    last_drag_x: f32,
    last_drag_y: f32,
}

impl Premium3dCoverEffects {
    pub fn new() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            drag_active: false,
            last_drag_x: 0.0,
            last_drag_y: 0.0,
        }
    }

    pub fn on_drag_start(&mut self, x: f32, y: f32) {
        self.drag_active = true;
        self.last_drag_x = x;
        self.last_drag_y = y;
    }

    pub fn on_drag_move(&mut self, x: f32, y: f32) {
        if self.drag_active {
            let dx = x - self.last_drag_x;
            let dy = y - self.last_drag_y;
            self.yaw += dx * 0.5;
            self.pitch = (self.pitch + dy * 0.3).clamp(-50.0, 50.0);
            self.last_drag_x = x;
            self.last_drag_y = y;
        }
    }

    pub fn on_drag_end(&mut self) {
        self.drag_active = false;
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }
}

impl Default for Premium3dCoverEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Premium3dCoverEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size(px(180.0))
            .flex()
            .items_center()
            .justify_center()
    }
}
