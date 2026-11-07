//! StickersLottie: Animated Lottie sticker playback and frame caching helpers.
//!
//! Upstream reference:
//! - `Telegram/SourceFiles/chat_helpers/stickers_lottie.h:L54-L75`
//! - `Telegram/SourceFiles/chat_helpers/stickers_lottie.cpp:L30-L50`
#![allow(dead_code)]

use gpui::*;
use std::collections::HashMap;

/// Area threshold above which Lottie animation frames are not cached to conserve memory
/// (upstream: `kDontCacheLottieAfterArea = 512 * 512`).
pub const K_DONT_CACHE_LOTTIE_AFTER_AREA: usize = 512 * 512;

/// Standard sticker display sizes for Lottie playback (upstream: `stickers_lottie.h:L54-L72`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum StickerLottieSize {
    #[default]
    MessageHistory = 0,
    StickerSet = 1,
    StickersPanel = 2,
    StickersFooter = 3,
    SetsListThumbnail = 4,
    InlineResults = 5,
    EmojiInteraction = 6,
    EmojiInteractionReserved1 = 7,
    EmojiInteractionReserved2 = 8,
    EmojiInteractionReserved3 = 9,
    EmojiInteractionReserved4 = 10,
    EmojiInteractionReserved5 = 11,
    EmojiInteractionReserved6 = 12,
    EmojiInteractionReserved7 = 13,
    ChatIntroHelloSticker = 14,
    StickerEmojiSize = 15,
    PinnedProfileUniqueGiftSize = 16,
}

impl StickerLottieSize {
    pub fn default_box(&self) -> (u32, u32) {
        match self {
            Self::MessageHistory => (256, 256),
            Self::StickerSet => (128, 128),
            Self::StickersPanel => (64, 64),
            Self::StickersFooter => (32, 32),
            Self::SetsListThumbnail => (48, 48),
            Self::InlineResults => (80, 80),
            Self::EmojiInteraction => (120, 120),
            Self::ChatIntroHelloSticker => (160, 160),
            Self::StickerEmojiSize => (20, 20),
            Self::PinnedProfileUniqueGiftSize => (96, 96),
            _ => (64, 64),
        }
    }
}

/// Compute Lottie cache key bit-shift for multi-size caching (upstream: `stickers_lottie.cpp:L42-L44`).
pub fn lottie_cache_key_shift(replacements_tag: u8, size_tag: StickerLottieSize) -> u8 {
    ((replacements_tag << 4) & 0xF0) | ((size_tag as u8) & 0x0F)
}

/// Calculate a 64-bit identifier for local sticker assets.
pub fn local_sticker_id(name: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in format!("local_sticker:{}", name).as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Cached Lottie frame descriptor.
#[derive(Debug, Clone)]
pub struct LottieFrame {
    pub frame_index: usize,
    pub width: u32,
    pub height: u32,
    pub rgba_data: Vec<u8>,
}

/// Authentic StickersLottie component state.
pub struct StickersLottie {
    pub size_tag: StickerLottieSize,
    pub replacements_tag: u8,
    pub current_frame: usize,
    pub total_frames: usize,
    pub is_playing: bool,
    pub fps: u32,
    pub frame_cache: HashMap<u8, LottieFrame>,
    pub updated_at: i64,
}

impl StickersLottie {
    pub fn new(size_tag: StickerLottieSize) -> Self {
        Self {
            size_tag,
            replacements_tag: 0,
            current_frame: 0,
            total_frames: 60,
            is_playing: true,
            fps: 60,
            frame_cache: HashMap::new(),
            updated_at: chrono::Utc::now().timestamp(),
        }
    }

    pub fn key_shift(&self) -> u8 {
        lottie_cache_key_shift(self.replacements_tag, self.size_tag)
    }

    pub fn should_cache_box(width: u32, height: u32) -> bool {
        (width as usize) * (height as usize) <= K_DONT_CACHE_LOTTIE_AFTER_AREA
    }

    pub fn advance_frame(&mut self) -> usize {
        if self.total_frames > 0 {
            self.current_frame = (self.current_frame + 1) % self.total_frames;
        }
        self.updated_at = chrono::Utc::now().timestamp();
        self.current_frame
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

impl Default for StickersLottie {
    fn default() -> Self {
        Self::new(StickerLottieSize::StickersPanel)
    }
}

impl Render for StickersLottie {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (w, h) = self.size_tag.default_box();

        div()
            .w(px(w as f32))
            .h(px(h as f32))
            .rounded(px(4.0))
            .bg(rgb(0xFAFAFA))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(rgb(0x888888))
                    .child(format!("{}/{}", self.current_frame, self.total_frames))
            )
    }
}
