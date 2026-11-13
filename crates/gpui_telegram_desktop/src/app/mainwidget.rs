//! Responsive 3-column / 2-column / 1-column layout coordinator and main view
//! Upstream: Telegram/SourceFiles/mainwidget.h, mainwidget.cpp
//! Subsystem: main | Feature ID: TCG-N46 | Disposition: gpui_ui_1to1

use serde::{Deserialize, Serialize};

/// Responsive column layout mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LayoutMode {
    #[default]
    ThreeColumn, // Width >= 1200: Dialogs | History | ThirdSection
    TwoColumn,   // Width >= 800:  Dialogs | History (ThirdSection as modal layer)
    OneColumn,   // Width < 800:   Single stack view (Dialogs OR History)
}

/// Navigation stack frame for chats and section mementos.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatStackItem {
    pub peer_id: u64,
    pub topic_root_id: Option<i64>,
    pub focus_msg_id: Option<i64>,
}

/// Main widget managing the responsive split-screen conversation stage and navigation stack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mainwidget {
    pub width: i32,
    pub height: i32,
    pub layout_mode: LayoutMode,
    pub dialogs_width: i32,
    pub third_section_width: i32,
    pub third_section_open: bool,
    pub current_peer_id: Option<u64>,
    pub chat_stack: Vec<ChatStackItem>,
    pub is_primary: bool,
}

impl Default for Mainwidget {
    fn default() -> Self {
        Self::new(1280, 800)
    }
}

impl Mainwidget {
    /// Creates a new main widget with initial window dimensions.
    pub fn new(initial_width: i32, initial_height: i32) -> Self {
        let mut widget = Self {
            width: initial_width,
            height: initial_height,
            layout_mode: LayoutMode::ThreeColumn,
            dialogs_width: 360,
            third_section_width: 360,
            third_section_open: false,
            current_peer_id: None,
            chat_stack: Vec::new(),
            is_primary: true,
        };
        widget.update_column_layout();
        widget
    }

    /// Updates container dimensions and recomputes the responsive column mode.
    /// In accordance with C++ `MainWidget::updateColumnLayout`:
    pub fn update_column_layout(&mut self) {
        if self.width >= 1200 {
            self.layout_mode = LayoutMode::ThreeColumn;
            self.dialogs_width = 360;
            self.third_section_width = 360;
        } else if self.width >= 800 {
            self.layout_mode = LayoutMode::TwoColumn;
            self.dialogs_width = 320;
            self.third_section_width = 0; // Third section opens in overlay layer
        } else {
            self.layout_mode = LayoutMode::OneColumn;
            self.dialogs_width = self.width;
            self.third_section_width = 0;
        }
    }

    /// Resizes the widget.
    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.update_column_layout();
    }

    /// Returns the bounding rectangle `(x, y, width, height)` of the left dialogs list.
    pub fn dialogs_rect(&self) -> (i32, i32, i32, i32) {
        match self.layout_mode {
            LayoutMode::ThreeColumn | LayoutMode::TwoColumn => {
                (0, 0, self.dialogs_width, self.height)
            }
            LayoutMode::OneColumn => {
                if self.current_peer_id.is_none() {
                    (0, 0, self.width, self.height)
                } else {
                    (0, 0, 0, 0) // Hidden when history is active in single column
                }
            }
        }
    }

    /// Returns the bounding rectangle `(x, y, width, height)` of the central history stage.
    pub fn history_rect(&self) -> (i32, i32, i32, i32) {
        match self.layout_mode {
            LayoutMode::ThreeColumn => {
                let right_w = if self.third_section_open {
                    self.third_section_width
                } else {
                    0
                };
                let center_w = (self.width - self.dialogs_width - right_w).max(0);
                (self.dialogs_width, 0, center_w, self.height)
            }
            LayoutMode::TwoColumn => {
                let center_w = (self.width - self.dialogs_width).max(0);
                (self.dialogs_width, 0, center_w, self.height)
            }
            LayoutMode::OneColumn => {
                if self.current_peer_id.is_some() {
                    (0, 0, self.width, self.height)
                } else {
                    (0, 0, 0, 0)
                }
            }
        }
    }

    /// Returns the bounding rectangle of the right drawer (third section) if visible.
    pub fn third_section_rect(&self) -> Option<(i32, i32, i32, i32)> {
        if self.layout_mode == LayoutMode::ThreeColumn && self.third_section_open {
            let left = self.width - self.third_section_width;
            Some((left, 0, self.third_section_width, self.height))
        } else {
            None
        }
    }

    /// Selects a chat to display in the conversation stage.
    pub fn show_history(&mut self, peer_id: u64, _focus_msg_id: Option<i64>) {
        if let Some(prev) = self.current_peer_id {
            if prev != peer_id {
                self.chat_stack.push(ChatStackItem {
                    peer_id: prev,
                    topic_root_id: None,
                    focus_msg_id: None,
                });
            }
        }
        self.current_peer_id = Some(peer_id);
    }

    /// Navigates back one step in the chat history stack.
    /// In accordance with C++ `MainWidget::showBackFromStack`:
    pub fn show_back_from_stack(&mut self) -> bool {
        if let Some(item) = self.chat_stack.pop() {
            self.current_peer_id = Some(item.peer_id);
            true
        } else if self.layout_mode == LayoutMode::OneColumn && self.current_peer_id.is_some() {
            self.current_peer_id = None; // Return to dialogs list
            true
        } else {
            false
        }
    }

    /// Opens the collapsible right info drawer.
    pub fn open_third_section(&mut self) {
        self.third_section_open = true;
    }

    /// Closes the collapsible right info drawer.
    pub fn close_third_section(&mut self) {
        self.third_section_open = false;
    }

    /// Returns true if the navigation stack is empty.
    #[inline]
    pub fn stack_is_empty(&self) -> bool {
        self.chat_stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_responsive_layout_modes() {
        let mut widget = Mainwidget::new(1400, 900);
        assert_eq!(widget.layout_mode, LayoutMode::ThreeColumn);

        let (dx, dy, dw, dh) = widget.dialogs_rect();
        assert_eq!((dx, dy, dw, dh), (0, 0, 360, 900));

        widget.open_third_section();
        let (hx, hy, hw, hh) = widget.history_rect();
        assert_eq!(hw, 1400 - 360 - 360);
        assert_eq!(hx, 360);

        // Resize to two-column
        widget.resize(900, 800);
        assert_eq!(widget.layout_mode, LayoutMode::TwoColumn);
        assert!(widget.third_section_rect().is_none());

        // Resize to mobile one-column
        widget.resize(600, 800);
        assert_eq!(widget.layout_mode, LayoutMode::OneColumn);
        assert_eq!(widget.dialogs_rect().2, 600); // Dialogs occupies full width

        widget.show_history(555, None);
        assert_eq!(widget.dialogs_rect().2, 0); // Hidden
        assert_eq!(widget.history_rect().2, 600); // History occupies full width

        assert!(widget.show_back_from_stack());
        assert_eq!(widget.current_peer_id, None);
        assert_eq!(widget.dialogs_rect().2, 600);
    }
}
