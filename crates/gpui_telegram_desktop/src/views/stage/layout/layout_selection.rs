//! Layout selection bitmask manipulation for grouped media items and albums
//! Upstream: Telegram/SourceFiles/layout/layout_selection.h, layout_selection.cpp
//! Subsystem: layout | Feature ID: TCG-N04 | Disposition: gpui_ui_1to1

/// Represents a 16-bit text or sub-item range selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct TextSelection {
    pub from: u16,
    pub to: u16,
}

impl TextSelection {
    pub const EMPTY: TextSelection = TextSelection { from: 0, to: 0 };
    pub const FULL: TextSelection = TextSelection { from: 0xFFFF, to: 0xFFFF };

    #[inline]
    pub const fn new(from: u16, to: u16) -> Self {
        Self { from, to }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.from == self.to && self.from != 0xFFFF
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.from == 0xFFFF && self.to == 0xFFFF
    }
}

/// Constant representing complete selection of a group or block.
pub const FULL_SELECTION: TextSelection = TextSelection::FULL;

/// Maximum number of items in a grouped media layout (e.g., photo/video album)
/// that can be represented via the 16-bit subgroup bitmask (bits 0..14).
pub const MAX_GROUP_SELECTION_ITEMS: usize = 0x0F;

/// Layout selection controller managing multi-item selections and subgroup bitmasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LayoutSelection {
    pub current: TextSelection,
}

impl LayoutSelection {
    /// Creates a new layout selection wrapper.
    pub fn new(selection: TextSelection) -> Self {
        Self { current: selection }
    }

    /// Determines whether the selection represents a sub-item bitmask in a group.
    /// In accordance with C++ `IsSubGroupSelection`:
    /// `selection.from == 0xFFFF && selection.to != 0xFFFF`.
    #[inline]
    pub fn is_sub_group_selection(selection: TextSelection) -> bool {
        selection.from == 0xFFFF && selection.to != 0xFFFF
    }

    /// Checks if a specific item index in a group (0..14) is selected.
    /// In accordance with C++ `IsGroupItemSelection`:
    /// Checks `index < MAX_GROUP_SELECTION_ITEMS`, `is_sub_group_selection`, and bit `(1 << index)`.
    pub fn is_group_item_selection(selection: TextSelection, index: usize) -> bool {
        if index >= MAX_GROUP_SELECTION_ITEMS {
            return false;
        }
        Self::is_sub_group_selection(selection) && ((selection.to & (1u16 << index)) != 0)
    }

    /// Returns the index of the first selected item in a group, or `None` if no items are selected.
    /// In accordance with C++ `FirstGroupItemIndex`:
    /// Scans bits 0..14 of `selection.to`.
    pub fn first_group_item_index(selection: TextSelection) -> Option<usize> {
        if !Self::is_sub_group_selection(selection) {
            return None;
        }
        for i in 0..MAX_GROUP_SELECTION_ITEMS {
            if (selection.to & (1u16 << i)) != 0 {
                return Some(i);
            }
        }
        None
    }

    /// Adds an item index to the group selection bitmask.
    /// In accordance with C++ `AddGroupItemSelection`:
    /// Sets bit `(1 << index)` in `selection.to` with `from = 0xFFFF`.
    pub fn add_group_item_selection(selection: TextSelection, index: usize) -> TextSelection {
        assert!(index < MAX_GROUP_SELECTION_ITEMS, "index out of group bounds");
        let bit = 1u16 << index;
        let to = if Self::is_sub_group_selection(selection) {
            selection.to | bit
        } else {
            bit
        };
        TextSelection { from: 0xFFFF, to }
    }

    /// Removes an item index from the group selection bitmask.
    pub fn remove_group_item_selection(selection: TextSelection, index: usize) -> TextSelection {
        if index >= MAX_GROUP_SELECTION_ITEMS || !Self::is_sub_group_selection(selection) {
            return selection;
        }
        let bit = 1u16 << index;
        let to = selection.to & !bit;
        if to == 0 {
            TextSelection::EMPTY
        } else {
            TextSelection { from: 0xFFFF, to }
        }
    }

    /// Checks if the current selection includes the specified group item index.
    pub fn contains_item(&self, index: usize) -> bool {
        Self::is_group_item_selection(self.current, index)
    }

    /// Adds an item to the current selection.
    pub fn add_item(&mut self, index: usize) {
        self.current = Self::add_group_item_selection(self.current, index);
    }

    /// Clears the selection.
    pub fn clear(&mut self) {
        self.current = TextSelection::EMPTY;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_group_selection_lifecycle() {
        let mut sel = TextSelection::EMPTY;
        assert!(!LayoutSelection::is_sub_group_selection(sel));

        // Add item 2
        sel = LayoutSelection::add_group_item_selection(sel, 2);
        assert!(LayoutSelection::is_sub_group_selection(sel));
        assert!(LayoutSelection::is_group_item_selection(sel, 2));
        assert!(!LayoutSelection::is_group_item_selection(sel, 0));
        assert_eq!(LayoutSelection::first_group_item_index(sel), Some(2));

        // Add item 0
        sel = LayoutSelection::add_group_item_selection(sel, 0);
        assert!(LayoutSelection::is_group_item_selection(sel, 0));
        assert!(LayoutSelection::is_group_item_selection(sel, 2));
        assert_eq!(LayoutSelection::first_group_item_index(sel), Some(0));

        // Remove item 2
        sel = LayoutSelection::remove_group_item_selection(sel, 2);
        assert!(!LayoutSelection::is_group_item_selection(sel, 2));
        assert!(LayoutSelection::is_group_item_selection(sel, 0));

        // Full selection is not a sub-group selection
        assert!(!LayoutSelection::is_sub_group_selection(FULL_SELECTION));
    }
}
