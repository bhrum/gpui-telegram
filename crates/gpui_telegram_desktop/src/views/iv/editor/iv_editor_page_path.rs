//! Instant View Block & Leaf Navigation Hierarchy Paths (`iv_editor_page_path.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

/// Classification of parent block containers in an article.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BlockContainerKind {
    #[default]
    Root,
    BlockChildren,
    ListItemChildren,
}

/// Single step traversal down the article container hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BlockContainerStep {
    pub kind: BlockContainerKind,
    pub block_index: i32,
    pub list_item_index: i32,
}

/// Full path identifying a nested block container.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BlockContainerPath {
    pub steps: Vec<BlockContainerStep>,
}

/// Precise path identifying a specific block within a container.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BlockPath {
    pub container: BlockContainerPath,
    pub index: i32,
}

/// Target description for replacing an existing block or media item in-place.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReplaceTarget {
    pub path: BlockPath,
    pub kind_name: String,
    pub media_id: u64,
    pub item_index: i32,
}

/// Specific leaf element within a block that can hold text focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LeafKind {
    #[default]
    BlockText,
    BlockCaption,
    ListItemText,
    TableCellText,
    MathFormula,
}

/// Exact address of an editable text leaf in the document.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LeafPath {
    pub kind: LeafKind,
    pub block: BlockPath,
    pub list_item_index: i32,
    pub table_row_index: i32,
    pub table_cell_index: i32,
}

pub fn block_children_container(path: &BlockPath) -> BlockContainerPath {
    let mut steps = path.container.steps.clone();
    steps.push(BlockContainerStep {
        kind: BlockContainerKind::BlockChildren,
        block_index: path.index,
        list_item_index: -1,
    });
    BlockContainerPath { steps }
}

pub fn list_item_children_container(path: &BlockPath, item_index: i32) -> BlockContainerPath {
    let mut steps = path.container.steps.clone();
    steps.push(BlockContainerStep {
        kind: BlockContainerKind::ListItemChildren,
        block_index: path.index,
        list_item_index: item_index,
    });
    BlockContainerPath { steps }
}

/// Navigation path tracker and breadcrumb renderer for the IV editor.
#[derive(Debug, Clone)]
pub struct IvEditorPagePathEditor {
    pub current_leaf: LeafPath,
    pub navigation_history: Vec<LeafPath>,
    pub is_active: bool,
}

impl Default for IvEditorPagePathEditor {
    fn default() -> Self {
        Self {
            current_leaf: LeafPath {
                kind: LeafKind::BlockText,
                block: BlockPath {
                    container: BlockContainerPath::default(),
                    index: 0,
                },
                list_item_index: -1,
                table_row_index: -1,
                table_cell_index: -1,
            },
            navigation_history: Vec::new(),
            is_active: false,
        }
    }
}

impl IvEditorPagePathEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn navigate_to(&mut self, leaf: LeafPath) {
        self.navigation_history.push(self.current_leaf.clone());
        self.current_leaf = leaf;
    }

    pub fn breadcrumb_string(&self) -> String {
        let mut parts = vec!["Document".to_string()];
        for step in &self.current_leaf.block.container.steps {
            match step.kind {
                BlockContainerKind::Root => parts.push("Root".to_string()),
                BlockContainerKind::BlockChildren => parts.push(format!("Block[{}]", step.block_index)),
                BlockContainerKind::ListItemChildren => {
                    parts.push(format!("Block[{}] > Item[{}]", step.block_index, step.list_item_index))
                }
            }
        }
        parts.push(format!("Block[{}]", self.current_leaf.block.index));
        match self.current_leaf.kind {
            LeafKind::BlockText => parts.push("Text".to_string()),
            LeafKind::BlockCaption => parts.push("Caption".to_string()),
            LeafKind::ListItemText => parts.push(format!("ItemText[{}]", self.current_leaf.list_item_index)),
            LeafKind::TableCellText => {
                parts.push(format!("Cell[{}, {}]", self.current_leaf.table_row_index, self.current_leaf.table_cell_index))
            }
            LeafKind::MathFormula => parts.push("Formula".to_string()),
        }
        parts.join(" > ")
    }
}

impl Render for IvEditorPagePathEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let breadcrumb = self.breadcrumb_string();
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
                    .child("Instant View Navigation Path")
            )
            .child(
                div()
                    .mt(px(4.0))
                    .text_size(px(12.0))
                    .text_color(rgb(0x2AABEE))
                    .child(breadcrumb)
            )
    }
}
