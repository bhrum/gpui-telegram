//! Target Symbol: IvMarkdownPrepareFormulasMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_prepare_formulas.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct FormulaMeasurementEntry {
    pub tex_source: String,
    pub width: i32,
    pub height: i32,
    pub ascent: i32,
    pub descent: i32,
}

pub fn formula_slot_count(text: &str) -> usize {
    let mut count = 0;
    let mut in_dollar = false;
    for ch in text.chars() {
        if ch == '$' {
            if in_dollar {
                count += 1;
                in_dollar = false;
            } else {
                in_dollar = true;
            }
        }
    }
    count
}

#[derive(Debug, Clone)]
pub struct IvMarkdownPrepareFormulasMarkdown {
    pub formula_measurements: HashMap<String, FormulaMeasurementEntry>,
    pub total_measured: usize,
}

impl Default for IvMarkdownPrepareFormulasMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownPrepareFormulasMarkdown {
    pub fn new() -> Self {
        Self {
            formula_measurements: HashMap::new(),
            total_measured: 0,
        }
    }

    pub fn remember_document_formula_measurement(&mut self, entry: FormulaMeasurementEntry) {
        self.total_measured += 1;
        self.formula_measurements.insert(entry.tex_source.clone(), entry);
    }

    pub fn measure_prepared_formulas(&mut self, formulas: &[String]) {
        for f in formulas {
            if !self.formula_measurements.contains_key(f) {
                let len = f.len() as i32;
                self.remember_document_formula_measurement(FormulaMeasurementEntry {
                    tex_source: f.clone(),
                    width: (len * 8).max(16),
                    height: 24,
                    ascent: 18,
                    descent: 6,
                });
            }
        }
    }
}

impl Render for IvMarkdownPrepareFormulasMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let cached = self.formula_measurements.len();
        let total = self.total_measured;

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
                    .child("Formula Measurement Registry"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Unique Cached: {}", cached))
                    .child(format!("Total Measured: {}", total)),
            )
    }
}
