//! Instant View List Block Engine & Item Numbering (`iv_editor_page_list.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

/// Visual and behavioral style of list blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListStyle {
    #[default]
    Bullet,
    Ordered,
    Task,
}

/// Checkbox state for task list items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskState {
    #[default]
    None,
    Unchecked,
    Checked,
}

/// Single item within a list block.
#[derive(Debug, Clone, Default)]
pub struct ArticleListItem {
    pub text: String,
    pub task_state: TaskState,
    pub explicit_number: Option<i32>,
}

/// Formats integers to Roman numeral representation.
pub fn to_roman_numeral(mut num: i32) -> String {
    if num <= 0 || num > 3999 {
        return num.to_string();
    }
    const ROMANS: &[(i32, &str)] = &[
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"),
    ];
    let mut result = String::new();
    for &(value, symbol) in ROMANS {
        while num >= value {
            result.push_str(symbol);
            num -= value;
        }
    }
    result
}

/// Computes the visual marker prefix string for a list item.
pub fn format_marker(style: ListStyle, index: usize, start: i32) -> String {
    match style {
        ListStyle::Bullet => "•".to_string(),
        ListStyle::Ordered => format!("{}.", start + (index as i32)),
        ListStyle::Task => "[ ]".to_string(),
    }
}

/// Container model for a list block with multiple nested items.
#[derive(Debug, Clone)]
pub struct ArticleListBlock {
    pub style: ListStyle,
    pub start_number: i32,
    pub items: Vec<ArticleListItem>,
}

impl Default for ArticleListBlock {
    fn default() -> Self {
        Self {
            style: ListStyle::Bullet,
            start_number: 1,
            items: vec![ArticleListItem {
                text: "Item 1".to_string(),
                task_state: TaskState::None,
                explicit_number: None,
            }],
        }
    }
}

pub fn lists_joinable(first: &ArticleListBlock, second: &ArticleListBlock) -> bool {
    first.style == second.style
}

pub fn lists_join_seamlessly(first: &mut ArticleListBlock, second: ArticleListBlock) -> bool {
    if !lists_joinable(first, &second) {
        return false;
    }
    first.items.extend(second.items);
    true
}

/// Controller for editing and managing list blocks in the Instant View editor.
#[derive(Debug, Clone)]
pub struct IvEditorPageListEditor {
    pub list: ArticleListBlock,
    pub selected_index: usize,
    pub is_active: bool,
}

impl Default for IvEditorPageListEditor {
    fn default() -> Self {
        Self {
            list: ArticleListBlock::default(),
            selected_index: 0,
            is_active: false,
        }
    }
}

impl IvEditorPageListEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_style(&mut self, style: ListStyle) {
        self.list.style = style;
        for item in &mut self.list.items {
            if style == ListStyle::Task && item.task_state == TaskState::None {
                item.task_state = TaskState::Unchecked;
            } else if style != ListStyle::Task {
                item.task_state = TaskState::None;
            }
        }
    }

    pub fn add_item(&mut self, text: impl Into<String>) {
        let task_state = if self.list.style == ListStyle::Task {
            TaskState::Unchecked
        } else {
            TaskState::None
        };
        self.list.items.push(ArticleListItem {
            text: text.into(),
            task_state,
            explicit_number: None,
        });
        self.selected_index = self.list.items.len() - 1;
    }

    pub fn toggle_task(&mut self, index: usize) -> bool {
        if let Some(item) = self.list.items.get_mut(index) {
            item.task_state = match item.task_state {
                TaskState::Unchecked => TaskState::Checked,
                TaskState::Checked => TaskState::Unchecked,
                TaskState::None => TaskState::Checked,
            };
            true
        } else {
            false
        }
    }
}

impl Render for IvEditorPageListEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let style_label = match self.list.style {
            ListStyle::Bullet => "Bullet List",
            ListStyle::Ordered => "Ordered List",
            ListStyle::Task => "Task Checklist",
        };

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
                    .child(format!("List Editor — {}", style_label))
            )
            .child(
                div()
                    .mt(px(8.0))
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .children(self.list.items.iter().enumerate().map(|(idx, item)| {
                        let marker = format_marker(self.list.style, idx, self.list.start_number);
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_color(rgb(0x2AABEE))
                                    .font_weight(FontWeight::BOLD)
                                    .child(marker)
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .child(item.text.clone())
                            )
                    }))
            )
    }
}
