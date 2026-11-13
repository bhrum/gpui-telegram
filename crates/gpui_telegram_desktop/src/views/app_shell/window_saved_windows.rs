//! WindowSavedWindows: Persistent multi-window layout, coordinates, and restoration state manager
//! Upstream: Telegram/SourceFiles/window/window_saved_windows.h, window_saved_windows.cpp

use gpui::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedWindowRecord {
    pub window_key: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_maximized: bool,
    pub last_active_ts: i64,
}

#[derive(Debug, Clone)]
pub struct WindowSavedWindows {
    pub records: HashMap<String, SavedWindowRecord>,
    pub restoration_in_progress: bool,
}

impl WindowSavedWindows {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
            restoration_in_progress: false,
        }
    }

    pub fn save_window(&mut self, record: SavedWindowRecord) {
        self.records.insert(record.window_key.clone(), record);
    }

    pub fn remove_window(&mut self, window_key: &str) -> Option<SavedWindowRecord> {
        self.records.remove(window_key)
    }

    pub fn get_window(&self, window_key: &str) -> Option<&SavedWindowRecord> {
        self.records.get(window_key)
    }

    pub fn maybe_begin_restore(&mut self) -> bool {
        if !self.records.is_empty() && !self.restoration_in_progress {
            self.restoration_in_progress = true;
            true
        } else {
            false
        }
    }

    pub fn finish_restore(&mut self) {
        self.restoration_in_progress = false;
    }

    pub fn count(&self) -> usize {
        self.records.len()
    }
}

impl Default for WindowSavedWindows {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowSavedWindows {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p(px(8.0))
            .child(format!("Saved Windows Count: {}", self.records.len()))
    }
}
