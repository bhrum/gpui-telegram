//! Info Memento Snapshot
//! Upstream: Telegram/SourceFiles/info/info_memento.h, Telegram/SourceFiles/info/info_memento.cpp

use super::info_controller::InfoSectionType;

#[derive(Debug, Clone, PartialEq)]
pub struct InfoMemento {
    pub section_type: InfoSectionType,
    pub scroll_top: f32,
    pub search_query: String,
    pub selected_tab_index: usize,
    pub sub_section_id: u64,
}

impl Default for InfoMemento {
    fn default() -> Self {
        Self {
            section_type: InfoSectionType::Profile,
            scroll_top: 0.0,
            search_query: String::new(),
            selected_tab_index: 0,
            sub_section_id: 0,
        }
    }
}

impl InfoMemento {
    pub fn new(section_type: InfoSectionType) -> Self {
        Self {
            section_type,
            scroll_top: 0.0,
            search_query: String::new(),
            selected_tab_index: 0,
            sub_section_id: 0,
        }
    }

    pub fn with_scroll(mut self, scroll_top: f32) -> Self {
        self.scroll_top = scroll_top.max(0.0);
        self
    }

    pub fn with_tab(mut self, tab_index: usize) -> Self {
        self.selected_tab_index = tab_index;
        self
    }

    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.search_query = query.into();
        self
    }

    pub fn section(&self) -> InfoSectionType {
        self.section_type
    }

    pub fn reset_scroll(&mut self) {
        self.scroll_top = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_memento_creation() {
        let memento = InfoMemento::new(InfoSectionType::Media)
            .with_scroll(145.5)
            .with_tab(2)
            .with_query("photos");

        assert_eq!(memento.section(), InfoSectionType::Media);
        assert_eq!(memento.scroll_top, 145.5);
        assert_eq!(memento.selected_tab_index, 2);
        assert_eq!(memento.search_query, "photos");
    }
}
