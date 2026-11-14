use crate::i18n::tr_str;
use crate::theme::{assets::{IconName, tg_icon}, TdesktopThemePalette};
use crate::views::boxes::modal_types::ActiveTextField;
use crate::views::modal_boxes_suite::ActiveModalDialog;
use gpui::*;

pub type DialogMut<V> = fn(&mut V) -> &mut ActiveModalDialog;
pub type DialogClose<V> = fn(&mut V);

pub fn modal_card(width: f32, pal: TdesktopThemePalette) -> Div {
    div()
        .w(px(width))
        .max_h(px(620.0))
        .rounded(px(14.0))
        .bg(rgb(pal.modal_bg))
        .border_1()
        .border_color(rgb(pal.border_fg))
        .p(px(20.0))
        .shadow_lg()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .on_mouse_down(MouseButton::Left, |_e, _w, cx| {
            cx.stop_propagation();
        })
}

pub fn modal_header<V: 'static>(
    icon: IconName,
    title: &str,
    pal: TdesktopThemePalette,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> impl IntoElement {
    div()
        .w_full()
        .flex()
        .items_center()
        .justify_between()
        .pb(px(8.0))
        .border_b_1()
        .border_color(rgb(pal.border_fg))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(tg_icon(icon, 18.0, pal.accent))
                .child(
                    div()
                        .text_size(px(16.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_primary))
                        .child(title.to_string()),
                ),
        )
        .child(
            div()
                .cursor_pointer()
                .w(px(28.0))
                .h(px(28.0))
                .rounded_full()
                .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
                .flex()
                .items_center()
                .justify_center()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _e, _w, cx| {
                        on_close(this);
                        cx.notify();
                    }),
                )
                .child(tg_icon(IconName::Close, 15.0, pal.icon_fg)),
        )
}

pub fn modal_field<V: 'static>(
    label: &str,
    value: &str,
    placeholder: &str,
    field: ActiveTextField,
    is_focused: bool,
    pal: TdesktopThemePalette,
    access: DialogMut<V>,
    cx: &mut Context<V>,
) -> impl IntoElement {
    let display = if value.is_empty() {
        placeholder.to_string()
    } else {
        value.to_string()
    };
    let text_col = if value.is_empty() {
        pal.text_secondary
    } else {
        pal.text_primary
    };

    div()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .child(
            div()
                .text_size(px(12.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(pal.text_link))
                .child(label.to_string()),
        )
        .child(
            div()
                .cursor_pointer()
                .w_full()
                .px(px(10.0))
                .py(px(8.0))
                .rounded(px(8.0))
                .bg(rgb(pal.input_bg))
                .border_1()
                .border_color(rgb(if is_focused { pal.accent } else { pal.border_fg }))
                .text_size(px(13.0))
                .text_color(rgb(text_col))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _e, _w, cx| {
                        access(this).active_field = field;
                        cx.notify();
                    }),
                )
                .child(display),
        )
}

pub fn modal_error_banner(err: Option<&String>, pal: TdesktopThemePalette) -> AnyElement {
    match err {
        Some(msg) => div()
            .w_full()
            .px(px(10.0))
            .py(px(7.0))
            .rounded(px(8.0))
            .bg(TdesktopThemePalette::rgba_with_alpha(pal.danger, 0.16))
            .text_size(px(12.0))
            .text_color(rgb(pal.danger))
            .child(msg.clone())
            .into_any_element(),
        None => div().into_any_element(),
    }
}

pub fn modal_footer_cancel<V: 'static>(
    lang: &str,
    pal: TdesktopThemePalette,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> impl IntoElement {
    div()
        .cursor_pointer()
        .px(px(14.0))
        .py(px(7.0))
        .rounded(px(8.0))
        .bg(rgb(pal.input_bg))
        .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
        .text_size(px(13.0))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(rgb(pal.text_secondary))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _e, _w, cx| {
                on_close(this);
                cx.notify();
            }),
        )
        .child(tr_str(lang, "取消", "Cancel").to_string())
}

pub fn modal_toggle_chip<V: 'static>(
    label: &str,
    active: bool,
    pal: TdesktopThemePalette,
    on_click: impl Fn(&mut V) + 'static,
    cx: &mut Context<V>,
) -> impl IntoElement {
    div()
        .cursor_pointer()
        .px(px(10.0))
        .py(px(6.0))
        .rounded(px(8.0))
        .bg(if active {
            TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.22)
        } else {
            rgb(pal.input_bg)
        })
        .border_1()
        .border_color(rgb(if active { pal.accent } else { pal.border_fg }))
        .flex()
        .items_center()
        .gap(px(6.0))
        .text_size(px(12.0))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(rgb(if active { pal.text_primary } else { pal.text_secondary }))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _e, _w, cx| {
                on_click(this);
                cx.notify();
            }),
        )
        .child(tg_icon(
            IconName::SelectCheck,
            13.0,
            if active { pal.accent } else { pal.text_secondary },
        ))
        .child(label.to_string())
}
