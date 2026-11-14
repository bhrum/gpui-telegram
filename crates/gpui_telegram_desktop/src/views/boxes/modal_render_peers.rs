use crate::i18n::tr_str;
use crate::theme::assets::IconName;
use crate::views::boxes::add_contact_box::{K_MAX_CHANNEL_DESCRIPTION, K_MAX_GROUP_CHANNEL_TITLE};
use crate::views::boxes::modal_theme::{
    modal_card, modal_error_banner, modal_field, modal_footer_cancel, modal_header,
    modal_toggle_chip, DialogClose, DialogMut,
};
use crate::views::boxes::modal_types::{ActiveTextField, ModalActionOutcome};
use crate::views::modal_boxes_suite::ActiveModalDialog;
use gpui::*;

pub fn render_new_group<V: 'static>(
    dlg: &ActiveModalDialog,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;

    modal_card(420.0, pal)
        .child(modal_header(
            IconName::GroupUsers,
            tr_str(lang, "新建群组", "New Group"),
            pal,
            on_close,
            cx,
        ))
        .child(modal_field(
            tr_str(lang, "群组名称", "Group Name"),
            &dlg.primary_input,
            tr_str(lang, "输入群组名称...", "Enter group name..."),
            ActiveTextField::Primary,
            dlg.active_field == ActiveTextField::Primary,
            pal,
            access,
            cx,
        ))
        .child(modal_toggle_chip(
            tr_str(lang, "启用话题分组 (Topics)", "Enable Forum Topics"),
            dlg.flag_a,
            pal,
            move |this| {
                let d = access(this);
                d.flag_a = !d.flag_a;
            },
            cx,
        ))
        .child(modal_error_banner(dlg.validation_error.as_ref(), pal))
        .child(
            div()
                .flex()
                .justify_end()
                .gap(px(8.0))
                .child(modal_footer_cancel(lang, pal, on_close, cx))
                .child(
                    div()
                        .cursor_pointer()
                        .px(px(16.0))
                        .py(px(7.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.accent))
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.active_button_fg))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                let d = access(this);
                                let title: String = d
                                    .primary_input
                                    .trim()
                                    .chars()
                                    .take(K_MAX_GROUP_CHANNEL_TITLE)
                                    .collect();
                                if title.is_empty() {
                                    d.validation_error = Some("Group name is required".into());
                                    cx.notify();
                                    return;
                                }
                                d.pending_outcome = Some(ModalActionOutcome::CreateGroupSubmitted {
                                    title,
                                    members_count: d.selected_members.len(),
                                    enable_forum_topics: d.flag_a,
                                });
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "创建群组", "Create").to_string()),
                ),
        )
        .into_any_element()
}

pub fn render_new_channel<V: 'static>(
    dlg: &ActiveModalDialog,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;

    modal_card(420.0, pal)
        .child(modal_header(
            IconName::ChannelMegaphone,
            tr_str(lang, "新建频道", "New Channel"),
            pal,
            on_close,
            cx,
        ))
        .child(modal_field(
            tr_str(lang, "频道名称", "Channel Name"),
            &dlg.primary_input,
            tr_str(lang, "输入频道名称...", "Channel title..."),
            ActiveTextField::Primary,
            dlg.active_field == ActiveTextField::Primary,
            pal,
            access,
            cx,
        ))
        .child(modal_field(
            tr_str(lang, "公开链接用户名 (可选)", "Public Link Username"),
            &dlg.secondary_input,
            "channel_username",
            ActiveTextField::Secondary,
            dlg.active_field == ActiveTextField::Secondary,
            pal,
            access,
            cx,
        ))
        .child(modal_toggle_chip(
            tr_str(lang, "公开频道", "Public Channel"),
            dlg.flag_a,
            pal,
            move |this| {
                let d = access(this);
                d.flag_a = !d.flag_a;
            },
            cx,
        ))
        .child(modal_error_banner(dlg.validation_error.as_ref(), pal))
        .child(
            div()
                .flex()
                .justify_end()
                .gap(px(8.0))
                .child(modal_footer_cancel(lang, pal, on_close, cx))
                .child(
                    div()
                        .cursor_pointer()
                        .px(px(16.0))
                        .py(px(7.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.accent))
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.active_button_fg))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                let d = access(this);
                                let title: String = d
                                    .primary_input
                                    .trim()
                                    .chars()
                                    .take(K_MAX_GROUP_CHANNEL_TITLE)
                                    .collect();
                                if title.is_empty() {
                                    d.validation_error = Some("Channel name is required".into());
                                    cx.notify();
                                    return;
                                }
                                let slug = d.secondary_input.trim().trim_start_matches('@').to_string();
                                d.pending_outcome = Some(ModalActionOutcome::CreateChannelSubmitted {
                                    title,
                                    username_slug: slug,
                                    is_public: d.flag_a,
                                });
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "创建频道", "Create").to_string()),
                ),
        )
        .into_any_element()
}

pub fn render_edit_peer<V: 'static>(
    dlg: &ActiveModalDialog,
    is_channel: bool,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let pal = dlg.palette;
    let lang = &dlg.language_id;
    let heading = if is_channel {
        tr_str(lang, "编辑频道", "Edit Channel")
    } else {
        tr_str(lang, "编辑群组", "Edit Group")
    };

    modal_card(420.0, pal)
        .child(modal_header(IconName::Edit, heading, pal, on_close, cx))
        .child(modal_field(
            tr_str(lang, "名称", "Title"),
            &dlg.primary_input,
            "Title...",
            ActiveTextField::Primary,
            dlg.active_field == ActiveTextField::Primary,
            pal,
            access,
            cx,
        ))
        .child(modal_field(
            tr_str(lang, "公开链接", "Username"),
            &dlg.secondary_input,
            "@username",
            ActiveTextField::Secondary,
            dlg.active_field == ActiveTextField::Secondary,
            pal,
            access,
            cx,
        ))
        .child(modal_field(
            tr_str(lang, "简介描述", "Description"),
            &dlg.tertiary_input,
            "Description...",
            ActiveTextField::Tertiary,
            dlg.active_field == ActiveTextField::Tertiary,
            pal,
            access,
            cx,
        ))
        .child(
            div()
                .flex()
                .justify_end()
                .gap(px(8.0))
                .child(modal_footer_cancel(lang, pal, on_close, cx))
                .child(
                    div()
                        .cursor_pointer()
                        .px(px(16.0))
                        .py(px(7.0))
                        .rounded(px(8.0))
                        .bg(rgb(pal.accent))
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(pal.active_button_fg))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                let d = access(this);
                                let title = d.primary_input.trim().to_string();
                                if title.is_empty() {
                                    d.validation_error = Some("Title cannot be empty".into());
                                    cx.notify();
                                    return;
                                }
                                let bio: String = d
                                    .tertiary_input
                                    .trim()
                                    .chars()
                                    .take(K_MAX_CHANNEL_DESCRIPTION)
                                    .collect();
                                d.pending_outcome = Some(ModalActionOutcome::SavePeerEdit {
                                    title,
                                    username: d.secondary_input.trim().trim_start_matches('@').to_string(),
                                    bio,
                                });
                                on_close(this);
                                cx.notify();
                            }),
                        )
                        .child(tr_str(lang, "保存", "Save").to_string()),
                ),
        )
        .into_any_element()
}
