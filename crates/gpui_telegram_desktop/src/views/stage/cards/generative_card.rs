use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_proto::cards::{GenerativeCard, GenerativeCardLayout, GenerativeComponent};

use crate::theme::TdesktopThemePalette;
use crate::views::stage::cards_interactive::InteractiveCardAction;

fn parse_hex_color(raw: Option<&str>, fallback: u32) -> u32 {
    let Some(s) = raw else { return fallback };
    let hex = s.trim().trim_start_matches('#');
    u32::from_str_radix(hex, 16).unwrap_or(fallback)
}

pub fn render_generative_card<V: 'static>(
    msg_id: u64,
    card: &GenerativeCard,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, InteractiveCardAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let safe_title = GenerativeCard::sanitize_text_once(&card.title);
    let safe_subtitle = card.subtitle.as_deref().map(GenerativeCard::sanitize_text_once);
    let safe_status = card.status.as_deref().map(GenerativeCard::sanitize_text_once);

    let mut body = match card.layout {
        GenerativeCardLayout::VerticalStack => div().flex().flex_col().gap(px(6.0)).w_full(),
        GenerativeCardLayout::HorizontalRow => {
            div().flex().flex_row().flex_wrap().items_center().gap(px(6.0)).w_full()
        }
        GenerativeCardLayout::KeyValueGrid => {
            div().flex().flex_row().flex_wrap().gap(px(8.0)).w_full()
        }
    };

    for (idx, comp) in card.components.iter().enumerate() {
        let el: AnyElement = match comp {
            GenerativeComponent::Text { text, bold, color } => {
                let fg = parse_hex_color(color.as_deref(), palette.text_primary);
                div()
                    .text_size(px(12.5))
                    .text_color(rgb(fg))
                    .when(*bold, |d| d.font_weight(FontWeight::BOLD))
                    .child(GenerativeCard::sanitize_text_once(text))
                    .into_any_element()
            }
            GenerativeComponent::Badge { label, color } => {
                let badge_bg = parse_hex_color(Some(color.as_str()), palette.accent);
                div()
                    .px(px(8.0))
                    .py(px(2.0))
                    .rounded(px(999.0))
                    .bg(rgb(badge_bg))
                    .text_size(px(10.5))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child(GenerativeCard::sanitize_text_once(label))
                    .into_any_element()
            }
            GenerativeComponent::Progress { value, max, label } => {
                let ratio = if !value.is_finite() || !max.is_finite() || *max <= 0.0 {
                    0.0
                } else {
                    (*value / *max).clamp(0.0, 1.0)
                };
                div()
                    .flex()
                    .flex_col()
                    .gap(px(3.0))
                    .w_full()
                    .when_some(label.as_ref(), |d, lbl| {
                        d.child(
                            div()
                                .text_size(px(11.0))
                                .text_color(rgb(palette.text_secondary))
                                .child(GenerativeCard::sanitize_text_once(lbl)),
                        )
                    })
                    .child(
                        div()
                            .w_full()
                            .h(px(6.0))
                            .rounded(px(3.0))
                            .bg(rgb(palette.border_fg))
                            .overflow_hidden()
                            .child(
                                div()
                                    .h_full()
                                    .w(relative(ratio))
                                    .rounded(px(3.0))
                                    .bg(rgb(palette.accent)),
                            ),
                    )
                    .into_any_element()
            }
            GenerativeComponent::Button {
                id,
                label,
                action_payload,
            } => {
                let btn_id = id.clone();
                let payload = action_payload.clone();
                let cb = on_action.clone();
                div()
                    .id(SharedString::from(format!("a2ui-btn-{}-{}-{}", msg_id, idx, id)))
                    .px(px(10.0))
                    .py(px(5.0))
                    .rounded(px(8.0))
                    .bg(palette.service_pill_rgba())
                    .cursor_pointer()
                    .hover(|s| s.opacity(0.85))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        cb(
                            this,
                            InteractiveCardAction::GenerativeButton(
                                msg_id,
                                btn_id.clone(),
                                payload.clone(),
                            ),
                            cx,
                        );
                    }))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(palette.service_pill_fg))
                    .child(GenerativeCard::sanitize_text_once(label))
                    .into_any_element()
            }
            GenerativeComponent::Divider => div()
                .w_full()
                .h(px(1.0))
                .bg(rgb(palette.border_fg))
                .into_any_element(),
        };
        body = body.child(el);
    }

    div()
        .flex()
        .flex_col()
        .w(px(300.0))
        .p(px(10.0))
        .rounded(px(10.0))
        .bg(rgb(palette.card_bg))
        .border_1()
        .border_color(rgb(palette.border_fg))
        .gap(px(6.0))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .gap(px(8.0))
                .child(
                    div()
                        .text_size(px(13.5))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(palette.text_primary))
                        .child(safe_title),
                )
                .when_some(safe_status, |d, st| {
                    d.child(
                        div()
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded(px(4.0))
                            .bg(palette.service_pill_rgba())
                            .text_size(px(10.0))
                            .text_color(rgb(palette.service_pill_fg))
                            .child(st),
                    )
                }),
        )
        .when_some(safe_subtitle, |d, sub| {
            d.child(
                div()
                    .text_size(px(11.5))
                    .text_color(rgb(palette.text_secondary))
                    .child(sub),
            )
        })
        .child(body)
        .into_any_element()
}
