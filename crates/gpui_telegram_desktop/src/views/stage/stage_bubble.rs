//! Complete 1:1 Telegram Desktop Message Row & Bubble Renderer (`stage_bubble`).

use gpui::*;
use gpui::prelude::FluentBuilder;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

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
use super::entities::{parse_rich_blocks, push_sanitized_no_breaks, RichBlock, StyledSegment};
use super::entities_render::{render_rich_message_body, RichTextAction};
use super::history_element::{MSG_MAX_WIDTH, MSG_MIN_WIDTH, MSG_PHOTO_SIZE, MSG_PHOTO_SKIP};
use super::reactions::button::{sort_reaction_pills, ReactionKey, ReactionPill};
use super::rich_meta::{format_view_count, resolve_image_source, MessageRichMeta};
use super::stage_context_menu::{
    render_hover_reaction_overlay, ContextMenuAction, ReactionOverlayLayout, ReactionOverlayMode,
};

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
    ToggleReactionPicker(u64),
    ToggleSelect(u64),
    OpenContextMenu(u64, Point<Pixels>),
    SetHover(Option<u64>),
    SetBubbleHover(u64, bool),
    SetBubbleGutterHover(u64, bool),
    SetReactionBarHover(u64, bool),
    SetReactionBridgeHover(u64, bool),
    Media(MediaCardAction),
    Interactive(InteractiveCardAction),
    Context(ContextMenuAction),
    RichText(RichTextAction),
    DownloadDoc(u64, String, String),
    OpenContact(u64),
    OpenWebUrl(String),
    OpenInstantView(String, String),
}

fn make_row_height_probe_canvas(
    msg_id: u64,
    probe: Rc<RefCell<HashMap<u64, f32>>>,
) -> impl IntoElement {
    canvas(
        move |bounds, _window, _cx| {
            let h = f32::from(bounds.size.height);
            if h > 1.0 {
                probe.borrow_mut().insert(msg_id, h);
            }
        },
        |_, _, _, _| {},
    )
    .absolute()
    .inset_0()
    .size_full()
}

fn measure_single_line_width_px(
    text: &str,
    font_px: f32,
    weight: FontWeight,
    window: &Window,
) -> f32 {
    if text.is_empty() {
        return 0.0;
    }
    let mut clean = String::new();
    push_sanitized_no_breaks(&mut clean, text);
    if clean.is_empty() {
        return 0.0;
    }
    let mut style = window.text_style();
    style.font_weight = weight;
    let run = style.to_run(clean.len());
    let shaped = window
        .text_system()
        .shape_line(SharedString::from(clean), px(font_px), &[run], None);
    f32::from(shaped.width).ceil()
}

fn measure_rich_line_width_px(
    segs: &[StyledSegment],
    font_px: f32,
    default_weight: FontWeight,
    revealed_spoilers: &HashSet<(u64, usize)>,
    msg_id: u64,
    window: &Window,
) -> f32 {
    if segs.is_empty() {
        return 0.0;
    }
    let base_style = window.text_style();
    let mut full = String::new();
    let mut runs = Vec::new();
    for (seg_idx, seg) in segs.iter().enumerate() {
        let start = full.len();
        if seg.flags.spoiler {
            let sid = seg.spoiler_id.unwrap_or(seg_idx);
            if revealed_spoilers.contains(&(msg_id, sid)) {
                push_sanitized_no_breaks(&mut full, &seg.text);
            } else {
                let mask_len = seg.text.chars().count().clamp(4, 18);
                for _ in 0..mask_len {
                    full.push('░');
                }
            }
        } else if seg.flags.code {
            let mut clean = String::new();
            push_sanitized_no_breaks(&mut clean, &seg.text);
            if !clean.is_empty() {
                full.push(' ');
                full.push_str(&clean);
                full.push(' ');
            }
        } else {
            push_sanitized_no_breaks(&mut full, &seg.text);
        }
        let seg_len = full.len() - start;
        if seg_len > 0 {
            let mut style = base_style.clone();
            style.font_weight = if seg.flags.bold {
                FontWeight::BOLD
            } else if seg.flags.code && default_weight == FontWeight::NORMAL {
                FontWeight::MEDIUM
            } else {
                default_weight
            };
            if seg.flags.italic {
                style.font_style = FontStyle::Italic;
            }
            runs.push(style.to_run(seg_len));
        }
    }
    if full.is_empty() || runs.is_empty() {
        return 0.0;
    }
    let shaped = window
        .text_system()
        .shape_line(SharedString::from(full), px(font_px), &runs, None);
    f32::from(shaped.width).ceil()
}

#[allow(clippy::too_many_arguments)]
fn compute_definite_bubble_width(
    msg: &MessageItemModel,
    meta: Option<&MessageRichMeta>,
    group: BubbleGroupLayout,
    is_outgoing: bool,
    revealed_spoilers: &HashSet<(u64, usize)>,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    eff_min_w: f32,
    eff_max_w: f32,
    window: &Window,
) -> Option<f32> {
    if meta.is_some_and(|m| m.contact.is_some() || m.document.is_some()) {
        return None;
    }
    match &msg.content {
        MessageContent::Text { .. } | MessageContent::BotKeyboard { .. } => {}
        _ => return None,
    }

    let horiz_pad = palette.scaled_px(22.0) + 6.0;
    let mut needed_w = eff_min_w;

    let meta_font = palette.scaled_px(10.5);
    let is_zh = lang_id.starts_with("zh");
    let is_edited = matches!(msg.content, MessageContent::Text { is_edited: true, .. })
        || meta.and_then(|m| m.edit_date.as_ref()).is_some();
    let views = meta.and_then(|m| m.views_count);
    let mut meta_w = measure_single_line_width_px(&msg.timestamp, meta_font, FontWeight::NORMAL, window)
        + horiz_pad
        + 6.0;
    if is_outgoing {
        meta_w += palette.scaled_px(18.0);
    }
    if is_edited {
        let elabel = if is_zh { "已编辑" } else { "edited" };
        meta_w += measure_single_line_width_px(elabel, meta_font, FontWeight::NORMAL, window)
            + palette.scaled_px(5.0);
    }
    if let Some(vc) = views {
        let vlabel = if is_zh {
            format!("{} 次浏览", format_view_count(vc))
        } else {
            format!("{} views", format_view_count(vc))
        };
        meta_w += measure_single_line_width_px(&vlabel, meta_font, FontWeight::NORMAL, window)
            + palette.scaled_px(5.0);
    }
    needed_w = needed_w.max(meta_w);

    if group.show_sender_name && !is_outgoing {
        needed_w = needed_w.max(
            measure_single_line_width_px(&msg.sender_name, 12.0, FontWeight::BOLD, window) + horiz_pad,
        );
    }
    if let Some(fwd) = meta.and_then(|m| m.forward_from.as_ref()) {
        let fwd_label = if is_zh {
            format!("转发自 {}", fwd.sender_name)
        } else {
            format!("Forwarded from {}", fwd.sender_name)
        };
        let fwd_w = measure_single_line_width_px(
            &fwd_label,
            palette.scaled_px(11.5),
            FontWeight::SEMIBOLD,
            window,
        ) + palette.scaled_px(16.0)
            + horiz_pad;
        needed_w = needed_w.max(fwd_w);
    }
    if let Some(reply) = &msg.reply_to {
        let reply_w = measure_single_line_width_px(&reply.sender_name, 11.5, FontWeight::BOLD, window)
            .max(measure_single_line_width_px(&reply.snippet, 11.5, FontWeight::NORMAL, window))
            + 20.0
            + horiz_pad;
        needed_w = needed_w.max(reply_w);
    }
    if !msg.reactions.is_empty() {
        let rx_w = (msg.reactions.len() as f32) * 52.0 + horiz_pad;
        needed_w = needed_w.max(rx_w);
    }

    let kb_rows_opt = match &msg.content {
        MessageContent::BotKeyboard { rows } => Some(rows.as_slice()),
        _ => msg.bot_keyboard.as_deref(),
    };
    if let Some(kb_rows) = kb_rows_opt {
        for r in kb_rows {
            if r.is_empty() {
                continue;
            }
            let row_sum: f32 = r
                .iter()
                .map(|btn| {
                    measure_single_line_width_px(&btn.text, 12.0, FontWeight::SEMIBOLD, window)
                        + 28.0
                })
                .sum::<f32>()
                + ((r.len().saturating_sub(1)) as f32 * 4.0);
            needed_w = needed_w.max(row_sum);
            if needed_w >= eff_max_w {
                return Some(eff_max_w);
            }
        }
    }

    if let MessageContent::Text { text, .. } = &msg.content {
        if meta.is_some_and(|m| m.webpage.is_some() || m.show_translation) {
            return Some(eff_max_w);
        }
        let explicit_entities = meta.map(|m| m.entities.as_slice()).unwrap_or(&[]);
        let blocks = parse_rich_blocks(text, explicit_entities);
        let font_px = palette.scaled_px(palette.font_size_px as f32);
        let q_font = palette.scaled_px((palette.font_size_px as f32 - 0.5).max(12.0));

        for block in blocks {
            match block {
                RichBlock::PreCode { .. } | RichBlock::Divider => return Some(eff_max_w),
                RichBlock::Paragraph(segs) => {
                    let line_w = measure_rich_line_width_px(
                        &segs,
                        font_px,
                        FontWeight::NORMAL,
                        revealed_spoilers,
                        msg.id,
                        window,
                    ) + horiz_pad;
                    needed_w = needed_w.max(line_w);
                }
                RichBlock::Heading { level, segments } => {
                    let scale = match level {
                        1 => 1.24,
                        2 => 1.16,
                        3 => 1.09,
                        _ => 1.03,
                    };
                    let h_px = (font_px * scale).round();
                    let line_w = measure_rich_line_width_px(
                        &segments,
                        h_px,
                        FontWeight::BOLD,
                        revealed_spoilers,
                        msg.id,
                        window,
                    ) + horiz_pad;
                    needed_w = needed_w.max(line_w);
                }
                RichBlock::ListItem { bullet, indent, segments } => {
                    let b_w = measure_single_line_width_px(&bullet, font_px, FontWeight::SEMIBOLD, window);
                    let s_w = measure_rich_line_width_px(
                        &segments,
                        font_px,
                        FontWeight::NORMAL,
                        revealed_spoilers,
                        msg.id,
                        window,
                    );
                    let line_w = ((indent as f32) * 12.0) + b_w + 6.0 + s_w + horiz_pad + 4.0;
                    needed_w = needed_w.max(line_w);
                }
                RichBlock::Blockquote { lines, collapsed_default, .. } => {
                    let can_collapse = lines.len() > 4;
                    let extra = palette.scaled_px(22.0) + if can_collapse { 20.0 } else { 0.0 };
                    let visible = if collapsed_default && can_collapse {
                        &lines[..4]
                    } else {
                        &lines[..]
                    };
                    for q_line in visible {
                        let line_w = measure_rich_line_width_px(
                            q_line,
                            q_font,
                            FontWeight::NORMAL,
                            revealed_spoilers,
                            msg.id,
                            window,
                        ) + extra
                            + horiz_pad;
                        needed_w = needed_w.max(line_w);
                        if needed_w >= eff_max_w {
                            return Some(eff_max_w);
                        }
                    }
                }
            }
            if needed_w >= eff_max_w {
                return Some(eff_max_w);
            }
        }
    }

    Some(needed_w.clamp(eff_min_w, eff_max_w).ceil())
}

#[allow(clippy::too_many_arguments)]
pub fn render_message_row_1to1<V: 'static>(
    msg: &MessageItemModel,
    meta: Option<&MessageRichMeta>,
    group: BubbleGroupLayout,
    stage_w: f32,
    is_selected: bool,
    has_any_selection: bool,
    reaction_overlay_mode: Option<ReactionOverlayMode>,
    reaction_overlay_layout: ReactionOverlayLayout,
    bubble_widths_probe: Option<Rc<RefCell<HashMap<u64, f32>>>>,
    bubble_bounds_probe: Option<Rc<RefCell<HashMap<u64, Bounds<Pixels>>>>>,
    row_heights_probe: Option<Rc<RefCell<HashMap<u64, f32>>>>,
    highlight_alpha: f32,
    is_unread_anchor: bool,
    date_divider_label: Option<&str>,
    revealed_spoilers: &HashSet<(u64, usize)>,
    expanded_quotes: &HashSet<(u64, usize)>,
    copied_code_msg: Option<u64>,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    window: &Window,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, BubbleUiAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let msg_id = msg.id;
    let is_outgoing = msg.direction == MessageDirection::Outgoing;

    if let MessageContent::Service { text, .. } = &msg.content {
        return div()
            .id(SharedString::from(format!("svc-row-{}", msg_id)))
            .relative()
            .w_full()
            .py(px(8.0))
            .flex_shrink_0()
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
            .when_some(row_heights_probe, |el, probe| {
                el.child(make_row_height_probe_canvas(msg_id, probe))
            })
            .into_any_element();
    }

    let r_lg = palette.scaled_px(palette.bubble_radius_px as f32);
    let r_sm = palette.scaled_px(4.0f32);
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

    let eff_min_w = palette.scaled_px(MSG_MIN_WIDTH);
    let avatar_skip = if !is_outgoing && group.reserve_avatar_column {
        MSG_PHOTO_SKIP
    } else {
        0.0
    };
    let sel_skip = if has_any_selection { 32.0 } else { 0.0 };
    let avail_w = (stage_w - 24.0 - 32.0 - 56.0 - avatar_skip - sel_skip).max(eff_min_w);
    let eff_max_w = palette.scaled_px(MSG_MAX_WIDTH).min(avail_w).max(eff_min_w);
    let definite_w = compute_definite_bubble_width(
        msg,
        meta,
        group,
        is_outgoing,
        revealed_spoilers,
        palette,
        lang_id,
        eff_min_w,
        eff_max_w,
        window,
    );

    let act_rclick = on_action.clone();
    let act_dbl = on_action.clone();
    let act_sel = on_action.clone();
    let act_hover = on_action.clone();
    let mut bubble = div()
        .id(SharedString::from(format!("bubble-{}", msg_id)))
        .relative()
        .flex()
        .flex_col()
        .flex_shrink_0()
        .when_some(definite_w, |el, bw| {
            el.w(px(bw)).min_w(px(bw)).max_w(px(bw))
        })
        .when(definite_w.is_none(), |el| {
            el.min_w(px(eff_min_w)).max_w(px(eff_max_w))
        })
        .px(px(palette.scaled_px(11.0)))
        .py(px(palette.scaled_px(7.0)))
        .rounded_tl(px(tl))
        .rounded_tr(px(tr))
        .rounded_br(px(br))
        .rounded_bl(px(bl))
        .bg(rgb(bubble_hex))
        .when(is_selected, |el| el.border_1().border_color(rgb(palette.accent)))
        .when(highlight_alpha > 0.01, |el| {
            el.border_2().border_color(TdesktopThemePalette::rgba_with_alpha(palette.accent, highlight_alpha))
        })
        .on_mouse_down(MouseButton::Left, cx.listener(move |this, ev: &MouseDownEvent, _, cx| {
            if has_any_selection {
                cx.stop_propagation();
                act_dbl(this, BubbleUiAction::ToggleSelect(msg_id), cx);
            } else if ev.click_count == 2 {
                cx.stop_propagation();
                act_dbl(this, BubbleUiAction::ToggleReaction(msg_id, "👍".to_string()), cx);
            }
        }))
        .on_mouse_down(MouseButton::Right, cx.listener(move |this, ev: &MouseDownEvent, _, cx| {
            act_rclick(this, BubbleUiAction::OpenContextMenu(msg_id, ev.position), cx);
        }))
        .on_hover(cx.listener(move |this, hovered, _, cx| {
            act_hover(this, BubbleUiAction::SetBubbleHover(msg_id, *hovered), cx);
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
        let fwd_label = if lang_id.starts_with("zh") {
            format!("转发自 {}", fwd.sender_name)
        } else {
            format!("Forwarded from {}", fwd.sender_name)
        };
        bubble = bubble.child(
            div()
                .mb(px(palette.scaled_px(3.0)))
                .flex()
                .items_center()
                .gap(px(palette.scaled_px(4.0)))
                .child(tg_icon(IconName::Forward, palette.scaled_px(12.0), palette.accent))
                .child(
                    div()
                        .text_size(px(palette.scaled_px(11.5)))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(palette.accent))
                        .child(fwd_label),
                ),
        );
    }

    if let Some(reply) = &msg.reply_to {
        let target_id = reply.message_id;
        let act_reply = on_action.clone();
        bubble = bubble.child(
            div()
                .id(SharedString::from(format!("reply-hdr-{}", msg_id)))
                .w_full()
                .min_w(px(0.0))
                .overflow_hidden()
                .mb(px(5.0))
                .pl(px(8.0))
                .pr(px(8.0))
                .py(px(4.0))
                .rounded(px(6.0))
                .bg(TdesktopThemePalette::rgba_with_alpha(palette.reply_bar_bg, 0.30))
                .border_l_2()
                .border_color(rgb(palette.reply_bar_fg))
                .cursor_pointer()
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .on_click(cx.listener(move |this, _, _, cx| {
                    act_reply(this, BubbleUiAction::ClickReplyQuote(target_id), cx);
                }))
                .child(
                    div()
                        .text_size(px(11.5))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(palette.reply_bar_fg))
                        .truncate()
                        .child(reply.sender_name.clone()),
                )
                .child(
                    div()
                        .text_size(px(11.5))
                        .text_color(rgb(palette.text_secondary))
                        .truncate()
                        .child(reply.snippet.clone()),
                ),
        );
    }

    bubble = bubble
        .child(body_el)
        .child(render_bottom_meta_strip(msg, meta, is_outgoing, palette, lang_id));

    if has_any_selection {
        let act_shield = on_action.clone();
        bubble = bubble.child(
            div()
                .id(SharedString::from(format!("bubble-select-shield-{}", msg_id)))
                .absolute()
                .inset_0()
                .size_full()
                .cursor_pointer()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        act_shield(this, BubbleUiAction::ToggleSelect(msg_id), cx);
                    }),
                ),
        );
    }

    if !msg.reactions.is_empty() {
        // Sort inline reaction pills per upstream `history_view_reactions.cpp:L177-L232`
        let raw_pills: Vec<ReactionPill> = msg
            .reactions
            .iter()
            .enumerate()
            .map(|(catalog_index, rx)| {
                let key = if rx.emoji == "⭐" {
                    ReactionKey::Paid
                } else {
                    ReactionKey::Emoji(rx.emoji.clone())
                };
                let mut pill = ReactionPill::new(key, rx.count as i32);
                pill.chosen = rx.user_reacted;
                pill.catalog_index = catalog_index;
                pill
            })
            .collect();
        let sorted_pills = sort_reaction_pills(raw_pills);

        let mut rx_bar = div().flex().flex_wrap().gap(px(4.0)).mt(px(5.0));
        for pill in sorted_pills {
            let em = pill.id.display_label();
            let act_rx = on_action.clone();
            let is_chosen = pill.chosen;
            let is_paid = pill.paid;
            let bg_color = if is_paid {
                rgba(0xFFA5002E)
            } else if is_chosen {
                rgb(palette.reaction_active_bg)
            } else {
                rgb(palette.reaction_bg)
            };
            let fg_hex = if is_chosen && !is_paid {
                0xFFFFFF
            } else if is_paid {
                0xD97706
            } else {
                palette.text_primary
            };
            rx_bar = rx_bar.child(
                div()
                    .id(SharedString::from(format!("rx-pill-{}-{}", msg_id, em)))
                    .h(px(24.0))
                    .px(px(8.0))
                    .rounded_full()
                    .bg(bg_color)
                    .when(is_chosen, |el| el.border_1().border_color(rgb(palette.accent)))
                    .when(is_paid && !is_chosen, |el| el.border_1().border_color(rgb(0xFFA500)))
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .cursor_pointer()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            cx.stop_propagation();
                            act_rx(this, BubbleUiAction::ToggleReaction(msg_id, em.clone()), cx);
                        }),
                    )
                    .text_size(px(12.0))
                    .child(pill.id.display_label())
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(fg_hex))
                            .child(format!("{}", pill.count)),
                    ),
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

    if bubble_widths_probe.is_some() || bubble_bounds_probe.is_some() {
        bubble = bubble.child(
            canvas(
                move |bounds, _window, _cx| {
                    let w = f32::from(bounds.size.width);
                    if w > 1.0 {
                        if let Some(ref probe) = bubble_widths_probe {
                            probe.borrow_mut().insert(msg_id, w);
                        }
                        if let Some(ref bounds_probe) = bubble_bounds_probe {
                            bounds_probe.borrow_mut().insert(msg_id, bounds);
                        }
                    }
                },
                |_, _, _, _| {},
            )
            .absolute()
            .inset_0()
            .size_full(),
        );
    }

    // Full-height invisible outer side corridor (`[W, W+38] × [0, H]` incoming / `[-38, 0] × [0, H]`
    // outgoing, strictly within `stack`'s `56px` side margin and `[0, H]` row height) so diagonal
    // cursor movement from the upper/middle part of multi-line messages toward `CornerButton` never
    // drops hover (`history_view_reactions_button.cpp`).
    if !has_any_selection {
        let act_gutter = on_action.clone();
        bubble = bubble.child(
            div()
                .id(SharedString::from(format!("bubble-gutter-{}", msg_id)))
                .absolute()
                .top_0()
                .bottom_0()
                .w(px(38.0))
                .when(is_outgoing, |el| el.left(px(-38.0)))
                .when(!is_outgoing, |el| el.right(px(-38.0)))
                .on_hover(cx.listener(move |this, hovered, _, cx| {
                    act_gutter(this, BubbleUiAction::SetBubbleGutterHover(msg_id, *hovered), cx);
                })),
        );
    }

    if let Some(overlay_mode) = reaction_overlay_mode.filter(|_| !has_any_selection) {
        let chosen_emojis: Vec<String> = msg
            .reactions
            .iter()
            .filter(|r| r.user_reacted)
            .map(|r| r.emoji.clone())
            .collect();
        let a_zone = on_action.clone();
        let a_bridge = on_action.clone();
        let a_exp = on_action.clone();
        let a_react = on_action.clone();
        bubble = bubble.child(render_hover_reaction_overlay(
            msg_id,
            is_outgoing,
            reaction_overlay_layout,
            overlay_mode,
            &chosen_emojis,
            palette,
            lang_id,
            cx,
            move |this, hovered, cx| {
                a_zone(this, BubbleUiAction::SetReactionBarHover(msg_id, hovered), cx);
            },
            move |this, hovered, cx| {
                a_bridge(this, BubbleUiAction::SetReactionBridgeHover(msg_id, hovered), cx);
            },
            move |this, cx| {
                a_exp(this, BubbleUiAction::ToggleReactionPicker(msg_id), cx);
            },
            move |this, emoji, cx| {
                a_react(this, BubbleUiAction::ToggleReaction(msg_id, emoji), cx);
            },
        ));
    }

    let mut stack = div()
        .flex()
        .flex_col()
        .flex_shrink_0()
        .when_some(definite_w, |el, bw| {
            el.w(px(bw)).min_w(px(bw)).max_w(px(bw))
        })
        .when(definite_w.is_none(), |el| {
            el.min_w(px(0.0)).max_w(px(eff_max_w))
        })
        .when(is_outgoing, |el| el.items_end().ml(px(56.0)))
        .when(!is_outgoing, |el| el.mr(px(56.0)));
    stack = stack.child(bubble);
    if let Some(kb) = &msg.bot_keyboard {
        let a_bot = on_action.clone();
        stack = stack.child(render_inline_bot_keyboard(msg_id, kb, palette, cx, move |this, i, cx| {
            a_bot(this, BubbleUiAction::Interactive(i), cx);
        }));
    }

    let mut row = div()
        .id(SharedString::from(format!("msg-row-{}", msg_id)))
        .relative()
        .w_full()
        .px(px(16.0))
        .pt(px(if group.attached_to_prev { 2.0 } else { 7.0 }))
        .flex_shrink_0()
        .flex()
        .items_end()
        .when(has_any_selection, |el| {
            el.cursor_pointer().on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
                    act_sel(this, BubbleUiAction::ToggleSelect(msg_id), cx);
                }),
            )
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
        return aligned_row
            .when_some(row_heights_probe, |el, probe| {
                el.child(make_row_height_probe_canvas(msg_id, probe))
            })
            .into_any_element();
    }

    div()
        .relative()
        .flex()
        .flex_col()
        .w_full()
        .flex_shrink_0()
        .when_some(date_divider_label, |col, label| {
            col.child(
                div().w_full().py(px(10.0)).flex_shrink_0().flex().justify_center().child(
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
                div().w_full().py(px(8.0)).flex_shrink_0().child(
                    div()
                        .w_full()
                        .py(px(4.0))
                        .bg(palette.service_pill_rgba())
                        .flex()
                        .justify_center()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(palette.accent))
                        .child(crate::i18n::tr_key_or(lang_id, "lng_unread_bar", "Unread Messages")),
                ),
            )
        })
        .child(aligned_row)
        .when_some(row_heights_probe, |el, probe| {
            el.child(make_row_height_probe_canvas(msg_id, probe))
        })
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
        let mut col = div().flex().flex_col().w_full().min_w(px(0.0)).gap(px(6.0)).child(render_document_card(
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
            let mut col = div().flex().flex_col().w_full().min_w(px(0.0)).gap(px(4.0)).child(render_rich_message_body(
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
    lang_id: &str,
) -> AnyElement {
    let fg = if is_outgoing { palette.msg_out_date_fg } else { palette.msg_in_date_fg };
    let is_edited = matches!(msg.content, MessageContent::Text { is_edited: true, .. })
        || meta.and_then(|m| m.edit_date.as_ref()).is_some();
    let views = meta.and_then(|m| m.views_count);
    let is_zh = lang_id.starts_with("zh");
    let meta_font = px(palette.scaled_px(10.5));

    div()
        .mt(px(palette.scaled_px(3.0)))
        .w_full()
        .flex()
        .items_center()
        .justify_end()
        .gap(px(palette.scaled_px(5.0)))
        .when_some(views, |el, vc| {
            let vlabel = if is_zh {
                format!("{} 次浏览", format_view_count(vc))
            } else {
                format!("{} views", format_view_count(vc))
            };
            el.child(div().text_size(meta_font).text_color(rgb(fg)).child(vlabel))
        })
        .when(is_edited, |el| {
            let elabel = if is_zh { "已编辑" } else { "edited" };
            el.child(div().text_size(meta_font).italic().text_color(rgb(fg)).child(elabel))
        })
        .child(div().text_size(meta_font).text_color(rgb(fg)).child(msg.timestamp.clone()))
        .when(is_outgoing, |el| {
            let (icon, tint) = match msg.delivery_status {
                DeliveryStatus::Sending => (IconName::ClockSending, fg),
                DeliveryStatus::Sent => (IconName::CheckSingle, palette.check_sent_fg),
                DeliveryStatus::Read => (IconName::CheckDouble, palette.check_read_fg),
            };
            el.child(tg_icon(icon, palette.scaled_px(13.0), tint))
        })
        .into_any_element()
}
