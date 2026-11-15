use crate::theme::palette::TdesktopThemePalette;
use crate::views::boxes::modal_render_gifts_stories::{
    render_send_credits, render_star_gifts, render_stories_viewer,
};
use crate::views::boxes::modal_render_media_bg::{
    render_background_picker, render_media_editor, render_translate_box,
};
use crate::views::boxes::modal_render_misc::{
    render_about_telegram, render_export_data, render_info_document, render_instant_view,
    render_language_picker, render_shortcuts_legend,
};
use crate::views::boxes::modal_render_moderation::{
    render_clear_chat_history, render_delete_messages, render_leave_or_delete_chat,
    render_pin_message, render_report_message,
};
use crate::views::boxes::modal_render_peers::{
    render_edit_peer, render_new_channel, render_new_group,
};
use crate::views::boxes::modal_render_poll_fwd::{render_create_poll, render_forward_box};
use crate::views::boxes::modal_render_security::{
    render_connection_proxy, render_edit_privacy, render_passcode_setup,
};
use crate::views::boxes::modal_theme::{DialogClose, DialogMut};
use crate::views::boxes::modal_types::ModalDialogKind;
use crate::views::modal_boxes_suite::ActiveModalDialog;
use gpui::*;

pub fn render_dialog_overlay<V: 'static>(
    dlg: &ActiveModalDialog,
    access: DialogMut<V>,
    on_close: DialogClose<V>,
    cx: &mut Context<V>,
) -> AnyElement {
    let Some(kind) = dlg.kind.clone() else {
        return div().into_any_element();
    };

    let content = match kind {
        ModalDialogKind::CreatePoll => render_create_poll(dlg, access, on_close, cx),
        ModalDialogKind::ForwardMessages {
            message_ids,
            preview_snippet,
        } => render_forward_box(dlg, message_ids, preview_snippet, access, on_close, cx),
        ModalDialogKind::NewGroup => render_new_group(dlg, access, on_close, cx),
        ModalDialogKind::NewChannel => render_new_channel(dlg, access, on_close, cx),
        ModalDialogKind::EditPeer {
            peer_title: _,
            peer_username: _,
            peer_bio: _,
            is_channel,
        } => render_edit_peer(dlg, is_channel, access, on_close, cx),
        ModalDialogKind::Contacts => {
            crate::views::boxes::modal_render_social::render_contacts_box(dlg, access, on_close, cx)
        }
        ModalDialogKind::AddContact => {
            crate::views::boxes::modal_render_social::render_add_contact(dlg, access, on_close, cx)
        }
        ModalDialogKind::CallsHistory => {
            crate::views::boxes::modal_render_social::render_calls_history(
                dlg, access, on_close, cx,
            )
        }
        ModalDialogKind::ConnectionProxy => render_connection_proxy(dlg, access, on_close, cx),
        ModalDialogKind::TranslateMessage { original_text } => {
            render_translate_box(dlg, original_text, access, on_close, cx)
        }
        ModalDialogKind::MediaEditor { is_video, .. } => {
            render_media_editor(dlg, is_video, access, on_close, cx)
        }
        ModalDialogKind::DeleteMessages {
            message_ids,
            peer_name,
        } => render_delete_messages(dlg, message_ids, peer_name, access, on_close, cx),
        ModalDialogKind::PinMessage {
            message_id,
            peer_name,
            is_group,
        } => render_pin_message(dlg, message_id, peer_name, is_group, access, on_close, cx),
        ModalDialogKind::ReportMessage {
            message_ids,
            peer_name,
        } => render_report_message(dlg, message_ids, peer_name, access, on_close, cx),
        ModalDialogKind::BackgroundPicker { peer_name } => {
            render_background_picker(dlg, peer_name, access, on_close, cx)
        }
        ModalDialogKind::PasscodeSetup => render_passcode_setup(dlg, access, on_close, cx),
        ModalDialogKind::LanguagePicker => render_language_picker(dlg, access, on_close, cx),
        ModalDialogKind::EditPrivacy { category } => {
            render_edit_privacy(dlg, category, access, on_close, cx)
        }
        ModalDialogKind::StarGifts { peer_name } => {
            render_star_gifts(dlg, peer_name, access, on_close, cx)
        }
        ModalDialogKind::SendCredits {
            invoice_title,
            stars_amount,
            balance_stars,
        } => render_send_credits(
            dlg,
            invoice_title,
            stars_amount,
            balance_stars,
            access,
            on_close,
            cx,
        ),
        ModalDialogKind::ExportData { chat_title } => {
            render_export_data(dlg, chat_title, access, on_close, cx)
        }
        ModalDialogKind::InstantView {
            title,
            url,
            body_paragraphs,
        } => render_instant_view(dlg, title, url, body_paragraphs, access, on_close, cx),
        ModalDialogKind::StoriesViewer { peer_name } => {
            render_stories_viewer(dlg, peer_name, access, on_close, cx)
        }
        ModalDialogKind::ShortcutsLegend => render_shortcuts_legend(dlg, on_close, cx),
        ModalDialogKind::AboutTelegram => render_about_telegram(dlg, access, on_close, cx),
        ModalDialogKind::InfoDocument { title, body } => {
            render_info_document(dlg, title, body, on_close, cx)
        }
        ModalDialogKind::ClearChatHistory { peer_name } => {
            render_clear_chat_history(dlg, peer_name, access, on_close, cx)
        }
        ModalDialogKind::LeaveOrDeleteChat { peer_name, is_group } => {
            render_leave_or_delete_chat(dlg, peer_name, is_group, access, on_close, cx)
        }
    };

    div()
        .absolute()
        .top_0()
        .left_0()
        .size_full()
        .bg(TdesktopThemePalette::rgba_with_alpha(0x000000, 0.58))
        .flex()
        .items_center()
        .justify_center()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _e, _w, cx| {
                on_close(this);
                cx.notify();
            }),
        )
        .child(content)
        .into_any_element()
}
