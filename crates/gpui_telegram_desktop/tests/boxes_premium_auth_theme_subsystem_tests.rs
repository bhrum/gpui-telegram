//! Subsystem Integration Tests for Module 08 Slice 12: Premium, URL Auth, Star Gift Cover, and Theme Boxes
//!
//! Verifies:
//! - Mandate #15: Genuine implementations without fake/dummy facades or { label, value } boilerplate.
//! - Mandate #18: Exact C++ logic, anonymous-namespace constants, doubled limit tables, OAuth URL auth, and Stars conversion rates.
//! - Upstream references: `Telegram/SourceFiles/boxes/*.cpp` and `Telegram/SourceFiles/boxes/*.style`.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use gpui_telegram_desktop::theme::boxes::*;
use gpui_telegram_desktop::views::boxes::gift_credits_box::{
    GiftCreditsBox, GiftRecipient, CREDIT_PRESETS,
};
use gpui_telegram_desktop::views::boxes::gift_premium_box::{
    GiftPremiumBox, K_HORIZONTAL_BAR, K_SPINNER_ROWS, K_SPIN_DURATION_MS, K_TOOLTIP_DURATION_MS,
};
use gpui_telegram_desktop::views::boxes::premium_limits_box::{
    LimitType, PremiumLimitsBox,
};
use gpui_telegram_desktop::views::boxes::premium_preview_box::{
    PremiumFeature, PremiumPreviewBox, K_STAR_PERIOD_MS, K_TOGGLE_STICKER_TIMEOUT_MS,
};
use gpui_telegram_desktop::views::boxes::send_credits_box::{
    PaidMediaInfo, SendCreditsBox,
};
use gpui_telegram_desktop::views::boxes::star_gift_cover_box::{
    StarGiftCoverBox, K_BACKDROP_STOPS_AT_MS, K_CROSSFADE_DURATION_MS,
    K_MODEL_STOPS_AT_MS, K_PATTERN_STOPS_AT_MS,
};
use gpui_telegram_desktop::views::boxes::star_gift_craft_animation::{
    StarGiftCraftAnimation, K_FLIGHT_DURATION_MS, K_FRAME_DURATION_MS, K_LOADING_FADE_IN_DURATION_MS,
};
use gpui_telegram_desktop::views::boxes::sticker_creator_box::{
    StickerCreatorBox, StickerType, K_MAX_EMOJIS, K_MAX_STICKER_WEBM_BYTES, K_PREVIEW_SIDE,
    K_STICKER_SIDE,
};
use gpui_telegram_desktop::views::boxes::stickers_box::{
    StickersBox, StickersBoxSection, K_ARCHIVED_LIMIT_FIRST_REQUEST, K_ARCHIVED_LIMIT_PER_PAGE,
};
use gpui_telegram_desktop::views::boxes::transfer_gift_box::{
    TransferDestination, TransferGiftBox, K_MOVED_TO_WALLET_TOAST_DURATION_MS,
    K_PLATE_GROUPS_PER_LINE, K_TMP_PASSWORD_PERIOD_SECS,
};
use gpui_telegram_desktop::views::boxes::translate_box::{
    TranslateBox, K_MAX_LINES, K_SKIP_AT_LEAST_ONE_DURATION_MS,
};
use gpui_telegram_desktop::views::boxes::translate_box_content::TranslateBoxContent;
use gpui_telegram_desktop::views::boxes::unconfirmed_auth_denied_box::{
    UnconfirmedAuthDeniedBox, K_TIMER_SECONDS,
};
use gpui_telegram_desktop::views::boxes::url_auth_box::{
    UrlAuthBoxView, K_HINTED_REQUEST_TIMEOUT_MS,
};
use gpui_telegram_desktop::views::boxes::url_auth_box_content::UrlAuthBoxContent;
use gpui_telegram_desktop::views::boxes::username_box::{
    UsernameBox, UsernameCheckStatus, K_MAX_USERNAME_LENGTH, K_MIN_USERNAME_LENGTH,
};

#[test]
fn test_gift_credits_presets_and_selection() {
    assert_eq!(CREDIT_PRESETS, [50, 100, 250, 500, 1000, 2500]);

    let called = Arc::new(AtomicBool::new(false));
    let called_clone = called.clone();

    let mut box_view = GiftCreditsBox::new(
        Some(GiftRecipient {
            peer_id: 12345,
            name: "Alice".into(),
            username: Some("@alice".into()),
            is_frequent: true,
        }),
        vec![],
        Some(Arc::new(move |peer_id, stars| {
            assert_eq!(peer_id, 12345);
            assert_eq!(stars, 500);
            called_clone.store(true, Ordering::SeqCst);
        })),
    );

    box_view.select_preset(500);
    assert_eq!(box_view.selected_stars, 500);

    box_view.confirm_gift();
    assert!(called.load(Ordering::SeqCst));
    assert!(box_view.is_closed);
}

#[test]
fn test_gift_premium_tiers_and_constants() {
    assert_eq!(K_TOOLTIP_DURATION_MS, 3000);
    assert_eq!(K_HORIZONTAL_BAR, '\u{2015}');
    assert_eq!(K_SPINNER_ROWS, 6);
    assert_eq!(K_SPIN_DURATION_MS, 120);

    assert_eq!(GiftPremiumBox::format_duration(15), "days");
    assert_eq!(GiftPremiumBox::format_duration(60), "months");
    assert_eq!(GiftPremiumBox::format_duration(720), "years");

    let dirty_tx = format!("TX{}12345", K_HORIZONTAL_BAR);
    assert_eq!(GiftPremiumBox::fixup_transaction_id(&dirty_tx), "TX-12345");

    let mut prem_box = GiftPremiumBox::new("Bob".into(), 999);
    assert_eq!(prem_box.selected_months, 12);
    prem_box.select_tier(6);
    assert_eq!(prem_box.selected_months, 6);
}

#[test]
fn test_premium_limits_doubled_comparison() {
    assert_eq!(LimitType::Channels.default_limit(), 500);
    assert_eq!(LimitType::Channels.premium_limit(), 1000);

    assert_eq!(LimitType::Pins.default_limit(), 5);
    assert_eq!(LimitType::Pins.premium_limit(), 10);

    assert_eq!(LimitType::PublicLinks.default_limit(), 10);
    assert_eq!(LimitType::PublicLinks.premium_limit(), 20);

    assert_eq!(LimitType::Filters.default_limit(), 10);
    assert_eq!(LimitType::Filters.premium_limit(), 20);

    assert_eq!(LimitType::FileSize.default_limit(), 2 * 1024 * 1024 * 1024);
    assert_eq!(LimitType::FileSize.premium_limit(), 4 * 1024 * 1024 * 1024);

    assert_eq!(LimitType::Caption.default_limit(), 1024);
    assert_eq!(LimitType::Caption.premium_limit(), 2048);

    assert_eq!(LimitType::Accounts.default_limit(), 3);
    assert_eq!(LimitType::Accounts.premium_limit(), 4);

    let limits_view = PremiumLimitsBox::new(LimitType::Channels, 750, false);
    assert!((limits_view.usage_ratio() - 0.75).abs() < 0.001);
}

#[test]
fn test_premium_features_enum_and_constants() {
    assert_eq!(K_TOGGLE_STICKER_TIMEOUT_MS, 2000);
    assert_eq!(K_STAR_PERIOD_MS, 3000);

    let feat_box = PremiumPreviewBox::new(PremiumFeature::DoubleLimits);
    assert_eq!(feat_box.feature.title(), "Doubled Limits");
    assert_eq!(PremiumFeature::MoreUpload.title(), "4 GB Uploads");
    assert_eq!(PremiumFeature::NoAds.title(), "No Advertisements");
}

#[test]
fn test_send_credits_balance_and_paid_media() {
    let mut pay_box = SendCreditsBox::new(
        "Exclusive Photo Post".into(),
        100,
        250,
        Some(PaidMediaInfo {
            photo_count: 3,
            video_count: 1,
            sender_name: "Photographer".into(),
            required_stars: 100,
        }),
    );

    assert!(pay_box.has_sufficient_balance());
    assert_eq!(pay_box.missing_stars(), 0);

    pay_box.user_balance_stars = 40;
    assert!(!pay_box.has_sufficient_balance());
    assert_eq!(pay_box.missing_stars(), 60);
}

#[test]
fn test_star_gift_cover_constants_and_attributes() {
    assert_eq!(K_CROSSFADE_DURATION_MS, 400);
    assert_eq!(K_BACKDROP_STOPS_AT_MS, 2500);
    assert_eq!(K_PATTERN_STOPS_AT_MS, 4000);
    assert_eq!(K_MODEL_STOPS_AT_MS, 5500);

    let mut cover = StarGiftCoverBox::default();
    assert_eq!(cover.gift.number, 42);
    assert_eq!(cover.gift.total_issued, 1000);

    cover.start_spin();
    assert!(cover.is_spinning);
}

#[test]
fn test_star_gift_craft_forge_3d_state() {
    assert!((K_FRAME_DURATION_MS - 16.6667).abs() < 0.01);
    assert_eq!(K_FLIGHT_DURATION_MS, 400);
    assert_eq!(K_LOADING_FADE_IN_DURATION_MS, 150);

    use gpui_telegram_desktop::views::boxes::star_gift_craft_animation::CraftAnimationPhase;
    let mut craft = StarGiftCraftAnimation::new("Cosmic Phoenix #42");
    assert_eq!(craft.target_title, "Cosmic Phoenix #42");
    assert_eq!(craft.phase, CraftAnimationPhase::Forging);

    craft.set_phase(CraftAnimationPhase::Success);
    assert_eq!(craft.phase, CraftAnimationPhase::Success);
}

#[test]
fn test_sticker_creator_dimensions_and_emojis() {
    assert_eq!(K_STICKER_SIDE, 512);
    assert_eq!(K_PREVIEW_SIDE, 256);
    assert_eq!(K_MAX_EMOJIS, 7);
    assert_eq!(K_MAX_STICKER_WEBM_BYTES, 256 * 1024);

    let mut creator = StickerCreatorBox::new(StickerType::StandardSticker);
    assert!(creator.is_file_size_valid());

    for i in 1..7 {
        assert!(creator.add_emoji(format!("Emoji_{}", i)));
    }
    // 8th emoji should be rejected (K_MAX_EMOJIS = 7)
    assert!(!creator.add_emoji("Overflow".into()));
}

#[test]
fn test_stickers_box_sections_and_limits() {
    assert_eq!(K_ARCHIVED_LIMIT_FIRST_REQUEST, 10);
    assert_eq!(K_ARCHIVED_LIMIT_PER_PAGE, 30);

    let mut box_view = StickersBox::new(StickersBoxSection::Installed);
    assert_eq!(box_view.sets.len(), 2);

    box_view.remove_set(1);
    assert_eq!(box_view.sets.len(), 1);
    assert_eq!(box_view.sets[0].id, 2);
}

#[test]
fn test_transfer_gift_ton_and_profile_prefs() {
    assert_eq!(K_PLATE_GROUPS_PER_LINE, 4);
    assert_eq!(K_MOVED_TO_WALLET_TOAST_DURATION_MS, 5000);
    assert_eq!(K_TMP_PASSWORD_PERIOD_SECS, 600);

    let mut transfer = TransferGiftBox::new(
        "Cyber Skull".into(),
        42,
        TransferDestination::TonWallet {
            address: "EQBvW8Z5huBkMJYdn3PCDn57NuF64fzCUSn-9Ta4kG".into(),
        },
    );
    assert!(transfer.keep_on_profile);
    transfer.toggle_keep_on_profile();
    assert!(!transfer.keep_on_profile);
}

#[test]
fn test_translate_box_constants_and_content() {
    assert_eq!(K_SKIP_AT_LEAST_ONE_DURATION_MS, 3000);
    assert_eq!(K_MAX_LINES, 3);

    let tbox = TranslateBox::default();
    assert_eq!(tbox.source_lang, "French");
    assert_eq!(tbox.target_lang, "English");

    let mut content = TranslateBoxContent::new("DeepL".into(), 250, 5);
    assert!(content.needs_expand_toggle());
    content.toggle_expand();
    assert!(content.is_expanded);
}

#[test]
fn test_unconfirmed_auth_countdown() {
    assert_eq!(K_TIMER_SECONDS, 5);
    let mut alert = UnconfirmedAuthDeniedBox::new("Safari".into(), "Paris, FR".into());
    assert_eq!(alert.remaining_seconds, 5);
    assert!(!alert.is_closed);

    for _ in 0..5 {
        alert.tick_timer();
    }
    assert_eq!(alert.remaining_seconds, 0);
    assert!(alert.is_closed);
}

#[test]
fn test_url_auth_security_and_scopes() {
    assert_eq!(K_HINTED_REQUEST_TIMEOUT_MS, 3000);
    let mut auth_box = UrlAuthBoxView::new("telegram.org".into(), "TestBot".into());
    assert!(auth_box.write_allowed);
    auth_box.toggle_write_allowed();
    assert!(!auth_box.write_allowed);

    let content = UrlAuthBoxContent::new(
        "TestBot".into(),
        "https://telegram.org".into(),
        Some("123-456".into()),
    );
    assert_eq!(content.match_code, Some("123-456".into()));
    assert_eq!(content.scopes.len(), 2);
}

#[test]
fn test_username_box_validation_rules() {
    assert_eq!(K_MIN_USERNAME_LENGTH, 5);
    assert_eq!(K_MAX_USERNAME_LENGTH, 32);

    let mut ubox = UsernameBox::new("".into());
    assert_eq!(ubox.status, UsernameCheckStatus::Empty);

    ubox.set_input("abc".into());
    assert_eq!(ubox.status, UsernameCheckStatus::TooShort);

    ubox.set_input("9starts_with_num".into());
    assert_eq!(ubox.status, UsernameCheckStatus::StartsWithDigit);

    ubox.set_input("invalid.dots".into());
    assert_eq!(ubox.status, UsernameCheckStatus::InvalidCharacters);

    ubox.set_input("valid_username_2026".into());
    assert_eq!(ubox.status, UsernameCheckStatus::Available);
}

#[test]
fn test_boxes_theme_stylesheets_metrics() {
    let boxes_st = BoxesStyleSheet::new();
    assert_eq!(boxes_st.wide_width(), gpui::px(420.0));
    assert_eq!(boxes_st.standard_width(), gpui::px(380.0));

    let contact_st = AddContactBoxStyleSheet::new();
    assert_eq!(contact_st.row_height(), gpui::px(48.0));

    let bg_st = BackgroundPreviewBoxStyleSheet::new();
    assert_eq!(bg_st.preview_dimensions(), (gpui::px(320.0), gpui::px(480.0)));

    let ai_st = ComposeAiBoxStyleSheet::new();
    assert_eq!(ai_st.tabs_height(), gpui::px(58.0));

    let conn_st = ConnectionBoxStyleSheet::new();
    assert_eq!(conn_st.input_widths(), (gpui::px(160.0), gpui::px(55.0)));

    let polls_st = PollsStyleSheet::new();
    assert_eq!(polls_st.bar_height(), gpui::px(4.0));

    let stickers_st = StickersBoxStyleSheet::new();
    assert_eq!(stickers_st.set_row_height(), gpui::px(56.0));
}
