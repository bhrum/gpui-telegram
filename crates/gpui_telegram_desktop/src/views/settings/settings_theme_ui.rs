//! Shared theme-reactive, localized, SVG-iconed GPUI widgets for all Telegram Settings views.

use crate::theme::assets::{IconName, tg_icon};
use crate::theme::palette::TdesktopThemePalette;
use gpui::prelude::FluentBuilder;
use gpui::*;

pub fn settings_container(pal: TdesktopThemePalette) -> Div {
    div()
        .w_full()
        .min_h_full()
        .p(px(pal.scaled_px(20.0)))
        .bg(rgb(pal.window_bg))
        .flex()
        .flex_col()
        .gap(px(pal.scaled_px(12.0)))
}

pub fn settings_card(pal: TdesktopThemePalette) -> Div {
    div()
        .w_full()
        .rounded(px(pal.scaled_px(12.0)))
        .bg(rgb(pal.card_bg))
        .border_1()
        .border_color(rgb(pal.border_fg))
        .p(px(pal.scaled_px(14.0)))
        .flex()
        .flex_col()
        .gap(px(pal.scaled_px(10.0)))
}

pub fn settings_section_header(
    icon: IconName,
    title: &str,
    subtitle: Option<&str>,
    pal: TdesktopThemePalette,
) -> AnyElement {
    div()
        .flex()
        .items_center()
        .gap(px(pal.scaled_px(10.0)))
        .pb(px(pal.scaled_px(4.0)))
        .child(
            div()
                .w(px(pal.scaled_px(34.0)))
                .h(px(pal.scaled_px(34.0)))
                .rounded_full()
                .bg(TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.16))
                .flex()
                .items_center()
                .justify_center()
                .child(tg_icon(icon, pal.scaled_px(18.0), pal.accent)),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_size(px(pal.scaled_px(16.0)))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .child(title.to_string()),
                )
                .when_some(subtitle, |el, sub| {
                    el.child(
                        div()
                            .text_size(px(pal.scaled_px(12.0)))
                            .text_color(rgb(pal.text_secondary))
                            .child(sub.to_string()),
                    )
                }),
        )
        .into_any_element()
}

pub fn settings_subsection_title(title: &str, pal: TdesktopThemePalette) -> AnyElement {
    div()
        .text_size(px(pal.scaled_px(12.5)))
        .font_weight(FontWeight::BOLD)
        .text_color(rgb(pal.text_link))
        .child(title.to_string())
        .into_any_element()
}

pub fn settings_toggle_row<V: 'static>(
    icon: IconName,
    title: &str,
    subtitle: Option<&str>,
    active: bool,
    pal: TdesktopThemePalette,
    on_toggle: impl Fn(&mut V) + 'static,
    cx: &mut Context<V>,
) -> AnyElement {
    div()
        .cursor_pointer()
        .w_full()
        .px(px(pal.scaled_px(10.0)))
        .py(px(pal.scaled_px(8.0)))
        .rounded(px(pal.scaled_px(8.0)))
        .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
        .flex()
        .items_center()
        .justify_between()
        .gap(px(pal.scaled_px(10.0)))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _e, _w, cx| {
                on_toggle(this);
                cx.notify();
            }),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(pal.scaled_px(10.0)))
                .child(tg_icon(icon, pal.scaled_px(16.0), pal.icon_fg))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_size(px(pal.scaled_px(13.5)))
                                .text_color(rgb(pal.text_primary))
                                .child(title.to_string()),
                        )
                        .when_some(subtitle, |el, sub| {
                            el.child(
                                div()
                                    .text_size(px(pal.scaled_px(11.5)))
                                    .text_color(rgb(pal.text_secondary))
                                    .child(sub.to_string()),
                            )
                        }),
                ),
        )
        .child(
            div()
                .w(px(pal.scaled_px(36.0)))
                .h(px(pal.scaled_px(20.0)))
                .rounded_full()
                .bg(rgb(if active { pal.accent } else { pal.muted_badge_bg }))
                .p(px(pal.scaled_px(2.0)))
                .flex()
                .items_center()
                .when(active, |el| el.justify_end())
                .when(!active, |el| el.justify_start())
                .child(
                    div()
                        .w(px(pal.scaled_px(16.0)))
                        .h(px(pal.scaled_px(16.0)))
                        .rounded_full()
                        .bg(rgb(pal.active_button_fg)),
                ),
        )
        .into_any_element()
}

pub fn settings_action_row<V: 'static>(
    icon: IconName,
    title: &str,
    value_text: &str,
    pal: TdesktopThemePalette,
    danger: bool,
    on_tap: impl Fn(&mut V) + 'static,
    cx: &mut Context<V>,
) -> AnyElement {
    let fg = if danger { pal.danger } else { pal.text_primary };
    div()
        .cursor_pointer()
        .w_full()
        .px(px(pal.scaled_px(10.0)))
        .py(px(pal.scaled_px(8.0)))
        .rounded(px(pal.scaled_px(8.0)))
        .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
        .flex()
        .items_center()
        .justify_between()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _e, _w, cx| {
                on_tap(this);
                cx.notify();
            }),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(pal.scaled_px(10.0)))
                .child(tg_icon(icon, pal.scaled_px(16.0), if danger { pal.danger } else { pal.icon_fg }))
                .child(div().text_size(px(pal.scaled_px(13.5))).text_color(rgb(fg)).child(title.to_string())),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(pal.scaled_px(6.0)))
                .child(
                    div()
                        .text_size(px(pal.scaled_px(12.5)))
                        .text_color(rgb(if danger { pal.danger } else { pal.text_link }))
                        .child(value_text.to_string()),
                )
                .child(tg_icon(IconName::ChevronRight, pal.scaled_px(13.0), pal.icon_fg)),
        )
        .into_any_element()
}

pub fn settings_info_row(label: &str, value: &str, pal: TdesktopThemePalette) -> AnyElement {
    div()
        .w_full()
        .px(px(pal.scaled_px(10.0)))
        .py(px(pal.scaled_px(7.0)))
        .rounded(px(pal.scaled_px(8.0)))
        .bg(rgb(pal.input_bg))
        .flex()
        .items_center()
        .justify_between()
        .child(div().text_size(px(pal.scaled_px(12.5))).text_color(rgb(pal.text_secondary)).child(label.to_string()))
        .child(div().text_size(px(pal.scaled_px(12.5))).font_weight(FontWeight::SEMIBOLD).text_color(rgb(pal.text_primary)).child(value.to_string()))
        .into_any_element()
}

pub fn settings_empty_state(icon: IconName, text: &str, pal: TdesktopThemePalette) -> AnyElement {
    div()
        .py(px(pal.scaled_px(20.0)))
        .flex()
        .flex_col()
        .items_center()
        .gap(px(pal.scaled_px(6.0)))
        .child(tg_icon(icon, pal.scaled_px(28.0), pal.text_secondary))
        .child(div().text_size(px(pal.scaled_px(12.5))).text_color(rgb(pal.text_secondary)).child(text.to_string()))
        .into_any_element()
}
