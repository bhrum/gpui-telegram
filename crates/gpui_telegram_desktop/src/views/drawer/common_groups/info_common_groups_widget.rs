//! Common Mutual Groups Section Container
//! Upstream: Telegram/SourceFiles/info/common_groups/info_common_groups_widget.h, Telegram/SourceFiles/info/common_groups/info_common_groups_widget.cpp

use super::info_common_groups_inner_widget::{CommonGroupRow, InfoCommonGroupsInnerWidgetCommonGroups};

#[derive(Debug, Clone)]
pub struct InfoCommonGroupsWidgetCommonGroups {
    pub inner: InfoCommonGroupsInnerWidgetCommonGroups,
    pub user_id: u64,
    pub section_title: String,
    pub is_search_field_focused: bool,
}

impl Default for InfoCommonGroupsWidgetCommonGroups {
    fn default() -> Self {
        Self {
            inner: InfoCommonGroupsInnerWidgetCommonGroups::default(),
            user_id: 0,
            section_title: "Common Groups".into(),
            is_search_field_focused: false,
        }
    }
}

impl InfoCommonGroupsWidgetCommonGroups {
    pub fn new(user_id: u64) -> Self {
        Self {
            inner: InfoCommonGroupsInnerWidgetCommonGroups::new(user_id),
            user_id,
            section_title: "Common Groups".into(),
            is_search_field_focused: false,
        }
    }

    pub fn set_search_focused(&mut self, focused: bool) {
        self.is_search_field_focused = focused;
    }

    pub fn search(&mut self, query: impl Into<String>) {
        self.inner.set_search_query(query);
    }

    pub fn title(&self) -> &str {
        &self.section_title
    }

    pub fn group_count(&self) -> usize {
        self.inner.groups.len()
    }

    pub fn add_group(&mut self, row: CommonGroupRow) {
        self.inner.add_group(row);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_groups_widget_container() {
        let mut widget = InfoCommonGroupsWidgetCommonGroups::new(555);
        assert_eq!(widget.user_id, 555);
        assert_eq!(widget.title(), "Common Groups");
        assert_eq!(widget.group_count(), 0);

        widget.add_group(CommonGroupRow {
            peer_id: 777,
            title: "Developers".into(),
            username: "devs".into(),
            member_count: 320,
            is_selected: false,
        });

        assert_eq!(widget.group_count(), 1);
        widget.set_search_focused(true);
        assert!(widget.is_search_field_focused);
    }
}
