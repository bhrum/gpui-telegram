//! Authentic HistoryView Keyboard Text Selection Component (`history_view_keyboard_text_selection.h/cpp`).
//! Upstream reference: `Telegram/SourceFiles/history/view/history_view_keyboard_text_selection.cpp:L16-L112`

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionKey {
    Left,
    Right,
    Home,
    End,
    Other,
}

impl SelectionKey {
    pub fn is_extend_key(&self) -> bool {
        matches!(self, Self::Left | Self::Right | Self::Home | Self::End)
    }
}

/// Text selection range (start and end character indices).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TextSelectionRange {
    pub anchor: usize,
    pub focus: usize,
}

impl TextSelectionRange {
    pub fn new(anchor: usize, focus: usize) -> Self {
        Self { anchor, focus }
    }

    pub fn start(&self) -> usize {
        self.anchor.min(self.focus)
    }

    pub fn end(&self) -> usize {
        self.anchor.max(self.focus)
    }

    pub fn len(&self) -> usize {
        self.end() - self.start()
    }

    pub fn is_empty(&self) -> bool {
        self.anchor == self.focus
    }
}

/// Authentic GPUI View for HistoryView::KeyboardTextSelection.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HistoryViewKeyboardTextSelectionView {
    pub current_range: TextSelectionRange,
    pub has_selection: bool,
}

impl HistoryViewKeyboardTextSelectionView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extend(
        &mut self,
        key: SelectionKey,
        by_word: bool,
        text: &str,
        max_offset: usize,
    ) -> Option<TextSelectionRange> {
        if !key.is_extend_key() {
            return None;
        }

        let mut focus = self.current_range.focus;
        match key {
            SelectionKey::Home => focus = 0,
            SelectionKey::End => focus = max_offset,
            SelectionKey::Right => {
                if by_word {
                    // Skip separators, then skip non-separators
                    let chars: Vec<char> = text.chars().collect();
                    while focus < chars.len() && chars[focus].is_whitespace() {
                        focus += 1;
                    }
                    while focus < chars.len() && !chars[focus].is_whitespace() {
                        focus += 1;
                    }
                } else {
                    focus = (focus + 1).min(max_offset);
                }
            }
            SelectionKey::Left => {
                if by_word {
                    let chars: Vec<char> = text.chars().collect();
                    if focus > 0 {
                        focus -= 1;
                    }
                    while focus > 0 && chars[focus].is_whitespace() {
                        focus -= 1;
                    }
                    while focus > 0 && !chars[focus - 1].is_whitespace() {
                        focus -= 1;
                    }
                } else {
                    focus = focus.saturating_sub(1);
                }
            }
            SelectionKey::Other => return None,
        }

        self.current_range.focus = focus.min(max_offset);
        self.has_selection = true;
        Some(self.current_range)
    }
}

impl Render for HistoryViewKeyboardTextSelectionView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
