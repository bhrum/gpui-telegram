//! Genuine History View Location Component (`SPEC-06-HISTORY-SLICE-6`)
//! Upstream sources:
//! - `Telegram/SourceFiles/history/view/media/history_view_location.h:L25-L135`
//! - `Telegram/SourceFiles/history/view/media/history_view_location.cpp:L48-L79`

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::views::stage::media::location::{
    RichLocationCard, remaining_time_text, remaining_time_progress,
    K_UNTIL_OFF_PERIOD, K_LIVE_ELAPSED_PART_OPACITY,
};

#[derive(Debug, Clone)]
pub struct HistoryViewLocationViewMedia {
    pub latitude: f64,
    pub longitude: f64,
    pub title: Option<String>,
    pub address: Option<String>,
    pub is_live: bool,
    pub live_period: i64,
    pub elapsed_seconds: i64,
}

impl HistoryViewLocationViewMedia {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            title: None,
            address: None,
            is_live: false,
            live_period: 0,
            elapsed_seconds: 0,
        }
    }

    pub fn new_live(latitude: f64, longitude: f64, period: i64) -> Self {
        Self {
            latitude,
            longitude,
            title: Some("Live Location".to_string()),
            address: None,
            is_live: true,
            live_period: period,
            elapsed_seconds: 0,
        }
    }

    pub fn advance_time(&mut self, delta_seconds: i64) {
        if self.is_live {
            self.elapsed_seconds += delta_seconds;
        }
    }

    pub fn is_active(&self) -> bool {
        if !self.is_live {
            true
        } else if self.live_period == K_UNTIL_OFF_PERIOD {
            true
        } else {
            self.elapsed_seconds < self.live_period
        }
    }
}

impl Default for HistoryViewLocationViewMedia {
    fn default() -> Self {
        Self::new(37.7749, -122.4194)
    }
}

impl Render for HistoryViewLocationViewMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut card = if self.is_live {
            RichLocationCard::new_live(self.latitude, self.longitude, self.live_period)
        } else {
            RichLocationCard::new_static(self.latitude, self.longitude)
        };
        card.title = self.title.clone();
        card.address = self.address.clone();
        card.elapsed_seconds = self.elapsed_seconds;
        card.render()
    }
}
