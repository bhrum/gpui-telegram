//! Settings Core Subsystem Tests
//! Covers: SettingsType, DetailedSettingsButton, SettingsCommon, SettingsCommonSession,
//! SettingsBuilder, SettingsCodes, SettingsFaqSuggestions, SettingsIntro, SettingsKeyNavigation,
//! SettingsNotificationsCommon, SettingsSearch, SettingsCreditsGraphics

use gpui_telegram_desktop::views::settings::*;
use gpui_telegram_desktop::views::settings::settings_type::SettingsSectionType;
use gpui_telegram_desktop::views::settings::settings_common::HighlightPhase;
use gpui_telegram_desktop::views::settings::settings_key_navigation::KeyNavigationDirection;
use gpui_telegram_desktop::views::settings::settings_notifications_common::ScreenCorner;
use gpui_telegram_desktop::views::settings::settings_credits_graphics::SmallBalanceSource;

#[test]
fn test_settings_section_types_catalog() {
    let types = [
        SettingsSectionType::Main,
        SettingsSectionType::Information,
        SettingsSectionType::Notifications,
        SettingsSectionType::PrivacySecurity,
        SettingsSectionType::Chat,
        SettingsSectionType::Folders,
        SettingsSectionType::Advanced,
        SettingsSectionType::Calls,
        SettingsSectionType::Credits,
        SettingsSectionType::Premium,
        SettingsSectionType::Shortcuts,
        SettingsSectionType::Websites,
        SettingsSectionType::Passkeys,
        SettingsSectionType::LocalPasscode,
        SettingsSectionType::PowerSaving,
        SettingsSectionType::LocalStorage,
        SettingsSectionType::Business,
        SettingsSectionType::Search,
    ];

    assert_eq!(types.len(), 18);
    assert!(SettingsSectionType::Main.is_root());
    assert!(!SettingsSectionType::Information.is_root());
    assert_eq!(SettingsSectionType::Calls.id_str(), "calls");
    assert_eq!(SettingsSectionType::Credits.display_title(), "Telegram Stars");

    let mut view = SettingsType::new();
    assert_eq!(view.current_section, SettingsSectionType::Main);
    view.set_section(SettingsSectionType::Websites);
    assert_eq!(view.current_section, SettingsSectionType::Websites);
    assert_eq!(view.title, "Logged in Websites");
}

#[test]
fn test_detailed_settings_button_state() {
    let mut btn = DetailedSettingsButton::new("Auto-Download", "Photos, Videos, Files");
    assert!(!btn.is_toggled);
    assert!(!btn.is_locked);
    assert!(btn.badge.is_none());

    btn.toggle();
    assert!(btn.is_toggled);
    btn.set_locked(true);
    assert!(btn.is_locked);
    btn.set_badge("NEW");
    assert_eq!(btn.badge.as_deref(), Some("NEW"));
}

#[test]
fn test_settings_common_highlight_transitions() {
    let mut common = SettingsCommon::new("Privacy Settings");
    assert_eq!(common.highlight_phase, HighlightPhase::Wait);

    common.advance_phase();
    assert_eq!(common.highlight_phase, HighlightPhase::FadeIn);
    common.advance_phase();
    assert_eq!(common.highlight_phase, HighlightPhase::Shown);
    common.start_fade_out();
    assert_eq!(common.highlight_phase, HighlightPhase::FadeOut);

    let words = common.search_words("Two-Step Verification Passcode");
    assert!(common.matches_words(&words, "verification"));
    assert!(!common.matches_words(&words, "unrelated"));
}

#[test]
fn test_settings_common_session_routing() {
    let session = SettingsCommonSession::new(12345);
    assert_eq!(session.account_id, 12345);
    assert!(session.has_menu("chat"));
    assert!(!session.has_menu("unknown_section"));
}

#[test]
fn test_settings_builder_fluent_dsl() {
    let mut builder = SettingsBuilder::new();
    builder.add_button("btn_notif", "Notifications", Some("Sounds, Badges"), None);
    builder.add_subsection_title("Privacy");
    builder.add_section_button("sec_priv", "Security & Privacy", SettingsSectionType::PrivacySecurity, None);

    assert_eq!(builder.items.len(), 3);
    let results = builder.search_registry.search("Privacy");
    assert!(!results.is_empty());
}

#[test]
fn test_settings_codes_debug_parser() {
    let mut codes = SettingsCodes::new();
    assert!(!codes.debug_mode);
    assert!(!codes.test_mode);

    assert!(codes.feed_code("debugmode"));
    assert!(codes.debug_mode);
    assert!(codes.feed_code("testmode"));
    assert!(codes.test_mode);
    assert!(!codes.feed_code("invalidcode"));
}

#[test]
fn test_settings_faq_suggestions_query() {
    let faq = SettingsFaqSuggestions::new();
    let results = faq.query("login");
    assert!(!results.is_empty());
    assert!(results[0].url.contains("telegram.org/faq"));
}

#[test]
fn test_settings_key_navigation_2d_grid() {
    let mut nav = SettingsKeyNavigation::new();
    nav.add_item("item_0", 0.0, 0.0, 100.0, 30.0);
    nav.add_item("item_1", 0.0, 40.0, 100.0, 30.0);
    nav.add_item("item_2", 120.0, 0.0, 100.0, 30.0);

    nav.focused_index = Some(0);
    let handled = nav.handle_direction(KeyNavigationDirection::Down);
    assert!(handled);
    assert_eq!(nav.focused_index, Some(1));

    let handled_right = nav.handle_direction(KeyNavigationDirection::Right);
    assert!(handled_right);
    assert_eq!(nav.focused_index, Some(2));
}

#[test]
fn test_settings_notifications_common_geometry() {
    let notif = SettingsNotificationsCommon::new();
    assert_eq!(notif.corner, ScreenCorner::BottomRight);
    assert_eq!(notif.clamped_sample_count(10), 5);
    assert_eq!(notif.clamped_sample_count(0), 1);
    assert_eq!(notif.clamped_sample_count(3), 3);
}

#[test]
fn test_settings_search_filtering() {
    let mut search = SettingsSearch::new();
    search.index_item("account_bio", "Bio & Details", "Bio", SettingsSectionType::Information);
    search.index_item("passcode_lock", "Passcode Lock", "Security", SettingsSectionType::LocalPasscode);

    let hits = search.search("lock");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].id, "passcode_lock");
}

#[test]
fn test_settings_credits_graphics_options() {
    let mut graphics = SettingsCreditsGraphics::new();
    assert_eq!(graphics.source, SmallBalanceSource::Bot);
    assert_eq!(graphics.balance, 1500);

    graphics.add_stars(500);
    assert_eq!(graphics.balance, 2000);
    assert_eq!(graphics.topup_options.len(), 4);
}
