//! Integration tests for Module 07 Slice 2: Stickers, Bot Keyboards & Inline Bots.
//!
//! Verifies:
//! - Stickers grid layout, modes, search, and recent limit (20)
//! - Footer section ID arithmetic and tab navigation
//! - Lottie sticker size tagging, area threshold, and cache bit-shift math
//! - Large emoji pack, skin tone modifiers, and index extraction
//! - Double outline diagonal sqrt(2) distance correction
//! - Dice pack unicode constants, slot machine flag, and game index mapping
//! - Gift box star tier conversion, premium/ton divider binary search
//! - Bot command chat wrapping (@username suffix) and filtering
//! - Bot reply keyboard matrix geometry, flags, and natural height
//! - Forwarding phrase factory localized string selection and icon resolution
//! - Rich paste format detection, tag offset range shifting, and offer throttling
//! - Spellchecker manifest parsing and LWC locale packing
//! - Inline bot attach webview lifecycle and peer type flags
//! - Inline bot prepared message confirmation dialog state
//! - Inline bot downloads state machine and action dispatch
//! - Inline bot layout internal mosaic aspect ratio bounds and row packing
//! - Inline bot 5MB cloud storage limit and key hashing
//! - Inline bot results widget 400ms debounce and responsive height math

use gpui_telegram_desktop::components::bots::bot_attach_web_view::{
    peer_types, peer_matches_types, AttachBotEntry, BotAttachWebView, K_POPULAR_APP_BOTS_LIMIT,
    K_PROLONG_TIMEOUT_SECS, K_REFRESH_BOTS_TIMEOUT_SECS,
};
use gpui_telegram_desktop::components::bots::inline_bot_confirm_prepared::{
    InlineBotConfirmPrepared, PreparedMessageInfo,
};
use gpui_telegram_desktop::components::bots::inline_bot_downloads::{
    DownloadAction, DownloadState, InlineBotDownloads, K_DOWNLOADS_VERSION,
    K_MAX_DOWNLOADS_BOTS, K_MAX_DOWNLOADS_PER_BOT,
};
use gpui_telegram_desktop::components::bots::inline_bot_layout_internal::{
    InlineBotLayoutInternal, InlineLayoutKind, LayoutItemInput, MAX_ASPECT_RATIO, MIN_ASPECT_RATIO,
};
use gpui_telegram_desktop::components::bots::inline_bot_layout_item::InlineBotLayoutItem;
use gpui_telegram_desktop::components::bots::inline_bot_result::{
    parse_result_type, InlineBotResult, InlineResultType, K_VIDEO_THUMB_MIME,
};
use gpui_telegram_desktop::components::bots::inline_bot_send_data::{
    InlineBotPayload, InlineBotSendData, InlineSendError,
};
use gpui_telegram_desktop::components::bots::inline_bot_storage::{
    compute_storage_key_hash, InlineBotStorage, K_MAX_STORAGE_SIZE,
};
use gpui_telegram_desktop::components::bots::inline_results_inner::{
    InlineCacheEntry, InlineResultsInner, K_MIN_AFTER_SCROLL_DELAY_MS, K_MIN_REPAINT_DELAY_MS,
};
use gpui_telegram_desktop::components::bots::inline_results_widget::{
    compute_results_height, InlineResultsWidget, K_EMOJI_PAN_HEIGHT_RATIO,
    K_INLINE_BOT_REQUEST_DELAY_MS, K_INLINE_RESULTS_MAX_HEIGHT, K_INLINE_RESULTS_MIN_HEIGHT,
};
use gpui_telegram_desktop::views::composer::bot_command::{
    wrap_command_in_chat, BotCommand, SendCommandRequest,
};
use gpui_telegram_desktop::views::composer::bot_keyboard::{
    flags as kb_flags, BotKeyboard, BotKeyboardButton, BotKeyboardButtonKind,
};
use gpui_telegram_desktop::views::composer::rich_paste_toast::{
    replace_text_with_tags, RichPasteOffer, RichPasteOfferThrottle, RichPasteToast, TaggedText,
    TextTag, K_OFFER_TIMEOUT_MS, K_TOAST_DURATION_MS,
};
use gpui_telegram_desktop::views::composer::share_message_phrase_factory::{
    format_forwarded_phrase, forwarded_phrase_icon, ForwardedPhraseArgs, ShareMessagePhraseFactory,
};
use gpui_telegram_desktop::views::composer::spellchecker_common::{
    encode_lwc, parse_manifest_location, DictionaryEntry, SpellcheckerCommon, DICT_EXTENSIONS,
    MANIFEST_CHANNEL, MANIFEST_POST_ID,
};
use gpui_telegram_desktop::views::composer::stickers_dice_pack::{
    is_slot, parse_dice_index, DiceGameKind, StickersDicePack, BBALL_STRING, DART_STRING,
    DICE_STRING, FBALL_STRING, PARTY_POPPER, SLOT_STRING,
};
use gpui_telegram_desktop::views::composer::stickers_emoji_image_loader::{
    corrected_diagonal_distance, OutlineMetrics, StickersEmojiImageLoader, DIAGONAL_SHIFTS,
    PLANAR_SHIFTS,
};
use gpui_telegram_desktop::views::composer::stickers_emoji_pack::{
    index_from_emoticon, EmojiAsset, SkinModifier, StickersEmojiPack, K_EMOJI_CACHES_COUNT,
    K_PREMIUM_CACHES_COUNT, K_REFRESH_TIMEOUT_SECS, K_STANDARD_EMOJI_SET_NAME,
};
use gpui_telegram_desktop::views::composer::stickers_gift_box_pack::{
    find_divider_index, months_for_stars, GiftBoxItem, StickersGiftBoxPack, PREMIUM_DIVIDERS,
    TON_DIVIDERS,
};
use gpui_telegram_desktop::views::composer::stickers_list_footer::{
    all_emoji_section_set_id, emoji_section_set_id, recent_emoji_section_set_id,
    search_emoji_section_set_id, set_id_emoji_section, StickerIconData, StickersListFooter,
    ValidateIconAnimations, K_EMOJI_SEARCH_LIMIT, K_EMOJI_SECTION_SET_ID_BASE,
};
use gpui_telegram_desktop::views::composer::stickers_list_widget::{
    StickerItem, StickerSet, StickersListMode, StickersListWidget, K_MIN_REPAINT_DELAY_MS as K_STICKER_REPAINT_MS,
    K_OFFICIAL_LOAD_LIMIT, K_PRELOAD_OFFICIAL_PAGES, K_RECENT_DISPLAY_LIMIT,
    K_SEARCH_REQUEST_DELAY_MS as K_STICKER_SEARCH_DELAY_MS,
};

#[test]
fn test_stickers_list_widget_modes_and_search() {
    assert_eq!(K_STICKER_SEARCH_DELAY_MS, 400);
    assert_eq!(K_RECENT_DISPLAY_LIMIT, 20);
    assert_eq!(K_PRELOAD_OFFICIAL_PAGES, 4);
    assert_eq!(K_OFFICIAL_LOAD_LIMIT, 40);
    assert_eq!(K_STICKER_REPAINT_MS, 33);

    let mut widget = StickersListWidget::new();
    assert_eq!(widget.mode, StickersListMode::Full);

    widget.set_mode(StickersListMode::Masks);
    assert_eq!(widget.mode, StickersListMode::Masks);

    let mut pack = StickerSet::new(101, "Animals", "animals_pack");
    pack.stickers.push(StickerItem {
        id: 1,
        document_id: 1001,
        alt_emoticon: "🐱".to_string(),
        is_premium: false,
        is_lottie: true,
        is_webm: false,
        pack_id: 101,
        thumbnail_url: None,
    });
    pack.stickers.push(StickerItem {
        id: 2,
        document_id: 1002,
        alt_emoticon: "🐶".to_string(),
        is_premium: true,
        is_lottie: false,
        is_webm: true,
        pack_id: 101,
        thumbnail_url: None,
    });
    widget.add_set(pack);
    assert_eq!(widget.active_stickers_count(), 2);

    // Search query matching
    widget.set_search_query("🐱");
    let matched = widget.prepare_stickers(false);
    assert_eq!(matched.len(), 1);
    assert_eq!(matched[0].alt_emoticon, "🐱");

    // Skip premium filter
    widget.set_search_query("");
    let non_premium = widget.prepare_stickers(true);
    assert_eq!(non_premium.len(), 1);
    assert_eq!(non_premium[0].id, 1);

    // Recent sticker insertion and cap
    for i in 0..25 {
        widget.add_recent(StickerItem {
            id: i,
            document_id: 2000 + i,
            alt_emoticon: "✨".to_string(),
            is_premium: false,
            is_lottie: false,
            is_webm: false,
            pack_id: 101,
            thumbnail_url: None,
        });
    }
    assert_eq!(widget.recent_stickers.len(), K_RECENT_DISPLAY_LIMIT);
}

#[test]
fn test_stickers_list_footer_section_ids() {
    assert_eq!(K_EMOJI_SECTION_SET_ID_BASE, 0x77FF_FFFF_FFFF_FFF0);
    assert_eq!(K_EMOJI_SEARCH_LIMIT, 32);

    let recent = recent_emoji_section_set_id();
    let all = all_emoji_section_set_id();
    let search = search_emoji_section_set_id();

    assert_eq!(all, K_EMOJI_SECTION_SET_ID_BASE);
    assert_eq!(recent, K_EMOJI_SECTION_SET_ID_BASE + 1);
    assert_eq!(search, K_EMOJI_SECTION_SET_ID_BASE + 10);

    assert_eq!(set_id_emoji_section(recent), Some(0));
    assert_eq!(set_id_emoji_section(all), None); // Under base+1
    assert_eq!(set_id_emoji_section(recent + 5), Some(5));

    let mut footer = StickersListFooter::new();
    footer.add_icon(StickerIconData {
        set_id: 1,
        title: "Pack 1".to_string(),
        is_megagroup: false,
        has_lottie: true,
        has_webm: false,
        pixw: 32,
        pixh: 32,
    });
    footer.add_icon(StickerIconData {
        set_id: 2,
        title: "Pack 2".to_string(),
        is_megagroup: false,
        has_lottie: false,
        has_webm: true,
        pixw: 32,
        pixh: 32,
    });

    assert_eq!(footer.active_set_id, Some(1));
    footer.select_next();
    assert_eq!(footer.active_set_id, Some(2));
    footer.select_prev();
    assert_eq!(footer.active_set_id, Some(1));
}

#[test]
fn test_stickers_lottie_cache_key_shift_and_sizes() {
    use gpui_telegram_desktop::views::composer::stickers_lottie::{
        local_sticker_id, lottie_cache_key_shift, StickerLottieSize, StickersLottie,
        K_DONT_CACHE_LOTTIE_AFTER_AREA,
    };

    assert_eq!(K_DONT_CACHE_LOTTIE_AFTER_AREA, 512 * 512);

    // Bit-shift test: ((replacementsTag << 4) & 0xF0) | (sizeTag & 0x0F)
    let shift = lottie_cache_key_shift(0x03, StickerLottieSize::StickersPanel);
    let expected = ((3 << 4) & 0xF0) | (2 & 0x0F);
    assert_eq!(shift, expected);

    // Stable 64-bit local sticker hash test
    let id1 = local_sticker_id("dice_0");
    let id2 = local_sticker_id("dice_0");
    let id3 = local_sticker_id("dice_1");
    assert_eq!(id1, id2);
    assert_ne!(id1, id3);

    let mut lottie = StickersLottie::new(StickerLottieSize::StickersPanel);
    lottie.total_frames = 10;
    assert_eq!(lottie.advance_frame(), 1);
    assert_eq!(lottie.advance_frame(), 2);
    lottie.reset();
    assert_eq!(lottie.current_frame, 0);

    assert!(StickersLottie::should_cache_box(256, 256));
    assert!(!StickersLottie::should_cache_box(1024, 1024));
}

#[test]
fn test_stickers_emoji_pack_and_skin_modifiers() {
    assert_eq!(K_STANDARD_EMOJI_SET_NAME, "RestrictedEmoji");
    assert_eq!(K_REFRESH_TIMEOUT_SECS, 7200);
    assert_eq!(K_EMOJI_CACHES_COUNT, 4);
    assert_eq!(K_PREMIUM_CACHES_COUNT, 8);

    assert_eq!(index_from_emoticon("1"), Some(0));
    assert_eq!(index_from_emoticon("5"), Some(4));
    assert_eq!(index_from_emoticon("9"), Some(8));
    assert_eq!(index_from_emoticon("\u{1F61F}"), Some(9));
    assert_eq!(index_from_emoticon("abc"), None);

    let mut pack = StickersEmojiPack::new("CustomEmoji");
    pack.insert_emoji(EmojiAsset {
        emoticon: "👍".to_string(),
        document_id: 555,
        skin_modifier: Some(SkinModifier::Color3),
        is_premium: false,
    });
    assert_eq!(pack.lookup("👍").map(|a| a.document_id), Some(555));
    assert!(!pack.needs_refresh());
}

#[test]
fn test_stickers_emoji_image_loader_outline_math() {
    assert_eq!(PLANAR_SHIFTS.len(), 4);
    assert_eq!(DIAGONAL_SHIFTS.len(), 4);

    // Check diagonal distance step via SafeRound(delta / sqrt(2))
    assert_eq!(corrected_diagonal_distance(1), 1);
    assert_eq!(corrected_diagonal_distance(2), 1);
    assert_eq!(corrected_diagonal_distance(3), 2);
    assert_eq!(corrected_diagonal_distance(4), 3);

    let loader = StickersEmojiImageLoader::new().with_metrics(OutlineMetrics::new(64.0, 2.0, 1.0));
    let offsets = loader.compute_outline_offsets();
    // 4 planar * 2 steps + 4 diagonal * 1 step = 8 + 4 = 12
    assert_eq!(offsets.len(), 12);
}

#[test]
fn test_stickers_dice_pack_game_kinds() {
    assert_eq!(DICE_STRING, "🎲");
    assert_eq!(DART_STRING, "🎯");
    assert_eq!(SLOT_STRING, "🎰");
    assert_eq!(FBALL_STRING, "⚽");
    assert_eq!(BBALL_STRING, "🏀");
    assert_eq!(PARTY_POPPER, "🎉");

    assert!(is_slot(SLOT_STRING));
    assert!(!is_slot(DICE_STRING));

    assert_eq!(DiceGameKind::from_emoji(DICE_STRING), Some(DiceGameKind::Dice));
    assert_eq!(DiceGameKind::from_emoji(SLOT_STRING), Some(DiceGameKind::Slot));
    assert_eq!(DiceGameKind::Slot.max_value(), 64);
    assert_eq!(DiceGameKind::Dice.max_value(), 6);

    assert_eq!(parse_dice_index('#'), Some(0));
    assert_eq!(parse_dice_index('1'), Some(1));
    assert_eq!(parse_dice_index('6'), Some(6));
    assert_eq!(parse_dice_index('9'), None);

    let mut pack = StickersDicePack::new(DICE_STRING);
    pack.bind_document(3, 9999);
    assert_eq!(pack.lookup(3), Some(9999));
}

#[test]
fn test_stickers_gift_box_pack_stars_and_dividers() {
    assert_eq!(PREMIUM_DIVIDERS, &[1, 3, 6, 12, 24]);
    assert_eq!(TON_DIVIDERS, &[0, 10, 50]);

    assert_eq!(months_for_stars(500), 3);
    assert_eq!(months_for_stars(1000), 3);
    assert_eq!(months_for_stars(1500), 6);
    assert_eq!(months_for_stars(2500), 12);
    assert_eq!(months_for_stars(5000), 12);

    // Exact and nearest divider search
    assert_eq!(find_divider_index(PREMIUM_DIVIDERS, 3, true), 1);
    // Upstream tie-breaker in GiftBoxPack::lookup: abs(2 - 1) < abs(2 - 3) is false, preserves it pointing to 3 (index 1)
    assert_eq!(find_divider_index(PREMIUM_DIVIDERS, 2, false), 1);
    assert_eq!(find_divider_index(PREMIUM_DIVIDERS, 5, false), 2); // 5 is closer to 6

    let mut pack = StickersGiftBoxPack::new();
    pack.add_premium_gift(GiftBoxItem {
        divider: 1,
        document_id: 111,
        title: "1 Month".to_string(),
    });
    pack.add_premium_gift(GiftBoxItem {
        divider: 3,
        document_id: 333,
        title: "3 Months".to_string(),
    });
    assert_eq!(pack.lookup_premium_by_stars(500).map(|g| g.document_id), Some(333));
}

#[test]
fn test_bot_command_wrapping() {
    let plain = "/start";
    assert_eq!(wrap_command_in_chat(plain, "my_bot", false), "/start");
    assert_eq!(wrap_command_in_chat(plain, "my_bot", true), "/start@my_bot");
    assert_eq!(wrap_command_in_chat("/start@other_bot", "my_bot", true), "/start@other_bot");

    let mut cmd = BotCommand::new();
    cmd.register_command("help", "Get assistance", "helper_bot");
    cmd.register_command("settings", "Configure bot", "helper_bot");
    cmd.register_command("status", "Check system", "system_bot");

    cmd.set_filter("s");
    let filtered = cmd.filtered_commands();
    assert_eq!(filtered.len(), 2); // settings, status
}

#[test]
fn test_bot_keyboard_geometry_and_flags() {
    let mut kb = BotKeyboard::new();
    assert!(!kb.has_markup());

    let rows = vec![
        vec![
            BotKeyboardButton::new("Option A", BotKeyboardButtonKind::Default),
            BotKeyboardButton::new("Option B", BotKeyboardButtonKind::Default),
        ],
        vec![
            BotKeyboardButton::new("Web App", BotKeyboardButtonKind::WebApp("https://app.t.me".to_string())),
        ],
    ];

    kb.set_markup(rows, kb_flags::RESIZE | kb_flags::SINGLE_USE, Some("Choose:".to_string()));
    assert!(kb.has_markup());
    assert!(kb.single_use);
    assert!(!kb.maximize_size);
    assert_eq!(kb.placeholder, Some("Choose:".to_string()));

    let nat_h = kb.natural_height();
    assert!(nat_h > 0.0);
    let calc_h = kb.calculated_height();
    assert!(calc_h <= kb.max_outer_height);
}

#[test]
fn test_share_message_phrase_factory() {
    let single_saved = ForwardedPhraseArgs {
        to_count: 1,
        single_message: true,
        to1_name: Some("Saved Messages".to_string()),
        to1_is_self: true,
        to2_name: None,
        to_self_with_premium_is_empty: false,
    };
    assert_eq!(format_forwarded_phrase(&single_saved), "Message forwarded to Saved Messages");
    assert_eq!(forwarded_phrase_icon(&single_saved), "toast/saved_messages");

    let multi_chat = ForwardedPhraseArgs {
        to_count: 1,
        single_message: false,
        to1_name: Some("Rust Developers".to_string()),
        to1_is_self: false,
        to2_name: None,
        to_self_with_premium_is_empty: false,
    };
    assert_eq!(format_forwarded_phrase(&multi_chat), "Messages forwarded to Rust Developers");
    assert_eq!(forwarded_phrase_icon(&multi_chat), "toast/forward");

    let two_chats = ForwardedPhraseArgs {
        to_count: 2,
        single_message: true,
        to1_name: Some("Alice".to_string()),
        to1_is_self: false,
        to2_name: Some("Bob".to_string()),
        to_self_with_premium_is_empty: false,
    };
    assert_eq!(format_forwarded_phrase(&two_chats), "Message forwarded to Alice and Bob");

    let many_chats = ForwardedPhraseArgs {
        to_count: 5,
        single_message: true,
        to1_name: None,
        to1_is_self: false,
        to2_name: None,
        to_self_with_premium_is_empty: false,
    };
    assert_eq!(format_forwarded_phrase(&many_chats), "Message forwarded to 5 chats");
}

#[test]
fn test_rich_paste_toast_tag_offsets_and_throttle() {
    assert_eq!(K_TOAST_DURATION_MS, 9000);
    assert_eq!(K_OFFER_TIMEOUT_MS, 30000);

    let original = TaggedText {
        text: "Hello World and Telegram".to_string(),
        tags: vec![
            TextTag { offset: 0, length: 5, id: "bold".to_string() },
            TextTag { offset: 16, length: 8, id: "italic".to_string() },
        ],
    };

    let replacement = TaggedText {
        text: "Rust Community".to_string(),
        tags: vec![TextTag { offset: 0, length: 4, id: "code".to_string() }],
    };

    // Replace "World" (idx 6..11) with "Rust Community" (len 14, shift = +9)
    let result = replace_text_with_tags(&original, 6, 11, &replacement);
    assert_eq!(result.text, "Hello Rust Community and Telegram");
    assert_eq!(result.tags.len(), 3);
    assert_eq!(result.tags[0].offset, 0); // "Hello" unaffected
    assert_eq!(result.tags[1].offset, 6); // "Rust" inserted tag
    assert_eq!(result.tags[2].offset, 25); // "Telegram" shifted: 16 + (14 - 5) = 25

    let mut throttle = RichPasteOfferThrottle::default();
    assert!(throttle.take(100_000));
    assert!(!throttle.take(110_000)); // Within 30s
    assert!(throttle.take(131_000)); // After 31s
}

#[test]
fn test_spellchecker_manifest_and_lwc() {
    assert_eq!(DICT_EXTENSIONS, &["dic", "aff"]);
    assert_eq!(MANIFEST_CHANNEL, "tdhbcfiles");
    assert_eq!(MANIFEST_POST_ID, 2949);

    assert_eq!(encode_lwc(10, 1, true), 10);
    assert_eq!(encode_lwc(10, 2, false), 10002);

    let parsed = parse_manifest_location("tdhbcfiles#2949");
    assert_eq!(parsed, Some(("tdhbcfiles".to_string(), 2949)));
    assert_eq!(parse_manifest_location("invalid"), None);

    let mut sc = SpellcheckerCommon::new();
    sc.enable_language(1);
    sc.enable_language(2);
    assert_eq!(sc.active_lang_ids.len(), 2);
    sc.disable_language(1);
    assert_eq!(sc.active_lang_ids.len(), 1);
}

#[test]
fn test_bot_attach_web_view_lifecycle() {
    assert_eq!(K_PROLONG_TIMEOUT_SECS, 60);
    assert_eq!(K_REFRESH_BOTS_TIMEOUT_SECS, 3600);
    assert_eq!(K_POPULAR_APP_BOTS_LIMIT, 100);

    let allowed = peer_types::USER | peer_types::GROUP;
    assert!(peer_matches_types(peer_types::USER, allowed));
    assert!(peer_matches_types(peer_types::GROUP, allowed));
    assert!(!peer_matches_types(peer_types::BROADCAST, allowed));

    let mut view = BotAttachWebView::new();
    let bot = AttachBotEntry {
        bot_id: 42,
        name: "TestMiniApp".to_string(),
        short_name: "test_app".to_string(),
        allowed_peer_types: allowed,
        in_main_menu: true,
        in_attach_menu: true,
        disclaimer_required: true,
        request_write_access: false,
    };

    view.launch(bot, "https://t.me/app");
    assert!(view.is_open);
    assert_eq!(view.url, "https://t.me/app");

    view.accept_disclaimer();
    assert!(view.disclaimer_accepted);

    view.set_fullscreen(true);
    assert!(view.is_fullscreen);

    view.close();
    assert!(!view.is_open);
}

#[test]
fn test_inline_bot_confirm_prepared() {
    let mut confirm = InlineBotConfirmPrepared::new();
    assert!(!confirm.is_presented);

    confirm.present(PreparedMessageInfo {
        bot_name: "SearchBot".to_string(),
        preview_text: "Top 10 Rust Crates".to_string(),
        has_media: false,
        target_peer_id: 777,
        target_thread_id: None,
    });
    assert!(confirm.is_presented);

    let confirmed = confirm.confirm();
    assert!(confirmed.is_some());
    assert!(!confirm.is_presented);
    assert!(confirm.confirmed);
}

#[test]
fn test_inline_bot_downloads_state_machine() {
    assert_eq!(K_DOWNLOADS_VERSION, 1);
    assert_eq!(K_MAX_DOWNLOADS_BOTS, 4096);
    assert_eq!(K_MAX_DOWNLOADS_PER_BOT, 16384);

    let mut mgr = InlineBotDownloads::new();
    let id = mgr.start_download(100, "https://example.com/asset.zip", "/tmp/asset.zip");
    assert_eq!(id, 1);

    mgr.update_progress(id, 50, 100);
    let entry = mgr.downloads.get(&id).unwrap();
    assert_eq!(entry.state, DownloadState::Loading(0.5));

    mgr.apply_action(id, DownloadAction::Pause);
    assert_eq!(mgr.downloads.get(&id).unwrap().state, DownloadState::Paused);

    mgr.apply_action(id, DownloadAction::Resume);
    assert_eq!(mgr.downloads.get(&id).unwrap().state, DownloadState::Loading(0.5));

    mgr.update_progress(id, 100, 100);
    assert_eq!(mgr.downloads.get(&id).unwrap().state, DownloadState::Completed);

    assert_eq!(mgr.list_for_bot(100).len(), 1);
}

#[test]
fn test_inline_bot_layout_mosaic_math() {
    assert_eq!(MIN_ASPECT_RATIO, 0.5);
    assert_eq!(MAX_ASPECT_RATIO, 2.5);

    let mut layout = InlineBotLayoutInternal::new(320.0);
    let items = vec![
        LayoutItemInput {
            id: "item1".to_string(),
            kind: InlineLayoutKind::Photo,
            raw_width: 200.0,
            raw_height: 200.0,
        },
        LayoutItemInput {
            id: "item2".to_string(),
            kind: InlineLayoutKind::Gif,
            raw_width: 300.0,
            raw_height: 150.0,
        },
    ];

    layout.compute_gallery_layout(&items);
    assert_eq!(layout.computed_rects.len(), 2);
    assert!(layout.total_height > 0.0);

    let hit_item = InlineBotLayoutItem::new("item1", 0.0, 0.0, 100.0, 100.0);
    assert!(hit_item.item_state.contains_point(50.0, 50.0));
    assert!(!hit_item.item_state.contains_point(150.0, 50.0));
}

#[test]
fn test_inline_bot_result_types_and_send_data() {
    assert_eq!(K_VIDEO_THUMB_MIME, "video/mp4");
    assert_eq!(parse_result_type("photo"), InlineResultType::Photo);
    assert_eq!(parse_result_type("video"), InlineResultType::Video);
    assert_eq!(parse_result_type("voice"), InlineResultType::Audio);
    assert_eq!(parse_result_type("gif"), InlineResultType::Gif);
    assert_eq!(parse_result_type("unknown_type"), InlineResultType::Unknown);

    let res = InlineBotResult::new("res_1", 999, InlineResultType::Photo);
    assert!(res.is_gallery_type());

    let send_data = InlineBotSendData::new_text("res_1", "Hello from bot");
    assert!(send_data.validate_send(false).is_ok());
    assert_eq!(send_data.validate_send(true), Err(InlineSendError::ChatRestricted));

    let empty_send = InlineBotSendData::new_text("res_2", "   ");
    assert_eq!(empty_send.validate_send(false), Err(InlineSendError::EmptyContent));
}

#[test]
fn test_inline_bot_storage_5mb_limit() {
    assert_eq!(K_MAX_STORAGE_SIZE, 5 * 1024 * 1024);

    let hash1 = compute_storage_key_hash("user_auth_token");
    let hash2 = compute_storage_key_hash("user_auth_token");
    let hash3 = compute_storage_key_hash("other_key");
    assert_eq!(hash1, hash2);
    assert_ne!(hash1, hash3);

    let mut storage = InlineBotStorage::new();
    assert!(storage.write(1, "key1", Some("val1")));
    assert_eq!(storage.read(1, "key1"), Some("val1"));
    assert_eq!(storage.total_size(1), "key1".len() + "val1".len());

    // Clear bot storage
    storage.clear(1);
    assert_eq!(storage.read(1, "key1"), None);
    assert_eq!(storage.total_size(1), 0);
}

#[test]
fn test_inline_results_widget_debounce_and_height() {
    assert_eq!(K_INLINE_BOT_REQUEST_DELAY_MS, 400);
    assert_eq!(K_INLINE_RESULTS_MIN_HEIGHT, 120.0);
    assert_eq!(K_INLINE_RESULTS_MAX_HEIGHT, 360.0);
    assert_eq!(K_EMOJI_PAN_HEIGHT_RATIO, 0.55);
    assert_eq!(K_MIN_REPAINT_DELAY_MS, 33);
    assert_eq!(K_MIN_AFTER_SCROLL_DELAY_MS, 33);

    let h = compute_results_height(600.0, 400.0);
    // 600 * 0.55 = 330.0, clamped between 120 and 360 -> 330.0
    assert_eq!(h, 330.0);

    let mut widget = InlineResultsWidget::new();
    widget.query_inline_bot(42, "gif", "cat");
    assert!(widget.is_visible);
    assert!(widget.is_loading);
    assert_eq!(widget.bot_username, "gif");

    widget.receive_results(InlineCacheEntry {
        next_offset: "offset_2".to_string(),
        switch_pm_text: None,
        switch_pm_start_token: None,
        switch_pm_url: None,
        results: vec![
            InlineBotResult::new("1", 42, InlineResultType::Gif),
            InlineBotResult::new("2", 42, InlineResultType::Gif),
        ],
        is_gallery: true,
    });
    assert!(!widget.is_loading);
    assert_eq!(widget.inner.cache.results.len(), 2);
    assert!(widget.inner.can_load_more());

    widget.inner.select_next();
    assert_eq!(widget.inner.selected_index, Some(1));

    widget.clear();
    assert!(!widget.is_visible);
}
