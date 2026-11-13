//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_colors_editor.h
//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_colors_editor.cpp
//! Subsystem: info | Feature ID: TCG-N34 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use super::info_userpic_color_circle_button::InfoUserpicColorCircleButtonUserpic;

pub const K_MAX_COLORS: usize = 4;
pub const K_MIN_COLORS: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButtonLayoutPosition {
    pub shown: bool,
    pub left: f32,
}

#[derive(Debug, Clone)]
pub struct InfoUserpicColorsEditorUserpic {
    pub colors: Vec<u32>, // RGBA colors
    pub chosen_index: usize,
    pub container_width: f32,
    pub button_size: f32,
}

impl InfoUserpicColorsEditorUserpic {
    pub fn new(initial_colors: Vec<u32>) -> Self {
        let colors = if initial_colors.is_empty() {
            vec![0x20e2cdff, 0x4d8dffff]
        } else {
            initial_colors.into_iter().take(K_MAX_COLORS).collect()
        };
        let chosen_index = colors.len().saturating_sub(1);
        Self {
            colors,
            chosen_index,
            container_width: 356.0,
            button_size: 30.0,
        }
    }

    pub fn can_add_color(&self) -> bool {
        self.colors.len() < K_MAX_COLORS
    }

    pub fn can_remove_color(&self) -> bool {
        self.colors.len() > K_MIN_COLORS
    }

    /// Upstream logic: `RandomColor`
    pub fn generate_pseudo_random_color(base_color: u32, seed: u8) -> u32 {
        let r = ((base_color >> 24) & 0xff) as u8;
        let g = ((base_color >> 16) & 0xff) as u8;
        let b = ((base_color >> 8) & 0xff) as u8;

        // Shift hue and modify saturation deterministically based on seed
        let new_r = r.wrapping_add(seed.wrapping_mul(37));
        let new_g = g.wrapping_add(seed.wrapping_mul(59));
        let new_b = b.wrapping_add(seed.wrapping_mul(83));
        ((new_r as u32) << 24) | ((new_g as u32) << 16) | ((new_b as u32) << 8) | 0xff
    }

    pub fn add_color(&mut self) -> bool {
        if !self.can_add_color() {
            return false;
        }
        let last = *self.colors.last().unwrap_or(&0x20e2cdff);
        let next_color = Self::generate_pseudo_random_color(last, (self.colors.len() as u8) + 1);
        self.colors.push(next_color);
        self.chosen_index = self.colors.len() - 1;
        true
    }

    pub fn remove_color(&mut self) -> bool {
        if !self.can_remove_color() {
            return false;
        }
        self.colors.pop();
        if self.chosen_index >= self.colors.len() {
            self.chosen_index = self.colors.len() - 1;
        }
        true
    }

    pub fn set_chosen_color(&mut self, color: u32) {
        if self.chosen_index < self.colors.len() {
            self.colors[self.chosen_index] = color;
        }
    }

    /// Upstream layout math: `calculatePositionFor`
    pub fn calculate_positions(&self) -> Vec<ButtonLayoutPosition> {
        let count = self.colors.len();
        let full_width = self.container_width;
        let width = self.button_size;
        let colors_width = (width * (count as f32)) + (width * ((count.saturating_sub(1)) as f32));
        let left = (full_width - colors_width) / 2.0;

        let mut result = Vec::with_capacity(count + 2);
        // Minus button position (index 0)
        result.push(ButtonLayoutPosition {
            shown: count > 1,
            left: left - (width * 2.0),
        });

        // Color buttons (indices 1..=count)
        for i in 0..K_MAX_COLORS {
            result.push(ButtonLayoutPosition {
                shown: i < count,
                left: left + ((i as f32) * width * 2.0),
            });
        }

        // Plus button position (last index)
        result.push(ButtonLayoutPosition {
            shown: count < K_MAX_COLORS,
            left: left + colors_width + width,
        });

        result
    }
}

impl Default for InfoUserpicColorsEditorUserpic {
    fn default() -> Self {
        Self::new(vec![0x20e2cdff, 0x4d8dffff])
    }
}

impl Render for InfoUserpicColorsEditorUserpic {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let chosen = self.chosen_index;

        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap(px(8.0));

        if self.can_remove_color() {
            row = row.child(
                div()
                    .w(px(30.0))
                    .h(px(30.0))
                    .rounded(px(15.0))
                    .bg(rgb(0x281c1c))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .child(div().text_sm().text_color(rgb(0xec3942)).child("-")),
            );
        }

        row = row.children(self.colors.iter().enumerate().map(|(idx, &color)| {
            div()
                .w(px(30.0))
                .h(px(30.0))
                .rounded(px(15.0))
                .bg(rgba(color))
                .cursor_pointer()
                .border(px(if idx == chosen { 3.0 } else { 0.0 }))
                .border_color(rgb(0xffffff))
        }));

        if self.can_add_color() {
            row = row.child(
                div()
                    .w(px(30.0))
                    .h(px(30.0))
                    .rounded(px(15.0))
                    .bg(rgb(0x1c2b3d))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .child(div().text_sm().text_color(rgb(0x5288c1)).child("+")),
            );
        }

        div()
            .flex()
            .flex_col()
            .items_center()
            .w(px(self.container_width))
            .p(px(12.0))
            .gap(px(12.0))
            .child(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colors_editor_add_remove_limits() {
        let mut editor = InfoUserpicColorsEditorUserpic::new(vec![0x111111ff]);
        assert_eq!(editor.colors.len(), 1);
        assert!(!editor.can_remove_color());
        assert!(editor.can_add_color());

        assert!(editor.add_color());
        assert!(editor.add_color());
        assert!(editor.add_color());
        assert_eq!(editor.colors.len(), 4);
        assert!(!editor.can_add_color());
        assert!(!editor.add_color()); // Reject above 4

        assert!(editor.remove_color());
        assert_eq!(editor.colors.len(), 3);
    }

    #[test]
    fn test_colors_editor_positions_calculation() {
        let editor = InfoUserpicColorsEditorUserpic::new(vec![0x111111ff, 0x222222ff]);
        let positions = editor.calculate_positions();
        assert_eq!(positions.len(), 6); // minus + 4 colors + plus

        // Minus button shown because count = 2 > 1
        assert!(positions[0].shown);
        // First two colors shown
        assert!(positions[1].shown);
        assert!(positions[2].shown);
        // Third and fourth colors hidden
        assert!(!positions[3].shown);
        assert!(!positions[4].shown);
        // Plus button shown because count = 2 < 4
        assert!(positions[5].shown);
    }
}
