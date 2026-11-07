//! Suggest options bar for channel suggested posts and paid post price configuration.
//! Upstream reference: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_suggest_options.cpp`
//! Upstream header: `tdesktop/Telegram/SourceFiles/history/view/controls/history_view_suggest_options.h`

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuggestMode {
    New,
    Change,
    Publish,
    Gift,
}

impl Default for SuggestMode {
    fn default() -> Self {
        Self::New
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuggestPostOptions {
    pub mode: SuggestMode,
    pub price_stars: u64,
    pub price_nano_ton: u64,
    pub commission_percent: u32,
    pub publish_at_timestamp: Option<u64>,
}

impl Default for SuggestPostOptions {
    fn default() -> Self {
        Self {
            mode: SuggestMode::New,
            price_stars: 0,
            price_nano_ton: 0,
            commission_percent: 15, // Standard Telegram commission ~15%
            publish_at_timestamp: None,
        }
    }
}

pub struct HistoryViewSuggestOptionsViewControls {
    pub options: SuggestPostOptions,
    pub is_visible: bool,
}

impl HistoryViewSuggestOptionsViewControls {
    pub fn new() -> Self {
        Self {
            options: SuggestPostOptions::default(),
            is_visible: false,
        }
    }

    /// Price received by creator after commission
    /// Upstream: `PriceAfterCommission` (`history_view_suggest_options.cpp:L96-98`)
    pub fn price_after_commission_stars(&self) -> u64 {
        let stars = self.options.price_stars;
        let cut = stars * (self.options.commission_percent as u64) / 100;
        stars.saturating_sub(cut)
    }

    pub fn set_stars_price(&mut self, stars: u64) {
        self.options.price_stars = stars;
        self.is_visible = stars > 0;
    }
}

impl Default for HistoryViewSuggestOptionsViewControls {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewSuggestOptionsViewControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let stars = self.options.price_stars;
        let net = self.price_after_commission_stars();

        div()
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .py_1()
            .bg(rgb(0x18222d))
            .border_b_1()
            .border_color(rgb(0x2a394a))
            .text_xs()
            .child(
                div()
                    .text_color(rgb(0xffd700))
                    .child(format!("Suggested Post: {} Stars (Creator gets: {})", stars, net)),
            )
            .into_any_element()
    }
}
