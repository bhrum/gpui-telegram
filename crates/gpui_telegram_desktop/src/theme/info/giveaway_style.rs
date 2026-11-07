//! Giveaway Stylesheet Tokens
//! Upstream: `Telegram/SourceFiles/info/channel_statistics/boosts/giveaway/giveaway.style:L1-L279`
//! Subsystem: info | Feature ID: TCG-N12 | Disposition: gpui_ui_1to1

#[derive(Debug, Clone)]
pub struct GiveawayChannelStatisticsBoostsGiveawayStyleSheet {
    pub is_active: bool,
}

impl GiveawayChannelStatisticsBoostsGiveawayStyleSheet {
    pub fn new() -> Self {
        Self { is_active: true }
    }

    pub fn type_list_item_height() -> f32 {
        52.0
    }

    pub fn photo_size() -> f32 {
        42.0
    }

    pub fn gift_code_top_height() -> f32 {
        195.0
    }

    pub fn gift_code_link_height() -> f32 {
        42.0
    }

    pub fn gift_code_start_button_height() -> f32 {
        42.0
    }

    pub fn gift_code_start_button_radius() -> f32 {
        6.0
    }

    pub fn giveaway_radio_x() -> f32 {
        21.0
    }

    pub fn giveaway_radio_y() -> f32 {
        16.0
    }
}

impl Default for GiveawayChannelStatisticsBoostsGiveawayStyleSheet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_giveaway_style_sheet() {
        let sheet = GiveawayChannelStatisticsBoostsGiveawayStyleSheet::default();
        assert!(sheet.is_active);
        assert_eq!(GiveawayChannelStatisticsBoostsGiveawayStyleSheet::type_list_item_height(), 52.0);
        assert_eq!(GiveawayChannelStatisticsBoostsGiveawayStyleSheet::photo_size(), 42.0);
        assert_eq!(GiveawayChannelStatisticsBoostsGiveawayStyleSheet::gift_code_top_height(), 195.0);
        assert_eq!(GiveawayChannelStatisticsBoostsGiveawayStyleSheet::gift_code_link_height(), 42.0);
        assert_eq!(GiveawayChannelStatisticsBoostsGiveawayStyleSheet::gift_code_start_button_height(), 42.0);
        assert_eq!(GiveawayChannelStatisticsBoostsGiveawayStyleSheet::gift_code_start_button_radius(), 6.0);
        assert_eq!(GiveawayChannelStatisticsBoostsGiveawayStyleSheet::giveaway_radio_x(), 21.0);
        assert_eq!(GiveawayChannelStatisticsBoostsGiveawayStyleSheet::giveaway_radio_y(), 16.0);
    }
}

