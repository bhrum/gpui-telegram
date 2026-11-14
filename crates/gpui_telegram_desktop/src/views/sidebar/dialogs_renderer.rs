//! 1:1 Telegram Desktop sidebar header, compose menu, stories strip, and archived header.

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use crate::views::chat_sidebar::{
    AddContactKind, DialogItemModel, SEARCH_HEADER_HEIGHT, SEARCH_HEIGHT, SEARCH_RADIUS,
};
use crate::views::sidebar::ui::dialogs_stories_list::DialogsStoriesListUi;
use crate::views::sidebar::ui::dialogs_suggestions::DialogsSuggestionsUi;

pub fn render_sidebar_header<V: 'static>(
    query: &str,
    is_search_focused: bool,
    is_plus_open: bool,
    has_modal: bool,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_menu_or_back: impl Fn(&mut V, &mut Context<V>) + 'static,
    on_focus_search: impl Fn(&mut V, &mut Context<V>) + 'static,
    on_clear_search: impl Fn(&mut V, &mut Context<V>) + 'static,
    on_toggle_plus: impl Fn(&mut V, &mut Context<V>) + 'static,
) -> AnyElement {
    let icon_color = palette.icon_fg;
    let hover_bg = rgb(palette.dialogs_hover_bg);
    let left_icon = if is_search_focused { IconName::BackArrow } else { IconName::HamburgerMenu };

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

    div()
        .h(px(SEARCH_HEADER_HEIGHT))
        .flex_shrink_0()
        .w_full()
        .px(px(10.0))
        .flex()
        .items_center()
        .gap(px(8.0))
        .child(
            div()
                .id("sidebar-hamburger-btn")
                .cursor_pointer()
                .w(px(32.0))
                .h(px(32.0))
                .rounded_full()
                .hover(move |s| s.bg(hover_bg))
                .flex()
                .items_center()
                .justify_center()
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                    on_menu_or_back(this, cx);
                }))
                .child(tg_icon(left_icon, 18.0, icon_color)),
        )
        .child(
            div()
                .id("sidebar-search-box")
                .cursor_pointer()
                .flex_1()
                .h(px(SEARCH_HEIGHT))
                .bg(rgb(palette.composer_bg))
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
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                    on_focus_search(this, cx);
                }))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .overflow_hidden()
                        .child(tg_icon(
                            IconName::Search,
                            14.0,
                            if is_search_focused { palette.accent } else { palette.icon_fg },
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
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                                cx.stop_propagation();
                                on_clear_search(this, cx);
                            }))
                            .child(tg_icon(IconName::Close, 12.0, icon_color)),
                    )
                }),
        )
        .child(
            div()
                .id("sidebar-plus-action-btn")
                .cursor_pointer()
                .w(px(SEARCH_HEIGHT))
                .h(px(SEARCH_HEIGHT))
                .flex_shrink_0()
                .rounded_full()
                .bg(if is_plus_open || has_modal {
                    rgb(palette.accent)
                } else {
                    rgb(palette.composer_bg)
                })
                .hover({
                    let accent_hover = rgb(palette.accent_hover);
                    move |s| s.bg(accent_hover)
                })
                .flex()
                .items_center()
                .justify_center()
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                    on_toggle_plus(this, cx);
                }))
                .child(tg_icon(
                    IconName::ComposePen,
                    16.0,
                    if is_plus_open || has_modal { palette.active_button_fg } else { palette.icon_fg },
                )),
        )
        .into_any_element()
}

pub fn render_plus_dropdown<V: 'static>(
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_add_contact: impl Fn(&mut V, AddContactKind, &mut Context<V>) + Clone + 'static,
    on_create_group: impl Fn(&mut V, &mut Context<V>) + 'static,
    on_open_contacts: impl Fn(&mut V, &mut Context<V>) + 'static,
) -> AnyElement {
    let row_bg = rgb(palette.composer_bg);
    let hover_bg = rgb(palette.dialogs_active_bg);
    let menu_row = |id: &'static str, icon: IconName, title: String, subtitle: String| {
        div()
            .id(id)
            .cursor_pointer()
            .px(px(10.0))
            .py(px(7.0))
            .rounded(px(8.0))
            .bg(row_bg)
            .hover(move |s| s.bg(hover_bg))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(9.0))
                    .child(tg_icon(icon, 16.0, palette.accent))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().text_size(px(12.5)).font_weight(FontWeight::BOLD).text_color(rgb(palette.text_primary)).child(title))
                            .child(div().text_size(px(10.5)).text_color(rgb(palette.text_secondary)).child(subtitle)),
                    ),
            )
            .child(tg_icon(IconName::ChevronRight, 12.0, palette.icon_fg))
    };

    let add_cb = on_add_contact.clone();
    let friend_cb = on_add_contact;
    div()
        .w_full()
        .px(px(10.0))
        .py(px(8.0))
        .bg(rgb(palette.menu_bg))
        .border_b_1()
        .border_color(rgb(palette.border_fg))
        .flex()
        .flex_col()
        .gap(px(5.0))
        .child(
            menu_row(
                "plus-menu-add-contact",
                IconName::BotRobot,
                tr_str(lang_id, "创建 AI Bot / 添加联系人", "Create AI Bot / Add Contact").to_string(),
                tr_str(lang_id, "通过用户名或 Bot Token 添加对话", "Start chat by username or bot handle").to_string(),
            )
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                add_cb(this, AddContactKind::AiBot, cx);
            })),
        )
        .child(
            menu_row(
                "plus-menu-add-friend",
                IconName::ContactsUsers,
                tr_str(lang_id, "添加好友", "New Private Chat").to_string(),
                tr_str(lang_id, "搜索联系人并建立私聊", "Add friend by handle or phone").to_string(),
            )
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                friend_cb(this, AddContactKind::Friend, cx);
            })),
        )
        .child(
            menu_row(
                "plus-menu-create-group",
                IconName::GroupUsers,
                tr_str(lang_id, "新建群组", "New Group").to_string(),
                tr_str(lang_id, "创建多人或协同群聊", "Create a group chat").to_string(),
            )
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                on_create_group(this, cx);
            })),
        )
        .child(
            menu_row(
                "plus-menu-contacts-modal",
                IconName::ContactsUsers,
                tr_str(lang_id, "联系人列表", "Contacts").to_string(),
                tr_str(lang_id, "查看全部联系人", "Browse saved contacts").to_string(),
            )
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                on_open_contacts(this, cx);
            })),
        )
        .into_any_element()
}

pub fn render_suggestion_bar<V: 'static>(
    suggestions: &DialogsSuggestionsUi,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_dismiss: impl Fn(&mut V, &mut Context<V>) + Clone + 'static,
) -> Option<AnyElement> {
    if !suggestions.is_visible {
        return None;
    }
    let item = suggestions.current_item.as_ref()?;
    let dismiss_cta = on_dismiss.clone();
    Some(
        div()
            .h(px(37.0))
            .flex_shrink_0()
            .w_full()
            .px(px(10.0))
            .bg(rgb(palette.composer_bg))
            .border_b_1()
            .border_color(rgb(palette.border_fg))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .overflow_hidden()
                    .child(tg_icon(IconName::PremiumStar, 13.0, palette.accent))
                    .child(div().text_size(px(12.0)).font_weight(FontWeight::BOLD).text_color(rgb(palette.text_primary)).child(item.title.clone())),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .when_some(item.button_text.clone(), |el, btn| {
                        el.child(
                            div()
                                .cursor_pointer()
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(4.0))
                                .bg(rgb(palette.accent))
                                .text_size(px(11.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(palette.active_button_fg))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                                    dismiss_cta(this, cx);
                                }))
                                .child(btn),
                        )
                    })
                    .child(
                        div()
                            .cursor_pointer()
                            .w(px(18.0))
                            .h(px(18.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                                on_dismiss(this, cx);
                            }))
                            .child(tg_icon(IconName::Close, 12.0, palette.icon_fg)),
                    ),
            )
            .into_any_element(),
    )
}

pub fn render_stories_strip<V: 'static>(
    stories: &DialogsStoriesListUi,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_toggle_expand: impl Fn(&mut V, &mut Context<V>) + 'static,
    on_click_story: impl Fn(&mut V, u64, String, &mut Context<V>) + Clone + 'static,
) -> Option<AnyElement> {
    if stories.elements.is_empty() {
        return None;
    }
    let limit = if stories.is_expanded { stories.elements.len() } else { 3 };
    let shown: Vec<_> = stories.elements.iter().take(limit).cloned().collect();
    let story_buttons: Vec<AnyElement> = shown
        .into_iter()
        .map(|s| {
            let sid = s.id;
            let peer = s.peer_name.clone();
            let cb = on_click_story.clone();
            div()
                .id(("sidebar-story-pill", sid))
                .cursor_pointer()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(2.0))
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                    cb(this, sid, peer.clone(), cx);
                }))
                .child(
                    div()
                        .w(px(30.0))
                        .h(px(30.0))
                        .rounded_full()
                        .bg(rgb(s.avatar_color))
                        .border_2()
                        .border_color(if s.unread_count > 0 { rgb(palette.accent) } else { rgb(palette.border_fg) })
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(10.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xFFFFFF))
                        .child(s.avatar_initials),
                )
                .into_any_element()
        })
        .collect();

    Some(
        div()
            .h(px(44.0))
            .flex_shrink_0()
            .w_full()
            .px(px(10.0))
            .flex()
            .items_center()
            .justify_between()
            .border_b_1()
            .border_color(rgb(palette.border_fg))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(div().text_size(px(11.0)).font_weight(FontWeight::BOLD).text_color(rgb(palette.text_secondary)).child(tr_str(lang_id, "动态", "STORIES")))
                    .children(story_buttons),
            )
            .child(
                div()
                    .id("sidebar-stories-expand-btn")
                    .cursor_pointer()
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                        on_toggle_expand(this, cx);
                    }))
                    .child(tg_icon(
                        if stories.is_expanded { IconName::ChevronDown } else { IconName::ChevronRight },
                        12.0,
                        palette.icon_fg,
                    )),
            )
            .into_any_element(),
    )
}

pub fn render_archived_header_row<V: 'static>(
    archived_dialogs: &[DialogItemModel],
    is_expanded: bool,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_toggle_archived: impl Fn(&mut V, &mut Context<V>) + 'static,
) -> Option<AnyElement> {
    if archived_dialogs.is_empty() {
        return None;
    }
    let preview: String = archived_dialogs
        .iter()
        .take(3)
        .map(|d| d.peer_name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let total_unread: u32 = archived_dialogs.iter().map(|d| d.unread_count).sum();
    let chev = if is_expanded { IconName::ChevronDown } else { IconName::ChevronRight };
    Some(
        div()
            .id("sidebar-archived-chats-row")
            .cursor_pointer()
            .h(px(56.0))
            .flex_shrink_0()
            .w_full()
            .px(px(10.0))
            .py(px(6.0))
            .bg(rgb(palette.dialogs_bg))
            .hover({
                let hover = rgb(palette.dialogs_hover_bg);
                move |s| s.bg(hover)
            })
            .border_b_1()
            .border_color(rgb(palette.border_fg))
            .flex()
            .items_center()
            .gap(px(12.0))
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                on_toggle_archived(this, cx);
            }))
            .child(
                div()
                    .w(px(42.0))
                    .h(px(42.0))
                    .flex_shrink_0()
                    .rounded_full()
                    .bg(rgb(palette.muted_badge_bg))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(tg_icon(IconName::ArchivedChats, 20.0, 0xFFFFFF)),
            )
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(div().text_size(px(13.5)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(palette.text_primary)).child(tr_str(lang_id, "已归档对话", "Archived Chats")))
                            .child(tg_icon(chev, 13.0, palette.icon_fg)),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(div().text_size(px(12.0)).text_color(rgb(palette.text_secondary)).child(preview))
                            .when(total_unread > 0, |el| {
                                el.child(
                                    div()
                                        .h(px(18.0))
                                        .min_w(px(18.0))
                                        .px(px(5.0))
                                        .rounded_full()
                                        .bg(rgb(palette.muted_badge_bg))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(10.5))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(palette.unread_badge_fg))
                                        .child(format!("{}", total_unread)),
                                )
                            }),
                    ),
            )
            .into_any_element(),
    )
}
