//! Info Content Scroll Container
//! Upstream: Telegram/SourceFiles/info/info_content_widget.h, Telegram/SourceFiles/info/info_content_widget.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct InfoContentWidget {
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub scroll_y: f32,
    pub content_height: f32,
    pub flexible_filler_height: f32,
    pub is_swiping_back: bool,
    pub swipe_offset_x: f32,
    pub swipe_back_threshold_ratio: f32,
}

impl Default for InfoContentWidget {
    fn default() -> Self {
        Self {
            viewport_width: 320.0,
            viewport_height: 600.0,
            scroll_y: 0.0,
            content_height: 400.0,
            flexible_filler_height: 200.0,
            is_swiping_back: false,
            swipe_offset_x: 0.0,
            swipe_back_threshold_ratio: 0.35,
        }
    }
}

impl InfoContentWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_geometry(&mut self, width: f32, height: f32) {
        self.viewport_width = width.max(260.0);
        self.viewport_height = height.max(100.0);
        self.recompute_filler();
    }

    pub fn set_content_height(&mut self, height: f32) {
        self.content_height = height.max(0.0);
        self.recompute_filler();
    }

    pub fn set_scroll_y(&mut self, y: f32) {
        self.scroll_y = y.max(0.0);
    }

    pub fn recompute_filler(&mut self) {
        let remaining = self.viewport_height - self.content_height;
        self.flexible_filler_height = remaining.max(0.0);
    }

    pub fn start_swipe_back(&mut self) {
        self.is_swiping_back = true;
        self.swipe_offset_x = 0.0;
    }

    pub fn update_swipe(&mut self, delta_x: f32) {
        if self.is_swiping_back {
            self.swipe_offset_x = delta_x.max(0.0);
        }
    }

    pub fn finish_swipe(&mut self) -> bool {
        let should_navigate = self.is_swiping_back &&
            (self.swipe_offset_x >= self.viewport_width * self.swipe_back_threshold_ratio);
        self.is_swiping_back = false;
        self.swipe_offset_x = 0.0;
        should_navigate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_widget_filler_and_swipe() {
        let mut widget = InfoContentWidget::new();
        widget.set_geometry(320.0, 600.0);
        widget.set_content_height(450.0);
        assert_eq!(widget.flexible_filler_height, 150.0);

        widget.start_swipe_back();
        widget.update_swipe(50.0);
        assert!(!widget.finish_swipe());

        widget.start_swipe_back();
        widget.update_swipe(150.0); // 150 > 320 * 0.35 = 112
        assert!(widget.finish_swipe());
    }
}
