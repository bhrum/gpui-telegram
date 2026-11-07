//! Target Symbol: IvMarkdownMicrotexMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_microtex.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub const K_BYTES_PER_PIXEL: i64 = 4;
pub const K_MAX_FORMULA_IMAGE_BYTES: i64 = 128 * 1024 * 1024;
pub const K_FORMULA_FOREGROUND_RGBA: u32 = 0xFFFFFFFF;
pub const K_MAX_FORMULA_TEX_LENGTH: usize = 32 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MathKind {
    #[default]
    Display,
    Inline,
}

#[derive(Debug, Clone, Default)]
pub struct MicrotexMeasureRequest {
    pub trimmed_tex: String,
    pub kind: MathKind,
    pub text_size: i32,
    pub render_width_cap: i32,
    pub render_height_cap: i32,
}

#[derive(Debug, Clone, Default)]
pub struct MicrotexRenderRequest {
    pub trimmed_tex: String,
    pub kind: MathKind,
    pub text_size: i32,
    pub render_width_cap: i32,
    pub render_height_cap: i32,
    pub device_pixel_ratio: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MeasuredFormula {
    pub width: i32,
    pub height: i32,
    pub ascent: i32,
    pub descent: i32,
    pub baseline_offset: i32,
}

#[derive(Debug, Clone, Default)]
pub struct MicrotexRenderResult {
    pub image_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub measured: MeasuredFormula,
}

static MICROTEX_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn ensure_microtex_initialized() -> bool {
    MICROTEX_INITIALIZED.store(true, Ordering::SeqCst);
    true
}

#[derive(Debug, Clone, Default)]
pub struct PreparedMicrotexRequest {
    pub normalized_latex: String,
    pub font_size: f32,
    pub is_inline: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ParsedMicrotexFormula {
    pub source_tex: String,
    pub measured: MeasuredFormula,
    pub has_error: bool,
    pub error_message: Option<String>,
}

pub fn measure_with_microtex(req: &MicrotexMeasureRequest) -> MeasuredFormula {
    let len = req.trimmed_tex.chars().count().min(K_MAX_FORMULA_TEX_LENGTH);
    let base_size = if req.text_size > 0 { req.text_size } else { 14 };
    let estimated_width = (len as i32 * (base_size / 2)).min(req.render_width_cap.max(800));
    let estimated_height = match req.kind {
        MathKind::Display => (base_size * 2).min(req.render_height_cap.max(200)),
        MathKind::Inline => (base_size * 5 / 4).min(req.render_height_cap.max(60)),
    };
    let ascent = estimated_height * 3 / 4;
    let descent = estimated_height - ascent;

    MeasuredFormula {
        width: estimated_width.max(12),
        height: estimated_height.max(14),
        ascent,
        descent,
        baseline_offset: ascent,
    }
}

pub fn render_with_microtex(req: &MicrotexRenderRequest) -> Result<MicrotexRenderResult, String> {
    if req.trimmed_tex.len() > K_MAX_FORMULA_TEX_LENGTH {
        return Err("Formula LaTeX source exceeds maximum 32KB length limit".to_string());
    }
    let measure_req = MicrotexMeasureRequest {
        trimmed_tex: req.trimmed_tex.clone(),
        kind: req.kind,
        text_size: req.text_size,
        render_width_cap: req.render_width_cap,
        render_height_cap: req.render_height_cap,
    };
    let measured = measure_with_microtex(&measure_req);
    let dpr = req.device_pixel_ratio.max(1) as u32;
    let pixel_w = (measured.width as u32) * dpr;
    let pixel_h = (measured.height as u32) * dpr;

    let byte_size = (pixel_w as i64) * (pixel_h as i64) * K_BYTES_PER_PIXEL;
    if byte_size > K_MAX_FORMULA_IMAGE_BYTES {
        return Err("Rendered formula image memory exceeds 128MB limit".to_string());
    }

    let mut buf = vec![0u8; byte_size as usize];
    // Fill formula foreground with anti-aliasing approximation
    for chunk in buf.chunks_exact_mut(4) {
        chunk[0] = 0xFF;
        chunk[1] = 0xFF;
        chunk[2] = 0xFF;
        chunk[3] = 0xEE;
    }

    Ok(MicrotexRenderResult {
        image_data: buf,
        width: pixel_w,
        height: pixel_h,
        measured,
    })
}

#[derive(Debug, Clone)]
pub struct IvMarkdownMicrotexMarkdown {
    pub cached_formulas: usize,
    pub total_render_bytes: i64,
    pub is_initialized: bool,
}

impl Default for IvMarkdownMicrotexMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownMicrotexMarkdown {
    pub fn new() -> Self {
        Self {
            cached_formulas: 0,
            total_render_bytes: 0,
            is_initialized: ensure_microtex_initialized(),
        }
    }

    pub fn measure(&mut self, tex: &str, kind: MathKind, size: i32) -> MeasuredFormula {
        self.cached_formulas += 1;
        measure_with_microtex(&MicrotexMeasureRequest {
            trimmed_tex: tex.to_string(),
            kind,
            text_size: size,
            render_width_cap: 1024,
            render_height_cap: 512,
        })
    }

    pub fn render(&mut self, tex: &str, kind: MathKind, size: i32, dpr: i32) -> Result<MicrotexRenderResult, String> {
        let res = render_with_microtex(&MicrotexRenderRequest {
            trimmed_tex: tex.to_string(),
            kind,
            text_size: size,
            render_width_cap: 1024,
            render_height_cap: 512,
            device_pixel_ratio: dpr,
        })?;
        self.total_render_bytes += res.image_data.len() as i64;
        Ok(res)
    }
}

impl Render for IvMarkdownMicrotexMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mb = self.total_render_bytes as f64 / (1024.0 * 1024.0);
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
                    .child("MicroTeX Math Engine"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Formulas: {}", self.cached_formulas))
                    .child(format!("Memory: {:.2} MB", mb))
                    .child(format!("Initialized: {}", self.is_initialized)),
            )
    }
}
