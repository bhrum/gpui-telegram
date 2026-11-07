//! Info Profile Top Bar component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_top_bar.cpp:L1-L4005`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_top_bar.h:L1-L397`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Collapsible profile header top bar widget.
/// Manages peer title, online status / member count, story indicators,
/// star gifts glares, and scroll interpolation.
#[derive(Debug, Clone)]
pub struct InfoProfileTopBarProfile {
    pub peer_id: i64,
    pub title: String,
    pub subtitle: String,
    pub collapse_ratio: f32,
    pub is_verified: bool,
    pub is_premium: bool,
    pub has_stories: bool,
    pub pinned_star_gifts_count: usize,
}

impl InfoProfileTopBarProfile {
    /// Upstream `kWaitBeforeGiftBadge = 1000ms` (`info_profile_top_bar.cpp:L154`).
    pub const WAIT_BEFORE_GIFT_BADGE_MS: u64 = 1000;
    /// Upstream `kGiftBadgeGlares = 3` (`info_profile_top_bar.cpp:L155`).
    pub const GIFT_BADGE_GLARES: usize = 3;
    /// Upstream `kMinPatternRadius = 8` (`info_profile_top_bar.cpp:L156`).
    pub const MIN_PATTERN_RADIUS: usize = 8;
    /// Upstream `kStoryOutlineFadeEnd = 0.4` (`info_profile_top_bar.cpp:L157`).
    pub const STORY_OUTLINE_FADE_END: f32 = 0.4;
    /// Upstream `kStoryOutlineFadeRange = 0.6` (`info_profile_top_bar.cpp:L158`).
    pub const STORY_OUTLINE_FADE_RANGE: f32 = 0.6;
    /// Upstream `kMinContrast = 5.5` (`info_profile_top_bar.cpp:L660`).
    pub const MIN_CONTRAST: f32 = 5.5;
    /// Upstream `kMaxPinnedToTopGifts = 6` (`info_profile_top_bar.cpp:L3502`).
    pub const MAX_PINNED_TO_TOP_GIFTS: usize = 6;

    pub fn new(peer_id: i64, title: impl Into<String>, subtitle: impl Into<String>) -> Self {
        Self {
            peer_id,
            title: title.into(),
            subtitle: subtitle.into(),
            collapse_ratio: 0.0,
            is_verified: false,
            is_premium: false,
            has_stories: false,
            pinned_star_gifts_count: 0,
        }
    }

    pub fn set_collapse_ratio(&mut self, ratio: f32) {
        self.collapse_ratio = ratio.clamp(0.0, 1.0);
    }

    pub fn set_verified(&mut self, verified: bool) {
        self.is_verified = verified;
    }

    pub fn set_premium(&mut self, premium: bool) {
        self.is_premium = premium;
    }

    pub fn set_has_stories(&mut self, stories: bool) {
        self.has_stories = stories;
    }

    pub fn set_pinned_star_gifts_count(&mut self, count: usize) {
        self.pinned_star_gifts_count = count.min(Self::MAX_PINNED_TO_TOP_GIFTS);
    }

    pub fn story_outline_opacity(&self) -> f32 {
        if self.collapse_ratio <= Self::STORY_OUTLINE_FADE_END {
            1.0
        } else {
            let progress = (self.collapse_ratio - Self::STORY_OUTLINE_FADE_END) / Self::STORY_OUTLINE_FADE_RANGE;
            (1.0 - progress).clamp(0.0, 1.0)
        }
    }
}

impl Default for InfoProfileTopBarProfile {
    fn default() -> Self {
        Self::new(0, "User", "offline")
    }
}

impl Render for InfoProfileTopBarProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_collapsed = self.collapse_ratio > 0.5;

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(56.0))
            .px(px(16.0))
            .bg(rgb(0xFFFFFF))
            .border_b_1()
            .border_color(rgb(0xE6E8EB))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(6.0))
                            .child(
                                div()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x222222))
                                    .child(self.title.clone())
                            )
                            .children(if self.is_verified {
                                Some(div().text_color(rgb(0x2AABEE)).child("✓"))
                            } else {
                                None
                            })
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x707579))
                            .child(self.subtitle.clone())
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded_md()
                            .bg(rgb(0xF1F2F4))
                            .text_xs()
                            .text_color(rgb(0x2AABEE))
                            .child(if is_collapsed { "Compact" } else { "Expanded" })
                    )
            )
    }
}
