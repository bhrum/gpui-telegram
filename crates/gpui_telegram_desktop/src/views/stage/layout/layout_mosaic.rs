//! Responsive mosaic layout algorithm for inline media, GIFs, and sticker grids
//! Upstream: Telegram/SourceFiles/layout/layout_mosaic.h, layout_mosaic.cpp
//! Subsystem: layout | Feature ID: TCG-N04 | Disposition: gpui_ui_1to1

use super::abstract_layout_item::AbstractLayoutItem;
use super::layout_position::LayoutPosition;

/// Maximum number of items in a single inline mosaic row.
pub const INLINE_ITEMS_MAX_PER_ROW: usize = 5;

/// Minimum width for an inline results item.
pub const INLINE_RESULTS_MIN_WIDTH: i32 = 40;

/// Result of point hit testing on a mosaic layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundItem {
    pub index: i32,
    pub exact: bool,
    pub relative_x: i32,
    pub relative_y: i32,
}

/// Single row in a mosaic layout containing items and computed dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MosaicRow {
    pub max_width: i32,
    pub height: i32,
    pub items: Vec<AbstractLayoutItem>,
}

/// Mosaic layout manager calculating proportional width distributions across rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutMosaic {
    pub big_width: i32,
    pub width: i32,
    pub right_skip: i32,
    pub padding_top: i32,
    pub padding_bottom: i32,
    pub padding_left: i32,
    pub padding_right: i32,
    pub rows: Vec<MosaicRow>,
}

impl LayoutMosaic {
    /// Creates a new mosaic layout with the given target big width.
    pub fn new(big_width: i32) -> Self {
        Self {
            big_width,
            width: big_width,
            right_skip: 4,
            padding_top: 0,
            padding_bottom: 0,
            padding_left: 0,
            padding_right: 0,
            rows: Vec::new(),
        }
    }

    /// Returns the height of the row at the specified index.
    pub fn row_height_at(&self, row: usize) -> Option<i32> {
        self.rows.get(row).map(|r| r.height)
    }

    /// Sets horizontal item spacing between cells.
    pub fn set_right_skip(&mut self, skip: i32) {
        self.right_skip = skip;
    }

    /// Sets container margins (top, right, bottom, left).
    pub fn set_padding(&mut self, top: i32, right: i32, bottom: i32, left: i32) {
        self.padding_top = top;
        self.padding_right = right;
        self.padding_bottom = bottom;
        self.padding_left = left;
    }

    /// Sets full available container width.
    pub fn set_full_width(&mut self, width: i32) {
        self.width = width;
    }

    /// Returns true if no rows are present.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Returns the total number of finalized rows.
    #[inline]
    pub fn rows_count(&self) -> usize {
        self.rows.len()
    }

    /// Clears all rows and resets item positions.
    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    /// Computes total desired height across all rows for a target width.
    pub fn count_desired_height(&mut self, new_width: i32) -> i32 {
        let target_width = if new_width > 0 { new_width } else { self.width };
        let mut total_height = 0;
        let right_skip = self.right_skip;
        let pad_h = self.padding_left + self.padding_right;

        for row in &mut self.rows {
            Self::layout_row(row, target_width, right_skip, pad_h);
            total_height += row.height;
        }

        self.padding_top + total_height + self.padding_bottom
    }

    /// Adds items and builds the mosaic rows.
    pub fn add_items(&mut self, items: Vec<AbstractLayoutItem>) {
        let mut current_row = MosaicRow::default();
        let mut sum_width = 0;

        for mut item in items {
            let row_idx = self.rows.len() as i32;
            let col_idx = current_row.items.len() as i32;
            item.set_position(LayoutPosition::position_to_index(row_idx, col_idx));

            if self.row_finalize(&mut current_row, &mut sum_width, false) {
                let new_row_idx = self.rows.len() as i32;
                item.set_position(LayoutPosition::position_to_index(new_row_idx, 0));
            }

            sum_width += item.get_max_width();
            if !current_row.items.is_empty() && self.right_skip > 0 {
                sum_width += self.right_skip;
            }

            current_row.items.push(item);
        }

        self.row_finalize(&mut current_row, &mut sum_width, true);
    }

    fn row_finalize(&mut self, row: &mut MosaicRow, sum_width: &mut i32, force: bool) -> bool {
        if row.items.is_empty() {
            return false;
        }

        let full = row.items.len() >= INLINE_ITEMS_MAX_PER_ROW;
        let big = *sum_width >= self.big_width;

        if full || big || force {
            row.max_width = if full || big { *sum_width } else { 0 };
            let pad_h = self.padding_left + self.padding_right;
            Self::layout_row(row, self.width, self.right_skip, pad_h);
            self.rows.push(std::mem::take(row));
            *sum_width = 0;
            return true;
        }
        false
    }

    fn layout_row(row: &mut MosaicRow, full_width: i32, right_skip: i32, pad_h: i32) {
        let count = row.items.len();
        if count == 0 {
            return;
        }

        let mut available_width = full_width - pad_h;
        let mut desired_width = if row.max_width > 0 {
            row.max_width
        } else {
            row.items.iter().map(|i| i.get_max_width()).sum::<i32>()
        };

        row.height = 0;

        for i in 0..count {
            let item_max_w = row.items[i].get_max_width();
            let w = if desired_width > 0 {
                (item_max_w * available_width) / desired_width
            } else {
                item_max_w
            };
            let actual_w = w.max(INLINE_RESULTS_MIN_WIDTH);
            let item_h = row.items[i].resize_get_height(actual_w);
            row.height = row.height.max(item_h);

            if desired_width > 0 {
                available_width -= actual_w;
                desired_width -= item_max_w;
                if i > 0 && right_skip > 0 {
                    available_width -= right_skip;
                    desired_width -= right_skip;
                }
            }
        }
    }

    /// Finds an item under a global coordinate point `(x, y)`.
    pub fn find_by_point(&self, global_x: i32, global_y: i32) -> FoundItem {
        let mut sx = global_x - self.padding_left;
        let mut sy = global_y - self.padding_top;
        let mut row_idx;
        let mut col_idx = -1;
        let mut exact = true;

        if self.rows.is_empty() {
            return FoundItem {
                index: -1,
                exact: false,
                relative_x: sx,
                relative_y: sy,
            };
        }

        if sy >= 0 {
            row_idx = self.rows.len() as i32;
            for (r, row) in self.rows.iter().enumerate() {
                if sy < row.height {
                    row_idx = r as i32;
                    break;
                }
                sy -= row.height;
            }
        } else {
            row_idx = 0;
            exact = false;
        }

        if row_idx >= self.rows.len() as i32 {
            row_idx = self.rows.len() as i32 - 1;
            exact = false;
        }

        if sx < 0 {
            sx = 0;
            exact = false;
        }

        if sx >= 0 && row_idx >= 0 && (row_idx as usize) < self.rows.len() {
            let row = &self.rows[row_idx as usize];
            col_idx = row.items.len() as i32;
            for (c, item) in row.items.iter().enumerate() {
                let w = item.get_width();
                if sx < w {
                    col_idx = c as i32;
                    break;
                }
                sx -= w + self.right_skip;
            }
            if col_idx >= row.items.len() as i32 || col_idx < 0 {
                col_idx = (row.items.len() as i32).saturating_sub(1);
                exact = false;
            }
        }

        let sel = if row_idx >= 0 && col_idx >= 0 {
            LayoutPosition::position_to_index(row_idx, col_idx)
        } else {
            -1
        };

        FoundItem {
            index: sel,
            exact,
            relative_x: sx,
            relative_y: sy,
        }
    }

    /// Finds the bounding rectangle `(x, y, width, height)` for an item by position index.
    pub fn find_rect(&self, index: i32) -> Option<(i32, i32, i32, i32)> {
        let pos = LayoutPosition::index_to_position(index);
        if !pos.is_valid() || (pos.row as usize) >= self.rows.len() {
            return None;
        }

        let mut top = self.padding_top;
        for r in 0..(pos.row as usize) {
            top += self.rows[r].height;
        }

        let row = &self.rows[pos.row as usize];
        if (pos.column as usize) >= row.items.len() {
            return None;
        }

        let mut left = self.padding_left;
        for c in 0..(pos.column as usize) {
            left += row.items[c].get_width() + self.right_skip;
        }

        let item = &row.items[pos.column as usize];
        Some((left, top, item.get_width(), item.get_height()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mosaic_layout_flow() {
        let mut mosaic = LayoutMosaic::new(300);
        let items = vec![
            AbstractLayoutItem::new(100, 80),
            AbstractLayoutItem::new(120, 80),
            AbstractLayoutItem::new(140, 80),
            AbstractLayoutItem::new(100, 80),
        ];

        mosaic.add_items(items);
        assert!(!mosaic.is_empty());
        assert!(mosaic.rows_count() >= 1);

        let total_h = mosaic.count_desired_height(300);
        assert!(total_h >= 80);

        let rect = mosaic.find_rect(0);
        assert!(rect.is_some());
    }
}
