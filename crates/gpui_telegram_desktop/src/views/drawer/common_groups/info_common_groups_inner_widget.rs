//! Common Mutual Groups List Inner Viewport
//! Upstream: `Telegram/SourceFiles/info/common_groups/info_common_groups_inner_widget.h`
//! Upstream: `Telegram/SourceFiles/info/common_groups/info_common_groups_inner_widget.cpp`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommonGroupRow {
    pub peer_id: u64,
    pub title: String,
    pub username: String,
    pub member_count: u32,
    pub is_selected: bool,
}

#[derive(Debug, Clone)]
pub struct InfoCommonGroupsInnerWidgetCommonGroups {
    pub user_id: u64,
    pub groups: Vec<CommonGroupRow>,
    pub search_query: String,
    pub row_height: f32,
    pub is_loading: bool,
    pub preload_group_id: u64,
    pub all_loaded: bool,
}

impl Default for InfoCommonGroupsInnerWidgetCommonGroups {
    fn default() -> Self {
        Self {
            user_id: 0,
            groups: Vec::new(),
            search_query: String::new(),
            row_height: 52.0,
            is_loading: false,
            preload_group_id: 0,
            all_loaded: false,
        }
    }
}

impl InfoCommonGroupsInnerWidgetCommonGroups {
    /// Upstream `kCommonGroupsPerPage = 40` (`info_common_groups_inner_widget.cpp:L27`).
    pub const COMMON_GROUPS_PER_PAGE: usize = 40;
    /// Upstream `kCommonGroupsSearchAfter = 20` (`info_common_groups_inner_widget.cpp:L28`).
    pub const COMMON_GROUPS_SEARCH_AFTER: usize = 20;

    pub fn new(user_id: u64) -> Self {
        Self {
            user_id,
            groups: Vec::new(),
            search_query: String::new(),
            row_height: 52.0,
            is_loading: false,
            preload_group_id: 0,
            all_loaded: false,
        }
    }

    pub fn add_group(&mut self, row: CommonGroupRow) {
        if !self.groups.iter().any(|g| g.peer_id == row.peer_id) {
            self.preload_group_id = row.peer_id;
            self.groups.push(row);
        }
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
    }

    pub fn filtered_groups(&self) -> Vec<&CommonGroupRow> {
        if self.search_query.trim().is_empty() {
            self.groups.iter().collect()
        } else {
            let q = self.search_query.trim().to_lowercase();
            self.groups
                .iter()
                .filter(|g| g.title.to_lowercase().contains(&q) || g.username.to_lowercase().contains(&q))
                .collect()
        }
    }

    pub fn total_height(&self) -> f32 {
        self.filtered_groups().len() as f32 * self.row_height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_groups_inner_widget_filter() {
        let mut inner = InfoCommonGroupsInnerWidgetCommonGroups::new(1001);
        inner.add_group(CommonGroupRow {
            peer_id: 2001,
            title: "Rust Language".into(),
            username: "rustlang".into(),
            member_count: 50000,
            is_selected: false,
        });
        inner.add_group(CommonGroupRow {
            peer_id: 2002,
            title: "Telegram Geeks".into(),
            username: "tggeeks".into(),
            member_count: 12000,
            is_selected: false,
        });

        assert_eq!(inner.filtered_groups().len(), 2);
        inner.set_search_query("rust");
        assert_eq!(inner.filtered_groups().len(), 1);
        assert_eq!(inner.filtered_groups()[0].title, "Rust Language");
        assert_eq!(InfoCommonGroupsInnerWidgetCommonGroups::COMMON_GROUPS_PER_PAGE, 40);
        assert_eq!(InfoCommonGroupsInnerWidgetCommonGroups::COMMON_GROUPS_SEARCH_AFTER, 20);
    }
}
