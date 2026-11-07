//! SectionWidget: Generic container component for navigation stack view sections
//! Upstream: Telegram/SourceFiles/window/section_widget.h, section_widget.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct SectionWidget {
    pub section_id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub has_back_button: bool,
    pub is_focused: bool,
    pub scroll_top: f32,
}

impl SectionWidget {
    pub fn new(section_id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            section_id: section_id.into(),
            title: title.into(),
            subtitle: None,
            has_back_button: true,
            is_focused: false,
            scroll_top: 0.0,
        }
    }

    pub fn set_subtitle(&mut self, subtitle: Option<String>) {
        self.subtitle = subtitle;
    }

    pub fn set_back_button(&mut self, enabled: bool) {
        self.has_back_button = enabled;
    }

    pub fn set_scroll_top(&mut self, offset: f32) {
        self.scroll_top = offset.max(0.0);
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }
}

impl Default for SectionWidget {
    fn default() -> Self {
        Self::new("default_section", "Section")
    }
}

impl Render for SectionWidget {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0E1621))
            .child(
                div()
                    .flex()
                    .items_center()
                    .h(px(44.0))
                    .px(px(16.0))
                    .bg(rgb(0x17212B))
                    .border_b_1()
                    .border_color(rgb(0x242F3D))
                    .children(self.has_back_button.then(|| {
                        div().mr(px(12.0)).text_color(rgb(0x5288C1)).child("←")
                    }))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(div().font_weight(FontWeight::BOLD).text_sm().text_color(rgb(0xFFFFFF)).child(self.title.clone()))
                            .children(self.subtitle.as_ref().map(|s| {
                                div().text_xs().text_color(rgb(0x82868A)).child(s.clone())
                            }))
                    )
            )
            .child(
                div()
                    .flex_1()
                    .p(px(16.0))
                    .child(format!("Section [{}] Content", self.section_id))
            )
    }
}
