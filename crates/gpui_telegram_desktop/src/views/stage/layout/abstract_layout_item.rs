//! Fundamental abstract layout item bounding box and geometry primitives
//! Upstream: Telegram/SourceFiles/layout/abstract_layout_item.h, abstract_layout_item.cpp
//! Subsystem: layout | Feature ID: TCG-N04 | Disposition: gpui_ui_1to1

/// Base paint context passing frame rendering timestamp and selection mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PaintContextBase {
    pub ms: i64,
    pub selecting: bool,
}

impl PaintContextBase {
    pub fn new(ms: i64, selecting: bool) -> Self {
        Self { ms, selecting }
    }
}

/// Abstract layout item representing the bounding box, constraints, and reflow geometry
/// for chat bubbles, media elements, mosaic cells, and document previews.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractLayoutItem {
    pub width: i32,
    pub height: i32,
    pub max_width: i32,
    pub min_height: i32,
    pub position: i32, // < 0 means unplaced or removed from layout
    pub shift_x: i32,
    pub shift_y: i32,
}

impl Default for AbstractLayoutItem {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            max_width: 0,
            min_height: 0,
            position: -1,
            shift_x: 0,
            shift_y: 0,
        }
    }
}

impl AbstractLayoutItem {
    /// Creates a new layout item with specified maximum width and minimum height constraints.
    pub fn new(max_width: i32, min_height: i32) -> Self {
        Self {
            width: max_width,
            height: min_height,
            max_width,
            min_height,
            position: -1,
            shift_x: 0,
            shift_y: 0,
        }
    }

    /// Returns the maximum allowable width (intrinsic content width).
    #[inline]
    pub fn get_max_width(&self) -> i32 {
        self.max_width
    }

    /// Returns the minimum allowable height.
    #[inline]
    pub fn get_min_height(&self) -> i32 {
        self.min_height
    }

    /// Reflows the item for a given target width and computes its new height.
    /// In accordance with C++ `AbstractLayoutItem::resizeGetHeight`:
    /// `_width = std::min(width, _maxw); _height = _minh; return _height;`
    pub fn resize_get_height(&mut self, width: i32) -> i32 {
        self.width = width.min(self.max_width);
        self.height = self.min_height;
        self.height
    }

    /// Returns the computed rendered width.
    #[inline]
    pub fn get_width(&self) -> i32 {
        self.width
    }

    /// Returns the computed rendered height.
    #[inline]
    pub fn get_height(&self) -> i32 {
        self.height
    }

    /// Sets the layout position index. A negative value indicates removal from layout.
    pub fn set_position(&mut self, position: i32) {
        self.position = position;
    }

    /// Returns the layout position index.
    #[inline]
    pub fn get_position(&self) -> i32 {
        self.position
    }

    /// Returns true if the item is placed in a layout (`position >= 0`).
    #[inline]
    pub fn is_placed(&self) -> bool {
        self.position >= 0
    }

    /// Sets horizontal translation shift.
    pub fn set_shift_x(&mut self, x: i32) {
        self.shift_x = x;
    }

    /// Sets vertical translation shift.
    pub fn set_shift_y(&mut self, y: i32) {
        self.shift_y = y;
    }

    /// Sets both translation offsets.
    pub fn set_shift(&mut self, x: i32, y: i32) {
        self.shift_x = x;
        self.shift_y = y;
    }

    /// Returns translation offset as `(x, y)`.
    #[inline]
    pub fn shift(&self) -> (i32, i32) {
        (self.shift_x, self.shift_y)
    }

    /// Hit-tests whether a 2D local coordinate lies within the item rectangle `(0, 0, width, height)`.
    #[inline]
    pub fn has_point(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abstract_layout_item_reflow() {
        let mut item = AbstractLayoutItem::new(320, 240);
        assert_eq!(item.get_max_width(), 320);
        assert_eq!(item.get_min_height(), 240);
        assert!(!item.is_placed());

        // Constrained width
        let h = item.resize_get_height(200);
        assert_eq!(h, 240);
        assert_eq!(item.get_width(), 200);

        // Exceeding width is clamped to max_width
        let h2 = item.resize_get_height(500);
        assert_eq!(h2, 240);
        assert_eq!(item.get_width(), 320);

        // Hit testing
        assert!(item.has_point(10, 10));
        assert!(!item.has_point(400, 10));
        assert!(!item.has_point(-5, 10));
    }
}
