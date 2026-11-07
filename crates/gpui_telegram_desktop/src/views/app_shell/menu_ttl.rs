//! MenuTtl: Auto-delete / TTL interval chooser menu (Off, 24h, 7d, 1m, custom)
//! Upstream: Telegram/SourceFiles/menu/menu_ttl.h, menu_ttl.cpp

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TtlPeriod {
    Disabled,
    Day1,
    Week1,
    Month1,
}

impl TtlPeriod {
    pub fn to_seconds(self) -> u32 {
        match self {
            TtlPeriod::Disabled => 0,
            TtlPeriod::Day1 => 86400,
            TtlPeriod::Week1 => 7 * 86400,
            TtlPeriod::Month1 => 30 * 86400,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MenuTtl {
    pub current_period: TtlPeriod,
    pub is_admin: bool,
}

impl MenuTtl {
    pub fn new(current_period: TtlPeriod, is_admin: bool) -> Self {
        Self {
            current_period,
            is_admin,
        }
    }

    pub fn set_period(&mut self, period: TtlPeriod) {
        if self.is_admin {
            self.current_period = period;
        }
    }
}

impl Default for MenuTtl {
    fn default() -> Self {
        Self::new(TtlPeriod::Disabled, true)
    }
}

impl Render for MenuTtl {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0xFFFFFF))
            .child(format!("Auto-Delete: {:?}", self.current_period))
    }
}
