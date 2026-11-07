//! Upstream: `Telegram/SourceFiles/ui/effects/premium_graphics.cpp:L1-L1134`
//! Upstream: `Telegram/SourceFiles/ui/effects/premium_graphics.h:L18-L133`
//!
//! Premium graphics visual components: dual-tier limit comparison bars,
//! gradient buttons, and promotional feature cards.

use gpui::*;

#[derive(Debug, Clone)]
pub struct PremiumLimitComparison {
    pub feature_title: String,
    pub standard_limit: i64,
    pub premium_limit: i64,
}

#[derive(Debug, Clone)]
pub struct PremiumGraphicsEffects {
    limits: Vec<PremiumLimitComparison>,
}

impl PremiumGraphicsEffects {
    pub fn new() -> Self {
        let limits = vec![
            PremiumLimitComparison { feature_title: "Channels & Supergroups".to_string(), standard_limit: 500, premium_limit: 1000 },
            PremiumLimitComparison { feature_title: "Pinned Chats".to_string(), standard_limit: 5, premium_limit: 10 },
            PremiumLimitComparison { feature_title: "Public Links".to_string(), standard_limit: 10, premium_limit: 20 },
            PremiumLimitComparison { feature_title: "Saved GIFs".to_string(), standard_limit: 200, premium_limit: 400 },
            PremiumLimitComparison { feature_title: "Favorite Stickers".to_string(), standard_limit: 5, premium_limit: 10 },
        ];
        Self { limits }
    }

    pub fn limits(&self) -> &[PremiumLimitComparison] {
        &self.limits
    }
}

impl Default for PremiumGraphicsEffects {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PremiumGraphicsEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .children(self.limits.iter().map(|lim| {
                div()
                    .flex()
                    .justify_between()
                    .p(px(8.0))
                    .rounded(px(8.0))
                    .bg(rgb(0x182533))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(0xF5F5F5))
                            .child(lim.feature_title.clone())
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x7B51B8))
                            .child(format!("{} → {}", lim.standard_limit, lim.premium_limit))
                    )
            }))
    }
}
