use gpui::*;
use gpui_telegram_proto::cards::InvoiceCard;

pub struct InvoiceCardView {
    pub invoice: InvoiceCard,
}

impl InvoiceCardView {
    pub fn new(invoice: InvoiceCard) -> Self {
        Self { invoice }
    }
}

impl Render for InvoiceCardView {
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
                    .child(self.invoice.title.clone()),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(rgb(0x4CAF50))
                    .child(format!("{} {}", self.invoice.total_amount, self.invoice.currency)),
            )
    }
}
