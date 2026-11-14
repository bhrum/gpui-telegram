//! Themed 1:1 GPUI Renderers for Media (Photo/Video), Mosaic Album, and 100-Bar Voice Cards.

use gpui::*;
use gpui::prelude::FluentBuilder;
use std::collections::HashSet;

use crate::i18n::tr_key_or;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use super::cards::models::{AlbumItem, MediaType};
use super::entities::RichEntity;
use super::entities_render::{render_rich_message_body, RichTextAction};
use super::rich_meta::{format_file_size, resolve_image_source};

#[derive(Debug, Clone)]
pub enum MediaCardAction {
    OpenViewer { msg_id: u64, url: String, is_video: bool, album_index: Option<usize> },
    ToggleSpoilerMedia(u64),
    ToggleVoicePlay(u64),
    CycleVoiceSpeed(u64),
    SeekVoice(u64, f32),
    RichText(RichTextAction),
}

pub fn resample_waveform_100(samples: &[u8]) -> [u8; 100] {
    let mut out = [4u8; 100];
    if samples.is_empty() {
        return out;
    }
    for (i, slot) in out.iter_mut().enumerate() {
        let src_pos = (i as f32 / 99.0) * ((samples.len().saturating_sub(1)) as f32);
        let idx0 = (src_pos.floor() as usize).min(samples.len() - 1);
        let idx1 = (idx0 + 1).min(samples.len() - 1);
        let frac = src_pos - idx0 as f32;
        let v = (samples[idx0] as f32) * (1.0 - frac) + (samples[idx1] as f32) * frac;
        *slot = v.round().clamp(0.0, 31.0) as u8;
    }
    out
}

pub fn render_media_bubble_card<V: 'static>(
    msg_id: u64,
    media_type: MediaType,
    url: &str,
    local_path: Option<&str>,
    width: u32,
    height: u32,
    duration_secs: Option<u32>,
    file_size_bytes: u64,
    caption: Option<&str>,
    is_streaming: bool,
    is_spoiler_media: bool,
    revealed_spoilers: &HashSet<(u64, usize)>,
    expanded_quotes: &HashSet<(u64, usize)>,
    copied_code_msg: Option<u64>,
    explicit_entities: &[RichEntity],
    palette: &TdesktopThemePalette,
    lang_id: &str,
    is_outgoing: bool,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, MediaCardAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let aspect = if height > 0 && width > 0 { width as f32 / height as f32 } else { 1.33 };
    let render_w = 340.0f32.clamp(160.0, 410.0);
    let render_h = (render_w / aspect).clamp(150.0, 320.0);
    let is_video = matches!(media_type, MediaType::Video);
    let img_src = resolve_image_source(url, local_path);
    let spoiler_revealed = !is_spoiler_media || revealed_spoilers.contains(&(msg_id, usize::MAX));
    let url_owned = url.to_string();
    let act_click = on_action.clone();

    let badge = if let Some(dur) = duration_secs {
        format!("{}:{:02} · {}", dur / 60, dur % 60, format_file_size(file_size_bytes))
    } else {
        format!("{} · {}", match media_type {
            MediaType::Photo => "Photo",
            MediaType::Video => "Video",
        }, format_file_size(file_size_bytes))
    };

    let media_box = div()
        .id(SharedString::from(format!("media-click-{}", msg_id)))
        .relative()
        .w(px(render_w))
        .h(px(render_h))
        .rounded(px(10.0))
        .overflow_hidden()
        .bg(TdesktopThemePalette::rgba_with_alpha(palette.modal_bg, 0.9))
        .cursor_pointer()
        .on_click(cx.listener(move |this, _, _, cx| {
            if !spoiler_revealed {
                act_click(this, MediaCardAction::ToggleSpoilerMedia(msg_id), cx);
            } else {
                act_click(this, MediaCardAction::OpenViewer {
                    msg_id,
                    url: url_owned.clone(),
                    is_video,
                    album_index: None,
                }, cx);
            }
        }))
        .when_some(img_src, |el, src| {
            el.child(img(src).size_full().object_fit(ObjectFit::Cover))
        })
        .when(!spoiler_revealed, |el| {
            el.child(
                div()
                    .absolute()
                    .inset_0()
                    .bg(TdesktopThemePalette::rgba_with_alpha(0x0D1520, 0.92))
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(px(6.0))
                    .child(tg_icon(IconName::Lock, 26.0, 0xFFFFFF))
                    .child(
                        div()
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded_full()
                            .bg(TdesktopThemePalette::rgba_with_alpha(0xFFFFFF, 0.16))
                            .text_size(px(12.0))
                            .text_color(rgb(0xFFFFFF))
                            .child(tr_key_or(lang_id, "lng_media_spoiler_tap", "Tap to view")),
                    ),
            )
        })
        .when(spoiler_revealed && is_video, |el| {
            el.child(
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .w(px(46.0))
                            .h(px(46.0))
                            .rounded_full()
                            .bg(TdesktopThemePalette::rgba_with_alpha(0x000000, 0.56))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(tg_icon(IconName::Play, 22.0, 0xFFFFFF)),
                    ),
            )
        })
        .child(
            div()
                .absolute()
                .top(px(8.0))
                .left(px(8.0))
                .px(px(8.0))
                .py(px(3.0))
                .rounded_full()
                .bg(TdesktopThemePalette::rgba_with_alpha(0x000000, 0.55))
                .text_size(px(11.0))
                .text_color(rgb(0xFFFFFF))
                .child(badge),
        )
        .when(is_streaming, |el| {
            el.child(
                div()
                    .absolute()
                    .top(px(8.0))
                    .right(px(8.0))
                    .px(px(7.0))
                    .py(px(2.0))
                    .rounded(px(4.0))
                    .bg(rgb(palette.accent))
                    .text_size(px(10.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child("HD STREAM"),
            )
        });

    div()
        .flex()
        .flex_col()
        .gap(px(6.0))
        .child(media_box)
        .when_some(caption.filter(|c| !c.is_empty()), |el, cap| {
            let act_rt = on_action.clone();
            el.child(render_rich_message_body(
                msg_id,
                cap,
                explicit_entities,
                revealed_spoilers,
                expanded_quotes,
                copied_code_msg,
                palette,
                lang_id,
                is_outgoing,
                cx,
                move |this, r_act, cx| act_rt(this, MediaCardAction::RichText(r_act), cx),
            ))
        })
        .into_any_element()
}

pub fn render_album_mosaic_card<V: 'static>(
    msg_id: u64,
    items: &[AlbumItem],
    caption: Option<&str>,
    revealed_spoilers: &HashSet<(u64, usize)>,
    expanded_quotes: &HashSet<(u64, usize)>,
    copied_code_msg: Option<u64>,
    explicit_entities: &[RichEntity],
    palette: &TdesktopThemePalette,
    lang_id: &str,
    is_outgoing: bool,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, MediaCardAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let count = items.len();
    let render_tile = |idx: usize, w: f32, h: f32, extra_count: usize| -> AnyElement {
        let item = &items[idx];
        let is_video = matches!(item.media_type, MediaType::Video);
        let url_owned = item.url.clone();
        let src = resolve_image_source(&item.url, None);
        let act = on_action.clone();
        div()
            .id(SharedString::from(format!("album-tile-{}-{}", msg_id, idx)))
            .relative()
            .w(px(w))
            .h(px(h))
            .rounded(px(6.0))
            .overflow_hidden()
            .bg(rgb(item.preview_color))
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| {
                act(this, MediaCardAction::OpenViewer {
                    msg_id,
                    url: url_owned.clone(),
                    is_video,
                    album_index: Some(idx),
                }, cx);
            }))
            .when_some(src, |el, s| {
                el.child(img(s).size_full().object_fit(ObjectFit::Cover))
            })
            .when(is_video, |el| {
                el.child(
                    div()
                        .absolute()
                        .inset_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .w(px(34.0))
                                .h(px(34.0))
                                .rounded_full()
                                .bg(TdesktopThemePalette::rgba_with_alpha(0x000000, 0.55))
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(tg_icon(IconName::Play, 16.0, 0xFFFFFF)),
                        ),
                )
            })
            .when(extra_count > 0, |el| {
                el.child(
                    div()
                        .absolute()
                        .inset_0()
                        .bg(TdesktopThemePalette::rgba_with_alpha(0x000000, 0.55))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(18.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xFFFFFF))
                        .child(format!("+{}", extra_count)),
                )
            })
            .into_any_element()
    };

    let grid = match count {
        0 => div().text_size(px(12.0)).text_color(rgb(palette.text_secondary)).child("[Empty Album]").into_any_element(),
        1 => render_tile(0, 324.0, 220.0, 0),
        2 => div()
            .flex()
            .gap(px(3.0))
            .child(render_tile(0, 161.0, 180.0, 0))
            .child(render_tile(1, 161.0, 180.0, 0))
            .into_any_element(),
        3 => div()
            .flex()
            .flex_col()
            .gap(px(3.0))
            .child(render_tile(0, 325.0, 170.0, 0))
            .child(
                div()
                    .flex()
                    .gap(px(3.0))
                    .child(render_tile(1, 161.0, 120.0, 0))
                    .child(render_tile(2, 161.0, 120.0, 0)),
            )
            .into_any_element(),
        _ => {
            let overflow = count.saturating_sub(4);
            div()
                .flex()
                .flex_col()
                .gap(px(3.0))
                .child(
                    div()
                        .flex()
                        .gap(px(3.0))
                        .child(render_tile(0, 161.0, 128.0, 0))
                        .child(render_tile(1, 161.0, 128.0, 0)),
                )
                .child(
                    div()
                        .flex()
                        .gap(px(3.0))
                        .child(render_tile(2, 161.0, 128.0, 0))
                        .child(render_tile(3, 161.0, 128.0, overflow)),
                )
                .into_any_element()
        }
    };

    div()
        .flex()
        .flex_col()
        .gap(px(6.0))
        .child(grid)
        .when_some(caption.filter(|c| !c.is_empty()), |el, cap| {
            let act_rt = on_action.clone();
            el.child(render_rich_message_body(
                msg_id,
                cap,
                explicit_entities,
                revealed_spoilers,
                expanded_quotes,
                copied_code_msg,
                palette,
                lang_id,
                is_outgoing,
                cx,
                move |this, r_act, cx| act_rt(this, MediaCardAction::RichText(r_act), cx),
            ))
        })
        .into_any_element()
}

pub fn render_voice_waveform_card<V: 'static>(
    msg_id: u64,
    duration_secs: u32,
    waveform_5bit: &[u8],
    playback_position_secs: f32,
    is_playing: bool,
    playback_speed: f32,
    palette: &TdesktopThemePalette,
    is_outgoing: bool,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, MediaCardAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let bars = resample_waveform_100(waveform_5bit);
    let progress = if duration_secs > 0 {
        (playback_position_secs / duration_secs as f32).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let active_bars = (progress * 100.0).round() as usize;
    let act_play = on_action.clone();
    let act_spd = on_action.clone();
    let played_fg = if is_outgoing { palette.active_button_fg } else { palette.accent };
    let unplayed_fg = TdesktopThemePalette::rgba_with_alpha(palette.text_secondary, 0.42);
    let sub_fg = if is_outgoing { palette.msg_out_date_fg } else { palette.msg_in_date_fg };
    let shown_secs = if playback_position_secs > 0.01 {
        playback_position_secs.round() as u32
    } else {
        duration_secs
    };
    let time_str = format!("{}:{:02} / {}:{:02}", shown_secs / 60, shown_secs % 60, duration_secs / 60, duration_secs % 60);
    let speed_str = if (playback_speed - 1.5).abs() < 0.05 {
        "1.5x"
    } else if (playback_speed - 2.0).abs() < 0.05 {
        "2x"
    } else {
        "1x"
    };

    div()
        .flex()
        .items_center()
        .gap(px(10.0))
        .min_w(px(280.0))
        .child(
            div()
                .id(SharedString::from(format!("voice-play-{}", msg_id)))
                .w(px(40.0))
                .h(px(40.0))
                .rounded_full()
                .bg(rgb(palette.accent))
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .hover(|s| s.bg(rgb(palette.accent_hover)))
                .on_click(cx.listener(move |this, _, _, cx| {
                    act_play(this, MediaCardAction::ToggleVoicePlay(msg_id), cx);
                }))
                .child(tg_icon(
                    if is_playing { IconName::Pause } else { IconName::Play },
                    18.0,
                    0xFFFFFF,
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .flex_1()
                .child(
                    div()
                        .flex()
                        .items_end()
                        .h(px(26.0))
                        .gap(px(1.0))
                        .children(bars.into_iter().enumerate().map(|(idx, val)| {
                            let bar_h = 3.0 + (val as f32 / 31.0) * 21.0;
                            let act_seek = on_action.clone();
                            let bar_pct = (idx as f32 + 0.5) / 100.0;
                            let target_sec = bar_pct * duration_secs as f32;
                            div()
                                .id(SharedString::from(format!("vbar-{}-{}", msg_id, idx)))
                                .w(px(1.6))
                                .h(px(bar_h))
                                .rounded_full()
                                .cursor_pointer()
                                .bg(if idx < active_bars {
                                    rgba((played_fg << 8) | 0xFF)
                                } else {
                                    unplayed_fg
                                })
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    act_seek(this, MediaCardAction::SeekVoice(msg_id, target_sec), cx);
                                }))
                        })),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(rgb(sub_fg))
                                .child(time_str),
                        )
                        .child(
                            div()
                                .id(SharedString::from(format!("voice-spd-{}", msg_id)))
                                .px(px(6.0))
                                .py(px(1.0))
                                .rounded(px(4.0))
                                .bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.16))
                                .cursor_pointer()
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    act_spd(this, MediaCardAction::CycleVoiceSpeed(msg_id), cx);
                                }))
                                .text_size(px(10.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(palette.accent))
                                .child(speed_str),
                        ),
                ),
        )
        .into_any_element()
}
