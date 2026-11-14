use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::theme::palette::{ACCENT_SWATCHES, WALLPAPER_PRESETS};
use crate::views::boxes::modal_theme::{
    modal_card, modal_field, modal_footer_cancel, modal_header, modal_toggle_chip, DialogClose,
    DialogMut,
};
use crate::views::boxes::modal_types::{ActiveTextField, ModalActionOutcome};
use crate::views::modal_boxes_suite::ActiveModalDialog;
use gpui::*;

pub fn render_translate_box<V: 'static>(
    dlg: &ActiveModalDialog,
    original_text: String,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let targets = ["English", "简体中文", "Русский", "Español", "日本語"];
    let translated_display = dlg
        .translated_text
        .clone()
        .unwrap_or_else(|| format!("[{}] {}", dlg.target_lang, original_text));
    let copy_payload = translated_display.clone();

    modal_card(440.0, pal)
        .child(modal_header(
            IconName::Translate,
            tr_str(lang, "翻译消息", "Translate Message"),
            pal,
            on_close,
            cx,
        ))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(6.0))
                .children(targets.into_iter().map(|t| {
                    let active = dlg.target_lang == t;
                    let ts = t.to_string();
                    div()
                        .cursor_pointer()
                        .px(px(10.0))
                        .py(px(5.0))
                        .rounded(px(8.0))
                        .bg(rgb(if active { pal.accent } else { pal.input_bg }))
                        .text_size(px(12.0))
                        .text_color(rgb(if active { pal.active_button_fg } else { pal.text_primary }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                access(this).target_lang = ts.clone();
                                cx.notify();
                            }),
                        )
                        .child(t)
                })),
        )
        .child(
            div()
                .p(px(10.0))
                .rounded(px(8.0))
                .bg(rgb(pal.input_bg))
                .text_size(px(12.5))
                .text_color(rgb(pal.text_secondary))
                .child(original_text),
        )
        .child(
            div()
                .p(px(12.0))
                .rounded(px(8.0))
                .bg(rgb(pal.card_bg))
                .border_1()
                .border_color(rgb(pal.border_fg))
                .text_size(px(13.5))
                .text_color(rgb(pal.text_primary))
                .child(translated_display),
        )
        .child(
            div()
                .flex()
                .justify_end()
                .gap(px(8.0))
                .child(modal_footer_cancel(lang, pal, on_close, cx))
                .child(
                    div()
                        .cursor_pointer()
                        .px(px(14.0))
                        .py(px(7.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.accent))
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.active_button_fg))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                access(this).pending_outcome =
                                    Some(ModalActionOutcome::CopyToClipboard(copy_payload.clone()));
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "复制译文", "Copy Translation").to_string()),
                ),
        )
        .into_any_element()
}

pub fn render_media_editor<V: 'static>(
    dlg: &ActiveModalDialog,
    is_video: bool,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let aspects = ["Original", "16:9", "4:3", "1:1", "9:16"];

    modal_card(480.0, pal)
        .child(modal_header(
            IconName::Edit,
            if is_video {
                tr_str(lang, "视频编辑器", "Video Editor")
            } else {
                tr_str(lang, "图片编辑器", "Photo Editor")
            },
            pal,
            on_close,
            cx,
        ))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .gap(px(6.0))
                        .children(aspects.into_iter().map(|asp| {
                            let active = dlg.crop_aspect == asp;
                            let asp_s = asp.to_string();
                            div()
                                .cursor_pointer()
                                .px(px(8.0))
                                .py(px(4.0))
                                .rounded(px(6.0))
                                .bg(rgb(if active { pal.accent } else { pal.input_bg }))
                                .text_size(px(11.5))
                                .text_color(rgb(if active { pal.active_button_fg } else { pal.text_primary }))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _e, _w, cx| {
                                        access(this).crop_aspect = asp_s.clone();
                                        cx.notify();
                                    }),
                                )
                                .child(asp)
                        })),
                )
                .child(
                    div()
                        .cursor_pointer()
                        .px(px(10.0))
                        .py(px(4.0))
                        .rounded(px(6.0))
                        .bg(rgb(pal.input_bg))
                        .text_size(px(11.5))
                        .text_color(rgb(pal.text_primary))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                let d = access(this);
                                d.rotation_deg = (d.rotation_deg + 90) % 360;
                                cx.notify();
                            }),
                        )
                        .child(format!("Rotate {}°", dlg.rotation_deg)),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(6.0))
                .children(ACCENT_SWATCHES.iter().take(6).map(|sw| {
                    let active = dlg.brush_color == sw.hex;
                    let hex = sw.hex;
                    div()
                        .cursor_pointer()
                        .w(px(22.0))
                        .h(px(22.0))
                        .rounded_full()
                        .bg(rgb(hex))
                        .border_2()
                        .border_color(rgb(if active { pal.text_primary } else { pal.border_fg }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                access(this).brush_color = hex;
                                cx.notify();
                            }),
                        )
                })),
        )
        .child(
            div()
                .flex()
                .gap(px(8.0))
                .child(modal_toggle_chip(
                    tr_str(lang, "剧透遮罩", "Hide With Spoiler"),
                    dlg.media_spoiler,
                    pal,
                    move |this| {
                        let d = access(this);
                        d.media_spoiler = !d.media_spoiler;
                    },
                    cx,
                ))
                .child(modal_toggle_chip(
                    tr_str(lang, "原图发送", "Send Uncompressed"),
                    dlg.send_uncompressed,
                    pal,
                    move |this| {
                        let d = access(this);
                        d.send_uncompressed = !d.send_uncompressed;
                    },
                    cx,
                )),
        )
        .child(modal_field(
            tr_str(lang, "媒体说明", "Caption"),
            &dlg.primary_input,
            tr_str(lang, "添加说明文字...", "Add a caption..."),
            ActiveTextField::Primary,
            dlg.active_field == ActiveTextField::Primary,
            pal,
            access,
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
                                d.pending_outcome = Some(ModalActionOutcome::SendEditedMedia {
                                    is_video,
                                    aspect_label: d.crop_aspect.clone(),
                                    rotation_deg: d.rotation_deg,
                                    has_spoiler: d.media_spoiler,
                                    uncompressed_file: d.send_uncompressed,
                                    caption: d.primary_input.clone(),
                                });
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "发送", "Send").to_string()),
                ),
        )
        .into_any_element()
}

pub fn render_background_picker<V: 'static>(
    dlg: &ActiveModalDialog,
    peer_name: Option<String>,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let title = match peer_name {
        Some(ref p) => format!("{} ({})", tr_str(lang, "聊天背景", "Chat Background"), p),
        None => tr_str(lang, "聊天背景", "Chat Background").to_string(),
    };

    modal_card(440.0, pal)
        .child(modal_header(IconName::Camera, &title, pal, on_close, cx))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap(px(10.0))
                .children(WALLPAPER_PRESETS.iter().map(|preset| {
                    let active = dlg.selected_wallpaper_id == preset.id;
                    let pid = preset.id.to_string();
                    div()
                        .cursor_pointer()
                        .w(px(124.0))
                        .rounded(px(10.0))
                        .overflow_hidden()
                        .border_2()
                        .border_color(rgb(if active { pal.accent } else { pal.border_fg }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                let d = access(this);
                                d.selected_wallpaper_id = pid.clone();
                                d.pending_outcome = Some(ModalActionOutcome::SelectBackground {
                                    preset_id: pid.clone(),
                                });
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(div().h(px(64.0)).bg(rgb(preset.top_hex)))
                        .child(
                            div()
                                .px(px(8.0))
                                .py(px(5.0))
                                .bg(rgb(pal.card_bg))
                                .text_size(px(11.5))
                                .text_color(rgb(pal.text_primary))
                                .child(preset.name),
                        )
                })),
        )
        .into_any_element()
}
