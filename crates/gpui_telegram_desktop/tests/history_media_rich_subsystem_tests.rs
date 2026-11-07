//! Subsystem & Adversarial Oracle Tests for Module 06 Slice 6 (History Rich Media Views)
//! Tests upstream C++ invariants from `Telegram/SourceFiles/history/view/media/`.

use gpui_telegram_desktop::views::stage::media::poll::*;
use gpui_telegram_desktop::views::stage::media::web_page::*;
use gpui_telegram_desktop::views::stage::media::invoice::*;
use gpui_telegram_desktop::views::stage::media::transfer::*;
use gpui_telegram_desktop::views::stage::media::todo::*;
use gpui_telegram_desktop::views::stage::media::gift::*;
use gpui_telegram_desktop::views::stage::media::location::*;
use gpui_telegram_desktop::views::stage::view::media::*;
use gpui_telegram_desktop::views::stage::view::media::history_view_suggest_decision::DecisionEmoji;

#[test]
fn test_count_nice_percent_rounding_and_edge_cases() {
    // Unequal split where remainder distributes to higher remainder:
    // 2 votes vs 1 vote out of 3 total votes -> 67% and 33% (sum 100%)
    let votes_unequal = [2, 1];
    let percents_unequal = count_nice_percent(&votes_unequal, 3);
    assert_eq!(percents_unequal, vec![67, 33]);
    let sum_unequal: i32 = percents_unequal.iter().sum();
    assert_eq!(sum_unequal, 100);

    // Exact equal split where remainder cannot be evenly divided among equal items:
    // 3 options with 1 vote each out of 3 total votes -> 33% each to avoid unfair bias
    let votes = [1, 1, 1];
    let percents = count_nice_percent(&votes, 3);
    assert_eq!(percents, vec![33, 33, 33]);

    // Zero total votes returns zero vector
    let zero_res = count_nice_percent(&[0, 0, 0], 0);
    assert_eq!(zero_res, vec![0, 0, 0]);

    // Asymmetric distribution: 50, 25, 25 out of 100
    let res2 = count_nice_percent(&[50, 25, 25], 100);
    assert_eq!(res2, vec![50, 25, 25]);

    // 4 options with 3 votes for 3 different items (C++ test case in L156-L160)
    // 0-vote option must never receive a remainder point!
    let four_options = [1, 1, 1, 0];
    let res3 = count_nice_percent(&four_options, 3);
    assert_eq!(res3, vec![33, 33, 33, 0]);
    assert_eq!(res3[3], 0);
}

#[test]
fn test_hash_poll_shuffle_value_crc32() {
    let h1 = hash_poll_shuffle_value(1001, 2002, b"Option A");
    let h2 = hash_poll_shuffle_value(1001, 2002, b"Option B");
    let h1_repeat = hash_poll_shuffle_value(1001, 2002, b"Option A");

    assert_eq!(h1, h1_repeat);
    assert_ne!(h1, h2);
    assert!(h1 > 0);
}

#[test]
fn test_slot_machine_reel_decoding_and_jackpot() {
    // Value 64: 777 Jackpot
    let jackpot = decode_slot_machine_reels(64);
    assert_eq!(jackpot, [SlotReelSymbol::Seven, SlotReelSymbol::Seven, SlotReelSymbol::Seven]);

    // Value 1: BAR, BAR, BAR
    let r1 = decode_slot_machine_reels(1);
    assert_eq!(r1, [SlotReelSymbol::Bar, SlotReelSymbol::Bar, SlotReelSymbol::Bar]);

    // Value 2: Berries, BAR, BAR
    let r2 = decode_slot_machine_reels(2);
    assert_eq!(r2, [SlotReelSymbol::Berries, SlotReelSymbol::Bar, SlotReelSymbol::Bar]);

    // Value 3: Lemon, BAR, BAR
    let r3 = decode_slot_machine_reels(3);
    assert_eq!(r3, [SlotReelSymbol::Lemon, SlotReelSymbol::Bar, SlotReelSymbol::Bar]);

    // Value 4: Seven, BAR, BAR
    let r4 = decode_slot_machine_reels(4);
    assert_eq!(r4, [SlotReelSymbol::Seven, SlotReelSymbol::Bar, SlotReelSymbol::Bar]);
}

#[test]
fn test_custom_emoji_dimension_scale() {
    assert_eq!(custom_emoji_dimension_scale(1), 1.0);
    assert_eq!(custom_emoji_dimension_scale(2), 0.7);
    assert_eq!(custom_emoji_dimension_scale(3), 0.52);
    assert_eq!(custom_emoji_dimension_scale(4), 0.40);
    assert_eq!(custom_emoji_dimension_scale(5), 0.40);
    assert_eq!(custom_emoji_dimension_scale(6), 0.32);
}

#[test]
fn test_article_thumb_dimensions() {
    // 200x100 thumbnail clamped to target height 80
    let w = article_thumb_width(200, 100, 80);
    assert_eq!(w, 80); // clamped to height

    // 100x200 thumbnail with target height 80
    let w2 = article_thumb_width(100, 200, 80);
    assert_eq!(w2, 40);

    // Height calculation with target width 120
    let h = article_thumb_height(100, 200, 120);
    assert_eq!(h, 240);
}

#[test]
fn test_page_to_phrase_mapping() {
    assert_eq!(page_to_phrase(WebPageType::Article, true), "INSTANT VIEW");
    assert_eq!(page_to_phrase(WebPageType::InstantView, false), "INSTANT VIEW");
    assert_eq!(page_to_phrase(WebPageType::Theme, false), "VIEW THEME");
    assert_eq!(page_to_phrase(WebPageType::Story, false), "VIEW STORY");
    assert_eq!(page_to_phrase(WebPageType::Channel, false), "VIEW CHANNEL");
    assert_eq!(page_to_phrase(WebPageType::Group, false), "VIEW GROUP");
    assert_eq!(page_to_phrase(WebPageType::Bot, false), "OPEN BOT");
    assert_eq!(page_to_phrase(WebPageType::Article, false), "OPEN LINK");
}

#[test]
fn test_invoice_credits_and_receipt_detection() {
    assert!(is_ordinary_unpaid_credits_invoice("XTR", 0, false, false));
    // Test invoice should not be ordinary unpaid
    assert!(!is_ordinary_unpaid_credits_invoice("XTR", 0, true, false));
    // Receipt invoice should not be unpaid
    assert!(!is_ordinary_unpaid_credits_invoice("XTR", 555, false, false));
    // Other currency should not be credits
    assert!(!is_ordinary_unpaid_credits_invoice("USD", 0, false, false));

    assert_eq!(format_amount_and_currency(50, "XTR"), "⭐ 50");
    assert_eq!(format_amount_and_currency(499, "USD"), "4.99 USD");
}

#[test]
fn test_call_duration_oracle() {
    assert_eq!(compute_call_duration(CallState::Incoming, 120), 120);
    assert_eq!(compute_call_duration(CallState::Outgoing, 45), 45);
    // Missed and Busy calls must force duration to 0
    assert_eq!(compute_call_duration(CallState::Missed, 30), 0);
    assert_eq!(compute_call_duration(CallState::Busy, 15), 0);
}

#[test]
fn test_ton_transfer_address_groups_and_bump_curve() {
    let addr = "EQD4FPq-PRDieyQKkKZjmNu4949494";
    let groups = format_ton_address_groups(addr);
    assert_eq!(groups.len(), 6);
    assert_eq!(groups[0], "EQD4");
    assert_eq!(groups[1], "FPq-");

    assert_eq!(format_nano_ton(5_000_000_000), "5.00 TON");
    assert_eq!(format_nano_ton(1_250_000_000), "1.25 TON");

    assert_eq!(K_SETTLE_BUMP.bump_duration(), 150 + 33 + 200 + 200);
    assert_eq!(K_READ_PRESS.bump_duration(), 33 + 0 + 167 + 300);
}

#[test]
fn test_todo_list_card_state_and_completion() {
    let mut todo = RichTodoListCard::new("Sprint Tasks");
    todo.add_task(1, "Fix Poll Bug", false);
    todo.add_task(2, "Review PR", false);

    assert_eq!(todo.total_count(), 2);
    assert_eq!(todo.completed_count(), 0);
    assert!(!todo.is_all_completed());

    todo.toggle_task(1, Some("Alice"));
    assert_eq!(todo.completed_count(), 1);
    assert!(!todo.is_all_completed());

    todo.toggle_task(2, Some("Bob"));
    assert_eq!(todo.completed_count(), 2);
    assert!(todo.is_all_completed());
}

#[test]
fn test_remaining_time_text_and_progress() {
    assert_eq!(remaining_time_text(0, K_UNTIL_OFF_PERIOD), "∞");
    assert_eq!(remaining_time_progress(0, K_UNTIL_OFF_PERIOD), 1.0);

    // 5 seconds remaining
    assert_eq!(remaining_time_text(55, 60), "5s");
    // 5 minutes remaining
    assert_eq!(remaining_time_text(300, 600), "5m");
    // 2 hours remaining
    assert_eq!(remaining_time_text(3600, 10800), "2h");
    // 2 days remaining
    assert_eq!(remaining_time_text(86400, 259200), "2d");

    let prog = remaining_time_progress(50, 100);
    assert!((prog - 0.5).abs() < 1e-6);
}

#[test]
fn test_star_gift_and_unique_collectible_cards() {
    let star_gift = RichStarGiftCard::new_star_gift("Birthday Star", 250);
    assert_eq!(star_gift.gift_type, GiftType::StarGift);
    assert_eq!(star_gift.num_stars, 250);

    let collectible = RichStarGiftCard::new_unique_collectible("Diamond Ring", 7, 500);
    assert_eq!(collectible.gift_type, GiftType::UniqueCollectible);
    assert_eq!(collectible.collectible_number, Some(7));
    assert_eq!(collectible.total_issued, Some(500));
}

#[test]
fn test_decision_emoji_symbols() {
    assert_eq!(DecisionEmoji::Agreement.as_str(), "🤝");
    assert_eq!(DecisionEmoji::Calendar.as_str(), "📅");
    assert_eq!(DecisionEmoji::Money.as_str(), "💰");
    assert_eq!(DecisionEmoji::Hourglass.as_str(), "⌛");
    assert_eq!(DecisionEmoji::Reload.as_str(), "🔄");
    assert_eq!(DecisionEmoji::Decline.as_str(), "❌");
    assert_eq!(DecisionEmoji::Discard.as_str(), "🚫");
    assert_eq!(DecisionEmoji::Warning.as_str(), "⚠️");
}

#[test]
fn test_genuine_media_views_initialization() {
    let mut poll_view = HistoryViewPollViewMedia::new(101, "Preferred Language?", PollType::Regular);
    poll_view.add_answer(1, "Rust", 40, false);
    poll_view.add_answer(2, "C++", 30, false);
    poll_view.add_answer(3, "Python", 30, false);
    assert_eq!(poll_view.total_votes, 100);
    assert_eq!(poll_view.answers[0].percent, 40);

    poll_view.vote(1);
    assert_eq!(poll_view.total_votes, 101);
    assert_eq!(poll_view.selected_option, Some(1));

    let mut web_view = HistoryViewWebPageViewMedia::new("https://telegram.org", "Telegram");
    web_view.set_factcheck("Verified official link", "US");
    assert!(web_view.factcheck.is_some());
    assert!(!web_view.factcheck.as_ref().unwrap().is_expanded);
    web_view.toggle_factcheck();
    assert!(web_view.factcheck.as_ref().unwrap().is_expanded);

    let call_view = HistoryViewCallViewMedia::new(CallState::Incoming, 75, true);
    assert_eq!(call_view.duration_seconds, 75);
    assert!(call_view.is_video);

    let dice_view = HistoryViewDiceViewMedia::new("🎲", 5).with_stake_outcome(2_000_000_000, 1_000_000_000);
    assert_eq!(dice_view.value, 5);
    assert_eq!(dice_view.outcome_nano_ton, 2_000_000_000);

    let slot_view = HistoryViewSlotMachineViewMedia::new(64);
    assert!(slot_view.is_jackpot());
    assert_eq!(slot_view.reels(), [SlotReelSymbol::Seven, SlotReelSymbol::Seven, SlotReelSymbol::Seven]);
}
