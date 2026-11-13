//! Target Symbol: IvMarkdownPrepareSerializeMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_prepare_serialize.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, Default)]
pub struct TextOffsetsRun {
    pub old_offset: usize,
    pub new_offset: usize,
    pub delta: i32,
}

#[derive(Debug, Clone, Default)]
pub struct TextOffsetsMap {
    pub runs: Vec<TextOffsetsRun>,
}

impl TextOffsetsMap {
    pub fn adjust_offset(&self, offset: usize) -> usize {
        let mut adj = offset as i32;
        for run in &self.runs {
            if offset >= run.old_offset {
                adj += run.delta;
            }
        }
        adj.max(0) as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Color {
    #[default]
    Default,
    Primary,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Type {
    #[default]
    Url,
    Callback,
    SwitchInline,
}

#[derive(Debug, Clone, Default)]
pub struct PeerTypes {
    pub flags: u32,
}

impl PeerTypes {
    pub fn from_raw(flags: u32) -> Self {
        Self { flags }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RichButtonLabelDates {
    pub date_created: i64,
    pub date_expires: i64,
}

#[derive(Debug, Clone, Default)]
pub struct InlineLinkButtonSpan {
    pub text: String,
    pub url: String,
    pub button_type: Type,
    pub button_color: Color,
}

pub fn expand_inline_text_objects(
    raw_text: &str,
    buttons: &[InlineLinkButtonSpan],
) -> (String, TextOffsetsMap) {
    let mut expanded = raw_text.to_string();
    let mut offset_map = TextOffsetsMap::default();

    for btn in buttons {
        let placeholder = "[btn]";
        if let Some(pos) = expanded.find(placeholder) {
            let replacement = format!("[Button: {}]", btn.text);
            let delta = replacement.len() as i32 - placeholder.len() as i32;
            expanded.replace_range(pos..pos + placeholder.len(), &replacement);
            offset_map.runs.push(TextOffsetsRun {
                old_offset: pos,
                new_offset: pos,
                delta,
            });
        }
    }

    (expanded, offset_map)
}

#[derive(Debug, Clone)]
pub struct IvMarkdownPrepareSerializeMarkdown {
    pub total_expansions: usize,
    pub last_delta: i32,
}

impl Default for IvMarkdownPrepareSerializeMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownPrepareSerializeMarkdown {
    pub fn new() -> Self {
        Self {
            total_expansions: 0,
            last_delta: 0,
        }
    }

    pub fn expand_buttons(&mut self, text: &str, buttons: &[InlineLinkButtonSpan]) -> String {
        self.total_expansions += 1;
        let (res, map) = expand_inline_text_objects(text, buttons);
        self.last_delta = map.runs.iter().map(|r| r.delta).sum();
        res
    }
}

impl Render for IvMarkdownPrepareSerializeMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let expansions = self.total_expansions;
        let delta = self.last_delta;

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
                    .child("Markdown Entity Offset Serializer"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Total Expansions: {}", expansions))
                    .child(format!("Last Offset Delta: {}", delta)),
            )
    }
}
