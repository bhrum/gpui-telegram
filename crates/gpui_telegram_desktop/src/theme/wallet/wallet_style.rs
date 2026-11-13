//! Telegram Desktop Wallet Theme & Layout StyleSheet
//! Upstream: `Telegram/SourceFiles/wallet/wallet.style`
//! Subsystem: `wallet`, Feature ID: `CAP-06`

use gpui::*;

#[derive(Debug, Clone)]
pub struct WalletStyleSheet {
    pub card_height: f32,
    pub card_margin: f32,
    pub card_radius: f32,
    pub card_softness: f32,
    pub card_field_step: f32,
    pub menu_icon_size: f32,
    pub menu_currency_icon_size: f32,
    pub panel_size: f32,
}

impl Default for WalletStyleSheet {
    fn default() -> Self {
        Self::new()
    }
}

impl WalletStyleSheet {
    pub fn new() -> Self {
        Self {
            card_height: 190.0,
            card_margin: 16.0,
            card_radius: 16.0,
            card_softness: 55.0,
            card_field_step: 4.0,
            menu_icon_size: 16.0,
            menu_currency_icon_size: 16.0,
            panel_size: 16.0,
        }
    }

    pub fn wallet_menu_icon() -> f32 {
        16.0
    }

    pub fn wallet_menu_currency_icon() -> f32 {
        16.0
    }

    pub fn wallet_panel_size() -> f32 {
        16.0
    }

    pub fn wallet_card_height() -> f32 {
        190.0
    }

    pub fn wallet_card_margin() -> f32 {
        16.0
    }

    pub fn wallet_card_radius() -> f32 {
        16.0
    }

    pub fn wallet_card_softness() -> f32 {
        55.0
    }

    pub fn wallet_card_field_step() -> f32 {
        4.0
    }

    pub fn card_background_gradient() -> (Hsla, Hsla) {
        (
            hsla(215.0 / 360.0, 0.90, 0.55, 1.0), // TON primary blue
            hsla(225.0 / 360.0, 0.85, 0.45, 1.0), // Deep gradient blue
        )
    }

    pub fn card_surface_shadow() -> BoxShadow {
        BoxShadow {
            color: hsla(0.0, 0.0, 0.0, 0.18),
            offset: point(px(0.0), px(4.0)),
            blur_radius: px(12.0),
            spread_radius: px(0.0),
        }
    }
}
