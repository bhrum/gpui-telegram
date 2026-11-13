//! Canonical Text Message Card (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4.1).

use gpui::*;
use gpui::prelude::FluentBuilder;
use super::models::{DeliveryStatus, MessageDirection, MessageItemModel, ReactionItem, ReplyReference};

pub const MSG_MAX_WIDTH: f32 = 430.0;
pub const MSG_MIN_WIDTH: f32 = 160.0;
pub const BUBBLE_RADIUS_LARGE: f32 = 16.0;
pub const BUBBLE_RADIUS_SMALL: f32 = 4.0;
pub const K_ATTACH_MESSAGE_TO_PREVIOUS_SECONDS_DELTA: i64 = 900;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BubbleRounding {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
    pub has_tail: bool,
}

impl Default for BubbleRounding {
    fn default() -> Self {
        Self {
            top_left: BUBBLE_RADIUS_LARGE,
            top_right: BUBBLE_RADIUS_LARGE,
            bottom_left: BUBBLE_RADIUS_LARGE,
            bottom_right: BUBBLE_RADIUS_LARGE,
            has_tail: true,
        }
    }
}

impl BubbleRounding {
    pub fn compute(is_outgoing: bool, attached_prev: bool, attached_next: bool) -> Self {
        let has_tail = !attached_next;
        if is_outgoing {
            Self {
                top_left: BUBBLE_RADIUS_LARGE,
                bottom_left: BUBBLE_RADIUS_LARGE,
                top_right: if attached_prev { BUBBLE_RADIUS_SMALL } else { BUBBLE_RADIUS_LARGE },
                bottom_right: if attached_next {
                    BUBBLE_RADIUS_SMALL
                } else {
                    2.0
                },
                has_tail,
            }
        } else {
            Self {
                top_right: BUBBLE_RADIUS_LARGE,
                bottom_right: BUBBLE_RADIUS_LARGE,
                top_left: if attached_prev { BUBBLE_RADIUS_SMALL } else { BUBBLE_RADIUS_LARGE },
                bottom_left: if attached_next {
                    BUBBLE_RADIUS_SMALL
                } else {
                    2.0
                },
                has_tail,
            }
        }
    }

    pub fn for_message(is_outgoing: bool, attached_prev: bool, attached_next: bool) -> Self {
        let has_tail = !attached_next;
        if is_outgoing {
            Self {
                top_left: BUBBLE_RADIUS_LARGE,
                bottom_left: BUBBLE_RADIUS_LARGE,
                top_right: if attached_prev { BUBBLE_RADIUS_SMALL } else { BUBBLE_RADIUS_LARGE },
                bottom_right: if attached_next { BUBBLE_RADIUS_SMALL } else { BUBBLE_RADIUS_LARGE },
                has_tail,
            }
        } else {
            Self {
                top_right: BUBBLE_RADIUS_LARGE,
                bottom_right: BUBBLE_RADIUS_LARGE,
                top_left: if attached_prev { BUBBLE_RADIUS_SMALL } else { BUBBLE_RADIUS_LARGE },
                bottom_left: if attached_next { BUBBLE_RADIUS_SMALL } else { BUBBLE_RADIUS_LARGE },
                has_tail,
            }
        }
    }
}

pub fn render_bubble_tail(direction: MessageDirection, bg_color: gpui::Rgba) -> impl IntoElement {
    match direction {
        MessageDirection::Incoming => div()
            .absolute()
            .bottom_0()
            .left(px(-6.0))
            .w(px(6.0))
            .h(px(10.0))
            .bg(bg_color)
            .rounded_tl(px(6.0))
            .rounded_bl(px(1.0)),
        MessageDirection::Outgoing => div()
            .absolute()
            .bottom_0()
            .right(px(-6.0))
            .w(px(6.0))
            .h(px(10.0))
            .bg(bg_color)
            .rounded_tr(px(6.0))
            .rounded_br(px(1.0)),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RichSpanKind {
    Plain,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Code,
    Link(String),
    Spoiler,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RichTextSpan {
    pub text: String,
    pub kind: RichSpanKind,
}

pub fn parse_markdown_spans(line: &str) -> Vec<RichTextSpan> {
    let mut spans = Vec::new();
    let n = line.len();
    let mut i = 0;
    let mut plain_start = 0;

    while i < n {
        let rest = &line[i..];

        // 1. Detect URLs (http:// or https://)
        if rest.starts_with("https://") || rest.starts_with("http://") {
            let mut end = i;
            while end < n && !line[end..].chars().next().map(|c| c.is_whitespace()).unwrap_or(true) {
                end += line[end..].chars().next().unwrap().len_utf8();
            }
            let mut url = &line[i..end];
            // Trim trailing punctuation (.,!?;:))
            while !url.is_empty() && [b'.', b',', b'!', b'?', b';', b':', b')'].contains(&url.as_bytes()[url.len() - 1]) {
                url = &url[..url.len() - 1];
                end -= 1;
            }
            if i > plain_start {
                spans.push(RichTextSpan {
                    text: line[plain_start..i].to_string(),
                    kind: RichSpanKind::Plain,
                });
            }
            spans.push(RichTextSpan {
                text: url.to_string(),
                kind: RichSpanKind::Link(url.to_string()),
            });
            i = end;
            plain_start = i;
            continue;
        }

        // 2. Inline Code: `...`
        if rest.starts_with('`') {
            if let Some(close_idx) = line[i + 1..].find('`') {
                let close = i + 1 + close_idx;
                if close > i + 1 {
                    if i > plain_start {
                        spans.push(RichTextSpan {
                            text: line[plain_start..i].to_string(),
                            kind: RichSpanKind::Plain,
                        });
                    }
                    spans.push(RichTextSpan {
                        text: line[i + 1..close].to_string(),
                        kind: RichSpanKind::Code,
                    });
                    i = close + 1;
                    plain_start = i;
                    continue;
                }
            }
        }

        // 3. Bold: **...**
        if rest.starts_with("**") {
            if let Some(close_idx) = line[i + 2..].find("**") {
                let close = i + 2 + close_idx;
                if close > i + 2 {
                    if i > plain_start {
                        spans.push(RichTextSpan {
                            text: line[plain_start..i].to_string(),
                            kind: RichSpanKind::Plain,
                        });
                    }
                    spans.push(RichTextSpan {
                        text: line[i + 2..close].to_string(),
                        kind: RichSpanKind::Bold,
                    });
                    i = close + 2;
                    plain_start = i;
                    continue;
                }
            }
        }

        // 4. Underline: __...__
        if rest.starts_with("__") {
            if let Some(close_idx) = line[i + 2..].find("__") {
                let close = i + 2 + close_idx;
                if close > i + 2 {
                    if i > plain_start {
                        spans.push(RichTextSpan {
                            text: line[plain_start..i].to_string(),
                            kind: RichSpanKind::Plain,
                        });
                    }
                    spans.push(RichTextSpan {
                        text: line[i + 2..close].to_string(),
                        kind: RichSpanKind::Underline,
                    });
                    i = close + 2;
                    plain_start = i;
                    continue;
                }
            }
        }

        // 5. Strikethrough: ~~...~~
        if rest.starts_with("~~") {
            if let Some(close_idx) = line[i + 2..].find("~~") {
                let close = i + 2 + close_idx;
                if close > i + 2 {
                    if i > plain_start {
                        spans.push(RichTextSpan {
                            text: line[plain_start..i].to_string(),
                            kind: RichSpanKind::Plain,
                        });
                    }
                    spans.push(RichTextSpan {
                        text: line[i + 2..close].to_string(),
                        kind: RichSpanKind::Strikethrough,
                    });
                    i = close + 2;
                    plain_start = i;
                    continue;
                }
            }
        }

        // 6. Spoiler: ||...||
        if rest.starts_with("||") {
            if let Some(close_idx) = line[i + 2..].find("||") {
                let close = i + 2 + close_idx;
                if close > i + 2 {
                    if i > plain_start {
                        spans.push(RichTextSpan {
                            text: line[plain_start..i].to_string(),
                            kind: RichSpanKind::Plain,
                        });
                    }
                    spans.push(RichTextSpan {
                        text: line[i + 2..close].to_string(),
                        kind: RichSpanKind::Spoiler,
                    });
                    i = close + 2;
                    plain_start = i;
                    continue;
                }
            }
        }

        // 7. Italic: *...* or _..._
        if rest.starts_with('*') && !rest.starts_with("**") {
            if let Some(close_idx) = line[i + 1..].find('*') {
                let close = i + 1 + close_idx;
                if close > i + 1 {
                    if i > plain_start {
                        spans.push(RichTextSpan {
                            text: line[plain_start..i].to_string(),
                            kind: RichSpanKind::Plain,
                        });
                    }
                    spans.push(RichTextSpan {
                        text: line[i + 1..close].to_string(),
                        kind: RichSpanKind::Italic,
                    });
                    i = close + 1;
                    plain_start = i;
                    continue;
                }
            }
        }
        if rest.starts_with('_') && !rest.starts_with("__") {
            if let Some(close_idx) = line[i + 1..].find('_') {
                let close = i + 1 + close_idx;
                if close > i + 1 {
                    if i > plain_start {
                        spans.push(RichTextSpan {
                            text: line[plain_start..i].to_string(),
                            kind: RichSpanKind::Plain,
                        });
                    }
                    spans.push(RichTextSpan {
                        text: line[i + 1..close].to_string(),
                        kind: RichSpanKind::Italic,
                    });
                    i = close + 1;
                    plain_start = i;
                    continue;
                }
            }
        }

        // Advance character
        let ch_len = line[i..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
        i += ch_len;
    }

    if plain_start < n {
        spans.push(RichTextSpan {
            text: line[plain_start..].to_string(),
            kind: RichSpanKind::Plain,
        });
    }

    spans
}

pub struct TextMessageView;

impl TextMessageView {
    pub fn extract_first_url(raw: &str) -> Option<String> {
        for token in raw.split_whitespace() {
            let trimmed = token.trim_matches(|c: char| matches!(c, '(' | ')' | '[' | ']' | '<' | '>' | '"' | ''' | ',' | '.' | ';' | '!'));
            if trimmed.starts_with("https://") || trimmed.starts_with("http://") {
                return Some(trimmed.to_string());
            }
        }
        None
    }

    pub fn render_rich_text(text: &str) -> impl IntoElement {
        let lines: Vec<&str> = text.split('\n').collect();
        div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .children(lines.into_iter().map(|line| {
                if line.is_empty() {
                    return div().h(px(14.0)).into_any_element();
                }
                let spans = parse_markdown_spans(line);
                div()
                    .flex()
                    .flex_wrap()
                    .items_baseline()
                    .gap_x(px(0.0))
                    .children(spans.into_iter().map(|span| {
                        let mut el = div().text_size(px(14.0)).line_height(px(19.0));
                        match span.kind {
                            RichSpanKind::Plain => {
                                el = el.text_color(rgb(0xFFFFFF)).child(span.text);
                            }
                            RichSpanKind::Bold => {
                                el = el
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xFFFFFF))
                                    .child(span.text);
                            }
                            RichSpanKind::Italic => {
                                el = el
                                    .italic()
                                    .text_color(rgb(0xFFFFFF))
                                    .child(span.text);
                            }
                            RichSpanKind::Underline => {
                                el = el
                                    .underline()
                                    .text_color(rgb(0xFFFFFF))
                                    .child(span.text);
                            }
                            RichSpanKind::Strikethrough => {
                                el = el
                                    .line_through()
                                    .text_color(rgb(0x8A9AA8))
                                    .child(span.text);
                            }
                            RichSpanKind::Code => {
                                el = el
                                    .font_family("Courier")
                                    .bg(rgba(0x00000044))
                                    .px(px(4.0))
                                    .py(px(1.0))
                                    .rounded(px(3.0))
                                    .text_color(rgb(0xF8F9FA))
                                    .child(span.text);
                            }
                            RichSpanKind::Link(_url) => {
                                el = el
                                    .text_color(rgb(0x40A7E3))
                                    .hover(|s| s.underline())
                                    .cursor_pointer()
                                    .child(span.text);
                            }
                            RichSpanKind::Spoiler => {
                                el = el
                                    .bg(rgb(0x242F3D))
                                    .px(px(4.0))
                                    .rounded(px(3.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .child(span.text);
                            }
                        }
                        el.into_any_element()
                    }))
                    .into_any_element()
            }))
    }

    pub fn render_default(
        message: &MessageItemModel,
        text: &str,
        is_edited: bool,
    ) -> impl IntoElement {
        Self::render(message, text, is_edited, BubbleRounding::default())
    }

    pub fn render(
        message: &MessageItemModel,
        text: &str,
        is_edited: bool,
        rounding: BubbleRounding,
    ) -> impl IntoElement {
        let is_outgoing = message.is_outgoing();
        let bg_color = if is_outgoing {
            rgb(0x2B5278) // Upstream msgInBg / msgOutBg
        } else {
            rgb(0x182533)
        };

        div()
            .relative()
            .flex()
            .flex_col()
            .min_w(px(MSG_MIN_WIDTH))
            .max_w(px(MSG_MAX_WIDTH))
            .rounded_tl(px(rounding.top_left))
            .rounded_tr(px(rounding.top_right))
            .rounded_bl(px(rounding.bottom_left))
            .rounded_br(px(rounding.bottom_right))
            .bg(bg_color)
            .pl(px(11.0))
            .pr(px(11.0))
            .pt(px(8.0))
            .pb(px(8.0))
            .gap(px(4.0))
            // Reply quote strip if replying to a message
            .when_some(message.reply_to.as_ref(), |el, reply| {
                el.child(Self::render_reply_quote(reply))
            })
            // Sender name (for incoming messages in group chats)
            .when(!is_outgoing, |el| {
                el.child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(message.avatar_color))
                        .child(message.sender_name.clone()),
                )
            })
            // Text body with timestamp inline / row
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_end()
                    .justify_between()
                    .gap(px(8.0))
                    .child(Self::render_rich_text(text))
                    // Metadata footer: edited + time + delivery status
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .when(is_edited, |el| {
                                el.child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(rgb(0x8A9AA8))
                                        .child("edited"),
                                )
                            })
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(rgb(0x8A9AA8))
                                    .child(message.timestamp.clone()),
                            )
                            .when(is_outgoing, |el| {
                                el.child(Self::render_status(message.delivery_status))
                            }),
                    ),
            )
            // Reactions pill strip
            .when(!message.reactions.is_empty(), |el| {
                el.child(Self::render_reactions(&message.reactions))
            })
            // Authentic 6x10px corner bubble tail on unattached trailing corners
            .when(rounding.has_tail, |el| {
                el.child(render_bubble_tail(message.direction, bg_color))
            })
    }

    fn render_reply_quote(reply: &ReplyReference) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .px(px(8.0))
            .py(px(4.0))
            .rounded(px(6.0))
            .bg(rgba(0x00000033))
            .border_l_2()
            .border_color(rgb(0x5288C1))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x5288C1))
                            .child(reply.sender_name.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x8A9AA8))
                            .child(reply.snippet.clone()),
                    ),
            )
    }

    fn render_status(status: DeliveryStatus) -> impl IntoElement {
        match status {
            DeliveryStatus::Sending => div()
                .text_size(px(11.0))
                .text_color(rgb(0x8A9AA8))
                .child("🕒"),
            DeliveryStatus::Sent => div()
                .text_size(px(11.0))
                .text_color(rgb(0x8A9AA8))
                .child("✓"),
            DeliveryStatus::Read => div()
                .text_size(px(11.0))
                .text_color(rgb(0x40A7E3))
                .child("✓✓"),
        }
    }

    pub fn render_reactions(reactions: &[ReactionItem]) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(4.0))
            .pt(px(2.0))
            .children(reactions.iter().map(|r| {
                let pill_bg = if r.user_reacted {
                    rgba(0x2AABEE44)
                } else {
                    rgba(0x00000033)
                };
                div()
                    .flex()
                    .items_center()
                    .gap(px(3.0))
                    .px(px(6.0))
                    .py(px(2.0))
                    .rounded_full()
                    .bg(pill_bg)
                    .child(div().text_size(px(12.0)).child(r.emoji.clone()))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0xFFFFFF))
                            .child(r.count.to_string()),
                    )
            }))
    }
}
