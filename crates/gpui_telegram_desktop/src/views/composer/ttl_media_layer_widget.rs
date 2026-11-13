//! TTL Self-Destructing Media Layer Overlay
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/chat_helpers/ttl_media_layer_widget.h:L1-L23`
//! - `Telegram/SourceFiles/chat_helpers/ttl_media_layer_widget.cpp:L1-L401`
//!
//! Subsystem: chat_helpers, Feature: TCG-N11

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TtlMediaType {
    Photo,
    Video,
    Voice,
}

/// Full-screen or modal layer for viewing self-destructing view-once media with timer countdown.
/// Target symbol in ledger: `TtlMediaLayerWidget`.
#[derive(Debug, Clone)]
pub struct TtlMediaLayerWidget {
    pub message_id: i64,
    pub media_type: TtlMediaType,
    pub ttl_seconds: u32,
    pub seconds_remaining: u32,
    pub is_opened: bool,
    pub is_expired: bool,
    pub screenshot_protection_enabled: bool,
}

impl Default for TtlMediaLayerWidget {
    fn default() -> Self {
        Self {
            message_id: 0,
            media_type: TtlMediaType::Photo,
            ttl_seconds: 30,
            seconds_remaining: 30,
            is_opened: false,
            is_expired: false,
            screenshot_protection_enabled: true,
        }
    }
}

impl TtlMediaLayerWidget {
    pub fn new(message_id: i64, media_type: TtlMediaType, ttl_seconds: u32) -> Self {
        Self {
            message_id,
            media_type,
            ttl_seconds,
            seconds_remaining: ttl_seconds,
            is_opened: false,
            is_expired: false,
            screenshot_protection_enabled: true,
        }
    }

    pub fn open(&mut self) {
        self.is_opened = true;
    }

    pub fn tick_second(&mut self) -> bool {
        if self.is_opened && !self.is_expired {
            if self.seconds_remaining > 0 {
                self.seconds_remaining -= 1;
            }
            if self.seconds_remaining == 0 {
                self.is_expired = true;
            }
        }
        self.is_expired
    }

    pub fn close(&mut self) {
        self.is_opened = false;
    }
}
