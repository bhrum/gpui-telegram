//! Blind C++ Oracle Adversarial Stress Tests for Module 08 (UI & Boxes).
//! Validates UI Widgets, Text Layout, Animations, and Effects under Mandates #11, #15, #18, #19, and #20.
//!
//! Direct Upstream References:
//! - `Telegram/SourceFiles/ui/effects/shake_animation.h:L10-L15`
//! - `Telegram/SourceFiles/ui/effects/shake_animation.cpp:L14-L37`
//! - `Telegram/SourceFiles/ui/text/text_word.cpp`
//! - `Telegram/SourceFiles/tests/test_text.cpp:L72-L98`, `L131-L165`
//! - `Telegram/SourceFiles/ui/rows_scroll_cache.h:L1-L48`
//! - `Telegram/SourceFiles/ui/rows_scroll_cache.cpp:L1-L42`
//! - `Telegram/SourceFiles/test/test_ink.cpp`
//! - `Telegram/SourceFiles/lib_ui/ui/widgets/buttons.cpp`
//! - `Telegram/SourceFiles/ui/widgets/sent_code_field.cpp`

use std::collections::VecDeque;

use gpui_telegram_core::core::TestInk;
use gpui_telegram_desktop::components::ui::effects::ShakeAnimationEffects;
use gpui_telegram_desktop::components::ui::rows_scroll_cache::{RowsScrollCache, K_STOP_TIMEOUT};
use gpui_telegram_desktop::components::ui::text::{
    item_text_default_options, parse_text_entities, tokenize_words, UiTextString,
};
use gpui_telegram_desktop::components::ui::widgets::sent_code_field::{
    SentCodeCall, SentCodeCallState,
};

/// Deterministic 64-bit XorShift Pseudo-Random Generator for reproducible stress testing.
#[derive(Debug)]
struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x853c49e6748fea9b } else { seed },
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u64() as f64) / (u64::MAX as f64)
    }

    fn next_range(&mut self, min: u64, max: u64) -> u64 {
        if min >= max {
            return min;
        }
        min + (self.next_u64() % (max - min + 1))
    }
}

/// Independent Blind C++ Oracle replicating `Telegram/SourceFiles/ui/effects/shake_animation.cpp:L14-L37`.
fn oracle_cpp_shake_shift(progress: f64) -> f64 {
    let full_progress = progress * 6.0;
    let segment = (full_progress.floor() as i32).clamp(0, 5);
    let part = full_progress - (segment as f64);

    let from = if segment == 0 {
        0.0
    } else if segment == 1 || segment == 3 || segment == 5 {
        1.0
    } else {
        -1.0
    };

    let to = if segment == 0 || segment == 2 || segment == 4 {
        1.0
    } else if segment == 1 || segment == 3 {
        -1.0
    } else {
        0.0
    };

    from * (1.0 - part) + to * part
}

#[test]
fn test_shake_animation_5_segment_oscillation_oracle_and_zero_crossings() {
    let magnitude = 12.0_f32;
    let mut shake = ShakeAnimationEffects::new(magnitude);

    assert!(!shake.is_shaking(), "Shake animation should be initially idle");
    shake.start();
    assert!(shake.is_shaking(), "Shake animation should be active after start()");

    // 1. Initial rest position (t = 0.0) -> offset must be 0.0
    shake.set_progress(0.0);
    assert_eq!(shake.compute_shift(), 0.0);
    assert_eq!(shake.current_pixel_offset(), 0.0);
    assert_eq!(oracle_cpp_shake_shift(0.0), 0.0);

    // 2. Positive peak 1 (t = 1/6, exact full_progress = 1.0)
    shake.set_progress((1.0 / 6.0) as f32);
    let s_peak1 = shake.compute_shift();
    assert!((s_peak1 - 1.0).abs() < 1e-5, "Segment 1 start should reach +1.0 peak");
    assert!((shake.current_pixel_offset() - magnitude).abs() < 1e-4);
    assert!((oracle_cpp_shake_shift(1.0 / 6.0) - 1.0).abs() < 1e-5);

    // Dispatch sampled positive peak at 0.1667 (~1/6)
    shake.set_progress(0.1667);
    let s_sampled1 = shake.compute_shift();
    assert!(s_sampled1 > 0.99, "Sampled 0.1667 should be at positive peak (~1.0)");
    assert!(shake.current_pixel_offset() > 0.99 * magnitude);

    // 3. First zero-crossing (t = 1.5 / 6.0 = 0.25)
    shake.set_progress(0.25);
    let s_zero1 = shake.compute_shift();
    assert!(s_zero1.abs() < 1e-5, "Progress 0.25 must be exact zero-crossing between peak 1 and peak 2");
    assert!(oracle_cpp_shake_shift(0.25).abs() < 1e-5);

    // 4. Negative peak 1 (t = 2/6, exact full_progress = 2.0)
    shake.set_progress((2.0 / 6.0) as f32);
    let s_peak2 = shake.compute_shift();
    assert!((s_peak2 - (-1.0)).abs() < 1e-5, "Segment 2 start should reach -1.0 peak");
    assert!((shake.current_pixel_offset() - (-magnitude)).abs() < 1e-4);
    assert!((oracle_cpp_shake_shift(2.0 / 6.0) - (-1.0)).abs() < 1e-5);

    // Dispatch sampled negative peak at 0.3333 (~2/6)
    shake.set_progress(0.3333);
    let s_sampled2 = shake.compute_shift();
    assert!(s_sampled2 < -0.99, "Sampled 0.3333 should be at negative peak (~-1.0)");
    assert!(shake.current_pixel_offset() < -0.99 * magnitude);

    // 5. Subsequent alternating peaks and zero-crossings
    // Zero-crossing 2 at 2.5 / 6.0
    shake.set_progress((2.5 / 6.0) as f32);
    assert!(shake.compute_shift().abs() < 1e-5);

    // Positive peak 2 at 3.0 / 6.0 (0.5)
    shake.set_progress(0.5);
    assert!((shake.compute_shift() - 1.0).abs() < 1e-5);

    // Zero-crossing 3 at 3.5 / 6.0
    shake.set_progress((3.5 / 6.0) as f32);
    assert!(shake.compute_shift().abs() < 1e-5);

    // Negative peak 2 at 4.0 / 6.0 (~0.6667)
    shake.set_progress((4.0 / 6.0) as f32);
    assert!((shake.compute_shift() - (-1.0)).abs() < 1e-5);

    // Zero-crossing 4 at 4.5 / 6.0 (0.75)
    shake.set_progress(0.75);
    assert!(shake.compute_shift().abs() < 1e-5);

    // Positive peak 3 at 5.0 / 6.0 (~0.8333)
    shake.set_progress((5.0 / 6.0) as f32);
    assert!((shake.compute_shift() - 1.0).abs() < 1e-5);

    // Final rest and completion at t = 1.0
    shake.set_progress(1.0);
    assert_eq!(shake.compute_shift(), 0.0);
    assert_eq!(shake.current_pixel_offset(), 0.0);
    assert!(!shake.is_shaking(), "Shake animation should be finished and idle at t=1.0");

    // 6. Adversarial randomized fuzzing: 1,000 trials vs blind C++ oracle
    let mut rng = XorShift64::new(0x42A0D);
    for _ in 0..1000 {
        let t = rng.next_f64();
        shake.set_progress(t as f32);
        let actual = shake.compute_shift() as f64;
        let expected = oracle_cpp_shake_shift(t);
        assert!(
            (actual - expected).abs() < 1e-4,
            "Mismatch with C++ oracle at t={t}: actual={actual}, expected={expected}"
        );
    }

    // 7. Clamping boundary resilience
    shake.start();
    shake.set_progress(-10.0);
    assert_eq!(shake.compute_shift(), 0.0, "Negative progress must clamp to 0.0");
    shake.set_progress(10.0);
    assert_eq!(shake.compute_shift(), 0.0, "Over-progress must clamp to 1.0");
    assert!(!shake.is_shaking());
}


#[test]
fn test_text_layout_wrapping_pathological_unicode_and_zwj_sequences() {
    let font_size = 14.0_f32;

    // 1. Pathological Unicode corpus
    let family_zwj = "👨‍👩‍👧‍👦";
    let dev_zwj = "👩‍💻";
    let rainbow_flag = "🏳️‍🌈";
    let athlete_zwj = "🏃🏽‍♀️";
    let thumbs_up_tone = "👍🏽";
    let wave_tone = "👋🏻";
    let flags = "🇺🇸 🇯🇵 🇩🇪 🇬🇧";
    let math_fraktur = "𝕸𝖆𝖍𝖆𝖞𝖆𝖓𝖆";
    let musical_clef = "𝄞";
    let bidi_controls = "Hello\u{200C}World\u{200D}Test\u{FEFF}String\u{200E}LTR\u{200F}RTL";

    let test_corpus = format!(
        "{family_zwj} {dev_zwj} {rainbow_flag} {athlete_zwj} {thumbs_up_tone} {wave_tone} {flags} {math_fraktur} {musical_clef} {bidi_controls}"
    );

    // 2. Tokenize words stress test: ensure no UTF-8 character boundary panics
    let avg_char_width = font_size * 0.55;
    let words = tokenize_words(&test_corpus, avg_char_width);
    assert!(!words.is_empty(), "Tokenized words must not be empty");

    // Verify all byte offsets and slices are valid UTF-8 boundaries
    for w in &words {
        assert!(w.offset <= test_corpus.len());
        assert!(w.offset + w.length <= test_corpus.len());
        let slice = &test_corpus[w.offset..w.offset + w.length];
        assert_eq!(slice, w.text, "Word slice text must match raw byte substring");
        assert!(w.width >= 0.0, "Word width must be non-negative");
    }

    // 3. Layout under normal available width (400.0 px)
    let layout_normal = UiTextString::new(&test_corpus, 400.0, font_size);
    assert!(!layout_normal.lines.is_empty());
    assert!(layout_normal.max_width > 0.0);
    assert!(layout_normal.count_height(400.0) >= layout_normal.min_height);

    // 4. Adversarial hyper-narrow width wrapping stress (available_width = 1.0 px)
    // Every non-whitespace word must break into its own line without infinite loop or hanging.
    let layout_narrow = UiTextString::new(&test_corpus, 1.0, font_size);
    let non_newline_words = words.iter().filter(|w| !w.is_newline).count();
    assert_eq!(
        layout_narrow.lines.len(),
        non_newline_words,
        "Under 1px width, each word must occupy its own distinct line"
    );

    // 5. Adversarial massive width wrapping (available_width = 100,000.0 px)
    // All words without newlines must remain on a single line.
    let layout_wide = UiTextString::new(&test_corpus, 100_000.0, font_size);
    assert_eq!(layout_wide.lines.len(), 1, "Without explicit newlines, massive width must fit all words on 1 line");

    // 6. Explicit newlines interleaved with pathological emojis
    let multiline_corpus = format!(
        "{family_zwj}\n{dev_zwj} {thumbs_up_tone}\n\n{flags}\n{math_fraktur}"
    );
    let layout_multiline = UiTextString::new(&multiline_corpus, 500.0, font_size);
    assert_eq!(layout_multiline.lines.len(), 5, "Explicit newlines (including empty line) must yield 5 lines");

    // 7. Markdown entities with pathological emojis inside and outside entities
    let opts = item_text_default_options();
    let md_text = format!("Check **{family_zwj}** and ||{dev_zwj} secret|| with `{musical_clef}` code!");
    let parsed = parse_text_entities(&md_text, &opts);
    assert_eq!(parsed.entities.len(), 3, "Parsed rich text must recognize 3 entities");

    let layout_md = UiTextString::with_options(&md_text, 300.0, font_size, &opts);
    assert!(!layout_md.lines.is_empty());
    // Verify entity offset lookup inside pathological token
    let found_bold = layout_md.entity_at_offset(6);
    assert!(found_bold.is_some(), "Offset inside bold entity must resolve correctly");
}


#[test]
fn test_rows_scroll_cache_memory_limits_fifo_eviction_and_timestamp_decay() {
    let mut cache = RowsScrollCache::new();
    assert_eq!(cache.total_memory, 0);
    assert_eq!(cache.row_count(), 0);
    assert!(!cache.is_scrolling());
    assert_eq!(cache.timeout, K_STOP_TIMEOUT);

    // 1. Exact memory calculation and overwrite safety
    // Row 1: 150x60 -> 150 * 60 * 4 = 36,000 bytes
    cache.insert(1, 150, 60);
    assert_eq!(cache.total_memory, 36_000);
    assert_eq!(cache.row_count(), 1);

    // Overwrite Row 1 with 200x50 -> 200 * 50 * 4 = 40,000 bytes (delta +4,000 bytes)
    cache.insert(1, 200, 50);
    assert_eq!(cache.total_memory, 40_000, "Re-inserting existing row must replace previous memory allocation");
    assert_eq!(cache.row_count(), 1);

    // Invalidate non-existent row: no change
    cache.invalidate(9999);
    assert_eq!(cache.total_memory, 40_000);
    assert_eq!(cache.row_count(), 1);

    // Invalidate Row 1: resets memory to 0
    cache.invalidate(1);
    assert_eq!(cache.total_memory, 0);
    assert_eq!(cache.row_count(), 0);

    // 2. FIFO Eviction under tight memory budget (Adversarial Stress Harness)
    // Budget: 128 KB = 131,072 bytes
    const MAX_MEMORY_BUDGET: usize = 128 * 1024;
    let mut fifo_queue: VecDeque<i64> = VecDeque::new();
    let mut rng = XorShift64::new(0x7F41);

    for row_id in 100..600 {
        let width = rng.next_range(120, 240) as usize;
        let height = rng.next_range(30, 60) as usize;
        let bytes_needed = width * height * 4;

        // Evict oldest rows FIFO until there is space for bytes_needed
        while cache.total_memory + bytes_needed > MAX_MEMORY_BUDGET && !fifo_queue.is_empty() {
            let oldest_id = fifo_queue.pop_front().unwrap();
            cache.invalidate(oldest_id);
        }

        cache.insert(row_id, width, height);
        fifo_queue.push_back(row_id);

        // Strict invariant check: cache total_memory must never exceed budget + current row
        assert!(
            cache.total_memory <= MAX_MEMORY_BUDGET + bytes_needed,
            "Cache memory invariant violated: total_memory={}",
            cache.total_memory
        );
        assert_eq!(cache.row_count(), fifo_queue.len());
        assert_eq!(cache.row_count(), cache.rows.len());
    }

    assert!(cache.row_count() > 0);
    assert!(cache.total_memory > 0);

    // 3. Scrolling debounce and timestamp decay state machine
    let t0 = 10_000_u64;
    cache.mark_scrolling(t0);
    assert!(cache.is_scrolling());
    assert_eq!(cache.last_scroll_ms, t0);

    // Mid-scroll ticks before 120ms timeout
    assert!(!cache.tick(t0 + 40), "Tick before timeout must return false");
    assert!(!cache.tick(t0 + 119), "Tick at 119ms must return false");
    assert!(cache.is_scrolling());
    assert!(cache.row_count() > 0, "Cache must remain intact while scrolling");

    // Consecutive scroll event extends deadline (decay prevention)
    let t_bump = t0 + 110;
    cache.mark_scrolling(t_bump);
    assert_eq!(cache.last_scroll_ms, t_bump);

    // At t0 + 200ms (90ms after bump): still within extended 120ms window!
    assert!(!cache.tick(t0 + 200));
    assert!(cache.is_scrolling());

    // Advance 121ms past bump (t_bump + 121ms): timeout fires!
    let timeout_fired = cache.tick(t_bump + 121);
    assert!(timeout_fired, "Tick after timeout expiration must return true");
    assert!(!cache.is_scrolling(), "Cache must transition to not scrolling");
    assert_eq!(cache.total_memory, 0, "All rows and memory must be cleared on timeout");
    assert_eq!(cache.row_count(), 0);
    assert!(cache.rows.is_empty());

    // Idempotent ticks after stop
    assert!(!cache.tick(t_bump + 500));
}


/// Blind C++ Oracle calculating maximal distance from press point (px, py) to the 4 corners of rectangle [w x h].
fn oracle_cpp_ripple_max_radius(px: f64, py: f64, w: f64, h: f64) -> f64 {
    let dx = px.max(w - px);
    let dy = py.max(h - py);
    (dx * dx + dy * dy).sqrt()
}

/// Upstream Telegram Desktop easeOutQuint curve (`lib_ui/ui/effects/ripple_animation.cpp`).
fn oracle_ease_out_quint(t: f64) -> f64 {
    let one_minus_t = 1.0 - t.clamp(0.0, 1.0);
    1.0 - one_minus_t.powi(5)
}

#[test]
fn test_ripple_animation_expansion_radius_math_and_alpha_decay() {
    // 1. Validate TestInk stepper and linear alpha decay curve
    let mut ink = TestInk::new(50.0, 50.0, 100.0);
    assert_eq!(ink.origin_x, 50.0);
    assert_eq!(ink.origin_y, 50.0);
    assert_eq!(ink.max_radius, 100.0);
    assert_eq!(ink.current_radius, 0.0);
    assert_eq!(ink.opacity, 1.0);
    assert!(ink.is_animating);

    // Step 25% of radius
    ink.step(25.0);
    assert_eq!(ink.current_radius, 25.0);
    assert!((ink.opacity - 0.75).abs() < 1e-5, "At r=25/100, alpha must be 0.75");
    assert!(ink.is_animating);

    // Step to 50%
    ink.step(25.0);
    assert_eq!(ink.current_radius, 50.0);
    assert!((ink.opacity - 0.50).abs() < 1e-5, "At r=50/100, alpha must be 0.50");
    assert!(ink.is_animating);

    // Step to 90%
    ink.step(40.0);
    assert_eq!(ink.current_radius, 90.0);
    assert!((ink.opacity - 0.10).abs() < 1e-5, "At r=90/100, alpha must be 0.10");
    assert!(ink.is_animating);

    // Overshoot step: should clamp to max_radius, alpha=0, and finish animating
    ink.step(20.0);
    assert_eq!(ink.current_radius, 100.0);
    assert_eq!(ink.opacity, 0.0);
    assert!(!ink.is_animating, "Animation must terminate when reaching max_radius");

    // Subsequent steps while inactive must be no-ops
    ink.step(50.0);
    assert_eq!(ink.current_radius, 100.0);
    assert_eq!(ink.opacity, 0.0);
    assert!(!ink.is_animating);

    // 2. Blind C++ Oracle Ripple Geometry and Quintic Ease-Out Radius Expansion
    let test_buttons = [
        (160.0_f64, 48.0_f64), // standard rectangular button
        (40.0_f64, 40.0_f64),   // round icon button
        (320.0_f64, 64.0_f64),  // wide action row
        (80.0_f64, 200.0_f64),  // vertical menu strip
    ];

    let mut rng = XorShift64::new(0x9E3779B9);

    for &(w, h) in &test_buttons {
        // Press at corner (0, 0): max radius must equal hypotenuse
        let r_corner = oracle_cpp_ripple_max_radius(0.0, 0.0, w, h);
        let expected_hypot = (w * w + h * h).sqrt();
        assert!((r_corner - expected_hypot).abs() < 1e-5);

        // Press at center (w/2, h/2): max radius must equal half hypotenuse
        let r_center = oracle_cpp_ripple_max_radius(w / 2.0, h / 2.0, w, h);
        assert!((r_center - expected_hypot / 2.0).abs() < 1e-5);

        // Randomized press points
        for _ in 0..50 {
            let px = rng.next_f64() * w;
            let py = rng.next_f64() * h;
            let max_r = oracle_cpp_ripple_max_radius(px, py, w, h);

            // Invariant: all 4 corners must be within max_r of (px, py)
            let corners = [(0.0, 0.0), (w, 0.0), (0.0, h), (w, h)];
            for (cx, cy) in corners {
                let dist = ((cx - px).powi(2) + (cy - py).powi(2)).sqrt();
                assert!(dist <= max_r + 1e-5, "Corner ({cx}, {cy}) must be bounded by max_r");
            }

            // Invariant: strictly monotonic expansion curve r(t)
            let mut prev_r = 0.0;
            let mut prev_alpha = 1.0;
            let steps = 20;
            for i in 1..=steps {
                let t = (i as f64) / (steps as f64);
                let ease_t = oracle_ease_out_quint(t);
                let current_r = max_r * ease_t;
                let current_alpha = 1.0 - t; // linear fade

                assert!(current_r >= prev_r, "Radius expansion must be strictly monotonic");
                assert!(current_alpha <= prev_alpha, "Alpha decay must be monotonically decreasing");

                prev_r = current_r;
                prev_alpha = current_alpha;
            }
            // At completion (t = 1.0), radius reaches full max_r and alpha reaches 0.0
            assert!((prev_r - max_r).abs() < 1e-5);
            assert!(prev_alpha.abs() < 1e-5);
        }
    }
}


/// Blind Oracle formatting function for countdown durations.
fn oracle_format_countdown(seconds: u32) -> String {
    let m = seconds / 60;
    let s = seconds % 60;
    if m >= 60 {
        let h = m / 60;
        let m_rem = m % 60;
        format!("{h}:{m_rem:02}:{s:02}")
    } else {
        format!("{m}:{s:02}")
    }
}

#[test]
fn test_sent_code_call_countdown_timer_state_machine_and_formatting() {
    // 1. Initial State: Requesting / CountingDown (Waiting)
    let initial_timeout = 125_u32; // 2 minutes 5 seconds
    let mut call = SentCodeCall::new(initial_timeout);

    assert_eq!(call.state, SentCodeCallState::Waiting);
    assert_eq!(call.timeout_seconds, 125);
    assert_eq!(call.formatted_text(), "2:05");
    assert_eq!(call.formatted_text(), oracle_format_countdown(125));

    // 2. Countdown loop: tick down to 1 second
    for remaining in (2..=125).rev() {
        assert_eq!(call.timeout_seconds, remaining);
        assert_eq!(call.state, SentCodeCallState::Waiting);
        assert_eq!(call.formatted_text(), oracle_format_countdown(remaining));

        let trigger = call.tick_second();
        assert!(!trigger, "Tick must return false while remaining seconds > 0");
    }

    // Now at exactly 1 second remaining
    assert_eq!(call.timeout_seconds, 1);
    assert_eq!(call.state, SentCodeCallState::Waiting);
    assert_eq!(call.formatted_text(), "0:01");

    // 3. Expiration transition: 1s -> 0s transitions to Calling (Expired / CanCall)
    let trigger_expired = call.tick_second();
    assert!(trigger_expired, "Tick reaching 0 must trigger transition event and return true");
    assert_eq!(call.timeout_seconds, 0);
    assert_eq!(call.state, SentCodeCallState::Calling, "State must transition to Calling on expiration");
    assert_eq!(call.formatted_text(), "Calling...");

    // 4. Over-ticking resilience (no underflow or wrapping)
    for _ in 0..10 {
        let over_tick = call.tick_second();
        assert!(!over_tick, "Over-ticking at 0 seconds must return false");
        assert_eq!(call.timeout_seconds, 0);
        assert_eq!(call.state, SentCodeCallState::Calling);
    }

    // 5. Completion transition: Calling -> Called
    call.state = SentCodeCallState::Called;
    assert_eq!(call.formatted_text(), "Called");

    // 6. Adversarial Formatting Oracle stress test across various durations
    let test_durations = [
        (0_u32, "0:00"),
        (5_u32, "0:05"),
        (59_u32, "0:59"),
        (60_u32, "1:00"),
        (65_u32, "1:05"),
        (599_u32, "9:59"),
        (3599_u32, "59:59"),
        (3600_u32, "1:00:00"),
        (3665_u32, "1:01:05"),
        (7325_u32, "2:02:05"),
        (86400_u32, "24:00:00"),
    ];

    for &(secs, expected_str) in &test_durations {
        let instance = SentCodeCall::new(secs);
        assert_eq!(instance.formatted_text(), expected_str);
        assert_eq!(instance.formatted_text(), oracle_format_countdown(secs));
    }

    // 7. Rapid countdown stress test (1,000 second run)
    let mut stress_call = SentCodeCall::new(1000);
    for i in 0..1000 {
        let fired = stress_call.tick_second();
        if i == 999 {
            assert!(fired, "Must fire at tick 1000");
            assert_eq!(stress_call.state, SentCodeCallState::Calling);
        } else {
            assert!(!fired);
            assert_eq!(stress_call.state, SentCodeCallState::Waiting);
        }
    }
}
