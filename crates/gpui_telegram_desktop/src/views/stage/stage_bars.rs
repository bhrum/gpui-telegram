//! Top Bar (54px), Multi-Select Bar, In-Chat Search Bar, Pinned Banner (46px) & Scroll-to-Bottom Button.

use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::i18n::{tr_fmt, tr_key_or, tr_str};
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
    DismissTopMoreMenu,
    ToggleTtlSubmenu,
    SetMessageTtl(u32),
    ViewProfile,
    ChangeWallpaper,
    TogglePinChat,
    ToggleBlockPeer,
    DeleteOrLeaveChat,
    JumpPinned,
    UnpinCurrent,
    EnterSelectMode,
    ClearSelection,
    ForwardSelected,
    CopySelected,
    DeleteSelected,
    ScrollToBottom,
    ClearHistory,
    ExportChat,
    ToggleMuteChat,
}

fn format_ttl_badge(ttl_seconds: u32, lang_id: &str) -> String {
    match ttl_seconds {
        0 => tr_str(lang_id, "关闭", "Off").to_string(),
        86_400 => tr_str(lang_id, "1 天", "1 day").to_string(),
        604_800 => tr_str(lang_id, "1 周", "1 week").to_string(),
        2_592_000 | 2_678_400 => tr_str(lang_id, "1 个月", "1 month").to_string(),
        secs if secs >= 86_400 => format!("{}d", secs / 86_400),
        secs if secs >= 3600 => format!("{}h", secs / 3600),
        secs => format!("{}s", secs),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn render_stage_top_bar<V: 'static>(
    chat_id: Option<u64>,
    title: &str,
    subtitle: &str,
    initials: &str,
    avatar_url: Option<&str>,
    select_mode_active: bool,
    selected_count: usize,
    message_ttl_seconds: u32,
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
    let bar_h = px(palette.scaled_px(54.0));
    if select_mode_active || selected_count > 0 {
        let count_str = selected_count.to_string();
        let a_fwd = on_action.clone();
        let a_cpy = on_action.clone();
        let a_del = on_action.clone();
        let a_clr = on_action.clone();
        let left_controls = if selected_count == 0 {
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(tg_icon(IconName::SelectCheck, palette.scaled_px(16.0), palette.accent))
                .child(
                    div()
                        .text_size(px(palette.scaled_px(13.0)))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(palette.text_primary))
                        .child(tr_str(lang_id, "点击消息进行多选", "Click messages to select")),
                )
        } else {
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
                                .text_size(px(palette.scaled_px(12.0)))
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
                                .text_size(px(palette.scaled_px(12.0)))
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
                                .text_size(px(palette.scaled_px(12.0)))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(palette.danger))
                                .child(tr_key_or(lang_id, "lng_context_delete_msg", "Delete")),
                        ),
                )
        };
        return div()
            .id("stage-multiselect-bar")
            .h(bar_h)
            .w_full()
            .px(px(14.0))
            .bg(rgb(palette.topbar_bg))
            .border_b_1()
            .border_color(rgb(palette.border_fg))
            .flex()
            .items_center()
            .justify_between()
            .child(left_controls)
            .child(
                div()
                    .id("ms-cancel-btn")
                    .px(px(10.0))
                    .py(px(6.0))
                    .rounded(px(6.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                    .on_click(cx.listener(move |this, _, _, cx| a_clr(this, TopBarUiAction::ClearSelection, cx)))
                    .text_size(px(palette.scaled_px(12.0)))
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
    let ttl_header_label = (message_ttl_seconds > 0).then(|| format_ttl_badge(message_ttl_seconds, lang_id));

    let bar = div()
        .relative()
        .h(bar_h)
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
                .gap(px(palette.scaled_px(10.0)))
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
                            .w(px(palette.scaled_px(40.0)))
                            .h(px(palette.scaled_px(40.0)))
                            .rounded_full()
                            .overflow_hidden()
                            .bg(rgb(grad_top))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(palette.scaled_px(14.0)))
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
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .child(
                                    div()
                                        .text_size(px(palette.scaled_px(14.0)))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(palette.text_primary))
                                        .child(title.to_string()),
                                )
                                .when_some(ttl_header_label, |row, ttl_lbl| {
                                    row.child(
                                        div()
                                            .id("topbar-ttl-badge")
                                            .px(px(palette.scaled_px(6.0)))
                                            .py(px(palette.scaled_px(1.5)))
                                            .rounded_full()
                                            .bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.16))
                                            .flex()
                                            .items_center()
                                            .gap(px(3.0))
                                            .child(tg_icon(IconName::ClockSending, palette.scaled_px(11.0), palette.accent))
                                            .child(
                                                div()
                                                    .text_size(px(palette.scaled_px(10.5)))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(palette.accent))
                                                    .child(ttl_lbl),
                                            ),
                                    )
                                }),
                        )
                        .when(!subtitle.is_empty(), |el| {
                            let is_live = subtitle == "online" || subtitle.contains("typing");
                            el.child(
                                div()
                                    .text_size(px(palette.scaled_px(12.0)))
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

    let _ = (search_query, search_match_cursor, search_match_total, tr_fmt);
    bar.into_any_element()
}

pub fn render_stage_search_bar<V: 'static>(
    search_query: &str,
    search_match_cursor: usize,
    search_match_total: usize,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, TopBarUiAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let a_prev = on_action.clone();
    let a_next = on_action.clone();
    let a_close = on_action;
    let match_lbl = if search_query.is_empty() {
        tr_str(lang_id, "输入关键词搜索", "Type to search").to_string()
    } else if search_match_total == 0 {
        tr_str(lang_id, "无匹配结果", "0 / 0").to_string()
    } else {
        format!("{} / {}", search_match_cursor + 1, search_match_total)
    };
    let display_text = if search_query.is_empty() {
        tr_str(lang_id, "| 搜索当前对话消息…", "| Search messages in this chat…").to_string()
    } else {
        format!("{}|", search_query)
    };

    div()
        .id("stage-in-chat-search-bar")
        .h(px(44.0))
        .w_full()
        .flex_shrink_0()
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
                .child(tg_icon(IconName::Search, 15.0, palette.accent))
                .child(
                    div()
                        .flex_1()
                        .px(px(10.0))
                        .py(px(5.0))
                        .rounded(px(8.0))
                        .bg(rgb(palette.input_bg))
                        .border_1()
                        .border_color(rgb(palette.accent))
                        .text_size(px(13.0))
                        .text_color(if search_query.is_empty() {
                            rgb(palette.text_secondary)
                        } else {
                            rgb(palette.text_primary)
                        })
                        .child(display_text),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(6.0))
                .child(
                    div()
                        .text_size(px(11.5))
                        .text_color(rgb(palette.text_secondary))
                        .child(match_lbl),
                )
                .child(
                    div()
                        .id("search-prev-btn")
                        .px(px(6.0))
                        .py(px(4.0))
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(palette.menu_hover_bg)))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                cx.stop_propagation();
                                a_prev(this, TopBarUiAction::SearchStep(-1), cx);
                            }),
                        )
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
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                cx.stop_propagation();
                                a_next(this, TopBarUiAction::SearchStep(1), cx);
                            }),
                        )
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
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                cx.stop_propagation();
                                a_close(this, TopBarUiAction::SearchClear, cx);
                            }),
                        )
                        .child(tg_icon(IconName::Close, 14.0, palette.icon_fg)),
                ),
        )
        .into_any_element()
}

pub fn render_stage_more_menu_overlay<V: 'static>(
    is_group_chat: bool,
    can_block_peer: bool,
    is_muted_chat: bool,
    is_pinned_chat: bool,
    is_peer_blocked: bool,
    message_ttl_seconds: u32,
    ttl_submenu_open: bool,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, TopBarUiAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let a_dismiss = on_action.clone();
    let a_mute = on_action.clone();
    let a_prof = on_action.clone();
    let a_srch = on_action.clone();
    let a_sel = on_action.clone();
    let a_ttl_toggle = on_action.clone();
    let a_wall = on_action.clone();
    let a_pin = on_action.clone();
    let a_exp = on_action.clone();
    let a_clr = on_action.clone();
    let a_blk = on_action.clone();
    let a_del = on_action.clone();

    let mute_label = if is_muted_chat {
        tr_str(lang_id, "开启通知", "Unmute notifications")
    } else {
        tr_str(lang_id, "静音通知", "Mute notifications")
    };
    let profile_label = if is_group_chat {
        tr_str(lang_id, "查看群组信息", "View group info")
    } else {
        tr_str(lang_id, "查看个人资料", "View profile")
    };
    let pin_label = if is_pinned_chat {
        tr_str(lang_id, "取消置顶对话", "Unpin chat")
    } else {
        tr_str(lang_id, "置顶对话", "Pin chat")
    };
    let block_label = if is_peer_blocked {
        tr_str(lang_id, "取消屏蔽用户", "Unblock user")
    } else {
        tr_str(lang_id, "屏蔽用户", "Block user")
    };
    let delete_label = if is_group_chat {
        tr_str(lang_id, "退出并删除群组", "Leave group")
    } else {
        tr_str(lang_id, "删除对话", "Delete chat")
    };
    let ttl_badge = format_ttl_badge(message_ttl_seconds, lang_id);
    let hover_bg = palette.menu_hover_bg;
    let pal = *palette;

    let ttl_options: [(u32, &'static str, &'static str); 4] = [
        (0, "关闭", "Off"),
        (86_400, "1 天", "1d"),
        (604_800, "1 周", "1w"),
        (2_592_000, "1 月", "1m"),
    ];

    let dropdown = div()
        .id("topbar-more-dropdown")
        .absolute()
        .top(px(50.0))
        .right(px(12.0))
        .w(px(238.0))
        .p(px(6.0))
        .rounded(px(12.0))
        .bg(rgb(pal.menu_bg))
        .border_1()
        .border_color(rgb(pal.border_fg))
        .shadow_xl()
        .occlude()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|_, _, _, cx| {
                cx.stop_propagation();
            }),
        )
        .flex()
        .flex_col()
        .gap(px(2.0))
        .child(more_menu_item(
            "tb-menu-mute",
            if is_muted_chat { IconName::Speaker } else { IconName::Mute },
            mute_label,
            false,
            palette,
            cx,
            move |this, cx| a_mute(this, TopBarUiAction::ToggleMuteChat, cx),
        ))
        .child(more_menu_item(
            "tb-menu-profile",
            IconName::ContactsUsers,
            profile_label,
            false,
            palette,
            cx,
            move |this, cx| a_prof(this, TopBarUiAction::ViewProfile, cx),
        ))
        .child(more_menu_item(
            "tb-menu-search",
            IconName::Search,
            tr_str(lang_id, "搜索聊天记录", "Search in chat"),
            false,
            palette,
            cx,
            move |this, cx| a_srch(this, TopBarUiAction::ToggleSearch, cx),
        ))
        .child(more_menu_item(
            "tb-menu-select",
            IconName::SelectCheck,
            tr_str(lang_id, "选择消息", "Select messages"),
            false,
            palette,
            cx,
            move |this, cx| a_sel(this, TopBarUiAction::EnterSelectMode, cx),
        ))
        .child(
            div()
                .id("tb-menu-ttl")
                .w_full()
                .px(px(10.0))
                .py(px(7.0))
                .rounded(px(6.0))
                .cursor_pointer()
                .hover(move |s| s.bg(rgb(hover_bg)))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        a_ttl_toggle(this, TopBarUiAction::ToggleTtlSubmenu, cx);
                    }),
                )
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(10.0))
                        .child(tg_icon(IconName::ClockSending, 15.0, pal.icon_fg))
                        .child(
                            div()
                                .text_size(px(13.0))
                                .text_color(rgb(pal.text_primary))
                                .child(tr_str(lang_id, "自动删除消息", "Auto-delete").to_string()),
                        ),
                )
                .child(
                    div()
                        .text_size(px(11.5))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(pal.accent))
                        .child(ttl_badge),
                ),
        )
        .when(ttl_submenu_open, |el| {
            el.child(
                div()
                    .w_full()
                    .px(px(6.0))
                    .py(px(4.0))
                    .rounded(px(8.0))
                    .bg(rgb(pal.input_bg))
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .children(ttl_options.into_iter().map(|(secs, zh_lbl, en_lbl)| {
                        let is_sel = message_ttl_seconds == secs;
                        let a_set = on_action.clone();
                        let lbl = tr_str(lang_id, zh_lbl, en_lbl).to_string();
                        div()
                            .cursor_pointer()
                            .flex_1()
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(if is_sel { rgb(pal.accent) } else { rgb(pal.card_bg) })
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(11.5))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(if is_sel {
                                rgb(pal.active_button_fg)
                            } else {
                                rgb(pal.text_primary)
                            })
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    cx.stop_propagation();
                                    a_set(this, TopBarUiAction::SetMessageTtl(secs), cx);
                                }),
                            )
                            .child(lbl)
                    })),
            )
        })
        .child(more_menu_item(
            "tb-menu-wallpaper",
            IconName::SunDay,
            tr_str(lang_id, "更换聊天背景", "Change wallpaper"),
            false,
            palette,
            cx,
            move |this, cx| a_wall(this, TopBarUiAction::ChangeWallpaper, cx),
        ))
        .child(more_menu_item(
            "tb-menu-pin-chat",
            IconName::Pin,
            pin_label,
            false,
            palette,
            cx,
            move |this, cx| a_pin(this, TopBarUiAction::TogglePinChat, cx),
        ))
        .child(more_menu_item(
            "tb-menu-export",
            IconName::FileDoc,
            tr_str(lang_id, "导出聊天记录", "Export chat history"),
            false,
            palette,
            cx,
            move |this, cx| a_exp(this, TopBarUiAction::ExportChat, cx),
        ))
        .child(div().h(px(1.0)).my(px(3.0)).bg(rgb(pal.border_fg)))
        .child(more_menu_item(
            "tb-menu-clear",
            IconName::Edit,
            tr_str(lang_id, "清空聊天记录", "Clear history"),
            false,
            palette,
            cx,
            move |this, cx| a_clr(this, TopBarUiAction::ClearHistory, cx),
        ))
        .when(can_block_peer, |el| {
            el.child(more_menu_item(
                "tb-menu-block",
                IconName::Lock,
                block_label,
                !is_peer_blocked,
                palette,
                cx,
                move |this, cx| a_blk(this, TopBarUiAction::ToggleBlockPeer, cx),
            ))
        })
        .child(more_menu_item(
            "tb-menu-delete-chat",
            IconName::DeleteTrash,
            delete_label,
            true,
            palette,
            cx,
            move |this, cx| a_del(this, TopBarUiAction::DeleteOrLeaveChat, cx),
        ));

    div()
        .id("topbar-more-backdrop")
        .absolute()
        .inset_0()
        .occlude()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _, _, cx| {
                cx.stop_propagation();
                a_dismiss(this, TopBarUiAction::DismissTopMoreMenu, cx);
            }),
        )
        .child(deferred(dropdown).with_priority(40))
        .into_any_element()
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
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _, _, cx| {
                cx.stop_propagation();
                on_click(this, cx);
            }),
        )
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
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _, _, cx| {
                cx.stop_propagation();
                on_click(this, cx);
            }),
        )
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
