//! Target Symbol: IvMarkdownPrepareStateMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_prepare_state.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct PrepareContext {
    pub max_warnings: usize,
    pub strict_mode: bool,
}

#[derive(Debug, Clone, Default)]
pub struct FootnoteDefinitionEntry {
    pub tag: String,
    pub ordinal: usize,
    pub text: String,
}

#[derive(Debug, Clone, Default)]
pub struct PrepareState {
    pub formulas: HashSet<String>,
    pub warnings: Vec<String>,
    pub formula_warnings: Vec<String>,
    pub terminal_failure: Option<String>,
}

impl PrepareState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn remember_formula(&mut self, tex: &str) {
        self.formulas.insert(tex.to_string());
    }

    pub fn add_prepare_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn add_formula_warning(&mut self, warning: String) {
        self.formula_warnings.push(warning);
    }

    pub fn add_prepare_warnings(&mut self, warnings: impl IntoIterator<Item = String>) {
        self.warnings.extend(warnings);
    }

    pub fn add_formula_warnings(&mut self, warnings: impl IntoIterator<Item = String>) {
        self.formula_warnings.extend(warnings);
    }

    pub fn set_terminal_failure(&mut self, failure: String) {
        self.terminal_failure = Some(failure);
    }

    pub fn clear_prepared_output(&mut self) {
        self.formulas.clear();
        self.warnings.clear();
        self.formula_warnings.clear();
        self.terminal_failure = None;
    }
}

#[derive(Debug, Clone, Default)]
pub struct NativeIvPrepareState {
    pub failure_reason: Option<String>,
    pub is_blocked: bool,
}

impl NativeIvPrepareState {
    pub fn set_failure(&mut self, reason: String) {
        self.failure_reason = Some(reason);
        self.is_blocked = true;
    }

    pub fn blocked(&self) -> bool {
        self.is_blocked
    }
}

#[derive(Debug, Clone)]
pub struct IvMarkdownPrepareStateMarkdown {
    pub state: PrepareState,
    pub native_state: NativeIvPrepareState,
}

impl Default for IvMarkdownPrepareStateMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownPrepareStateMarkdown {
    pub fn new() -> Self {
        Self {
            state: PrepareState::new(),
            native_state: NativeIvPrepareState::default(),
        }
    }
}

impl Render for IvMarkdownPrepareStateMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let formulas = self.state.formulas.len();
        let warnings = self.state.warnings.len();
        let formula_warnings = self.state.formula_warnings.len();
        let has_failure = self.state.terminal_failure.is_some() || self.native_state.is_blocked;

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
                    .child("Preparation State & Diagnostics"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Formulas: {}", formulas))
                    .child(format!("Warnings: {}", warnings))
                    .child(format!("Formula Warnings: {}", formula_warnings))
                    .child(format!("Failure: {}", has_failure)),
            )
    }
}
