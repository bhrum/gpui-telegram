//! Top Bar (54px), Multi-Select Bar, In-Chat Search Bar, Pinned Banner (46px) & Scroll-to-Bottom Button.

use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::i18n::{tr_fmt, tr_key_or};
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use super::rich_meta::resolve_image_source;

#[derive(Debug, Clone)]
pub enum TopBarUiAction {
    ToggleInspector,
    ToggleSearch,
    SearchStep(i32),
    SearchClear,
    StartCall { video: bool },
    ToggleTopMoreMenu,
    JumpPinned,
    UnpinCurrent,
    ClearSelection,
    ForwardSelected,
    CopySelected,
    DeleteSelected,
    ScrollToBottom,
    ClearHistory,
    ExportChat,
    ToggleMuteChat,
}

pub fn render_stage_top_bar<V: 'static>(
    chat_id: Option<u64>,
    title: &str,
    subtitle: &str,
    initials: &str,
    avatar_url: Option<&str>,
    selected_count: usize,
    search_open: bool,
    search_query: &str,
    search_match_cursor: usize,
    search_match_total: usize,
    is_inspector_open: bool,
    more_menu_open: bool,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, TopBarUiAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    if selected_count > 0 {
        let count_str = selected_count.to_string();
        let a_fwd = on_action.clone();
        let a_cpy = on_action.clone();
        let a_del = on_action.clone();
        let a_clr = on_action.clone();
        return div()
            .h(px(54.0))
            .w_full()
            .px(px(14.0))
            .bg(rgb(palette.topbar_bg))
            .border_b_1()
            .border_color(rgb(palette.border_fg))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .id("ms-fwd-btn")
                            .px(px(14.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(rgb(palette.accent))
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, _, cx| a_fwd(this, TopBarUiAction::ForwardSelected, cx)))
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .child(tg_icon(IconName::Forward, 14.0, 0xFFFFFF))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(format!("{} {}", tr_key_or(lang_id, "lng_context_forward_msg", "Forward"), count_str)),
                            ),
                    )
                    .child(
                        div()
                            .id("ms-copy-btn")
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.14))
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, _, cx| a_cpy(this, TopBarUiAction::CopySelected, cx)))
                            .flex()
                            .items_center()
                            .gap(px(5.0))
                            .child(tg_icon(IconName::Copy, 14.0, palette.accent))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(palette.accent))
                                    .child(tr_key_or(lang_id, "lng_copy", "Copy")),
                            ),
                    )
                    .child(
                        div()
                            .id("ms-del-btn")
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(TdesktopThemePalette::rgba_with_alpha(palette.danger, 0.16))
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, _, cx| a_del(this, TopBarUiAction::DeleteSelected, cx)))
                            .flex()
                            .items_center()
                            .gap(px(5.0))
                            .child(tg_icon(IconName::DeleteTrash, 14.0, palette.danger))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(palette.danger))
                                    .child(tr_key_or(lang_id, "lng_context_delete_msg", "Delete")),
                            ),
                    ),
            )
            .child(
                div()
                    .id("ms-cancel-btn")
                    .px(px(10.0))
                    .py(px(6.0))
                    .rounded(px(6.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                    .on_click(cx.listener(move |this, _, _, cx| a_clr(this, TopBarUiAction::ClearSelection, cx)))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(palette.accent))
                    .child(tr_key_or(lang_id, "lng_cancel", "Cancel")),
            )
            .into_any_element();
    }

    let pid = chat_id.unwrap_or(0) as i64;
    let (grad_top, _grad_bot) = palette.peer_avatar_gradient(pid);
    let img_src = avatar_url.and_then(|u| resolve_image_source(u, None));
    let has_chat = chat_id.is_some();
    let a_insp1 = on_action.clone();
    let a_insp2 = on_action.clone();
    let a_search = on_action.clone();
    let a_acall = on_action.clone();
    let a_vcall = on_action.clone();
    let a_more = on_action.clone();

    let mut bar = div()
        .relative()
        .h(px(54.0))
        .w_full()
        .px(px(14.0))
        .bg(rgb(palette.topbar_bg))
        .border_b_1()
        .border_color(rgb(palette.border_fg))
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .id("topbar-peer-header")
                .flex()
                .items_center()
                .gap(px(10.0))
                .flex_1()
                .overflow_hidden()
                .when(has_chat, |el| {
                    el.cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                        a_insp1(this, TopBarUiAction::ToggleInspector, cx);
                    }))
                })
                .when(has_chat, |el| {
                    el.child(
                        div()
                            .w(px(40.0))
                            .h(px(40.0))
                            .rounded_full()
                            .overflow_hidden()
                            .bg(rgb(grad_top))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .when_some(img_src, |av, src| av.child(img(src).size_full().object_fit(ObjectFit::Cover)))
                            .when(avatar_url.map_or(true, |u| u.trim().is_empty()), |av| {
                                av.child(initials.to_string())
                            }),
                    )
                })
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .overflow_hidden()
                        .child(
                            div()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(palette.text_primary))
                                .child(title.to_string()),
                        )
                        .when(!subtitle.is_empty(), |el| {
                            let is_live = subtitle == "online" || subtitle.contains("typing");
                            el.child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(if is_live { palette.online_dot_fg } else { palette.text_secondary }))
                                    .child(subtitle.to_string()),
                            )
                        }),
                ),
        )
        .when(has_chat, |el| {
            el.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .child(top_icon_btn("tb-search", IconName::Search, search_open, palette, cx, move |this, cx| {
                        a_search(this, TopBarUiAction::ToggleSearch, cx);
                    }))
                    .child(top_icon_btn("tb-call", IconName::PhoneCall, false, palette, cx, move |this, cx| {
                        a_acall(this, TopBarUiAction::StartCall { video: false }, cx);
                    }))
                    .child(top_icon_btn("tb-vcall", IconName::VideoCall, false, palette, cx, move |this, cx| {
                        a_vcall(this, TopBarUiAction::StartCall { video: true }, cx);
                    }))
                    .child(top_icon_btn("tb-info", IconName::SidebarPanel, is_inspector_open, palette, cx, move |this, cx| {
                        a_insp2(this, TopBarUiAction::ToggleInspector, cx);
                    }))
                    .child(top_icon_btn("tb-more", IconName::MoreVertical, more_menu_open, palette, cx, move |this, cx| {
                        a_more(this, TopBarUiAction::ToggleTopMoreMenu, cx);
                    })),
            )
        });

    if search_open {
        let a_prev = on_action.clone();
        let a_next = on_action.clone();
        let a_close = on_action.clone();
        let match_lbl = if search_query.is_empty() {
            tr_key_or(lang_id, "lng_dlg_filter", "Search").to_string()
        } else if search_match_total == 0 {
            "0 / 0".to_string()
        } else {
            format!("{} of {}", search_match_cursor + 1, search_match_total)
        };
        bar = bar.child(
            div()
                .absolute()
                .top(px(54.0))
                .left_0()
                .right_0()
                .h(px(44.0))
                .px(px(14.0))
                .bg(rgb(palette.topbar_bg))
                .border_b_1()
                .border_color(rgb(palette.border_fg))
                .flex()
                .items_center()
                .justify_between()
                .gap(px(8.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .flex_1()
                        .child(tg_icon(IconName::Search, 15.0, palette.icon_fg))
                        .child(
                            div()
                                .flex_1()
                                .px(px(10.0))
                                .py(px(5.0))
                                .rounded(px(8.0))
                                .bg(rgb(palette.input_bg))
                                .text_size(px(13.0))
                                .text_color(if search_query.is_empty() {
                                    rgb(palette.text_secondary)
                                } else {
                                    rgb(palette.text_primary)
                                })
                                .child(if search_query.is_empty() {
                                    "Search in chat... (type to filter)".to_string()
                                } else {
                                    search_query.to_string()
                                }),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .child(div().text_size(px(11.0)).text_color(rgb(palette.text_secondary)).child(match_lbl))
                        .child(
                            div()
                                .id("search-prev-btn")
                                .px(px(6.0))
                                .py(px(4.0))
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                                .on_click(cx.listener(move |this, _, _, cx| a_prev(this, TopBarUiAction::SearchStep(-1), cx)))
                                .text_size(px(12.0))
                                .text_color(rgb(palette.icon_fg))
                                .child("▲"),
                        )
                        .child(
                            div()
                                .id("search-next-btn")
                                .px(px(6.0))
                                .py(px(4.0))
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                                .on_click(cx.listener(move |this, _, _, cx| a_next(this, TopBarUiAction::SearchStep(1), cx)))
                                .text_size(px(12.0))
                                .text_color(rgb(palette.icon_fg))
                                .child("▼"),
                        )
                        .child(
                            div()
                                .id("search-close-btn")
                                .p(px(5.0))
                                .rounded_full()
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                                .on_click(cx.listener(move |this, _, _, cx| a_close(this, TopBarUiAction::SearchClear, cx)))
                                .child(tg_icon(IconName::Close, 14.0, palette.icon_fg)),
                        ),
                ),
        );
    }

    if more_menu_open {
        let a_mute = on_action.clone();
        let a_srch = on_action.clone();
        let a_exp = on_action.clone();
        let a_clr = on_action.clone();
        bar = bar.child(
            div()
                .id("topbar-more-dropdown")
                .absolute()
                .top(px(52.0))
                .right(px(14.0))
                .w(px(208.0))
                .p(px(6.0))
                .rounded(px(10.0))
                .bg(rgb(palette.menu_bg))
                .border_1()
                .border_color(rgb(palette.border_fg))
                .shadow_lg()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(more_menu_item("tb-menu-search", IconName::Search, tr_key_or(lang_id, "lng_dlg_filter", "Search"), false, palette, cx, move |this, cx| a_srch(this, TopBarUiAction::ToggleSearch, cx)))
                .child(more_menu_item("tb-menu-mute", IconName::Mute, tr_key_or(lang_id, "lng_profile_mute_notifications", "Mute / Unmute"), false, palette, cx, move |this, cx| a_mute(this, TopBarUiAction::ToggleMuteChat, cx)))
                .child(more_menu_item("tb-menu-export", IconName::FileDoc, tr_key_or(lang_id, "lng_profile_export_chat", "Export chat history"), false, palette, cx, move |this, cx| a_exp(this, TopBarUiAction::ExportChat, cx)))
                .child(div().h(px(1.0)).my(px(3.0)).bg(rgb(palette.border_fg)))
                .child(more_menu_item("tb-menu-clear", IconName::DeleteTrash, tr_key_or(lang_id, "lng_profile_clear_history", "Clear history"), true, palette, cx, move |this, cx| a_clr(this, TopBarUiAction::ClearHistory, cx))),
        );
    }

    let _ = tr_fmt;
    bar.into_any_element()
}

fn more_menu_item<V: 'static>(
    id: &'static str,
    icon: IconName,
    label: &str,
    danger: bool,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_click: impl Fn(&mut V, &mut Context<V>) + 'static,
) -> AnyElement {
    let fg = if danger { palette.danger } else { palette.text_primary };
    let icon_fg = if danger { palette.danger } else { palette.icon_fg };
    let hover_bg = palette.menu_hover_bg;
    div()
        .id(id)
        .w_full()
        .px(px(10.0))
        .py(px(7.0))
        .rounded(px(6.0))
        .cursor_pointer()
        .hover(move |s| s.bg(rgb(hover_bg)))
        .on_click(cx.listener(move |this, _, _, cx| on_click(this, cx)))
        .flex()
        .items_center()
        .gap(px(10.0))
        .child(tg_icon(icon, 15.0, icon_fg))
        .child(div().text_size(px(13.0)).text_color(rgb(fg)).child(label.to_string()))
        .into_any_element()
}

fn top_icon_btn<V: 'static>(
    id: &'static str,
    icon: IconName,
    active: bool,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_click: impl Fn(&mut V, &mut Context<V>) + 'static,
) -> AnyElement {
    let color = if active { palette.icon_active } else { palette.icon_fg };
    let hover_bg = palette.menu_hover_bg;
    div()
        .id(id)
        .w(px(34.0))
        .h(px(34.0))
        .rounded_full()
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .hover(move |s| s.bg(rgb(hover_bg)))
        .on_click(cx.listener(move |this, _, _, cx| on_click(this, cx)))
        .child(tg_icon(icon, 18.0, color))
        .into_any_element()
}

pub fn render_pinned_banner_46px<V: 'static>(
    pinned_snippet: &str,
    pinned_sender: &str,
    pinned_thumb_url: Option<&str>,
    pin_count: usize,
    active_pin_idx: usize,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, TopBarUiAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let a_jump = on_action.clone();
    let a_unpin = on_action.clone();
    let dashes = pin_count.clamp(1, 4);
    let active_dash = active_pin_idx % dashes;
    let dash_h = if dashes <= 1 { 26.0 } else if dashes == 2 { 12.0 } else if dashes == 3 { 8.0 } else { 5.5 };
    let thumb_src = pinned_thumb_url.and_then(|u| resolve_image_source(u, None));
    let header_title = if pin_count > 1 {
        format!(
            "{} #{}/{}",
            tr_key_or(lang_id, "lng_pinned_message", "Pinned Message"),
            active_pin_idx + 1,
            pin_count
        )
    } else if !pinned_sender.is_empty() {
        format!("{}: {}", tr_key_or(lang_id, "lng_pinned_message", "Pinned Message"), pinned_sender)
    } else {
        tr_key_or(lang_id, "lng_pinned_message", "Pinned Message").to_string()
    };

    div()
        .h(px(46.0))
        .w_full()
        .px(px(14.0))
        .bg(rgb(palette.topbar_bg))
        .border_b_1()
        .border_color(rgb(palette.border_fg))
        .flex()
        .items_center()
        .justify_between()
        .gap(px(10.0))
        .child(
            div()
                .id("pinned-banner-jump")
                .flex()
                .items_center()
                .gap(px(10.0))
                .flex_1()
                .overflow_hidden()
                .cursor_pointer()
                .on_click(cx.listener(move |this, _, _, cx| {
                    a_jump(this, TopBarUiAction::JumpPinned, cx);
                }))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .justify_center()
                        .gap(px(2.0))
                        .children((0..dashes).map(move |i| {
                            let is_active = i == active_dash;
                            div()
                                .w(px(2.5))
                                .h(px(dash_h))
                                .rounded_full()
                                .bg(if is_active {
                                    rgba((palette.accent << 8) | 0xFF)
                                } else {
                                    TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.32)
                                })
                        })),
                )
                .when_some(thumb_src, |el, src| {
                    el.child(
                        img(src)
                            .w(px(30.0))
                            .h(px(30.0))
                            .rounded(px(4.0))
                            .object_fit(ObjectFit::Cover),
                    )
                })
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .overflow_hidden()
                        .child(
                            div()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(palette.accent))
                                .child(header_title),
                        )
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(palette.text_secondary))
                                .child(pinned_snippet.to_string()),
                        ),
                ),
        )
        .child(
            div()
                .id("pinned-banner-close")
                .w(px(28.0))
                .h(px(28.0))
                .rounded_full()
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                .on_click(cx.listener(move |this, _, _, cx| {
                    a_unpin(this, TopBarUiAction::UnpinCurrent, cx);
                }))
                .child(tg_icon(IconName::Close, 14.0, palette.icon_fg)),
        )
        .into_any_element()
}

pub fn render_scroll_to_bottom_fab<V: 'static>(
    unread_count: u32,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_click: impl Fn(&mut V, &mut Context<V>) + 'static,
) -> AnyElement {
    div()
        .id("scroll-to-bottom-fab")
        .absolute()
        .bottom(px(16.0))
        .right(px(18.0))
        .w(px(42.0))
        .h(px(42.0))
        .rounded_full()
        .bg(rgb(palette.card_bg))
        .border_1()
        .border_color(rgb(palette.border_fg))
        .shadow_md()
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
        .on_click(cx.listener(move |this, _, _, cx| on_click(this, cx)))
        .child(tg_icon(IconName::ChevronDown, 20.0, palette.icon_fg))
        .when(unread_count > 0, |el| {
            el.child(
                div()
                    .absolute()
                    .top(px(-8.0))
                    .right(px(-4.0))
                    .px(px(6.0))
                    .py(px(1.0))
                    .rounded_full()
                    .bg(rgb(palette.unread_badge_bg))
                    .text_size(px(11.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(palette.unread_badge_fg))
                    .child(unread_count.to_string()),
            )
        })
        .into_any_element()
}
