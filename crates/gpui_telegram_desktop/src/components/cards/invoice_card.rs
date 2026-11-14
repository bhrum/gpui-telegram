use crate::theme::{assets::{IconName, tg_icon}, TdesktopThemePalette};
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_proto::cards::InvoiceCard;
use std::sync::Arc;

pub struct InvoiceCardView {
    pub invoice: InvoiceCard,
    pub palette: TdesktopThemePalette,
    pub on_pay: Option<Arc<dyn Fn(String) + Send + Sync>>,
}

impl InvoiceCardView {
    pub fn new(invoice: InvoiceCard) -> Self {
        Self {
            invoice,
            palette: TdesktopThemePalette::default(),
            on_pay: None,
        }
    }

    pub fn with_palette(mut self, palette: TdesktopThemePalette) -> Self {
        self.palette = palette;
        self
    }

    pub fn trigger_pay(&mut self) {
        if !self.invoice.is_paid {
            if let Some(ref cb) = self.on_pay {
                cb(self.invoice.id.clone());
            }
        }
    }
}

impl Render for InvoiceCardView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;

        div()
            .p(px(12.0))
            .rounded(px(pal.bubble_radius_px as f32))
            .bg(rgb(pal.card_bg))
            .border_1()
            .border_color(rgb(pal.border_fg))
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(14.0))
                            .text_color(rgb(pal.text_primary))
                            .child(self.invoice.title.clone()),
                    )
                    .child(tg_icon(IconName::WalletStars, 16.0, pal.accent)),
            )
            .when(!self.invoice.description.is_empty(), |el| {
                el.child(
                    div()
                        .text_size(px(12.5))
                        .text_color(rgb(pal.text_secondary))
                        .child(self.invoice.description.clone()),
                )
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.success))
                            .child(format!("{} {}", self.invoice.total_amount, self.invoice.currency)),
                    )
                    .child(
                        div()
                            .cursor_pointer()
                            .px(px(12.0))
                            .py(px(5.0))
                            .rounded(px(6.0))
                            .bg(rgb(if self.invoice.is_paid {
                                pal.muted_badge_bg
                            } else {
                                pal.accent
                            }))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(pal.active_button_fg))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _e, _w, cx| {
                                    this.trigger_pay();
                                    cx.notify();
                                }),
                            )
                            .child(if self.invoice.is_paid { "Paid" } else { "Pay" }),
                    ),
            )
    }
}
