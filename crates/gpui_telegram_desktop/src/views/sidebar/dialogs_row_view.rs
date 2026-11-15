//! 1:1 Telegram Desktop dialog row and forum topics tree renderer.

use gpui::prelude::FluentBuilder;
use gpui::*;

use crate::i18n::tr_str;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use crate::views::chat_sidebar::{DialogDeliveryStatus, DialogItemModel};
use crate::views::sidebar::dialogs_entry::DialogsEntry;
use crate::views::sidebar::dialogs_row::DialogsRow;
use crate::views::sidebar::ui::dialogs_layout::DialogsLayoutUi;
use crate::views::sidebar::ui::dialogs_message_view::{
    DialogMessageMediaBadge, DialogsMessageViewUi,
};
use crate::views::sidebar::ui::dialogs_pill::DialogsPillUi;
use crate::views::sidebar::ui::dialogs_topics_view::DialogsTopicsViewUi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarContextAction {
    TogglePin(u64),
    ToggleMute(u64),
    ToggleArchive(u64),
    ToggleUnread(u64),
    ClearHistory(u64),
    DeleteChat(u64),
}

#[derive(Debug, Clone, Default)]
pub struct SidebarDialogMeta {
    pub is_verified: bool,
    pub is_premium: bool,
    pub is_bot: bool,
    pub is_saved_messages: bool,
    pub is_unread_marked: bool,
    pub is_archived: bool,
    pub draft_text: Option<String>,
    pub media_badge: DialogMessageMediaBadge,
    pub topics: Option<DialogsTopicsViewUi>,
    pub is_topics_expanded: bool,
    pub folder_tags: Vec<String>,
}

fn delivery_svg(
    status: DialogDeliveryStatus,
    palette: &TdesktopThemePalette,
    is_selected: bool,
) -> AnyElement {
    let icon = match status {
        DialogDeliveryStatus::Sending => IconName::ClockSending,
        DialogDeliveryStatus::Sent => IconName::CheckSingle,
        DialogDeliveryStatus::Read => IconName::CheckDouble,
    };
    let color = if is_selected {
        palette.active_button_fg
    } else if status == DialogDeliveryStatus::Read {
        palette.check_read_fg
    } else {
        palette.check_sent_fg
    };
    tg_icon(icon, 13.0, color).into_any_element()
}

fn media_badge_svg(badge: DialogMessageMediaBadge, color: u32) -> Option<AnyElement> {
    let icon = match badge {
        DialogMessageMediaBadge::None => return None,
        DialogMessageMediaBadge::Photo => IconName::Camera,
        DialogMessageMediaBadge::Video => IconName::VideoRound,
        DialogMessageMediaBadge::Document => IconName::FileDoc,
        DialogMessageMediaBadge::Audio => IconName::Play,
        DialogMessageMediaBadge::Voice => IconName::Mic,
        DialogMessageMediaBadge::Location => IconName::LocationPin,
        DialogMessageMediaBadge::Poll => IconName::Poll,
        DialogMessageMediaBadge::Contact => IconName::ContactsUsers,
        DialogMessageMediaBadge::Game => IconName::BotRobot,
    };
    Some(tg_icon(icon, 12.0, color).into_any_element())
}

pub fn render_dialog_row_item<V: 'static>(
    item: &DialogItemModel,
    index: usize,
    is_selected: bool,
    is_narrow: bool,
    meta: &SidebarDialogMeta,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_select: impl Fn(&mut V, &mut Context<V>) + 'static,
    on_context: impl Fn(&mut V, Point<Pixels>, &mut Context<V>) + 'static,
    on_toggle_topics: impl Fn(&mut V, &mut Context<V>) + 'static,
) -> AnyElement {
    let mut entry = DialogsEntry::new(item.id as i64, item.peer_name.clone());
    entry.data.is_forum = meta.topics.as_ref().is_some_and(|t| !t.topics.is_empty());
    entry.set_active(is_selected);
    let mut row_model = DialogsRow::new(entry, index, 0);
    row_model.set_selected(is_selected);
    row_model.recount_height(if is_narrow { 1.0 } else { 0.0 });
    let row_height = palette.scaled_px(row_model.height.max(62) as f32);

    let mut preview_ui = if let Some(ref draft) = meta.draft_text {
        DialogsMessageViewUi::as_draft(draft.clone())
    } else {
        DialogsMessageViewUi::new(item.last_message.clone())
    };
    preview_ui.media_badge = meta.media_badge;
    let mut pill = DialogsPillUi::new(String::new(), item.unread_count);
    pill.is_active = is_selected;
    pill.is_muted = item.is_muted;

    let row_bg = if is_selected { rgb(palette.dialogs_active_bg) } else { rgb(palette.dialogs_bg) };
    let hover_bg = rgb(palette.dialogs_hover_bg);
    let name_fg = if is_selected { rgb(palette.active_button_fg) } else { rgb(palette.text_primary) };
    let sub_fg_hex = if is_selected { palette.active_button_fg } else { palette.text_secondary };
    let time_fg = if is_selected { rgb(palette.active_button_fg) } else { rgb(palette.msg_in_date_fg) };
    let icon_hex = if is_selected { palette.active_button_fg } else { palette.icon_fg };
    let badge_bg = if is_selected {
        rgb(palette.active_button_fg)
    } else if item.is_muted {
        rgb(palette.muted_badge_bg)
    } else {
        rgb(palette.unread_badge_bg)
    };
    let badge_fg = if is_selected { rgb(palette.accent) } else { rgb(palette.unread_badge_fg) };
    let avatar_bg = if meta.is_saved_messages { palette.accent } else { item.avatar_color };
    let avatar_size = px(palette.scaled_px(46.0));
    let _ = DialogsLayoutUi::AVATAR_SIZE;

    let root = div()
        .id(("sidebar-dialog-row", item.id))
        .cursor_pointer()
        .h(px(row_height))
        .flex_shrink_0()
        .w_full()
        .px(px(palette.scaled_px(10.0)))
        .py(px(palette.scaled_px(8.0)))
        .bg(row_bg)
        .hover(move |s| if is_selected { s } else { s.bg(hover_bg) })
        .flex()
        .items_center()
        .gap(px(palette.scaled_px(12.0)))
        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
            on_select(this, cx);
        }))
        .on_mouse_down(MouseButton::Right, cx.listener(move |this, ev: &MouseDownEvent, _w, cx| {
            on_context(this, ev.position, cx);
        }));

    let mut avatar = div()
        .relative()
        .w(avatar_size)
        .h(avatar_size)
        .flex_shrink_0()
        .rounded_full()
        .bg(rgb(avatar_bg))
        .flex()
        .items_center()
        .justify_center()
        .text_color(rgb(0xFFFFFF))
        .font_weight(FontWeight::BOLD)
        .text_size(px(palette.scaled_px(15.0)));
    avatar = if meta.is_saved_messages {
        avatar.child(tg_icon(IconName::SavedMessages, palette.scaled_px(22.0), 0xFFFFFF))
    } else if meta.is_bot && item.avatar_initials.chars().all(|c| !c.is_ascii_alphanumeric()) {
        avatar.child(tg_icon(IconName::BotRobot, palette.scaled_px(20.0), 0xFFFFFF))
    } else {
        avatar.child(item.avatar_initials.clone())
    };
    if item.is_online && !meta.is_saved_messages {
        avatar = avatar.child(
            div().absolute().bottom_0().right_0().w(px(palette.scaled_px(11.0))).h(px(palette.scaled_px(11.0)))
                .rounded_full().bg(rgb(palette.online_dot_fg)).border_2().border_color(row_bg),
        );
    }

    let mut title_row = div().flex().items_center().gap(px(palette.scaled_px(4.0))).overflow_hidden();
    title_row = title_row.child(
        div().text_size(px(palette.scaled_px(14.0))).font_weight(FontWeight::SEMIBOLD).text_color(name_fg).child(item.peer_name.clone()),
    );
    if meta.is_verified {
        title_row = title_row.child(tg_icon(IconName::Verified, palette.scaled_px(14.0), if is_selected { palette.active_button_fg } else { palette.accent }));
    }
    if meta.is_premium {
        title_row = title_row.child(tg_icon(IconName::PremiumStar, palette.scaled_px(13.0), if is_selected { palette.active_button_fg } else { palette.accent }));
    }
    if item.is_muted {
        title_row = title_row.child(tg_icon(IconName::Mute, palette.scaled_px(12.0), icon_hex));
    }

    let preview_font = px(palette.scaled_px(13.0));
    let preview_line: AnyElement = if preview_ui.is_draft {
        div().flex().items_center().gap(px(3.0)).overflow_hidden()
            .child(div().text_size(preview_font).font_weight(FontWeight::SEMIBOLD).text_color(rgb(palette.danger)).child(tr_str(lang_id, "草稿:", "Draft:")))
            .child(div().text_size(preview_font).text_color(rgb(sub_fg_hex)).child(preview_ui.text))
            .into_any_element()
    } else if let Some(ref typing) = item.typing_indicator {
        div().text_size(preview_font).text_color(if is_selected { rgb(palette.active_button_fg) } else { rgb(palette.accent) }).child(typing.clone()).into_any_element()
    } else {
        let mut line = div().flex().items_center().gap(px(4.0)).overflow_hidden();
        if let Some(icon) = media_badge_svg(preview_ui.media_badge, sub_fg_hex) {
            line = line.child(icon);
        }
        line.child(div().text_size(preview_font).text_color(rgb(sub_fg_hex)).child(preview_ui.text)).into_any_element()
    };

    let mut right_badges = div().flex().items_center().gap(px(palette.scaled_px(5.0))).flex_shrink_0();
    for (tag_idx, tag_label) in meta.folder_tags.iter().enumerate() {
        let display_tag = crate::views::sidebar::dialogs_overlays::localize_folder_label(tag_label, lang_id);
        let tag_bg = if is_selected {
            TdesktopThemePalette::rgba_with_alpha(palette.active_button_fg, 0.22)
        } else {
            TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.16)
        };
        let tag_fg = if is_selected {
            rgb(palette.active_button_fg)
        } else {
            rgb(palette.accent)
        };
        right_badges = right_badges.child(
            div()
                .id(("dialog-folder-tag", item.id.saturating_mul(10) + tag_idx as u64))
                .px(px(palette.scaled_px(5.0)))
                .py(px(palette.scaled_px(1.5)))
                .rounded(px(palette.scaled_px(4.0)))
                .bg(tag_bg)
                .text_size(px(palette.scaled_px(10.0)))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(tag_fg)
                .child(display_tag),
        );
    }
    if item.is_pinned && item.unread_count == 0 && !meta.is_unread_marked {
        right_badges = right_badges.child(tg_icon(IconName::Pin, palette.scaled_px(14.0), icon_hex));
    }
    if item.unread_count > 0 {
        let pill_h = px(palette.scaled_px(19.0));
        let pill_pad = px(palette.scaled_px(5.0));
        let _ = (DialogsPillUi::HEIGHT, DialogsPillUi::PADDING);
        right_badges = right_badges.child(
            div().h(pill_h).min_w(pill_h).px(pill_pad)
                .rounded_full().bg(badge_bg).flex().items_center().justify_center()
                .text_size(px(palette.scaled_px(11.5))).font_weight(FontWeight::BOLD).text_color(badge_fg)
                .child(format!("{}", pill.count)),
        );
    } else if meta.is_unread_marked {
        right_badges = right_badges.child(div().w(px(palette.scaled_px(10.0))).h(px(palette.scaled_px(10.0))).rounded_full().bg(badge_bg));
    }

    let top_row = div().w_full().flex().items_center().justify_between().gap(px(6.0))
        .child(title_row)
        .child(div().flex().items_center().gap(px(3.0)).flex_shrink_0()
            .when(item.unread_count == 0 && meta.draft_text.is_none(), |el| el.child(delivery_svg(item.delivery_status, palette, is_selected)))
            .child(div().text_size(px(palette.scaled_px(12.0))).text_color(time_fg).child(item.timestamp.clone())));

    let mut bottom_row = div().w_full().flex().items_center().justify_between().gap(px(6.0))
        .child(div().flex_1().overflow_hidden().child(preview_line))
        .child(right_badges);
    if meta.topics.as_ref().is_some_and(|t| !t.topics.is_empty()) {
        let chev = if meta.is_topics_expanded { IconName::ChevronDown } else { IconName::ChevronRight };
        bottom_row = bottom_row.child(
            div().id(("forum-topics-toggle", item.id)).cursor_pointer().px(px(4.0)).py(px(1.0))
                .rounded(px(4.0)).bg(rgb(palette.composer_bg)).flex().items_center().gap(px(2.0))
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _ev, _w, cx| {
                    cx.stop_propagation();
                    on_toggle_topics(this, cx);
                }))
                .child(tg_icon(chev, 10.0, palette.accent)),
        );
    }

    root.child(avatar)
        .child(div().flex_1().h_full().overflow_hidden().flex().flex_col().justify_between().child(top_row).child(bottom_row))
        .into_any_element()
}

trait DraftCheckExt {
    fn is_draft_empty(&self) -> bool;
}
impl DraftCheckExt for SidebarDialogMeta {
    fn is_draft_empty(&self) -> bool {
        self.draft_text.is_some()
    }
}
