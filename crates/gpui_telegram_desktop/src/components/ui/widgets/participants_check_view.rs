//! Authentic GPUI Participants Check View Component with Rotating Expander Arrow.
//! Reference: Telegram/SourceFiles/ui/widgets/participants_check_view.cpp:L33-L60
//! Reference: Telegram/SourceFiles/ui/widgets/participants_check_view.h:L20-L64

#![allow(dead_code, unused_imports, unused_variables)]

/// Default height and toggle size constants.
/// Reference: `participants_check_view.cpp:L47-L51`
pub const MODERATE_BOX_EXPAND_HEIGHT: f32 = 28.0;
pub const MODERATE_BOX_EXPAND_INNER_SKIP: f32 = 4.0;
pub const MODERATE_BOX_EXPAND_TOGGLE_SIZE: f32 = 16.0;

/// Target symbol for expandable participant checkbox view with chevron indicator.
#[derive(Debug, Clone)]
pub struct ParticipantsCheckViewWidgets {
    pub text: String,
    pub is_checked: bool,
    pub is_expanded: bool,
    pub duration_ms: u64,
    pub width: f32,
    pub height: f32,
    pub arrow_rotation: f32,
}

impl Default for ParticipantsCheckViewWidgets {
    fn default() -> Self {
        Self::new("Participants", false)
    }
}

impl ParticipantsCheckViewWidgets {
    pub fn new(text: impl Into<String>, checked: bool) -> Self {
        let t = text.into();
        let size = Self::compute_size(&t);
        Self {
            text: t,
            is_checked: checked,
            is_expanded: false,
            duration_ms: 150,
            width: size.0,
            height: size.1,
            arrow_rotation: 0.0,
        }
    }

    /// Computes component dimensions: `height + skip * 4 + textWidth + toggleSize`.
    /// Reference: `participants_check_view.cpp:L47-L51`
    pub fn compute_size(text: &str) -> (f32, f32) {
        let text_w = text.len() as f32 * 8.0;
        let w = MODERATE_BOX_EXPAND_HEIGHT
            + (MODERATE_BOX_EXPAND_INNER_SKIP * 4.0)
            + text_w
            + MODERATE_BOX_EXPAND_TOGGLE_SIZE;
        (w, MODERATE_BOX_EXPAND_HEIGHT)
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        let size = Self::compute_size(&self.text);
        self.width = size.0;
        self.height = size.1;
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.is_checked = checked;
    }

    pub fn toggle_expanded(&mut self) {
        self.is_expanded = !self.is_expanded;
        self.arrow_rotation = if self.is_expanded { 180.0 } else { 0.0 };
    }
}
