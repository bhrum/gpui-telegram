//! Overview Media Grid Layout Engine
//! Upstream: Telegram/SourceFiles/overview/overview_layout.h, Telegram/SourceFiles/overview/overview_layout.cpp

use gpui::*;

pub const K_STORY_RATIO: f32 = 1.46;
pub const K_MAX_INLINE_AREA: u32 = 1280 * 720;

#[derive(Debug, Clone)]
pub struct OverviewLayout {
    pub columns: usize,
    pub item_size: f32,
    pub gutter: f32,
    pub container_width: f32,
    pub items_count: usize,
}

impl Default for OverviewLayout {
    fn default() -> Self {
        Self {
            columns: 3,
            item_size: 104.0,
            gutter: 4.0,
            container_width: 320.0,
            items_count: 0,
        }
    }
}

impl OverviewLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_container_width(&mut self, width: f32) {
        self.container_width = width.max(100.0);
        self.recompute_columns();
    }

    pub fn set_items_count(&mut self, count: usize) {
        self.items_count = count;
    }

    pub fn recompute_columns(&mut self) {
        let available = self.container_width;
        let min_tile_width = 96.0;
        let cols = ((available + self.gutter) / (min_tile_width + self.gutter)).floor() as usize;
        self.columns = cols.max(1);
        let total_gutter = (self.columns - 1) as f32 * self.gutter;
        self.item_size = ((available - total_gutter) / self.columns as f32).max(1.0);
    }

    pub fn item_rect(&self, index: usize) -> (f32, f32, f32, f32) {
        if self.columns == 0 {
            return (0.0, 0.0, 0.0, 0.0);
        }
        let col = index % self.columns;
        let row = index / self.columns;
        let x = col as f32 * (self.item_size + self.gutter);
        let y = row as f32 * (self.item_size + self.gutter);
        (x, y, self.item_size, self.item_size)
    }

    pub fn total_height(&self) -> f32 {
        if self.columns == 0 || self.items_count == 0 {
            return 0.0;
        }
        let rows = (self.items_count + self.columns - 1) / self.columns;
        let total_gutters = if rows > 0 { (rows - 1) as f32 * self.gutter } else { 0.0 };
        rows as f32 * self.item_size + total_gutters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overview_grid_layout() {
        let mut layout = OverviewLayout::new();
        layout.set_container_width(320.0);
        assert!(layout.columns >= 3);

        layout.set_items_count(9);
        let (x, y, w, h) = layout.item_rect(0);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        assert!(w > 0.0);
        assert_eq!(w, h);

        assert!(layout.total_height() > 0.0);
    }
}
