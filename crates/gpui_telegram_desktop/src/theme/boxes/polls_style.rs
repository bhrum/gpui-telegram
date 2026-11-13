//! Authentic GPUI theme stylesheet for Telegram Polls & Quizzes.
//! Upstream reference: `Telegram/SourceFiles/boxes/polls.style:L1-L308`.

use gpui::{px, rgb, Pixels, Rgba};

#[derive(Debug, Clone)]
pub struct PollsStyleSheet {
    pub question_font_size: Pixels,
    pub answer_font_size: Pixels,
    pub option_padding: (Pixels, Pixels, Pixels, Pixels),
    pub radio_size: Pixels,
    pub check_size: Pixels,
    pub progress_bar_height: Pixels,
    pub timer_badge_radius: Pixels,
    pub question_fg: Rgba,
    pub answer_fg: Rgba,
    pub percent_fg: Rgba,
    pub correct_answer_fg: Rgba,
    pub wrong_answer_fg: Rgba,
    pub progress_bar_bg: Rgba,
    pub progress_bar_fill: Rgba,
}

impl Default for PollsStyleSheet {
    fn default() -> Self {
        Self {
            question_font_size: px(15.0),
            answer_font_size: px(14.0),
            option_padding: (px(12.0), px(8.0), px(12.0), px(8.0)),
            radio_size: px(22.0),
            check_size: px(22.0),
            progress_bar_height: px(4.0),
            timer_badge_radius: px(4.0),
            question_fg: rgb(0x0F172A),
            answer_fg: rgb(0x334155),
            percent_fg: rgb(0x64748B),
            correct_answer_fg: rgb(0x16A34A),
            wrong_answer_fg: rgb(0xDC2626),
            progress_bar_bg: rgb(0xE2E8F0),
            progress_bar_fill: rgb(0x3B82F6),
        }
    }
}

impl PollsStyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bar_height(&self) -> Pixels {
        self.progress_bar_height
    }
}
