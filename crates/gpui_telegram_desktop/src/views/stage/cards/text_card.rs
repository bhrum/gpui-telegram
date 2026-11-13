//! Canonical Text Message Card (`SPEC-02-GPUI-COMPONENT-HIERARCHY` § 2.4.1).
//! Supports 1:1 Telegram Desktop rich entities: Reply quotes, Forward headers,
//! Code blocks, Blockquotes, Interactive `||Spoilers||`, Inline Chat Translation,
//! and WebPage Link Previews with `⚡ INSTANT VIEW`.

use super::models::{DeliveryStatus, MessageItemModel, ReactionItem, ReplyReference};
use crate::views::tdesktop_theme::TdesktopThemePalette;
use gpui::prelude::FluentBuilder;
use gpui::*;

pub struct TextMessageView;

impl TextMessageView {
    pub fn render(message: &MessageItemModel, text: &str, is_edited: bool) -> impl IntoElement {
        Self::render_rich(
            message,
            text,
            is_edited,
            TdesktopThemePalette::default(),
            true,
            None,
        )
    }

    pub fn render_rich(
        message: &MessageItemModel,
        text: &str,
        is_edited: bool,
        pal: TdesktopThemePalette,
        spoiler_revealed: bool,
        inline_translation: Option<String>,
    ) -> impl IntoElement {
        let is_outgoing = message.is_outgoing();
        let bg_color = if is_outgoing {
            rgb(pal.msg_out_bg)
        } else {
            rgb(pal.msg_in_bg)
        };
        let text_fg = if pal.mode.is_dark() || (is_outgoing && pal.mode.is_dark()) {
            rgb(0xFFFFFF)
        } else {
            rgb(pal.text_primary)
        };
        let meta_fg = if pal.mode.is_dark() {
            rgb(0x8A9AA8)
        } else {
            rgb(pal.text_secondary)
        };

        let display_text = Self::format_spoilers_and_markdown(text, spoiler_revealed);
        let code_block_opt = Self::extract_fenced_code(text);
        let quote_line_opt = Self::extract_blockquote(text);
        let detected_url = Self::extract_first_url(text);

        div()
            .flex()
            .flex_col()
            .max_w(px(540.0))
            .rounded(px(pal.bubble_radius_px as f32))
            .bg(bg_color)
            .px(px(12.0))
            .py(px(8.0))
            .gap(px(5.0))
            // Reply quote strip if replying to a message
            .when_some(message.reply_to.as_ref(), |el, reply| {
                el.child(Self::render_reply_quote(reply, pal.accent))
            })
            // Sender name (for incoming messages)
            .when(!is_outgoing, |el| {
                el.child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(message.avatar_color))
                        .child(message.sender_name.clone()),
                )
            })
            // Blockquote callout if line starts with `>`
            .when_some(quote_line_opt, |el, qtext| {
                el.child(
                    div()
                        .px(px(8.0))
                        .py(px(4.0))
                        .rounded(px(6.0))
                        .bg(rgba(0x00000026))
                        .border_l_2()
                        .border_color(rgb(pal.accent))
                        .text_size(px(12.5))
                        .text_color(text_fg)
                        .child(format!("❝ {}", qtext)),
                )
            })
            // Fenced code block card if ```...``` present
            .when_some(code_block_opt, |el, (lang_lbl, code_src)| {
                el.child(
                    div()
                        .w_full()
                        .p(px(8.0))
                        .rounded(px(6.0))
                        .bg(rgb(0x0D141C))
                        .border_1()
                        .border_color(rgb(0x243447))
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_size(px(10.5))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.accent))
                                .child(format!("</> {}", lang_lbl)),
                        )
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(0xE2E8F0))
                                .child(code_src),
                        ),
                )
            })
            // Main Text body + Metadata footer
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_end()
                    .justify_between()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(pal.font_size_px as f32))
                            .line_height(px((pal.font_size_px + 5) as f32))
                            .text_color(text_fg)
                            .child(display_text),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .when(message.is_pinned, |el| {
                                el.child(div().text_size(px(10.5)).text_color(meta_fg).child("📌"))
                            })
                            .when(is_edited, |el| {
                                el.child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(meta_fg)
                                        .child("edited"),
                                )
                            })
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(meta_fg)
                                    .child(message.timestamp.clone()),
                            )
                            .when(is_outgoing, |el| {
                                el.child(Self::render_status(message.delivery_status, pal.accent))
                            }),
                    ),
            )
            // Inline Neural Translation Block (when Chat Translate Bar is active)
            .when_some(inline_translation, |el, tr_txt| {
                el.child(
                    div()
                        .mt(px(2.0))
                        .pt(px(4.0))
                        .border_t_1()
                        .border_color(rgba(0xFFFFFF22))
                        .text_size(px(12.5))
                        .text_color(rgb(pal.accent))
                        .child(format!("🌐 {}", tr_txt)),
                )
            })
            // WebPage Link Preview + Instant View Badge
            .when_some(detected_url, |el, url_str| {
                el.child(
                    div()
                        .mt(px(4.0))
                        .p(px(8.0))
                        .rounded(px(8.0))
                        .bg(rgba(0x00000028))
                        .border_l_2()
                        .border_color(rgb(pal.accent))
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_size(px(11.5))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.accent))
                                .child(format!("🔗 Web Preview • {}", url_str)),
                        )
                        .child(
                            div()
                                .px(px(8.0))
                                .py(px(4.0))
                                .rounded(px(5.0))
                                .bg(rgba(0x2AABEE33))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(11.5))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(pal.accent))
                                .child("⚡ INSTANT VIEW (点击打开即时预览阅读器)"),
                        ),
                )
            })
            // Reactions pill strip
            .when(!message.reactions.is_empty(), |el| {
                el.child(Self::render_reactions(&message.reactions))
            })
    }

    fn format_spoilers_and_markdown(raw: &str, revealed: bool) -> String {
        if !raw.contains("||") {
            return raw.to_string();
        }
        let parts: Vec<&str> = raw.split("||").collect();
        if parts.len() < 3 {
            return raw.to_string();
        }
        let mut out = String::new();
        for (i, seg) in parts.iter().enumerate() {
            if i % 2 == 1 {
                if revealed {
                    out.push_str(&format!("【{}】", seg));
                } else {
                    out.push_str("░░░ 点击展开剧透 Spoiler ░░░");
                }
            } else {
                out.push_str(seg);
            }
        }
        out
    }

    fn extract_fenced_code(raw: &str) -> Option<(String, String)> {
        let start = raw.find("```")?;
        let rest = &raw[start + 3..];
        let end = rest.find("```")?;
        let block = &rest[..end];
        let mut lines = block.lines();
        let first = lines.next().unwrap_or("code").trim();
        let body: String = lines.collect::<Vec<_>>().join("\n");
        if body.trim().is_empty() {
            Some(("code".to_string(), first.to_string()))
        } else {
            Some((
                if first.is_empty() {
                    "code".to_string()
                } else {
                    first.to_string()
                },
                body,
            ))
        }
    }

    fn extract_blockquote(raw: &str) -> Option<String> {
        for line in raw.lines() {
            if let Some(stripped) = line.trim_start().strip_prefix("> ") {
                return Some(stripped.to_string());
            }
        }
        None
    }

    pub fn extract_first_url(raw: &str) -> Option<String> {
        for token in raw.split_whitespace() {
            if token.starts_with("https://")
                || token.starts_with("http://")
                || token.starts_with("t.me/")
            {
                return Some(token.trim_end_matches(&['.', ',', ')', ']'][..]).to_string());
            }
        }
        None
    }

    fn render_reply_quote(reply: &ReplyReference, accent: u32) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .px(px(8.0))
            .py(px(4.0))
            .rounded(px(6.0))
            .bg(rgba(0x00000033))
            .border_l_2()
            .border_color(rgb(accent))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(accent))
                            .child(reply.sender_name.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x9FB1C1))
                            .child(reply.snippet.clone()),
                    ),
            )
    }

    fn render_status(status: DeliveryStatus, accent: u32) -> impl IntoElement {
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
                .text_color(rgb(accent))
                .child("✓✓"),
        }
    }

    pub fn render_reactions(reactions: &[ReactionItem]) -> impl IntoElement {
        div()
            .flex()
            .flex_wrap()
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
