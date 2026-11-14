//! Main Input Strip, Reply/Edit/Attachment Banners & Voice Record Bar (`composer_ui`).

use std::path::PathBuf;
use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::i18n::tr_key_or;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use super::editor_state::clamp_char_boundary;
use super::voice_gesture::VoiceGestureMachine;

#[derive(Debug, Clone)]
pub enum ComposerBarAction {
    CancelContext,
    ClearAttachments,
    ToggleAttachments,
    ToggleFormatting,
    ToggleEmojiPicker,
    FocusInput,
    SendOrSave,
    OpenSendOptions,
    StartVoice(Point<Pixels>),
    MoveVoice(Point<Pixels>),
    EndVoice,
    LockVoice,
    CancelVoice,
}

pub fn render_composer_context_banner<V: 'static>(
    replying_to: Option<&(u64, String, String)>,
    editing_message: Option<&(u64, String)>,
    attachments: &[PathBuf],
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, ComposerBarAction, &mut Context<V>) + Clone + 'static,
) -> Option<AnyElement> {
    let mut col = div().flex().flex_col().w_full();
    let mut has_any = false;

    if let Some((_id, sender, snippet)) = replying_to {
        has_any = true;
        let a = on_action.clone();
        col = col.child(banner_row(
            IconName::Reply,
            &format!("{} {}", tr_key_or(lang_id, "lng_reply_to", "Reply to"), sender),
            snippet,
            palette,
            cx,
            move |this, cx| a(this, ComposerBarAction::CancelContext, cx),
        ));
    } else if let Some((_id, orig)) = editing_message {
        has_any = true;
        let a = on_action.clone();
        col = col.child(banner_row(
            IconName::Edit,
            tr_key_or(lang_id, "lng_edit_message_title", "Edit Message"),
            orig,
            palette,
            cx,
            move |this, cx| a(this, ComposerBarAction::CancelContext, cx),
        ));
    }

    if !attachments.is_empty() {
        has_any = true;
        let a = on_action;
        let names = attachments
            .iter()
            .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .collect::<Vec<_>>()
            .join(", ");
        col = col.child(banner_row(
            IconName::Paperclip,
            &format!("{} ({})", tr_key_or(lang_id, "lng_attach_files_title", "Attachments"), attachments.len()),
            &names,
            palette,
            cx,
            move |this, cx| a(this, ComposerBarAction::ClearAttachments, cx),
        ));
    }

    if has_any { Some(col.into_any_element()) } else { None }
}

fn banner_row<V: 'static>(
    icon: IconName, title: &str, snippet: &str, palette: &TdesktopThemePalette,
    cx: &mut Context<V>, on_close: impl Fn(&mut V, &mut Context<V>) + 'static,
) -> AnyElement {
    div()
        .h(px(42.0))
        .w_full()
        .px(px(14.0))
        .bg(rgb(palette.compose_bg))
        .border_b_1()
        .border_color(rgb(palette.border_fg))
        .flex()
        .items_center()
        .justify_between()
        .gap(px(10.0))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(10.0))
                .flex_1()
                .overflow_hidden()
                .child(tg_icon(icon, 16.0, palette.accent))
                .child(div().w(px(2.5)).h(px(26.0)).rounded_full().bg(rgb(palette.accent)))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .overflow_hidden()
                        .child(div().text_size(px(12.0)).font_weight(FontWeight::BOLD).text_color(rgb(palette.accent)).child(title.to_string()))
                        .child(div().text_size(px(12.0)).text_color(rgb(palette.text_secondary)).child(snippet.to_string())),
                ),
        )
        .child(
            div()
                .id("composer-banner-close")
                .w(px(26.0))
                .h(px(26.0))
                .rounded_full()
                .cursor_pointer()
                .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                .flex()
                .items_center()
                .justify_center()
                .on_click(cx.listener(move |this, _, _, cx| on_close(this, cx)))
                .child(tg_icon(IconName::Close, 14.0, palette.icon_fg)),
        )
        .into_any_element()
}

#[allow(clippy::too_many_arguments)]
pub fn render_text_input_strip<V: 'static>(
    text: &str,
    cursor: usize,
    selection: Option<(usize, usize)>,
    preedit: Option<&str>,
    is_focused: bool,
    is_editing: bool,
    has_attachments: bool,
    is_formatting_open: bool,
    is_emoji_open: bool,
    content_height: Pixels,
    overflow_scroll: bool,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, ComposerBarAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let is_empty = text.is_empty() && preedit.map_or(true, |p| p.is_empty());
    let show_send = !is_empty || is_editing || has_attachments;
    let a_att = on_action.clone();
    let a_fmt = on_action.clone();
    let a_emo = on_action.clone();
    let a_foc = on_action.clone();
    let a_snd = on_action.clone();
    let a_sopt = on_action.clone();
    let a_mic = on_action;

    let cur = clamp_char_boundary(text, cursor);
    let before = &text[..cur];
    let after = &text[cur..];

    let text_el = if is_empty {
        div()
            .text_size(px(palette.font_size_px as f32))
            .text_color(rgb(palette.text_secondary))
            .child(tr_key_or(lang_id, "lng_message_ph", "Write a message..."))
            .into_any_element()
    } else if let Some((s, e)) = selection {
        let s = clamp_char_boundary(text, s);
        let e = clamp_char_boundary(text, e);
        div()
            .w_full()
            .flex()
            .flex_wrap()
            .items_center()
            .text_size(px(palette.font_size_px as f32))
            .text_color(rgb(palette.text_primary))
            .child(text[..s].to_string())
            .child(div().bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.35)).child(text[s..e].to_string()))
            .child(text[e..].to_string())
            .into_any_element()
    } else {
        div()
            .w_full()
            .flex()
            .flex_wrap()
            .items_center()
            .text_size(px(palette.font_size_px as f32))
            .text_color(rgb(palette.text_primary))
            .when(!before.is_empty(), |el| el.child(before.to_string()))
            .when_some(preedit.filter(|p| !p.is_empty()), |el, p| {
                el.child(div().underline().text_color(rgb(palette.accent)).child(p.to_string()))
            })
            .when(is_focused, |el| el.child(div().w(px(1.5)).h(px(17.0)).bg(rgb(palette.accent))))
            .when(!after.is_empty(), |el| el.child(after.to_string()))
            .into_any_element()
    };

    div()
        .w_full()
        .min_h(px(46.0))
        .px(px(10.0))
        .py(px(6.0))
        .bg(rgb(palette.composer_bg))
        .border_t_1()
        .border_color(rgb(palette.border_fg))
        .flex()
        .items_end()
        .gap(px(6.0))
        .child(
            div()
                .id("cmp-attach-btn")
                .w(px(36.0))
                .h(px(44.0))
                .rounded_full()
                .cursor_pointer()
                .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                .flex()
                .items_center()
                .justify_center()
                .on_click(cx.listener(move |this, _, _, cx| a_att(this, ComposerBarAction::ToggleAttachments, cx)))
                .child(tg_icon(IconName::Paperclip, 20.0, palette.icon_fg)),
        )
        .child(
            div()
                .id("cmp-input-wrap")
                .flex_1()
                .min_h(px(44.0))
                .h(content_height)
                .px(px(12.0))
                .py(px(10.0))
                .rounded(px(14.0))
                .bg(rgb(palette.input_bg))
                .border_1()
                .border_color(rgb(if is_focused { palette.accent } else { palette.border_fg }))
                .cursor_text()
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                    a_foc(this, ComposerBarAction::FocusInput, cx);
                }))
                .flex()
                .items_center()
                .justify_between()
                .gap(px(6.0))
                .child(
                    div()
                        .id("cmp-text-scroll")
                        .flex_1()
                        .when(overflow_scroll, |el| el.overflow_y_scroll().max_h(px(172.0)))
                        .child(text_el),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(4.0))
                        .child(
                            div()
                                .id("cmp-fmt-toggle")
                                .px(px(6.0))
                                .py(px(3.0))
                                .rounded(px(6.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                                .on_click(cx.listener(move |this, _, _, cx| a_fmt(this, ComposerBarAction::ToggleFormatting, cx)))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(if is_formatting_open { palette.icon_active } else { palette.icon_fg }))
                                .child("Aa"),
                        )
                        .child(
                            div()
                                .id("cmp-emoji-toggle")
                                .w(px(30.0))
                                .h(px(30.0))
                                .rounded_full()
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                                .flex()
                                .items_center()
                                .justify_center()
                                .on_click(cx.listener(move |this, _, _, cx| a_emo(this, ComposerBarAction::ToggleEmojiPicker, cx)))
                                .child(tg_icon(IconName::SmileEmoji, 20.0, if is_emoji_open { palette.icon_active } else { palette.icon_fg })),
                        ),
                ),
        )
        .child(if show_send {
            div()
                .id("cmp-send-btn")
                .w(px(44.0))
                .h(px(46.0))
                .rounded_full()
                .bg(rgb(palette.accent))
                .hover(|s| s.bg(rgb(palette.accent_hover)))
                .cursor_pointer()
                .flex()
                .items_center()
                .justify_center()
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                    a_snd(this, ComposerBarAction::SendOrSave, cx);
                }))
                .on_mouse_down(MouseButton::Right, cx.listener(move |this, _, _, cx| {
                    a_sopt(this, ComposerBarAction::OpenSendOptions, cx);
                }))
                .child(tg_icon(if is_editing { IconName::SelectCheck } else { IconName::SendPlane }, 20.0, 0xFFFFFF))
                .into_any_element()
        } else {
            div()
                .id("cmp-mic-btn")
                .w(px(44.0))
                .h(px(46.0))
                .rounded_full()
                .cursor_pointer()
                .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                .flex()
                .items_center()
                .justify_center()
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, ev: &MouseDownEvent, _, cx| {
                    a_mic(this, ComposerBarAction::StartVoice(ev.position), cx);
                }))
                .child(tg_icon(IconName::Mic, 20.0, palette.icon_fg))
                .into_any_element()
        })
        .into_any_element()
}

pub fn render_voice_recording_strip<V: 'static>(
    vg: &VoiceGestureMachine,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, ComposerBarAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let dur = vg.duration_secs();
    let mins = (dur as u32) / 60;
    let secs = (dur as u32) % 60;
    let tenths = ((dur * 10.0) as u32) % 10;
    let locked = vg.is_locked();
    let a_cancel = on_action.clone();
    let a_lock = on_action.clone();
    let a_send = on_action;

    div()
        .id("cmp-voice-bar")
        .w_full()
        .h(px(58.0))
        .px(px(14.0))
        .bg(rgb(palette.composer_bg))
        .border_t_1()
        .border_color(rgb(palette.border_fg))
        .flex()
        .items_center()
        .justify_between()
        .gap(px(12.0))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(10.0))
                .child(div().w(px(10.0)).h(px(10.0)).rounded_full().bg(rgb(palette.danger)))
                .child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(palette.text_primary))
                        .child(format!("{:01}:{:02}.{}", mins, secs, tenths)),
                )
                .child(
                    div().flex().items_center().gap(px(2.0)).children((0..14).map(|i| {
                        let h = 6.0 + (((i * 7 + (dur * 10.0) as usize) % 5) as f32) * 3.0;
                        div().w(px(2.5)).h(px(h)).rounded_full().bg(rgb(palette.accent))
                    })),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(10.0))
                .child(
                    div()
                        .id("voice-cancel-btn")
                        .px(px(10.0))
                        .py(px(6.0))
                        .rounded(px(8.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                        .on_click(cx.listener(move |this, _, _, cx| a_cancel(this, ComposerBarAction::CancelVoice, cx)))
                        .text_size(px(12.5))
                        .text_color(rgb(palette.danger))
                        .child(tr_key_or(lang_id, "lng_cancel", "Cancel")),
                )
                .when(!locked, |el| {
                    el.child(
                        div()
                            .id("voice-lock-btn")
                            .w(px(34.0))
                            .h(px(34.0))
                            .rounded_full()
                            .bg(rgb(palette.input_bg))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .on_click(cx.listener(move |this, _, _, cx| a_lock(this, ComposerBarAction::LockVoice, cx)))
                            .child(tg_icon(IconName::Lock, 15.0, palette.accent)),
                    )
                })
                .child(
                    div()
                        .id("voice-send-btn")
                        .w(px(40.0))
                        .h(px(40.0))
                        .rounded_full()
                        .bg(rgb(palette.accent))
                        .cursor_pointer()
                        .flex()
                        .items_center()
                        .justify_center()
                        .on_click(cx.listener(move |this, _, _, cx| a_send(this, ComposerBarAction::EndVoice, cx)))
                        .child(tg_icon(IconName::SendPlane, 18.0, 0xFFFFFF)),
                ),
        )
        .into_any_element()
}
