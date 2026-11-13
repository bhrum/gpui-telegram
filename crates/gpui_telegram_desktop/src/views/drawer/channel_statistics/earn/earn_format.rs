//! Channel Earn Amount Formatting
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/earn/earn_format.cpp:L1-L95`
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/earn/earn_format.h:L1-L27`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

pub const K_MINOR_PART_LENGTH: usize = 9;
pub const K_MAX_CHOPPED_ZERO: usize = K_MINOR_PART_LENGTH - 2; // 7
pub const K_APPROXIMATELY: char = '≈';

/// Upstream: `Info::ChannelEarn::MajorPart(EarnInt value)`
pub fn major_part(value: i64) -> String {
    let s = value.abs().to_string();
    let len = s.len();
    if len <= K_MINOR_PART_LENGTH {
        "0".to_string()
    } else {
        s[..len - K_MINOR_PART_LENGTH].to_string()
    }
}

/// Upstream: `Info::ChannelEarn::MajorPart(CreditsAmount value)`
pub fn credits_major_part(value: f64) -> String {
    (value as i64).to_string()
}

/// Upstream: `Info::ChannelEarn::MinorPart(EarnInt value)`
pub fn minor_part(value: i64) -> String {
    if value == 0 {
        return ".00".to_string();
    }
    let s = value.abs().to_string();
    let len = s.len();
    let unchopped = if len < K_MINOR_PART_LENGTH {
        let pad = K_MINOR_PART_LENGTH - len;
        format!(".{}{}", "0".repeat(pad), s)
    } else {
        format!(".{}", &s[len - K_MINOR_PART_LENGTH..])
    };

    let mut chars: Vec<char> = unchopped.chars().collect();
    let mut chopped_count = 0;
    while chars.len() > 3 && chars.last() == Some(&'0') && chopped_count < K_MAX_CHOPPED_ZERO {
        chars.pop();
        chopped_count += 1;
    }
    chars.into_iter().collect()
}

/// Upstream: `Info::ChannelEarn::MinorPart(CreditsAmount value)`
pub fn credits_minor_part(value: f64) -> String {
    let fractional = ((value * 100.0).abs() as i64) % 100;
    format!(".{:02}", fractional)
}

/// Upstream: `Info::ChannelEarn::ToUsd(EarnInt value, float64 rate, int afterFloat)`
pub fn to_usd(value: i64, rate: f64, after_float: Option<usize>) -> String {
    let ton = (value as f64) / 1_000_000_000.0;
    let usd = ton * rate;
    let precision = after_float.unwrap_or(2);
    format!("{}${:.prec$}", K_APPROXIMATELY, usd, prec = precision)
}

/// Upstream: `Info::ChannelEarn::ToUsd(CreditsAmount value, float64 rate, int afterFloat)`
pub fn credits_to_usd(credits: f64, rate: f64, after_float: Option<usize>) -> String {
    let usd = credits * rate;
    let precision = after_float.unwrap_or(2);
    format!("{}${:.prec$}", K_APPROXIMATELY, usd, prec = precision)
}

#[derive(Debug, Clone)]
pub struct EarnFormatChannelStatisticsEarn {
    pub raw_nanoton: i64,
    pub ton_rate: f64,
    pub is_visible: bool,
}

impl EarnFormatChannelStatisticsEarn {
    pub fn new() -> Self {
        Self {
            raw_nanoton: 12_500_000_000, // 12.5 TON
            ton_rate: 6.50,
            is_visible: true,
        }
    }

    pub fn formatted_major(&self) -> String {
        major_part(self.raw_nanoton)
    }

    pub fn formatted_minor(&self) -> String {
        minor_part(self.raw_nanoton)
    }

    pub fn formatted_usd(&self) -> String {
        to_usd(self.raw_nanoton, self.ton_rate, Some(2))
    }

    pub fn set_amount(&mut self, nanoton: i64, rate: f64) {
        self.raw_nanoton = nanoton;
        self.ton_rate = rate;
    }
}

impl Default for EarnFormatChannelStatisticsEarn {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for EarnFormatChannelStatisticsEarn {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .child(
                div()
                    .flex()
                    .items_baseline()
                    .gap_1()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .child(self.formatted_major()),
                    )
                    .child(
                        div()
                            .text_lg()
                            .text_color(rgb(0x8e8e93))
                            .child(self.formatted_minor()),
                    )
                    .child(div().text_sm().child("TON")),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x8e8e93))
                    .child(self.formatted_usd()),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nanoton_formatting() {
        assert_eq!(major_part(0), "0");
        assert_eq!(minor_part(0), ".00");

        assert_eq!(major_part(500_000_000), "0");
        assert_eq!(minor_part(500_000_000), ".50");

        assert_eq!(major_part(12_340_000_000), "12");
        assert_eq!(minor_part(12_340_000_000), ".34");

        assert_eq!(major_part(1_000_000_000), "1");
        assert_eq!(minor_part(1_000_000_000), ".00");
    }

    #[test]
    fn test_to_usd() {
        let usd = to_usd(10_000_000_000, 5.0, Some(2));
        assert_eq!(usd, "≈$50.00");
    }
}
