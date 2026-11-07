use gpui::*;

pub struct Badge {
    pub count: u32,
    pub is_muted: bool,
}

impl Badge {
    pub fn new(count: u32, is_muted: bool) -> Self {
        Self { count, is_muted }
    }
}

impl Render for Badge {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let bg_color = if self.is_muted {
            rgb(0x9E9E9E)
        } else {
            rgb(0x2AABEE)
        };

        div()
            .px(px(6.0))
            .py(px(2.0))
            .rounded_full()
            .bg(bg_color)
            .text_color(rgb(0xFFFFFF))
            .text_size(px(11.0))
            .child(self.count.to_string())
    }
}
