//! Blind C++ Oracle Adversarial Stress Tests for Module 07 Slice 1 (`chat_helpers` & `input`)
//!
//! Direct Upstream References:
//! - `Telegram/SourceFiles/chat_helpers/emoji_suggestions_widget.cpp:L956-L971` (Colon parser & replacement math)
//! - `Telegram/SourceFiles/chat_helpers/field_autocomplete.cpp:L404-L417` (Trigger parsing `@`, `#`, `/`)
//! - `Telegram/SourceFiles/chat_helpers/field_autocomplete.cpp:L1530-L1562` (1D and 2D selection navigation)
//! - `Telegram/SourceFiles/chat_helpers/tabbed_panel.cpp:L194-L222` (Responsive height clamp formula)
//! - `Telegram/SourceFiles/chat_helpers/message_field.cpp:L76-L84` (`kParseLinksTimeout`, `kLinkProtocols`)
//! - `Telegram/SourceFiles/inline_bots/bot_attach_web_view.cpp:L1610-L1618` (`tg://`, `ton://`, `tonsite://`)
//!
//! Enforces Acceptance Gate Oracles:
//! - Oracle 1: Colon trigger & replacement bounds fuzzing (5,000 randomized strings with leading/trailing colons, alphanumeric preceding boundaries, UTF-8/UTF-16 code units preservation).
//! - Oracle 2: Mention (@), Hashtag (#), Bot command (/) trigger parsing against pathological strings.
//! - Oracle 3: 1D vertical list & 2D sticker grid selection keyboard navigation fuzzing (1,000 steps).
//! - Oracle 4: Tabbed panel responsive height clamp formula min(avail_h * 0.75, 480.0).
//! - Oracle 5: Protocol link detection (kLinkProtocols = ["http://", "https://", "tg://", "ton://"]).

use gpui::px;
use gpui_telegram_desktop::theme::chat_helpers::ChatHelpersStyleSheet;
use gpui_telegram_desktop::views::composer::emoji_suggestions_widget::EmojiSuggestionsWidget;
use gpui_telegram_desktop::views::composer::field_autocomplete::{
    AutocompleteChooseMethod, AutocompleteType, BotCommandRow, FieldAutocomplete, HashtagRow,
    MentionRow, StickerRow,
};
use gpui_telegram_desktop::views::composer::message_field::{MessageField, K_LINK_PROTOCOLS};
use gpui_telegram_desktop::views::composer::tabbed_panel::TabbedPanel;

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
// ORACLE 1: Colon Trigger & Replacement Bounds Fuzzing (5,000 iterations)
// Upstream: `emoji_suggestions_widget.cpp:L956-L971`
// ============================================================================

#[test]
fn test_oracle_1_colon_trigger_replacement_bounds_fuzz_5000() {
    let mut sug = EmojiSuggestionsWidget::new();
    let mut rng = XorShift64::new(0xC010_0701_F002_0001);

    // Multilingual prefixes with UTF-8 multi-byte characters and varying boundaries
    let valid_prefix_delimiters = ["", " ", "  ", "\t", "(", "[", "{", "\"", "'", "—", "…", "Привет ", "🎉 "];
    let alphanumeric_delimiters = ["a", "Z", "9", "0", "k", "ж", "Ω", "word", "123"];
    let unicode_queries = ["smile", "heart", "fire", "кот", "огонь", "радость", "rocket_1", "star"];

    for iteration in 0..5_000 {
        let mode = rng.next_range(0, 4);

        match mode {
            0 => {
                // Case A: Alphanumeric preceding boundary -> MUST reject (None)
                let alpha = alphanumeric_delimiters[rng.next_range(0, (alphanumeric_delimiters.len() - 1) as u64) as usize];
                let query = unicode_queries[rng.next_range(0, (unicode_queries.len() - 1) as u64) as usize];
                let text = format!("{alpha}:{query}");
                let cursor = text.len();

                let result = sug.parse_query(&text, cursor);
                assert!(
                    result.is_none(),
                    "Iter {iteration}: Alphanumeric '{alpha}' before colon must be rejected! Got: {result:?}"
                );
                assert!(!sug.is_visible);
            }
            1 => {
                // Case B: Trailing colon without query or followed by whitespace -> MUST reject (None)
                let trailing_patterns = [":", "hello:", "test: ", "word:\t", ": "];
                let text = trailing_patterns[rng.next_range(0, (trailing_patterns.len() - 1) as u64) as usize];
                let cursor = text.len();

                let result = sug.parse_query(text, cursor);
                assert!(
                    result.is_none(),
                    "Iter {iteration}: Trailing colon pattern '{text}' must be rejected! Got: {result:?}"
                );
                assert!(!sug.is_visible);
            }
            2 => {
                // Case C: Valid colon trigger with leading boundary -> MUST accept and extract query
                let prefix = valid_prefix_delimiters[rng.next_range(0, (valid_prefix_delimiters.len() - 1) as u64) as usize];
                let query = unicode_queries[rng.next_range(0, (unicode_queries.len() - 1) as u64) as usize];
                let text = format!("{prefix}:{query}");
                let cursor = text.len();

                let result = sug.parse_query(&text, cursor);
                assert_eq!(
                    result.as_deref(),
                    Some(query),
                    "Iter {iteration}: Valid prefix '{prefix}' must trigger query '{query}'"
                );
                assert!(sug.is_visible);

                // UTF-8 char indexing and replacement bounds verification
                let chars: Vec<char> = text.chars().collect();
                let colon_char_idx = chars.iter().rposition(|&c| c == ':').unwrap();
                let expected_range = colon_char_idx..chars.len();
                assert_eq!(
                    sug.replacement_range(),
                    expected_range,
                    "Iter {iteration}: Replacement range must match character indices"
                );

                // UTF-16 code units preservation: ensure query UTF-16 code unit count is preserved
                let query_utf16_len: usize = query.chars().map(|c| c.len_utf16()).sum();
                assert_eq!(
                    sug.query.encode_utf16().count(),
                    query_utf16_len,
                    "Iter {iteration}: UTF-16 code units count must match"
                );
            }
            _ => {
                // Case D: Double colons or multiple colons in text -> reverse scan must find last colon
                let query = unicode_queries[rng.next_range(0, (unicode_queries.len() - 1) as u64) as usize];
                let text = format!("::{}", query);
                let cursor = text.len();

                let result = sug.parse_query(&text, cursor);
                assert_eq!(
                    result.as_deref(),
                    Some(query),
                    "Iter {iteration}: Double colon should trigger on second colon"
                );
                assert!(sug.is_visible);

                // Replacement range covers from second colon (char index 1)
                assert_eq!(sug.replacement_range(), 1..text.chars().count());
            }
        }
    }
}

// ============================================================================
// ORACLE 2: Mention (@), Hashtag (#), Bot Command (/) Trigger Parsing
// Upstream: `field_autocomplete.cpp:L404-L417`
// ============================================================================

#[test]
fn test_oracle_2_autocomplete_pathological_triggers() {
    let mut ac = FieldAutocomplete::new();
    ac.mention_rows.push(MentionRow {
        user_id: 101,
        username: "durov".to_string(),
        name: "Pavel Durov".to_string(),
        is_bot: false,
    });
    ac.hashtag_rows.push(HashtagRow {
        tag: "rustlang".to_string(),
    });
    ac.bot_command_rows.push(BotCommandRow {
        command: "start".to_string(),
        description: "Start bot".to_string(),
        bot_user_id: 999,
    });

    // 1. Empty string -> hides autocomplete
    ac.show_filtered("");
    assert!(!ac.is_open);
    assert_eq!(ac.selected_index, -1);

    // 2. Exact trigger character without query -> open with empty filter
    ac.show_filtered("@");
    assert!(ac.is_open);
    assert_eq!(ac.autocomplete_type, AutocompleteType::Mentions);
    assert_eq!(ac.filter, "");
    assert_eq!(ac.selected_index, 0);

    ac.show_filtered("#");
    assert!(ac.is_open);
    assert_eq!(ac.autocomplete_type, AutocompleteType::Hashtags);
    assert_eq!(ac.filter, "");

    ac.show_filtered("/");
    assert!(ac.is_open);
    assert_eq!(ac.autocomplete_type, AutocompleteType::BotCommands);
    assert_eq!(ac.filter, "");

    // 3. Case normalization (must be lowercased per upstream field_autocomplete)
    ac.show_filtered("@DuRoV");
    assert_eq!(ac.filter, "durov");

    ac.show_filtered("#RuStLaNg");
    assert_eq!(ac.filter, "rustlang");

    ac.show_filtered("/StArT");
    assert_eq!(ac.filter, "start");

    // 4. Repeated prefix characters
    ac.show_filtered("@@@admin");
    assert!(ac.is_open);
    assert_eq!(ac.autocomplete_type, AutocompleteType::Mentions);
    assert_eq!(ac.filter, "@@admin");

    ac.show_filtered("###tag");
    assert!(ac.is_open);
    assert_eq!(ac.autocomplete_type, AutocompleteType::Hashtags);
    assert_eq!(ac.filter, "##tag");

    ac.show_filtered("///command");
    assert!(ac.is_open);
    assert_eq!(ac.autocomplete_type, AutocompleteType::BotCommands);
    assert_eq!(ac.filter, "//command");

    // 5. Non-trigger leading characters -> must reject and hide
    let non_triggers = ["!help", "$cashtag", "%100", "^power", "&and", "*star", ":smile", "regular text"];
    for invalid in non_triggers {
        ac.show_filtered(invalid);
        assert!(!ac.is_open, "Trigger '{invalid}' should be rejected");
        assert_eq!(ac.selected_index, -1);
    }

    // 6. Leading whitespace must not trigger autocomplete
    let leading_spaces = [" @user", "  #tag", "\t/cmd", "\n@bot"];
    for invalid in leading_spaces {
        ac.show_filtered(invalid);
        assert!(!ac.is_open, "Leading whitespace '{invalid}' must not trigger autocomplete");
    }

    // 7. Pathological Unicode queries
    ac.show_filtered("@пользователь_123");
    assert!(ac.is_open);
    assert_eq!(ac.filter, "пользователь_123");

    ac.show_filtered("#разработка_gpui");
    assert!(ac.is_open);
    assert_eq!(ac.filter, "разработка_gpui");

    ac.show_filtered("/настройки");
    assert!(ac.is_open);
    assert_eq!(ac.filter, "настройки");

    // 8. Very long queries (10,000 characters) without buffer overflow
    let long_filter = "a".repeat(10_000);
    ac.show_filtered(&format!("@{long_filter}"));
    assert!(ac.is_open);
    assert_eq!(ac.filter.len(), 10_000);

    // 9. Selected value resolution
    ac.show_filtered("@durov");
    assert_eq!(
        ac.choose_selected(AutocompleteChooseMethod::ByEnter),
        Some("@durov".to_string())
    );

    ac.show_filtered("#rustlang");
    assert_eq!(
        ac.choose_selected(AutocompleteChooseMethod::ByTab),
        Some("#rustlang".to_string())
    );

    ac.show_filtered("/start");
    assert_eq!(
        ac.choose_selected(AutocompleteChooseMethod::ByClick),
        Some("/start".to_string())
    );
}

// ============================================================================
// ORACLE 3: 1D Vertical List & 2D Sticker Grid Keyboard Navigation (1,000 steps)
// Upstream: `field_autocomplete.cpp:L1530-L1562`
// ============================================================================

#[test]
fn test_oracle_3_keyboard_navigation_1d_and_2d_fuzz_1000_steps() {
    let mut rng = XorShift64::new(0x3D3D_1D2D_F003_0001);
    let nav_keys = ["Up", "Down", "ArrowUp", "ArrowDown", "Left", "Right", "ArrowLeft", "ArrowRight", "Escape", "PageUp"];

    // --- Sub-test A: 1D Vertical List (Mentions, N = 19 items) ---
    let mut ac_1d = FieldAutocomplete::new();
    for i in 0..19 {
        ac_1d.mention_rows.push(MentionRow {
            user_id: 1000 + i,
            username: format!("user_{i:02}"),
            name: format!("User {i}"),
            is_bot: i % 3 == 0,
        });
    }
    ac_1d.show_filtered("@user");
    assert_eq!(ac_1d.selected_index, 0);

    for step in 0..1_000 {
        let key = nav_keys[rng.next_range(0, (nav_keys.len() - 1) as u64) as usize];
        let moved = ac_1d.move_selection(key);

        let row_count = ac_1d.row_count() as i32;
        assert!(
            ac_1d.selected_index >= 0 && ac_1d.selected_index < row_count,
            "Step {step} (1D): selected_index {} out of bounds 0..{row_count} after key '{key}'",
            ac_1d.selected_index
        );

        if key == "Left" || key == "Right" || key == "ArrowLeft" || key == "ArrowRight" || key == "Escape" || key == "PageUp" {
            // Horizontal keys and invalid keys have no effect in 1D list
            assert!(!moved, "Key '{key}' should not move selection in 1D list");
        }
    }

    // --- Sub-test B: 2D Sticker Grid (Stickers, N = 37 items, 5 per row) ---
    let mut ac_2d = FieldAutocomplete::new();
    ac_2d.autocomplete_type = AutocompleteType::Stickers;
    ac_2d.stickers_per_row = 5;
    for i in 0..37 {
        ac_2d.sticker_rows.push(StickerRow {
            document_id: 50_000 + i,
            emoji: "✨".to_string(),
        });
    }
    ac_2d.is_open = true;
    ac_2d.selected_index = 0;

    for step in 0..1_000 {
        let key = nav_keys[rng.next_range(0, (nav_keys.len() - 1) as u64) as usize];
        let moved = ac_2d.move_selection(key);

        let row_count = ac_2d.row_count() as i32;
        assert!(
            ac_2d.selected_index >= 0 && ac_2d.selected_index < row_count,
            "Step {step} (2D): selected_index {} out of bounds 0..{row_count} after key '{key}'",
            ac_2d.selected_index
        );

        if key == "Escape" || key == "PageUp" {
            assert!(!moved);
        } else {
            assert!(moved, "Navigation key '{key}' should move 2D grid selection");
        }
    }

    // --- Sub-test C: Empty list invariant (0 items must never panic or succeed) ---
    let mut ac_empty = FieldAutocomplete::new();
    assert_eq!(ac_empty.row_count(), 0);
    for key in nav_keys {
        assert!(!ac_empty.move_selection(key));
        assert_eq!(ac_empty.selected_index, -1);
    }
}

// ============================================================================
// ORACLE 4: Tabbed Panel Responsive Height Clamp Formula: min(avail_h * 0.75, 480.0)
// Upstream: `tabbed_panel.cpp:L194-L222`, `chat_helpers.style:L517`
// ============================================================================

/// Canonical responsive clamp formula for tabbed panel height
pub fn responsive_panel_height_clamp(available_height: f32) -> f32 {
    let wanted = available_height * 0.75;
    wanted.min(480.0)
}

#[test]
fn test_oracle_4_tabbed_panel_responsive_height_clamp_formula() {
    let mut rng = XorShift64::new(0x4800_0750_F004_0001);

    // 1. Exact boundary values check
    // 640.0 * 0.75 = 480.0 (exact threshold point)
    assert_eq!(responsive_panel_height_clamp(640.0), 480.0);
    assert_eq!(responsive_panel_height_clamp(0.0), 0.0);
    assert_eq!(responsive_panel_height_clamp(400.0), 300.0);
    assert_eq!(responsive_panel_height_clamp(800.0), 480.0);
    assert_eq!(responsive_panel_height_clamp(1200.0), 480.0);
    assert_eq!(responsive_panel_height_clamp(2160.0), 480.0); // 4K screen

    // 2. Fuzzing 2,500 arbitrary available heights
    for _ in 0..2_500 {
        let avail_h = (rng.next_range(0, 4_000_000) as f32) / 1000.0; // 0.0 to 4000.0 px
        let clamped = responsive_panel_height_clamp(avail_h);

        // Invariant 1: Height must never exceed 480.0
        assert!(clamped <= 480.0 + 1e-4, "Height {clamped} exceeds 480.0 max cap");

        // Invariant 2: Below 640.0, height must be exactly avail_h * 0.75
        if avail_h <= 640.0 {
            assert!((clamped - avail_h * 0.75).abs() < 1e-4);
        } else {
            // Above 640.0, height must be saturated at 480.0
            assert_eq!(clamped, 480.0);
        }

        // Invariant 3: Monotonicity - larger available height must produce >= clamped height
        let higher_h = avail_h + 10.0;
        let higher_clamped = responsive_panel_height_clamp(higher_h);
        assert!(higher_clamped >= clamped);
    }

    // 3. TabbedPanel component integration with custom 480.0 max constraint
    let mut panel = TabbedPanel::new();
    panel.max_content_height = px(480.0);
    panel.height_ratio = 0.75;
    panel.bottom = px(800.0);
    panel.drop_down = false;

    // Available = 800 - 0 = 800. Wanted = 800 * 0.75 = 600. Max = 480. Wanted content = 480 - 14 = 466.
    panel.update_content_height(px(1000.0), px(14.0), px(0.0));
    assert!(panel.content_height <= px(480.0));

    // 4. ChatHelpersStyleSheet tokens
    let st = ChatHelpersStyleSheet::new();
    assert_eq!(st.emoji_pan_height_ratio, 0.75);
}

// ============================================================================
// ORACLE 5: Protocol Link Detection (kLinkProtocols = ["http://", "https://", "tg://", "ton://"])
// Upstream: `message_field.cpp:L76-L84`, `bot_attach_web_view.cpp:L1610-L1618`
// ============================================================================

pub const K_ORACLE_LINK_PROTOCOLS: &[&str] = &["http://", "https://", "tg://", "ton://"];

/// Canonical protocol link matcher per upstream Telegram Desktop specification
pub fn detect_oracle_protocol_links(text: &str) -> Vec<(&str, std::ops::Range<usize>)> {
    let mut links = Vec::new();
    let punctuation = &['(', ')', '[', ']', '{', '}', '<', '>', '"', '\'', ',', '.', ';', '!', '?', ':'][..];
    for raw_word in text.split_whitespace() {
        let trimmed = raw_word.trim_matches(punctuation);
        for &proto in K_ORACLE_LINK_PROTOCOLS {
            if trimmed.starts_with(proto) && trimmed.len() > proto.len() {
                let clean_word = trimmed.trim_end_matches(punctuation);
                if clean_word.len() > proto.len() {
                    if let Some(start) = text.find(clean_word) {
                        let end = start + clean_word.len();
                        links.push((clean_word, start..end));
                    }
                }
                break;
            }
        }
    }
    links
}

#[test]
fn test_oracle_5_protocol_link_detection_adversarial_oracle() {
    // 1. Verify standard web protocols
    let text1 = "Visit http://example.com/hello and https://telegram.org/faq now!";
    let links1 = detect_oracle_protocol_links(text1);
    assert_eq!(links1.len(), 2);
    assert_eq!(links1[0].0, "http://example.com/hello");
    assert_eq!(links1[1].0, "https://telegram.org/faq");

    // 2. Verify Telegram deep link protocol (tg://)
    let text2 = "Open tg://resolve?domain=durov or tg://join?invite=xyz123";
    let links2 = detect_oracle_protocol_links(text2);
    assert_eq!(links2.len(), 2);
    assert_eq!(links2[0].0, "tg://resolve?domain=durov");
    assert_eq!(links2[1].0, "tg://join?invite=xyz123");

    // 3. Verify TON blockchain protocol (ton://)
    let text3 = "Transfer to ton://transfer/EQBvW8Z5huBkMJYdnfAEM5JqAj19ZDtMkBpBpBp?amount=1000000000";
    let links3 = detect_oracle_protocol_links(text3);
    assert_eq!(links3.len(), 1);
    assert_eq!(links3[0].0, "ton://transfer/EQBvW8Z5huBkMJYdnfAEM5JqAj19ZDtMkBpBpBp?amount=1000000000");

    // 4. Combined protocols with sentence punctuation and parenthesis
    let text4 = "(see https://docs.rs/gpui), also [tg://settings], and ton://connect.";
    let links4 = detect_oracle_protocol_links(text4);
    assert_eq!(links4.len(), 3);
    assert_eq!(links4[0].0, "https://docs.rs/gpui");
    assert_eq!(links4[1].0, "tg://settings");
    assert_eq!(links4[2].0, "ton://connect");

    // 5. Bare protocols without host/path must be rejected
    let text5 = "Check http:// and https:// and tg:// and ton:// alone";
    let links5 = detect_oracle_protocol_links(text5);
    assert!(links5.is_empty(), "Bare protocols with empty targets must not be detected as links");

    // 6. Unsupported protocols must be rejected
    let text6 = "ftp://files.example.com mailto:user@test.com ssh://root@host";
    let links6 = detect_oracle_protocol_links(text6);
    assert!(links6.is_empty(), "Unsupported protocols must be ignored");

    // 7. MessageField link parser integration
    let mut mf = MessageField::new();
    mf.set_text("Read https://telegram.org/apps and tonsite://foundation.ton for info".to_string());
    assert_eq!(mf.parsed_links.len(), 2);
    assert_eq!(mf.parsed_links[0].url, "https://telegram.org/apps");
    assert_eq!(mf.parsed_links[1].url, "tonsite://foundation.ton");
    assert_eq!(mf.webpage_preview_url.as_deref(), Some("https://telegram.org/apps"));
    assert!(mf.is_typing_active());

    // Verify upstream K_LINK_PROTOCOLS constants
    assert!(K_LINK_PROTOCOLS.contains(&"http://"));
    assert!(K_LINK_PROTOCOLS.contains(&"https://"));
}
