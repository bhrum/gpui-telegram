//! Text layout engine for Instant View
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

#[derive(Debug, Clone, Default)]
pub struct TextLineMetric {
    pub line_index: usize,
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
}

#[derive(Debug, Clone, Default)]
pub struct IvTextLayout {
    pub lines: Vec<TextLineMetric>,
    pub natural_width: f32,
    pub total_height: f32,
}

impl IvTextLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compute_layout(&mut self, text: &str, max_width: f32) {
        self.lines.clear();
        let char_count = text.chars().count();
        let est_width = (char_count as f32 * 8.0).min(max_width);
        let line_count = ((char_count as f32 * 8.0) / max_width.max(1.0)).ceil() as usize;

        for i in 0..line_count.max(1) {
            self.lines.push(TextLineMetric {
                line_index: i,
                width: est_width,
                height: 20.0,
                baseline: 16.0,
            });
        }

        self.natural_width = est_width;
        self.total_height = line_count.max(1) as f32 * 20.0;
    }
}
