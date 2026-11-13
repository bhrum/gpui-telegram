//! History navigation stack for Instant View
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

#[derive(Debug, Clone, Default)]
pub struct HistoryEntry {
    pub url: String,
    pub title: String,
    pub scroll_offset_y: f32,
}

#[derive(Debug, Clone, Default)]
pub struct IvHistoryStack {
    pub back_stack: Vec<HistoryEntry>,
    pub forward_stack: Vec<HistoryEntry>,
    pub current: Option<HistoryEntry>,
}

impl IvHistoryStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, url: impl Into<String>, title: impl Into<String>) {
        if let Some(cur) = self.current.take() {
            self.back_stack.push(cur);
        }
        self.forward_stack.clear();
        self.current = Some(HistoryEntry {
            url: url.into(),
            title: title.into(),
            scroll_offset_y: 0.0,
        });
    }

    pub fn can_go_back(&self) -> bool {
        !self.back_stack.is_empty()
    }

    pub fn can_go_forward(&self) -> bool {
        !self.forward_stack.is_empty()
    }

    pub fn go_back(&mut self) -> Option<&HistoryEntry> {
        if let Some(prev) = self.back_stack.pop() {
            if let Some(cur) = self.current.take() {
                self.forward_stack.push(cur);
            }
            self.current = Some(prev);
            self.current.as_ref()
        } else {
            None
        }
    }

    pub fn go_forward(&mut self) -> Option<&HistoryEntry> {
        if let Some(next) = self.forward_stack.pop() {
            if let Some(cur) = self.current.take() {
                self.back_stack.push(cur);
            }
            self.current = Some(next);
            self.current.as_ref()
        } else {
            None
        }
    }
}
