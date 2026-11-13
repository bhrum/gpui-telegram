use gpui::*;

pub struct TalkModeOrb {
    pub energy_level: f32,
    pub is_active: bool,
}

impl TalkModeOrb {
    pub fn new() -> Self {
        Self {
            energy_level: 0.0,
            is_active: false,
        }
    }
}

impl Default for TalkModeOrb {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for TalkModeOrb {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(160.0))
            .h(px(160.0))
            .rounded_full()
            .bg(rgb(0x2AABEE))
            .flex()
            .items_center()
            .justify_center()
            .child("Orb")
    }
}
