use gpui::*;

pub struct ThinkingBubble {
    pub thoughts: String,
    pub is_expanded: bool,
}

impl ThinkingBubble {
    pub fn new(thoughts: String) -> Self {
        Self {
            thoughts,
            is_expanded: false,
        }
    }
}

impl Render for ThinkingBubble {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p(px(8.0))
            .rounded(px(8.0))
            .bg(rgb(0xF5F5F5))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x757575))
                    .child(self.thoughts.clone()),
            )
    }
}
