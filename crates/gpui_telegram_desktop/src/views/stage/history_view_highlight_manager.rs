//! History View Highlight Manager & Jump-To Animation Orchestrator.
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/history/history_view_highlight_manager.h:L32-L75`
//! - Upstream: `Telegram/SourceFiles/history/history_view_highlight_manager.cpp:L42-L78`
//! - Upstream: `Telegram/SourceFiles/history/history_view_highlight_manager.cpp:L200-L318`
//! - Animation Constants:
//!   - `st::activeFadeInDuration = 200;` (ms, history_view_highlight_manager.cpp:L258)
//!   - `st::activeFadeOutDuration = 600;` (ms, history_view_highlight_manager.cpp:L252)
//!   - `st::fadeWrapDuration = 300;` (ms, history_view_highlight_manager.cpp:L287)

use std::collections::VecDeque;
use gpui::*;

pub const ACTIVE_FADE_IN_DURATION_MS: u64 = 200;
pub const ACTIVE_FADE_OUT_DURATION_MS: u64 = 600;
pub const FADE_WRAP_DURATION_MS: u64 = 300;

/// Quote or snippet coordinate within a target message bubble.
/// C++ citation: `HistoryView::SelectedQuote` (history_view_highlight_manager.h:L30).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SelectedQuote {
    pub message_id: u64,
    pub text_range: Option<(usize, usize)>,
    pub todo_list_id: Option<u64>,
    pub poll_option: Option<u32>,
    pub rich_page: Option<u32>,
}

/// Highlight animation visual parameters.
/// C++ citation: `Ui::ChatPaintHighlight` (history_view_highlight_manager.cpp:L63-L78, L213-L230).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChatPaintHighlight {
    pub opacity: f32,
    pub collapsion: f32,
}

impl Default for ChatPaintHighlight {
    fn default() -> Self {
        Self {
            opacity: 0.0,
            collapsion: 0.0,
        }
    }
}

/// Element highlighter managing queue and sequential animation playback.
/// C++ citation: `HistoryView::ElementHighlighter` (history_view_highlight_manager.h:L32-L49).
pub struct HistoryViewHighlightManager {
    queue: VecDeque<SelectedQuote>,
    current_highlight: Option<SelectedQuote>,
    is_animating: bool,
    animation_start_ms: u64,
    fading_out: bool,
    collapsing: bool,
    last_highlighted_id: Option<u64>,
}

impl HistoryViewHighlightManager {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            current_highlight: None,
            is_animating: false,
            animation_start_ms: 0,
            fading_out: false,
            collapsing: false,
            last_highlighted_id: None,
        }
    }

    /// Enqueues a message jump quote highlight.
    /// C++ citation: `history_view_highlight_manager.cpp:L28-L36`.
    pub fn enqueue(&mut self, quote: SelectedQuote, now_ms: u64) {
        self.queue.push_back(quote);
        self.check_next_highlight(now_ms);
    }

    /// Triggers immediate highlight, bypassing or interrupting queue.
    /// C++ citation: `history_view_highlight_manager.cpp:L38-L40`.
    pub fn highlight(&mut self, quote: SelectedQuote, now_ms: u64) {
        self.current_highlight = Some(quote);
        self.is_animating = true;
        self.animation_start_ms = now_ms;
        self.fading_out = false;
        self.collapsing = false;
    }

    /// Advances to next pending quote if current animation has finished.
    /// C++ citation: `history_view_highlight_manager.cpp:L42-L61`.
    pub fn check_next_highlight(&mut self, now_ms: u64) {
        if self.is_animating {
            return;
        }
        if let Some(next) = self.queue.pop_front() {
            self.highlight(next, now_ms);
        }
    }

    /// Computes current paint opacity and collapsion at timestamp `now_ms`.
    /// C++ citation: `history_view_highlight_manager.cpp:L213-L230`.
    pub fn paint_state(&mut self, now_ms: u64) -> Option<(u64, ChatPaintHighlight)> {
        let quote = self.current_highlight.as_ref()?;
        let elapsed = now_ms.saturating_sub(self.animation_start_ms);

        let total_cycle = ACTIVE_FADE_IN_DURATION_MS + FADE_WRAP_DURATION_MS + ACTIVE_FADE_OUT_DURATION_MS;
        if elapsed >= total_cycle {
            let finished_id = quote.message_id;
            self.last_highlighted_id = Some(finished_id);
            self.current_highlight = None;
            self.is_animating = false;
            self.check_next_highlight(now_ms);
            return None;
        }

        let opacity = if elapsed < ACTIVE_FADE_IN_DURATION_MS {
            (elapsed as f32) / (ACTIVE_FADE_IN_DURATION_MS as f32)
        } else if elapsed < ACTIVE_FADE_IN_DURATION_MS + FADE_WRAP_DURATION_MS {
            1.0
        } else {
            let fade_elapsed = elapsed - (ACTIVE_FADE_IN_DURATION_MS + FADE_WRAP_DURATION_MS);
            1.0 - (fade_elapsed as f32 / ACTIVE_FADE_OUT_DURATION_MS as f32)
        };

        let collapsion = if quote.text_range.is_some() && elapsed > ACTIVE_FADE_IN_DURATION_MS {
            1.0
        } else {
            0.0
        };

        Some((quote.message_id, ChatPaintHighlight {
            opacity: opacity.clamp(0.0, 1.0),
            collapsion,
        }))
    }

    pub fn current_message_id(&self) -> Option<u64> {
        self.current_highlight.as_ref().map(|q| q.message_id)
    }

    pub fn is_animating(&self) -> bool {
        self.is_animating
    }
}

impl Default for HistoryViewHighlightManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryViewHighlightManager {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().into_any_element()
    }
}
