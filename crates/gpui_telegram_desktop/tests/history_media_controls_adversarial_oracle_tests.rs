//! Blind C++ Oracle Adversarial Stress Tests for Module 06 (`history`)
//! Media, Reactions & Controls Subsystems.
//!
//! Direct Upstream References:
//! - `Telegram/SourceFiles/history/view/media/history_view_poll.cpp:L25-L35, L50-L90, L144-L210`
//! - `Telegram/SourceFiles/history/view/controls/history_view_voice_record_bar.cpp:L39-L43, L60-L75, L2690-L2800`
//! - `Telegram/SourceFiles/history/view/reactions/history_view_reactions.cpp:L60-L100, L180-L207`
//! - `Telegram/SourceFiles/history/view/media/history_view_media_spoiler.cpp:L32-L75`
//! - `Telegram/SourceFiles/history/view/media/history_view_media_spoiler.h:L19-L37`
//!
//! Oracles Tested:
//! 1. Poll Fairness & Remainder Distribution Oracle (1,000 randomized vote distributions)
//! 2. IEEE 802.3 Bitwise CRC32 Poll Shuffle Oracle (polynomial 0xEDB88320, 10,000 checks)
//! 3. Voice Record Bar FSM & Gesture Clamping Oracle (0.35 lock, 0.8 cancel, 6,000s duration cap)
//! 4. Reactions Sorting Tie-Breakers Oracle (500 reaction pill ordering scenarios)
//! 5. 60-Frame Looping Dust Particle Animation Oracle (64 deterministic vectors, 60 frames, alpha curve)

use std::collections::HashSet;
use gpui_telegram_desktop::views::composer::controls::{
    VoiceRecordBar, VoiceRecordState, StopType,
    ANIMATION_THRESHOLD, AUDIO_VOICE_MAX_LENGTH_SECS, AUDIO_VOICE_UPDATE_VIEW_MS,
    LOCK_HEIGHT_PX, SLIDE_CANCEL_THRESHOLD_PX,
};
use gpui_telegram_desktop::views::stage::media::poll::{
    adjust_percent_count, count_nice_percent, hash_poll_shuffle_value, PercentCounterItem, PollType,
};
use gpui_telegram_desktop::views::stage::media::spoiler::{
    MediaSpoilerState, SPOILER_FRAME_COUNT, SPOILER_PARTICLE_COUNT,
};
use gpui_telegram_desktop::views::stage::reactions::button::{
    sort_reaction_pills, ReactionKey, ReactionPill,
};
use gpui_telegram_desktop::views::stage::view::controls::history_view_voice_record_bar::HistoryViewVoiceRecordBarViewControls;
use gpui_telegram_desktop::views::stage::view::media::history_view_poll::HistoryViewPollViewMedia;
use gpui_telegram_desktop::views::stage::view::media::history_view_media_spoiler::HistoryViewMediaSpoilerViewMedia;
use gpui_telegram_desktop::views::stage::view::reactions::history_view_reactions::HistoryViewReactionsViewReactions;

// Deterministic Pseudo-Random Number Generator (XorShift64)
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
// ORACLE 1: Poll Fairness & Remainder Distribution Oracle
// Upstream: `Telegram/SourceFiles/history/view/media/history_view_poll.cpp:L144-L200`
// ============================================================================

#[test]
fn test_oracle_1_poll_fairness_and_remainder_distribution_1000_trials() {
    let mut rng = XorShift64::new(0xABCD_1234_5678_9ABC);

    for trial in 0..1000 {
        let num_options = rng.next_range(2, 10) as usize;
        let mut votes = Vec::with_capacity(num_options);
        for _ in 0..num_options {
            votes.push(rng.next_range(0, 10_000) as i32);
        }
        let total: i32 = votes.iter().sum();
        if total == 0 {
            continue;
        }

        let percents = count_nice_percent(&votes, total);
        assert_eq!(percents.len(), num_options, "Trial {}: output length mismatch", trial);

        // Property 1: No negative adjustments
        // Each option's percent must be >= raw integer percent: (votes * 100) / total
        for (i, &v) in votes.iter().enumerate() {
            let base_percent = (v * 100) / total;
            assert!(
                percents[i] >= base_percent,
                "Trial {}: option {} received negative adjustment: {} < {}",
                trial, i, percents[i], base_percent
            );
            assert!(
                percents[i] <= base_percent + 1,
                "Trial {}: option {} increased by more than 1: {} > {}",
                trial, i, percents[i], base_percent + 1
            );
            if v == 0 {
                // Property 2: Zero votes must NEVER receive a remainder point
                assert_eq!(
                    percents[i], 0,
                    "Trial {}: option {} with 0 votes received non-zero percent {}",
                    trial, i, percents[i]
                );
            }
        }

        // Property 3: Remainder fairness
        // If two options have equal remainder and equal initial percent, they must receive the same adjustment
        for i in 0..num_options {
            let rem_i = (votes[i] * 100) - ((votes[i] * 100) / total) * total;
            let base_i = (votes[i] * 100) / total;
            for j in (i + 1)..num_options {
                let rem_j = (votes[j] * 100) - ((votes[j] * 100) / total) * total;
                let base_j = (votes[j] * 100) / total;

                if rem_i == rem_j && base_i == base_j {
                    assert_eq!(
                        percents[i] - base_i,
                        percents[j] - base_j,
                        "Trial {}: fairness violation between tied options {} and {}",
                        trial, i, j
                    );
                }

                // If rem_i > rem_j, option j cannot get +1 unless option i also got +1
                if rem_i > rem_j && rem_j > 0 {
                    let bump_i = percents[i] - base_i;
                    let bump_j = percents[j] - base_j;
                    if bump_j == 1 {
                        assert_eq!(
                            bump_i, 1,
                            "Trial {}: option {} with lower remainder {} was promoted ahead of option {} with higher remainder {}",
                            trial, j, rem_j, i, rem_i
                        );
                    }
                }
            }
        }

        // Property 4: Sum is always 100% or 99% (when an unbroken tie prevents fair distribution)
        let sum: i32 = percents.iter().sum();
        assert!(
            sum == 99 || sum == 100,
            "Trial {}: sum of percents is invalid: {} (must be 99 or 100)",
            trial, sum
        );
    }
}

#[test]
fn test_oracle_1_adjust_percent_count_strictly_sums_to_100_percent() {
    // Stress test `adjust_percent_count` directly with 1,000 synthetic configurations
    // where remainder groups can be satisfied
    let mut rng = XorShift64::new(0xFEED_FACE_CAFE_BABE);

    for trial in 0..1000 {
        let count = rng.next_range(2, 8) as usize;
        let mut items = Vec::with_capacity(count);
        let mut base_sum = 0;

        // Generate distinct non-zero remainders so `equal == 1 <= left` always holds
        for idx in 0..count {
            let pct = rng.next_range(5, 20) as i32;
            base_sum += pct;
            items.push(PercentCounterItem {
                index: idx,
                percent: pct,
                remainder: (count - idx) as i32 * 10,
            });
        }

        let left = (100 - base_sum).clamp(0, count as i32);
        let initial_sum: i32 = items.iter().map(|it| it.percent).sum();

        adjust_percent_count(&mut items, left);

        let final_sum: i32 = items.iter().map(|it| it.percent).sum();
        assert_eq!(
            final_sum,
            initial_sum + left,
            "Trial {}: adjust_percent_count did not distribute exact left {} (initial={}, final={})",
            trial, left, initial_sum, final_sum
        );
    }

    // Verify stage view integration
    let mut poll_view = HistoryViewPollViewMedia::new(101, "Favorite Language?", PollType::Regular);
    poll_view.add_answer(1, "Rust", 70, true);
    poll_view.add_answer(2, "C++", 30, false);
    assert_eq!(poll_view.answers[0].percent, 70);
    assert_eq!(poll_view.answers[1].percent, 30);
    poll_view.vote(1);
    assert_eq!(poll_view.total_votes, 101);
}

// ============================================================================
// ORACLE 2: IEEE 802.3 Bitwise CRC32 Poll Shuffle Oracle
// Upstream: `Telegram/SourceFiles/history/view/media/history_view_poll.cpp:L202-L210`
// Polynomial: `0xEDB88320`
// ============================================================================

/// Independent reference bitwise implementation of IEEE 802.3 CRC32
fn reference_ieee_802_3_crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

#[test]
fn test_oracle_2_crc32_known_vector_and_polynomial_match() {
    // 1. Verify against known IEEE 802.3 test vector
    // Standard test vector: "123456Option A789012"
    let user_id = 123456u64;
    let poll_id = 789012u64;
    let option = b"Option A";

    let hash = hash_poll_shuffle_value(user_id, poll_id, option);
    // Standard IEEE 802.3 CRC32 for b"123456Option A789012" is 0x46D8BCE0
    assert_eq!(hash, 0x46D8_BCE0, "Known vector CRC32 mismatch!");

    // 2. Cross-verify against independent reference across 1,000 randomized inputs
    let mut rng = XorShift64::new(0x1337_C0DE_9999_7777);
    for _ in 0..1000 {
        let uid = rng.next_u64();
        let pid = rng.next_u64();
        let opt_len = rng.next_range(1, 32) as usize;
        let opt: Vec<u8> = (0..opt_len).map(|_| rng.next_range(32, 126) as u8).collect();

        let actual = hash_poll_shuffle_value(uid, pid, &opt);

        let mut expected_buf = Vec::new();
        expected_buf.extend_from_slice(uid.to_string().as_bytes());
        expected_buf.extend_from_slice(&opt);
        expected_buf.extend_from_slice(pid.to_string().as_bytes());
        let expected = reference_ieee_802_3_crc32(&expected_buf);

        assert_eq!(actual, expected, "Mismatch against reference IEEE 802.3 CRC32!");
    }
}

#[test]
fn test_oracle_2_crc32_collision_resistance_and_avalanche_stress() {
    let mut rng = XorShift64::new(0xAAAA_5555_1234_4321);
    let mut seen_hashes = HashSet::with_capacity(10_000);

    for i in 0..10_000 {
        let uid = rng.next_u64();
        let pid = rng.next_u64();
        let opt_text = format!("Option_{}", i);
        let h = hash_poll_shuffle_value(uid, pid, opt_text.as_bytes());

        // Collision-free across randomized test set
        assert!(
            seen_hashes.insert(h),
            "Collision detected at trial {}: hash {:#X}",
            i, h
        );
    }

    // Avalanche effect check: 1-bit flip produces significant bit change (at least 20% bit flips)
    let uid = 9876543210u64;
    let pid = 1234567890u64;
    let h1 = hash_poll_shuffle_value(uid, pid, b"Test Option Alpha");
    let h2 = hash_poll_shuffle_value(uid ^ 1, pid, b"Test Option Alpha");
    let diff_bits = (h1 ^ h2).count_ones();
    assert!(
        diff_bits >= 8,
        "Poor avalanche effect: 1-bit flip produced only {} changed bits",
        diff_bits
    );
}

// ============================================================================
// ORACLE 3: Voice Record Bar FSM & Gesture Clamping Oracle
// Upstream: `Telegram/SourceFiles/history/view/controls/history_view_voice_record_bar.cpp:L39-L43, L2690-L2800`
// ============================================================================

#[test]
fn test_oracle_3_voice_record_bar_gesture_clamping_and_fsm() {
    let mut bar = VoiceRecordBar::new();
    assert_eq!(bar.state, VoiceRecordState::Idle);

    // Initial state: not active, not locked
    assert!(!bar.is_active());
    assert!(!bar.is_locked());
    assert_eq!(ANIMATION_THRESHOLD, 0.35);
    assert_eq!(AUDIO_VOICE_MAX_LENGTH_SECS, 6000);
    assert_eq!(AUDIO_VOICE_UPDATE_VIEW_MS, 200);

    // 1. Gesture Progress Clamping
    // Downward drag (dy >= 0.0) is always clamped to 0.0
    assert_eq!(bar.calc_lock_progress(0.0), 0.0);
    assert_eq!(bar.calc_lock_progress(50.0), 0.0);
    assert_eq!(bar.calc_lock_progress(1000.0), 0.0);

    // Upward drag ratio clamped between 0.0 and 1.0
    let progress_half = bar.calc_lock_progress(-LOCK_HEIGHT_PX * 0.5);
    assert!((progress_half - 0.5).abs() < 1e-4);

    let progress_full = bar.calc_lock_progress(-LOCK_HEIGHT_PX);
    assert!((progress_full - 1.0).abs() < 1e-4);

    let progress_overflow = bar.calc_lock_progress(-LOCK_HEIGHT_PX * 2.0);
    assert_eq!(progress_overflow, 1.0, "Progress must clamp at 1.0");

    // 2. Upward drag beyond 0.35 triggers Lock
    bar.start_holding(false);
    assert!(bar.is_active());
    // Upward drag where delta_y triggers lock (progress > 0.35 and delta_y < -50px)
    bar.handle_drag(0.0, -52.0);
    assert!(
        bar.is_locked(),
        "Upward drag to -52.0px (progress = {:.3} > 0.35) must trigger Lock",
        52.0 / LOCK_HEIGHT_PX
    );

    // 3. Slide beyond 0.8 cancels recording
    let mut cancel_bar = VoiceRecordBar::new();
    cancel_bar.start_holding(false);

    // Slide 70px (ratio 0.70 < 0.80): remains in Holding
    cancel_bar.handle_drag(-70.0, 0.0);
    match cancel_bar.state {
        VoiceRecordState::Holding { .. } => {}
        _ => panic!("Slide below 0.8 threshold should NOT cancel"),
    }

    // Slide 80px or beyond (ratio >= 0.80, threshold = -80.0px): transitions to Discarded
    cancel_bar.handle_drag(SLIDE_CANCEL_THRESHOLD_PX - 1.0, 0.0);
    match cancel_bar.state {
        VoiceRecordState::Discarded { ref reason } => {
            assert!(reason.contains("Slid left"), "Expected slide cancel reason, got: {}", reason);
        }
        _ => panic!("Slide beyond 0.8 must discard recording"),
    }

    // 4. Duration cap at 6,000s (100 minutes)
    let mut dur_bar = VoiceRecordBar::new();
    dur_bar.start_holding(false);
    assert_eq!(dur_bar.max_duration_secs, 6000);

    // Advance by 1,000 seconds
    dur_bar.tick_duration(1_000_000);
    match dur_bar.state {
        VoiceRecordState::Holding { duration_ms, .. } => assert_eq!(duration_ms, 1_000_000),
        _ => panic!("Expected Holding state"),
    }

    // Advance beyond 6,000 seconds (e.g. 10,000 seconds total)
    dur_bar.tick_duration(9_000_000);
    match dur_bar.state {
        VoiceRecordState::Holding { duration_ms, .. } => {
            assert_eq!(
                duration_ms,
                6_000_000,
                "Duration must be capped at 6,000s (6,000,000ms), got: {}",
                duration_ms
            );
        }
        _ => panic!("Expected Holding state"),
    }

    // 5. Stage wrapper controls
    let mut stage_bar = HistoryViewVoiceRecordBarViewControls::new();
    stage_bar.start_holding(true);
    assert!(stage_bar.is_active());
    stage_bar.handle_drag(0.0, -LOCK_HEIGHT_PX);
    assert!(stage_bar.is_locked());
    stage_bar.stop(StopType::Cancel);
    assert!(!stage_bar.is_active());
}

// ============================================================================
// ORACLE 4: Reactions Sorting Tie-Breakers Oracle
// Upstream: `Telegram/SourceFiles/history/view/reactions/history_view_reactions.cpp:L180-L207`
// Invariant:
// 1. Paid stars reactions strictly sort first.
// 2. Count minus user vote descending.
// 3. Catalog index resolves exact ties ascending.
// ============================================================================

#[test]
fn test_oracle_4_reactions_sorting_tie_breakers_500_scenarios() {
    let mut rng = XorShift64::new(0x7777_8888_9999_AAAA);

    for scenario in 0..500 {
        let num_pills = rng.next_range(2, 20) as usize;
        let mut pills = Vec::with_capacity(num_pills);

        for i in 0..num_pills {
            let is_paid = (rng.next_range(0, 10) == 0) && (i == 0); // At most one paid or random
            let key = if is_paid {
                ReactionKey::Paid
            } else if rng.next_range(0, 1) == 0 {
                ReactionKey::Emoji(format!("emoji_{}", i))
            } else {
                ReactionKey::Custom(rng.next_u64())
            };

            let count = rng.next_range(1, 500) as i32;
            let chosen = rng.next_range(0, 1) == 1;
            let catalog_index = rng.next_range(0, 100) as usize;

            let mut pill = ReactionPill::new(key, count);
            pill.paid = is_paid;
            pill.chosen = chosen;
            pill.catalog_index = catalog_index;
            pills.push(pill);
        }

        let sorted = sort_reaction_pills(pills);
        assert_eq!(sorted.len(), num_pills, "Scenario {}: element count changed", scenario);

        // Verify sorted ordering properties
        for i in 0..(num_pills - 1) {
            let a = &sorted[i];
            let b = &sorted[i + 1];

            // Invariant 1: Paid reaction strictly sorts first
            if a.paid != b.paid {
                assert!(
                    a.paid && !b.paid,
                    "Scenario {}: Paid reaction not sorted first at indices {} and {}",
                    scenario, i, i + 1
                );
                continue;
            }

            // Invariant 2: Count minus user vote descending
            let eff_a = a.count - if a.chosen { 1 } else { 0 };
            let eff_b = b.count - if b.chosen { 1 } else { 0 };

            if eff_a != eff_b {
                assert!(
                    eff_a > eff_b,
                    "Scenario {}: Count tie-breaker failed: eff_a ({}) < eff_b ({}) at {} vs {}",
                    scenario, eff_a, eff_b, i, i + 1
                );
            } else {
                // Invariant 3: Exact ties broken by catalog index ascending
                assert!(
                    a.catalog_index <= b.catalog_index,
                    "Scenario {}: Catalog index tie-breaker failed: cat_a ({}) > cat_b ({}) at {} vs {}",
                    scenario, a.catalog_index, b.catalog_index, i, i + 1
                );
            }
        }

        // Idempotency check: sorting an already-sorted vector produces the exact same order
        let resorted = sort_reaction_pills(sorted.clone());
        for i in 0..num_pills {
            assert_eq!(sorted[i].id, resorted[i].id);
            assert_eq!(sorted[i].count, resorted[i].count);
            assert_eq!(sorted[i].chosen, resorted[i].chosen);
            assert_eq!(sorted[i].catalog_index, resorted[i].catalog_index);
        }
    }

    // Verify stage view integration
    let mut reactions_view = HistoryViewReactionsViewReactions::new(42);
    reactions_view.add_reaction(ReactionKey::Emoji("👍".to_string()), true);
    reactions_view.add_reaction(ReactionKey::Paid, false);
    reactions_view.add_reaction(ReactionKey::Emoji("❤️".to_string()), false);
    assert_eq!(reactions_view.pills.len(), 3);
    assert_eq!(reactions_view.pills[0].id, ReactionKey::Paid); // Paid strictly first
}

// ============================================================================
// ORACLE 5: 60-Frame Looping Dust Particle Animation Oracle
// Upstream: `Telegram/SourceFiles/history/view/media/history_view_media_spoiler.cpp:L32-L75`
// ============================================================================

#[test]
fn test_oracle_5_spoiler_particles_60_frame_looping_and_alpha() {
    let mut state = MediaSpoilerState::default();

    // 1. Initial State: Exactly 64 deterministic particle vectors
    assert_eq!(state.particles.len(), SPOILER_PARTICLE_COUNT);
    assert_eq!(SPOILER_PARTICLE_COUNT, 64);
    assert_eq!(SPOILER_FRAME_COUNT, 60);

    for (i, p) in state.particles.iter().enumerate() {
        assert!(
            p.x >= 0.0 && p.x <= 1.0,
            "Particle {}: x out of bounds: {}", i, p.x
        );
        assert!(
            p.y >= 0.0 && p.y <= 1.0,
            "Particle {}: y out of bounds: {}", i, p.y
        );
        assert!(
            p.vx >= -0.02 && p.vx <= 0.02,
            "Particle {}: vx out of bounds: {}", i, p.vx
        );
        assert!(
            p.vy >= -0.02 && p.vy <= 0.02,
            "Particle {}: vy out of bounds: {}", i, p.vy
        );
        assert!(
            p.alpha >= 0.3 && p.alpha <= 0.9,
            "Particle {}: alpha out of bounds: {}", i, p.alpha
        );
        assert!(
            p.size >= 1.5 && p.size <= 3.5,
            "Particle {}: size out of bounds: {}", i, p.size
        );
    }

    // 2. 60-Frame Looping: Frame index strictly loops 0..60
    assert_eq!(state.current_frame, 0);
    for frame in 1..=60 {
        state.advance_frame();
        assert_eq!(
            state.current_frame,
            frame % SPOILER_FRAME_COUNT,
            "Frame counter mismatch at step {}",
            frame
        );

        // Particle coordinates must remain finite and wrapped in [0.0, 1.0)
        for (i, p) in state.particles.iter().enumerate() {
            assert!(
                p.x.is_finite() && p.x >= 0.0 && p.x < 1.0,
                "Particle {} x coordinate corrupted at frame {}: {}",
                i, frame, p.x
            );
            assert!(
                p.y.is_finite() && p.y >= 0.0 && p.y < 1.0,
                "Particle {} y coordinate corrupted at frame {}: {}",
                i, frame, p.y
            );
        }
    }
    // At 60 steps, current_frame loops back to 0
    assert_eq!(state.current_frame, 0);

    // 3. Reveal and Alpha Curve Progression
    assert_eq!(state.opacity(), 1.0);
    state.is_revealed = true;

    let mut opacities = Vec::new();
    opacities.push(state.opacity());

    for _ in 0..10 {
        state.advance_frame();
        opacities.push(state.opacity());
    }

    // Opacity must monotonically decrease from 1.0 to 0.0
    for i in 0..(opacities.len() - 1) {
        assert!(
            opacities[i] >= opacities[i + 1],
            "Opacity did not decrease monotonically: {} < {}",
            opacities[i], opacities[i + 1]
        );
    }
    assert_eq!(
        *opacities.last().unwrap(),
        0.0,
        "Opacity must reach 0.0 upon full reveal"
    );

    // 4. View media wrapper
    let mut spoiler_view = HistoryViewMediaSpoilerViewMedia::new()
        .with_paid_stars(50)
        .with_sensitive(true);
    assert!(!spoiler_view.is_revealed());
    assert!(spoiler_view.paid_tag.is_some());
    assert!(spoiler_view.sensitive_tag.is_some());
    spoiler_view.reveal();
    assert!(spoiler_view.is_revealed());
}
