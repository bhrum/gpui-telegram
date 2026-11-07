//! Rich Todo List & Interactive Checklist Cards (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_todo_list.cpp:L50-L150`

use gpui::*;
use gpui::prelude::FluentBuilder;

#[derive(Debug, Clone)]
pub struct TodoTaskItem {
    pub id: u32,
    pub text: String,
    pub is_completed: bool,
    pub completed_by: Option<String>,
    pub completed_date: u64,
}

#[derive(Debug, Clone)]
pub struct RichTodoListCard {
    pub title: String,
    pub tasks: Vec<TodoTaskItem>,
}

impl RichTodoListCard {
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

    pub fn toggle_task(&mut self, id: u32, by: Option<&str>) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.is_completed = !task.is_completed;
            if task.is_completed {
                task.completed_by = by.map(|s| s.to_string());
                task.completed_date = chrono::Utc::now().timestamp() as u64;
            } else {
                task.completed_by = None;
                task.completed_date = 0;
            }
        }
    }

    pub fn completed_count(&self) -> usize {
        self.tasks.iter().filter(|t| t.is_completed).count()
    }

    pub fn total_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_all_completed(&self) -> bool {
        !self.tasks.is_empty() && self.completed_count() == self.total_count()
    }

    pub fn render(self) -> impl IntoElement {
        let completed = self.completed_count();
        let total = self.total_count();
        let all_done = self.is_all_completed();
        let title = self.title;
        let tasks = self.tasks;

        div()
            .flex()
            .flex_col()
            .w(px(340.0))
            .p(px(14.0))
            .rounded(px(12.0))
            .bg(rgb(0x182533))
            .border_1()
            .border_color(if all_done { rgb(0x4CAF50) } else { rgb(0x233140) })
            .gap(px(10.0))
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(15.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(title),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(if all_done { rgb(0x4CAF50) } else { rgb(0x8A9AA8) })
                            .child(format!("{}/{}", completed, total)),
                    ),
            )
            // Progress Bar
            .child(
                div()
                    .w_full()
                    .h(px(4.0))
                    .rounded_full()
                    .bg(rgb(0x101921))
                    .child(
                        div()
                            .w(px(if total > 0 { ((completed as f32 / total as f32) * 312.0).max(4.0) } else { 0.0 }))
                            .h(px(4.0))
                            .rounded_full()
                            .bg(if all_done { rgb(0x4CAF50) } else { rgb(0x2AABEE) }),
                    ),
            )
            // Task List
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .children(tasks.into_iter().map(|task| {
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .p(px(6.0))
                            .rounded(px(6.0))
                            .bg(if task.is_completed { rgba(0x4CAF5015) } else { rgba(0x00000000) })
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .w(px(18.0))
                                            .h(px(18.0))
                                            .rounded(px(4.0))
                                            .border_2()
                                            .border_color(if task.is_completed { rgb(0x4CAF50) } else { rgb(0x6B808D) })
                                            .bg(if task.is_completed { rgb(0x4CAF50) } else { rgba(0x00000000) })
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(11.0))
                                            .text_color(rgb(0xFFFFFF))
                                            .child(if task.is_completed { "✓" } else { "" }),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .text_color(if task.is_completed { rgb(0x8A9AA8) } else { rgb(0xFFFFFF) })
                                            .child(task.text),
                                    ),
                            )
                            .when_some(task.completed_by, |el, by| {
                                el.child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(rgb(0x4CAF50))
                                        .child(format!("done by {}", by)),
                                )
                            })
                    })),
            )
    }
}
