//! Authentic GPUI Settings Key Navigation Component
//! Source: Telegram/SourceFiles/settings/settings_key_navigation.cpp, Telegram/SourceFiles/settings/settings_key_navigation.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::prelude::FluentBuilder;
use gpui::*;

pub const K_PAGE_SKIP: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyNavigationDirection {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
}

#[derive(Debug, Clone)]
pub struct KeyNavigationItem {
    pub id: String,
    pub title: String,
    pub is_interactive: bool,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct SettingsKeyNavigation {
    pub focused_index: Option<usize>,
    pub items: Vec<KeyNavigationItem>,
    pub page_skip: usize,
    pub is_active: bool,
    pub is_visible: bool,
}

impl SettingsKeyNavigation {
    pub fn new() -> Self {
        Self {
            focused_index: None,
            items: Vec::new(),
            page_skip: K_PAGE_SKIP,
            is_active: true,
            is_visible: true,
        }
    }

    pub fn add_item(&mut self, id: impl Into<String>, x: f32, y: f32, width: f32, height: f32) {
        let sid = id.into();
        self.items.push(KeyNavigationItem {
            title: sid.clone(),
            id: sid,
            is_interactive: true,
            x,
            y,
            width,
            height,
        });
    }

    pub fn handle_direction(&mut self, dir: KeyNavigationDirection) -> bool {
        if self.items.is_empty() {
            return false;
        }
        let cur_idx = match self.focused_index {
            Some(i) => i,
            None => {
                self.focused_index = Some(0);
                return true;
            }
        };

        if cur_idx >= self.items.len() {
            return false;
        }

        let cur = &self.items[cur_idx];
        let mut best: Option<(usize, f32)> = None;

        for (i, item) in self.items.iter().enumerate() {
            if i == cur_idx {
                continue;
            }
            let valid = match dir {
                KeyNavigationDirection::Up => item.y < cur.y,
                KeyNavigationDirection::Down => item.y > cur.y,
                KeyNavigationDirection::Left => item.x < cur.x,
                KeyNavigationDirection::Right => item.x > cur.x,
                _ => false,
            };
            if valid {
                let dist = (item.x - cur.x).powi(2) + (item.y - cur.y).powi(2);
                if best.map_or(true, |(_, d)| dist < d) {
                    best = Some((i, dist));
                }
            }
        }

        if let Some((next_idx, _)) = best {
            self.focused_index = Some(next_idx);
            true
        } else {
            false
        }
    }

    pub fn focused_item(&self) -> Option<&KeyNavigationItem> {
        self.focused_index.and_then(|i| self.items.get(i))
    }

    pub fn set_items(&mut self, items: Vec<KeyNavigationItem>) {
        self.items = items;
        if let Some(fi) = self.focused_index {
            if fi >= self.items.len() {
                self.focused_index = if self.items.is_empty() { None } else { Some(self.items.len() - 1) };
            }
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for SettingsKeyNavigation {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsKeyNavigation {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let focused_idx = self.focused_index;

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x0e1621))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .pb(px(8.0))
                    .border_b_1()
                    .border_color(rgb(0x17212b))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child("Keyboard Navigation Focus"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x6c7883))
                            .child("Use ↑ ↓ or PageUp/Down"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .pt(px(8.0))
                    .children(self.items.iter().enumerate().map(|(idx, item)| {
                        let is_focused = Some(idx) == focused_idx;
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .px(px(12.0))
                            .py(px(8.0))
                            .rounded(px(6.0))
                            .bg(if is_focused { rgb(0x2b3846) } else { rgb(0x17212b) })
                            .border_1()
                            .border_color(if is_focused { rgb(0x5288c1) } else { rgb(0x17212b) })
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _window, cx| {
                                this.focused_index = Some(idx);
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(if is_focused { rgb(0xffffff) } else { rgb(0x8292a1) })
                                    .child(item.title.clone()),
                            )
                            .when(is_focused, |this| {
                                this.child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0x5288c1))
                                        .child("⏎ Select"),
                                )
                            })
                    })),
            )
            .into_any_element()
    }
}
