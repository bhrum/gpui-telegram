//! GPUI Telegram Theme: Payments Style Sheet
//! Upstream Source: `Telegram/SourceFiles/payments/ui/payments.style`
//! Subsystem: payments | Feature: CAP-06

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::{px, rgb, Pixels, Rgba, Size};

/// Complete theme metrics extracted from `payments.style`
#[derive(Debug, Clone)]
pub struct PaymentsUiStyleSheet {
    pub payments_panel_width: Pixels,
    pub payments_panel_height: Pixels,
    pub payments_panel_button_height: Pixels,
    pub payments_panel_button_width_offset: Pixels,
    pub payments_panel_padding_top: Pixels,
    pub payments_panel_padding_right: Pixels,
    pub payments_panel_padding_bottom: Pixels,
    pub payments_panel_padding_left: Pixels,
    pub payments_cover_padding_top: Pixels,
    pub payments_cover_padding_right: Pixels,
    pub payments_cover_padding_bottom: Pixels,
    pub payments_cover_padding_left: Pixels,
    pub payments_thumbnail_width: Pixels,
    pub payments_thumbnail_height: Pixels,
    pub payments_thumbnail_skip: Pixels,
    pub payments_prices_top_skip: Pixels,
    pub payments_prices_bottom_skip: Pixels,
    pub payments_price_padding_horizontal: Pixels,
    pub payments_price_padding_vertical: Pixels,
    pub payments_tip_skip: Pixels,
    pub payments_tip_button_height: Pixels,
    pub payments_tip_buttons_padding: Pixels,
    pub payments_sections_top_skip: Pixels,
    pub payments_field_padding_horizontal: Pixels,
    pub payments_save_checkbox_padding_top: Pixels,
    pub payments_save_checkbox_padding_bottom: Pixels,
    pub payments_expire_cvc_skip: Pixels,
    pub payments_shipping_margin_horizontal: Pixels,
    pub payments_shipping_margin_vertical: Pixels,
    pub payments_loading_size: Pixels,
    pub payments_loading_thickness: Pixels,
    pub bot_webview_panel_width: Pixels,
    pub bot_webview_panel_height: Pixels,
    pub bot_webview_bottom_button_height: Pixels,
    pub bot_webview_shell_radius: Pixels,
    pub bot_webview_shell_header_height: Pixels,
    pub bot_webview_shell_menu_button_size: Pixels,
    pub bg_color: Rgba,
    pub text_color: Rgba,
    pub accent_color: Rgba,
    pub border_color: Rgba,
}

impl Default for PaymentsUiStyleSheet {
    fn default() -> Self {
        Self {
            payments_panel_width: px(392.0),
            payments_panel_height: px(600.0),
            payments_panel_button_height: px(36.0),
            payments_panel_button_width_offset: px(-36.0),
            payments_panel_padding_top: px(8.0),
            payments_panel_padding_right: px(12.0),
            payments_panel_padding_bottom: px(15.0),
            payments_panel_padding_left: px(12.0),
            payments_cover_padding_top: px(26.0),
            payments_cover_padding_right: px(0.0),
            payments_cover_padding_bottom: px(26.0),
            payments_cover_padding_left: px(13.0),
            payments_thumbnail_width: px(80.0),
            payments_thumbnail_height: px(80.0),
            payments_thumbnail_skip: px(18.0),
            payments_prices_top_skip: px(12.0),
            payments_prices_bottom_skip: px(13.0),
            payments_price_padding_horizontal: px(28.0),
            payments_price_padding_vertical: px(6.0),
            payments_tip_skip: px(8.0),
            payments_tip_button_height: px(28.0),
            payments_tip_buttons_padding: px(26.0),
            payments_sections_top_skip: px(11.0),
            payments_field_padding_horizontal: px(28.0),
            payments_save_checkbox_padding_top: px(20.0),
            payments_save_checkbox_padding_bottom: px(8.0),
            payments_expire_cvc_skip: px(34.0),
            payments_shipping_margin_horizontal: px(27.0),
            payments_shipping_margin_vertical: px(11.0),
            payments_loading_size: px(24.0),
            payments_loading_thickness: px(4.0),
            bot_webview_panel_width: px(384.0),
            bot_webview_panel_height: px(694.0),
            bot_webview_bottom_button_height: px(40.0),
            bot_webview_shell_radius: px(7.0),
            bot_webview_shell_header_height: px(60.0),
            bot_webview_shell_menu_button_size: px(36.0),
            bg_color: rgb(0xFFFFFF),
            text_color: rgb(0x222222),
            accent_color: rgb(0x2AABEE),
            border_color: rgb(0xE0E0E0),
        }
    }
}

impl PaymentsUiStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn panel_size(&self) -> Size<Pixels> {
        Size {
            width: self.payments_panel_width,
            height: self.payments_panel_height,
        }
    }

    pub fn thumbnail_size(&self) -> Size<Pixels> {
        Size {
            width: self.payments_thumbnail_width,
            height: self.payments_thumbnail_height,
        }
    }

    pub fn bot_webview_size(&self) -> Size<Pixels> {
        Size {
            width: self.bot_webview_panel_width,
            height: self.bot_webview_panel_height,
        }
    }
}
