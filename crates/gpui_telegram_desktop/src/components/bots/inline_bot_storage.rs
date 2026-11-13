//! InlineBotStorage: 5MB client-side cloud key-value storage for Telegram Mini Apps.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/inline_bots/inline_bot_storage.h:L18-L50`
//! - `Telegram/SourceFiles/inline_bots/inline_bot_storage.cpp:L16-L86`
#![allow(dead_code)]

use gpui::*;
use std::collections::HashMap;

/// Maximum storage size per bot in bytes (upstream: `kMaxStorageSize = (5 << 20)` = 5MB).
pub const K_MAX_STORAGE_SIZE: usize = 5 * 1024 * 1024;

/// Compute 64-bit key hash for bot storage indexing (upstream: `KeyHash`, `inline_bot_storage.cpp:L20-L22`).
pub fn compute_storage_key_hash(key: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in key.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Key-value storage partition for a single bot.
#[derive(Debug, Clone, Default)]
pub struct BotStoragePartition {
    pub entries: HashMap<String, String>,
    pub total_size: usize,
}

/// Authentic InlineBotStorage component state.
pub struct InlineBotStorage {
    pub bot_partitions: HashMap<u64, BotStoragePartition>,
    pub updated_at: i64,
}

impl InlineBotStorage {
    pub fn new() -> Self {
        Self {
            bot_partitions: HashMap::new(),
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn write(&mut self, bot_id: u64, key: &str, value: Option<&str>) -> bool {
        if let Some(v) = value {
            if v.len() > K_MAX_STORAGE_SIZE {
                return false;
            }
        }

        let partition = self.bot_partitions.entry(bot_id).or_default();
        let old_size = partition
            .entries
            .get(key)
            .map(|old_v| key.len() + old_v.len())
            .unwrap_or(0);
        let new_size = value.map(|v| key.len() + v.len()).unwrap_or(0);

        let potential_total = partition.total_size - old_size + new_size;
        if potential_total > K_MAX_STORAGE_SIZE {
            return false;
        }

        partition.total_size = potential_total;
        if let Some(v) = value {
            partition.entries.insert(key.to_string(), v.to_string());
        } else {
            partition.entries.remove(key);
        }

        self.updated_at = chrono::Utc::now().timestamp();
        true
    }

    pub fn read(&self, bot_id: u64, key: &str) -> Option<&str> {
        self.bot_partitions
            .get(&bot_id)
            .and_then(|p| p.entries.get(key).map(|s| s.as_str()))
    }

    pub fn clear(&mut self, bot_id: u64) {
        self.bot_partitions.remove(&bot_id);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn total_size(&self, bot_id: u64) -> usize {
        self.bot_partitions
            .get(&bot_id)
            .map(|p| p.total_size)
            .unwrap_or(0)
    }
}

impl Default for InlineBotStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InlineBotStorage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.bot_partitions.len();

        div()
            .flex()
            .items_center()
            .p(px(8.0))
            .bg(rgb(0xFFFFFF))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x333333))
                    .child(format!("BotStorage: {} active bot partitions", count))
            )
    }
}
