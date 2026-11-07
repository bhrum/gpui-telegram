//! WindowAdaptive: Responsive column layout and chat presentation breakpoint state machine
//! Upstream: Telegram/SourceFiles/window/window_adaptive.h, window_adaptive.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

pub const ONE_COLUMN_THRESHOLD: f32 = 600.0;
pub const THREE_COLUMN_THRESHOLD: f32 = 1024.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowLayout {
    OneColumn,
    Normal,
    ThreeColumn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatLayout {
    Normal,
    Wide,
}

#[derive(Debug, Clone)]
pub struct WindowAdaptive {
    pub window_layout: WindowLayout,
    pub chat_layout: ChatLayout,
    pub current_width: f32,
    pub current_height: f32,
}

impl WindowAdaptive {
    pub fn new() -> Self {
        Self {
            window_layout: WindowLayout::Normal,
            chat_layout: ChatLayout::Normal,
            current_width: 800.0,
            current_height: 600.0,
        }
    }

    pub fn with_dimensions(width: f32, height: f32) -> Self {
        let mut adaptive = Self {
            window_layout: WindowLayout::Normal,
            chat_layout: ChatLayout::Normal,
            current_width: width,
            current_height: height,
        };
        adaptive.update_from_width(width);
        adaptive
    }

    pub fn update_from_width(&mut self, width: f32) {
        self.current_width = width;
        if width < ONE_COLUMN_THRESHOLD {
            self.window_layout = WindowLayout::OneColumn;
            self.chat_layout = ChatLayout::Normal;
        } else if width < THREE_COLUMN_THRESHOLD {
            self.window_layout = WindowLayout::Normal;
            self.chat_layout = ChatLayout::Normal;
        } else {
            self.window_layout = WindowLayout::ThreeColumn;
            self.chat_layout = ChatLayout::Wide;
        }
    }

    pub fn set_window_layout(&mut self, layout: WindowLayout) {
        self.window_layout = layout;
    }

    pub fn set_chat_layout(&mut self, layout: ChatLayout) {
        self.chat_layout = layout;
    }

    pub fn is_one_column(&self) -> bool {
        self.window_layout == WindowLayout::OneColumn
    }

    pub fn is_normal(&self) -> bool {
        self.window_layout == WindowLayout::Normal
    }

    pub fn is_three_column(&self) -> bool {
        self.window_layout == WindowLayout::ThreeColumn
    }

    pub fn is_chat_wide(&self) -> bool {
        self.chat_layout == ChatLayout::Wide
    }
}

impl Default for WindowAdaptive {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowAdaptive {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap(px(8.0))
            .p(px(8.0))
            .child(format!("Layout: {:?}", self.window_layout))
            .child(format!("Chat: {:?}", self.chat_layout))
            .child(format!("Width: {:.0}px", self.current_width))
    }
}
