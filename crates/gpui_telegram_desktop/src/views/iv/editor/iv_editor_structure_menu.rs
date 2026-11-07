//! Target Symbol: IvEditorStructureMenuEditor
//! Upstream: Telegram/SourceFiles/iv/editor/iv_editor_structure_menu.h, Telegram/SourceFiles/iv/editor/iv_editor_structure_menu.cpp
//! Module 13: Instant View Editor Structure Menu Subsystem
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListFormatType {
    #[default]
    Bullet,
    Numbers,
    LowercaseLetters,
    UppercaseLetters,
    LowercaseRoman,
    UppercaseRoman,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableCellAlignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructureMenuAction {
    ChangeListType(ListFormatType),
    IndentListItem,
    OutdentListItem,
    AddRowAbove,
    AddRowBelow,
    AddColumnLeft,
    AddColumnRight,
    DeleteRow,
    DeleteColumn,
    DeleteTable,
    SetAlignment(TableCellAlignment),
    ToggleHeaderRow,
}

#[derive(Debug, Clone)]
pub struct StructureMenuItem {
    pub action: StructureMenuAction,
    pub title: String,
    pub shortcut: Option<String>,
    pub is_checked: bool,
    pub is_destructive: bool,
}

#[derive(Debug, Clone)]
pub struct PreparedEditTableCellRange {
    pub start_row: usize,
    pub start_col: usize,
    pub row_count: usize,
    pub col_count: usize,
}

#[derive(Debug, Clone)]
pub struct PreparedListItemRange {
    pub item_index: usize,
    pub depth: usize,
    pub is_ordered: bool,
}

#[derive(Debug, Clone)]
pub struct IvEditorStructureMenuEditor {
    pub is_visible: bool,
    pub menu_title: String,
    pub items: Vec<StructureMenuItem>,
    pub selected_action: Option<StructureMenuAction>,
    pub current_list_type: ListFormatType,
    pub current_alignment: TableCellAlignment,
}

impl Default for IvEditorStructureMenuEditor {
    fn default() -> Self {
        Self {
            is_visible: false,
            menu_title: "Formatting".to_string(),
            items: Vec::new(),
            selected_action: None,
            current_list_type: ListFormatType::Bullet,
            current_alignment: TableCellAlignment::Left,
        }
    }
}

impl IvEditorStructureMenuEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fill_list_change_menu(&mut self, current: ListFormatType) {
        self.menu_title = "List Type".to_string();
        self.current_list_type = current;
        self.items = vec![
            StructureMenuItem {
                action: StructureMenuAction::ChangeListType(ListFormatType::Bullet),
                title: "Bullet List".to_string(),
                shortcut: None,
                is_checked: current == ListFormatType::Bullet,
                is_destructive: false,
            },
            StructureMenuItem {
                action: StructureMenuAction::ChangeListType(ListFormatType::Numbers),
                title: "Numbered (1, 2, 3)".to_string(),
                shortcut: None,
                is_checked: current == ListFormatType::Numbers,
                is_destructive: false,
            },
            StructureMenuItem {
                action: StructureMenuAction::ChangeListType(ListFormatType::LowercaseLetters),
                title: "Letters (a, b, c)".to_string(),
                shortcut: None,
                is_checked: current == ListFormatType::LowercaseLetters,
                is_destructive: false,
            },
            StructureMenuItem {
                action: StructureMenuAction::ChangeListType(ListFormatType::LowercaseRoman),
                title: "Roman Numerals (i, ii, iii)".to_string(),
                shortcut: None,
                is_checked: current == ListFormatType::LowercaseRoman,
                is_destructive: false,
            },
        ];
        self.is_visible = true;
    }

    pub fn fill_table_change_menu(&mut self, alignment: TableCellAlignment) {
        self.menu_title = "Table Structure".to_string();
        self.current_alignment = alignment;
        self.items = vec![
            StructureMenuItem {
                action: StructureMenuAction::AddRowAbove,
                title: "Insert Row Above".to_string(),
                shortcut: None,
                is_checked: false,
                is_destructive: false,
            },
            StructureMenuItem {
                action: StructureMenuAction::AddRowBelow,
                title: "Insert Row Below".to_string(),
                shortcut: None,
                is_checked: false,
                is_destructive: false,
            },
            StructureMenuItem {
                action: StructureMenuAction::AddColumnLeft,
                title: "Insert Column Left".to_string(),
                shortcut: None,
                is_checked: false,
                is_destructive: false,
            },
            StructureMenuItem {
                action: StructureMenuAction::AddColumnRight,
                title: "Insert Column Right".to_string(),
                shortcut: None,
                is_checked: false,
                is_destructive: false,
            },
            StructureMenuItem {
                action: StructureMenuAction::DeleteRow,
                title: "Delete Row".to_string(),
                shortcut: None,
                is_checked: false,
                is_destructive: true,
            },
            StructureMenuItem {
                action: StructureMenuAction::DeleteColumn,
                title: "Delete Column".to_string(),
                shortcut: None,
                is_checked: false,
                is_destructive: true,
            },
            StructureMenuItem {
                action: StructureMenuAction::DeleteTable,
                title: "Delete Table".to_string(),
                shortcut: None,
                is_checked: false,
                is_destructive: true,
            },
        ];
        self.is_visible = true;
    }

    pub fn select_item(&mut self, index: usize) -> Option<StructureMenuAction> {
        if let Some(item) = self.items.get(index) {
            let act = item.action.clone();
            self.selected_action = Some(act.clone());
            self.is_visible = false;
            Some(act)
        } else {
            None
        }
    }
}

impl Render for IvEditorStructureMenuEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let mut list_container = div()
            .flex()
            .flex_col()
            .w(px(220.0))
            .bg(rgb(0x232E3C))
            .rounded(px(8.0))
            .shadow_md()
            .p(px(6.0))
            .gap(px(2.0))
            .child(
                div()
                    .px(px(8.0))
                    .py(px(4.0))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x82868E))
                    .child(self.menu_title.clone()),
            );

        for (idx, item) in self.items.iter().enumerate() {
            let text_color = if item.is_destructive {
                rgb(0xEE6868)
            } else if item.is_checked {
                rgb(0x5288C1)
            } else {
                rgb(0xF5F5F5)
            };

            let prefix = if item.is_checked { "✓ " } else { "  " };

            list_container = list_container.child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .px(px(8.0))
                    .py(px(5.0))
                    .rounded(px(4.0))
                    .hover(|s| s.bg(rgb(0x2F3B4C)))
                    .text_size(px(13.0))
                    .text_color(text_color)
                    .child(format!("{}{}", prefix, item.title))
            );
        }

        list_container.into_any_element()
    }
}
