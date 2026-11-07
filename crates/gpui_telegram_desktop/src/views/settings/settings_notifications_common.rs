//! Upstream Source: Telegram/SourceFiles/settings/settings_notifications_common.h
//! Subsystem: settings | Feature ID: TCG-N31

use gpui::*;

pub const K_MAX_NOTIFICATIONS_COUNT: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenCorner {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

impl ScreenCorner {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::TopLeft => "Top Left",
            Self::TopRight => "Top Right",
            Self::BottomRight => "Bottom Right",
            Self::BottomLeft => "Bottom Left",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SettingsNotificationsCommon {
    pub title: String,
    pub notifications_count: usize,
    pub corner: ScreenCorner,
    pub name_shown: bool,
    pub preview_shown: bool,
    pub sound_enabled: bool,
    pub is_visible: bool,
}

impl SettingsNotificationsCommon {
    pub fn new() -> Self {
        Self {
            title: "Notification Settings Common".into(),
            notifications_count: 3,
            corner: ScreenCorner::BottomRight,
            name_shown: true,
            preview_shown: true,
            sound_enabled: true,
            is_visible: true,
        }
    }

    pub fn clamped_sample_count(&self, count: usize) -> usize {
        count.clamp(1, K_MAX_NOTIFICATIONS_COUNT)
    }

    pub fn set_count(&mut self, count: usize) {
        self.notifications_count = count.clamp(1, K_MAX_NOTIFICATIONS_COUNT);
    }

    pub fn set_corner(&mut self, corner: ScreenCorner) {
        self.corner = corner;
    }

    pub fn toggle_name_shown(&mut self) {
        self.name_shown = !self.name_shown;
    }

    pub fn toggle_preview_shown(&mut self) {
        self.preview_shown = !self.preview_shown;
    }

    pub fn toggle_sound(&mut self) {
        self.sound_enabled = !self.sound_enabled;
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for SettingsNotificationsCommon {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for SettingsNotificationsCommon {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let current_corner = self.corner;
        let count = self.notifications_count;

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x17212b))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .pb(px(8.0))
                    .border_b_1()
                    .border_color(rgb(0x0e1621))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child("Desktop Notifications Placement"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8292a1))
                            .child(format!("Max {} alerts", count)),
                    ),
            )
            .child(
                // Screen corner selector mockup
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .py(px(12.0))
                    .child(
                        div()
                            .w(px(240.0))
                            .h(px(140.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x0e1621))
                            .border_2()
                            .border_color(rgb(0x2b3846))
                            .p(px(8.0))
                            .flex()
                            .flex_col()
                            .justify_between()
                            .child(
                                // Top corners row
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .child(
                                        div()
                                            .size(px(28.0))
                                            .rounded(px(4.0))
                                            .bg(if current_corner == ScreenCorner::TopLeft { rgb(0x5288c1) } else { rgb(0x232e3c) })
                                            .hover(|s| s.bg(rgb(0x2b3846)))
                                            .cursor_pointer()
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                this.set_corner(ScreenCorner::TopLeft);
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        div()
                                            .size(px(28.0))
                                            .rounded(px(4.0))
                                            .bg(if current_corner == ScreenCorner::TopRight { rgb(0x5288c1) } else { rgb(0x232e3c) })
                                            .hover(|s| s.bg(rgb(0x2b3846)))
                                            .cursor_pointer()
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                this.set_corner(ScreenCorner::TopRight);
                                                cx.notify();
                                            })),
                                    ),
                            )
                            .child(
                                // Bottom corners row
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .child(
                                        div()
                                            .size(px(28.0))
                                            .rounded(px(4.0))
                                            .bg(if current_corner == ScreenCorner::BottomLeft { rgb(0x5288c1) } else { rgb(0x232e3c) })
                                            .hover(|s| s.bg(rgb(0x2b3846)))
                                            .cursor_pointer()
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                this.set_corner(ScreenCorner::BottomLeft);
                                                cx.notify();
                                            })),
                                    )
                                    .child(
                                        div()
                                            .size(px(28.0))
                                            .rounded(px(4.0))
                                            .bg(if current_corner == ScreenCorner::BottomRight { rgb(0x5288c1) } else { rgb(0x232e3c) })
                                            .hover(|s| s.bg(rgb(0x2b3846)))
                                            .cursor_pointer()
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                                                this.set_corner(ScreenCorner::BottomRight);
                                                cx.notify();
                                            })),
                                    ),
                            ),
                    ),
            )
            .child(
                div()
                    .text_center()
                    .text_xs()
                    .text_color(rgb(0x8292a1))
                    .child(format!("Selected: {}", current_corner.display_name())),
            )
            .into_any_element()
    }
}
