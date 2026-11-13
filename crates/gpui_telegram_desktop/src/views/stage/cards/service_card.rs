//! Canonical Service Message Card (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4.7).

use gpui::*;
use super::models::ServiceType;

pub struct ServiceMessageView;

impl ServiceMessageView {
    pub fn render(text: &str, service_type: ServiceType) -> impl IntoElement {
        let icon = match service_type {
            ServiceType::UserJoined => "👋 ",
            ServiceType::PinnedMessage => "📌 ",
            ServiceType::SecurityKeyChanged => "🔐 ",
            ServiceType::CallEnded => "📞 ",
            ServiceType::GroupCreated => "👥 ",
            ServiceType::DateDivider => "",
        };

        div()
            .w_full()
            .flex()
            .justify_center()
            .mt(px(10.0))
            .mb(px(2.0))
            .child(
                div()
                    .pl(px(12.0))
                    .pr(px(12.0))
                    .pt(px(3.0))
                    .pb(px(4.0))
                    .rounded_full()
                    .bg(rgba(0x00000055)) // Translucent centered service pill
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0xD1E0EC))
                            .child(format!("{}{}", icon, text)),
                    ),
            )
    }
}
