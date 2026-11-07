use gpui::*;
use gpui_telegram_proto::cards::TodoCard;

pub struct TodoCardView {
    pub todo: TodoCard,
}

impl TodoCardView {
    pub fn new(todo: TodoCard) -> Self {
        Self { todo }
    }
}

impl Render for TodoCardView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p(px(12.0))
            .rounded(px(8.0))
            .bg(rgb(0xFFFFFF))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child(self.todo.title.clone()),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x757575))
                    .child(format!("{} tasks", self.todo.items.len())),
            )
    }
}
