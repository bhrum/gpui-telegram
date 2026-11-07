//! Target Symbol: IvMarkdownThemeMarkdown
//! Subsystem: iv, Feature: TCG-N31
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_theme.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyntaxHighlightColors {
    pub keyword: u32,
    pub comment: u32,
    pub string: u32,
    pub number: u32,
    pub type_name: u32,
}

impl Default for SyntaxHighlightColors {
    fn default() -> Self {
        Self {
            keyword: 0xFFD75A77,
            comment: 0xFF7E848F,
            string: 0xFF58B582,
            number: 0xFFE09A51,
            type_name: 0xFF4DB5FF,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ChatStyle {
    pub font_size: f32,
    pub line_spacing: f32,
}

#[derive(Debug, Clone, Default)]
pub struct ChatTheme {
    pub bg_color: u32,
    pub text_color: u32,
    pub syntax: SyntaxHighlightColors,
}

pub fn create_standalone_chat_theme() -> ChatTheme {
    ChatTheme {
        bg_color: 0xFF17212B,
        text_color: 0xFFF5F5F5,
        syntax: SyntaxHighlightColors::default(),
    }
}

#[derive(Debug, Clone)]
pub struct IvMarkdownThemeMarkdown {
    pub theme: ChatTheme,
    pub style: ChatStyle,
}

impl Default for IvMarkdownThemeMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownThemeMarkdown {
    pub fn new() -> Self {
        Self {
            theme: create_standalone_chat_theme(),
            style: ChatStyle {
                font_size: 14.0,
                line_spacing: 1.4,
            },
        }
    }
}

impl Render for IvMarkdownThemeMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let keyword = self.theme.syntax.keyword;
        let string = self.theme.syntax.string;
        let comment = self.theme.syntax.comment;

        div()
            .flex()
            .flex_col()
            .gap_2()
            .p_3()
            .bg(rgb(0x1e2124))
            .text_color(rgb(0xf0f0f0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child("Markdown Syntax Theme"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Keyword Color: #{:08X}", keyword))
                    .child(format!("String Color: #{:08X}", string))
                    .child(format!("Comment Color: #{:08X}", comment)),
            )
    }
}
