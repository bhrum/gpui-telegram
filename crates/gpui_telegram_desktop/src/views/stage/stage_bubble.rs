//! Complete 1:1 Telegram Desktop Message Row & Bubble Renderer (`stage_bubble`).

use gpui::*;
use gpui::prelude::FluentBuilder;
use std::collections::HashSet;

use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use super::cards::models::{
    DeliveryStatus, MessageContent, MessageDirection, MessageItemModel,
};
use super::cards_interactive::{
    render_call_card, render_inline_bot_keyboard, render_invoice_card, render_location_card,
    render_poll_card, render_story_card, InteractiveCardAction,
};
use super::cards_media::{
    render_album_mosaic_card, render_media_bubble_card, render_voice_waveform_card, MediaCardAction,
};
use super::document_contact_cards::{
    render_contact_card, render_document_card, render_webpage_preview_card,
};
use super::entities_render::{render_rich_message_body, RichTextAction};
use super::history_element::{MSG_MAX_WIDTH, MSG_MIN_WIDTH, MSG_PHOTO_SIZE, MSG_PHOTO_SKIP};
use super::rich_meta::{format_view_count, resolve_image_source, MessageRichMeta};
use super::stage_context_menu::{render_hover_reaction_pill, ContextMenuAction};

#[derive(Debug, Clone, Copy)]
pub struct BubbleGroupLayout {
    pub attached_to_prev: bool,
    pub attached_to_next: bool,
    pub show_sender_name: bool,
    pub show_bottom_avatar: bool,
    pub show_tail: bool,
    pub reserve_avatar_column: bool,
}

#[derive(Debug, Clone)]
pub enum BubbleUiAction {
    ClickReplyQuote(u64),
    ToggleReaction(u64, String),
    ToggleSelect(u64),
    OpenContextMenu(u64, Point<Pixels>),
    SetHover(Option<u64>),
    Media(MediaCardAction),
    Interactive(InteractiveCardAction),
    Context(ContextMenuAction),
    RichText(RichTextAction),
    DownloadDoc(u64, String, String),
    OpenContact(u64),
    OpenWebUrl(String),
    OpenInstantView(String, String),
}

pub fn render_message_row_1to1<V: 'static>(
    msg: &MessageItemModel,
    meta: Option<&MessageRichMeta>,
    group: BubbleGroupLayout,
    is_selected: bool,
    has_any_selection: bool,
    is_hovered: bool,
    highlight_alpha: f32,
    is_unread_anchor: bool,
    date_divider_label: Option<&str>,
    revealed_spoilers: &HashSet<(u64, usize)>,
    expanded_quotes: &HashSet<(u64, usize)>,
    copied_code_msg: Option<u64>,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, BubbleUiAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let msg_id = msg.id;
    let is_outgoing = msg.direction == MessageDirection::Outgoing;

    if let MessageContent::Service { text, .. } = &msg.content {
        return div()
            .id(SharedString::from(format!("svc-row-{}", msg_id)))
            .w_full()
            .my(px(8.0))
            .flex()
            .justify_center()
            .child(
                div()
                    .px(px(14.0))
                    .py(px(4.0))
                    .rounded_full()
                    .bg(palette.service_pill_rgba())
                    .text_size(px(12.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(palette.service_pill_fg))
                    .child(text.clone()),
            )
            .into_any_element();
    }

    let r_lg = palette.bubble_radius_px as f32;
    let r_sm = 4.0f32;
    let (tl, tr, br, bl) = if is_outgoing {
        (
            r_lg,
            if group.attached_to_prev { r_sm } else { r_lg },
            if group.attached_to_next { r_sm } else if group.show_tail { 2.0 } else { r_lg },
            r_lg,
        )
    } else {
        (
            if group.attached_to_prev { r_sm } else { r_lg },
            r_lg,
            r_lg,
            if group.attached_to_next { r_sm } else if group.show_tail { 2.0 } else { r_lg },
        )
    };

    let bubble_hex = match (is_outgoing, is_selected) {
        (true, true) => palette.msg_out_selected_bg,
        (true, false) => palette.msg_out_bg,
        (false, true) => palette.msg_in_selected_bg,
        (false, false) => palette.msg_in_bg,
    };

    let body_el = render_bubble_inner_content(
        msg, meta, is_outgoing, revealed_spoilers, expanded_quotes, copied_code_msg, palette, lang_id, cx, on_action.clone(),
    );

    let act_rclick = on_action.clone();
    let act_sel = on_action.clone();
    let act_hover = on_action.clone();
    let mut bubble = div()
        .id(SharedString::from(format!("bubble-{}", msg_id)))
        .relative()
        .min_w(px(MSG_MIN_WIDTH))
        .max_w(px(MSG_MAX_WIDTH))
        .px(px(11.0))
        .py(px(7.0))
        .rounded_tl(px(tl))
        .rounded_tr(px(tr))
        .rounded_br(px(br))
        .rounded_bl(px(bl))
        .bg(rgb(bubble_hex))
        .when(is_selected, |el| el.border_1().border_color(rgb(palette.accent)))
        .when(highlight_alpha > 0.01, |el| {
            el.border_2().border_color(TdesktopThemePalette::rgba_with_alpha(palette.accent, highlight_alpha))
        })
        .on_mouse_down(MouseButton::Right, cx.listener(move |this, ev: &MouseDownEvent, _, cx| {
            act_rclick(this, BubbleUiAction::OpenContextMenu(msg_id, ev.position), cx);
        }))
        .on_hover(cx.listener(move |this, hovered, _, cx| {
            act_hover(this, BubbleUiAction::SetHover(if *hovered { Some(msg_id) } else { None }), cx);
        }));

    if group.show_sender_name && !is_outgoing {
        let name_hex = palette.peer_name_color(msg.sender_id as i64);
        bubble = bubble.child(
            div()
                .mb(px(2.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(name_hex))
                .child(msg.sender_name.clone()),
        );
    }

    if let Some(fwd) = meta.and_then(|m| m.forward_from.as_ref()) {
        bubble = bubble.child(
            div()
                .mb(px(3.0))
                .flex()
                .items_center()
                .gap(px(4.0))
                .child(tg_icon(IconName::Forward, 12.0, palette.accent))
                .child(
                    div()
                        .text_size(px(11.5))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(palette.accent))
                        .child(format!("Forwarded from {}", fwd.sender_name)),
                ),
        );
    }

    if let Some(reply) = &msg.reply_to {
        let target_id = reply.message_id;
        let act_reply = on_action.clone();
        bubble = bubble.child(
            div()
                .id(SharedString::from(format!("reply-hdr-{}", msg_id)))
                .mb(px(5.0))
                .pl(px(8.0))
                .pr(px(8.0))
                .py(px(4.0))
                .rounded(px(6.0))
                .bg(TdesktopThemePalette::rgba_with_alpha(palette.reply_bar_bg, 0.30))
                .border_l_2()
                .border_color(rgb(palette.reply_bar_fg))
                .cursor_pointer()
                .on_click(cx.listener(move |this, _, _, cx| {
                    act_reply(this, BubbleUiAction::ClickReplyQuote(target_id), cx);
                }))
                .child(
                    div()
                        .text_size(px(11.5))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(palette.reply_bar_fg))
                        .child(reply.sender_name.clone()),
                )
                .child(
                    div()
                        .text_size(px(11.5))
                        .text_color(rgb(palette.text_secondary))
                        .child(reply.snippet.clone()),
                ),
        );
    }

    bubble = bubble
        .child(body_el)
        .child(render_bottom_meta_strip(msg, meta, is_outgoing, palette));

    if !msg.reactions.is_empty() {
        let mut rx_bar = div().flex().flex_wrap().gap(px(4.0)).mt(px(5.0));
        for rx in &msg.reactions {
            let em = rx.emoji.clone();
            let act_rx = on_action.clone();
            let bg_hex = if rx.user_reacted { palette.reaction_active_bg } else { palette.reaction_bg };
            rx_bar = rx_bar.child(
                div()
                    .id(SharedString::from(format!("rx-pill-{}-{}", msg_id, rx.emoji)))
                    .px(px(7.0))
                    .py(px(2.0))
                    .rounded_full()
                    .bg(rgb(bg_hex))
                    .when(rx.user_reacted, |el| el.border_1().border_color(rgb(palette.accent)))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        act_rx(this, BubbleUiAction::ToggleReaction(msg_id, em.clone()), cx);
                    }))
                    .text_size(px(12.0))
                    .text_color(rgb(if rx.user_reacted { 0xFFFFFF } else { palette.text_primary }))
                    .child(format!("{} {}", rx.emoji, rx.count)),
            );
        }
        bubble = bubble.child(rx_bar);
    }

    if group.show_tail {
        let tail_el = if is_outgoing {
            div().absolute().bottom(px(0.0)).right(px(-6.0)).child(tg_icon(IconName::BubbleTailOut, 12.0, bubble_hex))
        } else {
            div().absolute().bottom(px(0.0)).left(px(-6.0)).child(tg_icon(IconName::BubbleTailIn, 12.0, bubble_hex))
        };
        bubble = bubble.child(tail_el);
    }

    let mut stack = div().flex().flex_col().when(is_outgoing, |el| el.items_end());
    if is_hovered && !has_any_selection {
        let a_ctx = on_action.clone();
        stack = stack.child(render_hover_reaction_pill(msg_id, is_outgoing, palette, cx, move |this, c, cx| {
            a_ctx(this, BubbleUiAction::Context(c), cx);
        }));
    }
    stack = stack.child(bubble);
    if let Some(kb) = &msg.bot_keyboard {
        let a_bot = on_action.clone();
        stack = stack.child(render_inline_bot_keyboard(msg_id, kb, palette, cx, move |this, i, cx| {
            a_bot(this, BubbleUiAction::Interactive(i), cx);
        }));
    }

    let mut row = div()
        .id(SharedString::from(format!("msg-row-{}", msg_id)))
        .w_full()
        .px(px(16.0))
        .mt(px(if group.attached_to_prev { 2.0 } else { 7.0 }))
        .flex()
        .items_end()
        .when(has_any_selection, |el| {
            el.cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                act_sel(this, BubbleUiAction::ToggleSelect(msg_id), cx);
            }))
        })
        .when(highlight_alpha > 0.01, |el| {
            el.bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, highlight_alpha * 0.14))
        });

    if has_any_selection {
        row = row.child(
            div()
                .w(px(22.0))
                .h(px(22.0))
                .mr(px(10.0))
                .mb(px(8.0))
                .rounded_full()
                .border_2()
                .border_color(rgb(if is_selected { palette.accent } else { palette.text_secondary }))
                .when(is_selected, |el| {
                    el.bg(rgb(palette.accent))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(tg_icon(IconName::SelectCheck, 12.0, 0xFFFFFF))
                }),
        );
    }

    if !is_outgoing && group.reserve_avatar_column {
        let av_src = meta.and_then(|m| m.avatar_url.as_deref()).and_then(|u| resolve_image_source(u, None));
        row = row.child(
            div().w(px(MSG_PHOTO_SKIP)).flex().justify_start().when(group.show_bottom_avatar, |el| {
                el.child(
                    div()
                        .w(px(MSG_PHOTO_SIZE))
                        .h(px(MSG_PHOTO_SIZE))
                        .rounded_full()
                        .overflow_hidden()
                        .bg(rgb(palette.peer_avatar_color(msg.sender_id as i64)))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xFFFFFF))
                        .when_some(av_src, |a, s| a.child(img(s).size_full().object_fit(ObjectFit::Cover)))
                        .when(meta.and_then(|m| m.avatar_url.as_ref()).is_none(), |a| {
                            a.child(msg.avatar_initials.clone())
                        }),
                )
            }),
        );
    }

    let aligned_row = if is_outgoing {
        row.justify_end().child(stack)
    } else {
        row.justify_start().child(stack)
    };

    if date_divider_label.is_none() && !is_unread_anchor {
        return aligned_row.into_any_element();
    }

    div()
        .flex()
        .flex_col()
        .w_full()
        .when_some(date_divider_label, |col, label| {
            col.child(
                div().w_full().my(px(10.0)).flex().justify_center().child(
                    div()
                        .px(px(12.0))
                        .py(px(3.0))
                        .rounded_full()
                        .bg(palette.service_pill_rgba())
                        .text_size(px(12.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(palette.service_pill_fg))
                        .child(label.to_string()),
                ),
            )
        })
        .when(is_unread_anchor, |col| {
            col.child(
                div()
                    .w_full()
                    .my(px(8.0))
                    .py(px(4.0))
                    .bg(palette.service_pill_rgba())
                    .flex()
                    .justify_center()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(palette.accent))
                    .child(crate::i18n::tr_key_or(lang_id, "lng_unread_bar", "Unread Messages")),
            )
        })
        .child(aligned_row)
        .into_any_element()
}

fn render_bubble_inner_content<V: 'static>(
    msg: &MessageItemModel,
    meta: Option<&MessageRichMeta>,
    is_outgoing: bool,
    revealed_spoilers: &HashSet<(u64, usize)>,
    expanded_quotes: &HashSet<(u64, usize)>,
    copied_code_msg: Option<u64>,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, BubbleUiAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let msg_id = msg.id;
    let explicit_entities = meta.map(|m| m.entities.as_slice()).unwrap_or(&[]);

    if let Some(doc) = meta.and_then(|m| m.document.as_ref()) {
        let a_dl = on_action.clone();
        let mut col = div().flex().flex_col().gap(px(6.0)).child(render_document_card(
            msg_id, doc, palette, cx, move |this, mid, u, n, cx| {
                a_dl(this, BubbleUiAction::DownloadDoc(mid, u, n), cx);
            },
        ));
        if let MessageContent::Text { text, .. } = &msg.content {
            if !text.is_empty() && text != &doc.file_name {
                let a_rt = on_action.clone();
                col = col.child(render_rich_message_body(
                    msg_id, text, explicit_entities, revealed_spoilers, expanded_quotes,
                    copied_code_msg, palette, lang_id, is_outgoing, cx,
                    move |this, r, cx| a_rt(this, BubbleUiAction::RichText(r), cx),
                ));
            }
        }
        return col.into_any_element();
    }

    if let Some(contact) = meta.and_then(|m| m.contact.as_ref()) {
        let a_c = on_action.clone();
        return render_contact_card(msg_id, contact, palette, lang_id, cx, move |this, uid, cx| {
            a_c(this, BubbleUiAction::OpenContact(uid), cx);
        });
    }

    match &msg.content {
        MessageContent::Text { text, .. } => {
            let a_rt = on_action.clone();
            let mut col = div().flex().flex_col().gap(px(4.0)).child(render_rich_message_body(
                msg_id, text, explicit_entities, revealed_spoilers, expanded_quotes,
                copied_code_msg, palette, lang_id, is_outgoing, cx,
                move |this, r, cx| a_rt(this, BubbleUiAction::RichText(r), cx),
            ));
            if let Some(m) = meta {
                if m.show_translation {
                    if let Some(ref tr_txt) = m.translated_text {
                        col = col.child(
                            div()
                                .mt(px(4.0))
                                .pt(px(5.0))
                                .border_t_1()
                                .border_color(TdesktopThemePalette::rgba_with_alpha(palette.border_fg, 0.45))
                                .text_size(px(12.5))
                                .text_color(rgb(palette.accent))
                                .child(format!("🌐 {}", tr_txt)),
                        );
                    }
                }
                if let Some(ref wp) = m.webpage {
                    let a_url = on_action.clone();
                    let a_iv = on_action.clone();
                    col = col.child(render_webpage_preview_card(
                        msg_id, wp, palette, lang_id, cx,
                        move |this, u, cx| a_url(this, BubbleUiAction::OpenWebUrl(u), cx),
                        move |this, u, t, cx| a_iv(this, BubbleUiAction::OpenInstantView(u, t), cx),
                    ));
                }
            }
            col.into_any_element()
        }
        MessageContent::Media {
            media_type, url, width, height, duration_secs, file_size_bytes, caption, is_streaming,
        } => {
            let local_path = meta.and_then(|m| m.local_media_path.as_deref());
            let is_spoiler = meta.map_or(false, |m| m.is_spoiler_media);
            let a_m = on_action.clone();
            render_media_bubble_card(
                msg_id, *media_type, url, local_path, *width, *height, *duration_secs,
                *file_size_bytes, caption.as_deref(), *is_streaming, is_spoiler,
                revealed_spoilers, expanded_quotes, copied_code_msg, explicit_entities,
                palette, lang_id, is_outgoing, cx,
                move |this, ma, cx| a_m(this, BubbleUiAction::Media(ma), cx),
            )
        }
        MessageContent::Album { items, caption } => {
            let a_m = on_action.clone();
            render_album_mosaic_card(
                msg_id, items, caption.as_deref(), revealed_spoilers, expanded_quotes,
                copied_code_msg, explicit_entities, palette, lang_id, is_outgoing, cx,
                move |this, ma, cx| a_m(this, BubbleUiAction::Media(ma), cx),
            )
        }
        MessageContent::Voice {
            duration_secs, waveform_5bit, playback_position_secs, is_playing, playback_speed,
        } => {
            let a_m = on_action.clone();
            render_voice_waveform_card(
                msg_id, *duration_secs, waveform_5bit, *playback_position_secs, *is_playing,
                *playback_speed, palette, is_outgoing, cx,
                move |this, ma, cx| a_m(this, BubbleUiAction::Media(ma), cx),
            )
        }
        MessageContent::Poll {
            question, poll_type, options, total_votes, is_closed, selected_option_id,
        } => {
            let expl = meta.and_then(|m| m.quiz_explanation.as_deref());
            let a_i = on_action.clone();
            render_poll_card(
                msg_id, question, *poll_type, options, *total_votes, *is_closed,
                *selected_option_id, expl, palette, lang_id, cx,
                move |this, ia, cx| a_i(this, BubbleUiAction::Interactive(ia), cx),
            )
        }
        MessageContent::Location { latitude, longitude, title, address } => {
            render_location_card(*latitude, *longitude, title, address, palette)
        }
        MessageContent::Call { call_type, direction, duration_secs, status_text } => {
            let a_i = on_action.clone();
            render_call_card(msg_id, *call_type, *direction, *duration_secs, status_text, palette, cx, move |this, ia, cx| {
                a_i(this, BubbleUiAction::Interactive(ia), cx);
            })
        }
        MessageContent::Story { user_name, avatar_initials, caption, preview_color, expires_in } => {
            let a_i = on_action.clone();
            render_story_card(
                msg_id, user_name, avatar_initials, caption, *preview_color, expires_in,
                palette, cx, move |this, ia, cx| {
                    a_i(this, BubbleUiAction::Interactive(ia), cx);
                },
            )
        }
        MessageContent::Invoice { title, description, amount_stars, currency, status, receipt_id } => {
            let a_i = on_action.clone();
            render_invoice_card(
                msg_id, title, description, *amount_stars, currency, *status,
                receipt_id.as_deref(), palette, cx,
                move |this, ia, cx| a_i(this, BubbleUiAction::Interactive(ia), cx),
            )
        }
        MessageContent::BotKeyboard { rows } => {
            let a_i = on_action.clone();
            render_inline_bot_keyboard(msg_id, rows, palette, cx, move |this, ia, cx| {
                a_i(this, BubbleUiAction::Interactive(ia), cx);
            })
        }
        MessageContent::Service { text, .. } => div().text_size(px(12.0)).child(text.clone()).into_any_element(),
    }
}

fn render_bottom_meta_strip(
    msg: &MessageItemModel,
    meta: Option<&MessageRichMeta>,
    is_outgoing: bool,
    palette: &TdesktopThemePalette,
) -> AnyElement {
    let fg = if is_outgoing { palette.msg_out_date_fg } else { palette.msg_in_date_fg };
    let is_edited = matches!(msg.content, MessageContent::Text { is_edited: true, .. })
        || meta.and_then(|m| m.edit_date.as_ref()).is_some();
    let views = meta.and_then(|m| m.views_count);

    div()
        .mt(px(3.0))
        .w_full()
        .flex()
        .items_center()
        .justify_end()
        .gap(px(5.0))
        .when_some(views, |el, vc| {
            el.child(div().text_size(px(10.5)).text_color(rgb(fg)).child(format!("{} views", format_view_count(vc))))
        })
        .when(is_edited, |el| {
            el.child(div().text_size(px(10.5)).italic().text_color(rgb(fg)).child("edited"))
        })
        .child(div().text_size(px(10.5)).text_color(rgb(fg)).child(msg.timestamp.clone()))
        .when(is_outgoing, |el| {
            let (icon, tint) = match msg.delivery_status {
                DeliveryStatus::Sending => (IconName::ClockSending, fg),
                DeliveryStatus::Sent => (IconName::CheckSingle, palette.check_sent_fg),
                DeliveryStatus::Read => (IconName::CheckDouble, palette.check_read_fg),
            };
            el.child(tg_icon(icon, 13.0, tint))
        })
        .into_any_element()
}
