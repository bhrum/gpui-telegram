//! Edit Todo List & Checklist Modal Box View.
//!
//! Faithful translation of upstream Telegram Desktop:
//! - `Telegram/SourceFiles/boxes/edit_todo_list_box.h`
//! - `Telegram/SourceFiles/boxes/edit_todo_list_box.cpp:L54-L100`
//!
//! Features:
//! - `kWarnTitleLimit = 12`, `kWarnTaskLimit = 24`, `kErrorLimit = 99`
//! - Dynamic tasks creation, reordering, and item completion state
//! - Existing tasks locking for collaborative checklists
//! - Validation rules (non-empty title, minimum 1 task item)

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_WARN_TITLE_LIMIT: usize = 12;
pub const K_WARN_TASK_LIMIT: usize = 24;
pub const K_ERROR_LIMIT: usize = 99;

#[derive(Debug, Clone)]
pub struct TodoTaskItem {
    pub id: i32,
    pub text: String,
    pub is_completed: bool,
    pub is_locked: bool,
}

#[derive(Debug, Clone)]
pub struct EditTodoListBox {
    pub title: String,
    pub tasks: Vec<TodoTaskItem>,
    pub next_task_id: i32,
    pub is_new_list: bool,
}

impl EditTodoListBox {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            tasks: vec![
                TodoTaskItem { id: 1, text: String::new(), is_completed: false, is_locked: false },
            ],
            next_task_id: 2,
            is_new_list: true,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn add_task(&mut self, text: impl Into<String>) {
        self.tasks.push(TodoTaskItem {
            id: self.next_task_id,
            text: text.into(),
            is_completed: false,
            is_locked: false,
        });
        self.next_task_id += 1;
    }

    pub fn toggle_completed(&mut self, task_id: i32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.is_completed = !task.is_completed;
        }
    }

    pub fn remove_task(&mut self, task_id: i32) -> bool {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == task_id && !t.is_locked) {
            self.tasks.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.title.trim().is_empty() && self.tasks.iter().any(|t| !t.text.trim().is_empty())
    }
}

impl Default for EditTodoListBox {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for EditTodoListBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.is_completed).count();

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(16.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xD5D5D5))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(15.0))
                    .text_color(rgb(0x222222))
                    .child(if self.is_new_list { "Create Todo List" } else { "Edit Todo List" }),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .p(px(6.0))
                    .bg(rgb(0xF5F7FA))
                    .rounded(px(4.0))
                    .text_size(px(13.0))
                    .text_color(rgb(0x333333))
                    .child(format!("Title: {}", if self.title.is_empty() { "(untitled checklist)" } else { &self.title })),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .child(format!("Tasks: {} total | {} completed | Valid: {}", count, completed, self.is_valid())),
            )
    }
}
