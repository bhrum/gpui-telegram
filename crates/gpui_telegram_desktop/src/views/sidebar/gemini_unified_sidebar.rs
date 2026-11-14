//! Gemini-inspired unified collapsible left sidebar views (`288px` expanded / `64px` collapsed rail).

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use crate::views::chat_sidebar::{COLLAPSED_WIDTH, SEARCH_HEIGHT, SEARCH_RADIUS};
use crate::views::nav_rail::{NavDestination, NavRail};

/// Snapshot of workspace navigation and account state rendered inside the unified sidebar.
#[derive(Debug, Clone)]
pub struct UnifiedNavSnapshot {
    pub active_destination: NavDestination,
    pub active_folder_id: Option<i32>,
    pub chats_unread: u32,
    pub calls_unread: u32,
    pub tasks_unread: u32,
    pub mcp_unread: u32,
    pub saved_unread: u32,
    pub account_initials: String,
    pub account_name: String,
    pub is_connected: bool,
}

impl UnifiedNavSnapshot {
    /// Derives a live snapshot from an optional [`NavRail`] and dialog unread counts.
    pub fn from_rail_and_unread(
        rail: Option<&NavRail>,
        standalone_dest: NavDestination,
        total_dialog_unread: u32,
    ) -> Self {
        if let Some(r) = rail {
            let badge_for = |dest: NavDestination| -> u32 {
                r.items
                    .iter()
                    .find(|it| it.destination == dest)
                    .map(|it| it.unread_badge)
                    .unwrap_or(0)
            };
            Self {
                active_destination: r.active_destination,
                active_folder_id: r.active_folder_id,
                chats_unread: badge_for(NavDestination::Chats).max(total_dialog_unread),
                calls_unread: badge_for(NavDestination::Calls),
                tasks_unread: badge_for(NavDestination::TaskCenter),
                mcp_unread: badge_for(NavDestination::McpTools),
                saved_unread: badge_for(NavDestination::SavedMessages),
                account_initials: r.account_initials.clone(),
                account_name: r.account_name.clone(),
                is_connected: r.is_connected,
            }
        } else {
            Self {
                active_destination: standalone_dest,
                active_folder_id: None,
                chats_unread: total_dialog_unread,
                calls_unread: 0,
                tasks_unread: 0,
                mcp_unread: 0,
                saved_unread: 0,
                account_initials: "TG".to_string(),
                account_name: String::new(),
                is_connected: false,
            }
        }
    }
}

/// User actions emitted by the unified Gemini-style sidebar chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnifiedSidebarAction {
    ToggleCollapsed,
    SelectDestination(NavDestination),
    OpenMainMenu,
    ToggleTheme,
    Logout,
    FocusSearch,
    ExitSearch,
    ClearSearch,
    TogglePlusMenu,
    ToggleNavMore,
    OpenContacts,
    SetHover(Option<&'static str>),
}

fn format_compact_badge(count: u32) -> String {
    if count > 999 {
        "999+".to_string()
    } else {
        count.to_string()
    }
}

/// High-contrast floating pill tooltip painted in GPUI's deferred pass above adjacent panes.
pub fn render_floating_pill_tooltip(label: &str, palette: &TdesktopThemePalette) -> Deferred {
    render_floating_pill_tooltip_at(label, 54.0, 3.0, palette)
}

fn render_floating_pill_tooltip_at(
    label: &str,
    left_px: f32,
    top_px: f32,
    palette: &TdesktopThemePalette,
) -> Deferred {
    let (pill_bg, pill_fg) = if palette.mode.is_dark() {
        (rgb(0xF1F3F4), rgb(0x1F1F1F))
    } else {
        (rgb(0x202124), rgb(0xFFFFFF))
    };

    deferred(
        div()
            .absolute()
            .left(px(left_px))
            .top(px(top_px))
            .px(px(12.0))
            .py(px(6.0))
            .rounded_full()
            .bg(pill_bg)
            .shadow_md()
            .whitespace_nowrap()
            .text_size(px(12.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(pill_fg)
            .child(label.to_string()),
    )
    .with_priority(25)
}

fn render_collapsed_icon_btn<V: 'static>(
    id: &'static str,
    hover_key: &'static str,
    icon: IconName,
    tooltip_label: &str,
    is_active: bool,
    unread_badge: u32,
    hovered_item: Option<&'static str>,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, UnifiedSidebarAction, &mut Context<V>) + Clone + 'static,
    click_action: UnifiedSidebarAction,
) -> AnyElement {
    let is_hovered = hovered_item == Some(hover_key);
    let active_bg = TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.20);
    let hover_bg = rgb(palette.dialogs_hover_bg);
    let icon_color = if is_active {
        palette.accent
    } else if is_hovered {
        palette.text_primary
    } else {
        palette.icon_fg
    };
    let on_hover_cb = on_action.clone();
    let on_click_cb = on_action;

    div()
        .id(id)
        .relative()
        .w(px(36.0))
        .h(px(36.0))
        .rounded_full()
        .cursor_pointer()
        .when(is_active, |el| el.bg(active_bg))
        .hover(move |s| if is_active { s } else { s.bg(hover_bg) })
        .flex()
        .items_center()
        .justify_center()
        .on_hover(cx.listener(move |this, hovered, _w, cx| {
            let next = if *hovered { Some(hover_key) } else { None };
            on_hover_cb(this, UnifiedSidebarAction::SetHover(next), cx);
        }))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _ev, _w, cx| {
                on_click_cb(this, click_action, cx);
            }),
        )
        .child(tg_icon(icon, 18.0, icon_color))
        .when(unread_badge > 0, |el| {
            el.child(
                div()
                    .absolute()
                    .top(px(1.0))
                    .right(px(0.0))
                    .min_w(px(16.0))
                    .h(px(16.0))
                    .px(px(4.0))
                    .rounded_full()
                    .bg(rgb(palette.unread_badge_bg))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(9.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(palette.unread_badge_fg))
                    .child(format_compact_badge(unread_badge)),
            )
        })
        .when(is_hovered, |el| {
            el.child(render_floating_pill_tooltip(tooltip_label, palette))
        })
        .into_any_element()
}

/// Renders the `64px` Gemini-style collapsed rail with hover brand-to-expand morphing and tooltips.
pub fn render_collapsed_gemini_sidebar<V: 'static>(
    nav: &UnifiedNavSnapshot,
    hovered_item: Option<&'static str>,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, UnifiedSidebarAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let toggle_hovered = hovered_item == Some("toggle");
    let account_hovered = hovered_item == Some("account");
    let hover_bg = rgb(palette.dialogs_hover_bg);
    let avatar_grad = palette.peer_avatar_gradient(1);

    let on_toggle_hover = on_action.clone();
    let on_toggle_click = on_action.clone();
    let on_account_hover = on_action.clone();
    let on_account_click = on_action.clone();

    let nav_items: [(
        &'static str,
        &'static str,
        NavDestination,
        IconName,
        &'static str,
        &'static str,
        u32,
    ); 7] = [
        (
            "rail-dest-chats",
            "chats",
            NavDestination::Chats,
            IconName::FolderAll,
            "全部会话",
            "All Chats",
            nav.chats_unread,
        ),
        (
            "rail-dest-calls",
            "calls",
            NavDestination::Calls,
            IconName::PhoneCall,
            "通话",
            "Calls",
            nav.calls_unread,
        ),
        (
            "rail-dest-tasks",
            "tasks",
            NavDestination::TaskCenter,
            IconName::Poll,
            "任务中心",
            "Task Center",
            nav.tasks_unread,
        ),
        (
            "rail-dest-mcp",
            "mcp",
            NavDestination::McpTools,
            IconName::TerminalMcp,
            "MCP 工具",
            "MCP Tools",
            nav.mcp_unread,
        ),
        (
            "rail-dest-talk",
            "talk",
            NavDestination::TalkMode,
            IconName::Mic,
            "语音对话",
            "Talk Mode",
            0,
        ),
        (
            "rail-dest-remote",
            "remote",
            NavDestination::RemoteComputer,
            IconName::ComputerDesktop,
            "远程电脑",
            "Remote Computer",
            0,
        ),
        (
            "rail-dest-saved",
            "saved",
            NavDestination::SavedMessages,
            IconName::SavedMessages,
            "收藏夹",
            "Saved Messages",
            nav.saved_unread,
        ),
    ];

    div()
        .id("unified-sidebar-collapsed")
        .relative()
        .w(px(COLLAPSED_WIDTH))
        .flex_shrink_0()
        .h_full()
        .bg(rgb(palette.dialogs_bg))
        .border_r_1()
        .border_color(rgb(palette.border_fg))
        .flex()
        .flex_col()
        .items_center()
        .justify_between()
        .py(px(10.0))
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(4.0))
                .child(
                    div()
                        .id("sidebar-expand-btn")
                        .relative()
                        .w(px(38.0))
                        .h(px(38.0))
                        .rounded_full()
                        .cursor_pointer()
                        .when(toggle_hovered, |el| el.bg(hover_bg))
                        .hover(move |s| s.bg(hover_bg))
                        .flex()
                        .items_center()
                        .justify_center()
                        .on_hover(cx.listener(move |this, hovered, _w, cx| {
                            let next = if *hovered { Some("toggle") } else { None };
                            on_toggle_hover(this, UnifiedSidebarAction::SetHover(next), cx);
                        }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _ev, _w, cx| {
                                on_toggle_click(this, UnifiedSidebarAction::ToggleCollapsed, cx);
                            }),
                        )
                        .child(if toggle_hovered {
                            tg_icon(IconName::SidebarLeftOpen, 19.0, palette.text_primary)
                        } else {
                            tg_icon(IconName::TelegramPlaneLogo, 20.0, palette.accent)
                        })
                        .when(toggle_hovered, |el| {
                            el.child(render_floating_pill_tooltip(
                                tr_str(lang_id, "打开边栏", "Open sidebar"),
                                palette,
                            ))
                        }),
                )
                .child(render_collapsed_icon_btn(
                    "rail-compose-btn",
                    "compose",
                    IconName::ComposePen,
                    tr_str(lang_id, "发起新对话", "New Chat"),
                    false,
                    0,
                    hovered_item,
                    palette,
                    cx,
                    on_action.clone(),
                    UnifiedSidebarAction::TogglePlusMenu,
                ))
                .child(render_collapsed_icon_btn(
                    "rail-search-btn",
                    "search",
                    IconName::Search,
                    tr_str(lang_id, "搜索", "Search"),
                    false,
                    0,
                    hovered_item,
                    palette,
                    cx,
                    on_action.clone(),
                    UnifiedSidebarAction::FocusSearch,
                ))
                .child(
                    div()
                        .w(px(24.0))
                        .h(px(1.0))
                        .my(px(4.0))
                        .bg(rgb(palette.border_fg)),
                )
                .children(nav_items.into_iter().map(
                    |(id, hkey, dest, icon, zh, en, badge)| {
                        let is_active = if dest == NavDestination::Chats {
                            matches!(
                                nav.active_destination,
                                NavDestination::Chats | NavDestination::Folders
                            )
                        } else {
                            nav.active_destination == dest
                        };
                        render_collapsed_icon_btn(
                            id,
                            hkey,
                            icon,
                            tr_str(lang_id, zh, en),
                            is_active,
                            badge,
                            hovered_item,
                            palette,
                            cx,
                            on_action.clone(),
                            UnifiedSidebarAction::SelectDestination(dest),
                        )
                    },
                )),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(6.0))
                .child(render_collapsed_icon_btn(
                    "rail-theme-btn",
                    "theme",
                    if palette.mode.is_dark() {
                        IconName::SunDay
                    } else {
                        IconName::NightMoon
                    },
                    tr_str(lang_id, "切换主题", "Toggle theme"),
                    false,
                    0,
                    hovered_item,
                    palette,
                    cx,
                    on_action.clone(),
                    UnifiedSidebarAction::ToggleTheme,
                ))
                .child(render_collapsed_icon_btn(
                    "rail-settings-btn",
                    "settings",
                    IconName::SettingsGear,
                    tr_str(lang_id, "设置", "Settings"),
                    nav.active_destination == NavDestination::Settings,
                    0,
                    hovered_item,
                    palette,
                    cx,
                    on_action.clone(),
                    UnifiedSidebarAction::SelectDestination(NavDestination::Settings),
                ))
                .child(
                    div()
                        .id("rail-account-btn")
                        .relative()
                        .w(px(34.0))
                        .h(px(34.0))
                        .rounded_full()
                        .bg(rgb(avatar_grad.0))
                        .hover(|s| s.bg(rgb(palette.accent_hover)))
                        .cursor_pointer()
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xFFFFFF))
                        .on_hover(cx.listener(move |this, hovered, _w, cx| {
                            let next = if *hovered { Some("account") } else { None };
                            on_account_hover(this, UnifiedSidebarAction::SetHover(next), cx);
                        }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _ev, _w, cx| {
                                on_account_click(this, UnifiedSidebarAction::OpenMainMenu, cx);
                            }),
                        )
                        .child(nav.account_initials.clone())
                        .child(
                            div()
                                .absolute()
                                .bottom_0()
                                .right_0()
                                .w(px(9.0))
                                .h(px(9.0))
                                .rounded_full()
                                .bg(if nav.is_connected {
                                    rgb(palette.online_dot_fg)
                                } else {
                                    rgb(palette.muted_badge_bg)
                                })
                                .border_1()
                                .border_color(rgb(palette.dialogs_bg)),
                        )
                        .when(account_hovered, |el| {
                            el.child(render_floating_pill_tooltip(
                                tr_str(lang_id, "账号与菜单", "Account & menu"),
                                palette,
                            ))
                        }),
                ),
        )
        .into_any_element()
}

/// Renders the `46px` expanded sidebar top brand bar with search toggle and collapse button.
pub fn render_expanded_gemini_top_bar<V: 'static>(
    is_search_active: bool,
    hovered_item: Option<&'static str>,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, UnifiedSidebarAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let hover_bg = rgb(palette.dialogs_hover_bg);
    let collapse_hovered = hovered_item == Some("collapse");
    let on_brand = on_action.clone();
    let on_search = on_action.clone();
    let on_collapse_hover = on_action.clone();
    let on_collapse_click = on_action;

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
                .id("sidebar-brand-home-btn")
                .cursor_pointer()
                .px(px(6.0))
                .py(px(4.0))
                .rounded(px(10.0))
                .hover(move |s| s.bg(hover_bg))
                .flex()
                .items_center()
                .gap(px(8.0))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _ev, _w, cx| {
                        on_brand(
                            this,
                            UnifiedSidebarAction::SelectDestination(NavDestination::Chats),
                            cx,
                        );
                    }),
                )
                .child(tg_icon(IconName::TelegramPlaneLogo, 20.0, palette.accent))
                .child(
                    div()
                        .text_size(px(15.5))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(palette.text_primary))
                        .child("Telegram"),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(6.0))
                .child(
                    div()
                        .id("sidebar-search-toggle-btn")
                        .cursor_pointer()
                        .w(px(32.0))
                        .h(px(32.0))
                        .rounded_full()
                        .when(is_search_active, |el| {
                            el.bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.18))
                        })
                        .hover(move |s| s.bg(hover_bg))
                        .flex()
                        .items_center()
                        .justify_center()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _ev, _w, cx| {
                                let act = if is_search_active {
                                    UnifiedSidebarAction::ExitSearch
                                } else {
                                    UnifiedSidebarAction::FocusSearch
                                };
                                on_search(this, act, cx);
                            }),
                        )
                        .child(tg_icon(
                            IconName::Search,
                            16.0,
                            if is_search_active {
                                palette.accent
                            } else {
                                palette.icon_fg
                            },
                        )),
                )
                .child(
                    div()
                        .id("sidebar-collapse-btn")
                        .relative()
                        .cursor_pointer()
                        .w(px(32.0))
                        .h(px(32.0))
                        .rounded_full()
                        .bg(rgb(palette.input_bg))
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
                            palette.text_primary,
                        ))
                        .when(collapse_hovered, |el| {
                            el.child(render_floating_pill_tooltip_at(
                                tr_str(lang_id, "收起边栏", "Collapse sidebar"),
                                42.0,
                                2.0,
                                palette,
                            ))
                        }),
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
        .px(px(10.0))
        .pb(px(6.0))
        .flex()
        .items_center()
        .gap(px(6.0))
        .child(
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

/// Renders the pinned `52px` expanded footer with account switcher, theme toggle, settings, and logout.
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
    let status_label = if nav.is_connected {
        tr_str(lang_id, "在线", "Online")
    } else {
        tr_str(lang_id, "离线", "Offline")
    };
    let is_settings_active = nav.active_destination == NavDestination::Settings;

    let on_account = on_action.clone();
    let on_theme = on_action.clone();
    let on_settings = on_action.clone();
    let on_logout = on_action;

    div()
        .h(px(52.0))
        .flex_shrink_0()
        .w_full()
        .px(px(10.0))
        .border_t_1()
        .border_color(rgb(palette.border_fg))
        .flex()
        .items_center()
        .justify_between()
        .gap(px(6.0))
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
                        .w(px(32.0))
                        .h(px(32.0))
                        .flex_shrink_0()
                        .rounded_full()
                        .bg(rgb(avatar_grad.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xFFFFFF))
                        .child(nav.account_initials.clone())
                        .child(
                            div()
                                .absolute()
                                .bottom_0()
                                .right_0()
                                .w(px(9.0))
                                .h(px(9.0))
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
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_size(px(12.5))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(palette.text_primary))
                                .whitespace_nowrap()
                                .overflow_hidden()
                                .child(display_name),
                        )
                        .child(
                            div()
                                .text_size(px(10.5))
                                .text_color(if nav.is_connected {
                                    rgb(palette.online_dot_fg)
                                } else {
                                    rgb(palette.text_secondary)
                                })
                                .child(status_label),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(4.0))
                .flex_shrink_0()
                .child(
                    div()
                        .id("sidebar-theme-toggle-btn")
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
                                on_theme(this, UnifiedSidebarAction::ToggleTheme, cx);
                            }),
                        )
                        .child(tg_icon(
                            if palette.mode.is_dark() {
                                IconName::SunDay
                            } else {
                                IconName::NightMoon
                            },
                            15.0,
                            palette.icon_fg,
                        )),
                )
                .child(
                    div()
                        .id("sidebar-settings-btn")
                        .cursor_pointer()
                        .w(px(30.0))
                        .h(px(30.0))
                        .rounded_full()
                        .when(is_settings_active, |el| {
                            el.bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.18))
                        })
                        .hover(move |s| if is_settings_active { s } else { s.bg(hover_bg) })
                        .flex()
                        .items_center()
                        .justify_center()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _ev, _w, cx| {
                                on_settings(
                                    this,
                                    UnifiedSidebarAction::SelectDestination(
                                        NavDestination::Settings,
                                    ),
                                    cx,
                                );
                            }),
                        )
                        .child(tg_icon(
                            IconName::SettingsGear,
                            15.0,
                            if is_settings_active {
                                palette.accent
                            } else {
                                palette.icon_fg
                            },
                        )),
                )
                .child(
                    div()
                        .id("sidebar-logout-btn")
                        .cursor_pointer()
                        .w(px(30.0))
                        .h(px(30.0))
                        .rounded_full()
                        .hover(|s| s.bg(rgba(0xE5393528)))
                        .flex()
                        .items_center()
                        .justify_center()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _ev, _w, cx| {
                                on_logout(this, UnifiedSidebarAction::Logout, cx);
                            }),
                        )
                        .child(tg_icon(IconName::BackArrow, 15.0, palette.danger)),
                ),
        )
        .into_any_element()
}
