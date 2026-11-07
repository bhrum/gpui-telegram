//! Channel Earn Currency Icons & Vector Badges
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/earn/earn_icons.cpp:L1-L250`
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/earn/earn_icons.h:L1-L51`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Upstream: `Ui::Earn::kCurrencyBody` SVG path
pub const K_CURRENCY_BODY: &str = "\
M26.0148187,12.0018928 L45.6469002,12.0007985 \
C48.1777253,12.0066542 49.5893649,12.0554519 50.8804395,12.462099 \
C52.179101,12.873841 53.3833619,13.5432922 54.423511,14.4349062 \
C55.5981446,15.4450073 56.4762152,16.8342664 58.2300327,19.6157469 \
L62.7287285,26.754584 C64.0735736,28.8873418 64.9397669,30.2612984 65.5583699,31.7827827 \
C66.177002,33.3043444 66.5273397,34.9213567 66.936058,37.3394627 \
L67.7287285,42.0232497 C68.1402283,44.4578135 68.3475736,46.0694119 68.1965565,47.6698661";

/// Upstream: `Ui::Earn::kCurrencyShine` SVG path
pub const K_CURRENCY_SHINE: &str = "\
M35.4851813,12.0018928 L45.6469002,12.0007985 \
C48.1777253,12.0066542 49.5893649,12.0554519 50.8804395,12.462099 \
C52.179101,12.873841 53.3833619,13.5432922 54.423511,14.4349062";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EarnCurrencyKind {
    Ton,
    Stars,
    Usd,
}

#[derive(Debug, Clone)]
pub struct IconDescriptor {
    pub kind: EarnCurrencyKind,
    pub size: f32,
    pub primary_color: u32,
    pub shine_opacity: f32,
}

impl IconDescriptor {
    pub fn ton(size: f32) -> Self {
        Self {
            kind: EarnCurrencyKind::Ton,
            size,
            primary_color: 0x0088cc,
            shine_opacity: 0.85,
        }
    }

    pub fn stars(size: f32) -> Self {
        Self {
            kind: EarnCurrencyKind::Stars,
            size,
            primary_color: 0xffa800,
            shine_opacity: 0.90,
        }
    }

    pub fn usd(size: f32) -> Self {
        Self {
            kind: EarnCurrencyKind::Usd,
            size,
            primary_color: 0x4fae4e,
            shine_opacity: 0.70,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EarnIconsChannelStatisticsEarn {
    pub descriptor: IconDescriptor,
    pub is_visible: bool,
}

impl EarnIconsChannelStatisticsEarn {
    pub fn new() -> Self {
        Self {
            descriptor: IconDescriptor::ton(24.0),
            is_visible: true,
        }
    }

    pub fn with_kind(kind: EarnCurrencyKind, size: f32) -> Self {
        let descriptor = match kind {
            EarnCurrencyKind::Ton => IconDescriptor::ton(size),
            EarnCurrencyKind::Stars => IconDescriptor::stars(size),
            EarnCurrencyKind::Usd => IconDescriptor::usd(size),
        };
        Self {
            descriptor,
            is_visible: true,
        }
    }

    pub fn set_currency(&mut self, kind: EarnCurrencyKind) {
        self.descriptor.kind = kind;
        match kind {
            EarnCurrencyKind::Ton => {
                self.descriptor.primary_color = 0x0088cc;
            }
            EarnCurrencyKind::Stars => {
                self.descriptor.primary_color = 0xffa800;
            }
            EarnCurrencyKind::Usd => {
                self.descriptor.primary_color = 0x4fae4e;
            }
        }
    }
}

impl Default for EarnIconsChannelStatisticsEarn {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for EarnIconsChannelStatisticsEarn {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let size = self.descriptor.size;
        let color = rgb(self.descriptor.primary_color);
        let symbol = match self.descriptor.kind {
            EarnCurrencyKind::Ton => "💎",
            EarnCurrencyKind::Stars => "⭐",
            EarnCurrencyKind::Usd => "$",
        };

        div()
            .flex()
            .items_center()
            .justify_center()
            .w(px(size))
            .h(px(size))
            .rounded_full()
            .bg(color)
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0xffffff))
                    .font_weight(FontWeight::BOLD)
                    .child(symbol),
            )
    }
}
