//! Info Section Widget Base
//! Upstream: Telegram/SourceFiles/info/info_section_widget.h, Telegram/SourceFiles/info/info_section_widget.cpp

use gpui::*;
use super::info_controller::InfoSectionType;

#[derive(Debug, Clone)]
pub struct InfoSectionWidget {
    pub section_type: InfoSectionType,
    pub has_top_bar_shadow: bool,
    pub is_active: bool,
    pub content_height: f32,
    pub viewport_width: f32,
}

impl Default for InfoSectionWidget {
    fn default() -> Self {
        Self {
            section_type: InfoSectionType::Profile,
            has_top_bar_shadow: false,
            is_active: true,
            content_height: 600.0,
            viewport_width: 320.0,
        }
    }
}

impl InfoSectionWidget {
    pub fn new(section_type: InfoSectionType) -> Self {
        Self {
            section_type,
            has_top_bar_shadow: false,
            is_active: true,
            content_height: 600.0,
            viewport_width: 320.0,
        }
    }

    pub fn set_has_top_bar_shadow(&mut self, has_shadow: bool) {
        self.has_top_bar_shadow = has_shadow;
    }

    pub fn desired_shadow_visibility(&self, scroll_y: f32) -> bool {
        scroll_y > 4.0
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn update_geometry(&mut self, width: f32, height: f32) {
        self.viewport_width = width.max(260.0);
        self.content_height = height.max(0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_section_widget_geometry() {
        let mut widget = InfoSectionWidget::new(InfoSectionType::CommonGroups);
        assert_eq!(widget.section_type, InfoSectionType::CommonGroups);
        assert!(!widget.desired_shadow_visibility(2.0));
        assert!(widget.desired_shadow_visibility(5.0));

        widget.update_geometry(380.0, 720.0);
        assert_eq!(widget.viewport_width, 380.0);
        assert_eq!(widget.content_height, 720.0);
    }
}
