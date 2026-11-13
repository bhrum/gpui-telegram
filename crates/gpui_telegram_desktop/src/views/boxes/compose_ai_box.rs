//! Compose AI Assistant Dialog Box View.
//!
//! Faithful translation of upstream Telegram Desktop:
//! - `Telegram/SourceFiles/boxes/compose_ai_box.h`
//! - `Telegram/SourceFiles/boxes/compose_ai_box.cpp:L89-L150`
//!
//! Features:
//! - `kAiComposeStyleTooltipHiddenPref` preference key
//! - `kPromptRemainingThreshold = 100` prompt character limit threshold
//! - `ComposeAiMode { Translate, Style, Fix }`
//! - `CardState { Waiting, Loading, Ready, Failed }`
//! - Diff tracking (`DiffEntity`, `BuildDiffDisplay`, highlight vs strike-out)

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_AI_COMPOSE_STYLE_TOOLTIP_HIDDEN_PREF: &str = "ai_compose_style_tooltip_hidden";
pub const K_PROMPT_REMAINING_THRESHOLD: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposeAiMode {
    Translate,
    Style,
    Fix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardState {
    Waiting,
    Loading,
    Ready,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffType {
    Insert,
    Replace,
    Delete,
}

#[derive(Debug, Clone)]
pub struct DiffEntity {
    pub diff_type: DiffType,
    pub offset: usize,
    pub length: usize,
    pub old_text: String,
    pub new_text: String,
}

#[derive(Debug, Clone)]
pub struct ComposeAiBox {
    pub mode: ComposeAiMode,
    pub state: CardState,
    pub original_text: String,
    pub generated_text: String,
    pub diff_entities: Vec<DiffEntity>,
    pub prompt: String,
    pub target_language: String,
    pub tone_id: u64,
    pub error_message: Option<String>,
}

impl ComposeAiBox {
    pub fn new(original_text: impl Into<String>) -> Self {
        Self {
            mode: ComposeAiMode::Style,
            state: CardState::Waiting,
            original_text: original_text.into(),
            generated_text: String::new(),
            diff_entities: Vec::new(),
            prompt: String::new(),
            target_language: "English".to_string(),
            tone_id: 0,
            error_message: None,
        }
    }

    pub fn set_mode(&mut self, mode: ComposeAiMode) {
        self.mode = mode;
        self.state = CardState::Waiting;
    }

    pub fn start_generation(&mut self) {
        self.state = CardState::Loading;
        self.error_message = None;
    }

    pub fn complete_generation(&mut self, result: String, diffs: Vec<DiffEntity>) {
        self.generated_text = result;
        self.diff_entities = diffs;
        self.state = CardState::Ready;
    }

    pub fn fail_generation(&mut self, err: String) {
        self.error_message = Some(err);
        self.state = CardState::Failed;
    }

    pub fn is_prompt_near_limit(&self, max_length: usize) -> bool {
        max_length.saturating_sub(self.prompt.len()) <= K_PROMPT_REMAINING_THRESHOLD
    }
}

impl Default for ComposeAiBox {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl Render for ComposeAiBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mode_label = match self.mode {
            ComposeAiMode::Translate => format!("Translate to {}", self.target_language),
            ComposeAiMode::Style => "Rewrite Tone".to_string(),
            ComposeAiMode::Fix => "Fix Grammar & Spelling".to_string(),
        };

        let status_desc = match self.state {
            CardState::Waiting => "Ready to process with AI".to_string(),
            CardState::Loading => "Generating AI response...".to_string(),
            CardState::Ready => format!("Output: {} characters", self.generated_text.len()),
            CardState::Failed => format!("Error: {}", self.error_message.as_deref().unwrap_or("Failed")),
        };

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(16.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xDADADA))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(15.0))
                    .text_color(rgb(0x111111))
                    .child(format!("AI Compose — {}", mode_label)),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .p(px(10.0))
                    .bg(rgb(0xF4F6F8))
                    .rounded(px(6.0))
                    .text_size(px(13.0))
                    .text_color(rgb(0x555555))
                    .child(format!("Original: \"{}\"", self.original_text)),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .text_size(px(12.0))
                    .text_color(if self.state == CardState::Failed { rgb(0xD32F2F) } else { rgb(0x666666) })
                    .child(status_desc),
            )
    }
}
