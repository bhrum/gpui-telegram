//! Subsystem tests for Module 08 Media Boxes, Star Gifts, Stickers & Personalization Layer.
//! Validates 1:1 parity with Telegram Desktop upstream implementations.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use gpui_telegram_desktop::views::boxes::star_gift_box::{
    StarGiftBox, K_PRICE_TAB_ALL, K_PRICE_TAB_COLLECTIBLES, K_PRICE_TAB_MY,
    K_RESELL_PRICE_CACHE_LIFETIME_MS, K_SENT_TOAST_DURATION_MS,
};
use gpui_telegram_desktop::views::boxes::star_gift_preview_box::{
    GiftAttributeTab, StarGiftPreviewBox, K_GIFTS_PER_ROW, K_SWITCH_TIMEOUT_MS,
};
use gpui_telegram_desktop::views::boxes::star_gift_auction_box::{
    BidType, StarGiftAuctionBox, K_AUCTION_ABOUT_SHOWN_PREF, K_BID_PLACED_TOAST_DURATION_MS,
    K_MAX_SHOWN_BID, K_SHOW_TOP_PLACES,
};
use gpui_telegram_desktop::views::boxes::star_gift_resale_box::{
    ResaleGiftsSort, StarGiftResaleBox, K_FILTERS_COUNT, K_RESALE_BOUGHT_TOAST_DURATION_MS,
};
use gpui_telegram_desktop::views::boxes::star_gift_craft_box::{
    CraftSlotItem, StarGiftCraftBox, K_CRAFT_SLOTS,
};
use gpui_telegram_desktop::views::boxes::star_gift_cover_box::{
    StarGiftCoverBox, K_CROSSFADE_DURATION_MS, K_SPINNER_BACKDROPS,
};
use gpui_telegram_desktop::views::boxes::star_gift_craft_animation::{
    CraftAnimationPhase, StarGiftCraftAnimation, K_FLIGHT_DURATION_MS, K_FRAME_DURATION_MS,
};
use gpui_telegram_desktop::views::boxes::transfer_gift_box::{
    TransferGiftBox, TransferTargetMode, K_MOVED_TO_WALLET_TOAST_DURATION_MS,
    K_TMP_PASSWORD_PERIOD_SECS,
};
use gpui_telegram_desktop::views::boxes::sticker_set_box::{
    StickerInstallStatus, StickerSetBox, K_EMOJI_PER_ROW, K_STICKERS_PER_ROW,
};
use gpui_telegram_desktop::views::boxes::stickers_box::{
    StickersBox, StickersSection, K_ARCHIVED_LIMIT_FIRST_REQUEST, K_ARCHIVED_LIMIT_PER_PAGE,
};
use gpui_telegram_desktop::views::boxes::sticker_creator_box::{
    StickerCreatorBox, StickerType, K_MAX_EMOJIS, K_PREVIEW_SIDE, K_STICKER_SIDE,
    K_VIDEO_STICKER_FPS,
};
use gpui_telegram_desktop::views::boxes::language_box::{
    LanguageBox, K_LANGUAGE_NAME_PREFIX, K_TRANSLATE_TO_PREFIX,
};
use gpui_telegram_desktop::views::boxes::reactions_settings_box::{
    ReactionsSettingsBox, K_CENTER_SIZE_MULTIPLIER, K_MAX_LANDING_WAIT_MS,
};
use gpui_telegram_desktop::views::boxes::choose_filter_box::{
    ChooseFilterBox, K_SCALE,
};
use gpui_telegram_desktop::views::boxes::filters::edit_filter_box::{
    ChatFilterModel, EditFilterBoxFilters, K_MAX_FILTER_TITLE_LENGTH,
};
use gpui_telegram_desktop::views::boxes::filters::edit_filter_chats_list::{
    EditFilterChatsListFilters, CHAT_FLAG_CHANNELS, CHAT_FLAG_CONTACTS, CHAT_FLAG_GROUPS,
};
use gpui_telegram_desktop::views::boxes::filters::edit_filter_chats_preview::EditFilterChatsPreviewFilters;
use gpui_telegram_desktop::views::boxes::filters::edit_filter_links::{
    EditFilterLinksFilters, K_MAX_LINK_TITLE_LENGTH,
};
use gpui_telegram_desktop::views::boxes::background_box::{
    BackgroundBox, WallpaperKind, K_BACKGROUNDS_IN_ROW,
};
use gpui_telegram_desktop::views::boxes::background_preview_box::{
    BackgroundPreviewBox, K_MAX_WALL_PAPER_SLUG_LENGTH,
};

#[test]
fn test_star_gift_box_pricing_and_resale_commission() {
    let mut gift_box = StarGiftBox::new(1001, "Bob");
    assert_eq!(gift_box.recipient_id, 1001);
    assert_eq!(gift_box.active_tab, K_PRICE_TAB_ALL);
    assert_eq!(K_PRICE_TAB_COLLECTIBLES, -2);
    assert_eq!(K_PRICE_TAB_MY, -1);
    assert!(!gift_box.is_anonymous);
    assert_eq!(K_SENT_TOAST_DURATION_MS, 3000);
    assert_eq!(K_RESELL_PRICE_CACHE_LIFETIME_MS, 60000);

    // Test TON conversion
    let ton_nanos = 100_000_000_000; // 100 TON
    let stars = StarGiftBox::stars_from_ton(ton_nanos);
    assert_eq!(stars, 5000);
    assert_eq!(StarGiftBox::ton_from_stars(stars), ton_nanos);

    // Test 80/20 Resale Commission Split
    let (proceeds, fee) = StarGiftBox::resale_proceeds_and_fee(1000);
    assert_eq!(proceeds, 800);
    assert_eq!(fee, 200);

    // Test Anonymous toggle and Send callback
    gift_box.toggle_anonymous();
    assert!(gift_box.is_anonymous);

    let sent_id = Arc::new(AtomicU64::new(0));
    let sent_clone = sent_id.clone();
    gift_box.on_send = Some(Arc::new(move |id, anon, _comment| {
        assert!(anon);
        sent_clone.store(id, Ordering::SeqCst);
    }));

    gift_box.select_gift(5);
    gift_box.submit();
    assert!(gift_box.is_closed);
    assert_eq!(sent_id.load(Ordering::SeqCst), 5);
}

#[test]
fn test_star_gift_preview_and_auction_mechanics() {
    let mut preview = StarGiftPreviewBox::new(777, "Cosmic Skull");
    assert_eq!(K_GIFTS_PER_ROW, 3);
    assert_eq!(K_SWITCH_TIMEOUT_MS, 3000);
    assert_eq!(preview.active_tab, GiftAttributeTab::Model);
    assert_eq!(preview.models.len(), 3);

    preview.set_tab(GiftAttributeTab::Pattern);
    assert_eq!(preview.active_tab, GiftAttributeTab::Pattern);

    // Auction tests
    let mut auction = StarGiftAuctionBox::new(888, "Golden Trophy", 1000);
    assert_eq!(K_AUCTION_ABOUT_SHOWN_PREF, "gift_auction_about_shown");
    assert_eq!(K_BID_PLACED_TOAST_DURATION_MS, 5000);
    assert_eq!(K_MAX_SHOWN_BID, 30000);
    assert_eq!(K_SHOW_TOP_PLACES, 3);
    assert_eq!(auction.current_highest_bid, 1000);

    let bid_placed = Arc::new(AtomicBool::new(false));
    let bp_clone = bid_placed.clone();
    auction.on_place_bid = Some(Arc::new(move |amt| {
        assert_eq!(amt, 1500);
        bp_clone.store(true, Ordering::SeqCst);
    }));

    auction.place_bid(1500);
    assert_eq!(auction.current_highest_bid, 1500);
    assert_eq!(auction.my_bid, Some(1500));
    assert_eq!(auction.my_bid_type, BidType::Winning);
    assert_eq!(auction.leaderboard[0].position, 1);
    assert!(auction.leaderboard[0].is_me);
    assert!(bid_placed.load(Ordering::SeqCst));
}

#[test]
fn test_star_gift_resale_and_crafting_dice_forge() {
    let mut resale = StarGiftResaleBox::new(303, "Cyber Skull Market");
    assert_eq!(K_FILTERS_COUNT, 4);
    assert_eq!(K_RESALE_BOUGHT_TOAST_DURATION_MS, 4000);

    let (seller, fee) = StarGiftResaleBox::calculate_commission(2000);
    assert_eq!(seller, 1600);
    assert_eq!(fee, 400);

    resale.set_sort(ResaleGiftsSort::Price);
    assert_eq!(resale.entries[0].price_stars, 1200);

    // Crafting box
    let mut craft = StarGiftCraftBox::new(999, "Dragon Phoenix");
    assert_eq!(K_CRAFT_SLOTS, 4);
    assert_eq!(craft.success_permille, 0);

    craft.put_gift_in_slot(0, CraftSlotItem {
        gift_id: 11,
        title: "Star".into(),
        stars_value: 25,
        icon_emoji: "⭐".into(),
    });
    assert_eq!(craft.success_permille, 250);
    assert_eq!(StarGiftCraftBox::format_percent(craft.success_permille), "25%");

    craft.put_gift_in_slot(1, CraftSlotItem {
        gift_id: 12,
        title: "Rocket".into(),
        stars_value: 50,
        icon_emoji: "🚀".into(),
    });
    assert_eq!(craft.success_permille, 500);
    assert_eq!(StarGiftCraftBox::format_percent(craft.success_permille), "50%");

    craft.remove_gift_from_slot(0);
    assert_eq!(craft.success_permille, 250);

    // Craft animation
    let mut anim = StarGiftCraftAnimation::new("Cosmic Phoenix #42");
    assert_eq!(K_FRAME_DURATION_MS, 16.6667);
    assert_eq!(K_FLIGHT_DURATION_MS, 400);
    assert_eq!(anim.phase, CraftAnimationPhase::Forging);
    anim.set_phase(CraftAnimationPhase::Success);
    assert_eq!(anim.phase, CraftAnimationPhase::Success);

    // Cover box
    let cover = StarGiftCoverBox::default();
    assert_eq!(K_CROSSFADE_DURATION_MS, 400);
    assert_eq!(K_SPINNER_BACKDROPS, 6);
    assert_eq!(cover.gift.number, 42);

    // Transfer box
    let mut transfer = TransferGiftBox::new(
        "Cyber Skull".into(),
        42,
        TransferTargetMode::TelegramUser {
            peer_id: 555,
            name: "Alice".into(),
        },
    );
    assert_eq!(K_MOVED_TO_WALLET_TOAST_DURATION_MS, 5000);
    assert_eq!(K_TMP_PASSWORD_PERIOD_SECS, 600);
    assert_eq!(
        transfer.destination,
        TransferTargetMode::TelegramUser {
            peer_id: 555,
            name: "Alice".into(),
        }
    );
    transfer.set_destination(TransferTargetMode::TonWallet {
        address: "EQBvW8Z5huBkMJYdn3PCDn57NuF64fzCUSn-9Ta4kG".into(),
    });
    assert_eq!(
        transfer.destination,
        TransferTargetMode::TonWallet {
            address: "EQBvW8Z5huBkMJYdn3PCDn57NuF64fzCUSn-9Ta4kG".into(),
        }
    );
    transfer.toggle_keep_on_profile();
    assert!(!transfer.keep_on_profile);
}

#[test]
fn test_sticker_set_fsm_and_creator_dimensions() {
    let mut set_box = StickerSetBox::new(404, "Cool Dogs", "cool_dogs");
    assert_eq!(K_STICKERS_PER_ROW, 5);
    assert_eq!(K_EMOJI_PER_ROW, 8);
    assert_eq!(set_box.status, StickerInstallStatus::NotInstalled);

    // Transition: NotInstalled -> Installed
    set_box.toggle_install();
    assert_eq!(set_box.status, StickerInstallStatus::Installed);

    // Transition: Installed -> NotInstalled
    set_box.toggle_install();
    assert_eq!(set_box.status, StickerInstallStatus::NotInstalled);

    // Stickers box
    let mut stickers_box = StickersBox::new(StickersSection::Installed);
    assert_eq!(K_ARCHIVED_LIMIT_FIRST_REQUEST, 10);
    assert_eq!(K_ARCHIVED_LIMIT_PER_PAGE, 30);
    assert_eq!(stickers_box.section, StickersSection::Installed);
    assert_eq!(stickers_box.sets.len(), 2);

    stickers_box.remove_set(1);
    assert_eq!(stickers_box.sets.len(), 1);

    // Sticker Creator
    let mut creator = StickerCreatorBox::new(StickerType::StandardSticker);
    assert_eq!(K_STICKER_SIDE, 512);
    assert_eq!(K_PREVIEW_SIDE, 256);
    assert_eq!(K_MAX_EMOJIS, 7);
    assert_eq!(K_VIDEO_STICKER_FPS, 30.0);
    assert_eq!(creator.sticker_type, StickerType::StandardSticker);

    creator.add_emoji("😎".into());
    creator.add_emoji("🚀".into());
    assert_eq!(creator.assigned_emojis.len(), 3);
    creator.remove_emoji(0);
    assert_eq!(creator.assigned_emojis.len(), 2);
}

#[test]
fn test_language_and_reactions_settings() {
    let mut lang_box = LanguageBox::new("en");
    assert_eq!(K_LANGUAGE_NAME_PREFIX, "cloud_lng_language_");
    assert_eq!(K_TRANSLATE_TO_PREFIX, "cloud_lng_translate_to_");
    assert_eq!(lang_box.languages.len(), 9);

    // Filter test
    lang_box.search_query = "rus".into();
    let filtered = lang_box.filter_languages();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].id, "ru");

    let switched = Arc::new(AtomicBool::new(false));
    let s_clone = switched.clone();
    lang_box.on_switch = Some(Arc::new(move |id| {
        assert_eq!(id, "ru");
        s_clone.store(true, Ordering::SeqCst);
    }));

    lang_box.switch_language("ru".into());
    assert_eq!(lang_box.current_language_id, "ru");
    assert!(switched.load(Ordering::SeqCst));

    // Reactions settings
    let mut reactions = ReactionsSettingsBox::new();
    assert_eq!(K_CENTER_SIZE_MULTIPLIER, 1.6);
    assert_eq!(K_MAX_LANDING_WAIT_MS, 3000);
    assert_eq!(reactions.quick_reaction, "👍");
    reactions.set_quick_reaction("🔥");
    assert_eq!(reactions.quick_reaction, "🔥");
    reactions.toggle_animations();
    assert!(!reactions.enable_animations);
}

#[test]
fn test_chat_filters_and_wallpapers() {
    let mut choose_filter = ChooseFilterBox::new(999, "Tech News");
    assert_eq!(K_SCALE, 0.75);
    assert_eq!(choose_filter.filters.len(), 4);
    assert!(choose_filter.filters[0].is_chat_included);

    choose_filter.toggle_filter(1);
    assert!(!choose_filter.filters[0].is_chat_included);

    // Edit filter box
    let mut edit_filter = EditFilterBoxFilters::new(ChatFilterModel {
        id: 5,
        title: "VeryLongFilterNameExceedingLimit".into(),
        icon_emoji: "💼".into(),
        color_index: 1,
        flags: 0,
        included_peers_count: 5,
        excluded_peers_count: 1,
    }, true);
    assert_eq!(K_MAX_FILTER_TITLE_LENGTH, 12);
    edit_filter.set_title("VeryLongFilterNameExceedingLimit");
    assert_eq!(edit_filter.filter.title.len(), 12);
    assert_eq!(edit_filter.filter.title, "VeryLongFilt");

    // Edit filter chats list
    let mut chats_list = EditFilterChatsListFilters::new(false);
    assert_eq!(chats_list.type_options.len(), 8);
    let initial_flags = chats_list.compute_flags();
    assert_eq!(CHAT_FLAG_CHANNELS, 1 << 5);
    assert_eq!(initial_flags & CHAT_FLAG_GROUPS, CHAT_FLAG_GROUPS);
    chats_list.toggle_flag(CHAT_FLAG_CONTACTS);
    let updated_flags = chats_list.compute_flags();
    assert_eq!(updated_flags & CHAT_FLAG_CONTACTS, CHAT_FLAG_CONTACTS);

    // Edit filter chips preview
    let mut preview = EditFilterChatsPreviewFilters::new();
    assert_eq!(preview.chips.len(), 3);
    preview.remove_chip(1);
    assert_eq!(preview.chips.len(), 2);

    // Edit filter links
    let links_box = EditFilterLinksFilters::new(1, "Crypto Folder");
    assert_eq!(K_MAX_LINK_TITLE_LENGTH, 32);
    assert_eq!(links_box.links.len(), 1);

    // Background box
    let mut bg_box = BackgroundBox::new(Some(123), Some("Alice".into()));
    assert_eq!(K_BACKGROUNDS_IN_ROW, 3);
    assert_eq!(bg_box.wallpapers.len(), 6);
    assert_eq!(bg_box.wallpapers[0].kind, WallpaperKind::Pattern);
    bg_box.select_wallpaper(3);
    assert_eq!(bg_box.selected_wallpaper_id, Some(3));

    // Background preview box
    let mut bg_preview = BackgroundPreviewBox::new(2, "Cosmic Indigo", Some("Alice".into()));
    assert_eq!(K_MAX_WALL_PAPER_SLUG_LENGTH, 255);
    assert!(!bg_preview.is_blurred);
    assert_eq!(bg_preview.dimming_percent, 20);
    bg_preview.toggle_blur();
    assert!(bg_preview.is_blurred);
    bg_preview.set_dimming(45);
    assert_eq!(bg_preview.dimming_percent, 45);
    bg_preview.toggle_apply_for_both();
    assert!(bg_preview.apply_for_both);
}
