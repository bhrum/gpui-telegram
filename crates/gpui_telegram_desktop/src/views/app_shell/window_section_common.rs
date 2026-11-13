//! WindowSectionCommon: Transition animations, easing curves, slide directions, and gesture back handling
//! Upstream: Telegram/SourceFiles/window/window_section_common.h
//! Subsystem: window | Feature ID: TCG-N46 | Disposition: gpui_ui_1to1

use gpui::*;
use serde::{Deserialize, Serialize};

/// Result of a section action dispatch matching upstream Window::SectionActionResult
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SectionActionResult {
    Handle,   // Handle an action and stay in current section
    Fallback, // Ignore an action and fallback to HistoryWidget
    Ignore,   // Ignore an action and stay in current section
}

impl Default for SectionActionResult {
    fn default() -> Self {
        Self::Handle
    }
}

/// Slide direction for window section transitions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlideDirection {
    FromLeft,
    FromRight,
    FromTop,
    FromBottom,
}

impl Default for SlideDirection {
    fn default() -> Self {
        Self::FromRight
    }
}

/// Easing curve algorithms for fluid section transitions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EasingCurve {
    Linear,
    EaseOutCirc,
    EaseInOutQuad,
    EaseOutCubic,
}

impl Default for EasingCurve {
    fn default() -> Self {
        Self::EaseOutCirc
    }
}

impl EasingCurve {
    pub fn evaluate(&self, t: f32) -> f32 {
        let clamped = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => clamped,
            Self::EaseOutCirc => {
                let diff = clamped - 1.0;
                (1.0 - diff * diff).max(0.0).sqrt()
            }
            Self::EaseInOutQuad => {
                if clamped < 0.5 {
                    2.0 * clamped * clamped
                } else {
                    -1.0 + (4.0 - 2.0 * clamped) * clamped
                }
            }
            Self::EaseOutCubic => {
                let diff = clamped - 1.0;
                diff * diff * diff + 1.0
            }
        }
    }
}

/// Parameters configuring a section slide transition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SectionSlideParams {
    pub direction: SlideDirection,
    pub duration_ms: f32,
    pub easing: EasingCurve,
    pub with_fade: bool,
    pub gesture_driven: bool,
    pub gesture_progress: f32,
}

impl SectionSlideParams {
    pub fn new(direction: SlideDirection) -> Self {
        Self {
            direction,
            duration_ms: 250.0,
            easing: EasingCurve::EaseOutCirc,
            with_fade: true,
            gesture_driven: false,
            gesture_progress: 0.0,
        }
    }

    pub fn default_forward() -> Self {
        Self::new(SlideDirection::FromRight)
    }

    pub fn default_back() -> Self {
        Self::new(SlideDirection::FromLeft)
    }

    pub fn with_easing(mut self, easing: EasingCurve) -> Self {
        self.easing = easing;
        self
    }

    pub fn with_duration(mut self, duration_ms: f32) -> Self {
        self.duration_ms = duration_ms.max(50.0);
        self
    }
}

impl Default for SectionSlideParams {
    fn default() -> Self {
        Self::default_forward()
    }
}

/// Active transition state machine tracking progress and gesture interaction
#[derive(Debug, Clone, PartialEq)]
pub struct SectionTransition {
    pub params: SectionSlideParams,
    pub current_progress: f32,
    pub elapsed_ms: f32,
    pub is_animating: bool,
    pub gesture_back_active: bool,
}

impl SectionTransition {
    pub fn new(params: SectionSlideParams) -> Self {
        Self {
            params,
            current_progress: 0.0,
            elapsed_ms: 0.0,
            is_animating: true,
            gesture_back_active: false,
        }
    }

    pub fn start_gesture_back(&mut self) {
        self.gesture_back_active = true;
        self.is_animating = false;
        self.params.gesture_driven = true;
        self.params.direction = SlideDirection::FromLeft;
        self.current_progress = 0.0;
    }

    pub fn update_gesture_back(&mut self, delta_x: f32, container_width: f32) {
        if !self.gesture_back_active || container_width <= 0.0 {
            return;
        }
        let progress = (delta_x / container_width).clamp(0.0, 1.0);
        self.params.gesture_progress = progress;
        self.current_progress = progress;
    }

    pub fn commit_gesture_back(&mut self) -> bool {
        self.gesture_back_active = false;
        // Gesture commits if dragged past 35% threshold
        let should_commit = self.current_progress >= 0.35;
        if should_commit {
            self.current_progress = 1.0;
            self.is_animating = false;
        } else {
            self.current_progress = 0.0;
            self.is_animating = false;
        }
        should_commit
    }

    pub fn cancel_gesture_back(&mut self) {
        self.gesture_back_active = false;
        self.current_progress = 0.0;
        self.is_animating = false;
    }

    pub fn step(&mut self, dt_ms: f32) {
        if !self.is_animating || self.gesture_back_active {
            return;
        }
        self.elapsed_ms += dt_ms;
        let t = (self.elapsed_ms / self.params.duration_ms).min(1.0);
        self.current_progress = self.params.easing.evaluate(t);
        if t >= 1.0 {
            self.current_progress = 1.0;
            self.is_animating = false;
        }
    }

    pub fn is_finished(&self) -> bool {
        !self.is_animating && !self.gesture_back_active
    }

    pub fn offset_px(&self, container_width: f32) -> f32 {
        let remaining = 1.0 - self.current_progress;
        match self.params.direction {
            SlideDirection::FromRight => remaining * container_width,
            SlideDirection::FromLeft => -remaining * container_width,
            SlideDirection::FromTop => -remaining * container_width,
            SlideDirection::FromBottom => remaining * container_width,
        }
    }

    pub fn opacity(&self) -> f32 {
        if self.params.with_fade {
            self.current_progress.clamp(0.0, 1.0)
        } else {
            1.0
        }
    }
}

/// Authentic window section controller coordinating transitions, slide parameters, and back gestures
#[derive(Debug, Clone)]
pub struct WindowSectionCommon {
    pub active_transition: Option<SectionTransition>,
    pub last_action_result: SectionActionResult,
}

impl WindowSectionCommon {
    pub fn new() -> Self {
        Self {
            active_transition: None,
            last_action_result: SectionActionResult::Handle,
        }
    }

    pub fn start_transition(&mut self, params: SectionSlideParams) {
        self.active_transition = Some(SectionTransition::new(params));
    }

    pub fn handle_action(&mut self, result: SectionActionResult) {
        self.last_action_result = result;
    }

    pub fn is_in_transition(&self) -> bool {
        self.active_transition
            .as_ref()
            .map(|t| !t.is_finished())
            .unwrap_or(false)
    }

    pub fn gesture_back_active(&self) -> bool {
        self.active_transition
            .as_ref()
            .map(|t| t.gesture_back_active)
            .unwrap_or(false)
    }

    pub fn update_gesture(&mut self, delta_x: f32, width: f32) {
        if let Some(t) = self.active_transition.as_mut() {
            t.update_gesture_back(delta_x, width);
        }
    }

    pub fn commit_or_cancel_gesture(&mut self) -> bool {
        if let Some(t) = self.active_transition.as_mut() {
            t.commit_gesture_back()
        } else {
            false
        }
    }

    pub fn tick(&mut self, dt_ms: f32) {
        if let Some(t) = self.active_transition.as_mut() {
            t.step(dt_ms);
        }
    }
}

impl Default for WindowSectionCommon {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowSectionCommon {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_trans = self.is_in_transition();
        div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(8.0))
            .bg(rgb(0x17212B))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_sm()
                    .text_color(rgb(0xFFFFFF))
                    .child("Section Container")
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x82868A))
                    .child(format!(
                        "Transition: {}, Last Action: {:?}",
                        if is_trans { "Active" } else { "Idle" },
                        self.last_action_result
                    ))
            )
    }
}
