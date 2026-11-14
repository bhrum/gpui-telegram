use crate::i18n::tr_str;
use crate::theme::assets::{IconName, tg_icon};
use crate::views::boxes::create_poll_box::{
    K_MAX_OPTIONS_COUNT, K_OPTION_LIMIT, K_QUESTION_LIMIT,
};
use crate::views::boxes::modal_theme::{
    modal_card, modal_error_banner, modal_field, modal_footer_cancel, modal_header,
    modal_toggle_chip, DialogClose, DialogMut,
};
use crate::views::boxes::modal_types::{ActiveTextField, ModalActionOutcome};
use crate::views::modal_boxes_suite::ActiveModalDialog;
use crate::views::stage::cards::{PollOption, PollType};
use gpui::prelude::FluentBuilder;
use gpui::*;

pub fn render_create_poll<V: 'static>(
    dlg: &ActiveModalDialog,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let quiz = dlg.poll_quiz_mode;
    let opts = dlg.poll_options.clone();
    let correct = dlg.poll_correct_idx;

    modal_card(440.0, pal)
        .child(modal_header(
            IconName::Poll,
            tr_str(lang, "创建投票 / 测验", "Create Poll"),
            pal,
            on_close,
            cx,
        ))
        .child(modal_field(
            &format!(
                "{} ({}/{})",
                tr_str(lang, "问题", "Question"),
                dlg.primary_input.chars().count(),
                K_QUESTION_LIMIT
            ),
            &dlg.primary_input,
            tr_str(lang, "输入投票问题...", "Ask a question..."),
            ActiveTextField::Primary,
            dlg.active_field == ActiveTextField::Primary,
            pal,
            access,
            cx,
        ))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(6.0))
                .children(opts.iter().enumerate().map(|(idx, opt_txt)| {
                    let foc = dlg.active_field == ActiveTextField::OptionIndex(idx);
                    let is_corr = correct == idx;
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .when(quiz, |el| {
                            el.child(
                                div()
                                    .cursor_pointer()
                                    .w(px(22.0))
                                    .h(px(22.0))
                                    .rounded_full()
                                    .bg(rgb(if is_corr { pal.success } else { pal.input_bg }))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _e, _w, cx| {
                                            access(this).poll_correct_idx = idx;
                                            cx.notify();
                                        }),
                                    )
                                    .child(tg_icon(IconName::SelectCheck, 12.0, pal.active_button_fg)),
                            )
                        })
                        .child(
                            div()
                                .cursor_pointer()
                                .flex_1()
                                .px(px(8.0))
                                .py(px(6.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .border_1()
                                .border_color(rgb(if foc { pal.accent } else { pal.border_fg }))
                                .text_size(px(13.0))
                                .text_color(rgb(if opt_txt.is_empty() {
                                    pal.text_secondary
                                } else {
                                    pal.text_primary
                                }))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _e, _w, cx| {
                                        access(this).active_field = ActiveTextField::OptionIndex(idx);
                                        cx.notify();
                                    }),
                                )
                                .child(if opt_txt.is_empty() {
                                    format!("{} {}", tr_str(lang, "选项", "Option"), idx + 1)
                                } else {
                                    opt_txt.clone()
                                }),
                        )
                        .when(opts.len() > 2, |el| {
                            el.child(
                                div()
                                    .cursor_pointer()
                                    .p(px(4.0))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _e, _w, cx| {
                                            let d = access(this);
                                            if d.poll_options.len() > 2 && idx < d.poll_options.len() {
                                                d.poll_options.remove(idx);
                                            }
                                            cx.notify();
                                        }),
                                    )
                                    .child(tg_icon(IconName::DeleteTrash, 14.0, pal.danger)),
                            )
                        })
                })),
        )
        .when(opts.len() < K_MAX_OPTIONS_COUNT, |el| {
            el.child(
                div()
                    .cursor_pointer()
                    .text_size(px(12.5))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(pal.text_link))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _e, _w, cx| {
                            let d = access(this);
                            if d.poll_options.len() < K_MAX_OPTIONS_COUNT {
                                d.poll_options.push(String::new());
                            }
                            cx.notify();
                        }),
                    )
                    .child(tr_str(lang, "+ 添加选项", "+ Add an option").to_string()),
            )
        })
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(6.0))
                .child(modal_toggle_chip(
                    tr_str(lang, "匿名投票", "Anonymous"),
                    dlg.poll_anonymous,
                    pal,
                    move |this| {
                        let d = access(this);
                        d.poll_anonymous = !d.poll_anonymous;
                    },
                    cx,
                ))
                .child(modal_toggle_chip(
                    tr_str(lang, "多选", "Multiple Answers"),
                    dlg.poll_multiple,
                    pal,
                    move |this| {
                        let d = access(this);
                        d.poll_multiple = !d.poll_multiple;
                        if d.poll_multiple {
                            d.poll_quiz_mode = false;
                        }
                    },
                    cx,
                ))
                .child(modal_toggle_chip(
                    tr_str(lang, "测验模式", "Quiz Mode"),
                    dlg.poll_quiz_mode,
                    pal,
                    move |this| {
                        let d = access(this);
                        d.poll_quiz_mode = !d.poll_quiz_mode;
                        if d.poll_quiz_mode {
                            d.poll_multiple = false;
                        }
                    },
                    cx,
                )),
        )
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
                                let q = d.primary_input.trim().to_string();
                                let valid_opts: Vec<String> = d
                                    .poll_options
                                    .iter()
                                    .map(|o| o.trim().chars().take(K_OPTION_LIMIT).collect::<String>())
                                    .filter(|o| !o.is_empty())
                                    .collect();
                                if q.is_empty() || q.chars().count() > K_QUESTION_LIMIT || valid_opts.len() < 2 {
                                    d.validation_error = Some(
                                        tr_str(
                                            &d.language_id,
                                            "请输入投票问题并至少填写 2 个选项",
                                            "Enter a question and at least 2 options",
                                        )
                                        .into(),
                                    );
                                    cx.notify();
                                    return;
                                }
                                let poll_type = if d.poll_quiz_mode {
                                    PollType::Quiz
                                } else if d.poll_anonymous {
                                    PollType::Anonymous
                                } else {
                                    PollType::Regular
                                };
                                let options = valid_opts
                                    .into_iter()
                                    .enumerate()
                                    .map(|(i, text)| PollOption {
                                        id: (i + 1) as u32,
                                        text,
                                        votes: 0,
                                        percentage: 0.0,
                                        is_selected: false,
                                        is_correct: d.poll_quiz_mode && i == d.poll_correct_idx,
                                    })
                                    .collect();
                                d.pending_outcome = Some(ModalActionOutcome::CreatePollSubmitted {
                                    question: q,
                                    poll_type,
                                    options,
                                });
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "创建", "Create").to_string()),
                ),
        )
        .into_any_element()
}

pub fn render_forward_box<V: 'static>(
    dlg: &ActiveModalDialog,
    message_ids: Vec<u64>,
    preview_snippet: String,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let query = dlg.primary_input.to_lowercase();
    let targets: Vec<_> = dlg
        .forward_targets
        .iter()
        .filter(|(_, name, _, _)| query.is_empty() || name.to_lowercase().contains(&query))
        .cloned()
        .collect();

    modal_card(420.0, pal)
        .child(modal_header(
            IconName::Forward,
            &format!("{} ({})", tr_str(lang, "转发消息", "Forward"), message_ids.len()),
            pal,
            on_close,
            cx,
        ))
        .child(
            div()
                .p(px(8.0))
                .rounded(px(8.0))
                .bg(rgb(pal.input_bg))
                .text_size(px(12.0))
                .text_color(rgb(pal.text_secondary))
                .child(preview_snippet),
        )
        .child(modal_toggle_chip(
            tr_str(lang, "显示发送者名称", "Show Sender Name"),
            dlg.flag_a,
            pal,
            move |this| {
                let d = access(this);
                d.flag_a = !d.flag_a;
            },
            cx,
        ))
        .child(modal_field(
            tr_str(lang, "筛选会话", "Filter Chats"),
            &dlg.primary_input,
            tr_str(lang, "搜索目标会话...", "Search target chats..."),
            ActiveTextField::Primary,
            dlg.active_field == ActiveTextField::Primary,
            pal,
            access,
            cx,
        ))
        .map(|el| {
            if targets.is_empty() {
                el.child(
                    div()
                        .p(px(14.0))
                        .text_size(px(12.5))
                        .text_color(rgb(pal.text_secondary))
                        .child(tr_str(lang, "暂无可转发的目标会话。", "No available target chats.")),
                )
            } else {
                el.children(targets.into_iter().map(|(cid, name, init, col)| {
                    let mids = message_ids.clone();
                    div()
                        .cursor_pointer()
                        .h(px(44.0))
                        .px(px(10.0))
                        .rounded(px(8.0))
                        .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
                        .flex()
                        .items_center()
                        .justify_between()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                let d = access(this);
                                d.pending_outcome = Some(ModalActionOutcome::ForwardToChat {
                                    target_chat_id: cid,
                                    message_ids: mids.clone(),
                                    show_sender: d.flag_a,
                                    extra_caption: d.secondary_input.clone(),
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
                                        .w(px(32.0))
                                        .h(px(32.0))
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
                                .child(div().text_size(px(13.0)).text_color(rgb(pal.text_primary)).child(name)),
                        )
                        .child(tg_icon(IconName::SendPlane, 15.0, pal.accent))
                }))
            }
        })
        .into_any_element()
}
