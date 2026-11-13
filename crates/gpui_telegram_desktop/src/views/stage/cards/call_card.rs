//! Canonical Call Log Message Card (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4.8).

use gpui::*;
use super::models::{CallDirection, CallType, MessageItemModel};

pub struct CallMessageView;

impl CallMessageView {
    pub fn render(
        message: &MessageItemModel,
        call_type: CallType,
        direction: CallDirection,
        duration_secs: u32,
        status_text: &str,
    ) -> impl IntoElement {
        let is_outgoing = message.is_outgoing();
        let bg_color = if is_outgoing { rgb(0x2B5278) } else { rgb(0x182533) };

        let (icon_symbol, icon_color, label) = match (call_type, direction) {
            (CallType::Voice, CallDirection::Incoming) => ("↙📞", rgb(0x4CAF50), "Incoming Voice Call"),
            (CallType::Voice, CallDirection::Outgoing) => ("↗📞", rgb(0x2AABEE), "Outgoing Voice Call"),
            (CallType::Voice, CallDirection::Missed) => ("✕📞", rgb(0xE53935), "Missed Voice Call"),
            (CallType::Video, CallDirection::Incoming) => ("↙📹", rgb(0x4CAF50), "Incoming Video Call"),
            (CallType::Video, CallDirection::Outgoing) => ("↗📹", rgb(0x2AABEE), "Outgoing Video Call"),
            (CallType::Video, CallDirection::Missed) => ("✕📹", rgb(0xE53935), "Missed Video Call"),
        };

        div()
            .flex()
            .items_center()
            .justify_between()
            .w(px(280.0))
            .rounded(px(14.0))
            .bg(bg_color)
            .px(px(12.0))
            .py(px(8.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    // Call icon bubble
                    .child(
                        div()
                            .w(px(36.0))
                            .h(px(36.0))
                            .rounded_full()
                            .bg(rgba(0x00000033))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(16.0))
                            .child(icon_symbol),
                    )
                    // Details (Type, duration, status)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(icon_color)
                                    .child(label),
                            )
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .child(if duration_secs > 0 {
                                        format!("{} ({:02}:{:02})", status_text, duration_secs / 60, duration_secs % 60)
                                    } else {
                                        status_text.to_string()
                                    }),
                            ),
                    ),
            )
            // Timestamp
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(rgb(0x8A9AA8))
                    .child(message.timestamp.clone()),
            )
    }
}
