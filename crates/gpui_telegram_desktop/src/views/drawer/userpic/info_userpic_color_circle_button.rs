//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_color_circle_button.h
//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_color_circle_button.cpp
//! Subsystem: info | Feature ID: TCG-N34 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_DEFAULT_CIRCLE_SIZE: f32 = 30.0; // st::userpicBuilderEmojiAccentColorSize

#[derive(Debug, Clone)]
pub struct InfoUserpicColorCircleButtonUserpic {
    pub index: usize,
    pub color_rgba: u32,
    pub selected_progress: f32, // 0.0 to 1.0
    pub size: f32,
}

impl InfoUserpicColorCircleButtonUserpic {
    pub fn new(index: usize, color_rgba: u32) -> Self {
        Self {
            index,
            color_rgba,
            selected_progress: 0.0,
            size: K_DEFAULT_CIRCLE_SIZE,
        }
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn set_color(&mut self, color_rgba: u32) {
        self.color_rgba = color_rgba;
    }

    /// Upstream logic: `setSelectedProgress`
    pub fn set_selected_progress(&mut self, progress: f32) {
        self.selected_progress = progress.clamp(0.0, 1.0);
    }

    pub fn is_selected(&self) -> bool {
        self.selected_progress >= 0.5
    }
}

impl Default for InfoUserpicColorCircleButtonUserpic {
    fn default() -> Self {
        Self::new(0, 0x20e2cdff)
    }
}

impl Render for InfoUserpicColorCircleButtonUserpic {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let size = self.size;
        let progress = self.selected_progress;
        let ring_width = progress * 3.0;

        let mut el = div()
            .w(px(size))
            .h(px(size))
            .rounded(px(size / 2.0))
            .bg(rgba(self.color_rgba))
            .cursor_pointer();

        if progress > 0.0 {
            el = el.border_color(rgb(0xffffff)).border(px(ring_width));
        }

        el
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_circle_button_selection_progress() {
        let mut btn = InfoUserpicColorCircleButtonUserpic::new(3, 0xffa743ff);
        assert_eq!(btn.index, 3);
        assert!(!btn.is_selected());
        assert_eq!(btn.selected_progress, 0.0);

        btn.set_selected_progress(0.4);
        assert!(!btn.is_selected());

        btn.set_selected_progress(0.8);
        assert!(btn.is_selected());

        btn.set_selected_progress(1.5); // Clamped
        assert_eq!(btn.selected_progress, 1.0);
    }
}
