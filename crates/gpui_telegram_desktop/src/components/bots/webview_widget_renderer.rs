//! Widget rendering helpers for [`BotAttachWebView`] Mini App UI schemas.

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_telegram_proto::miniapp_contracts::{MiniAppUiWidget, MiniAppWidgetType};

use super::bot_attach_web_view::{BotAttachWebView, BotWebViewInputTarget};

impl BotAttachWebView {
    pub(crate) fn parse_hex_color(raw: Option<&str>, fallback: u32) -> u32 {
        raw.and_then(|s| {
            let trimmed = s.trim().trim_start_matches('#');
            if trimmed.len() == 6 {
                u32::from_str_radix(trimmed, 16).ok()
            } else {
                None
            }
        })
        .unwrap_or(fallback)
    }

    pub(crate) fn render_widget(&self, w: &MiniAppUiWidget, cx: &mut Context<Self>) -> AnyElement {
        let pal = self.palette;
        let wid = w.id.clone();
        let is_focused = matches!(
            &self.focused_input,
            Some(BotWebViewInputTarget::WidgetText(fid)) if fid == &w.id
        );

        match w.kind {
            MiniAppWidgetType::Header => div()
                .w_full()
                .pt(px(4.0))
                .pb(px(2.0))
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(
                    div()
                        .text_size(px(15.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .child(if !w.label.is_empty() {
                            w.label.clone()
                        } else {
                            w.value.clone()
                        }),
                )
                .when_some(w.subtitle.clone(), |col, sub| {
                    col.child(
                        div()
                            .text_size(px(11.5))
                            .text_color(rgb(pal.text_secondary))
                            .child(sub),
                    )
                })
                .into_any_element(),

            MiniAppWidgetType::Badge => div()
                .px(px(10.0))
                .py(px(4.0))
                .rounded_full()
                .bg(rgb(pal.composer_bg))
                .border_1()
                .border_color(rgb(pal.accent))
                .text_size(px(11.5))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(pal.accent))
                .child(if w.value.is_empty() {
                    w.label.clone()
                } else {
                    format!("{}: {}", w.label, w.value)
                })
                .into_any_element(),

            MiniAppWidgetType::MetricCard => div()
                .w_full()
                .p(px(10.0))
                .rounded(px(10.0))
                .bg(rgb(pal.card_bg))
                .border_1()
                .border_color(rgb(pal.border_fg))
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .text_size(px(11.5))
                                .text_color(rgb(pal.text_secondary))
                                .child(w.label.clone()),
                        )
                        .when_some(w.subtitle.clone(), |col, sub| {
                            col.child(
                                div()
                                    .text_size(px(10.5))
                                    .text_color(rgb(pal.msg_in_date_fg))
                                    .child(sub),
                            )
                        }),
                )
                .child(
                    div()
                        .text_size(px(16.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.accent))
                        .child(w.value.clone()),
                )
                .into_any_element(),

            MiniAppWidgetType::Text => div()
                .w_full()
                .text_size(px(12.5))
                .text_color(rgb(pal.text_primary))
                .child(if w.value.is_empty() {
                    w.label.clone()
                } else if w.label.is_empty() {
                    w.value.clone()
                } else {
                    format!("{}: {}", w.label, w.value)
                })
                .into_any_element(),

            MiniAppWidgetType::TextInput => {
                let wid_click = wid.clone();
                let placeholder = w
                    .placeholder
                    .clone()
                    .unwrap_or_else(|| "Click to type…".to_string());
                let is_empty = w.value.is_empty();
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .when(!w.label.is_empty(), |col| {
                        col.child(
                            div()
                                .text_size(px(11.5))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(pal.text_secondary))
                                .child(w.label.clone()),
                        )
                    })
                    .child(
                        div()
                            .id(SharedString::from(format!("miniapp-widget-{}", wid)))
                            .cursor_text()
                            .w_full()
                            .min_h(px(36.0))
                            .px(px(10.0))
                            .py(px(8.0))
                            .rounded(px(8.0))
                            .bg(rgb(pal.input_bg))
                            .border_1()
                            .border_color(rgb(if is_focused {
                                pal.accent
                            } else {
                                pal.border_fg
                            }))
                            .text_size(px(12.5))
                            .text_color(rgb(if is_empty {
                                pal.text_secondary
                            } else {
                                pal.text_primary
                            }))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.focused_input =
                                        Some(BotWebViewInputTarget::WidgetText(wid_click.clone()));
                                    cx.notify();
                                }),
                            )
                            .child(if is_empty { placeholder } else { w.value.clone() }),
                    )
                    .into_any_element()
            }

            MiniAppWidgetType::Select => {
                let current = w.value.clone();
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap(px(5.0))
                    .when(!w.label.is_empty(), |col| {
                        col.child(
                            div()
                                .text_size(px(11.5))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(pal.text_secondary))
                                .child(w.label.clone()),
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap(px(6.0))
                            .children(w.options.iter().cloned().map(|opt| {
                                let selected = current == opt;
                                let wid_tap = wid.clone();
                                let opt_val = opt.clone();
                                div()
                                    .id(SharedString::from(format!(
                                        "miniapp-select-{}-{}",
                                        wid, opt
                                    )))
                                    .cursor_pointer()
                                    .px(px(10.0))
                                    .py(px(5.0))
                                    .rounded(px(999.0))
                                    .bg(rgb(if selected { pal.accent } else { pal.card_bg }))
                                    .border_1()
                                    .border_color(rgb(if selected {
                                        pal.accent
                                    } else {
                                        pal.border_fg
                                    }))
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(if selected {
                                        0xFFFFFF
                                    } else {
                                        pal.text_primary
                                    }))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            if let Some(target) = this
                                                .ui_schema
                                                .widgets
                                                .iter_mut()
                                                .find(|item| item.id == wid_tap)
                                            {
                                                target.value = opt_val.clone();
                                            }
                                            this.queue_widget_event(
                                                wid_tap.clone(),
                                                serde_json::json!(opt_val),
                                            );
                                            cx.notify();
                                        }),
                                    )
                                    .child(opt)
                            })),
                    )
                    .into_any_element()
            }

            MiniAppWidgetType::Toggle => {
                let checked = w.checked;
                let wid_tap = wid.clone();
                div()
                    .id(SharedString::from(format!("miniapp-toggle-{}", wid)))
                    .cursor_pointer()
                    .w_full()
                    .p(px(10.0))
                    .rounded(px(10.0))
                    .bg(rgb(pal.card_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .items_center()
                    .justify_between()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            let next = !checked;
                            if let Some(target) = this
                                .ui_schema
                                .widgets
                                .iter_mut()
                                .find(|item| item.id == wid_tap)
                            {
                                target.checked = next;
                            }
                            this.queue_widget_event(wid_tap.clone(), serde_json::json!(next));
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(pal.text_primary))
                                    .child(w.label.clone()),
                            )
                            .when_some(w.subtitle.clone(), |col, sub| {
                                col.child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(rgb(pal.text_secondary))
                                        .child(sub),
                                )
                            }),
                    )
                    .child(
                        div()
                            .w(px(36.0))
                            .h(px(20.0))
                            .rounded_full()
                            .p(px(2.0))
                            .bg(rgb(if checked {
                                pal.accent
                            } else {
                                pal.muted_badge_bg
                            }))
                            .flex()
                            .items_center()
                            .when(checked, |el| el.justify_end())
                            .when(!checked, |el| el.justify_start())
                            .child(div().w(px(16.0)).h(px(16.0)).rounded_full().bg(rgb(0xFFFFFF))),
                    )
                    .into_any_element()
            }

            MiniAppWidgetType::Counter => {
                let count = w.count;
                let min_v = w.min.unwrap_or(0);
                let max_v = w.max.unwrap_or(999);
                let wid_dec = wid.clone();
                let wid_inc = wid.clone();
                div()
                    .w_full()
                    .p(px(10.0))
                    .rounded(px(10.0))
                    .bg(rgb(pal.card_bg))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgb(pal.text_primary))
                                    .child(w.label.clone()),
                            )
                            .when_some(w.subtitle.clone(), |col, sub| {
                                col.child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(rgb(pal.text_secondary))
                                        .child(sub),
                                )
                            }),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id(SharedString::from(format!("miniapp-counter-dec-{}", wid)))
                                    .cursor_pointer()
                                    .w(px(26.0))
                                    .h(px(26.0))
                                    .rounded(px(7.0))
                                    .bg(rgb(pal.input_bg))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            let next = (count - 1).max(min_v);
                                            if let Some(target) = this
                                                .ui_schema
                                                .widgets
                                                .iter_mut()
                                                .find(|item| item.id == wid_dec)
                                            {
                                                target.count = next;
                                            }
                                            this.queue_widget_event(
                                                wid_dec.clone(),
                                                serde_json::json!(next),
                                            );
                                            cx.notify();
                                        }),
                                    )
                                    .child("-"),
                            )
                            .child(
                                div()
                                    .min_w(px(24.0))
                                    .flex()
                                    .justify_center()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(count.to_string()),
                            )
                            .child(
                                div()
                                    .id(SharedString::from(format!("miniapp-counter-inc-{}", wid)))
                                    .cursor_pointer()
                                    .w(px(26.0))
                                    .h(px(26.0))
                                    .rounded(px(7.0))
                                    .bg(rgb(pal.accent))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            let next = (count + 1).min(max_v);
                                            if let Some(target) = this
                                                .ui_schema
                                                .widgets
                                                .iter_mut()
                                                .find(|item| item.id == wid_inc)
                                            {
                                                target.count = next;
                                            }
                                            this.queue_widget_event(
                                                wid_inc.clone(),
                                                serde_json::json!(next),
                                            );
                                            cx.notify();
                                        }),
                                    )
                                    .child("+"),
                            ),
                    )
                    .into_any_element()
            }

            MiniAppWidgetType::ListItem => {
                let wid_tap = wid.clone();
                let val = w.value.clone();
                div()
                    .id(SharedString::from(format!("miniapp-list-{}", wid)))
                    .cursor_pointer()
                    .w_full()
                    .p(px(10.0))
                    .rounded(px(10.0))
                    .bg(rgb(pal.card_bg))
                    .hover(|s| s.bg(rgb(pal.dialogs_hover_bg)))
                    .border_1()
                    .border_color(rgb(pal.border_fg))
                    .flex()
                    .items_center()
                    .justify_between()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.queue_widget_event(wid_tap.clone(), serde_json::json!(val));
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_size(px(12.5))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(pal.text_primary))
                                    .child(w.label.clone()),
                            )
                            .when_some(w.subtitle.clone(), |col, sub| {
                                col.child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(rgb(pal.text_secondary))
                                        .child(sub),
                                )
                            }),
                    )
                    .when(!w.value.is_empty(), |row| {
                        row.child(
                            div()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(pal.accent))
                                .child(w.value.clone()),
                        )
                    })
                    .into_any_element()
            }

            MiniAppWidgetType::Button => {
                let wid_tap = wid.clone();
                let action_val = w
                    .action_id
                    .clone()
                    .unwrap_or_else(|| w.label.clone());
                let is_danger = w.variant.as_deref() == Some("danger");
                div()
                    .id(SharedString::from(format!("miniapp-btn-{}", wid)))
                    .cursor_pointer()
                    .w_full()
                    .h(px(36.0))
                    .rounded(px(9.0))
                    .bg(rgb(if is_danger {
                        pal.danger
                    } else {
                        pal.composer_bg
                    }))
                    .border_1()
                    .border_color(rgb(if is_danger { pal.danger } else { pal.accent }))
                    .hover(|s| s.bg(rgb(pal.dialogs_hover_bg)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(12.5))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(if is_danger {
                        0xFFFFFF
                    } else {
                        pal.text_link
                    }))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.queue_widget_event(
                                wid_tap.clone(),
                                serde_json::json!(action_val.clone()),
                            );
                            cx.notify();
                        }),
                    )
                    .child(w.label.clone())
                    .into_any_element()
            }
        }
    }
}
