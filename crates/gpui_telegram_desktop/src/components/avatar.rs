use gpui::*;

pub struct Avatar {
    pub label: String,
    pub size: f32,
}

impl Avatar {
    pub fn new(label: impl Into<String>, size: f32) -> Self {
        Self {
            label: label.into(),
            size,
        }
    }
}

impl Render for Avatar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(self.size))
            .h(px(self.size))
            .rounded_full()
            .bg(rgb(0x2AABEE))
            .flex()
            .items_center()
            .justify_center()
            .text_color(rgb(0xFFFFFF))
            .child(self.label.chars().next().unwrap_or('?').to_string())
    }
}
