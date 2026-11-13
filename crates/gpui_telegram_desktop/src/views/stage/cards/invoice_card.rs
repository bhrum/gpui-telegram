//! Canonical Invoice Message Card (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4.10).

use gpui::*;
use super::models::{InvoiceStatus, MessageItemModel};

pub struct InvoiceMessageView;

impl InvoiceMessageView {
    pub fn render(
        message: &MessageItemModel,
        title: &str,
        description: &str,
        amount_stars: u64,
        currency: &str,
        status: InvoiceStatus,
        receipt_id: Option<&str>,
    ) -> impl IntoElement {
        let is_outgoing = message.is_outgoing();
        let bg_color = if is_outgoing { rgb(0x2B5278) } else { rgb(0x182533) };

        div()
            .flex()
            .flex_col()
            .w(px(320.0))
            .rounded(px(14.0))
            .bg(bg_color)
            .p(px(12.0))
            .gap(px(10.0))
            // Product Header & Star Badge
            .child(
                div()
                    .flex()
                    .items_start()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_size(px(15.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(title.to_string()),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .child(description.to_string()),
                            ),
                    )
                    // Star Icon
                    .child(
                        div()
                            .w(px(36.0))
                            .h(px(36.0))
                            .rounded_full()
                            .bg(rgb(0xF39C12)) // Gold Star tone
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(18.0))
                            .text_color(rgb(0xFFFFFF))
                            .child("⭐"),
                    ),
            )
            // Amount & Currency Display
            .child(
                div()
                    .flex()
                    .items_baseline()
                    .gap(px(6.0))
                    .child(
                        div()
                            .text_size(px(22.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFD700)) // Gold
                            .child(format!("⭐ {}", amount_stars)),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(format!("({})", currency)),
                    ),
            )
            // Pay Action or Paid Receipt Status
            .child(
                match status {
                    InvoiceStatus::Pending => {
                        div()
                            .w_full()
                            .h(px(36.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x2AABEE))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(format!("Pay ⭐ {}", amount_stars))
                    }
                    InvoiceStatus::Paid => {
                        div()
                            .w_full()
                            .h(px(36.0))
                            .rounded(px(8.0))
                            .bg(rgba(0x4CAF5033))
                            .border_1()
                            .border_color(rgb(0x4CAF50))
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(6.0))
                            .child(div().text_color(rgb(0x4CAF50)).child("✓"))
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x4CAF50))
                                    .child(if let Some(r) = receipt_id {
                                        format!("Receipt #{}", r)
                                    } else {
                                        "Payment Completed".to_string()
                                    }),
                            )
                    }
                    InvoiceStatus::Refunded => {
                        div()
                            .w_full()
                            .h(px(36.0))
                            .rounded(px(8.0))
                            .bg(rgba(0xE5393522))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(13.0))
                            .text_color(rgb(0xE53935))
                            .child("Payment Refunded")
                    }
                },
            )
            // Timestamp
            .child(
                div()
                    .flex()
                    .justify_end()
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(message.timestamp.clone()),
                    ),
            )
    }
}
