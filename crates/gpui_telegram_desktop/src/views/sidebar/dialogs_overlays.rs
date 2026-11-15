//! 1:1 Telegram Desktop folder strip, forum topics tree, and context menu overlays.

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use crate::views::sidebar::dialogs_quick_action::{DialogsQuickAction, QuickDialogAction};
use crate::views::sidebar::dialogs_row_view::SidebarContextAction;
use crate::views::sidebar::dialogs_three_state_icon::DialogsThreeStateIcon;
use crate::views::sidebar::ui::dialogs_quick_action::QuickDialogActionType;
use crate::views::sidebar::ui::dialogs_quick_action_context::DialogsQuickActionContextUi;
use crate::views::sidebar::ui::dialogs_topics_view::DialogsTopicsViewUi;

pub fn localize_folder_label(folder: &str, lang_id: &str) -> String {
    match folder {
        "All" => tr_str(lang_id, "全部", "All").to_string(),
        "Bots" => tr_str(lang_id, "机器人", "Bots").to_string(),
        "Groups" => tr_str(lang_id, "群组", "Groups").to_string(),
        "Channels" => tr_str(lang_id, "频道", "Channels").to_string(),
        "Personal" => tr_str(lang_id, "私聊", "Personal").to_string(),
        "Work" => tr_str(lang_id, "工作", "Work").to_string(),
        "Unread" => tr_str(lang_id, "未读", "Unread").to_string(),
        other => other.to_string(),
    }
}

pub fn render_folder_tabs_strip<V: 'static>(
    folders: &[(String, u32)],
    active_folder: &str,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_select_folder: impl Fn(&mut V, String, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let icon_states = DialogsThreeStateIcon::new("folder", "folder_over", "folder_active");
    let tabs: Vec<AnyElement> = folders
        .iter()
        .enumerate()
        .map(|(idx, (folder, unread))| {
            let is_active = folder == active_folder;
            let _state_key = icon_states.select(is_active, false);
            let fname = folder.clone();
            let display_label = localize_folder_label(folder, lang_id);
            let cb = on_select_folder.clone();
            let text_hover = rgb(palette.text_primary);
            div()
                .id(("sidebar-folder-tab", idx as u64))
                .cursor_pointer()
                .relative()
                .h_full()
                .flex_shrink_0()
                .flex()
                .items_center()
                .gap(px(palette.scaled_px(4.0)))
                .px(px(palette.scaled_px(6.0)))
                .text_size(px(palette.scaled_px(13.0)))
                .font_weight(if is_active { FontWeight::BOLD } else { FontWeight::NORMAL })
                .text_color(if is_active { rgb(palette.accent) } else { rgb(palette.text_secondary) })
                .hover(move |s| if is_active { s } else { s.text_color(text_hover) })
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                    cb(this, fname.clone(), cx);
                }))
                .child(display_label)
                .when(*unread > 0, |el| {
                    el.child(
                        div()
                            .h(px(palette.scaled_px(16.0)))
                            .min_w(px(palette.scaled_px(16.0)))
                            .px(px(palette.scaled_px(4.0)))
                            .rounded_full()
                            .bg(if is_active { rgb(palette.accent) } else { rgb(palette.muted_badge_bg) })
                            .text_color(rgb(palette.unread_badge_fg))
                            .text_size(px(palette.scaled_px(10.0)))
                            .font_weight(FontWeight::BOLD)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(format!("{}", unread)),
                    )
                })
                .when(is_active, |el| {
                    el.child(div().absolute().bottom_0().left_0().right_0().h(px(2.0)).bg(rgb(palette.accent)))
                })
                .into_any_element()
        })
        .collect();

    div()
        .h(px(palette.scaled_px(36.0)))
        .flex_shrink_0()
        .w_full()
        .px(px(palette.scaled_px(8.0)))
        .flex()
        .items_center()
        .gap(px(palette.scaled_px(10.0)))
        .border_b_1()
        .border_color(rgb(palette.border_fg))
        .overflow_x_hidden()
        .children(tabs)
        .into_any_element()
}

pub fn render_forum_topics_subtree<V: 'static>(
    topics: &DialogsTopicsViewUi,
    selected_topic: Option<(u64, u64)>,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_select_topic: impl Fn(&mut V, u64, u64, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let forum_id = topics.forum_id;
    let rows: Vec<AnyElement> = topics
        .topics
        .iter()
        .filter(|t| !t.is_hidden)
        .map(|topic| {
            let tid = topic.topic_id;
            let is_sel = selected_topic == Some((forum_id, tid));
            let cb = on_select_topic.clone();
            let row_bg = if is_sel { rgb(palette.dialogs_active_bg) } else { rgb(palette.dialogs_bg) };
            let hover_bg = rgb(palette.dialogs_hover_bg);
            div()
                .id(("sidebar-topic-row", tid))
                .cursor_pointer()
                .h(px(34.0))
                .w_full()
                .pl(px(58.0))
                .pr(px(10.0))
                .bg(row_bg)
                .hover(move |s| if is_sel { s } else { s.bg(hover_bg) })
                .flex()
                .items_center()
                .justify_between()
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                    cb(this, forum_id, tid, cx);
                }))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .overflow_hidden()
                        .child(tg_icon(IconName::FolderChat, 12.0, palette.accent))
                        .child(
                            div()
                                .text_size(px(12.5))
                                .text_color(if is_sel { rgb(palette.active_button_fg) } else { rgb(palette.text_primary) })
                                .child(topic.title.clone()),
                        )
                        .when(topic.is_closed, |el| el.child(tg_icon(IconName::Lock, 11.0, palette.icon_fg))),
                )
                .when(topic.unread_count > 0, |el| {
                    el.child(
                        div()
                            .h(px(16.0))
                            .min_w(px(16.0))
                            .px(px(4.0))
                            .rounded_full()
                            .bg(rgb(palette.unread_badge_bg))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(10.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(palette.unread_badge_fg))
                            .child(format!("{}", topic.unread_count)),
                    )
                })
                .into_any_element()
        })
        .collect();
    div().w_full().border_b_1().border_color(rgb(palette.border_fg)).flex().flex_col().children(rows).into_any_element()
}

pub fn render_dialog_context_menu<V: 'static>(
    dialog_id: u64,
    anchor: Point<Pixels>,
    is_pinned: bool,
    is_muted: bool,
    is_archived: bool,
    has_unread: bool,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, SidebarContextAction, &mut Context<V>) + Clone + 'static,
    on_dismiss: impl Fn(&mut V, &mut Context<V>) + 'static,
) -> AnyElement {
    let mut qa_pin = DialogsQuickAction::new(dialog_id as i64, QuickDialogAction::Pin);
    qa_pin.update_state(false, is_muted, is_pinned, has_unread, is_archived, true, false);
    let _ctx_pin = DialogsQuickActionContextUi::new(dialog_id, QuickDialogActionType::Pin);

    let x = f32::from(anchor.x).clamp(8.0, 92.0);
    let y = f32::from(anchor.y).clamp(52.0, 420.0);
    let hover = rgb(palette.menu_hover_bg);

    let items: [(IconName, String, SidebarContextAction, bool); 6] = [
        (IconName::Pin, if is_pinned { tr_str(lang_id, "取消置顶", "Unpin").into() } else { tr_str(lang_id, "置顶对话", "Pin").into() }, SidebarContextAction::TogglePin(dialog_id), false),
        (IconName::Mute, if is_muted { tr_str(lang_id, "取消静音", "Unmute").into() } else { tr_str(lang_id, "静音通知", "Mute").into() }, SidebarContextAction::ToggleMute(dialog_id), false),
        (IconName::CheckDouble, if has_unread { tr_str(lang_id, "标记为已读", "Mark as Read").into() } else { tr_str(lang_id, "标记为未读", "Mark as Unread").into() }, SidebarContextAction::ToggleUnread(dialog_id), false),
        (IconName::ArchivedChats, if is_archived { tr_str(lang_id, "取消归档", "Unarchive").into() } else { tr_str(lang_id, "归档对话", "Archive").into() }, SidebarContextAction::ToggleArchive(dialog_id), false),
        (IconName::Close, tr_str(lang_id, "清空历史记录", "Clear History").into(), SidebarContextAction::ClearHistory(dialog_id), false),
        (IconName::DeleteTrash, tr_str(lang_id, "删除对话", "Delete Chat").into(), SidebarContextAction::DeleteChat(dialog_id), true),
    ];

    let rows: Vec<AnyElement> = items
        .into_iter()
        .enumerate()
        .map(|(idx, (icon, label, act, danger))| {
            let cb = on_action.clone();
            let fg = if danger { palette.danger } else { palette.text_primary };
            let ic = if danger { palette.danger } else { palette.icon_fg };
            div()
                .id(("sidebar-ctx-item", idx as u64))
                .cursor_pointer()
                .h(px(32.0))
                .px(px(10.0))
                .rounded(px(6.0))
                .hover(move |s| s.bg(hover))
                .flex()
                .items_center()
                .gap(px(8.0))
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                    cx.stop_propagation();
                    cb(this, act, cx);
                }))
                .child(tg_icon(icon, 14.0, ic))
                .child(div().text_size(px(12.5)).text_color(rgb(fg)).child(label))
                .into_any_element()
        })
        .collect();

    div()
        .id("sidebar-context-backdrop")
        .absolute()
        .inset_0()
        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
            on_dismiss(this, cx);
        }))
        .child(
            div()
                .id("sidebar-context-menu")
                .absolute()
                .left(px(x))
                .top(px(y))
                .w(px(180.0))
                .p(px(4.0))
                .rounded(px(8.0))
                .bg(rgb(palette.menu_bg))
                .border_1()
                .border_color(rgb(palette.border_fg))
                .shadow_md()
                .flex()
                .flex_col()
                .children(rows),
        )
        .into_any_element()
}
