//! Instant View Table 2D Grid Layout & Slot Occupancy (`iv_editor_page_table_grid.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

/// Reference geometry bounds of an individual table cell in grid space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TableGridCellReference {
    pub row_index: i32,
    pub cell_index: i32,
    pub row_from: i32,
    pub row_till: i32,
    pub column_from: i32,
    pub column_till: i32,
}

impl TableGridCellReference {
    pub fn intersects(&self, r_from: i32, r_till: i32, c_from: i32, c_till: i32) -> bool {
        self.row_from < r_till
            && self.row_till > r_from
            && self.column_from < c_till
            && self.column_till > c_from
    }
}

/// 2D Table grid occupancy structure computing multi-cell spans.
#[derive(Debug, Clone, Default)]
pub struct TableGrid {
    pub cells: Vec<TableGridCellReference>,
    pub occupancy: Vec<Vec<bool>>,
    pub row_count: usize,
    pub column_count: usize,
}

pub fn normalize_table_span(span: i32) -> i32 {
    span.clamp(1, 100)
}

pub fn can_occupy_table_slots(
    occupancy: &[Vec<bool>],
    row: usize,
    col: usize,
    rowspan: usize,
    colspan: usize,
) -> bool {
    for r in row..(row + rowspan) {
        if r >= occupancy.len() {
            return false;
        }
        for c in col..(col + colspan) {
            if c >= occupancy[r].len() || occupancy[r][c] {
                return false;
            }
        }
    }
    true
}

pub fn mark_table_slots(
    occupancy: &mut [Vec<bool>],
    row: usize,
    col: usize,
    rowspan: usize,
    colspan: usize,
) {
    for r in row..(row + rowspan) {
        if r < occupancy.len() {
            for c in col..(col + colspan) {
                if c < occupancy[r].len() {
                    occupancy[r][c] = true;
                }
            }
        }
    }
}

/// Builds complete 2D occupancy matrix from cell span definitions.
pub fn build_table_grid(cell_spans: &[Vec<(usize, usize)>]) -> TableGrid {
    let row_count = cell_spans.len();
    let mut max_cols = 0;
    for row in cell_spans {
        let sum_cols: usize = row.iter().map(|(_, c)| *c).sum();
        max_cols = max_cols.max(sum_cols);
    }
    let column_count = max_cols.max(1);

    let mut occupancy = vec![vec![false; column_count]; row_count];
    let mut cells = Vec::new();

    for (r_idx, row) in cell_spans.iter().enumerate() {
        let mut c_idx = 0;
        for (cell_idx, &(rowspan, colspan)) in row.iter().enumerate() {
            while c_idx < column_count && occupancy[r_idx][c_idx] {
                c_idx += 1;
            }
            if c_idx >= column_count {
                break;
            }
            let valid_rowspan = rowspan.max(1).min(row_count - r_idx);
            let valid_colspan = colspan.max(1).min(column_count - c_idx);

            mark_table_slots(&mut occupancy, r_idx, c_idx, valid_rowspan, valid_colspan);
            cells.push(TableGridCellReference {
                row_index: r_idx as i32,
                cell_index: cell_idx as i32,
                row_from: r_idx as i32,
                row_till: (r_idx + valid_rowspan) as i32,
                column_from: c_idx as i32,
                column_till: (c_idx + valid_colspan) as i32,
            });
            c_idx += valid_colspan;
        }
    }

    TableGrid {
        cells,
        occupancy,
        row_count,
        column_count,
    }
}

/// Instant View table grid layout editor and selection inspector.
#[derive(Debug, Clone)]
pub struct IvEditorPageTableGridEditor {
    pub grid: TableGrid,
    pub selected_cells: Vec<TableGridCellReference>,
    pub is_active: bool,
}

impl Default for IvEditorPageTableGridEditor {
    fn default() -> Self {
        let default_table = vec![
            vec![(1, 1), (1, 1), (1, 1)],
            vec![(1, 1), (1, 1), (1, 1)],
        ];
        let grid = build_table_grid(&default_table);
        Self {
            grid,
            selected_cells: Vec::new(),
            is_active: false,
        }
    }
}

impl IvEditorPageTableGridEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select_range(&mut self, r_from: i32, r_till: i32, c_from: i32, c_till: i32) {
        self.selected_cells = self
            .grid
            .cells
            .iter()
            .filter(|c| c.intersects(r_from, r_till, c_from, c_till))
            .cloned()
            .collect();
    }
}

impl Render for IvEditorPageTableGridEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
                    .child("Instant View Table Grid Inspector")
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .child(format!(
                        "Grid: {} rows x {} columns | Cells: {} | Selected: {}",
                        self.grid.row_count, self.grid.column_count, self.grid.cells.len(), self.selected_cells.len()
                    ))
            )
    }
}
