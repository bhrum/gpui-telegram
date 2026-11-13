//! Channel Boost Badge & Radial Progress Widget
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/boost_badge.cpp:L1-L190`
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/boost_badge.h:L1-L47`
//! Subsystem: info | Feature ID: TCG-N32 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone)]
pub struct BoostBadgeGeometry {
    pub badge_height: f32,
    pub padding_horizontal: f32,
    pub padding_vertical: f32,
    pub bg_color: u32,
    pub fg_color: u32,
    pub bg_opacity: f32,
}

impl Default for BoostBadgeGeometry {
    fn default() -> Self {
        Self {
            badge_height: 20.0,
            padding_horizontal: 8.0,
            padding_vertical: 2.0,
            bg_color: 0x9b51e0, // Telegram Premium purple
            fg_color: 0xffffff,
            bg_opacity: 0.85,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoostBadgeChannelStatisticsBoostsGiveaway {
    pub boost_count: usize,
    pub custom_label: Option<String>,
    pub is_loading: bool,
    pub spin_angle: f32,
    pub geometry: BoostBadgeGeometry,
    pub is_visible: bool,
}

impl BoostBadgeChannelStatisticsBoostsGiveaway {
    pub fn new() -> Self {
        Self {
            boost_count: 1,
            custom_label: None,
            is_loading: false,
            spin_angle: 0.0,
            geometry: BoostBadgeGeometry::default(),
            is_visible: true,
        }
    }

    pub fn with_boosts(boosts: usize) -> Self {
        let mut badge = Self::new();
        badge.boost_count = boosts;
        badge
    }

    pub fn set_boosts(&mut self, boosts: usize) {
        self.boost_count = boosts;
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }

    pub fn advance_spin(&mut self, delta_degrees: f32) {
        self.spin_angle = (self.spin_angle + delta_degrees) % 360.0;
    }

    pub fn badge_text(&self) -> String {
        if let Some(ref label) = self.custom_label {
            label.clone()
        } else {
            format!("⚡ {}", self.boost_count)
        }
    }

    pub fn computed_width(&self) -> f32 {
        let text_len = self.badge_text().len() as f32;
        let char_width = 7.5;
        text_len * char_width + self.geometry.padding_horizontal * 2.0
    }
}

impl Default for BoostBadgeChannelStatisticsBoostsGiveaway {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for BoostBadgeChannelStatisticsBoostsGiveaway {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let h = self.geometry.badge_height;
        let bg = rgb(self.geometry.bg_color);
        let fg = rgb(self.geometry.fg_color);

        if self.is_loading {
            div()
                .flex()
                .items_center()
                .justify_center()
                .w(px(h))
                .h(px(h))
                .rounded_full()
                .border_2()
                .border_color(bg)
                .child(
                    div()
                        .text_xs()
                        .text_color(fg)
                        .child("↻"),
                )
        } else {
            div()
                .flex()
                .items_center()
                .justify_center()
                .h(px(h))
                .px(px(self.geometry.padding_horizontal))
                .rounded_full()
                .bg(bg)
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(fg)
                        .child(self.badge_text()),
                )
        }
    }
}
