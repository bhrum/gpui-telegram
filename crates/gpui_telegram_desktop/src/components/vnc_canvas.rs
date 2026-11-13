use gpui::*;

pub struct VncCanvas {
    pub remote_connected: bool,
}

impl VncCanvas {
    pub fn new() -> Self {
        Self {
            remote_connected: false,
        }
    }
}

impl Default for VncCanvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for VncCanvas {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .bg(rgb(0x000000))
            .flex()
            .items_center()
            .justify_center()
            .text_color(rgb(0xFFFFFF))
            .child(if self.remote_connected {
                "Remote Desktop Connected"
            } else {
                "Connecting to Remote Host..."
            })
    }
}
