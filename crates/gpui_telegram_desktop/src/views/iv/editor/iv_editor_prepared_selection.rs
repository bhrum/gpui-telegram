//! Instant View Multi-Block Selection Geometry (`iv_editor_prepared_selection.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

/// Normalized contiguous integer range [from..till).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NormalizedIntegerRange {
    pub from: i32,
    pub till: i32,
}

impl NormalizedIntegerRange {
    pub fn is_empty(&self) -> bool {
        self.from < 0 || self.till <= self.from
    }

    pub fn len(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            (self.till - self.from) as usize
        }
    }
}

pub fn normalize_integer_range(a: i32, b: i32) -> NormalizedIntegerRange {
    NormalizedIntegerRange {
        from: a.min(b),
        till: a.max(b),
    }
}

pub fn index_in_range(index: i32, from: i32, till: i32) -> bool {
    index >= from && index < till
}

/// Structural hierarchy tier owning the active selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StructuralOwnerKind {
    #[default]
    None,
    Block,
    ListItem,
    TableRow,
    TableCell,
}

/// Structural entity owning an edit selection hit.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StructuralOwner {
    pub kind: StructuralOwnerKind,
    pub block_index: Option<usize>,
    pub list_item_index: Option<usize>,
    pub table_row: Option<usize>,
    pub table_cell: Option<usize>,
}

impl StructuralOwner {
    pub fn is_valid(&self) -> bool {
        self.kind != StructuralOwnerKind::None
    }
}

/// Full description of a selection range in the prepared editor layout.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PreparedSelection {
    pub range: NormalizedIntegerRange,
    pub owner: StructuralOwner,
    pub forward: bool,
}

pub fn lifted_block_selection(anchor_block: usize, focus_block: usize) -> NormalizedIntegerRange {
    let a = anchor_block as i32;
    let b = focus_block as i32;
    normalize_integer_range(a, b + 1)
}

/// Instant View prepared selection inspector and range controller.
#[derive(Debug, Clone)]
pub struct IvEditorPreparedSelectionEditor {
    pub selection: PreparedSelection,
    pub total_selections_made: usize,
    pub is_active: bool,
}

impl Default for IvEditorPreparedSelectionEditor {
    fn default() -> Self {
        Self {
            selection: PreparedSelection {
                range: NormalizedIntegerRange { from: 0, till: 1 },
                owner: StructuralOwner {
                    kind: StructuralOwnerKind::Block,
                    block_index: Some(0),
                    list_item_index: None,
                    table_row: None,
                    table_cell: None,
                },
                forward: true,
            },
            total_selections_made: 1,
            is_active: false,
        }
    }
}

impl IvEditorPreparedSelectionEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_range(&mut self, from: i32, till: i32, forward: bool) {
        self.selection.range = normalize_integer_range(from, till);
        self.selection.forward = forward;
        self.total_selections_made += 1;
    }

    pub fn set_owner(&mut self, owner: StructuralOwner) {
        self.selection.owner = owner;
    }

    pub fn clear(&mut self) {
        self.selection.range = NormalizedIntegerRange { from: -1, till: -1 };
        self.selection.owner = StructuralOwner::default();
    }
}

impl Render for IvEditorPreparedSelectionEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (from, till) = (self.selection.range.from, self.selection.range.till);
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(12.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child("Instant View Selection Geometry")
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .child(format!(
                        "Range: [{}..{}) (len: {}) | Owner: {:?} | Forward: {}",
                        from, till, self.selection.range.len(), self.selection.owner.kind, self.selection.forward
                    ))
            )
    }
}
