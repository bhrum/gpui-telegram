//! Target Symbol: IvMarkdownMathRendererMarkdown
//! Module 13: Instant View LaTeX Formula Rasterization Cache & Metrics
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const MAX_CACHED_FORMULAS: usize = 512;
pub const FORMULA_OVERSAMPLE: f32 = 2.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FormulaCacheKey {
    pub expression: String,
    pub font_size_pt: u32,
    pub is_display_block: bool,
    pub color_rgb: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderedFormulaMetrics {
    pub width: f32,
    pub height: f32,
    pub baseline_offset: f32,
}

#[derive(Debug, Clone)]
pub struct IvMarkdownMathRendererMarkdown {
    pub max_cache_size: usize,
    pub oversample_factor: f32,
    pub entries: Vec<(FormulaCacheKey, RenderedFormulaMetrics)>,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

impl Default for IvMarkdownMathRendererMarkdown {
    fn default() -> Self {
        Self {
            max_cache_size: MAX_CACHED_FORMULAS,
            oversample_factor: FORMULA_OVERSAMPLE,
            entries: Vec::new(),
            cache_hits: 0,
            cache_misses: 0,
        }
    }
}

impl IvMarkdownMathRendererMarkdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn query_or_render(&mut self, key: FormulaCacheKey) -> RenderedFormulaMetrics {
        for (k, metrics) in &self.entries {
            if k == &key {
                self.cache_hits += 1;
                return metrics.clone();
            }
        }

        self.cache_misses += 1;
        let estimated_width = (key.expression.len() as f32 * (key.font_size_pt as f32 * 0.6)).max(16.0);
        let estimated_height = if key.is_display_block {
            key.font_size_pt as f32 * 2.5
        } else {
            key.font_size_pt as f32 * 1.3
        };
        let metrics = RenderedFormulaMetrics {
            width: estimated_width,
            height: estimated_height,
            baseline_offset: estimated_height * 0.75,
        };

        if self.entries.len() >= self.max_cache_size {
            self.entries.remove(0);
        }
        self.entries.push((key, metrics.clone()));

        metrics
    }

    pub fn clear_cache(&mut self) {
        self.entries.clear();
        self.cache_hits = 0;
        self.cache_misses = 0;
    }

    pub fn hit_rate(&self) -> f32 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f32 / total as f32
        }
    }
}

impl Render for IvMarkdownMathRendererMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let size = self.entries.len();
        let hit_pct = (self.hit_rate() * 100.0).round() as i32;
        div()
            .flex()
            .p_1()
            .text_xs()
            .text_color(rgb(0x6c7883))
            .child(format!("Math Renderer Cache: {} items (hit rate {}%)", size, hit_pct))
    }
}
