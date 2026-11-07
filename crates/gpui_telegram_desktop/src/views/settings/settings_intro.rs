//! Upstream Source: Telegram/SourceFiles/settings/settings_intro.cpp, Telegram/SourceFiles/settings/settings_intro.h
//! Subsystem: settings | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use gpui::*;

#[derive(Debug, Clone)]
pub struct SettingsIntroFeature {
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone)]
pub struct SettingsIntro {
    pub title: String,
    pub features: Vec<SettingsIntroFeature>,
    pub min_width_px: f32,
    pub is_visible: bool,
}

impl SettingsIntro {
    pub const MIN_SUPPORTED_WIDTH: f32 = 380.0;

    pub fn new() -> Self {
        Self {
            title: "Telegram Settings Overview".into(),
            features: vec![
                SettingsIntroFeature {
                    icon: "👤",
                    title: "Edit Profile",
                    description: "Change your name, username, bio, profile photo and peer colors.",
                },
                SettingsIntroFeature {
                    icon: "🔒",
                    title: "Privacy and Security",
                    description: "Control who can see your phone number, set up Two-Step Verification and passkeys.",
                },
                SettingsIntroFeature {
                    icon: "🔔",
                    title: "Notifications & Sounds",
                    description: "Customize notification alerts, badge counters, and screen corner placements.",
                },
                SettingsIntroFeature {
                    icon: "⭐",
                    title: "Telegram Stars & Premium",
                    description: "Unlock 4GB uploads, faster speeds, animated emoji statuses and support digital creators.",
                },
            ],
            min_width_px: Self::MIN_SUPPORTED_WIDTH,
            is_visible: true,
        }
    }

    pub fn minimal_supported_width() -> f32 {
        Self::MIN_SUPPORTED_WIDTH
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for SettingsIntro {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsIntro {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        div()
            .flex()
            .flex_col()
            .size_full()
            .min_w(px(self.min_width_px))
            .bg(rgb(0x0e1621))
            .child(
                // TopBar with title and close action
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(20.0))
                    .py(px(14.0))
                    .bg(rgb(0x17212b))
                    .border_b_1()
                    .border_color(rgb(0x0e1621))
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child(self.title.clone()),
                    )
                    .child(
                        div()
                            .size(px(24.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(12.0))
                            .hover(|s| s.bg(rgb(0x232e3c)))
                            .cursor_pointer()
                            .text_color(rgb(0x8292a1))
                            .child("✕"),
                    ),
            )
            .child(
                // Scrollable feature intro cards
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .p(px(20.0))
                    .children(self.features.iter().map(|f| {
                        div()
                            .flex()
                            .flex_row()
                            .items_start()
                            .gap(px(16.0))
                            .p(px(12.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x17212b))
                            .child(
                                div()
                                    .size(px(36.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded(px(8.0))
                                    .bg(rgb(0x2b3846))
                                    .text_lg()
                                    .child(f.icon),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(2.0))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0xffffff))
                                            .child(f.title),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x8292a1))
                                            .child(f.description),
                                    ),
                            )
                    })),
            )
            .into_any_element()
    }
}
