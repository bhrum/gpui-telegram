//! Upstream Source: Telegram/SourceFiles/intro/intro.style
//! Subsystem: intro | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct IntroStyleSheet {
    pub bg_color: Rgba,
    pub card_bg: Rgba,
    pub title_color: Rgba,
    pub description_color: Rgba,
    pub accent_color: Rgba,
    pub border_color: Rgba,
    pub error_color: Rgba,

    // Cover & Header geometry
    pub cover_height: Pixels,
    pub cover_max_width: Pixels,
    pub cover_icons_min_skip: Pixels,
    pub cover_icon_left: Pixels,
    pub cover_icon_top: Pixels,
    pub cover_title_top: Pixels,
    pub cover_description_top: Pixels,
    pub plane_width: Pixels,
    pub plane_height: Pixels,

    // Step container geometry
    pub step_width: Pixels,
    pub step_height: Pixels,
    pub step_height_full: Pixels,
    pub step_top_min: Pixels,
    pub step_field_top: Pixels,
    pub content_top_add: Pixels,
    pub next_top: Pixels,
    pub slide_duration_ms: u64,

    // Interactive buttons
    pub next_button_width: Pixels,
    pub next_button_height: Pixels,
    pub next_button_radius: Pixels,
    pub back_button_size: Pixels,

    // Phone & Country Inputs
    pub country_width: Pixels,
    pub country_height_min: Pixels,
    pub country_code_width: Pixels,
    pub phone_width: Pixels,
    pub phone_top: Pixels,

    // Segmented Code Input
    pub code_digit_height: Pixels,
    pub code_digit_border_width: Pixels,
    pub code_digit_skip: Pixels,
    pub code_digit_font_size: Pixels,

    // Password & Recovery
    pub password_top: Pixels,
    pub password_hint_top: Pixels,
    pub reset_bottom: Pixels,
    pub terms_bottom: Pixels,

    // QR Code Onboarding
    pub qr_max_size: Pixels,
    pub qr_background_skip: Pixels,
    pub qr_background_radius: Pixels,
    pub qr_labels_width: Pixels,
    pub qr_title_width: Pixels,
    pub qr_title_top: Pixels,
    pub qr_steps_top: Pixels,
    pub qr_center_size: Pixels,
    pub qr_login_link_top: Pixels,
}

impl Default for IntroStyleSheet {
    fn default() -> Self {
        Self {
            bg_color: rgb(0x0e1621),
            card_bg: rgb(0x17212b),
            title_color: rgb(0xffffff),
            description_color: rgb(0x8292a1),
            accent_color: rgb(0x2aabee),
            border_color: rgb(0x242f3d),
            error_color: rgb(0xee5655),

            cover_height: px(208.0),
            cover_max_width: px(880.0),
            cover_icons_min_skip: px(120.0),
            cover_icon_left: px(50.0),
            cover_icon_top: px(46.0),
            cover_title_top: px(136.0),
            cover_description_top: px(174.0),
            plane_width: px(48.0),
            plane_height: px(38.0),

            step_width: px(380.0),
            step_height: px(384.0),
            step_height_full: px(590.0),
            step_top_min: px(76.0),
            step_field_top: px(96.0),
            content_top_add: px(30.0),
            next_top: px(266.0),
            slide_duration_ms: 200,

            next_button_width: px(300.0),
            next_button_height: px(42.0),
            next_button_radius: px(6.0),
            back_button_size: px(56.0),

            country_width: px(300.0),
            country_height_min: px(61.0),
            country_code_width: px(64.0),
            phone_width: px(225.0),
            phone_top: px(6.0),

            code_digit_height: px(50.0),
            code_digit_border_width: px(4.0),
            code_digit_skip: px(10.0),
            code_digit_font_size: px(20.0),

            password_top: px(74.0),
            password_hint_top: px(151.0),
            reset_bottom: px(20.0),
            terms_bottom: px(20.0),

            qr_max_size: px(180.0),
            qr_background_skip: px(12.0),
            qr_background_radius: px(8.0),
            qr_labels_width: px(292.0),
            qr_title_width: px(320.0),
            qr_title_top: px(196.0),
            qr_steps_top: px(232.0),
            qr_center_size: px(44.0),
            qr_login_link_top: px(368.0),
        }
    }
}

impl IntroStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }
}
