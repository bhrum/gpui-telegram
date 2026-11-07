//! Giveaway Country Filter Modal Box
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/select_countries_box.cpp:L1-L229`
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/select_countries_box.h:L1-L25`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct CountryEntry {
    pub code: String,
    pub name: String,
    pub flag: String,
}

#[derive(Debug, Clone)]
pub struct SelectCountriesBoxChannelStatisticsBoostsGiveaway {
    pub selected_countries: HashSet<String>,
    pub search_query: String,
    pub countries: Vec<CountryEntry>,
    pub is_visible: bool,
}

impl SelectCountriesBoxChannelStatisticsBoostsGiveaway {
    pub fn new() -> Self {
        let mut b = Self {
            selected_countries: HashSet::new(),
            search_query: String::new(),
            countries: Vec::new(),
            is_visible: true,
        };
        b.init_default_countries();
        b
    }

    pub fn init_default_countries(&mut self) {
        self.countries = vec![
            CountryEntry { code: "US".into(), name: "United States".into(), flag: "🇺🇸".into() },
            CountryEntry { code: "DE".into(), name: "Germany".into(), flag: "🇩🇪".into() },
            CountryEntry { code: "GB".into(), name: "United Kingdom".into(), flag: "🇬🇧".into() },
            CountryEntry { code: "FR".into(), name: "France".into(), flag: "🇫🇷".into() },
            CountryEntry { code: "ES".into(), name: "Spain".into(), flag: "🇪🇸".into() },
            CountryEntry { code: "IT".into(), name: "Italy".into(), flag: "🇮🇹".into() },
            CountryEntry { code: "BR".into(), name: "Brazil".into(), flag: "🇧🇷".into() },
            CountryEntry { code: "IN".into(), name: "India".into(), flag: "🇮🇳".into() },
            CountryEntry { code: "JP".into(), name: "Japan".into(), flag: "🇯🇵".into() },
        ];
    }

    pub fn toggle_country(&mut self, code: &str) {
        if self.selected_countries.contains(code) {
            self.selected_countries.remove(code);
        } else {
            self.selected_countries.insert(code.to_string());
        }
    }

    pub fn is_selected(&self, code: &str) -> bool {
        self.selected_countries.contains(code)
    }

    pub fn set_search_query(&mut self, query: impl Into<String>) {
        self.search_query = query.into();
    }

    pub fn filtered_countries(&self) -> Vec<&CountryEntry> {
        let q = self.search_query.to_lowercase();
        self.countries
            .iter()
            .filter(|c| q.is_empty() || c.name.to_lowercase().contains(&q) || c.code.to_lowercase().contains(&q))
            .collect()
    }
}

impl Default for SelectCountriesBoxChannelStatisticsBoostsGiveaway {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SelectCountriesBoxChannelStatisticsBoostsGiveaway {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let filtered = self.filtered_countries();
        let selected_count = self.selected_countries.len();

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .gap_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child("Select Countries"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8e8e93))
                            .child(format!("{} selected", selected_count)),
                    ),
            )
            .child(
                div()
                    .p(px(8.0))
                    .rounded_md()
                    .bg(rgb(0x242f3d))
                    .child(
                        div()
                            .text_sm()
                            .text_color(if self.search_query.is_empty() { rgb(0x8e8e93) } else { rgb(0xffffff) })
                            .child(if self.search_query.is_empty() { "Search countries...".to_string() } else { self.search_query.clone() }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(filtered.into_iter().map(|country| {
                        let is_checked = self.is_selected(&country.code);
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px(px(8.0))
                            .py(px(6.0))
                            .rounded_md()
                            .bg(if is_checked { rgb(0x1e2c3a) } else { rgb(0x00000000) })
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(div().text_base().child(country.flag.clone()))
                                    .child(div().text_sm().text_color(rgb(0xffffff)).child(country.name.clone())),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(if is_checked { rgb(0x0088cc) } else { rgb(0x6c7883) })
                                    .child(if is_checked { "☑" } else { "☐" }),
                            )
                    })),
            )
    }
}
