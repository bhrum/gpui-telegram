//! Profile Information Card Block Widget
//! Upstream: Telegram/SourceFiles/profile/profile_block_widget.h, Telegram/SourceFiles/profile/profile_block_widget.cpp

#[derive(Debug, Clone)]
pub struct ProfileBlockWidget {
    pub section_title: String,
    pub corner_radius: f32,
    pub padding_h: f32,
    pub padding_v: f32,
    pub is_expanded: bool,
    pub has_separator: bool,
}

impl Default for ProfileBlockWidget {
    fn default() -> Self {
        Self {
            section_title: String::new(),
            corner_radius: 8.0,
            padding_h: 16.0,
            padding_v: 12.0,
            is_expanded: true,
            has_separator: true,
        }
    }
}

impl ProfileBlockWidget {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            section_title: title.into(),
            corner_radius: 8.0,
            padding_h: 16.0,
            padding_v: 12.0,
            is_expanded: true,
            has_separator: true,
        }
    }

    pub fn set_expanded(&mut self, expanded: bool) {
        self.is_expanded = expanded;
    }

    pub fn toggle_expanded(&mut self) -> bool {
        self.is_expanded = !self.is_expanded;
        self.is_expanded
    }

    pub fn set_padding(&mut self, padding_h: f32, padding_v: f32) {
        self.padding_h = padding_h.max(0.0);
        self.padding_v = padding_v.max(0.0);
    }

    pub fn set_has_separator(&mut self, has_separator: bool) {
        self.has_separator = has_separator;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_block_widget_expansion() {
        let mut block = ProfileBlockWidget::new("User Bio");
        assert_eq!(block.section_title, "User Bio");
        assert!(block.is_expanded);

        assert!(!block.toggle_expanded());
        assert!(!block.is_expanded);
        assert!(block.toggle_expanded());
        assert!(block.is_expanded);
    }
}
