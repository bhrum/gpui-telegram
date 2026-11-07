//! Info Drawer Top Bar Header
//! Upstream: Telegram/SourceFiles/info/info_top_bar.h, Telegram/SourceFiles/info/info_top_bar.cpp

#[derive(Debug, Clone)]
pub struct InfoTopBar {
    pub title: String,
    pub subtitle: String,
    pub is_search_active: bool,
    pub is_selection_mode: bool,
    pub selected_count: usize,
    pub can_go_back: bool,
    pub height: f32,
    pub has_shadow: bool,
}

impl Default for InfoTopBar {
    fn default() -> Self {
        Self {
            title: "Info".into(),
            subtitle: String::new(),
            is_search_active: false,
            is_selection_mode: false,
            selected_count: 0,
            can_go_back: false,
            height: 54.0,
            has_shadow: false,
        }
    }
}

impl InfoTopBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_subtitle(&mut self, subtitle: impl Into<String>) {
        self.subtitle = subtitle.into();
    }

    pub fn set_can_go_back(&mut self, can_go_back: bool) {
        self.can_go_back = can_go_back;
    }

    pub fn toggle_search(&mut self) {
        self.is_search_active = !self.is_search_active;
    }

    pub fn enter_selection_mode(&mut self, initial_count: usize) {
        self.is_selection_mode = true;
        self.selected_count = initial_count;
    }

    pub fn exit_selection_mode(&mut self) {
        self.is_selection_mode = false;
        self.selected_count = 0;
    }

    pub fn set_selected_count(&mut self, count: usize) {
        self.selected_count = count;
        if count == 0 {
            self.is_selection_mode = false;
        } else {
            self.is_selection_mode = true;
        }
    }

    pub fn generate_selected_text(&self) -> String {
        if self.selected_count == 1 {
            "1 item selected".into()
        } else {
            format!("{} items selected", self.selected_count)
        }
    }

    pub fn set_shadow(&mut self, has_shadow: bool) {
        self.has_shadow = has_shadow;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_top_bar_selection() {
        let mut top_bar = InfoTopBar::new();
        top_bar.set_title("Shared Media");
        top_bar.set_can_go_back(true);

        assert_eq!(top_bar.title, "Shared Media");
        assert!(top_bar.can_go_back);
        assert!(!top_bar.is_selection_mode);

        top_bar.enter_selection_mode(3);
        assert!(top_bar.is_selection_mode);
        assert_eq!(top_bar.generate_selected_text(), "3 items selected");

        top_bar.set_selected_count(1);
        assert_eq!(top_bar.generate_selected_text(), "1 item selected");

        top_bar.exit_selection_mode();
        assert!(!top_bar.is_selection_mode);
        assert_eq!(top_bar.selected_count, 0);
    }
}
