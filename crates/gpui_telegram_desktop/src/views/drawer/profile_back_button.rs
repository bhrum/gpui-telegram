//! Profile Circular Back Button
//! Upstream: Telegram/SourceFiles/profile/profile_back_button.h, Telegram/SourceFiles/profile/profile_back_button.cpp

use gpui::*;

/// Top-bar circular back navigation button with title, subtext, and elision.
#[derive(Debug, Clone)]
pub struct ProfileBackButton {
    pub text: String,
    pub subtext: Option<String>,
    pub circle_radius: f32,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub hover_opacity: f32,
    pub opacity: f32,
    pub cached_width: f32,
    pub elision_width: f32,
}

impl Default for ProfileBackButton {
    fn default() -> Self {
        Self {
            text: String::new(),
            subtext: None,
            circle_radius: 18.0,
            is_hovered: false,
            is_pressed: false,
            hover_opacity: 0.0,
            opacity: 1.0,
            cached_width: -1.0,
            elision_width: 240.0,
        }
    }
}

impl ProfileBackButton {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_text(text: impl Into<String>) -> Self {
        let mut btn = Self::default();
        btn.text = text.into();
        btn
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.cached_width = -1.0;
    }

    pub fn set_subtext(&mut self, subtext: impl Into<String>) {
        self.subtext = Some(subtext.into());
        self.cached_width = -1.0;
    }

    pub fn clear_subtext(&mut self) {
        self.subtext = None;
        self.cached_width = -1.0;
    }

    pub fn set_circle_radius(&mut self, radius: f32) {
        self.circle_radius = radius.max(8.0);
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.is_hovered = hovered;
        self.hover_opacity = if hovered { 0.08 } else { 0.0 };
    }

    pub fn set_pressed(&mut self, pressed: bool) {
        self.is_pressed = pressed;
        if pressed {
            self.hover_opacity = 0.15;
        } else if self.is_hovered {
            self.hover_opacity = 0.08;
        } else {
            self.hover_opacity = 0.0;
        }
    }

    pub fn update_elision(&mut self, available_width: f32) {
        self.cached_width = available_width;
        let icon_reserved = self.circle_radius * 2.0 + 16.0;
        self.elision_width = (available_width - icon_reserved).max(40.0);
    }

    pub fn elided_text(&self, max_chars: usize) -> String {
        if self.text.chars().count() > max_chars && max_chars > 3 {
            let truncated: String = self.text.chars().take(max_chars - 3).collect();
            format!("{}...", truncated)
        } else {
            self.text.clone()
        }
    }

    pub fn height(&self) -> f32 {
        54.0
    }
}

impl Render for ProfileBackButton {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_pressed = self.is_pressed;
        let is_hovered = self.is_hovered;
        let circle_diameter = self.circle_radius * 2.0;

        div()
            .flex()
            .flex_row()
            .items_center()
            .h(px(self.height()))
            .px(px(8.0))
            .gap(px(10.0))
            .cursor_pointer()
            .opacity(self.opacity)
            .hover(|s| s.bg(rgb(0xF5F7FA)))
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                this.set_pressed(true);
                cx.notify();
            }))
            .on_mouse_up(MouseButton::Left, cx.listener(|this, _, _window, cx| {
                this.set_pressed(false);
                cx.notify();
            }))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(circle_diameter))
                    .h(px(circle_diameter))
                    .rounded_full()
                    .bg(if is_pressed {
                        rgb(0xDFE3E8)
                    } else if is_hovered {
                        rgb(0xEBEEF2)
                    } else {
                        rgb(0xF0F2F5)
                    })
                    .text_color(rgb(0x222222))
                    .font_weight(FontWeight::BOLD)
                    .child("‹")
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .max_w(px(self.elision_width))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x222222))
                            .overflow_hidden()
                            .child(self.text.clone())
                    )
                    .children(self.subtext.as_ref().map(|sub| {
                        div()
                            .text_xs()
                            .text_color(rgb(0x707579))
                            .overflow_hidden()
                            .child(sub.clone())
                    }))
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_back_button_interactions() {
        let mut btn = ProfileBackButton::new();
        assert_eq!(btn.circle_radius, 18.0);
        assert_eq!(btn.hover_opacity, 0.0);
        assert_eq!(btn.opacity, 1.0);

        btn.set_hovered(true);
        assert!(btn.is_hovered);
        assert_eq!(btn.hover_opacity, 0.08);

        btn.set_pressed(true);
        assert!(btn.is_pressed);
        assert_eq!(btn.hover_opacity, 0.15);

        btn.set_pressed(false);
        assert_eq!(btn.hover_opacity, 0.08);

        btn.set_hovered(false);
        assert_eq!(btn.hover_opacity, 0.0);
    }

    #[test]
    fn test_profile_back_button_text_and_elision() {
        let mut btn = ProfileBackButton::with_text("Telegram Channel Information");
        btn.set_subtext("1,450 subscribers");
        assert_eq!(btn.text, "Telegram Channel Information");
        assert_eq!(btn.subtext.as_deref(), Some("1,450 subscribers"));

        btn.update_elision(200.0);
        assert_eq!(btn.cached_width, 200.0);
        assert!(btn.elision_width > 0.0);

        let elided = btn.elided_text(15);
        assert_eq!(elided, "Telegram Cha...");

        btn.set_opacity(1.5);
        assert_eq!(btn.opacity, 1.0);
        btn.set_opacity(-0.5);
        assert_eq!(btn.opacity, 0.0);

        btn.clear_subtext();
        assert!(btn.subtext.is_none());
    }
}
