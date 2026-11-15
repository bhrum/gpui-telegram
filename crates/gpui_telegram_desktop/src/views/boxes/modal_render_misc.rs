use crate::i18n::tr_str;
use crate::theme::assets::{IconName, tg_icon};
use crate::views::boxes::about_box::AboutBox;
use crate::views::boxes::keys_legend::KeysLegend;
use crate::views::boxes::language_box::LanguageBox;
use crate::views::boxes::modal_theme::{
    modal_card, modal_field, modal_footer_cancel, modal_header, modal_toggle_chip, DialogClose,
    DialogMut,
};
use crate::views::boxes::modal_types::{ActiveTextField, ModalActionOutcome};
use crate::views::modal_boxes_suite::ActiveModalDialog;
use gpui::prelude::FluentBuilder;
use gpui::*;

pub fn render_language_picker<V: 'static>(
    dlg: &ActiveModalDialog,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let mut lb = LanguageBox::new(lang);
    lb.search_query = dlg.primary_input.clone();
    let items = lb.filter_languages();

    modal_card(400.0, pal)
        .child(modal_header(
            IconName::Translate,
            tr_str(lang, "选择语言", "Language"),
            pal,
            on_close,
            cx,
        ))
        .child(modal_field(
            tr_str(lang, "搜索语言", "Search"),
            &dlg.primary_input,
            tr_str(lang, "输入语言名称...", "Search language..."),
            ActiveTextField::Primary,
            dlg.active_field == ActiveTextField::Primary,
            pal,
            access,
            cx,
        ))
        .children(items.into_iter().map(|item| {
            let active = item.id == *lang;
            let lid = item.id.clone();
            div()
                .cursor_pointer()
                .px(px(12.0))
                .py(px(8.0))
                .rounded(px(8.0))
                .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
                .flex()
                .items_center()
                .justify_between()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _e, _w, cx| {
                        let d = access(this);
                        d.language_id = lid.clone();
                        d.pending_outcome = Some(ModalActionOutcome::SelectLanguage {
                            language_code: lid.clone(),
                        });
                        on_close(this);
                        cx.notify();
                    }),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_size(px(13.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(pal.text_primary))
                                .child(item.native_name),
                        )
                        .child(
                            div()
                                .text_size(px(11.5))
                                .text_color(rgb(pal.text_secondary))
                                .child(item.name),
                        ),
                )
                .child(tg_icon(
                    IconName::SelectCheck,
                    14.0,
                    if active { pal.accent } else { pal.text_secondary },
                ))
        }))
        .into_any_element()
}

pub fn render_export_data<V: 'static>(
    dlg: &ActiveModalDialog,
    chat_title: String,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;

    modal_card(410.0, pal)
        .child(modal_header(
            IconName::FileDoc,
            &format!("{} — {}", tr_str(lang, "导出聊天记录", "Export Chat History"), chat_title),
            pal,
            on_close,
            cx,
        ))
        .child(modal_toggle_chip(
            tr_str(lang, "导出为 JSON 格式 (否则为 HTML)", "Export as JSON (otherwise HTML)"),
            dlg.flag_a,
            pal,
            move |this| {
                let d = access(this);
                d.flag_a = !d.flag_a;
            },
            cx,
        ))
        .map(|el| {
            if let Some(ref path) = dlg.export_status_path {
                el.child(
                    div()
                        .p(px(10.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.input_bg))
                        .text_size(px(12.0))
                        .text_color(rgb(pal.success))
                        .child(path.clone()),
                )
            } else {
                el
            }
        })
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
                                d.pending_outcome = Some(ModalActionOutcome::RunChatExport {
                                    format_json: d.flag_a,
                                });
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "开始导出", "Export").to_string()),
                ),
        )
        .into_any_element()
}

pub fn render_instant_view<V: 'static>(
    dlg: &ActiveModalDialog,
    title: String,
    url: String,
    body_paragraphs: Vec<String>,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let url_copy = url.clone();

    modal_card(540.0, pal)
        .child(modal_header(IconName::FileDoc, &title, pal, on_close, cx))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_size(px(11.5))
                        .text_color(rgb(pal.text_link))
                        .child(url),
                )
                .child(
                    div()
                        .flex()
                        .gap(px(6.0))
                        .child(modal_toggle_chip(
                            &format!("{}%", dlg.iv_zoom_pct),
                            false,
                            pal,
                            move |this| {
                                let d = access(this);
                                d.iv_zoom_pct = if d.iv_zoom_pct >= 130 { 90 } else { d.iv_zoom_pct + 10 };
                            },
                            cx,
                        ))
                        .child(modal_toggle_chip(
                            tr_str(lang, "复制链接", "Copy URL"),
                            false,
                            pal,
                            move |this| {
                                access(this).pending_outcome =
                                    Some(ModalActionOutcome::CopyToClipboard(url_copy.clone()));
                            },
                            cx,
                        )),
                ),
        )
        .children(body_paragraphs.into_iter().map(|para| {
            div()
                .text_size(px(13.5 * (dlg.iv_zoom_pct as f32 / 100.0)))
                .text_color(rgb(pal.text_primary))
                .child(para)
        }))
        .into_any_element()
}

pub fn render_shortcuts_legend<V: 'static>(
    dlg: &ActiveModalDialog,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let legend = KeysLegend::new();

    modal_card(440.0, pal)
        .child(modal_header(
            IconName::SettingsGear,
            tr_str(lang, "键盘快捷键", "Keyboard Shortcuts"),
            pal,
            on_close,
            cx,
        ))
        .children(legend.categories.into_iter().map(|cat| {
            div()
                .flex()
                .flex_col()
                .gap(px(6.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.text_link))
                        .child(cat.name),
                )
                .children(cat.items.into_iter().map(|item| {
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .text_size(px(12.5))
                        .child(div().text_color(rgb(pal.text_primary)).child(item.description))
                        .child(
                            div()
                                .px(px(8.0))
                                .py(px(3.0))
                                .rounded(px(6.0))
                                .bg(rgb(pal.input_bg))
                                .text_size(px(11.5))
                                .text_color(rgb(pal.text_secondary))
                                .child(item.key_combination),
                        )
                }))
        }))
        .into_any_element()
}

pub fn render_about_telegram<V: 'static>(
    dlg: &ActiveModalDialog,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let about = AboutBox::new();
    let faq = about.faq_url.clone();
    let github = about.github_url.clone();

    modal_card(400.0, pal)
        .child(modal_header(
            IconName::BotRobot,
            tr_str(lang, "关于 AI Chat", "About AI Chat"),
            pal,
            on_close,
            cx,
        ))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(12.0))
                .child(tg_icon(IconName::BotRobot, 40.0, pal.accent))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_size(px(15.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.text_primary))
                                .child("AI Chat Desktop"),
                        )
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(pal.text_secondary))
                                .child(format!("Version {}", about.version_text)),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .gap(px(8.0))
                .child(modal_toggle_chip(
                    "FAQ",
                    false,
                    pal,
                    move |this| {
                        access(this).pending_outcome =
                            Some(ModalActionOutcome::CopyToClipboard(faq.clone()));
                    },
                    cx,
                ))
                .child(modal_toggle_chip(
                    "GitHub",
                    false,
                    pal,
                    move |this| {
                        access(this).pending_outcome =
                            Some(ModalActionOutcome::CopyToClipboard(github.clone()));
                    },
                    cx,
                )),
        )
        .into_any_element()
}

pub fn render_info_document<V: 'static>(
    dlg: &ActiveModalDialog,
    title: String,
    body: String,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;

    modal_card(440.0, pal)
        .child(modal_header(IconName::FileDoc, &title, pal, on_close, cx))
        .child(
            div()
                .text_size(px(13.0))
                .text_color(rgb(pal.text_primary))
                .child(body),
        )
        .child(
            div()
                .flex()
                .justify_end()
                .child(modal_footer_cancel(lang, pal, on_close, cx)),
        )
        .into_any_element()
}
