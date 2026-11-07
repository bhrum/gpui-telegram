//! Genuine History View Todo List Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_todo_list.h:L24-L150`
//! - `Telegram/SourceFiles/history/view/media/history_view_todo_list.cpp:L50-L150`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::todo::{RichTodoListCard, TodoTaskItem};

#[derive(Debug, Clone)]
pub struct HistoryViewTodoListViewMedia {
    pub title: String,
    pub tasks: Vec<TodoTaskItem>,
}

impl HistoryViewTodoListViewMedia {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, id: u32, text: impl Into<String>, is_completed: bool) {
        self.tasks.push(TodoTaskItem {
            id,
            text: text.into(),
            is_completed,
            completed_by: None,
            completed_date: 0,
        });
    }

    pub fn toggle(&mut self, id: u32, by: Option<&str>) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.is_completed = !task.is_completed;
            task.completed_by = if task.is_completed { by.map(|s| s.to_string()) } else { None };
        }
    }

    pub fn is_all_done(&self) -> bool {
        !self.tasks.is_empty() && self.tasks.iter().all(|t| t.is_completed)
    }
}

impl Default for HistoryViewTodoListViewMedia {
    fn default() -> Self {
        Self::new("Tasks")
    }
}

impl Render for HistoryViewTodoListViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut card = RichTodoListCard::new(&self.title);
        card.tasks = self.tasks.clone();
        card.render()
    }
}
