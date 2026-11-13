//! History View Top Toast & Tooltip System.
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/history/history_view_top_toast.h:L31-L63`
//! - Upstream: `Telegram/SourceFiles/history/history_view_top_toast.cpp:L23-L32`
//! - Anonymous Constants:
//!   - `kAnchoredTooltipDuration = 4 * 1000;` (4000ms duration, history_view_top_toast.cpp:L25)
//!   - `CountToastDuration`: reading speed clamp `(1000 * len / 14)` clamped to `[5000ms .. 8000ms]` (history_view_top_toast.cpp:L27-L32)

use gpui::*;

pub const ANCHORED_TOOLTIP_DURATION_MS: u64 = 4000;

/// Calculates toast duration in milliseconds based on reading speed (14 chars/sec).
/// C++ citation: `history_view_top_toast.cpp:L27-L32`.
pub fn count_toast_duration_ms(text_length: usize) -> u64 {
    let computed = (1000 * text_length as u64) / 14;
    computed.clamp(5000, 8000)
}

/// Floating banner toast anchored to the top of the history view.
/// C++ citation: `HistoryView::InfoTooltip` (history_view_top_toast.h:L31-L45).
#[derive(Debug, Clone)]
pub struct InfoTooltip {
    pub text: String,
    pub duration_ms: u64,
    pub is_visible: bool,
}

impl InfoTooltip {
    pub fn new(text: impl Into<String>) -> Self {
        let t = text.into();
        let duration = count_toast_duration_ms(t.len());
        Self {
            text: t,
            duration_ms: duration,
            is_visible: true,
        }
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }
}

/// Tooltip anchored to a specific message item or coordinate in the transcript.
/// C++ citation: `HistoryView::AnchoredTooltip` (history_view_top_toast.h:L47-L63).
#[derive(Debug, Clone)]
pub struct AnchoredTooltip {
    pub message_id: Option<u64>,
    pub text: String,
    pub anchor_y: f32,
    pub duration_ms: u64,
    pub is_visible: bool,
}

impl AnchoredTooltip {
    pub fn new(message_id: Option<u64>, text: impl Into<String>, anchor_y: f32) -> Self {
        Self {
            message_id,
            text: text.into(),
            anchor_y,
            duration_ms: ANCHORED_TOOLTIP_DURATION_MS,
            is_visible: true,
        }
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }
}

/// Top-level toast controller for the chat stage.
/// C++ equivalent: `HistoryViewTopToast` / `InfoTooltip` + `AnchoredTooltip` host.
pub struct HistoryViewTopToast {
    active_info: Option<InfoTooltip>,
    active_anchored: Option<AnchoredTooltip>,
}

impl HistoryViewTopToast {
    pub fn new() -> Self {
        Self {
            active_info: None,
            active_anchored: None,
        }
    }

    /// Shows a top info banner with auto-computed reading duration.
    pub fn show_info(&mut self, text: impl Into<String>) {
        self.active_info = Some(InfoTooltip::new(text));
    }

    /// Shows an anchored tooltip at a target message offset.
    pub fn show_anchored(&mut self, msg_id: Option<u64>, text: impl Into<String>, y: f32) {
        self.active_anchored = Some(AnchoredTooltip::new(msg_id, text, y));
    }

    pub fn hide_all(&mut self) {
        self.active_info = None;
        self.active_anchored = None;
    }

    pub fn active_info(&self) -> Option<&InfoTooltip> {
        self.active_info.as_ref()
    }

    pub fn active_anchored(&self) -> Option<&AnchoredTooltip> {
        self.active_anchored.as_ref()
    }
}

impl Default for HistoryViewTopToast {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewTopToast {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(ref info) = self.active_info {
            if info.is_visible {
                return div()
                    .absolute()
                    .top(px(12.0))
                    .left(px(24.0))
                    .right(px(24.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .px(px(16.0))
                            .py(px(8.0))
                            .rounded(px(20.0))
                            .bg(rgb(0x18181B))
                            .text_color(rgb(0xFFFFFF))
                            .text_size(px(13.0))
                            .shadow_lg()
                            .child(info.text.clone())
                    )
                    .into_any_element();
            }
        }

        div().into_any_element()
    }
}
