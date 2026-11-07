//! Navigation controller for Instant View
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use super::iv_history_stack::{HistoryEntry, IvHistoryStack};

#[derive(Debug, Clone, Default)]
pub struct IvNavigationController {
    pub history: IvHistoryStack,
    pub active_url: String,
    pub is_open: bool,
}

impl IvNavigationController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open_article(&mut self, url: impl Into<String>, title: impl Into<String>) {
        let u = url.into();
        self.active_url = u.clone();
        self.history.push(u, title);
        self.is_open = true;
    }

    pub fn navigate_back(&mut self) -> Option<String> {
        if let Some(entry) = self.history.go_back() {
            self.active_url = entry.url.clone();
            Some(self.active_url.clone())
        } else {
            None
        }
    }

    pub fn navigate_forward(&mut self) -> Option<String> {
        if let Some(entry) = self.history.go_forward() {
            self.active_url = entry.url.clone();
            Some(self.active_url.clone())
        } else {
            None
        }
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }
}
