//! Target Symbol: IvMarkdownParseValidateMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_parse_validate.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const K_UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];
pub const K_UTF16_BE_BOM: [u8; 2] = [0xFE, 0xFF];
pub const K_UTF16_LE_BOM: [u8; 2] = [0xFF, 0xFE];
pub const K_UTF32_BE_BOM: [u8; 4] = [0x00, 0x00, 0xFE, 0xFF];
pub const K_UTF32_LE_BOM: [u8; 4] = [0xFF, 0xFE, 0x00, 0x00];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationFailureReason {
    UnsupportedEncodingUtf16,
    UnsupportedEncodingUtf32,
    ContainsNullBytes,
    SourceEmpty,
    SourceTooLarge,
}

#[derive(Debug, Clone)]
pub struct ValidationFailure {
    pub reason: ValidationFailureReason,
    pub message: String,
    pub byte_offset: usize,
}

pub fn has_prefix(data: &[u8], prefix: &[u8]) -> bool {
    data.starts_with(prefix)
}

pub fn validate_markdown_source_for_iv(source: &[u8]) -> Result<&[u8], ValidationFailure> {
    if source.is_empty() {
        return Err(ValidationFailure {
            reason: ValidationFailureReason::SourceEmpty,
            message: "Markdown source is empty".to_string(),
            byte_offset: 0,
        });
    }

    if has_prefix(source, &K_UTF32_BE_BOM) || has_prefix(source, &K_UTF32_LE_BOM) {
        return Err(ValidationFailure {
            reason: ValidationFailureReason::UnsupportedEncodingUtf32,
            message: "UTF-32 encoded source is not supported; UTF-8 is required".to_string(),
            byte_offset: 0,
        });
    }

    if has_prefix(source, &K_UTF16_BE_BOM) || has_prefix(source, &K_UTF16_LE_BOM) {
        return Err(ValidationFailure {
            reason: ValidationFailureReason::UnsupportedEncodingUtf16,
            message: "UTF-16 encoded source is not supported; UTF-8 is required".to_string(),
            byte_offset: 0,
        });
    }

    // Strip UTF-8 BOM if present
    let content = if has_prefix(source, &K_UTF8_BOM) {
        &source[3..]
    } else {
        source
    };

    if let Some(pos) = content.iter().position(|&b| b == 0) {
        return Err(ValidationFailure {
            reason: ValidationFailureReason::ContainsNullBytes,
            message: format!("Forbidden null byte detected at offset {}", pos),
            byte_offset: pos,
        });
    }

    Ok(content)
}

#[derive(Debug, Clone)]
pub struct IvMarkdownParseValidateMarkdown {
    pub total_validations: usize,
    pub total_failures: usize,
    pub last_failure: Option<ValidationFailure>,
}

impl Default for IvMarkdownParseValidateMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownParseValidateMarkdown {
    pub fn new() -> Self {
        Self {
            total_validations: 0,
            total_failures: 0,
            last_failure: None,
        }
    }

    pub fn validate<'a>(&mut self, source: &'a [u8]) -> Result<&'a [u8], ValidationFailure> {
        self.total_validations += 1;
        match validate_markdown_source_for_iv(source) {
            Ok(clean) => Ok(clean),
            Err(e) => {
                self.total_failures += 1;
                self.last_failure = Some(e.clone());
                Err(e)
            }
        }
    }
}

impl Render for IvMarkdownParseValidateMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let total = self.total_validations;
        let fails = self.total_failures;
        let pass = total.saturating_sub(fails);

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
                    .child("Markdown Source Validator"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Total Checked: {}", total))
                    .child(format!("Passed: {}", pass))
                    .child(format!("Failures: {}", fails)),
            )
    }
}
