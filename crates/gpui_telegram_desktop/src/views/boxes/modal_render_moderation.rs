use crate::i18n::tr_str;
use crate::theme::assets::{IconName, tg_icon};
use crate::theme::palette::TdesktopThemePalette;
use crate::views::boxes::delete_messages_box::DeleteMessagesBox;
use crate::views::boxes::modal_theme::{
    modal_card, modal_error_banner, modal_field, modal_footer_cancel, modal_header,
    modal_toggle_chip, DialogClose, DialogMut,
};
use crate::views::boxes::modal_types::{ActiveTextField, ModalActionOutcome};
use crate::views::boxes::pin_messages_box::{PinChatType, PinMessagesBox};
use crate::views::boxes::report_messages_box::{ReportMessagesBox, ReportReason};
use crate::views::modal_boxes_suite::ActiveModalDialog;
use gpui::*;

pub fn render_delete_messages<V: 'static>(
    dlg: &ActiveModalDialog,
    message_ids: Vec<u64>,
    peer_name: String,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let box_model = DeleteMessagesBox::new_selected(
        message_ids.iter().map(|&id| id as i64).collect(),
        peer_name.clone(),
    );
    let count = box_model.selected_count().max(1);
    let revoke = dlg.flag_a;

    modal_card(390.0, pal)
        .child(modal_header(
            IconName::DeleteTrash,
            tr_str(lang, "删除消息", "Delete Messages"),
            pal,
            on_close,
            cx,
        ))
        .child(
            div()
                .text_size(px(13.5))
                .text_color(rgb(pal.text_primary))
                .child(format!(
                    "{} {} {}?",
                    tr_str(lang, "确定要删除这", "Are you sure you want to delete"),
                    count,
                    tr_str(lang, "条消息吗", if count == 1 { "message" } else { "messages" })
                )),
        )
        .child(modal_toggle_chip(
            &format!("{} {}", tr_str(lang, "同时为对方删除", "Also delete for"), peer_name),
            revoke,
            pal,
            move |this| {
                let d = access(this);
                d.flag_a = !d.flag_a;
            },
            cx,
        ))
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
                        .bg(rgb(pal.danger))
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.active_button_fg))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                let d = access(this);
                                let ids = message_ids.clone();
                                d.pending_outcome = Some(ModalActionOutcome::ConfirmDeleteMessages {
                                    message_ids: ids,
                                    revoke_for_everyone: d.flag_a,
                                });
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "删除", "Delete").to_string()),
                ),
        )
        .into_any_element()
}

pub fn render_pin_message<V: 'static>(
    dlg: &ActiveModalDialog,
    message_id: u64,
    peer_name: String,
    is_group: bool,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let chat_type = if is_group {
        PinChatType::Group
    } else {
        PinChatType::PrivateUser
    };
    let pin_box = PinMessagesBox::new(message_id, chat_type, peer_name.clone());

    modal_card(390.0, pal)
        .child(modal_header(
            IconName::Pin,
            tr_str(lang, "置顶消息", "Pin Message"),
            pal,
            on_close,
            cx,
        ))
        .child(
            div()
                .text_size(px(13.5))
                .text_color(rgb(pal.text_primary))
                .child(pin_box.confirmation_text()),
        )
        .child(modal_toggle_chip(
            if is_group {
                tr_str(lang, "通知所有成员", "Notify all members")
            } else {
                tr_str(lang, "同时为对方置顶", "Also pin for both")
            },
            if is_group { dlg.flag_a } else { dlg.flag_b },
            pal,
            move |this| {
                let d = access(this);
                if is_group {
                    d.flag_a = !d.flag_a;
                } else {
                    d.flag_b = !d.flag_b;
                }
            },
            cx,
        ))
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
                                d.pending_outcome = Some(ModalActionOutcome::ConfirmPinMessage {
                                    message_id,
                                    notify_members: d.flag_a,
                                    pin_for_both: d.flag_b,
                                });
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "置顶", "Pin").to_string()),
                ),
        )
        .into_any_element()
}

pub fn render_report_message<V: 'static>(
    dlg: &ActiveModalDialog,
    message_ids: Vec<u64>,
    peer_name: String,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let reasons = [
        ReportReason::Spam,
        ReportReason::Violence,
        ReportReason::Pornography,
        ReportReason::ChildAbuse,
        ReportReason::Copyright,
        ReportReason::IllegalDrugs,
        ReportReason::PersonalDetails,
        ReportReason::Other,
    ];
    let selected = dlg.selected_report_reason;

    modal_card(420.0, pal)
        .child(modal_header(
            IconName::MoreVertical,
            &format!("{} {}", tr_str(lang, "举报", "Report"), peer_name),
            pal,
            on_close,
            cx,
        ))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(6.0))
                .children(reasons.into_iter().map(|reason| {
                    let active = selected == Some(reason);
                    div()
                        .cursor_pointer()
                        .px(px(10.0))
                        .py(px(6.0))
                        .rounded(px(8.0))
                        .bg(if active {
                            TdesktopThemePalette::rgba_with_alpha(pal.danger, 0.2)
                        } else {
                            rgb(pal.input_bg)
                        })
                        .border_1()
                        .border_color(rgb(if active { pal.danger } else { pal.border_fg }))
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .text_size(px(12.0))
                        .text_color(rgb(pal.text_primary))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                let d = access(this);
                                d.selected_report_reason = Some(reason);
                                d.validation_error = None;
                                cx.notify();
                            }),
                        )
                        .child(tg_icon(
                            IconName::SelectCheck,
                            12.0,
                            if active { pal.danger } else { pal.text_secondary },
                        ))
                        .child(reason.title())
                })),
        )
        .child(modal_field(
            tr_str(lang, "补充说明", "Additional Comment"),
            &dlg.primary_input,
            tr_str(lang, "选填，如选择“其他”则必填...", "Optional (required for Other)..."),
            ActiveTextField::Primary,
            dlg.active_field == ActiveTextField::Primary,
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
                        .bg(rgb(pal.danger))
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.active_button_fg))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                let d = access(this);
                                let mut rb = ReportMessagesBox::new(0, Vec::new());
                                if let Some(r) = d.selected_report_reason {
                                    rb.select_reason(r);
                                }
                                rb.set_comment(&d.primary_input);
                                if !rb.can_submit() {
                                    d.validation_error = Some("Select a report reason (and comment for Other)".into());
                                    cx.notify();
                                    return;
                                }
                                let reason = d.selected_report_reason.unwrap_or(ReportReason::Spam);
                                d.pending_outcome = Some(ModalActionOutcome::SubmitReport {
                                    message_ids: message_ids.clone(),
                                    reason,
                                    comment: rb.comment,
                                });
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "提交举报", "Report").to_string()),
                ),
        )
        .into_any_element()
}
