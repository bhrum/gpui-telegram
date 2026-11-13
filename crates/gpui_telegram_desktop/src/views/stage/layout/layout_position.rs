//! Layout matrix position encoding and decoding
//! Upstream: Telegram/SourceFiles/layout/layout_position.h, layout_position.cpp
//! Subsystem: layout | Feature ID: TCG-N04 | Disposition: gpui_ui_1to1

/// The matrix row shift constant used across Telegram Desktop mosaic and grid layouts.
/// In composite layouts (stickers, emojis, media grids), row and column coordinates are packed
/// into a single 32-bit integer: index = row * MATRIX_ROW_SHIFT + column.
pub const MATRIX_ROW_SHIFT: i32 = 40000;

/// 2D grid position coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Position {
    pub row: i32,
    pub column: i32,
}

impl Position {
    pub const INVALID: Position = Position { row: -1, column: -1 };

    #[inline]
    pub const fn new(row: i32, column: i32) -> Self {
        Self { row, column }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.row >= 0 && self.column >= 0
    }

    #[inline]
    pub fn to_index(&self) -> i32 {
        LayoutPosition::position_to_index(self.row, self.column)
    }
}

/// Layout position controller providing encoding, decoding, and matrix bounds validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LayoutPosition {
    pub position: Position,
}

impl LayoutPosition {
    /// Creates a new layout position container.
    pub fn new(row: i32, column: i32) -> Self {
        Self {
            position: Position::new(row, column),
        }
    }

    /// Decodes a 1D packed index into a 2D (row, column) position.
    /// In accordance with C++ `Layout::IndexToPosition`:
    /// if `index >= 0`, `row = index / 40000`, `col = index % 40000`.
    /// Otherwise, returns invalid `(-1, -1)`.
    #[inline]
    pub fn index_to_position(index: i32) -> Position {
        if index >= 0 {
            Position {
                row: index / MATRIX_ROW_SHIFT,
                column: index % MATRIX_ROW_SHIFT,
            }
        } else {
            Position::INVALID
        }
    }

    /// Encodes 2D (row, column) coordinates into a packed 1D index.
    /// In accordance with C++ `Layout::PositionToIndex`: `row * 40000 + column`.
    #[inline]
    pub fn position_to_index(row: i32, column: i32) -> i32 {
        row * MATRIX_ROW_SHIFT + column
    }

    /// Encodes a `Position` structure into a packed 1D index.
    #[inline]
    pub fn from_position(position: &Position) -> i32 {
        Self::position_to_index(position.row, position.column)
    }

    /// Returns the current position.
    pub fn current_position(&self) -> Position {
        self.position
    }

    /// Updates the position with row and column.
    pub fn set(&mut self, row: i32, column: i32) {
        self.position.row = row;
        self.position.column = column;
    }

    /// Updates from a packed index.
    pub fn set_from_index(&mut self, index: i32) {
        self.position = Self::index_to_position(index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_to_position_roundtrip() {
        let row = 5;
        let col = 12;
        let index = LayoutPosition::position_to_index(row, col);
        assert_eq!(index, 5 * 40000 + 12);
        let pos = LayoutPosition::index_to_position(index);
        assert_eq!(pos.row, row);
        assert_eq!(pos.column, col);
        assert!(pos.is_valid());
    }

    #[test]
    fn test_negative_index_is_invalid() {
        let pos = LayoutPosition::index_to_position(-1);
        assert_eq!(pos, Position::INVALID);
        assert!(!pos.is_valid());
    }
}
