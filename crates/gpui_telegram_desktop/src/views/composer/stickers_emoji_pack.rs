//! StickersEmojiPack: Large emoji rendering and emoji-sticker interaction packs.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/chat_helpers/stickers_emoji_pack.h:L53-L60`
//! - `Telegram/SourceFiles/chat_helpers/stickers_emoji_pack.cpp:L38-L90`
#![allow(dead_code)]

use gpui::*;
use std::collections::HashMap;

/// Standard restricted emoji pack set name (upstream: `kStandardEmojiSetName = "RestrictedEmoji"`).
pub const K_STANDARD_EMOJI_SET_NAME: &str = "RestrictedEmoji";

/// Pack refresh timeout in seconds (upstream: `kRefreshTimeout = 7200 * 1000ms`).
pub const K_REFRESH_TIMEOUT_SECS: u64 = 7200;

/// Number of cached emoji scale levels (upstream: `kEmojiCachesCount = 4`).
pub const K_EMOJI_CACHES_COUNT: usize = 4;

/// Number of premium cached emoji packs (upstream: `kPremiumCachesCount = 8`).
pub const K_PREMIUM_CACHES_COUNT: usize = 8;

/// Skin modifier color indices 1..5 (upstream: `stickers_emoji_pack.cpp:L62-L85`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SkinModifier {
    Color1 = 1,
    Color2 = 2,
    Color3 = 3,
    Color4 = 4,
    Color5 = 5,
}

/// Parse emoticon digit into 0-indexed number (upstream: `stickers_emoji_pack.cpp:L44-L54`).
pub fn index_from_emoticon(emoticon: &str) -> Option<usize> {
    let mut chars = emoticon.chars();
    let first = chars.next()?;
    if ('1'..='9').contains(&first) {
        Some((first as usize) - ('1' as usize))
    } else if first == '\u{1F61F}' {
        Some(9)
    } else {
        None
    }
}

/// Large emoji image descriptor.
#[derive(Debug, Clone)]
pub struct EmojiAsset {
    pub emoticon: String,
    pub document_id: u64,
    pub skin_modifier: Option<SkinModifier>,
    pub is_premium: bool,
}

/// Authentic StickersEmojiPack state.
pub struct StickersEmojiPack {
    pub pack_name: String,
    pub emoji_map: HashMap<String, EmojiAsset>,
    pub last_refreshed: i64,
    pub default_skin: Option<SkinModifier>,
    pub updated_at: i64,
}

impl StickersEmojiPack {
    pub fn new(pack_name: impl Into<String>) -> Self {
        Self {
            pack_name: pack_name.into(),
            emoji_map: HashMap::new(),
            last_refreshed: chrono::Utc::now().timestamp(),
            default_skin: None,
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn insert_emoji(&mut self, asset: EmojiAsset) {
        self.emoji_map.insert(asset.emoticon.clone(), asset);
        self.updated_at = chrono::Utc::now().timestamp();
    }

    pub fn lookup(&self, emoticon: &str) -> Option<&EmojiAsset> {
        self.emoji_map.get(emoticon)
    }

    pub fn needs_refresh(&self) -> bool {
        let now = chrono::Utc::now().timestamp();
        now - self.last_refreshed > (K_REFRESH_TIMEOUT_SECS as i64)
    }

    pub fn set_default_skin(&mut self, skin: Option<SkinModifier>) {
        self.default_skin = skin;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

impl Default for StickersEmojiPack {
    fn default() -> Self {
        Self::new(K_STANDARD_EMOJI_SET_NAME)
    }
}

impl Render for StickersEmojiPack {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.emoji_map.len();

        div()
            .flex()
            .flex_col()
            .p(px(8.0))
            .bg(rgb(0xFFFFFF))
            .child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .child(format!("{}: {} emojis", self.pack_name, count))
            )
    }
}
