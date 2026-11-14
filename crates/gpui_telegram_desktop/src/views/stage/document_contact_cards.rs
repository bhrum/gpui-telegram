//! Rich Cards for WebPage Previews (with Instant View trigger), Documents/Files & Contacts.

use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::i18n::tr_key_or;
use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use super::rich_meta::{
    format_file_size, resolve_image_source, ContactMeta, DocumentMeta, WebPageMeta,
};

pub fn render_webpage_preview_card<V: 'static>(
    msg_id: u64,
    wp: &WebPageMeta,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_open_url: impl Fn(&mut V, String, &mut Context<V>) + 'static,
    on_open_iv: impl Fn(&mut V, String, String, &mut Context<V>) + 'static,
) -> AnyElement {
    let url_open = wp.url.clone();
    let url_iv = wp.url.clone();
    let title_iv = wp.title.clone();
    let site_label = wp
        .site_name
        .clone()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "Web Preview".to_string());
    let has_iv = wp.has_instant_view;

    div()
        .mt(px(6.0))
        .pl(px(10.0))
        .pr(px(8.0))
        .py(px(6.0))
        .rounded(px(8.0))
        .bg(TdesktopThemePalette::rgba_with_alpha(palette.reply_bar_bg, 0.25))
        .border_l_2()
        .border_color(rgb(palette.accent))
        .flex()
        .flex_col()
        .gap(px(4.0))
        .child(
            div()
                .flex()
                .items_start()
                .justify_between()
                .gap(px(8.0))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .gap(px(2.0))
                        .child(
                            div()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(palette.accent))
                                .child(site_label),
                        )
                        .when(!wp.title.is_empty(), |el| {
                            el.child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(palette.text_primary))
                                    .child(wp.title.clone()),
                            )
                        })
                        .when(!wp.description.is_empty(), |el| {
                            el.child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(palette.text_secondary))
                                    .child(wp.description.clone()),
                            )
                        }),
                )
                .when_some(
                    wp.thumb_url
                        .as_deref()
                        .and_then(|u| resolve_image_source(u, None)),
                    |el, img_src| {
                        el.child(
                            img(img_src)
                                .w(px(56.0))
                                .h(px(56.0))
                                .rounded(px(6.0))
                                .object_fit(ObjectFit::Cover),
                        )
                    },
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(6.0))
                .mt(px(2.0))
                .when(has_iv, |el| {
                    el.child(
                        div()
                            .id(SharedString::from(format!("iv-open-{}", msg_id)))
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.16))
                            .border_1()
                            .border_color(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.4))
                            .cursor_pointer()
                            .hover(|s| s.bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.28)))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                on_open_iv(this, url_iv.clone(), title_iv.clone(), cx);
                            }))
                            .text_size(px(11.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(palette.accent))
                            .child(tr_key_or(lang_id, "lng_view_button_iv", "INSTANT VIEW")),
                    )
                })
                .child(
                    div()
                        .id(SharedString::from(format!("wp-url-{}", msg_id)))
                        .px(px(8.0))
                        .py(px(4.0))
                        .rounded(px(6.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.12)))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            on_open_url(this, url_open.clone(), cx);
                        }))
                        .text_size(px(11.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(palette.text_link))
                        .child(tr_key_or(lang_id, "lng_open_link", "OPEN LINK")),
                ),
        )
        .into_any_element()
}

pub fn render_document_card<V: 'static>(
    msg_id: u64,
    doc: &DocumentMeta,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_download: impl Fn(&mut V, u64, String, String, &mut Context<V>) + 'static,
) -> AnyElement {
    let ext = doc
        .file_name
        .rsplit('.')
        .next()
        .unwrap_or("FILE")
        .to_uppercase();
    let badge_color = match ext.as_str() {
        "PDF" => 0xE53935,
        "ZIP" | "TAR" | "GZ" | "7Z" => 0xF59E0B,
        "DOC" | "DOCX" | "TXT" | "MD" | "RS" => 0x2AABEE,
        "MP3" | "FLAC" | "WAV" | "OGG" => 0x8B5CF6,
        _ => palette.accent,
    };
    let dl_url = doc.download_url.clone();
    let dl_name = doc.file_name.clone();
    let sub_line = if doc.is_downloading {
        format!("{} · {:.0}%", format_file_size(doc.file_size_bytes), doc.download_progress * 100.0)
    } else if doc.local_path.is_some() {
        format!("{} · Ready", format_file_size(doc.file_size_bytes))
    } else {
        format_file_size(doc.file_size_bytes)
    };

    div()
        .id(SharedString::from(format!("doc-card-{}", msg_id)))
        .flex()
        .items_center()
        .gap(px(10.0))
        .p(px(6.0))
        .rounded(px(8.0))
        .cursor_pointer()
        .hover(|s| s.bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.08)))
        .on_click(cx.listener(move |this, _, _, cx| {
            on_download(this, msg_id, dl_url.clone(), dl_name.clone(), cx);
        }))
        .child(
            div()
                .w(px(42.0))
                .h(px(42.0))
                .rounded(px(10.0))
                .bg(rgb(badge_color))
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .child(tg_icon(IconName::FileDoc, 18.0, 0xFFFFFF))
                .child(
                    div()
                        .text_size(px(8.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xFFFFFF))
                        .child(ext.chars().take(4).collect::<String>()),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .overflow_hidden()
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(palette.text_primary))
                        .child(doc.file_name.clone()),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(rgb(palette.text_secondary))
                        .child(sub_line),
                ),
        )
        .into_any_element()
}

pub fn render_contact_card<V: 'static>(
    msg_id: u64,
    contact: &ContactMeta,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_open_contact: impl Fn(&mut V, u64, &mut Context<V>) + 'static,
) -> AnyElement {
    let full_name = if contact.last_name.is_empty() {
        contact.first_name.clone()
    } else {
        format!("{} {}", contact.first_name, contact.last_name)
    };
    let initial = full_name.chars().next().unwrap_or('?').to_uppercase().to_string();
    let uid = contact.user_id.unwrap_or(msg_id);
    let avatar_bg = palette.peer_avatar_color(uid as i64);

    div()
        .flex()
        .items_center()
        .justify_between()
        .gap(px(10.0))
        .min_w(px(230.0))
        .p(px(4.0))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(10.0))
                .child(
                    div()
                        .w(px(40.0))
                        .h(px(40.0))
                        .rounded_full()
                        .bg(rgb(avatar_bg))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(15.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xFFFFFF))
                        .child(initial),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_size(px(13.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(palette.text_primary))
                                .child(full_name),
                        )
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(palette.text_secondary))
                                .child(contact.phone_number.clone()),
                        ),
                ),
        )
        .child(
            div()
                .id(SharedString::from(format!("contact-msg-{}", msg_id)))
                .px(px(10.0))
                .py(px(5.0))
                .rounded(px(6.0))
                .bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.16))
                .cursor_pointer()
                .hover(|s| s.bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.28)))
                .on_click(cx.listener(move |this, _, _, cx| {
                    on_open_contact(this, uid, cx);
                }))
                .text_size(px(11.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(palette.accent))
                .child(tr_key_or(lang_id, "lng_profile_send_message", "Message")),
        )
        .into_any_element()
}
