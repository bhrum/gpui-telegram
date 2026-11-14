//! Hover Quick-Reactions Pill & Full Right-Click Message Context Menu.

use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::i18n::tr_key_or;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};

#[derive(Debug, Clone)]
pub enum ContextMenuAction {
    React(u64, String),
    Reply(u64),
    Edit(u64),
    TogglePin(u64),
    CopyText(u64),
    Forward(u64),
    Translate(u64),
    Select(u64),
    Delete(u64),
    CloseMenu,
}

const QUICK_REACTIONS: &[&str] = &["👍", "❤️", "🔥", "🎉", "😁", "😢", "👏"];

pub fn render_hover_reaction_pill<V: 'static>(
    msg_id: u64,
    is_outgoing: bool,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, ContextMenuAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let mut row = div()
        .flex()
        .items_center()
        .gap(px(2.0))
        .px(px(6.0))
        .py(px(3.0))
        .rounded_full()
        .bg(rgb(palette.menu_bg))
        .border_1()
        .border_color(rgb(palette.border_fg))
        .shadow_sm();

    for (idx, &emoji) in QUICK_REACTIONS[..5].iter().enumerate() {
        let em = emoji.to_string();
        let act = on_action.clone();
        row = row.child(
            div()
                .id(SharedString::from(format!("qreact-{}-{}", msg_id, idx)))
                .w(px(24.0))
                .h(px(24.0))
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                .on_click(cx.listener(move |this, _, _, cx| {
                    act(this, ContextMenuAction::React(msg_id, em.clone()), cx);
                }))
                .text_size(px(14.0))
                .child(emoji),
        );
    }

    let container = div()
        .flex()
        .when(is_outgoing, |el| el.justify_end())
        .mb(px(2.0))
        .child(row);

    container.into_any_element()
}

pub fn render_message_context_menu<V: 'static>(
    msg_id: u64,
    pos: Point<Pixels>,
    is_outgoing: bool,
    is_pinned: bool,
    can_edit: bool,
    is_translated: bool,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, ContextMenuAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let act_bg = on_action.clone();
    let act_react = on_action.clone();
    let menu_x = f32::from(pos.x).max(16.0).min(680.0);
    let menu_y = f32::from(pos.y).max(60.0).min(460.0);

    let mut menu_box = div()
        .id("msg-context-menu-popup")
        .absolute()
        .left(px(menu_x))
        .top(px(menu_y))
        .w(px(224.0))
        .p(px(6.0))
        .rounded(px(10.0))
        .bg(rgb(palette.menu_bg))
        .border_1()
        .border_color(rgb(palette.border_fg))
        .shadow_lg()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation());

    // Quick-reactions top strip
    let mut rx_row = div()
        .flex()
        .items_center()
        .justify_between()
        .px(px(4.0))
        .py(px(4.0))
        .mb(px(4.0))
        .border_b_1()
        .border_color(rgb(palette.border_fg));

    for (idx, &emoji) in QUICK_REACTIONS.iter().enumerate() {
        let em = emoji.to_string();
        let a = act_react.clone();
        rx_row = rx_row.child(
            div()
                .id(SharedString::from(format!("ctx-rx-{}-{}", msg_id, idx)))
                .w(px(26.0))
                .h(px(26.0))
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                .on_click(cx.listener(move |this, _, _, cx| {
                    a(this, ContextMenuAction::React(msg_id, em.clone()), cx);
                }))
                .text_size(px(15.0))
                .child(emoji),
        );
    }

    menu_box = menu_box
        .child(rx_row)
        .child(ctx_menu_item("ctx-reply", IconName::Reply, tr_key_or(lang_id, "lng_context_reply_msg", "Reply"), false, palette, cx, {
            let a = on_action.clone();
            move |this, cx| a(this, ContextMenuAction::Reply(msg_id), cx)
        }))
        .when(is_outgoing && can_edit, |el| {
            let a = on_action.clone();
            el.child(ctx_menu_item("ctx-edit", IconName::Edit, tr_key_or(lang_id, "lng_context_edit_msg", "Edit"), false, palette, cx, move |this, cx| {
                a(this, ContextMenuAction::Edit(msg_id), cx)
            }))
        })
        .child(ctx_menu_item(
            "ctx-pin",
            IconName::Pin,
            if is_pinned {
                tr_key_or(lang_id, "lng_context_unpin_msg", "Unpin")
            } else {
                tr_key_or(lang_id, "lng_context_pin_msg", "Pin")
            },
            false,
            palette,
            cx,
            {
                let a = on_action.clone();
                move |this, cx| a(this, ContextMenuAction::TogglePin(msg_id), cx)
            },
        ))
        .child(ctx_menu_item("ctx-copy", IconName::Copy, tr_key_or(lang_id, "lng_context_copy_text", "Copy Text"), false, palette, cx, {
            let a = on_action.clone();
            move |this, cx| a(this, ContextMenuAction::CopyText(msg_id), cx)
        }))
        .child(ctx_menu_item("ctx-forward", IconName::Forward, tr_key_or(lang_id, "lng_context_forward_msg", "Forward"), false, palette, cx, {
            let a = on_action.clone();
            move |this, cx| a(this, ContextMenuAction::Forward(msg_id), cx)
        }))
        .child(ctx_menu_item(
            "ctx-translate",
            IconName::Translate,
            if is_translated { "Show Original" } else { tr_key_or(lang_id, "lng_context_translate", "Translate") },
            false,
            palette,
            cx,
            {
                let a = on_action.clone();
                move |this, cx| a(this, ContextMenuAction::Translate(msg_id), cx)
            },
        ))
        .child(ctx_menu_item("ctx-select", IconName::SelectCheck, tr_key_or(lang_id, "lng_context_select_msg", "Select"), false, palette, cx, {
            let a = on_action.clone();
            move |this, cx| a(this, ContextMenuAction::Select(msg_id), cx)
        }))
        .child(div().my(px(2.0)).h(px(1.0)).w_full().bg(rgb(palette.border_fg)))
        .child(ctx_menu_item("ctx-delete", IconName::DeleteTrash, tr_key_or(lang_id, "lng_context_delete_msg", "Delete"), true, palette, cx, {
            let a = on_action.clone();
            move |this, cx| a(this, ContextMenuAction::Delete(msg_id), cx)
        }));

    div()
        .id("msg-ctx-backdrop")
        .absolute()
        .inset_0()
        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
            act_bg(this, ContextMenuAction::CloseMenu, cx);
        }))
        .child(menu_box)
        .into_any_element()
}

fn ctx_menu_item<V: 'static>(
    id: &'static str,
    icon: IconName,
    label: &str,
    danger: bool,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_click: impl Fn(&mut V, &mut Context<V>) + 'static,
) -> Stateful<Div> {
    let fg = if danger { palette.danger } else { palette.text_primary };
    let icon_fg = if danger { palette.danger } else { palette.icon_fg };
    let hover_bg = palette.menu_hover_bg;
    div()
        .id(id)
        .px(px(10.0))
        .py(px(6.0))
        .rounded(px(6.0))
        .flex()
        .items_center()
        .gap(px(10.0))
        .cursor_pointer()
        .hover(move |s| s.bg(rgb(hover_bg)))
        .on_click(cx.listener(move |this, _, _, cx| on_click(this, cx)))
        .child(tg_icon(icon, 15.0, icon_fg))
        .child(div().text_size(px(13.0)).text_color(rgb(fg)).child(label.to_string()))
}
