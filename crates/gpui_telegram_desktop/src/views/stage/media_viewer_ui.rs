//! Fullscreen Lightbox Renderer & Controls for `MediaViewer` (`media_viewer_ui`).

use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::i18n::tr_key_or;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use super::rich_meta::resolve_image_source;
use crate::views::media_viewer::MediaViewerItem;

#[derive(Debug, Clone)]
pub enum MediaViewerUiAction {
    Close,
    Prev,
    Next,
    SelectIndex(usize),
    ZoomIn,
    ZoomOut,
    ResetZoom,
    RotateRight,
    TogglePlay,
    SaveDownload,
    Forward,
    OpenEditor,
    PanStart(Point<Pixels>),
    PanMove(Point<Pixels>),
    PanEnd,
}

#[allow(clippy::too_many_arguments)]
pub fn render_media_viewer_overlay<V: 'static>(
    items: &[MediaViewerItem],
    current_index: usize,
    zoom_level: f32,
    rotation_deg: u32,
    pan_offset: Point<Pixels>,
    is_playing: bool,
    saved_notice: Option<&str>,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, MediaViewerUiAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let cur = items.get(current_index);
    let total = items.len();
    let base_w = if rotation_deg % 180 == 90 { 520.0 } else { 760.0 };
    let base_h = if rotation_deg % 180 == 90 { 760.0 } else { 520.0 };
    let view_w = base_w * zoom_level;
    let view_h = base_h * zoom_level;

    let a_bg = on_action.clone();
    let a_close = on_action.clone();
    let a_dl = on_action.clone();
    let a_fwd = on_action.clone();
    let a_ed = on_action.clone();
    let a_prev = on_action.clone();
    let a_next = on_action.clone();
    let a_zout = on_action.clone();
    let a_zin = on_action.clone();
    let a_zrst = on_action.clone();
    let a_rot = on_action.clone();
    let a_play = on_action.clone();
    let a_pstart = on_action.clone();
    let a_pmove = on_action.clone();
    let a_pend = on_action.clone();

    div()
        .id("media-viewer-backdrop")
        .absolute()
        .inset_0()
        .bg(TdesktopThemePalette::rgba_with_alpha(0x05080D, 0.94))
        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| a_bg(this, MediaViewerUiAction::Close, cx)))
        .on_mouse_move(cx.listener(move |this, ev: &MouseMoveEvent, _, cx| a_pmove(this, MediaViewerUiAction::PanMove(ev.position), cx)))
        .on_mouse_up(MouseButton::Left, cx.listener(move |this, _, _, cx| a_pend(this, MediaViewerUiAction::PanEnd, cx)))
        .flex()
        .flex_col()
        .justify_between()
        .child(
            div()
                .id("mv-header-bar")
                .h(px(56.0))
                .px(px(20.0))
                .flex()
                .items_center()
                .justify_between()
                .on_mouse_down(MouseButton::Left, |_, _, _| {})
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(div().text_size(px(14.0)).font_weight(FontWeight::BOLD).text_color(rgb(0xFFFFFF)).child(cur.map(|i| i.sender_name.clone()).unwrap_or_default()))
                        .child(div().text_size(px(12.0)).text_color(rgb(0x9FB0C0)).child(format!("{} • {}/{}", cur.map(|i| i.date.as_str()).unwrap_or(""), current_index + 1, total.max(1)))),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .when_some(saved_notice, |el, msg| {
                            el.child(div().px(px(10.0)).py(px(4.0)).rounded_full().bg(rgb(palette.success)).text_size(px(11.5)).text_color(rgb(0xFFFFFF)).child(msg.to_string()))
                        })
                        .child(mv_icon_btn("mv-edit", IconName::Edit, cx, move |this, cx| a_ed(this, MediaViewerUiAction::OpenEditor, cx)))
                        .child(mv_icon_btn("mv-fwd", IconName::Forward, cx, move |this, cx| a_fwd(this, MediaViewerUiAction::Forward, cx)))
                        .child(mv_icon_btn("mv-dl", IconName::FileDoc, cx, move |this, cx| a_dl(this, MediaViewerUiAction::SaveDownload, cx)))
                        .child(mv_icon_btn("mv-close", IconName::Close, cx, move |this, cx| a_close(this, MediaViewerUiAction::Close, cx))),
                ),
        )
        .child(
            div()
                .relative()
                .flex_1()
                .w_full()
                .overflow_hidden()
                .flex()
                .items_center()
                .justify_center()
                .when(total > 1, |el| {
                    el.child(
                        div()
                            .id("mv-prev-btn")
                            .absolute()
                            .left(px(18.0))
                            .w(px(44.0))
                            .h(px(44.0))
                            .rounded_full()
                            .bg(TdesktopThemePalette::rgba_with_alpha(0x162230, 0.78))
                            .hover(|s| s.bg(rgb(palette.accent)))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| a_prev(this, MediaViewerUiAction::Prev, cx)))
                            .child(tg_icon(IconName::BackArrow, 20.0, 0xFFFFFF)),
                    )
                })
                .when_some(cur, |el, item| {
                    let src = resolve_image_source(&item.url, None);
                    el.child(
                        div()
                            .id("mv-media-canvas")
                            .relative()
                            .left(pan_offset.x)
                            .top(pan_offset.y)
                            .w(px(view_w))
                            .h(px(view_h))
                            .rounded(px(8.0))
                            .overflow_hidden()
                            .bg(rgb(0x0D1520))
                            .cursor_grab()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, ev: &MouseDownEvent, _, cx| {
                                a_pstart(this, MediaViewerUiAction::PanStart(ev.position), cx);
                            }))
                            .flex()
                            .items_center()
                            .justify_center()
                            .when_some(src, |c, s| c.child(img(s).size_full().object_fit(ObjectFit::Contain)))
                            .when(item.is_video, |c| {
                                c.child(
                                    div()
                                        .id("mv-video-play")
                                        .absolute()
                                        .w(px(64.0))
                                        .h(px(64.0))
                                        .rounded_full()
                                        .bg(TdesktopThemePalette::rgba_with_alpha(0x000000, 0.65))
                                        .cursor_pointer()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                            a_play(this, MediaViewerUiAction::TogglePlay, cx);
                                        }))
                                        .child(tg_icon(if is_playing { IconName::Pause } else { IconName::Play }, 26.0, 0xFFFFFF)),
                                )
                            }),
                    )
                })
                .when(total > 1, |el| {
                    el.child(
                        div()
                            .id("mv-next-btn")
                            .absolute()
                            .right(px(18.0))
                            .w(px(44.0))
                            .h(px(44.0))
                            .rounded_full()
                            .bg(TdesktopThemePalette::rgba_with_alpha(0x162230, 0.78))
                            .hover(|s| s.bg(rgb(palette.accent)))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| a_next(this, MediaViewerUiAction::Next, cx)))
                            .child(tg_icon(IconName::ChevronRight, 20.0, 0xFFFFFF)),
                    )
                }),
        )
        .child(
            div()
                .id("mv-footer-bar")
                .py(px(12.0))
                .px(px(20.0))
                .on_mouse_down(MouseButton::Left, |_, _, _| {})
                .flex()
                .flex_col()
                .items_center()
                .gap(px(10.0))
                .when_some(cur.and_then(|i| i.caption.as_ref()), |el, cap| {
                    el.child(div().max_w(px(680.0)).text_size(px(13.5)).text_color(rgb(0xFFFFFF)).child(cap.clone()))
                })
                .child(
                    div()
                        .px(px(14.0))
                        .py(px(6.0))
                        .rounded_full()
                        .bg(TdesktopThemePalette::rgba_with_alpha(0x162230, 0.90))
                        .flex()
                        .items_center()
                        .gap(px(12.0))
                        .child(mv_text_btn("mv-zout", "−", cx, move |this, cx| a_zout(this, MediaViewerUiAction::ZoomOut, cx)))
                        .child(mv_text_btn("mv-zpct", &format!("{}%", (zoom_level * 100.0).round() as i32), cx, move |this, cx| a_zrst(this, MediaViewerUiAction::ResetZoom, cx)))
                        .child(mv_text_btn("mv-zin", "+", cx, move |this, cx| a_zin(this, MediaViewerUiAction::ZoomIn, cx)))
                        .child(mv_text_btn("mv-rot", &format!("↻ {}°", rotation_deg), cx, move |this, cx| a_rot(this, MediaViewerUiAction::RotateRight, cx))),
                )
                .when(total > 1, |el| {
                    let mut strip = div().flex().items_center().gap(px(6.0));
                    for (idx, it) in items.iter().enumerate() {
                        let a_sel = on_action.clone();
                        let src = resolve_image_source(&it.url, None);
                        strip = strip.child(
                            div()
                                .id(SharedString::from(format!("mv-thumb-{}", idx)))
                                .w(px(48.0))
                                .h(px(48.0))
                                .rounded(px(6.0))
                                .overflow_hidden()
                                .bg(rgb(0x162230))
                                .border_2()
                                .border_color(if idx == current_index { rgb(palette.accent) } else { rgba(0x00000000) })
                                .cursor_pointer()
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| a_sel(this, MediaViewerUiAction::SelectIndex(idx), cx)))
                                .when_some(src, |t, s| t.child(img(s).size_full().object_fit(ObjectFit::Cover))),
                        );
                    }
                    el.child(strip)
                }),
        )
        .when(false, |el| el.child(tr_key_or(lang_id, "lng_mediaview_save_as", "Save").to_string()))
        .into_any_element()
}

fn mv_icon_btn<V: 'static>(id: &'static str, icon: IconName, cx: &mut Context<V>, on_click: impl Fn(&mut V, &mut Context<V>) + 'static) -> AnyElement {
    div()
        .id(id)
        .w(px(36.0))
        .h(px(36.0))
        .rounded_full()
        .bg(TdesktopThemePalette::rgba_with_alpha(0x162230, 0.72))
        .hover(|s| s.bg(TdesktopThemePalette::rgba_with_alpha(0x2B3B4E, 0.90)))
        .cursor_pointer()
        .flex()
        .items_center()
        .justify_center()
        .on_click(cx.listener(move |this, _, _, cx| on_click(this, cx)))
        .child(tg_icon(icon, 16.0, 0xFFFFFF))
        .into_any_element()
}

fn mv_text_btn<V: 'static>(id: &'static str, label: &str, cx: &mut Context<V>, on_click: impl Fn(&mut V, &mut Context<V>) + 'static) -> AnyElement {
    div()
        .id(id)
        .px(px(8.0))
        .py(px(3.0))
        .rounded(px(6.0))
        .hover(|s| s.bg(TdesktopThemePalette::rgba_with_alpha(0x2B3B4E, 0.85)))
        .cursor_pointer()
        .text_size(px(12.5))
        .font_weight(FontWeight::BOLD)
        .text_color(rgb(0xFFFFFF))
        .on_click(cx.listener(move |this, _, _, cx| on_click(this, cx)))
        .child(label.to_string())
        .into_any_element()
}
