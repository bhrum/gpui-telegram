//! GPUI Rich Text, Code Block Copy, Collapsible Blockquote & Spoiler Renderer.

use gpui::*;
use gpui::prelude::FluentBuilder;
use std::collections::HashSet;

use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use crate::i18n::tr_key_or;
use super::entities::{parse_rich_blocks, push_sanitized_no_breaks, RichBlock, RichEntity, StyledSegment};
use std::ops::Range;

#[derive(Debug, Clone)]
pub enum RichTextAction {
    ToggleSpoiler(u64, usize),
    ToggleBlockquote(u64, usize),
    CopyCode(String),
    OpenUrl(String),
    ClickMention(String),
    ClickHashtag(String),
    ClickBotCommand(String),
}

pub fn render_rich_message_body<V: 'static>(
    msg_id: u64,
    text: &str,
    explicit_entities: &[RichEntity],
    revealed_spoilers: &HashSet<(u64, usize)>,
    expanded_quotes: &HashSet<(u64, usize)>,
    copied_code_msg: Option<u64>,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    _is_outgoing: bool,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, RichTextAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let blocks = parse_rich_blocks(text, explicit_entities);
    let font_px = palette.scaled_px(palette.font_size_px as f32);
    let body_line_h = (font_px * 1.42).round().max(font_px + 5.0);
    let mut col = div().flex().flex_col().gap(px(3.0)).w_full().min_w(px(0.0));

    for (block_idx, block) in blocks.into_iter().enumerate() {
        match block {
            RichBlock::Paragraph(segs) => {
                col = col.child(render_inline_segments(
                    msg_id,
                    block_idx * 1000,
                    segs,
                    font_px,
                    body_line_h,
                    None,
                    revealed_spoilers,
                    palette,
                    cx,
                    on_action.clone(),
                ));
            }
            RichBlock::Heading { level, segments } => {
                let scale = match level {
                    1 => 1.24,
                    2 => 1.16,
                    3 => 1.09,
                    _ => 1.03,
                };
                let h_px = (font_px * scale).round();
                let h_line_h = (h_px * 1.32).round().max(h_px + 4.0);
                let top_margin = if block_idx == 0 { 1.0 } else { 6.0 };
                col = col.child(
                    div()
                        .w_full()
                        .min_w(px(0.0))
                        .mt(px(top_margin))
                        .mb(px(2.0))
                        .child(render_inline_segments(
                            msg_id,
                            block_idx * 1000,
                            segments,
                            h_px,
                            h_line_h,
                            Some(FontWeight::BOLD),
                            revealed_spoilers,
                            palette,
                            cx,
                            on_action.clone(),
                        )),
                );
            }
            RichBlock::Divider => {
                col = col.child(
                    div()
                        .w_full()
                        .h(px(1.0))
                        .my(px(5.0))
                        .flex_shrink_0()
                        .bg(TdesktopThemePalette::rgba_with_alpha(palette.border_fg, 0.55)),
                );
            }
            RichBlock::ListItem { bullet, indent, segments } => {
                col = col.child(
                    div()
                        .w_full()
                        .min_w(px(0.0))
                        .pl(px((indent as f32) * 12.0))
                        .flex()
                        .flex_row()
                        .items_start()
                        .gap(px(6.0))
                        .child(
                            div()
                                .flex_shrink_0()
                                .text_size(px(font_px))
                                .line_height(px(body_line_h))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(palette.accent))
                                .child(bullet),
                        )
                        .child(
                            div().flex_1().min_w(px(0.0)).child(render_inline_segments(
                                msg_id,
                                block_idx * 1000,
                                segments,
                                font_px,
                                body_line_h,
                                None,
                                revealed_spoilers,
                                palette,
                                cx,
                                on_action.clone(),
                            )),
                        ),
                );
            }
            RichBlock::PreCode { language, code } => {
                let code_copy = code.clone();
                let act = on_action.clone();
                let is_copied = copied_code_msg == Some(msg_id);
                let lang_label = if language.is_empty() {
                    "code".to_string()
                } else {
                    language
                };
                let code_font = palette.scaled_px((palette.font_size_px as f32 - 1.5).max(11.5));
                col = col.child(
                    div()
                        .flex()
                        .flex_col()
                        .w_full()
                        .min_w(px(0.0))
                        .my(px(3.0))
                        .rounded(px(8.0))
                        .bg(rgb(palette.code_bg))
                        .border_1()
                        .border_color(TdesktopThemePalette::rgba_with_alpha(palette.border_fg, 0.55))
                        .overflow_hidden()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .px(px(10.0))
                                .py(px(4.0))
                                .bg(TdesktopThemePalette::rgba_with_alpha(palette.border_fg, 0.35))
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(palette.text_secondary))
                                        .child(lang_label),
                                )
                                .child(
                                    div()
                                        .id(SharedString::from(format!("copy-code-{}-{}", msg_id, block_idx)))
                                        .flex()
                                        .items_center()
                                        .gap(px(4.0))
                                        .px(px(6.0))
                                        .py(px(2.0))
                                        .rounded(px(4.0))
                                        .cursor_pointer()
                                        .hover(|s| s.bg(TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.18)))
                                        .on_click(cx.listener(move |this, _ev, _window, cx| {
                                            cx.write_to_clipboard(ClipboardItem::new_string(code_copy.clone()));
                                            act(this, RichTextAction::CopyCode(code_copy.clone()), cx);
                                        }))
                                        .child(tg_icon(IconName::Copy, 12.0, palette.accent))
                                        .child(
                                            div()
                                                .text_size(px(11.0))
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(rgb(palette.accent))
                                                .child(if is_copied {
                                                    tr_key_or(lang_id, "lng_copied", "Copied")
                                                } else {
                                                    tr_key_or(lang_id, "lng_copy", "Copy")
                                                }),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .w_full()
                                .min_w(px(0.0))
                                .px(px(palette.scaled_px(10.0)))
                                .py(px(palette.scaled_px(8.0)))
                                .text_size(px(code_font))
                                .line_height(px(code_font + 5.0))
                                .text_color(rgb(palette.code_fg))
                                .child(code),
                        ),
                );
            }
            RichBlock::Blockquote { lines, collapsed_default, block_id } => {
                let is_expanded = expanded_quotes.contains(&(msg_id, block_id)) || !collapsed_default;
                let can_collapse = lines.len() > 4;
                let visible_slice = if is_expanded || !can_collapse {
                    &lines[..]
                } else {
                    &lines[..4]
                };
                let q_font = palette.scaled_px((palette.font_size_px as f32 - 0.5).max(12.0));
                let q_line_h = (q_font * 1.4).round().max(q_font + 4.0);
                let mut quote_col = div().flex_1().min_w(px(0.0)).flex().flex_col().gap(px(2.0));
                for (line_idx, segs) in visible_slice.iter().enumerate() {
                    if segs.is_empty() {
                        quote_col = quote_col.child(div().w_full().h(px(4.0)).flex_shrink_0());
                    } else {
                        quote_col = quote_col.child(render_inline_segments(
                            msg_id,
                            block_idx * 1000 + line_idx + 1,
                            segs.clone(),
                            q_font,
                            q_line_h,
                            None,
                            revealed_spoilers,
                            palette,
                            cx,
                            on_action.clone(),
                        ));
                    }
                }
                let act = on_action.clone();
                col = col.child(
                    div()
                        .id(SharedString::from(format!("quote-{}-{}", msg_id, block_id)))
                        .w_full()
                        .min_w(px(0.0))
                        .overflow_hidden()
                        .flex()
                        .flex_row()
                        .items_start()
                        .justify_between()
                        .my(px(palette.scaled_px(3.0)))
                        .px(px(palette.scaled_px(10.0)))
                        .py(px(palette.scaled_px(6.0)))
                        .rounded(px(palette.scaled_px(6.0)))
                        .bg(TdesktopThemePalette::rgba_with_alpha(palette.reply_bar_bg, 0.25))
                        .border_l_2()
                        .border_color(rgb(palette.reply_bar_fg))
                        .when(can_collapse, |el| {
                            el.cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                                act(this, RichTextAction::ToggleBlockquote(msg_id, block_id), cx);
                            }))
                        })
                        .child(quote_col)
                        .when(can_collapse, |el| {
                            el.child(
                                div().pl(px(6.0)).pt(px(2.0)).flex_shrink_0().child(tg_icon(
                                    if is_expanded { IconName::ChevronDown } else { IconName::ChevronRight },
                                    13.0,
                                    palette.reply_bar_fg,
                                )),
                            )
                        }),
                );
            }
        }
    }
    col.into_any_element()
}

fn is_closing_punct(c: char) -> bool {
    matches!(
        c,
        '。' | '，' | '、' | '；' | '：' | '？' | '！' | '）' | '》' | '】' | '」' | '』' | '”' | '’'
            | '.' | ',' | ';' | ':' | '?' | '!' | ')' | ']' | '}' | '"' | '\''
    )
}

fn is_cjk_char(c: char) -> bool {
    matches!(
        c as u32,
        0x2E80..=0x9FFF | 0xF900..=0xFAFF | 0xFE30..=0xFE4F | 0xFF00..=0xFFEF
    )
}

pub fn split_wrappable_inline_tokens(text: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut chars = text.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c == '\r' || c == '\n' {
            chars.next();
            continue;
        }
        if is_cjk_char(c) {
            chars.next();
            if is_closing_punct(c) {
                if let Some(prev) = tokens.last_mut() {
                    prev.push(c);
                    continue;
                }
            }
            let mut tok = String::new();
            tok.push(c);
            while let Some(&next) = chars.peek() {
                if is_closing_punct(next) {
                    tok.push(next);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(tok);
            continue;
        }
        if c == ' ' || c == '\t' {
            chars.next();
            if let Some(prev) = tokens.last_mut() {
                prev.push(' ');
            } else {
                tokens.push(" ".to_string());
            }
            continue;
        }
        let mut run = String::new();
        while let Some(&next) = chars.peek() {
            if next == '\r' || next == '\n' || next == ' ' || next == '\t' || is_cjk_char(next) {
                break;
            }
            run.push(next);
            chars.next();
        }
        while matches!(chars.peek(), Some(&' ')) {
            run.push(' ');
            chars.next();
        }
        if run.chars().all(is_closing_punct) {
            if let Some(prev) = tokens.last_mut() {
                if !prev.ends_with(' ') {
                    prev.push_str(&run);
                    continue;
                }
            }
        }
        tokens.push(run);
    }
    tokens
}

fn render_inline_segments<V: 'static>(
    msg_id: u64,
    block_key: usize,
    segs: Vec<StyledSegment>,
    font_px: f32,
    line_height_px: f32,
    default_weight: Option<FontWeight>,
    revealed_spoilers: &HashSet<(u64, usize)>,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, RichTextAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    if segs.is_empty() {
        return div().w_full().h(px(6.0)).flex_shrink_0().into_any_element();
    }

    let mut full_text = String::new();
    let mut highlights: Vec<(Range<usize>, HighlightStyle)> = Vec::new();
    let mut clickable_ranges: Vec<Range<usize>> = Vec::new();
    let mut clickable_actions: Vec<RichTextAction> = Vec::new();

    for (seg_idx, seg) in segs.into_iter().enumerate() {
        let f = &seg.flags;
        let start = full_text.len();

        let interactive_action = if f.spoiler {
            let sid = seg.spoiler_id.unwrap_or(seg_idx);
            let revealed = revealed_spoilers.contains(&(msg_id, sid));
            if revealed {
                push_sanitized_no_breaks(&mut full_text, &seg.text);
            } else {
                let mask_len = seg.text.chars().count().clamp(4, 18);
                for _ in 0..mask_len {
                    full_text.push('░');
                }
            }
            Some(RichTextAction::ToggleSpoiler(msg_id, sid))
        } else if f.code {
            let mut clean = String::new();
            push_sanitized_no_breaks(&mut clean, &seg.text);
            if !clean.is_empty() {
                full_text.push(' ');
                full_text.push_str(&clean);
                full_text.push(' ');
            }
            f.link_url.clone().map(RichTextAction::OpenUrl)
        } else {
            push_sanitized_no_breaks(&mut full_text, &seg.text);
            if let Some(url) = f.link_url.clone() {
                Some(RichTextAction::OpenUrl(url))
            } else if let Some(m) = f.mention.clone() {
                Some(RichTextAction::ClickMention(m))
            } else if let Some(h) = f.hashtag.clone() {
                Some(RichTextAction::ClickHashtag(h))
            } else if let Some(cmd) = f.bot_command.clone() {
                Some(RichTextAction::ClickBotCommand(cmd))
            } else {
                None
            }
        };

        let end = full_text.len();
        if start >= end {
            continue;
        }

        if let Some(action) = interactive_action.clone() {
            clickable_ranges.push(start..end);
            clickable_actions.push(action);
        }

        let is_link = interactive_action.is_some() && !f.spoiler;
        let has_style = f.spoiler
            || f.code
            || is_link
            || f.bold
            || f.italic
            || f.underline
            || f.strike;

        if has_style {
            let mut hs = HighlightStyle::default();
            if f.spoiler {
                let sid = seg.spoiler_id.unwrap_or(seg_idx);
                let revealed = revealed_spoilers.contains(&(msg_id, sid));
                hs.background_color = Some(Hsla::from(if revealed {
                    TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.14)
                } else {
                    TdesktopThemePalette::rgba_with_alpha(palette.text_secondary, 0.42)
                }));
                hs.color = Some(Hsla::from(if revealed {
                    rgb(palette.text_primary)
                } else {
                    rgb(palette.text_secondary)
                }));
            } else if f.code {
                hs.color = Some(Hsla::from(rgb(palette.code_fg)));
                hs.background_color = Some(Hsla::from(rgb(palette.code_bg)));
                if f.bold || default_weight.is_none() {
                    hs.font_weight = Some(if f.bold {
                        FontWeight::BOLD
                    } else {
                        FontWeight::MEDIUM
                    });
                }
                if f.link_url.is_some() || f.underline {
                    hs.underline = Some(UnderlineStyle {
                        thickness: px(1.0),
                        color: Some(Hsla::from(rgb(palette.text_link))),
                        wavy: false,
                    });
                }
            } else {
                if is_link {
                    hs.color = Some(Hsla::from(rgb(palette.text_link)));
                }
                if f.bold {
                    hs.font_weight = Some(FontWeight::BOLD);
                }
                if f.italic {
                    hs.font_style = Some(FontStyle::Italic);
                }
                if f.underline || f.link_url.is_some() {
                    hs.underline = Some(UnderlineStyle {
                        thickness: px(1.0),
                        color: Some(Hsla::from(rgb(if is_link {
                            palette.text_link
                        } else {
                            palette.text_primary
                        }))),
                        wavy: false,
                    });
                }
                if f.strike {
                    hs.strikethrough = Some(StrikethroughStyle {
                        thickness: px(1.0),
                        color: Some(Hsla::from(rgb(palette.text_primary))),
                    });
                }
            }
            highlights.push((start..end, hs));
        }
    }

    if full_text.is_empty() {
        return div().w_full().h(px(6.0)).flex_shrink_0().into_any_element();
    }

    let styled = if highlights.is_empty() {
        StyledText::new(SharedString::from(full_text))
    } else {
        StyledText::new(SharedString::from(full_text)).with_highlights(highlights)
    };

    let mut wrapper = div()
        .w_full()
        .min_w(px(0.0))
        .text_size(px(font_px))
        .line_height(px(line_height_px))
        .text_color(rgb(palette.text_primary))
        .when_some(default_weight, |el, w| el.font_weight(w));

    if clickable_ranges.is_empty() {
        wrapper = wrapper.child(styled);
    } else {
        let weak = cx.weak_entity();
        let act = on_action;
        let interactive = InteractiveText::new(
            SharedString::from(format!("rich-txt-{}-{}", msg_id, block_key)),
            styled,
        )
        .on_click(clickable_ranges, move |range_ix, _window, app_cx| {
            if let Some(action) = clickable_actions.get(range_ix).cloned() {
                let _ = weak.update(app_cx, |this, view_cx| {
                    act(this, action, view_cx);
                });
            }
        });
        wrapper = wrapper.child(interactive);
    }

    wrapper.into_any_element()
}
