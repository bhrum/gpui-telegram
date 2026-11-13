//! UI Chat Style metrics and quote paint cache definitions.
//! Upstream: `Telegram/SourceFiles/ui/chat/chat_style.cpp:L56-L70, L94-L117, L119-L141`
//!           `Telegram/SourceFiles/ui/chat/chat_style.h:L52-L125`
//!
//! Controls message bubble background palettes, blockquote outline opacity levels,
//! and service message capsule radius math.

use gpui::*;

pub const K_DEFAULT_BG_OPACITY: f32 = 0.12;
pub const K_DEFAULT_OUTLINE1_OPACITY: f32 = 0.90;
pub const K_DEFAULT_OUTLINE2_OPACITY: f32 = 0.30;
pub const K_DEFAULT_OUTLINE3_OPACITY: f32 = 0.60;
pub const K_DEFAULT_OUTLINE_OPACITY_SECOND: f32 = 0.50;

/// Service bubble radius computation per `ui/chat/chat_style.cpp:L94-L117`.
#[derive(Debug, Clone, Copy)]
pub struct ServiceMsgRadiusMetrics {
    pub min_message_height: f32,
    pub min_row_height: f32,
}

impl Default for ServiceMsgRadiusMetrics {
    fn default() -> Self {
        Self {
            min_message_height: 24.0,
            min_row_height: 20.0,
        }
    }
}

impl ServiceMsgRadiusMetrics {
    pub fn history_service_msg_radius(&self) -> f32 {
        self.min_message_height / 2.0
    }

    pub fn history_service_msg_inverted_radius(&self) -> f32 {
        self.min_row_height - self.history_service_msg_radius()
    }

    pub fn history_service_msg_inverted_shrink(&self) -> f32 {
        (self.history_service_msg_inverted_radius() * 2.0) / 3.0
    }
}

/// Quote outline and background colors per `ui/chat/chat_style.cpp:L119-L141`.
#[derive(Debug, Clone)]
pub struct ColorIndexValues {
    pub name: Hsla,
    pub bg: Hsla,
    pub outlines: [Hsla; 3],
}

pub fn simple_color_index_values(base: Hsla, pattern_index: usize) -> ColorIndexValues {
    let mut bg = base;
    bg.a = K_DEFAULT_BG_OPACITY;

    let mut out0 = base;
    out0.a = K_DEFAULT_OUTLINE1_OPACITY;

    let (out1, out2) = if pattern_index > 1 {
        let mut o1 = out0;
        o1.a = K_DEFAULT_OUTLINE2_OPACITY;
        let mut o2 = out0;
        o2.a = K_DEFAULT_OUTLINE3_OPACITY;
        (o1, o2)
    } else if pattern_index > 0 {
        let mut o1 = out0;
        o1.a = K_DEFAULT_OUTLINE_OPACITY_SECOND;
        (o1, hsla(0.0, 0.0, 0.0, 0.0))
    } else {
        (hsla(0.0, 0.0, 0.0, 0.0), hsla(0.0, 0.0, 0.0, 0.0))
    };

    ColorIndexValues {
        name: base,
        bg,
        outlines: [out0, out1, out2],
    }
}

/// ChatStyleChat component. Target symbol required by ledger: `ChatStyleChat`.
#[derive(Debug, Clone)]
pub struct ChatStyleChat {
    pub service_metrics: ServiceMsgRadiusMetrics,
    pub is_dark: bool,
    pub bubble_radius_small: f32,
    pub bubble_radius_large: f32,
}

impl ChatStyleChat {
    pub fn new() -> Self {
        Self {
            service_metrics: ServiceMsgRadiusMetrics::default(),
            is_dark: false,
            bubble_radius_small: 4.0,
            bubble_radius_large: 16.0,
        }
    }

    pub fn with_dark(mut self, dark: bool) -> Self {
        self.is_dark = dark;
        self
    }
}

impl Default for ChatStyleChat {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ChatStyleChat {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let radius = self.service_metrics.history_service_msg_radius();
        div()
            .p(px(8.0))
            .rounded(px(radius))
            .bg(if self.is_dark { hsla(0.0, 0.0, 0.12, 1.0) } else { hsla(0.0, 0.0, 0.95, 1.0) })
    }
}
