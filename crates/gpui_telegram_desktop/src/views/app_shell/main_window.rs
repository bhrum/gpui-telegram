//! MainWindow: Top-level desktop window frame, titlebar, and work mode coordinator
//! Upstream: Telegram/SourceFiles/window/main_window.h, main_window.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkMode {
    Normal,
    TrayOnly,
    WindowAndTray,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub maximized: bool,
}

impl Default for WindowPosition {
    fn default() -> Self {
        Self {
            x: 100,
            y: 100,
            width: 1024,
            height: 768,
            maximized: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MainWindow {
    pub title: String,
    pub subtitle: Option<String>,
    pub position: WindowPosition,
    pub is_active: bool,
    pub is_minimized: bool,
    pub is_maximized: bool,
    pub is_fullscreen: bool,
    pub work_mode: WorkMode,
    pub show_right_column: bool,
    pub unread_badge: i32,
    pub screen_reader_active: bool,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            title: "Telegram".to_string(),
            subtitle: None,
            position: WindowPosition::default(),
            is_active: true,
            is_minimized: false,
            is_maximized: false,
            is_fullscreen: false,
            work_mode: WorkMode::WindowAndTray,
            show_right_column: false,
            unread_badge: 0,
            screen_reader_active: false,
        }
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_subtitle(&mut self, subtitle: Option<String>) {
        self.subtitle = subtitle;
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn toggle_visibility(&mut self) {
        self.is_minimized = !self.is_minimized;
    }

    pub fn minimize(&mut self) {
        self.is_minimized = true;
    }

    pub fn unminimize(&mut self) {
        self.is_minimized = false;
        self.is_active = true;
    }

    pub fn toggle_maximize(&mut self) {
        self.is_maximized = !self.is_maximized;
        self.position.maximized = self.is_maximized;
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.is_fullscreen = fullscreen;
    }

    pub fn show_right_column(&mut self) {
        self.show_right_column = true;
    }

    pub fn hide_right_column(&mut self) {
        self.show_right_column = false;
    }

    pub fn toggle_right_column(&mut self) {
        self.show_right_column = !self.show_right_column;
    }

    pub fn set_unread_badge(&mut self, count: i32) {
        self.unread_badge = count.max(0);
    }

    pub fn apply_saved_position(&mut self, pos: WindowPosition) {
        self.is_maximized = pos.maximized;
        self.position = pos;
    }

    pub fn hide_no_quit(&mut self) -> bool {
        if self.work_mode == WorkMode::TrayOnly || self.work_mode == WorkMode::WindowAndTray {
            self.is_minimized = true;
            true
        } else {
            false
        }
    }

    pub fn prevent_or_invoke<F: FnOnce()>(&mut self, callback: F) {
        if self.is_active {
            callback();
        }
    }

    pub fn create_screen_reader_bar(&mut self) {
        self.screen_reader_active = true;
    }

    pub fn is_x11(&self) -> bool {
        false
    }

    pub fn default_application_icon(&self) -> &'static str {
        "assets/icons/telegram_desktop.png"
    }
}

impl Default for MainWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let title_text = if self.unread_badge > 0 {
            format!("({}) {}", self.unread_badge, self.title)
        } else {
            self.title.clone()
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0E1621))
            .text_color(rgb(0xFFFFFF))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(38.0))
                    .px(px(12.0))
                    .bg(rgb(0x18222D))
                    .border_b_1()
                    .border_color(rgb(0x242F3D))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(div().font_weight(FontWeight::SEMIBOLD).text_sm().child(title_text))
                            .children(self.subtitle.as_ref().map(|s| {
                                div().text_xs().text_color(rgb(0x82868A)).child(s.clone())
                            }))
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .child(div().text_xs().text_color(rgb(0x6C7883)).child(if self.is_active { "Active" } else { "Inactive" }))
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .size_full()
                    .child(div().p(px(16.0)).child("Telegram Desktop App Stage"))
            )
    }
}
