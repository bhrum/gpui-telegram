//! Blind C++ Oracle Adversarial Stress Tests for Module 07
//! Stickers, Bot Keyboards & Inline Bots Subsystems.
//!
//! Direct Upstream References:
//! - `Telegram/SourceFiles/chat_helpers/stickers_lottie.h:L54-L75`
//! - `Telegram/SourceFiles/chat_helpers/stickers_lottie.cpp:L30-L50`
//! - `Telegram/SourceFiles/chat_helpers/bot_keyboard.h:L23-L69`
//! - `Telegram/SourceFiles/chat_helpers/bot_keyboard.cpp:L28-L450`
//! - `Telegram/SourceFiles/chat_helpers/stickers_gift_box_pack.h:L22-L60`
//! - `Telegram/SourceFiles/chat_helpers/stickers_gift_box_pack.cpp:L20-L75`
//! - `Telegram/SourceFiles/inline_bots/inline_results_widget.h:L58-L85`
//! - `Telegram/SourceFiles/inline_bots/inline_results_widget.cpp:L27-L133`
//! - `Telegram/SourceFiles/inline_bots/inline_bot_storage.h:L18-L50`
//! - `Telegram/SourceFiles/inline_bots/inline_bot_storage.cpp:L16-L86`
//!
//! Oracles Tested:
//! 1. Lottie Cache Key Shift bitwise parity: ((replacements_tag << 4) & 0xF0) | (size_tag & 0x0F) (10,000 trials vs blind C++ bitwise oracle matching stickers_lottie.cpp:L32-L44).
//! 2. Bot Reply Keyboard Layout & Geometry Calculator (100 complex multi-row layouts with flags SINGLE_USE, RESIZE, PERSISTENT, FORCE_REPLY).
//! 3. Stars-to-Months Gift Conversion & Dividers boundary math (stickers_gift_box_pack.cpp:L20-L42).
//! 4. Inline Results Height Formula clamp(SafeRound(emojiPanHeightRatio * bottom), minHeight, maxHeight) & 400ms debounce.
//! 5. Inline Bot Storage 5MB ceiling (kMaxStorageSize = (5 << 20)) and XXH64 hashing uniqueness.

use std::collections::HashSet;
use gpui_telegram_desktop::views::composer::stickers_lottie::{
    lottie_cache_key_shift, local_sticker_id, StickerLottieSize, K_DONT_CACHE_LOTTIE_AFTER_AREA,
};
use gpui_telegram_desktop::views::composer::bot_keyboard::{
    flags as kb_flags, BotKeyboard, BotKeyboardButton, BotKeyboardButtonKind,
};
use gpui_telegram_desktop::views::composer::stickers_gift_box_pack::{
    find_divider_index, months_for_stars, PREMIUM_DIVIDERS, TON_DIVIDERS,
};
use gpui_telegram_desktop::components::bots::inline_results_widget::{
    compute_results_height, K_EMOJI_PAN_HEIGHT_RATIO, K_INLINE_BOT_REQUEST_DELAY_MS,
    K_INLINE_RESULTS_MAX_HEIGHT, K_INLINE_RESULTS_MIN_HEIGHT,
};
use gpui_telegram_desktop::components::bots::inline_bot_storage::{
    compute_storage_key_hash, InlineBotStorage, K_MAX_STORAGE_SIZE,
};

/// Deterministic Pseudo-Random Number Generator (XorShift64)
struct XorShift64(u64);
impl XorShift64 {
    fn new(seed: u64) -> Self {
        Self(if seed == 0 { 0x853c49e6748fea9b } else { seed })
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn next_range(&mut self, min: u64, max: u64) -> u64 {
        if min >= max {
            return min;
        }
        min + (self.next_u64() % (max - min + 1))
    }
}

// ============================================================================
// Oracle 1: Lottie Cache Key Shift bitwise parity
// Upstream C++ reference: stickers_lottie.cpp:L42-L44
// uint8 LottieCacheKeyShift(uint8 replacementsTag, StickerLottieSize sizeTag) {
//     return ((replacementsTag << 4) & 0xF0) | (uint8(sizeTag) & 0x0F);
// }
// ============================================================================

/// Blind C++ implementation of stickers_lottie.cpp:L42-L44
fn blind_cpp_lottie_cache_key_shift(replacements_tag: u8, size_tag_u8: u8) -> u8 {
    let rep_shifted = (replacements_tag.wrapping_shl(4)) & 0xF0;
    let size_masked = size_tag_u8 & 0x0F;
    rep_shifted | size_masked
}

#[test]
fn test_oracle_1_lottie_cache_key_shift_bitwise_parity_10000_trials() {
    assert_eq!(K_DONT_CACHE_LOTTIE_AFTER_AREA, 512 * 512);

    let all_sizes = [
        StickerLottieSize::MessageHistory,
        StickerLottieSize::StickerSet,
        StickerLottieSize::StickersPanel,
        StickerLottieSize::StickersFooter,
        StickerLottieSize::SetsListThumbnail,
        StickerLottieSize::InlineResults,
        StickerLottieSize::EmojiInteraction,
        StickerLottieSize::EmojiInteractionReserved1,
        StickerLottieSize::EmojiInteractionReserved2,
        StickerLottieSize::EmojiInteractionReserved3,
        StickerLottieSize::EmojiInteractionReserved4,
        StickerLottieSize::EmojiInteractionReserved5,
        StickerLottieSize::EmojiInteractionReserved6,
        StickerLottieSize::EmojiInteractionReserved7,
        StickerLottieSize::ChatIntroHelloSticker,
        StickerLottieSize::StickerEmojiSize,
        StickerLottieSize::PinnedProfileUniqueGiftSize,
    ];

    let mut rng = XorShift64::new(0xDEAD_BEEF_CAFE_0001);

    for trial in 0..10_000 {
        // Random 8-bit replacementsTag (0..=255)
        let replacements_tag = (rng.next_u64() & 0xFF) as u8;
        // Random size from enum or raw u8
        let size_idx = rng.next_range(0, (all_sizes.len() - 1) as u64) as usize;
        let size_tag = all_sizes[size_idx];
        let size_tag_u8 = size_tag as u8;

        let rust_actual = lottie_cache_key_shift(replacements_tag, size_tag);
        let blind_oracle = blind_cpp_lottie_cache_key_shift(replacements_tag, size_tag_u8);

        assert_eq!(
            rust_actual, blind_oracle,
            "Bitwise mismatch at trial {trial}: rep=0x{replacements_tag:02X}, size={size_tag_u8}"
        );

        // Invariant: Upper 4 bits must match replacements_tag upper/lower 4 bits, lower 4 bits match size_tag & 0xF
        assert_eq!(rust_actual & 0x0F, size_tag_u8 & 0x0F);
        assert_eq!((rust_actual >> 4) & 0x0F, (replacements_tag << 4 >> 4) & 0x0F);
    }

    // Verify 64-bit sticker ID uniqueness and determinism
    let mut seen_ids = HashSet::new();
    for i in 0..1000 {
        let name = format!("lottie_anim_{}", i);
        let id = local_sticker_id(&name);
        assert!(seen_ids.insert(id), "Local sticker ID collision on index {i}");
        // Determinism
        assert_eq!(id, local_sticker_id(&name));
    }
}

// ============================================================================
// Oracle 2: Bot Reply Keyboard Layout & Geometry Calculator
// Upstream reference: bot_keyboard.cpp:L28-L450
// Flags: SINGLE_USE (0x01), RESIZE (0x02), PERSISTENT (0x04), FORCE_REPLY (0x08)
// Natural height: row_height * len + spacing * (len - 1)
// Calculated height: padding (8.0) + natural_height, min/max clamped by flags
// ============================================================================

/// Blind C++ Oracle for Bot Keyboard geometry and flag resolution
struct BlindCppBotKeyboardOracle {
    single_use: bool,
    persistent: bool,
    force_reply: bool,
    maximize_size: bool,
    row_count: usize,
    max_outer_height: f32,
}

impl BlindCppBotKeyboardOracle {
    fn new(flags_mask: u32, row_count: usize, max_outer_height: f32) -> Self {
        Self {
            single_use: (flags_mask & 0x01) != 0,
            maximize_size: (flags_mask & 0x02) == 0, // RESIZE flag clears maximize_size
            persistent: (flags_mask & 0x04) != 0,
            force_reply: (flags_mask & 0x08) != 0,
            row_count,
            max_outer_height,
        }
    }

    fn natural_height(&self) -> f32 {
        const ROW_HEIGHT: f32 = 36.0;
        const SPACING: f32 = 6.0;
        if self.row_count == 0 {
            0.0
        } else {
            (self.row_count as f32) * ROW_HEIGHT + ((self.row_count - 1) as f32) * SPACING
        }
    }

    fn calculated_height(&self) -> f32 {
        const PADDING: f32 = 8.0;
        let h = PADDING + self.natural_height();
        if self.maximize_size {
            h.max(self.max_outer_height)
        } else {
            h.min(self.max_outer_height)
        }
    }
}

#[test]
fn test_oracle_2_bot_reply_keyboard_geometry_and_flags_100_layouts() {
    let mut rng = XorShift64::new(0xFEED_FACE_CAFE_0002);

    for layout_idx in 0..100 {
        let num_rows = rng.next_range(0, 15) as usize;
        let mut rows = Vec::with_capacity(num_rows);

        for r in 0..num_rows {
            let num_cols = rng.next_range(1, 8) as usize;
            let mut row = Vec::with_capacity(num_cols);
            for c in 0..num_cols {
                let kind = match rng.next_range(0, 4) {
                    0 => BotKeyboardButtonKind::Default,
                    1 => BotKeyboardButtonKind::Url(format!("https://t.me/cmd_{r}_{c}")),
                    2 => BotKeyboardButtonKind::Callback(vec![r as u8, c as u8]),
                    _ => BotKeyboardButtonKind::WebApp(format!("https://app.t.me/{r}")),
                };
                row.push(BotKeyboardButton::new(format!("Btn {r}:{c}"), kind));
            }
            rows.push(row);
        }

        // Test flag combinations covering all 16 bit permutations
        let flags_mask = (rng.next_u64() & 0x0F) as u32;
        let max_outer_h = (rng.next_range(150, 400) as f32);

        let mut kb = BotKeyboard::new();
        kb.max_outer_height = max_outer_h;
        kb.set_markup(rows, flags_mask, Some(format!("Placeholder {layout_idx}")));

        let oracle = BlindCppBotKeyboardOracle::new(flags_mask, num_rows, max_outer_h);

        // Parity assertions
        assert_eq!(kb.single_use, oracle.single_use, "single_use mismatch at layout {layout_idx}");
        assert_eq!(kb.persistent, oracle.persistent, "persistent mismatch at layout {layout_idx}");
        assert_eq!(kb.force_reply, oracle.force_reply, "force_reply mismatch at layout {layout_idx}");
        assert_eq!(kb.maximize_size, oracle.maximize_size, "maximize_size mismatch at layout {layout_idx}");

        assert_eq!(kb.has_markup(), num_rows > 0);

        let nat_h = kb.natural_height();
        let oracle_nat_h = oracle.natural_height();
        assert!(
            (nat_h - oracle_nat_h).abs() < 1e-4,
            "natural_height mismatch at layout {layout_idx}: actual={nat_h}, oracle={oracle_nat_h}"
        );

        let calc_h = kb.calculated_height();
        let oracle_calc_h = oracle.calculated_height();
        assert!(
            (calc_h - oracle_calc_h).abs() < 1e-4,
            "calculated_height mismatch at layout {layout_idx}: actual={calc_h}, oracle={oracle_calc_h}"
        );

        // Invariant bounds
        if !kb.maximize_size && num_rows > 0 {
            assert!(calc_h <= max_outer_h, "Non-maximized keyboard exceeded max_outer_height");
        }
        if kb.maximize_size && num_rows > 0 {
            assert!(calc_h >= max_outer_h, "Maximized keyboard did not reach max_outer_height");
        }
    }
}

// ============================================================================
// Oracle 3: Stars-to-Months Gift Conversion & Dividers boundary math
// Upstream reference: stickers_gift_box_pack.cpp:L20-L75
// Months for stars:
// stars <= 1000 -> 3 months
// 1000 < stars < 2500 -> 6 months
// stars >= 2500 -> 12 months
// Dividers: PREMIUM_DIVIDERS = [1, 3, 6, 12, 24], TON_DIVIDERS = [0, 10, 50]
// Lower bound nearest / exact matching
// ============================================================================

fn blind_cpp_months_for_stars(stars: i32) -> i32 {
    if stars <= 1000 {
        3
    } else if stars < 2500 {
        6
    } else {
        12
    }
}

fn blind_cpp_find_divider(dividers: &[i32], target: i32, exact: bool) -> usize {
    if dividers.is_empty() {
        return 0;
    }
    // std::lower_bound
    let mut it = 0;
    while it < dividers.len() && dividers[it] < target {
        it += 1;
    }
    if it == 0 {
        return 0;
    }
    if it == dividers.len() {
        return dividers.len() - 1;
    }
    if exact {
        // shift = (*it > target) ? 1 : 0
        // it - shift
        let shift = if dividers[it] > target { 1 } else { 0 };
        return it.saturating_sub(shift);
    } else {
        // shift = (std::abs(target - (*(it - 1))) < std::abs(target - (*it))) ? -1 : 0
        // in C++ it - shift, so if dist(prev) < dist(next), it - (-1) wait:
        // C++: it - shift where shift is -1 => it - (-1) = it + 1?
        // Let's re-verify C++:
        // const auto shift = exact ? ... : (std::abs(divider - (*(it - 1))) < std::abs(divider - (*it))) ? -1 : 0;
        // const auto index = int(std::distance(begin(pack.dividers), it - shift));
        // Note: in C++ it is an iterator. When prev distance is smaller, it moves to it - 1.
        let prev_dist = (target - dividers[it - 1]).abs();
        let curr_dist = (dividers[it] - target).abs();
        if prev_dist < curr_dist {
            it - 1
        } else {
            it
        }
    }
}

#[test]
fn test_oracle_3_stars_to_months_and_dividers_boundary_math() {
    // 1. Stars to months boundary checks
    let test_stars = [
        (-100, 3), (0, 3), (1, 3), (500, 3), (999, 3), (1000, 3),
        (1001, 6), (1500, 6), (2499, 6),
        (2500, 12), (2501, 12), (5000, 12), (100000, 12),
    ];
    for (stars, expected_months) in test_stars {
        let actual = months_for_stars(stars);
        let oracle = blind_cpp_months_for_stars(stars);
        assert_eq!(actual, expected_months, "Failed for stars={stars}");
        assert_eq!(actual, oracle, "Oracle mismatch for stars={stars}");
    }

    // 2. Dividers boundary tests (PREMIUM_DIVIDERS: [1, 3, 6, 12, 24])
    assert_eq!(PREMIUM_DIVIDERS, &[1, 3, 6, 12, 24]);
    assert_eq!(TON_DIVIDERS, &[0, 10, 50]);

    // Exact matches
    for &d in PREMIUM_DIVIDERS {
        assert_eq!(find_divider_index(PREMIUM_DIVIDERS, d, true), blind_cpp_find_divider(PREMIUM_DIVIDERS, d, true));
        assert_eq!(find_divider_index(PREMIUM_DIVIDERS, d, false), blind_cpp_find_divider(PREMIUM_DIVIDERS, d, false));
    }

    // Stress testing 500 arbitrary query values
    let mut rng = XorShift64::new(0x3333_4444_5555_6666);
    for _ in 0..500 {
        let q = rng.next_range(0, 100) as i32;
        let exact = (rng.next_u64() % 2) == 1;

        let actual_prem = find_divider_index(PREMIUM_DIVIDERS, q, exact);
        let oracle_prem = blind_cpp_find_divider(PREMIUM_DIVIDERS, q, exact);
        assert_eq!(
            actual_prem, oracle_prem,
            "Premium divider search mismatch for q={q}, exact={exact}"
        );

        let actual_ton = find_divider_index(TON_DIVIDERS, q, exact);
        let oracle_ton = blind_cpp_find_divider(TON_DIVIDERS, q, exact);
        assert_eq!(
            actual_ton, oracle_ton,
            "TON divider search mismatch for q={q}, exact={exact}"
        );
    }
}

// ============================================================================
// Oracle 4: Inline Results Height Formula & 400ms Debounce
// Upstream reference: inline_results_widget.cpp:L27-L133
// Height formula: clamp(SafeRound(emojiPanHeightRatio * bottom), minHeight, maxHeight)
// clamped to bottomSpace and maxContentHeight
// Debounce: 400ms
// ============================================================================

fn blind_cpp_compute_results_height(bottom_space: f32, max_content_height: f32) -> f32 {
    let raw = (bottom_space * 0.55).round();
    let clamped_min_max = if raw < 120.0 {
        120.0
    } else if raw > 360.0 {
        360.0
    } else {
        raw
    };
    clamped_min_max.min(bottom_space).min(max_content_height)
}

#[test]
fn test_oracle_4_inline_results_height_and_debounce_math() {
    assert_eq!(K_INLINE_BOT_REQUEST_DELAY_MS, 400);
    assert_eq!(K_INLINE_RESULTS_MIN_HEIGHT, 120.0);
    assert_eq!(K_INLINE_RESULTS_MAX_HEIGHT, 360.0);
    assert_eq!(K_EMOJI_PAN_HEIGHT_RATIO, 0.55);

    let mut rng = XorShift64::new(0x7777_8888_9999_AAAA);

    // 1,000 randomized viewport bottom space & max content height scenarios
    for i in 0..1000 {
        let bottom_space = (rng.next_range(50, 1200) as f32) * 0.5; // 25.0 to 600.0
        let max_content_height = (rng.next_range(50, 1000) as f32) * 0.5; // 25.0 to 500.0

        let actual = compute_results_height(bottom_space, max_content_height);
        let oracle = blind_cpp_compute_results_height(bottom_space, max_content_height);

        assert!(
            (actual - oracle).abs() < 1e-4,
            "Height formula mismatch at scenario {i}: bottom={bottom_space}, max_content={max_content_height}: actual={actual}, oracle={oracle}"
        );

        // Core upstream constraints:
        assert!(actual <= bottom_space, "Height cannot exceed bottom_space");
        assert!(actual <= max_content_height, "Height cannot exceed max_content_height");
        if bottom_space >= K_INLINE_RESULTS_MAX_HEIGHT && max_content_height >= K_INLINE_RESULTS_MAX_HEIGHT {
            let wanted = (bottom_space * 0.55).round();
            if wanted >= K_INLINE_RESULTS_MAX_HEIGHT {
                assert_eq!(actual, K_INLINE_RESULTS_MAX_HEIGHT);
            }
        }
    }
}

// ============================================================================
// Oracle 5: Inline Bot Storage 5MB ceiling and key hash uniqueness
// Upstream reference: inline_bot_storage.cpp:L16-L86
// kMaxStorageSize = (5 << 20) = 5,242,880 bytes
// KeyHash uniqueness across various alphanumeric keys
// ============================================================================

#[test]
fn test_oracle_5_inline_bot_storage_5mb_limit_and_hash_uniqueness() {
    // 5MB ceiling: (5 << 20)
    assert_eq!(K_MAX_STORAGE_SIZE, 5 << 20);
    assert_eq!(K_MAX_STORAGE_SIZE, 5 * 1024 * 1024);

    let mut storage = InlineBotStorage::new();
    let bot_id = 99999;

    // 1. Storage boundary rejection: payload exceeding 5MB
    let oversized = "x".repeat(K_MAX_STORAGE_SIZE + 1);
    let ok = storage.write(bot_id, "big_key", Some(&oversized));
    assert!(!ok, "Write exceeding 5MB ceiling should be rejected");

    // 2. Exact 5MB boundary: key + value <= 5MB
    let key = "k";
    let valid_val = "v".repeat(K_MAX_STORAGE_SIZE - key.len());
    let ok = storage.write(bot_id, key, Some(&valid_val));
    assert!(ok, "Write fitting within 5MB should succeed");
    assert_eq!(storage.total_size(bot_id), K_MAX_STORAGE_SIZE);

    // Any additional write without replacing must fail due to ceiling
    let extra_ok = storage.write(bot_id, "k2", Some("overflow"));
    assert!(!extra_ok, "Additional write exceeding 5MB partition size must fail");

    // Deleting / writing None frees space
    let del_ok = storage.write(bot_id, key, None);
    assert!(del_ok, "Deleting key should succeed");
    assert_eq!(storage.total_size(bot_id), 0);

    // 3. Hash uniqueness across 2,000 distinct generated keys
    let mut hash_set = HashSet::new();
    let mut rng = XorShift64::new(0xAAAA_BBBB_CCCC_DDDD);

    for i in 0..2000 {
        let key_len = rng.next_range(5, 32) as usize;
        let s: String = (0..key_len)
            .map(|_| {
                let byte = (rng.next_u64() % 62) as u8;
                match byte {
                    0..=25 => (b'a' + byte) as char,
                    26..=51 => (b'A' + (byte - 26)) as char,
                    _ => (b'0' + (byte - 52)) as char,
                }
            })
            .collect();
        let key = format!("bot_pref_{}_{}", i, s);
        let h = compute_storage_key_hash(&key);
        assert!(
            hash_set.insert(h),
            "Hash collision detected on key '{key}' at iteration {i}"
        );
    }
}
