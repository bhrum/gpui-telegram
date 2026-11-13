use gpui::*;

pub struct VoiceBubblePlayer {
    pub duration_secs: u32,
    pub is_playing: bool,
    pub progress: f32,
}

impl VoiceBubblePlayer {
    pub fn new(duration_secs: u32) -> Self {
        Self {
            duration_secs,
            is_playing: false,
            progress: 0.0,
        }
    }
}

impl Render for VoiceBubblePlayer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .p(px(8.0))
            .child(
                div()
                    .w(px(32.0))
                    .h(px(32.0))
                    .rounded_full()
                    .bg(rgb(0x2AABEE))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0xFFFFFF))
                    .child(if self.is_playing { "||" } else { "▶" }),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .child(format!("{}:{:02}", self.duration_secs / 60, self.duration_secs % 60)),
            )
    }
}
