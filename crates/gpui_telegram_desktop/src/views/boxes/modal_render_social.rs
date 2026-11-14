use crate::i18n::tr_str;
use crate::theme::assets::{IconName, tg_icon};
use crate::views::boxes::add_contact_box::{AddContactBox, ContactValidationError};
use crate::views::boxes::modal_theme::{
    modal_card, modal_error_banner, modal_field, modal_footer_cancel, modal_header, DialogClose,
    DialogMut,
};
use crate::views::boxes::modal_types::{ActiveTextField, ModalActionOutcome, ModalDialogKind};
use crate::views::modal_boxes_suite::ActiveModalDialog;
use gpui::prelude::FluentBuilder;
use gpui::*;

pub fn render_contacts_box<V: 'static>(
    dlg: &ActiveModalDialog,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let q = dlg.primary_input.to_lowercase();
    let list: Vec<_> = dlg
        .contacts_list
        .iter()
        .filter(|(n, u, _, _, _)| {
            q.is_empty() || n.to_lowercase().contains(&q) || u.to_lowercase().contains(&q)
        })
        .cloned()
        .collect();

    modal_card(420.0, pal)
        .child(modal_header(
            IconName::ContactsUsers,
            tr_str(lang, "联系人", "Contacts"),
            pal,
            on_close,
            cx,
        ))
        .child(modal_field(
            tr_str(lang, "搜索联系人", "Search"),
            &dlg.primary_input,
            tr_str(lang, "输入姓名或用户名搜索...", "Search contacts..."),
            ActiveTextField::Primary,
            dlg.active_field == ActiveTextField::Primary,
            pal,
            access,
            cx,
        ))
        .map(|el| {
            if list.is_empty() {
                el.child(
                    div()
                        .py(px(20.0))
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(6.0))
                        .child(tg_icon(IconName::ContactsUsers, 32.0, pal.text_secondary))
                        .child(
                            div()
                                .text_size(px(13.0))
                                .text_color(rgb(pal.text_secondary))
                                .child(tr_str(lang, "暂无联系人", "No contacts yet")),
                        ),
                )
            } else {
                el.child(
                    div()
                        .id("contacts-modal-scroll")
                        .max_h(px(260.0))
                        .overflow_y_scroll()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .children(list.into_iter().map(|(name, uname, init, col, online)| {
                            let nm = name.clone();
                            let ini = init.clone();
                            div()
                                .cursor_pointer()
                                .h(px(48.0))
                                .px(px(10.0))
                                .rounded(px(8.0))
                                .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
                                .flex()
                                .items_center()
                                .justify_between()
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _e, _w, cx| {
                                        access(this).pending_outcome =
                                            Some(ModalActionOutcome::OpenContactChat {
                                                name: nm.clone(),
                                                initials: ini.clone(),
                                                color: col,
                                            });
                                        on_close(this);
                                        cx.notify();
                                    }),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(px(10.0))
                                        .child(
                                            div()
                                                .w(px(36.0))
                                                .h(px(36.0))
                                                .rounded_full()
                                                .bg(rgb(col))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .text_size(px(12.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(pal.active_button_fg))
                                                .child(init),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .child(div().text_size(px(13.5)).text_color(rgb(pal.text_primary)).child(name))
                                                .child(div().text_size(px(11.5)).text_color(rgb(pal.text_secondary)).child(uname)),
                                        ),
                                )
                                .child(div().text_size(px(11.0)).text_color(rgb(if online {
                                    pal.online_dot_fg
                                } else {
                                    pal.text_secondary
                                })).child(if online { "online" } else { "" }))
                        })),
                )
            }
        })
        .child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .child(
                    div()
                        .cursor_pointer()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(pal.text_link))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                let targets = access(this).forward_targets.clone();
                                access(this).open(ModalDialogKind::AddContact, targets);
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "+ 添加联系人", "+ Add Contact").to_string()),
                )
                .child(modal_footer_cancel(lang, pal, on_close, cx)),
        )
        .into_any_element()
}

pub fn render_add_contact<V: 'static>(
    dlg: &ActiveModalDialog,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;

    modal_card(400.0, pal)
        .child(modal_header(
            IconName::ContactsUsers,
            tr_str(lang, "添加联系人", "Add Contact"),
            pal,
            on_close,
            cx,
        ))
        .child(modal_field(
            tr_str(lang, "名字 (必填)", "First Name"),
            &dlg.primary_input,
            "Alice",
            ActiveTextField::Primary,
            dlg.active_field == ActiveTextField::Primary,
            pal,
            access,
            cx,
        ))
        .child(modal_field(
            tr_str(lang, "姓氏 (可选)", "Last Name"),
            &dlg.secondary_input,
            "Smith",
            ActiveTextField::Secondary,
            dlg.active_field == ActiveTextField::Secondary,
            pal,
            access,
            cx,
        ))
        .child(modal_field(
            tr_str(lang, "手机号码", "Phone Number"),
            &dlg.tertiary_input,
            "+1 555 010 9988",
            ActiveTextField::Tertiary,
            dlg.active_field == ActiveTextField::Tertiary,
            pal,
            access,
            cx,
        ))
        .child(modal_error_banner(dlg.validation_error.as_ref(), pal))
        .child(
            div()
                .flex()
                .justify_end()
                .gap(px(8.0))
                .child(modal_footer_cancel(lang, pal, on_close, cx))
                .child(
                    div()
                        .cursor_pointer()
                        .px(px(16.0))
                        .py(px(7.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.accent))
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.active_button_fg))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                let d = access(this);
                                let mut validator = AddContactBox::new();
                                validator.set_fields(&d.primary_input, &d.secondary_input, &d.tertiary_input);
                                if !validator.validate() {
                                    d.validation_error = Some(match validator.error {
                                        Some(ContactValidationError::EmptyFirstName) => "First name is required".into(),
                                        Some(ContactValidationError::InvalidPhoneNumber) => "Enter a valid phone (7-15 digits)".into(),
                                        _ => "Invalid contact fields".into(),
                                    });
                                    cx.notify();
                                    return;
                                }
                                d.pending_outcome = Some(ModalActionOutcome::AddContactSubmitted {
                                    first_name: validator.first_name,
                                    last_name: validator.last_name,
                                    phone: validator.phone,
                                });
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "保存联系人", "Create").to_string()),
                ),
        )
        .into_any_element()
}

pub fn render_calls_history<V: 'static>(
    dlg: &ActiveModalDialog,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let missed_only = dlg.calls_missed_only;
    let logs: Vec<_> = dlg
        .call_logs
        .iter()
        .filter(|c| !missed_only || c.is_missed)
        .cloned()
        .collect();

    modal_card(420.0, pal)
        .child(modal_header(
            IconName::PhoneCall,
            tr_str(lang, "通话记录", "Recent Calls"),
            pal,
            on_close,
            cx,
        ))
        .child(
            div()
                .flex()
                .gap(px(8.0))
                .child(
                    div()
                        .cursor_pointer()
                        .px(px(12.0))
                        .py(px(5.0))
                        .rounded(px(6.0))
                        .bg(rgb(if !missed_only { pal.accent } else { pal.input_bg }))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(if !missed_only { pal.active_button_fg } else { pal.text_primary }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                access(this).calls_missed_only = false;
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "全部", "All").to_string()),
                )
                .child(
                    div()
                        .cursor_pointer()
                        .px(px(12.0))
                        .py(px(5.0))
                        .rounded(px(6.0))
                        .bg(rgb(if missed_only { pal.danger } else { pal.input_bg }))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(if missed_only { pal.active_button_fg } else { pal.text_primary }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                access(this).calls_missed_only = true;
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "未接来电", "Missed").to_string()),
                ),
        )
        .map(|el| {
            if logs.is_empty() {
                el.child(
                    div()
                        .py(px(24.0))
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(6.0))
                        .child(tg_icon(IconName::PhoneCall, 32.0, pal.text_secondary))
                        .child(
                            div()
                                .text_size(px(13.0))
                                .text_color(rgb(pal.text_secondary))
                                .child(tr_str(lang, "暂无通话记录", "No recent calls")),
                        ),
                )
            } else {
                el.children(logs.into_iter().map(|entry| {
                    let peer = entry.peer_name.clone();
                    let vid = entry.is_video;
                    div()
                        .cursor_pointer()
                        .p(px(10.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.input_bg))
                        .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
                        .flex()
                        .items_center()
                        .justify_between()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                access(this).pending_outcome = Some(ModalActionOutcome::StartCallWithPeer {
                                    peer_name: peer.clone(),
                                    is_video: vid,
                                });
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .child(div().text_size(px(13.5)).text_color(rgb(if entry.is_missed { pal.danger } else { pal.text_primary })).child(entry.peer_name))
                                .child(div().text_size(px(11.5)).text_color(rgb(pal.text_secondary)).child(format!("{} • {}", entry.description, entry.timestamp))),
                        )
                        .child(tg_icon(if entry.is_video { IconName::VideoCall } else { IconName::PhoneCall }, 16.0, pal.accent))
                }))
            }
        })
        .into_any_element()
}
