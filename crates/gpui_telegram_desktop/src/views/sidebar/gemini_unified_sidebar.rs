//! Gemini-inspired unified collapsible left sidebar views (`288px` expanded / `64px` collapsed rail).

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use crate::views::chat_sidebar::{DialogItemModel, COLLAPSED_WIDTH};
use crate::views::nav_rail::{NavDestination, NavRail};

pub use super::gemini_expanded_sidebar::*;

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
    SelectDialog(u64),
    OpenMainMenu,
    OpenAddFriend,
    ToggleTheme,
    Logout,
    FocusSearch,
    ExitSearch,
    ClearSearch,
    TogglePlusMenu,
    ToggleNavMore,
    OpenContacts,
    SetHover(Option<&'static str>),
    SetHoverDialog(Option<u64>),
}

pub(super) fn format_compact_badge(count: u32) -> String {
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

pub(super) fn render_floating_pill_tooltip_at(
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

/// Renders the `64px` collapsed rail with expand toggle, new-chat/contacts shortcuts, scrollable chat avatars, add-friend & account triggers.
pub fn render_collapsed_gemini_sidebar<V: 'static>(
    nav: &UnifiedNavSnapshot,
    dialogs: &[&DialogItemModel],
    active_chat_id: Option<u64>,
    hovered_item: Option<&'static str>,
    hovered_dialog_id: Option<u64>,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, UnifiedSidebarAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let toggle_hovered = hovered_item == Some("toggle");
    let compose_hovered = hovered_item == Some("compose");
    let account_hovered = hovered_item == Some("account");
    let hover_bg = rgb(palette.dialogs_hover_bg);
    let active_ring = rgb(palette.accent);
    let avatar_grad = palette.peer_avatar_gradient(1);

    let on_toggle_hover = on_action.clone();
    let on_toggle_click = on_action.clone();
    let on_compose_hover = on_action.clone();
    let on_compose_click = on_action.clone();
    let on_account_hover = on_action.clone();
    let on_account_click = on_action.clone();

    div()
        .id("unified-sidebar-collapsed")
        .relative()
        .w(px(palette.scaled_px(COLLAPSED_WIDTH).min(96.0)))
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
                .flex_shrink_0()
                .gap(px(6.0))
                .child(
                    div()
                        .id("sidebar-expand-btn")
                        .relative()
                        .w(px(36.0))
                        .h(px(36.0))
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
                        .child(tg_icon(
                            IconName::SidebarLeftOpen,
                            18.0,
                            if toggle_hovered {
                                palette.text_primary
                            } else {
                                palette.icon_fg
                            },
                        ))
                        .when(toggle_hovered, |el| {
                            el.child(render_floating_pill_tooltip(
                                tr_str(lang_id, "打开边栏", "Open sidebar"),
                                palette,
                            ))
                        }),
                )
                .child(
                    div()
                        .id("rail-compose-btn")
                        .relative()
                        .w(px(36.0))
                        .h(px(36.0))
                        .rounded_full()
                        .cursor_pointer()
                        .bg(rgb(palette.input_bg))
                        .border_1()
                        .border_color(rgb(palette.border_fg))
                        .when(compose_hovered, |el| el.bg(hover_bg))
                        .hover(move |s| s.bg(hover_bg))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(18.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(palette.text_primary))
                        .on_hover(cx.listener(move |this, hovered, _w, cx| {
                            let next = if *hovered { Some("compose") } else { None };
                            on_compose_hover(this, UnifiedSidebarAction::SetHover(next), cx);
                        }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _ev, _w, cx| {
                                on_compose_click(this, UnifiedSidebarAction::TogglePlusMenu, cx);
                            }),
                        )
                        .child("+")
                        .when(compose_hovered, |el| {
                            el.child(render_floating_pill_tooltip(
                                tr_str(lang_id, "发起新对话", "New Chat"),
                                palette,
                            ))
                        }),
                )
                .child(render_collapsed_icon_btn(
                    "rail-contacts-btn",
                    "contacts",
                    IconName::ContactsUsers,
                    tr_str(lang_id, "联系人", "Contacts"),
                    false,
                    0,
                    hovered_item,
                    palette,
                    cx,
                    on_action.clone(),
                    UnifiedSidebarAction::OpenContacts,
                ))
                .child(
                    div()
                        .w(px(24.0))
                        .h(px(1.0))
                        .my(px(2.0))
                        .bg(rgb(palette.border_fg)),
                ),
        )
        .child(
            div()
                .id("rail-dialogs-scroll")
                .flex_1()
                .min_h(px(0.0))
                .w_full()
                .overflow_y_scroll()
                .py(px(4.0))
                .flex()
                .flex_col()
                .items_center()
                .gap(px(8.0))
                .children(dialogs.iter().map(|dialog| {
                    let dialog_id = dialog.id;
                    let is_active = active_chat_id == Some(dialog_id);
                    let is_hovered = hovered_dialog_id == Some(dialog_id);
                    let avatar_bg = if dialog.avatar_color == 0 {
                        palette.peer_avatar_gradient(dialog_id as i64).0
                    } else {
                        dialog.avatar_color
                    };
                    let unread_bg = if dialog.is_muted {
                        rgb(palette.muted_badge_bg)
                    } else {
                        rgb(palette.unread_badge_bg)
                    };
                    let on_d_hover = on_action.clone();
                    let on_d_click = on_action.clone();

                    div()
                        .id(("rail-dialog-btn", dialog_id as usize))
                        .relative()
                        .w(px(42.0))
                        .h(px(42.0))
                        .rounded_full()
                        .flex_shrink_0()
                        .cursor_pointer()
                        .flex()
                        .items_center()
                        .justify_center()
                        .when(is_active, |el| el.border_2().border_color(active_ring))
                        .when(!is_active && is_hovered, |el| el.bg(hover_bg))
                        .on_hover(cx.listener(move |this, hovered, _w, cx| {
                            let next = if *hovered { Some(dialog_id) } else { None };
                            on_d_hover(this, UnifiedSidebarAction::SetHoverDialog(next), cx);
                        }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _ev, _w, cx| {
                                on_d_click(this, UnifiedSidebarAction::SelectDialog(dialog_id), cx);
                            }),
                        )
                        .child(
                            div()
                                .relative()
                                .w(px(34.0))
                                .h(px(34.0))
                                .rounded_full()
                                .bg(rgb(avatar_bg))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xFFFFFF))
                                .child(dialog.avatar_initials.clone())
                                .when(dialog.is_online, |el| {
                                    el.child(
                                        div()
                                            .absolute()
                                            .bottom(px(-1.0))
                                            .right(px(-1.0))
                                            .w(px(9.0))
                                            .h(px(9.0))
                                            .rounded_full()
                                            .bg(rgb(palette.online_dot_fg))
                                            .border_1()
                                            .border_color(rgb(palette.dialogs_bg)),
                                    )
                                })
                                .when(dialog.unread_count > 0, |el| {
                                    el.child(
                                        div()
                                            .absolute()
                                            .top(px(-3.0))
                                            .right(px(-4.0))
                                            .min_w(px(16.0))
                                            .h(px(16.0))
                                            .px(px(4.0))
                                            .rounded_full()
                                            .bg(unread_bg)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(9.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(palette.unread_badge_fg))
                                            .child(format_compact_badge(dialog.unread_count)),
                                    )
                                }),
                        )
                        .when(is_hovered, |el| {
                            el.child(render_floating_pill_tooltip(&dialog.peer_name, palette))
                        })
                })),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .flex_shrink_0()
                .pt(px(6.0))
                .gap(px(6.0))
                .child(render_collapsed_icon_btn(
                    "rail-invite-btn",
                    "invite",
                    IconName::GroupUsers,
                    tr_str(lang_id, "添加好友", "Add"),
                    false,
                    0,
                    hovered_item,
                    palette,
                    cx,
                    on_action,
                    UnifiedSidebarAction::OpenAddFriend,
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
