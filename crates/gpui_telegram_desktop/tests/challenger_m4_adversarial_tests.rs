//! Milestone 4 Empirical Adversarial Challenge Suite (`crates/gpui_telegram_desktop`).
//!
//! Deep adversarial verification covering:
//! 1. IME Commit Fencing under rapid candidate bursts & Cocoa trailing return trapping (Feature F7)
//! 2. Markdown Parser resilience under fuzzed nesting, unclosed tags & pathological edge cases (Feature F10)
//! 3. 25,000-message virtualized scrolling, prefix-sum monotonicity, and boundary geometry (Feature F8)

use gpui_telegram_desktop::views::*;
use gpui_telegram_desktop::views::composer::{ImeSafetyMachine, ImeState, KeyAction};
use gpui_telegram_desktop::views::conversation_stage::*;
use gpui_telegram_desktop::views::stage::cards::text_card::{parse_markdown_spans, RichSpanKind};
use gpui::{px, Point, Size};
use std::time::Instant;

// ============================================================================
// PART 1: IME Commit Fencing Under Rapid Candidate Bursts (Feature F7)
// ============================================================================

#[test]
fn test_challenge_ime_rapid_candidate_selection_bursts_1000_cycles() {
    let mut ime = ImeSafetyMachine::new();
    let candidates = [
        ("ceshi", "测试"),
        ("nihao", "你好"),
        ("shijie", "世界"),
        ("zhongwen", "中文"),
        ("shurufa", "输入法"),
        ("houxuan", "候选"),
        ("kuaijie", "快捷"),
        ("gpui", "GPUI"),
        ("telegram", "电报"),
        ("adversarial", "对抗性"),
    ];

    // 1,000 rapid cycles of IME composition, candidate commits, and trailing return trapping
    for i in 0..1000 {
        let (preedit, candidate) = candidates[i % candidates.len()];

        // 1. Start IME composition
        ime.on_ime_start();
        assert!(ime.is_composing(), "Cycle {}: must be in Composing state", i);
        assert!(!ime.just_committed, "Cycle {}: fence must not be active on start", i);

        // 2. Rapid keystroke updates within composition window
        for step in 1..=preedit.len() {
            ime.on_ime_update(&preedit[..step], step);
            assert_eq!(ime.preedit_text(), Some(&preedit[..step]));
        }

        // 3. User selects candidate (OS commits candidate text)
        ime.on_ime_commit(candidate);
        assert!(ime.just_committed, "Cycle {}: Commit fence MUST be raised upon commit", i);
        assert!(matches!(ime.state, ImeState::Committed { .. }));
        assert!(ime.is_composing_or_committed());

        // 4. Cocoa / OS immediately fires trailing Return keydown in the same event tick
        let trailing_action = ime.handle_enter_key(false);
        assert_eq!(
            trailing_action,
            KeyAction::ImeCandidateChosen,
            "Cycle {}: Trailing Return MUST be trapped by fence and return ImeCandidateChosen, NEVER SendMessage!",
            i
        );
        assert!(!ime.just_committed, "Cycle {}: Fence must be reset after consuming trailing Return", i);
        assert_eq!(ime.state, ImeState::Idle, "Cycle {}: IME state must return to Idle", i);
    }

    // Verify buffer integrity
    assert!(!ime.text().is_empty());
    let buffer_len_before_send = ime.text().len();

    // 5. Subsequent intentional Return keydown without IME activity dispatches message
    let intentional_send = ime.handle_enter_key(false);
    assert_eq!(intentional_send, KeyAction::SendMessage, "Deliberate Return keydown must send message");
    assert_eq!(ime.text().len(), buffer_len_before_send, "Send action does not alter text buffer directly");
}

#[test]
fn test_challenge_ime_hostile_burst_sequences_and_edge_transitions() {
    let mut ime = ImeSafetyMachine::new();

    // 1. Empty commit burst
    ime.on_ime_start();
    ime.on_ime_update("", 0);
    ime.on_ime_commit("");
    assert!(ime.just_committed, "Fence raised even on empty commit");
    let action = ime.handle_enter_key(false);
    assert_eq!(action, KeyAction::ImeCandidateChosen, "Absorbs trailing return on empty commit");
    // Deliberate enter on empty buffer produces None (not SendMessage)
    assert_eq!(ime.handle_enter_key(false), KeyAction::None);

    // 2. Direct enter selection without pre-commit (raw pinyin acceptance)
    ime.on_ime_start();
    ime.on_ime_update("pinyin", 6);
    let direct_action = ime.handle_enter_key(false);
    assert_eq!(direct_action, KeyAction::ImeCandidateChosen);
    assert_eq!(ime.text(), "pinyin");
    // Trailing return immediately absorbed
    assert_eq!(ime.handle_enter_key(false), KeyAction::ImeCandidateChosen);
    // Third enter sends
    assert_eq!(ime.handle_enter_key(false), KeyAction::SendMessage);

    ime.clear();

    // 3. Repeated cancellation mid-flight during bursts
    for _ in 0..50 {
        ime.on_ime_start();
        ime.on_ime_update("fragment", 8);
        ime.on_ime_cancel();
        assert_eq!(ime.state, ImeState::Idle);
        assert!(!ime.just_committed);
    }
    assert_eq!(ime.text(), "");

    // 4. Typing immediately clears commit fence without requiring enter
    ime.on_ime_start();
    ime.on_ime_update("test", 4);
    ime.on_ime_commit("测试");
    assert!(ime.just_committed);
    // User types space or punctuation
    ime.insert_text(" ");
    assert!(!ime.just_committed, "Typing must clear commit fence");
    assert_eq!(ime.state, ImeState::Idle);
    // Enter now sends
    assert_eq!(ime.handle_enter_key(false), KeyAction::SendMessage);

    ime.clear();

    // 5. Shift+Enter multiline insertion safety during and after IME
    ime.insert_text("First line");
    ime.on_ime_start();
    ime.on_ime_update("second", 6);
    ime.on_ime_commit("第二行");
    // Trailing return absorbed
    assert_eq!(ime.handle_enter_key(false), KeyAction::ImeCandidateChosen);
    // Shift+Enter inserts newline
    assert_eq!(ime.handle_enter_key(true), KeyAction::InsertNewline);
    assert_eq!(ime.text(), "First line第二行\n");

    // 6. UTF-8 multi-byte backspace boundary fuzzing
    let complex_text = "🎉你好🚀世界🦀Arabic:مرحبا\nEmoji:👨‍👩‍👧‍👦";
    ime.clear();
    ime.insert_text(complex_text);
    let char_count = complex_text.chars().count();
    for _ in 0..char_count {
        ime.backspace();
    }
    assert_eq!(ime.text(), "");
    assert_eq!(ime.cursor_index, 0);
}

// ============================================================================
// PART 2: Markdown Parser Fuzzing & Unclosed Tags (Feature F10)
// ============================================================================

#[test]
fn test_challenge_markdown_pathological_unclosed_and_nested_tags() {
    // 1. Extreme repeated unclosed tag bursts
    let pathological_unclosed = [
        "*", "**", "***", "****", "*****", "******",
        "_", "__", "___", "____", "_____", "______",
        "~", "~~", "~~~", "~~~~", "~~~~~", "~~~~~~",
        "|", "||", "|||", "||||", "|||||", "||||||",
        "`", "``", "```", "````", "`````", "``````",
        "************************************************************",
        "____________________________________________________________",
        "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
        "||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||",
        "````````````````````````````````````````````````````````````",
    ];

    for &input in &pathological_unclosed {
        let spans = parse_markdown_spans(input);
        assert!(!spans.is_empty(), "Spans must not be empty for unclosed input: {}", input);
        // Reconstruct text from spans
        let reconstructed: String = spans.iter().map(|s| s.text.as_str()).collect();
        // Since delimiters are unclosed, the text should be preserved
        assert!(!reconstructed.is_empty());
    }

    // 2. Interleaved and overlapping tags
    let interleaved_cases = [
        "**bold _italic** still italic_",
        "||spoiler **bold|| still bold**",
        "__underline ~~strike__ still strike~~",
        "***triple bold and italic***",
        "****four asterisks enclosed****",
        "*****five asterisks enclosed*****",
        "`code with **bold** and *italic* inside`",
        "**bold with `code` and ||spoiler|| inside**",
        "||spoiler with `code` and **bold** inside||",
        "*italic with `code` inside*",
        "*a*b*c*d*e*f*g*",
        "||a||b||c||d||e||",
        "__a__b__c__d__e__",
        "~~a~~b~~c~~d~~e~~",
        "`a``b``c``d``e`",
        "Plain text before **bold** and middle _italic_ and end ||spoiler||",
    ];

    for &case in &interleaved_cases {
        let spans = parse_markdown_spans(case);
        assert!(!spans.is_empty(), "Failed to parse interleaved case: {}", case);
    }

    // 3. URLs with pathological punctuation, non-ASCII codepoints & nested syntax
    let edge_urls = [
        "https://",
        "http://",
        "https://.",
        "https://..",
        "https://example.com/path/to/resource?query=1&foo=bar#section",
        "https://user:password@secure.example.com:8443/api?key=val#hash",
        "Check this link: https://t.me/durov, and (https://rust-lang.org)!",
        "https://example.com/path,with,commas...and...dots!???;;::",
        "https://example.com/unicode_中文_🚀_тест",
        "Text https://a.com and http://b.org and https://c.net/path.",
    ];

    for &u in &edge_urls {
        let spans = parse_markdown_spans(u);
        assert!(!spans.is_empty(), "Failed to parse URL: {}", u);
    }

    // Explicit check on punctuation trimming
    let test_url = "See https://telegram.org/faq, immediately!";
    let url_spans = parse_markdown_spans(test_url);
    let link_span = url_spans.iter().find(|s| matches!(s.kind, RichSpanKind::Link(_))).expect("Expected Link span");
    assert_eq!(link_span.text, "https://telegram.org/faq");
}

#[test]
fn test_challenge_markdown_randomized_fuzzing_1000_iterations() {
    let tokens = [
        "**", "*", "__", "_", "~~", "||", "`", " ", "\t",
        "Hello", "世界", "Rust", "🚀", "🦀", "https://t.me",
        "http://example.com/api", "foo_bar", "test*case", "123",
        "(parentheses)", "[brackets]", "{braces}", ", ", ". ", "! ",
    ];

    let start = Instant::now();
    let mut rng_state: u64 = 0xDEADBEEF_CAFE1234;

    // Linear Congruential Generator for reproducible pseudo-random fuzzing
    let mut next_rand = || -> usize {
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (rng_state >> 32) as usize
    };

    for _ in 0..1000 {
        let num_tokens = (next_rand() % 15) + 1;
        let mut fuzzed_str = String::new();
        for _ in 0..num_tokens {
            let idx = next_rand() % tokens.len();
            fuzzed_str.push_str(tokens[idx]);
        }

        // Must never panic, hang, or crash
        let spans = parse_markdown_spans(&fuzzed_str);
        if !fuzzed_str.is_empty() {
            assert!(!spans.is_empty());
        }
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 500,
        "Fuzzed markdown parsing 1,000 strings took too long: {:?}",
        elapsed
    );
}

// ============================================================================
// PART 3: 25,000-Message Virtualized Scrolling & Boundary Geometry (Feature F8)
// ============================================================================

fn generate_25k_test_messages(count: usize) -> Vec<MessageItemModel> {
    let mut msgs = Vec::with_capacity(count);
    for i in 0..count {
        let msg_id = (i as u64) + 1;
        let is_outgoing = (i % 3) == 0;
        let sender_id = if is_outgoing { 1 } else { (i % 5 as usize) as u64 + 2 };
        let timestamp = format!("{:02}:{:02}", (i / 60) % 24, i % 60);

        let content = match i % 11 {
            0 => MessageContent::Service {
                text: format!("Date Divider Day #{}", i / 11),
                service_type: ServiceType::DateDivider,
            },
            1 => MessageContent::Text {
                text: format!("Short single-line text message #{}", i),
                is_edited: false,
            },
            2 => MessageContent::Text {
                text: format!(
                    "Multi-line paragraph message #{}\nLine 2 details and specs\nLine 3 Telegram 1:1 UI/UX alignment\nLine 4 virtualized SumTree",
                    i
                ),
                is_edited: true,
            },
            3 => MessageContent::Voice {
                duration_secs: (i % 60) as u32 + 5,
                waveform_5bit: vec![(i % 32) as u8; 32],
                playback_position_secs: 0.0,
                is_playing: false,
                playback_speed: 1.0,
            },
            4 => MessageContent::Media {
                media_type: MediaType::Photo,
                url: format!("https://cdn.example.com/photo_{}.jpg", i),
                width: 1280,
                height: 720,
                duration_secs: None,
                file_size_bytes: 850_000,
                caption: Some(format!("Photo attachment #{} caption text", i)),
                is_streaming: false,
            },
            5 => MessageContent::Album {
                items: vec![
                    AlbumItem {
                        id: 1,
                        media_type: MediaType::Photo,
                        url: format!("https://cdn.example.com/album_{}_1.jpg", i),
                        aspect_ratio: 1.33,
                        preview_color: 0x2C3E50,
                        caption: None,
                    },
                    AlbumItem {
                        id: 2,
                        media_type: MediaType::Photo,
                        url: format!("https://cdn.example.com/album_{}_2.jpg", i),
                        aspect_ratio: 1.33,
                        preview_color: 0x34495E,
                        caption: None,
                    },
                ],
                caption: Some("Mosaic collage item".to_string()),
            },
            6 => MessageContent::Location {
                latitude: 40.7128 + (i as f64 * 0.0001),
                longitude: -74.0060 + (i as f64 * 0.0001),
                title: format!("Location pin venue #{}", i),
                address: "Broadway, New York, NY".to_string(),
            },
            7 => MessageContent::Poll {
                question: format!("Team poll question #{}?", i),
                poll_type: PollType::Regular,
                options: vec![
                    PollOption { id: 1, text: "Choice Alpha".to_string(), votes: 15, percentage: 75.0, is_selected: false, is_correct: false },
                    PollOption { id: 2, text: "Choice Beta".to_string(), votes: 5, percentage: 25.0, is_selected: false, is_correct: false },
                ],
                total_votes: 20,
                is_closed: false,
                selected_option_id: None,
            },
            8 => MessageContent::Call {
                call_type: CallType::Voice,
                direction: CallDirection::Incoming,
                duration_secs: 124,
                status_text: "Incoming Voice Call (2:04)".to_string(),
            },
            9 => MessageContent::Story {
                user_name: format!("Channel News #{}", i),
                avatar_initials: "CN".to_string(),
                caption: format!("Story reply #{}: Release announcement", i),
                preview_color: 0x6C5CE7,
                expires_in: "18h 42m".to_string(),
            },
            _ => MessageContent::Invoice {
                title: format!("Telegram Stars In-App Purchase #{}", i),
                description: "Digital badge unlocked".to_string(),
                amount_stars: 100,
                currency: "Telegram Stars".to_string(),
                status: InvoiceStatus::Paid,
                receipt_id: Some(format!("rcpt_{}", i)),
            },
        };

        msgs.push(MessageItemModel {
            id: msg_id,
            sender_id,
            sender_name: if is_outgoing { "Me".to_string() } else { format!("User_{}", sender_id) },
            avatar_initials: if is_outgoing { "ME".to_string() } else { "US".to_string() },
            avatar_color: 0x2AABEE,
            direction: if is_outgoing { MessageDirection::Outgoing } else { MessageDirection::Incoming },
            timestamp,
            delivery_status: DeliveryStatus::Read,
            reply_to: None,
            reactions: vec![],
            content,
            bot_keyboard: None,
            is_pinned: false,
        });
    }
    msgs
}

#[test]
fn test_challenge_virtualization_25k_messages_monotonicity_and_bounds() {
    let mut cache = RowHeightCache::new();
    let count = 25_000;
    let messages = generate_25k_test_messages(count);

    // 1. Rebuild prefix sums for 25,000 messages
    let start = Instant::now();
    cache.rebuild_prefix_sums(&messages);
    let duration = start.elapsed();

    println!("Rebuilt prefix sums for 25,000 messages in {:?}", duration);
    assert!(
        duration.as_millis() < 100,
        "Prefix sum rebuild for 25,000 messages took too long: {:?}",
        duration
    );

    assert_eq!(cache.len(), count);
    assert!(!cache.is_dirty());
    assert!(cache.total_height > 0.0);

    // 2. Strict Monotonicity: prefix_sums[i] must be strictly greater than prefix_sums[i - 1]
    let mut prev = 0.0f32;
    for (idx, &curr) in cache.prefix_sums.iter().enumerate() {
        assert!(
            curr > prev,
            "Prefix sum non-monotonic at index {}: prev={}, curr={}",
            idx,
            prev,
            curr
        );
        let row_h = curr - prev;
        assert!(
            row_h >= 24.0,
            "Row height at index {} too small: {}",
            idx,
            row_h
        );
        assert!(
            row_h <= 600.0,
            "Row height at index {} too large: {}",
            idx,
            row_h
        );
        prev = curr;
    }

    assert_eq!(cache.total_height, *cache.prefix_sums.last().unwrap());

    // 3. Binary Search Visible Range across 30 scroll positions
    let viewport_h = 750.0;
    let overscan_px = 250.0;
    let total_h = cache.total_height;

    let test_scroll_positions = [
        -500.0, // top overscroll
        0.0,
        100.0,
        total_h * 0.05,
        total_h * 0.10,
        total_h * 0.20,
        total_h * 0.30,
        total_h * 0.40,
        total_h * 0.50,
        total_h * 0.60,
        total_h * 0.70,
        total_h * 0.80,
        total_h * 0.90,
        total_h * 0.95,
        total_h - viewport_h - 100.0,
        total_h - viewport_h,
        total_h + 1000.0, // bottom overscroll
    ];

    for &scroll_top in &test_scroll_positions {
        let (start_idx, end_idx) = cache.visible_range(scroll_top, viewport_h, overscan_px, count);
        assert!(
            start_idx <= end_idx,
            "start_idx ({}) > end_idx ({}) at scroll_top {}",
            start_idx,
            end_idx,
            scroll_top
        );
        assert!(
            end_idx < count,
            "end_idx ({}) out of bounds ({}) at scroll_top {}",
            end_idx,
            count,
            scroll_top
        );

        // Bounded window size: Viewport (750) + 2 * Overscan (250) = 1250px.
        // Min estimated row height is 32px (Service), max visible items <= 55 items.
        let visible_count = end_idx - start_idx + 1;
        assert!(
            visible_count <= 65,
            "Visible count {} exceeded bound 65 at scroll_top {}",
            visible_count,
            scroll_top
        );

        // Spacer math consistency: top_spacer + rendered_height + bottom_spacer == total_h
        let top_spacer = if start_idx > 0 {
            cache.prefix_sums[start_idx - 1]
        } else {
            0.0
        };
        let bottom_spacer = if end_idx < count - 1 {
            cache.total_height - cache.prefix_sums[end_idx]
        } else {
            0.0
        };
        let rendered_h = cache.prefix_sums[end_idx]
            - if start_idx > 0 { cache.prefix_sums[start_idx - 1] } else { 0.0 };

        let reconstructed = top_spacer + rendered_h + bottom_spacer;
        assert!(
            (reconstructed - total_h).abs() < 0.1,
            "Reconstructed total {} != actual total {}",
            reconstructed,
            total_h
        );
    }
}

#[test]
fn test_challenge_virtualization_25k_dynamic_mutations_and_stickiness() {
    let mut stage = ConversationStage::new();
    stage.messages = generate_25k_test_messages(25_000);
    stage.row_height_cache.rebuild_prefix_sums(&stage.messages);

    // 1. Stickiness verification at bottom:
    // With offset at bottom (Point::default()), incoming messages keep is_at_bottom() true
    assert!(stage.is_at_bottom());
    for i in 0..50 {
        stage.add_incoming_message(999_000 + i, &format!("Incoming real-time message #{}", i));
        assert!(stage.is_at_bottom(), "Incoming message #{} broke bottom anchoring!", i);
    }

    // 2. History reading isolation:
    // User scrolled up to read earlier history (offset y = -15,000px)
    stage.set_simulated_scroll_offset(Point::new(px(0.0), px(-15_000.0)), Size::new(px(0.0), px(500_000.0)));
    assert!(!stage.is_at_bottom());
    let offset_before = stage.scroll_handle.offset().y;

    for i in 0..50 {
        stage.add_incoming_message(999_100 + i, &format!("Background chat message #{}", i));
        assert!(!stage.is_at_bottom(), "Background message erroneously jumped to bottom!");
        assert_eq!(
            stage.scroll_handle.offset().y,
            offset_before,
            "Background message altered history scroll offset!"
        );
    }

    // 3. Rapid scrolling gesture simulation: 10,000 offset queries in < 100ms
    let start = Instant::now();
    for step in 0..10_000 {
        let test_offset = (step as f32) * 10.0;
        let (s, e) = stage.row_height_cache.visible_range(test_offset, 800.0, 200.0, stage.messages.len());
        assert!(s <= e && e < stage.messages.len());
    }
    let scroll_duration = start.elapsed();
    assert!(
        scroll_duration.as_millis() < 200,
        "10,000 virtual scroll queries took too long: {:?}",
        scroll_duration
    );
}
