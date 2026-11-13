use gpui::*;

pub struct ForwardBoxView {
    pub message_count: usize,
}

impl ForwardBoxView {
    pub fn new(message_count: usize) -> Self {
        Self { message_count }
    }
}

impl Render for ForwardBoxView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(320.0))
            .p(px(16.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .child(format!("Forward {} messages to...", self.message_count))
    }
}
