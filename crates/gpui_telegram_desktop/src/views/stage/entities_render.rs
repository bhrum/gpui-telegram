//! GPUI Rich Text, Code Block Copy, Collapsible Blockquote & Spoiler Renderer.

use gpui::*;
use gpui::prelude::FluentBuilder;
use std::collections::HashSet;

use crate::theme::{tg_icon, IconName, TdesktopThemePalette};
use crate::i18n::tr_key_or;
use super::entities::{parse_rich_blocks, RichBlock, RichEntity, StyledSegment};

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
    let mut col = div().flex().flex_col().gap(px(2.0)).w_full();

    for (block_idx, block) in blocks.into_iter().enumerate() {
        match block {
            RichBlock::Paragraph(segs) => {
                col = col.child(render_inline_segments(
                    msg_id,
                    block_idx,
                    segs,
                    revealed_spoilers,
                    palette,
                    cx,
                    on_action.clone(),
                ));
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
                col = col.child(
                    div()
                        .flex()
                        .flex_col()
                        .w_full()
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
                                .px(px(10.0))
                                .py(px(8.0))
                                .text_size(px((palette.font_size_px as f32 - 1.5).max(11.5)))
                                .text_color(rgb(palette.code_fg))
                                .child(code),
                        ),
                );
            }
            RichBlock::Blockquote { lines, collapsed_default, block_id } => {
                let is_expanded = expanded_quotes.contains(&(msg_id, block_id)) || !collapsed_default;
                let can_collapse = lines.len() > 3;
                let visible_text = if is_expanded || !can_collapse {
                    lines.join("\n")
                } else {
                    lines[..3].join("\n")
                };
                let act = on_action.clone();
                col = col.child(
                    div()
                        .id(SharedString::from(format!("quote-{}-{}", msg_id, block_id)))
                        .flex()
                        .items_start()
                        .justify_between()
                        .w_full()
                        .my(px(3.0))
                        .px(px(10.0))
                        .py(px(6.0))
                        .rounded(px(6.0))
                        .bg(TdesktopThemePalette::rgba_with_alpha(palette.reply_bar_bg, 0.25))
                        .border_l_2()
                        .border_color(rgb(palette.reply_bar_fg))
                        .when(can_collapse, |el| {
                            el.cursor_pointer().on_click(cx.listener(move |this, _, _, cx| {
                                act(this, RichTextAction::ToggleBlockquote(msg_id, block_id), cx);
                            }))
                        })
                        .child(
                            div()
                                .flex_1()
                                .text_size(px((palette.font_size_px as f32 - 1.0).max(12.0)))
                                .text_color(rgb(palette.text_primary))
                                .child(visible_text),
                        )
                        .when(can_collapse, |el| {
                            el.child(
                                div().pl(px(6.0)).pt(px(2.0)).child(tg_icon(
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
    block_idx: usize,
    segs: Vec<StyledSegment>,
    revealed_spoilers: &HashSet<(u64, usize)>,
    palette: &TdesktopThemePalette,
    cx: &mut Context<V>,
    on_action: impl Fn(&mut V, RichTextAction, &mut Context<V>) + Clone + 'static,
) -> AnyElement {
    let font_px = palette.font_size_px as f32;
    if segs.is_empty() {
        return div().w_full().h(px(8.0)).into_any_element();
    }
    if segs.len() == 1 && !segs[0].flags.bold && !segs[0].flags.italic && !segs[0].flags.underline
        && !segs[0].flags.strike && !segs[0].flags.code && !segs[0].flags.spoiler
        && segs[0].flags.link_url.is_none() && segs[0].flags.mention.is_none()
        && segs[0].flags.hashtag.is_none() && segs[0].flags.bot_command.is_none()
    {
        return div()
            .w_full()
            .text_size(px(font_px))
            .line_height(px(font_px + 5.0))
            .text_color(rgb(palette.text_primary))
            .child(segs.into_iter().next().unwrap().text)
            .into_any_element();
    }

    let mut row = div().flex().flex_row().flex_wrap().items_center().w_full();
    for (seg_idx, seg) in segs.into_iter().enumerate() {
        let f = &seg.flags;
        if f.spoiler {
            let sid = seg.spoiler_id.unwrap_or(seg_idx);
            let revealed = revealed_spoilers.contains(&(msg_id, sid));
            let act = on_action.clone();
            let text_val = seg.text;
            let mask_len = text_val.chars().count().clamp(4, 18);
            row = row.child(
                div()
                    .id(SharedString::from(format!("spoiler-{}-{}-{}", msg_id, block_idx, sid)))
                    .px(px(4.0))
                    .py(px(1.0))
                    .rounded(px(4.0))
                    .cursor_pointer()
                    .bg(if revealed {
                        TdesktopThemePalette::rgba_with_alpha(palette.accent, 0.14)
                    } else {
                        TdesktopThemePalette::rgba_with_alpha(palette.text_secondary, 0.42)
                    })
                    .on_click(cx.listener(move |this, _, _, cx| {
                        act(this, RichTextAction::ToggleSpoiler(msg_id, sid), cx);
                    }))
                    .text_size(px(font_px))
                    .text_color(if revealed {
                        rgba((palette.text_primary << 8) | 0xFF)
                    } else {
                        TdesktopThemePalette::rgba_with_alpha(palette.text_secondary, 0.25)
                    })
                    .child(if revealed {
                        text_val
                    } else {
                        "░".repeat(mask_len)
                    }),
            );
            continue;
        }

        if f.code {
            row = row.child(
                div()
                    .px(px(5.0))
                    .py(px(1.0))
                    .rounded(px(4.0))
                    .bg(rgb(palette.code_bg))
                    .text_size(px((font_px - 1.0).max(11.5)))
                    .text_color(rgb(palette.code_fg))
                    .child(seg.text),
            );
            continue;
        }

        let interactive_action = if let Some(url) = f.link_url.clone() {
            Some(RichTextAction::OpenUrl(url))
        } else if let Some(m) = f.mention.clone() {
            Some(RichTextAction::ClickMention(m))
        } else if let Some(h) = f.hashtag.clone() {
            Some(RichTextAction::ClickHashtag(h))
        } else if let Some(cmd) = f.bot_command.clone() {
            Some(RichTextAction::ClickBotCommand(cmd))
        } else {
            None
        };

        let is_link = interactive_action.is_some();
        let text_color = if is_link { palette.text_link } else { palette.text_primary };

        if let Some(action) = interactive_action {
            let act = on_action.clone();
            row = row.child(
                div()
                    .id(SharedString::from(format!("link-{}-{}-{}", msg_id, block_idx, seg_idx)))
                    .cursor_pointer()
                    .hover(|s| s.opacity(0.85))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        act(this, action.clone(), cx);
                    }))
                    .text_size(px(font_px))
                    .line_height(px(font_px + 5.0))
                    .text_color(rgb(text_color))
                    .when(f.bold, |el| el.font_weight(FontWeight::BOLD))
                    .when(f.italic, |el| el.italic())
                    .when(f.underline || f.link_url.is_some(), |el| el.underline())
                    .when(f.strike, |el| el.line_through())
                    .child(seg.text),
            );
            continue;
        }
        // Split long plain segments cleanly so CJK and ASCII wrap without breaking baselines
        for word in split_wrappable_inline_tokens(&seg.text) {
            row = row.child(
                div()
                    .text_size(px(font_px))
                    .line_height(px(font_px + 5.0))
                    .text_color(rgb(text_color))
                    .when(f.bold, |el| el.font_weight(FontWeight::BOLD))
                    .when(f.italic, |el| el.italic())
                    .when(f.underline, |el| el.underline())
                    .when(f.strike, |el| el.line_through())
                    .child(word),
            );
        }
    }
    row.into_any_element()
}
