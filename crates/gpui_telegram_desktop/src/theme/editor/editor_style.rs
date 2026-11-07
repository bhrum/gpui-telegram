//! Telegram Desktop 1:1 Parity: `EditorStyleSheet`
//! Subsystem: `editor` | Feature ID: `TCG-N06`
//! Upstream Source: `Telegram/SourceFiles/editor/editor.style:L1-L400`
#![allow(dead_code)]

use gpui::{px, rgb, Pixels, Rgba};

/// Authentic GPUI Telegram theme style sheet embodying all metric tokens and
/// pixel dimensions declared in `Telegram/SourceFiles/editor/editor.style`.
#[derive(Debug, Clone)]
pub struct EditorStyleSheet {
    // Canvas & Controls Bar (L16-L27)
    pub photo_editor_controls_height: Pixels,
    pub photo_editor_controls_bottom_skip: Pixels,
    pub photo_editor_controls_center_skip: Pixels,
    pub photo_editor_bar_animation_duration_ms: u32,
    pub photo_editor_button_bar_height: Pixels,
    pub photo_editor_button_bar_width: Pixels,

    // Colors (L29-L35)
    pub bg_color: Rgba,
    pub text_color: Rgba,
    pub button_icon_fg: Rgba,
    pub button_icon_fg_over: Rgba,
    pub button_icon_fg_active: Rgba,
    pub button_icon_fg_inactive: Rgba,
    pub edge_button_bg: Rgba,

    // Buttons & Shapes (L92-L147)
    pub text_button_size: Pixels,
    pub text_button_glyph_skip: Pixels,
    pub shape_fill_duration_ms: u32,
    pub shape_fill_radius: Pixels,
    pub align_icon_size: Pixels,

    // Color Palette & Tools (L166-L185)
    pub color_button_size: Pixels,
    pub color_button_border: Pixels,
    pub color_palette_item_size: Pixels,
    pub color_palette_gap: Pixels,
    pub color_palette_selection_width: Pixels,
    pub color_palette_plus_line: Pixels,
    pub tool_button_size: Pixels,
    pub tool_button_gap: Pixels,
    pub tool_button_hover_duration_ms: u32,
    pub tool_button_select_duration_ms: u32,
    pub tool_button_selected_opacity: f32,

    // Brush Engine Constants (L186-L204)
    pub marker_opacity: f32,
    pub marker_size_multiplier: f32,
    pub arrow_head_length_factor: f32,
    pub arrow_head_min_distance_factor: f32,
    pub arrow_head_angle_degrees: f32,
    pub eraser_preview_opacity: f32,
    pub blur_size_multiplier: f32,
    pub blur_preview_opacity: f32,
    pub blur_radius: f32,
    pub brush_size_control_height: Pixels,
    pub brush_size_control_collapsed_width: Pixels,
    pub brush_size_control_expanded_top_width: Pixels,
    pub brush_size_control_expanded_bottom_width: Pixels,
    pub brush_size_control_hit_padding: Pixels,

    // Crop Geometry (L205-L211)
    pub crop_point_size: Pixels,
    pub crop_min_size: Pixels,
    pub item_handle_size: Pixels,
    pub sticky_trigger: Pixels,
    pub sticky_line_width: Pixels,

    // Audio & Timeline (L250-L285)
    pub audio_disc_size: Pixels,
    pub audio_disc_skip: Pixels,
    pub audio_disc_cover_size: Pixels,
    pub audio_disc_ring: Pixels,
    pub audio_timeline_bar_width: Pixels,
    pub audio_timeline_bar_skip: Pixels,
    pub audio_timeline_bar_ratio: f32,
    pub audio_timeline_bar_opacity: f32,

    // Video Timeline (L287-L360)
    pub video_timeline_strip_height: Pixels,
    pub video_timeline_label_height: Pixels,
    pub video_timeline_trim_strip_height: Pixels,
    pub video_timeline_dot_size: Pixels,
    pub video_timeline_dot_active_size: Pixels,
    pub video_timeline_radius: Pixels,
    pub video_timeline_handle_width: Pixels,
    pub video_quality_slider_dot_size: Pixels,
    pub video_quality_slider_dot_active_size: Pixels,
    pub video_quality_slider_height: Pixels,

    // Link Preview & Pill (L385-L399)
    pub message_reference_width: Pixels,
    pub link_preview_max_height: Pixels,
    pub link_preview_padding: Pixels,
    pub link_theme_size: Pixels,
}

impl Default for EditorStyleSheet {
    fn default() -> Self {
        Self {
            photo_editor_controls_height: px(146.0),
            photo_editor_controls_bottom_skip: px(20.0),
            photo_editor_controls_center_skip: px(6.0),
            photo_editor_bar_animation_duration_ms: 100,
            photo_editor_button_bar_height: px(48.0),
            photo_editor_button_bar_width: px(422.0),

            bg_color: rgb(0x000000),
            text_color: rgb(0xFFFFFF),
            button_icon_fg: rgb(0xFFFFFF),
            button_icon_fg_over: rgb(0xCCCCCC),
            button_icon_fg_active: rgb(0x2AABEE),
            button_icon_fg_inactive: rgb(0x777777),
            edge_button_bg: rgb(0x202020),

            text_button_size: px(42.0),
            text_button_glyph_skip: px(3.0),
            shape_fill_duration_ms: 160,
            shape_fill_radius: px(13.0),
            align_icon_size: px(24.0),

            color_button_size: px(24.0),
            color_button_border: px(1.0),
            color_palette_item_size: px(20.0),
            color_palette_gap: px(6.0),
            color_palette_selection_width: px(2.0),
            color_palette_plus_line: px(2.0),
            tool_button_size: px(20.0),
            tool_button_gap: px(18.0),
            tool_button_hover_duration_ms: 350,
            tool_button_select_duration_ms: 120,
            tool_button_selected_opacity: 0.35,

            marker_opacity: 0.35,
            marker_size_multiplier: 2.5,
            arrow_head_length_factor: 2.5,
            arrow_head_min_distance_factor: 1.5,
            arrow_head_angle_degrees: 26.0,
            eraser_preview_opacity: 0.25,
            blur_size_multiplier: 3.0,
            blur_preview_opacity: 0.25,
            blur_radius: 20.0,
            brush_size_control_height: px(280.0),
            brush_size_control_collapsed_width: px(6.0),
            brush_size_control_expanded_top_width: px(25.0),
            brush_size_control_expanded_bottom_width: px(4.0),
            brush_size_control_hit_padding: px(24.0),

            crop_point_size: px(10.0),
            crop_min_size: px(20.0),
            item_handle_size: px(10.0),
            sticky_trigger: px(12.0),
            sticky_line_width: px(2.0),

            audio_disc_size: px(48.0),
            audio_disc_skip: px(12.0),
            audio_disc_cover_size: px(30.0),
            audio_disc_ring: px(2.0),
            audio_timeline_bar_width: px(2.0),
            audio_timeline_bar_skip: px(3.0),
            audio_timeline_bar_ratio: 0.6,
            audio_timeline_bar_opacity: 0.4,

            video_timeline_strip_height: px(54.0),
            video_timeline_label_height: px(18.0),
            video_timeline_trim_strip_height: px(32.0),
            video_timeline_dot_size: px(5.0),
            video_timeline_dot_active_size: px(9.0),
            video_timeline_radius: px(6.0),
            video_timeline_handle_width: px(11.0),
            video_quality_slider_dot_size: px(7.0),
            video_quality_slider_dot_active_size: px(11.0),
            video_quality_slider_height: px(32.0),

            message_reference_width: px(600.0),
            link_preview_max_height: px(320.0),
            link_preview_padding: px(12.0),
            link_theme_size: px(36.0),
        }
    }
}

impl EditorStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
}
