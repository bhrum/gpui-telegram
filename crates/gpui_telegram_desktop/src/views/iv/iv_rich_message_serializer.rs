//! Target Symbol: IvRichMessageSerializer
//! Module 13: Instant View Rich Message Serialization & MTProto Compiler
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const DEFAULT_MAP_WIDTH: u32 = 640;
pub const DEFAULT_MAP_HEIGHT: u32 = 360;
pub const NO_ENTITY_INDEX: i32 = -1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SerializeMode {
    #[default]
    Draft,
    FinalSubmit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializeStatus {
    Success,
    EmptyContent,
    MissingRequiredTitle,
    InvalidMediaReference(String),
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct SerializeResult {
    pub status: SerializeStatus,
    pub block_count: usize,
    pub byte_count: usize,
}

#[derive(Debug, Clone, Default)]
pub struct IvRichMessageSerializer {
    pub mode: SerializeMode,
    pub last_result: Option<SerializeResult>,
}

impl IvRichMessageSerializer {
    pub fn new(mode: SerializeMode) -> Self {
        Self {
            mode,
            last_result: None,
        }
    }

    pub fn serialize_document(&mut self, title: &str, paragraphs: &[String]) -> SerializeResult {
        if self.mode == SerializeMode::FinalSubmit && title.trim().is_empty() {
            let res = SerializeResult {
                status: SerializeStatus::MissingRequiredTitle,
                block_count: 0,
                byte_count: 0,
            };
            self.last_result = Some(res.clone());
            return res;
        }

        let non_empty: Vec<&String> = paragraphs.iter().filter(|p| !p.trim().is_empty()).collect();
        if non_empty.is_empty() && title.trim().is_empty() {
            let res = SerializeResult {
                status: SerializeStatus::EmptyContent,
                block_count: 0,
                byte_count: 0,
            };
            self.last_result = Some(res.clone());
            return res;
        }

        let mut total_bytes = title.len() + 32;
        for p in &non_empty {
            total_bytes += p.len() + 16;
        }

        let res = SerializeResult {
            status: SerializeStatus::Success,
            block_count: non_empty.len() + if title.is_empty() { 0 } else { 1 },
            byte_count: total_bytes,
        };
        self.last_result = Some(res.clone());
        res
    }

    pub fn is_valid(&self) -> bool {
        matches!(self.last_result, Some(SerializeResult { status: SerializeStatus::Success, .. }))
    }
}

impl Render for IvRichMessageSerializer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (status_text, status_color) = match &self.last_result {
            Some(SerializeResult { status: SerializeStatus::Success, block_count, byte_count }) => (
                format!("Compiled {} blocks ({} bytes)", block_count, byte_count),
                rgb(0x61C554),
            ),
            Some(SerializeResult { status: SerializeStatus::EmptyContent, .. }) => (
                "Validation Error: Article content is empty".to_string(),
                rgb(0xED6A5E),
            ),
            Some(SerializeResult { status: SerializeStatus::MissingRequiredTitle, .. }) => (
                "Validation Error: Title is required for final submission".to_string(),
                rgb(0xED6A5E),
            ),
            Some(SerializeResult { status: SerializeStatus::InvalidMediaReference(msg), .. }) => (
                format!("Media Error: {}", msg),
                rgb(0xED6A5E),
            ),
            Some(SerializeResult { status: SerializeStatus::Failed(err), .. }) => (
                format!("Error: {}", err),
                rgb(0xED6A5E),
            ),
            None => ("Serializer Idle".to_string(), rgb(0x8D95A3)),
        };

        div()
            .flex()
            .flex_row()
            .justify_between()
            .items_center()
            .p(px(8.0))
            .bg(rgb(0x1B1F26))
            .rounded(px(6.0))
            .child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xCCD2DE))
                    .child(match self.mode {
                        SerializeMode::Draft => "Mode: Draft (Auto-Save)",
                        SerializeMode::FinalSubmit => "Mode: Final Submit (Strict)",
                    }),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(status_color)
                    .child(status_text),
            )
    }
}
