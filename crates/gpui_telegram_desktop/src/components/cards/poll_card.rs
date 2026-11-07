use gpui::*;
use gpui_telegram_proto::cards::PollCard;

pub struct PollCardView {
    pub poll: PollCard,
}

impl PollCardView {
    pub fn new(poll: PollCard) -> Self {
        Self { poll }
    }
}

impl Render for PollCardView {
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
                    .child(self.poll.question.clone()),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x757575))
                    .child(format!("{} votes", self.poll.total_voters)),
            )
    }
}
