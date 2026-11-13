//! Target Symbol: IvMarkdownArticlePaintMarkdown
//! Module 13: Instant View Article Painting Pipeline & Shimmer Animation
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const THINKING_GRADIENT_SLIDE_DURATION_MS: f32 = 1200.0;
pub const THINKING_GRADIENT_WAIT_DURATION_MS: f32 = 600.0;
pub const THINKING_GRADIENT_FULL_DURATION_MS: f32 = 1800.0;

#[derive(Debug, Clone, PartialEq)]
pub struct ThinkingGradientState {
    pub elapsed_ms: f32,
    pub is_active: bool,
}

impl Default for ThinkingGradientState {
    fn default() -> Self {
        Self {
            elapsed_ms: 0.0,
            is_active: true,
        }
    }
}

impl ThinkingGradientState {
    pub fn advance(&mut self, delta_ms: f32) {
        if !self.is_active {
            return;
        }
        self.elapsed_ms = (self.elapsed_ms + delta_ms) % THINKING_GRADIENT_FULL_DURATION_MS;
    }

    pub fn phase(&self) -> f32 {
        if self.elapsed_ms < THINKING_GRADIENT_SLIDE_DURATION_MS {
            self.elapsed_ms / THINKING_GRADIENT_SLIDE_DURATION_MS
        } else {
            1.0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TableOwnershipSlot {
    pub table_id: u64,
    pub row_index: usize,
    pub col_index: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaintClipBounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for PaintClipBounds {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 600.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IvMarkdownArticlePaintMarkdown {
    pub is_dark_theme: bool,
    pub gradient_state: ThinkingGradientState,
    pub ownership_slot: Option<TableOwnershipSlot>,
    pub clip_bounds: PaintClipBounds,
}

impl Default for IvMarkdownArticlePaintMarkdown {
    fn default() -> Self {
        Self {
            is_dark_theme: true,
            gradient_state: ThinkingGradientState::default(),
            ownership_slot: None,
            clip_bounds: PaintClipBounds::default(),
        }
    }
}

impl IvMarkdownArticlePaintMarkdown {
    pub fn new(is_dark_theme: bool) -> Self {
        Self {
            is_dark_theme,
            gradient_state: ThinkingGradientState::default(),
            ownership_slot: None,
            clip_bounds: PaintClipBounds::default(),
        }
    }

    pub fn set_clip_bounds(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.clip_bounds = PaintClipBounds { x, y, width, height };
    }

    pub fn update_gradient(&mut self, delta_ms: f32) {
        self.gradient_state.advance(delta_ms);
    }

    pub fn set_table_ownership(&mut self, table_id: u64, row: usize, col: usize) {
        self.ownership_slot = Some(TableOwnershipSlot {
            table_id,
            row_index: row,
            col_index: col,
        });
    }

    pub fn clear_table_ownership(&mut self) {
        self.ownership_slot = None;
    }

    pub fn background_color(&self) -> Hsla {
        if self.is_dark_theme {
            rgb(0x181a1f).into()
        } else {
            rgb(0xffffff).into()
        }
    }
}

impl Render for IvMarkdownArticlePaintMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let phase = self.gradient_state.phase();
        let bg_color = self.background_color();

        div()
            .flex()
            .flex_col()
            .w_full()
            .p_2()
            .bg(bg_color)
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x6c7883))
                    .child(format!("Painter: phase {:.2}", phase)),
            )
    }
}
