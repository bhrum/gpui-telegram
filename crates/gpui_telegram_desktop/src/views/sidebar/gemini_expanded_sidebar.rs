//! Expanded (`288px`) Gemini-inspired unified left sidebar sections.

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use crate::views::chat_sidebar::{SEARCH_HEIGHT, SEARCH_RADIUS};
use crate::views::nav_rail::NavDestination;
use crate::views::sidebar::gemini_unified_sidebar::{
    format_compact_badge, render_floating_pill_tooltip_at, UnifiedNavSnapshot,
    UnifiedSidebarAction,
};

/// Renders the `46px` expanded sidebar top bar with collapse toggle and quick action buttons.
pub fn render_expanded_gemini_top_bar<V: 'static>(
    nav: &UnifiedNavSnapshot,
    is_plus_active: bool,
    hovered_item: Option<&'static str>,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, UnifiedSidebarAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let hover_bg = rgb(palette.dialogs_hover_bg);
    let active_bg = TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.16);
    let collapse_hovered = hovered_item == Some("collapse");
    let on_collapse_hover = on_action.clone();
    let on_collapse_click = on_action.clone();
    let on_plus_click = on_action.clone();

    let render_top_icon_btn =
        |id: &'static str,
         icon: IconName,
         is_active: bool,
         badge: u32,
         action: UnifiedSidebarAction| {
            let on_btn = on_action.clone();
            let fg = if is_active {
                palette.accent
            } else {
                palette.icon_fg
            };
            div()
                .id(id)
                .relative()
                .cursor_pointer()
                .w(px(30.0))
                .h(px(30.0))
                .rounded_full()
                .when(is_active, |el| el.bg(active_bg))
                .hover(move |s| if is_active { s } else { s.bg(hover_bg) })
                .flex()
                .items_center()
                .justify_center()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _ev, _w, cx| {
                        on_btn(this, action, cx);
                    }),
                )
                .child(tg_icon(icon, 16.0, fg))
                .when(badge > 0, |el| {
                    el.child(
                        div()
                            .absolute()
                            .top(px(2.0))
                            .right(px(2.0))
                            .w(px(7.0))
                            .h(px(7.0))
                            .rounded_full()
                            .bg(rgb(palette.unread_badge_bg)),
                    )
                })
        };

    let tasks_target = if nav.active_destination == NavDestination::TaskCenter {
        NavDestination::Chats
    } else {
        NavDestination::TaskCenter
    };
    let remote_target = if nav.active_destination == NavDestination::RemoteComputer {
        NavDestination::Chats
    } else {
        NavDestination::RemoteComputer
    };
    let calls_target = if nav.active_destination == NavDestination::Calls {
        NavDestination::Chats
    } else {
        NavDestination::Calls
    };

    div()
        .h(px(46.0))
        .flex_shrink_0()
        .w_full()
        .px(px(12.0))
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .id("sidebar-collapse-btn")
                .relative()
                .cursor_pointer()
                .w(px(30.0))
                .h(px(30.0))
                .rounded_full()
                .hover(move |s| s.bg(hover_bg))
                .flex()
                .items_center()
                .justify_center()
                .on_hover(cx.listener(move |this, hovered, _w, cx| {
                    let next = if *hovered { Some("collapse") } else { None };
                    on_collapse_hover(this, UnifiedSidebarAction::SetHover(next), cx);
                }))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _ev, _w, cx| {
                        on_collapse_click(this, UnifiedSidebarAction::ToggleCollapsed, cx);
                    }),
                )
                .child(tg_icon(
                    IconName::SidebarLeftClose,
                    16.0,
                    palette.icon_fg,
                ))
                .when(collapse_hovered, |el| {
                    el.child(render_floating_pill_tooltip_at(
                        tr_str(lang_id, "收起边栏", "Collapse sidebar"),
                        38.0,
                        2.0,
                        palette,
                    ))
                }),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(4.0))
                .child(render_top_icon_btn(
                    "top-action-tasks",
                    IconName::Poll,
                    nav.active_destination == NavDestination::TaskCenter,
                    nav.tasks_unread,
                    UnifiedSidebarAction::SelectDestination(tasks_target),
                ))
                .child(render_top_icon_btn(
                    "top-action-remote",
                    IconName::ComputerDesktop,
                    nav.active_destination == NavDestination::RemoteComputer,
                    0,
                    UnifiedSidebarAction::SelectDestination(remote_target),
                ))
                .child(render_top_icon_btn(
                    "top-action-contacts",
                    IconName::ContactsUsers,
                    false,
                    0,
                    UnifiedSidebarAction::OpenContacts,
                ))
                .child(render_top_icon_btn(
                    "top-action-calls",
                    IconName::PhoneCall,
                    nav.active_destination == NavDestination::Calls,
                    nav.calls_unread,
                    UnifiedSidebarAction::SelectDestination(calls_target),
                ))
                .child(
                    div()
                        .id("sidebar-plus-action-btn")
                        .cursor_pointer()
                        .ml(px(4.0))
                        .w(px(30.0))
                        .h(px(30.0))
                        .rounded_full()
                        .bg(if is_plus_active {
                            active_bg
                        } else {
                            rgb(palette.input_bg)
                        })
                        .border_1()
                        .border_color(if is_plus_active {
                            rgb(palette.accent)
                        } else {
                            rgb(palette.border_fg)
                        })
                        .hover(move |s| s.bg(hover_bg))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(17.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(if is_plus_active {
                            rgb(palette.accent)
                        } else {
                            rgb(palette.text_primary)
                        })
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _ev, _w, cx| {
                                on_plus_click(this, UnifiedSidebarAction::TogglePlusMenu, cx);
                            }),
                        )
                        .child("+"),
                ),
        )
        .into_any_element()
}

/// Renders the inline search bar shown when searching inside the expanded sidebar.
pub fn render_expanded_gemini_search_bar<V: 'static>(
    query: &str,
    is_search_focused: bool,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, UnifiedSidebarAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let hover_bg = rgb(palette.dialogs_hover_bg);
    let search_placeholder = if is_search_focused {
        tr_str(lang_id, "| 搜索", "| Search").to_string()
    } else {
        tr_str(lang_id, "搜索", "Search").to_string()
    };
    let search_display = if query.is_empty() {
        search_placeholder
    } else if is_search_focused {
        format!("{}|", query)
    } else {
        query.to_string()
    };

    let on_back = on_action.clone();
    let on_focus = on_action.clone();
    let on_clear = on_action;

    div()
        .w_full()
        .flex_shrink_0()
        .px(px(12.0))
        .pb(px(8.0))
        .flex()
        .items_center()
        .gap(px(6.0))
        .when(is_search_focused || !query.is_empty(), |el| {
            el.child(
                div()
                    .id("sidebar-search-back-btn")
                    .cursor_pointer()
                    .w(px(30.0))
                    .h(px(30.0))
                    .rounded_full()
                    .hover(move |s| s.bg(hover_bg))
                    .flex()
                    .items_center()
                    .justify_center()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _ev, _w, cx| {
                            on_back(this, UnifiedSidebarAction::ExitSearch, cx);
                        }),
                    )
                    .child(tg_icon(IconName::BackArrow, 16.0, palette.icon_fg)),
            )
        })
        .child(
            div()
                .id("sidebar-search-box")
                .cursor_pointer()
                .flex_1()
                .h(px(SEARCH_HEIGHT))
                .bg(rgb(palette.input_bg))
                .border_1()
                .border_color(if is_search_focused {
                    rgb(palette.accent)
                } else {
                    rgb(palette.border_fg)
                })
                .rounded(px(SEARCH_RADIUS))
                .px(px(10.0))
                .flex()
                .items_center()
                .justify_between()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _ev, _w, cx| {
                        on_focus(this, UnifiedSidebarAction::FocusSearch, cx);
                    }),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .overflow_hidden()
                        .child(tg_icon(
                            IconName::Search,
                            14.0,
                            if is_search_focused {
                                palette.accent
                            } else {
                                palette.icon_fg
                            },
                        ))
                        .child(
                            div()
                                .text_size(px(13.0))
                                .text_color(if query.is_empty() {
                                    rgb(palette.text_secondary)
                                } else {
                                    rgb(palette.text_primary)
                                })
                                .child(search_display),
                        ),
                )
                .when(is_search_focused || !query.is_empty(), |el| {
                    el.child(
                        div()
                            .id("sidebar-search-clear-btn")
                            .cursor_pointer()
                            .w(px(20.0))
                            .h(px(20.0))
                            .rounded_full()
                            .hover(move |s| s.bg(hover_bg))
                            .flex()
                            .items_center()
                            .justify_center()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _ev, _w, cx| {
                                    cx.stop_propagation();
                                    on_clear(this, UnifiedSidebarAction::ClearSearch, cx);
                                }),
                            )
                            .child(tg_icon(IconName::Close, 12.0, palette.icon_fg)),
                    )
                }),
        )
        .into_any_element()
}

fn render_expanded_nav_row<V: 'static>(
    id: &'static str,
    icon: IconName,
    label: &str,
    is_active: bool,
    unread_badge: u32,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, UnifiedSidebarAction, &mut Context<V>) + 'static,
    action: UnifiedSidebarAction,
) -> AnyElement {
    let active_bg = TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.18);
    let hover_bg = rgb(palette.dialogs_hover_bg);
    let icon_color = if is_active {
        palette.accent
    } else {
        palette.icon_fg
    };
    let text_color = if is_active {
        rgb(palette.accent)
    } else {
        rgb(palette.text_primary)
    };

    div()
        .id(id)
        .cursor_pointer()
        .w_full()
        .h(px(31.0))
        .px(px(12.0))
        .rounded_full()
        .when(is_active, |el| el.bg(active_bg))
        .hover(move |s| if is_active { s } else { s.bg(hover_bg) })
        .flex()
        .items_center()
        .justify_between()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _ev, _w, cx| {
                on_action(this, action, cx);
            }),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(10.0))
                .overflow_hidden()
                .child(tg_icon(icon, 16.0, icon_color))
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(if is_active {
                            FontWeight::SEMIBOLD
                        } else {
                            FontWeight::MEDIUM
                        })
                        .text_color(text_color)
                        .child(label.to_string()),
                ),
        )
        .when(unread_badge > 0, |el| {
            el.child(
                div()
                    .h(px(17.0))
                    .min_w(px(18.0))
                    .px(px(5.0))
                    .rounded_full()
                    .bg(rgb(palette.unread_badge_bg))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(10.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(palette.unread_badge_fg))
                    .child(format_compact_badge(unread_badge)),
            )
        })
        .into_any_element()
}

/// Renders the Gemini-style primary action button, primary navigation pills, and expandable More rows.
pub fn render_expanded_gemini_nav_section<V: 'static>(
    nav: &UnifiedNavSnapshot,
    is_plus_open: bool,
    has_modal: bool,
    is_nav_more_expanded: bool,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, UnifiedSidebarAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let hover_bg = rgb(palette.dialogs_hover_bg);
    let accent_hover = rgb(palette.accent_hover);
    let plus_active = is_plus_open || has_modal;
    let effective_more_open = is_nav_more_expanded
        || matches!(
            nav.active_destination,
            NavDestination::TalkMode
                | NavDestination::RemoteComputer
                | NavDestination::SavedMessages
        );

    let on_plus = on_action.clone();
    let on_more = on_action.clone();

    let primary_items: [(&'static str, NavDestination, IconName, &'static str, &'static str, u32); 4] = [
        (
            "sidebar-nav-chats",
            NavDestination::Chats,
            IconName::FolderAll,
            "全部会话",
            "All Chats",
            nav.chats_unread,
        ),
        (
            "sidebar-nav-calls",
            NavDestination::Calls,
            IconName::PhoneCall,
            "通话",
            "Calls",
            nav.calls_unread,
        ),
        (
            "sidebar-nav-tasks",
            NavDestination::TaskCenter,
            IconName::Poll,
            "任务中心",
            "Task Center",
            nav.tasks_unread,
        ),
        (
            "sidebar-nav-mcp",
            NavDestination::McpTools,
            IconName::TerminalMcp,
            "MCP 工具",
            "MCP Tools",
            nav.mcp_unread,
        ),
    ];

    div()
        .w_full()
        .flex_shrink_0()
        .px(px(10.0))
        .pb(px(6.0))
        .flex()
        .flex_col()
        .gap(px(2.0))
        .child(
            div()
                .id("sidebar-plus-action-btn")
                .cursor_pointer()
                .w_full()
                .h(px(36.0))
                .mb(px(4.0))
                .px(px(12.0))
                .rounded_full()
                .bg(rgb(if plus_active {
                    palette.accent
                } else {
                    palette.input_bg
                }))
                .hover(move |s| {
                    s.bg(if plus_active {
                        accent_hover
                    } else {
                        hover_bg
                    })
                })
                .flex()
                .items_center()
                .justify_between()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _ev, _w, cx| {
                        on_plus(this, UnifiedSidebarAction::TogglePlusMenu, cx);
                    }),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(10.0))
                        .child(tg_icon(
                            IconName::ComposePen,
                            16.0,
                            if plus_active {
                                palette.active_button_fg
                            } else {
                                palette.text_primary
                            },
                        ))
                        .child(
                            div()
                                .text_size(px(13.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(if plus_active {
                                    palette.active_button_fg
                                } else {
                                    palette.text_primary
                                }))
                                .child(tr_str(lang_id, "发起新对话", "New Chat")),
                        ),
                )
                .child(tg_icon(
                    IconName::ChevronDown,
                    12.0,
                    if plus_active {
                        palette.active_button_fg
                    } else {
                        palette.icon_fg
                    },
                )),
        )
        .children(
            primary_items
                .into_iter()
                .map(|(id, dest, icon, zh, en, badge)| {
                    let is_active = if dest == NavDestination::Chats {
                        matches!(
                            nav.active_destination,
                            NavDestination::Chats | NavDestination::Folders
                        )
                    } else {
                        nav.active_destination == dest
                    };
                    render_expanded_nav_row(
                        id,
                        icon,
                        tr_str(lang_id, zh, en),
                        is_active,
                        badge,
                        palette,
                        cx,
                        on_action.clone(),
                        UnifiedSidebarAction::SelectDestination(dest),
                    )
                }),
        )
        .child(
            div()
                .id("sidebar-nav-more-btn")
                .cursor_pointer()
                .w_full()
                .h(px(29.0))
                .px(px(12.0))
                .rounded_full()
                .hover(move |s| s.bg(hover_bg))
                .flex()
                .items_center()
                .justify_between()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _ev, _w, cx| {
                        on_more(this, UnifiedSidebarAction::ToggleNavMore, cx);
                    }),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(10.0))
                        .child(
                            div()
                                .w(px(16.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(12.5))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(palette.icon_fg))
                                .child(if effective_more_open { "⌃" } else { "⋯" }),
                        )
                        .child(
                            div()
                                .text_size(px(12.5))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(palette.text_secondary))
                                .child(if effective_more_open {
                                    tr_str(lang_id, "收起", "Less")
                                } else {
                                    tr_str(lang_id, "更多", "More")
                                }),
                        ),
                ),
        )
        .when(effective_more_open, |el| {
            el.child(render_expanded_nav_row(
                "sidebar-nav-talk",
                IconName::Mic,
                tr_str(lang_id, "语音对话", "Talk Mode"),
                nav.active_destination == NavDestination::TalkMode,
                0,
                palette,
                cx,
                on_action.clone(),
                UnifiedSidebarAction::SelectDestination(NavDestination::TalkMode),
            ))
            .child(render_expanded_nav_row(
                "sidebar-nav-remote",
                IconName::ComputerDesktop,
                tr_str(lang_id, "远程电脑", "Remote Computer"),
                nav.active_destination == NavDestination::RemoteComputer,
                0,
                palette,
                cx,
                on_action.clone(),
                UnifiedSidebarAction::SelectDestination(NavDestination::RemoteComputer),
            ))
            .child(render_expanded_nav_row(
                "sidebar-nav-saved",
                IconName::SavedMessages,
                tr_str(lang_id, "收藏夹", "Saved Messages"),
                nav.active_destination == NavDestination::SavedMessages,
                nav.saved_unread,
                palette,
                cx,
                on_action.clone(),
                UnifiedSidebarAction::SelectDestination(NavDestination::SavedMessages),
            ))
            .child(render_expanded_nav_row(
                "sidebar-nav-contacts",
                IconName::ContactsUsers,
                tr_str(lang_id, "联系人", "Contacts"),
                false,
                0,
                palette,
                cx,
                on_action.clone(),
                UnifiedSidebarAction::OpenContacts,
            ))
        })
        .into_any_element()
}

/// Renders the pinned `52px` expanded footer with account popover trigger and add/invite pill.
pub fn render_expanded_gemini_footer<V: 'static>(
    nav: &UnifiedNavSnapshot,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, UnifiedSidebarAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let hover_bg = rgb(palette.dialogs_hover_bg);
    let avatar_grad = palette.peer_avatar_gradient(1);
    let display_name = if nav.account_name.trim().is_empty() {
        tr_str(lang_id, "我的账号", "My Account").to_string()
    } else {
        nav.account_name.clone()
    };

    let on_account = on_action.clone();
    let on_invite = on_action;

    div()
        .h(px(52.0))
        .flex_shrink_0()
        .w_full()
        .px(px(12.0))
        .border_t_1()
        .border_color(rgb(palette.border_fg))
        .flex()
        .items_center()
        .justify_between()
        .gap(px(8.0))
        .child(
            div()
                .id("sidebar-account-footer-btn")
                .cursor_pointer()
                .flex_1()
                .min_w(px(0.0))
                .px(px(6.0))
                .py(px(5.0))
                .rounded(px(10.0))
                .hover(move |s| s.bg(hover_bg))
                .flex()
                .items_center()
                .gap(px(8.0))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _ev, _w, cx| {
                        on_account(this, UnifiedSidebarAction::OpenMainMenu, cx);
                    }),
                )
                .child(
                    div()
                        .relative()
                        .w(px(30.0))
                        .h(px(30.0))
                        .flex_shrink_0()
                        .rounded_full()
                        .bg(rgb(avatar_grad.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(11.5))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xFFFFFF))
                        .child(nav.account_initials.clone())
                        .child(
                            div()
                                .absolute()
                                .bottom_0()
                                .right_0()
                                .w(px(8.5))
                                .h(px(8.5))
                                .rounded_full()
                                .bg(if nav.is_connected {
                                    rgb(palette.online_dot_fg)
                                } else {
                                    rgb(palette.muted_badge_bg)
                                })
                                .border_1()
                                .border_color(rgb(palette.dialogs_bg)),
                        ),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w(px(0.0))
                        .overflow_hidden()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(palette.text_primary))
                        .whitespace_nowrap()
                        .child(display_name),
                ),
        )
        .child(
            div()
                .id("sidebar-footer-invite-btn")
                .cursor_pointer()
                .flex_shrink_0()
                .h(px(30.0))
                .px(px(10.0))
                .rounded_full()
                .bg(rgb(palette.input_bg))
                .border_1()
                .border_color(rgb(palette.border_fg))
                .hover(move |s| s.bg(hover_bg))
                .flex()
                .items_center()
                .gap(px(5.0))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _ev, _w, cx| {
                        on_invite(this, UnifiedSidebarAction::OpenAddFriend, cx);
                    }),
                )
                .child(tg_icon(IconName::GroupUsers, 14.0, palette.icon_fg))
                .child(
                    div()
                        .text_size(px(12.5))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(palette.text_primary))
                        .child(tr_str(lang_id, "添加", "Add")),
                ),
        )
        .into_any_element()
}
