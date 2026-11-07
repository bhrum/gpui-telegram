//! Upstream C++ source: `Telegram/SourceFiles/ui/unread_counter_format.cpp:L1-L37` & `unread_counter_format.h:L1-L15`
//! Authentic GPUI implementation of unread counter formatting and metric abbreviator.
#![allow(dead_code)]

/// Formatter for dialog, tab, and folder unread badge counts.
/// C++ companion: `FormatUnreadCounter` (`unread_counter_format.cpp:L8-L37`).
#[derive(Debug, Clone, Copy, Default)]
pub struct UnreadCounterFormat;

impl UnreadCounterFormat {
    /// Formats an unread counter value into a human-readable badge label (`unread_counter_format.cpp:L8-L23`).
    pub fn format(unread_counter: i32, has_mention_or_reaction: bool, narrow: bool) -> String {
        if unread_counter <= 0 {
            return String::new();
        }
        if !narrow {
            return unread_counter.to_string();
        }
        if has_mention_or_reaction && unread_counter > 999 {
            return "99+".to_string();
        }
        if unread_counter > 999_999 {
            return "99999+".to_string();
        }
        unread_counter.to_string()
    }

    /// Formats large counters using metric suffixes (K, M, B) (`unread_counter_format.cpp:L25-L36`).
    pub fn format_short(unread_counter: i64) -> String {
        if unread_counter < 1_000 {
            unread_counter.to_string()
        } else if unread_counter < 1_000_000 {
            format!("{}K", unread_counter / 1_000)
        } else if unread_counter < 1_000_000_000 {
            format!("{}M", unread_counter / 1_000_000)
        } else {
            format!("{}B", unread_counter / 1_000_000_000)
        }
    }
}
