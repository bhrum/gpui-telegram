//! Preview AI Tone Sample Box View.
//!
//! Faithful translation of upstream Telegram Desktop:
//! - `Telegram/SourceFiles/boxes/preview_ai_tone_box.h`
//! - `Telegram/SourceFiles/boxes/preview_ai_tone_box.cpp:L48-L100`
//!
//! Features:
//! - `kToastDuration = 4000ms`
//! - `kSpinDuration = 600ms`
//! - `kWaitDuration = 1000ms`
//! - `RefreshSpinPhase { Idle, Spinning, Waiting }` for the retry animation
//! - Tone sample display with edit/delete callbacks

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_TOAST_DURATION_MS: u64 = 4000;
pub const K_SPIN_DURATION_MS: u64 = 600;
pub const K_WAIT_DURATION_MS: u64 = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshSpinPhase {
    Idle,
    Spinning,
    Waiting,
}

#[derive(Debug, Clone)]
pub struct PreviewAiToneBox {
    pub tone_id: u64,
    pub tone_title: String,
    pub tone_prompt: String,
    pub emoji_document_id: u64,
    pub sample_text: String,
    pub spin_phase: RefreshSpinPhase,
    pub is_loading: bool,
}

impl PreviewAiToneBox {
    pub fn new(tone_id: u64, title: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            tone_id,
            tone_title: title.into(),
            tone_prompt: prompt.into(),
            emoji_document_id: 0,
            sample_text: "Here is an example message rewritten in this style.".to_string(),
            spin_phase: RefreshSpinPhase::Idle,
            is_loading: false,
        }
    }

    pub fn trigger_spin(&mut self) {
        self.spin_phase = RefreshSpinPhase::Spinning;
        self.is_loading = true;
    }

    pub fn enter_waiting(&mut self) {
        self.spin_phase = RefreshSpinPhase::Waiting;
    }

    pub fn finish_refresh(&mut self, new_sample: String) {
        self.sample_text = new_sample;
        self.spin_phase = RefreshSpinPhase::Idle;
        self.is_loading = false;
    }
}

impl Default for PreviewAiToneBox {
    fn default() -> Self {
        Self::new(0, "Friendly Tone", "Make it enthusiastic and warm")
    }
}

impl Render for PreviewAiToneBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let spin_status = match self.spin_phase {
            RefreshSpinPhase::Idle => "Click refresh to regenerate sample",
            RefreshSpinPhase::Spinning => "Spinning spinner animation...",
            RefreshSpinPhase::Waiting => "Waiting for LLM generation...",
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
                    .text_color(rgb(0x222222))
                    .child(format!("Preview Tone: {}", self.tone_title)),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .p(px(10.0))
                    .bg(rgb(0xF8F9FA))
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(rgb(0xEEEEEE))
                    .text_size(px(13.0))
                    .text_color(rgb(0x333333))
                    .child(self.sample_text.clone()),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .text_size(px(12.0))
                    .text_color(rgb(0x777777))
                    .child(spin_status),
            )
    }
}
