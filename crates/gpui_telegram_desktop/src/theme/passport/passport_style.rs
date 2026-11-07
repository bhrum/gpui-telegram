//! Telegram Passport Theme Style Sheet
//! Upstream style: `Telegram/SourceFiles/passport/passport.style`
//! Subsystem: `passport`, Feature: `TCG-N02`

use gpui::{px, rgb, Pixels, Rgba};

/// Visual geometry tokens, dimensions, and typography metrics for Telegram Passport.
#[derive(Debug, Clone)]
pub struct PassportStyleSheet {
    // Window & Panel dimensions
    pub panel_width: Pixels,
    pub panel_height: Pixels,
    pub panel_scroll_delta_top: Pixels,
    pub panel_scroll_delta_bottom: Pixels,

    // Password view metrics
    pub password_field_bottom: Pixels,
    pub password_field_skip: Pixels,
    pub password_hint_skip: Pixels,
    pub password_userpic_skip: Pixels,
    pub password_userpic_size: Pixels,
    pub password_submit_width: Pixels,
    pub password_submit_height: Pixels,
    pub password_submit_bottom: Pixels,
    pub password_forgot_bottom: Pixels,
    pub password_icon_height: Pixels,

    // Form header & userpic
    pub form_userpic_size: Pixels,
    pub form_divider_height: Pixels,
    pub form_authorize_height: Pixels,

    // Row & Item list metrics
    pub row_padding_left: Pixels,
    pub row_padding_top: Pixels,
    pub row_padding_right: Pixels,
    pub row_padding_bottom: Pixels,
    pub row_skip: Pixels,
    pub row_icon_skip: Pixels,

    // Scan item metrics
    pub scan_row_size: Pixels,
    pub scan_text_left: Pixels,
    pub scan_name_top: Pixels,
    pub scan_status_top: Pixels,
    pub scan_border: Pixels,

    // Details input field metrics
    pub details_field_left: Pixels,
    pub details_field_top: Pixels,
    pub details_field_min_height: Pixels,
    pub details_date_field_min_height: Pixels,
    pub details_field_skip_min: Pixels,
    pub details_skip: Pixels,
    pub details_gender_skip: Pixels,
    pub request_type_skip: Pixels,

    // Palette colors
    pub bg_color: Rgba,
    pub text_color: Rgba,
    pub subtext_color: Rgba,
    pub active_text_color: Rgba,
    pub accent_color: Rgba,
    pub border_color: Rgba,
    pub error_color: Rgba,
    pub attention_color: Rgba,
}

impl Default for PassportStyleSheet {
    fn default() -> Self {
        Self {
            // st::passportPanelSize = size(392px, 600px)
            panel_width: px(392.0),
            panel_height: px(600.0),
            panel_scroll_delta_top: px(6.0),
            panel_scroll_delta_bottom: px(6.0),

            // Password view metrics
            password_field_bottom: px(306.0),
            password_field_skip: px(29.0),
            password_hint_skip: px(10.0),
            password_userpic_skip: px(14.0),
            password_userpic_size: px(80.0),
            password_submit_width: px(200.0),
            password_submit_height: px(44.0),
            password_submit_bottom: px(72.0),
            password_forgot_bottom: px(36.0),
            password_icon_height: px(224.0),

            // Form header & userpic
            form_userpic_size: px(60.0),
            form_divider_height: px(13.0),
            form_authorize_height: px(49.0),

            // Row metrics: margins(22px, 8px, 25px, 8px)
            row_padding_left: px(22.0),
            row_padding_top: px(8.0),
            row_padding_right: px(25.0),
            row_padding_bottom: px(8.0),
            row_skip: px(2.0),
            row_icon_skip: px(10.0),

            // Scan item metrics
            scan_row_size: px(40.0),
            scan_text_left: px(53.0),
            scan_name_top: px(1.0),
            scan_status_top: px(22.0),
            scan_border: px(1.0),

            // Details input field metrics
            details_field_left: px(116.0),
            details_field_top: px(2.0),
            details_field_min_height: px(32.0),
            details_date_field_min_height: px(30.0),
            details_field_skip_min: px(12.0),
            details_skip: px(30.0),
            details_gender_skip: px(20.0),
            request_type_skip: px(16.0),

            // Theme colors
            bg_color: rgb(0xFFFFFF),
            text_color: rgb(0x222222),
            subtext_color: rgb(0x707579),
            active_text_color: rgb(0x2AABEE),
            accent_color: rgb(0x2AABEE),
            border_color: rgb(0xDADCE0),
            error_color: rgb(0xE53935),
            attention_color: rgb(0xEE5555),
        }
    }
}

impl PassportStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn panel_size(&self) -> (Pixels, Pixels) {
        (self.panel_width, self.panel_height)
    }

    pub fn password_submit_size(&self) -> (Pixels, Pixels) {
        (self.password_submit_width, self.password_submit_height)
    }

    pub fn content_width(&self, full_width: Pixels) -> Pixels {
        full_width - self.row_padding_left - self.row_padding_right
    }

    pub fn row_vertical_padding(&self) -> Pixels {
        self.row_padding_top + self.row_padding_bottom
    }
}
