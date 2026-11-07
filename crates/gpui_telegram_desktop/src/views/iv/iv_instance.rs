//! Target Symbol: IvInstance
//! Module 13: Instant View Master Service Coordinator
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const ALLOW_PAGE_RELOAD_AFTER_MS: u64 = 1000;

#[derive(Debug, Clone)]
pub struct ActiveIvWindow {
    pub id: u64,
    pub webpage_id: u64,
    pub url: String,
    pub title: String,
    pub is_focused: bool,
    pub is_minimized: bool,
}

#[derive(Debug, Clone, Default)]
pub struct IvInstance {
    pub windows: Vec<ActiveIvWindow>,
    pub active_window_id: Option<u64>,
    pub next_id: u64,
}

impl IvInstance {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            active_window_id: None,
            next_id: 1,
        }
    }

    pub fn is_iv_preferred(url: &str) -> bool {
        let lower = url.to_lowercase();
        lower.contains("t.me/iv?")
            || lower.contains("telegram.org/")
            || lower.starts_with("ton://")
            || lower.contains(".ton/")
    }

    pub fn open_page(&mut self, webpage_id: u64, url: impl Into<String>, title: impl Into<String>) -> u64 {
        let u = url.into();
        let t = title.into();

        if let Some(existing) = self.windows.iter_mut().find(|w| w.webpage_id == webpage_id && webpage_id != 0) {
            existing.is_focused = true;
            existing.is_minimized = false;
            let id = existing.id;
            self.active_window_id = Some(id);
            return id;
        }

        let id = self.next_id;
        self.next_id += 1;
        self.windows.push(ActiveIvWindow {
            id,
            webpage_id,
            url: u,
            title: t,
            is_focused: true,
            is_minimized: false,
        });
        self.active_window_id = Some(id);
        id
    }

    pub fn focus_window(&mut self, id: u64) -> bool {
        if let Some(w) = self.windows.iter_mut().find(|w| w.id == id) {
            w.is_focused = true;
            w.is_minimized = false;
            self.active_window_id = Some(id);
            true
        } else {
            false
        }
    }

    pub fn close_window(&mut self, id: u64) -> bool {
        if let Some(pos) = self.windows.iter().position(|w| w.id == id) {
            self.windows.remove(pos);
            if self.active_window_id == Some(id) {
                self.active_window_id = self.windows.last().map(|w| w.id);
            }
            true
        } else {
            false
        }
    }

    pub fn close_active(&mut self) -> bool {
        if let Some(id) = self.active_window_id {
            self.close_window(id)
        } else {
            false
        }
    }

    pub fn minimize_active(&mut self) -> bool {
        if let Some(id) = self.active_window_id {
            if let Some(w) = self.windows.iter_mut().find(|w| w.id == id) {
                w.is_minimized = true;
                w.is_focused = false;
                return true;
            }
        }
        false
    }

    pub fn close_all(&mut self) {
        self.windows.clear();
        self.active_window_id = None;
    }

    pub fn has_active_window(&self) -> bool {
        !self.windows.is_empty()
    }
}

impl Render for IvInstance {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut tabs = div()
            .flex()
            .flex_row()
            .w_full()
            .h(px(38.0))
            .bg(rgb(0x181B22))
            .border_b_1()
            .border_color(rgb(0x282E38))
            .items_center()
            .px(px(6.0))
            .gap(px(4.0));

        for win in &self.windows {
            let is_active = self.active_window_id == Some(win.id);
            let (bg, text_col) = if is_active {
                (rgb(0x242B36), rgb(0xFFFFFF))
            } else {
                (rgb(0x1C212A), rgb(0x8C94A2))
            };

            tabs = tabs.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .px(px(10.0))
                    .py(px(4.0))
                    .rounded(px(6.0))
                    .bg(bg)
                    .text_color(text_col)
                    .text_size(px(12.0))
                    .gap(px(6.0))
                    .child(
                        div()
                            .max_w(px(160.0))
                            .child(win.title.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(rgb(0x5A6270))
                            .hover(|s| s.text_color(rgb(0xED6A5E)))
                            .child("✕"),
                    ),
            );
        }

        if self.windows.is_empty() {
            tabs = tabs.child(
                div()
                    .px(px(10.0))
                    .text_size(px(12.0))
                    .text_color(rgb(0x525A68))
                    .child("No active Instant View windows"),
            );
        }

        tabs
    }
}
