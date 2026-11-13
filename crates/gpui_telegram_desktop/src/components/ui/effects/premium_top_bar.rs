//! Upstream: `Telegram/SourceFiles/ui/effects/premium_top_bar.cpp:L27-L36`, `L54-L60`
//! Upstream: `Telegram/SourceFiles/ui/effects/premium_top_bar.h:L18-L142`
//!
//! Premium modal hero header container.
//! Features body animation ratio 0.90, title additional scale 0.15, min contrast 4.5,
//! and 3D object embedding (Star: 2.0x, Diamond: 1.58x, Coin: 1.85x).

use gpui::*;

pub const BODY_ANIMATION_PART: f32 = 0.90;
pub const TITLE_ADDITIONAL_SCALE: f32 = 0.15;
pub const MIN_ACCEPTABLE_CONTRAST: f32 = 4.5;
pub const STAR_3D_SCALE: f32 = 2.0;
pub const DIAMOND_3D_SCALE: f32 = 1.58;
pub const COIN_3D_SCALE: f32 = 1.85;
pub const STAR_PARTICLES_FIELD_SCALE: f32 = 3.0;

#[derive(Debug, Clone)]
pub struct PremiumTopBarEffects {
    title: String,
    subtitle: String,
    height: f32,
    body_progress: f32,
}

impl PremiumTopBarEffects {
    pub fn new(title: impl Into<String>, subtitle: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: subtitle.into(),
            height: 220.0,
            body_progress: 1.0,
        }
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.body_progress = progress.clamp(0.0, 1.0);
    }

    pub fn title_scale(&self) -> f32 {
        1.0 + (1.0 - self.body_progress) * TITLE_ADDITIONAL_SCALE
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn subtitle(&self) -> &str {
        &self.subtitle
    }
}

impl Default for PremiumTopBarEffects {
    fn default() -> Self {
        Self::new("Telegram Premium", "Unlock exclusive features and limits")
    }
}

impl Render for PremiumTopBarEffects {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h(px(self.height))
            .bg(rgb(0x182533))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(6.0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(20.0))
                    .text_color(rgb(0xF5F5F5))
                    .child(self.title.clone())
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0x707579))
                    .child(self.subtitle.clone())
            )
    }
}
