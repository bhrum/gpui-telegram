//! Upstream Source: Telegram/SourceFiles/settings/settings_common.cpp, Telegram/SourceFiles/settings/settings_common.h
//! Subsystem: settings | Feature ID: TCG-N31 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui::prelude::FluentBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightShape {
    Rect,
    Ellipse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightPhase {
    Wait,
    FadeIn,
    Shown,
    FadeOut,
}

#[derive(Debug, Clone)]
pub struct HighlightArgs {
    pub shape: HighlightShape,
    pub radius: f32,
    pub opacity: f32,
    pub show_delay_ms: u64,
    pub show_duration_ms: u64,
    pub shown_duration_ms: u64,
    pub hide_duration_ms: u64,
}

impl Default for HighlightArgs {
    fn default() -> Self {
        Self {
            shape: HighlightShape::Rect,
            radius: 8.0,
            opacity: 0.4,
            show_delay_ms: 400,
            show_duration_ms: 600,
            shown_duration_ms: 400,
            hide_duration_ms: 600,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsIconType {
    Rounded,
    Round,
    Simple,
}

#[derive(Debug, Clone)]
pub struct SettingsIconDescriptor {
    pub icon_name: String,
    pub icon_type: SettingsIconType,
    pub bg_color: u32,
    pub new_badge: bool,
}

impl SettingsIconDescriptor {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            icon_name: name.into(),
            icon_type: SettingsIconType::Rounded,
            bg_color: 0x5288c1,
            new_badge: false,
        }
    }

    pub fn with_badge(mut self, badge: bool) -> Self {
        self.new_badge = badge;
        self
    }
}

pub fn search_words(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '@' && c != '#')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

pub fn matches_words(terms: &[String], query: &str) -> bool {
    let query_words = search_words(query);
    if query_words.is_empty() {
        return false;
    }
    query_words.iter().all(|q| {
        terms.iter().any(|word| word.starts_with(q) || word.contains(q))
    })
}

#[derive(Debug, Clone)]
pub struct SettingsCommon {
    pub title: String,
    pub is_visible: bool,
    pub highlight_phase: HighlightPhase,
    pub highlight_args: HighlightArgs,
    pub highlight_target: Option<String>,
}

impl SettingsCommon {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            is_visible: true,
            highlight_phase: HighlightPhase::Wait,
            highlight_args: HighlightArgs::default(),
            highlight_target: None,
        }
    }

    pub fn advance_phase(&mut self) {
        self.next_phase();
    }

    pub fn search_words(&self, text: &str) -> Vec<String> {
        search_words(text)
    }

    pub fn matches_words(&self, terms: &[String], target: &str) -> bool {
        matches_words(terms, target)
    }

    pub fn highlight(&mut self, target: impl Into<String>, args: Option<HighlightArgs>) {
        self.highlight_target = Some(target.into());
        if let Some(a) = args {
            self.highlight_args = a;
        }
        self.highlight_phase = HighlightPhase::FadeIn;
    }

    pub fn next_phase(&mut self) {
        self.highlight_phase = match self.highlight_phase {
            HighlightPhase::Wait => HighlightPhase::FadeIn,
            HighlightPhase::FadeIn => HighlightPhase::Shown,
            HighlightPhase::Shown => HighlightPhase::FadeOut,
            HighlightPhase::FadeOut => HighlightPhase::Wait,
        };
    }

    pub fn start_fade_out(&mut self) {
        self.highlight_phase = HighlightPhase::FadeOut;
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }
}

impl Default for SettingsCommon {
    fn default() -> Self {
        Self::new("Settings Common Infrastructure")
    }
}

impl Render for SettingsCommon {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible {
            return div().into_any_element();
        }

        let is_highlighted = matches!(self.highlight_phase, HighlightPhase::FadeIn | HighlightPhase::Shown);

        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(16.0))
            .bg(rgb(0x0e1621))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child(self.title.clone()),
                    )
                    .when(is_highlighted, |this| {
                        this.child(
                            div()
                                .px(px(6.0))
                                .py(px(2.0))
                                .rounded(px(4.0))
                                .bg(rgb(0x5288c1))
                                .text_xs()
                                .text_color(rgb(0xffffff))
                                .child("Highlighted"),
                        )
                    }),
            )
            .when_some(self.highlight_target.clone(), |this, target| {
                this.child(
                    div()
                        .mt(px(8.0))
                        .text_xs()
                        .text_color(rgb(0x8292a1))
                        .child(format!("Target: {} [{:?}]", target, self.highlight_phase)),
                )
            })
            .into_any_element()
    }
}
