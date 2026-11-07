//! Authentic GPUI Multi-Select Token Field with Animated Delete Badges.
//! Reference: Telegram/SourceFiles/ui/widgets/multi_select.cpp:L25-L80
//! Reference: Telegram/SourceFiles/ui/widgets/multi_select.h:L25-L77

#![allow(dead_code, unused_imports, unused_variables)]

/// Wide scaling multiplier for multi-select tag layout.
/// Reference: `multi_select.cpp:L25`
pub const K_WIDE_SCALE: i32 = 3;

#[derive(Debug, Clone, PartialEq)]
pub struct SelectedEntityItem {
    pub id: u64,
    pub title: String,
    pub is_active: bool,
    pub is_over_delete: bool,
    pub is_hiding: bool,
    pub opacity: f32,
}

/// Target symbol for tokenized multi-peer or multi-tag selection field with search input.
#[derive(Debug, Clone)]
pub struct MultiSelectWidgets {
    pub items: Vec<SelectedEntityItem>,
    pub query_text: String,
    pub placeholder: String,
    pub is_focused: bool,
    pub max_visible_items: usize,
}

impl Default for MultiSelectWidgets {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiSelectWidgets {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            query_text: String::new(),
            placeholder: "Search".to_string(),
            is_focused: false,
            max_visible_items: 20,
        }
    }

    /// Adds a selected entity pill item.
    /// Reference: `multi_select.cpp:L27-L35`
    pub fn add_item(&mut self, id: u64, title: impl Into<String>) {
        if !self.items.iter().any(|it| it.id == id) {
            self.items.push(SelectedEntityItem {
                id,
                title: title.into(),
                is_active: false,
                is_over_delete: false,
                is_hiding: false,
                opacity: 1.0,
            });
            self.query_text.clear();
        }
    }

    /// Removes an item by entity ID.
    /// Reference: `multi_select.cpp:L76-L80`
    pub fn remove_item(&mut self, id: u64) -> bool {
        let initial_len = self.items.len();
        self.items.retain(|it| it.id != id);
        self.items.len() < initial_len
    }

    /// Handles Backspace key in search input field: if query is empty, remove the last item.
    pub fn on_backspace(&mut self) -> Option<u64> {
        if self.query_text.is_empty() && !self.items.is_empty() {
            let removed = self.items.pop()?;
            Some(removed.id)
        } else {
            None
        }
    }

    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query_text = query.into();
    }

    pub fn item_ids(&self) -> Vec<u64> {
        self.items.iter().map(|it| it.id).collect()
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.query_text.clear();
    }
}
