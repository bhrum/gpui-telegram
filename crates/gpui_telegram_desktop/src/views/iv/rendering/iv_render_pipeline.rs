//! Multi-stage render pipeline for Instant View
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use super::iv_layout_cache::IvLayoutCache;
use super::iv_paint_context::IvPaintContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PipelineStage {
    #[default]
    Idle,
    LayoutPass,
    RasterizePass,
    CompositePass,
}

#[derive(Debug, Clone, Default)]
pub struct IvRenderPipeline {
    pub current_stage: PipelineStage,
    pub layout_cache: IvLayoutCache,
    pub paint_context: IvPaintContext,
}

impl IvRenderPipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute_layout(&mut self) {
        self.current_stage = PipelineStage::LayoutPass;
        // Run layout measurements
        self.current_stage = PipelineStage::Idle;
    }

    pub fn execute_composite(&mut self) {
        self.current_stage = PipelineStage::CompositePass;
        // Composite visible elements
        self.current_stage = PipelineStage::Idle;
    }
}
