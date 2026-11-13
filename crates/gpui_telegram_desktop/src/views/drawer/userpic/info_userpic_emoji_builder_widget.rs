//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_widget.h
//! Upstream: Telegram/SourceFiles/info/userpic/info_userpic_emoji_builder_widget.cpp
//! Subsystem: info | Feature ID: TCG-N34 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use super::info_userpic_bubble_wrap::InfoUserpicBubbleWrapUserpic;
use super::info_userpic_color_circle_button::InfoUserpicColorCircleButtonUserpic;
use super::info_userpic_colors_editor::InfoUserpicColorsEditorUserpic;
use super::info_userpic_emoji_builder_common::{EmojiBuilderResult, EmojiBuilderStartData};
use super::info_userpic_emoji_builder_preview::InfoUserpicEmojiBuilderPreviewUserpic;

pub const K_PRESET_COLORS_COUNT: usize = 7;
pub const K_TOTAL_PALETTE_BUTTONS: usize = 8; // 7 presets + 1 custom/special

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectorTab {
    Emoji,
    Stickers,
}

#[derive(Debug, Clone)]
pub struct InfoUserpicEmojiBuilderWidgetUserpic {
    pub preview: InfoUserpicEmojiBuilderPreviewUserpic,
    pub palette_presets: Vec<Vec<u32>>,
    pub selected_palette_index: usize,
    pub custom_editor: InfoUserpicColorsEditorUserpic,
    pub active_tab: SelectorTab,
    pub recent_documents: Vec<u64>,
    pub is_forum: bool,
}

impl InfoUserpicEmojiBuilderWidgetUserpic {
    pub fn new(start_data: EmojiBuilderStartData) -> Self {
        let palette = Self::build_palette_gradients();
        let selected_index = start_data.builder_color_index % K_PRESET_COLORS_COUNT;
        let preview_colors = palette[selected_index].clone();

        let mut preview = InfoUserpicEmojiBuilderPreviewUserpic::new(100.0, start_data.is_forum);
        preview.set_gradient_colors(preview_colors);
        if start_data.document_id > 0 {
            preview.set_document_id(start_data.document_id);
        }

        Self {
            preview,
            palette_presets: palette,
            selected_palette_index: selected_index,
            custom_editor: InfoUserpicColorsEditorUserpic::new(start_data.gradient_editor_colors),
            active_tab: SelectorTab::Emoji,
            recent_documents: start_data.documents,
            is_forum: start_data.is_forum,
        }
    }

    /// Upstream logic: `PaletteGradients` with 180-degree diagonal rotation
    pub fn build_palette_gradients() -> Vec<Vec<u32>> {
        let mut raw = vec![
            vec![0x20e2cdff, 0x0ee1f1ff, 0x4d8dffff, 0x2bbfffff],
            vec![0x45f7b7ff, 0x1ff1d9ff, 0x5eb6fbff, 0x1fceebff],
            vec![0xc1e526ff, 0x80df2bff, 0x09d260ff, 0x5edc40ff],
            vec![0xffd412ff, 0xffa743ff, 0xf5694eff, 0xf5772cff],
            vec![0xf6a730ff, 0xff7742ff, 0xf64884ff, 0xef5b41ff],
            vec![0xffb23aff, 0xfe7e62ff, 0xf94ba0ff, 0xfb5c80ff],
            vec![0xff72a9ff, 0xe269ffff, 0x837cffff, 0xb063ffff],
        ];

        // Upstream: rotate 180 degrees via std::swap(g[0], g[2]) and std::swap(g[1], g[3])
        for g in &mut raw {
            g.swap(0, 2);
            g.swap(1, 3);
        }
        raw
    }

    /// Upstream layout formula: `AlignChildren`
    pub fn compute_child_step(full_width: f32, widget_width: f32, count: usize) -> f32 {
        if count <= 1 {
            return 0.0;
        }
        let total_widgets_width = widget_width * (count as f32);
        (full_width - total_widgets_width) / ((count - 1) as f32)
    }

    pub fn select_preset(&mut self, index: usize) {
        if index < K_PRESET_COLORS_COUNT {
            self.selected_palette_index = index;
            let colors = self.palette_presets[index].clone();
            self.preview.set_gradient_colors(colors);
        } else if index == K_PRESET_COLORS_COUNT {
            // Special custom gradient button
            self.selected_palette_index = index;
            let colors = self.custom_editor.colors.clone();
            self.preview.set_gradient_colors(colors);
        }
    }

    pub fn select_document(&mut self, doc_id: u64) {
        self.preview.set_document_id(doc_id);
    }

    pub fn toggle_tab(&mut self) {
        self.active_tab = match self.active_tab {
            SelectorTab::Emoji => SelectorTab::Stickers,
            SelectorTab::Stickers => SelectorTab::Emoji,
        };
    }

    pub fn emit_result(&self) -> EmojiBuilderResult {
        EmojiBuilderResult {
            document_id: self.preview.document_id.unwrap_or(0),
            colors: self.preview.colors.clone(),
            is_forum: self.is_forum,
        }
    }
}

impl Default for InfoUserpicEmojiBuilderWidgetUserpic {
    fn default() -> Self {
        Self::new(EmojiBuilderStartData::default())
    }
}

impl Render for InfoUserpicEmojiBuilderWidgetUserpic {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.selected_palette_index;
        let tab = self.active_tab;

        let preview_color = *self.preview.colors.first().unwrap_or(&0x20e2cdff);
        let radius = if self.is_forum { px(28.0) } else { px(50.0) };

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap(px(12.0))
            .child(
                div()
                    .flex()
                    .justify_center()
                    .child(
                        div()
                            .w(px(100.0))
                            .h(px(100.0))
                            .rounded(radius)
                            .bg(rgba(preview_color))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(div().text_xl().child(if self.preview.document_id.is_some() { "🎭" } else { "😀" })),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_center()
                    .gap(px(6.0))
                    .children((0..K_TOTAL_PALETTE_BUTTONS).map(|idx| {
                        let is_special = idx == K_PRESET_COLORS_COUNT;
                        let color = if is_special {
                            *self.custom_editor.colors.first().unwrap_or(&0xffffff)
                        } else {
                            self.palette_presets[idx][0]
                        };
                        div()
                            .w(px(30.0))
                            .h(px(30.0))
                            .rounded(px(15.0))
                            .bg(rgba(color))
                            .cursor_pointer()
                            .border(px(if idx == selected { 3.0 } else { 0.0 }))
                            .border_color(rgb(0xffffff))
                    })),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_around()
                    .p(px(8.0))
                    .bg(rgb(0x17212b))
                    .rounded(px(6.0))
                    .child(
                        div()
                            .text_xs()
                            .cursor_pointer()
                            .text_color(if tab == SelectorTab::Emoji { rgb(0x5288c1) } else { rgb(0x707579) })
                            .child("Emojis"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .cursor_pointer()
                            .text_color(if tab == SelectorTab::Stickers { rgb(0x5288c1) } else { rgb(0x707579) })
                            .child("Stickers"),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_presets_swapping() {
        let presets = InfoUserpicEmojiBuilderWidgetUserpic::build_palette_gradients();
        assert_eq!(presets.len(), K_PRESET_COLORS_COUNT);

        // First preset originally had: [0x20e2cdff, 0x0ee1f1ff, 0x4d8dffff, 0x2bbfffff]
        // After swap(0, 2) and swap(1, 3):
        // index 0 is 0x4d8dffff, index 1 is 0x2bbfffff, index 2 is 0x20e2cdff, index 3 is 0x0ee1f1ff
        assert_eq!(presets[0][0], 0x4d8dffff);
        assert_eq!(presets[0][1], 0x2bbfffff);
        assert_eq!(presets[0][2], 0x20e2cdff);
        assert_eq!(presets[0][3], 0x0ee1f1ff);
    }

    #[test]
    fn test_align_children_step() {
        let step = InfoUserpicEmojiBuilderWidgetUserpic::compute_child_step(356.0, 30.0, 8);
        // (356 - (30 * 8)) / (8 - 1) = (356 - 240) / 7 = 116 / 7 ≈ 16.57
        assert!((step - (116.0 / 7.0)).abs() < 1e-4);
    }

    #[test]
    fn test_builder_widget_selection_and_result() {
        let mut widget = InfoUserpicEmojiBuilderWidgetUserpic::default();
        widget.select_preset(2);
        assert_eq!(widget.selected_palette_index, 2);

        widget.select_document(8888);
        assert_eq!(widget.preview.document_id, Some(8888));

        let res = widget.emit_result();
        assert_eq!(res.document_id, 8888);
        assert_eq!(res.colors, widget.palette_presets[2]);
    }
}
