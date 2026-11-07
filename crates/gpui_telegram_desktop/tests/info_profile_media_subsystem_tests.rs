//! Subsystem integration tests for Module 11 Slice 1: Info Profile & Shared Media
//! Validates authentic business logic, state machines, and calculations.

use gpui_telegram_desktop::views::drawer::media::info_media_buttons::InfoMediaButtonsMedia;
use gpui_telegram_desktop::views::drawer::media::info_media_common::{
    InfoMediaCommonMedia, SharedMediaType,
};
use gpui_telegram_desktop::views::drawer::media::info_media_grid_zoom::InfoMediaGridZoomMedia;
use gpui_telegram_desktop::views::drawer::media::info_media_list_section::InfoMediaListSectionMedia;
use gpui_telegram_desktop::views::drawer::media::info_media_list_widget::InfoMediaListWidgetMedia;
use gpui_telegram_desktop::views::drawer::media::info_media_provider::InfoMediaProviderMedia;
use gpui_telegram_desktop::views::drawer::profile::info_profile_actions::InfoProfileActionsProfile;
use gpui_telegram_desktop::views::drawer::profile::info_profile_badge::{
    InfoProfileBadgeProfile, PeerBadgeKind,
};
use gpui_telegram_desktop::views::drawer::profile::info_profile_balance_pill::InfoProfileBalancePillProfile;
use gpui_telegram_desktop::views::drawer::profile::info_profile_birthday_effect::InfoProfileBirthdayEffectProfile;
use gpui_telegram_desktop::views::drawer::profile::info_profile_members::InfoProfileMembersProfile;
use gpui_telegram_desktop::views::drawer::profile::info_profile_section_stack::{
    InfoProfileSectionStackProfile, ProfileSectionKind,
};
use gpui_telegram_desktop::views::drawer::profile::info_profile_share_theme::InfoProfileShareThemeProfile;
use gpui_telegram_desktop::views::drawer::profile::info_profile_top_bar::InfoProfileTopBarProfile;
use gpui_telegram_desktop::views::drawer::profile::info_profile_values::InfoProfileValuesProfile;
use gpui_telegram_desktop::views::drawer::profile::tabs::info_profile_tab_content::ProfileTabKind;
use gpui_telegram_desktop::views::drawer::profile::tabs::info_profile_tabs_host::InfoProfileTabsHostProfileTabs;
use gpui_telegram_desktop::views::drawer::profile::tabs::info_profile_tabs_strip::{
    InfoProfileTabsStripProfileTabs, StripTabItem,
};

#[test]
fn test_media_buttons_and_counts() {
    let mut buttons = InfoMediaButtonsMedia::new(1001);
    assert_eq!(buttons.active_type, SharedMediaType::PhotoVideo);
    assert_eq!(buttons.count_for(SharedMediaType::PhotoVideo), 0);

    buttons.set_count(SharedMediaType::PhotoVideo, 42);
    buttons.set_count(SharedMediaType::Files, 15);
    buttons.set_active_type(SharedMediaType::Files);

    assert_eq!(buttons.active_type, SharedMediaType::Files);
    assert_eq!(buttons.count_for(SharedMediaType::PhotoVideo), 42);
    assert_eq!(buttons.count_for(SharedMediaType::Files), 15);
}

#[test]
fn test_media_common_and_preload_formula() {
    let mut common = InfoMediaCommonMedia::new();
    assert_eq!(common.preload_screens_count, 2.0);
    assert_eq!(common.active_media_type, SharedMediaType::PhotoVideo);

    // If remaining screens < 2.0, preload triggers
    assert!(common.should_preload(8.5, 10.0)); // 1.5 < 2.0 -> true
    assert!(!common.should_preload(5.0, 10.0)); // 5.0 >= 2.0 -> false

    common.set_media_type(SharedMediaType::SharedLinks);
    assert_eq!(common.active_media_type, SharedMediaType::SharedLinks);
}

#[test]
fn test_media_grid_zoom_steps_and_item_sizing() {
    let mut zoom = InfoMediaGridZoomMedia::new();
    assert_eq!(zoom.current_columns, 3);

    // Zoom in reduces columns (larger items)
    assert!(zoom.zoom_in());
    assert_eq!(zoom.current_columns, 2);
    assert!(zoom.zoom_in());
    assert_eq!(zoom.current_columns, 1);
    assert!(!zoom.zoom_in()); // hit min

    // Zoom out increases columns (smaller items)
    assert!(zoom.zoom_out());
    assert_eq!(zoom.current_columns, 2);

    let item_w = zoom.item_size(300.0, 4.0);
    assert_eq!(item_w, (300.0 - 4.0) / 2.0);
}

#[test]
fn test_media_list_section_and_stickiness() {
    let mut sec = InfoMediaListSectionMedia::new(2024, 11, 25);
    assert_eq!(sec.title, "November 2024");
    assert_eq!(sec.items_count, 25);

    sec.update_geometry(150.0, 300.0);
    assert!(!sec.check_sticky(100.0));
    assert!(sec.check_sticky(150.0));
    assert!(sec.check_sticky(300.0));
    assert!(!sec.check_sticky(450.0));
}

#[test]
fn test_media_list_widget_selection() {
    let mut list = InfoMediaListWidgetMedia::new(2002, SharedMediaType::PhotoVideo);
    assert_eq!(list.selected_count(), 0);
    assert!(!list.is_selection_active);

    list.toggle_select_item(101);
    list.toggle_select_item(102);
    assert_eq!(list.selected_count(), 2);
    assert!(list.is_selection_active);

    list.toggle_select_item(101);
    assert_eq!(list.selected_count(), 1);

    list.clear_selection();
    assert_eq!(list.selected_count(), 0);
    assert!(!list.is_selection_active);
}

#[test]
fn test_media_provider_pagination_lifecycle() {
    let mut provider = InfoMediaProviderMedia::new(3003, SharedMediaType::Files);
    assert!(provider.has_more);
    assert!(!provider.is_loading);

    assert!(provider.start_loading());
    assert!(provider.is_loading);
    assert!(!provider.start_loading()); // double start prevented

    provider.finish_loading(50, 10050, true);
    assert!(!provider.is_loading);
    assert_eq!(provider.loaded_count, 50);
    assert_eq!(provider.last_max_id, 10050);

    provider.reset();
    assert_eq!(provider.loaded_count, 0);
    assert_eq!(provider.last_max_id, 0);
}

#[test]
fn test_profile_top_bar_and_actions() {
    let mut top_bar = InfoProfileTopBarProfile::new(4004, "Alice", "online");
    assert_eq!(top_bar.title, "Alice");
    top_bar.set_collapse_ratio(0.8);
    assert_eq!(top_bar.collapse_ratio, 0.8);
    top_bar.set_verified(true);
    assert!(top_bar.is_verified);

    let mut actions = InfoProfileActionsProfile::new(4004);
    actions.set_phone("+123456789");
    actions.set_username("alice_test");
    actions.set_bio("GPUI Telegram Enthusiast");
    actions.toggle_notifications();

    assert_eq!(actions.phone.as_deref(), Some("+123456789"));
    assert_eq!(actions.username.as_deref(), Some("alice_test"));
    assert!(!actions.notifications_enabled);
}

#[test]
fn test_profile_section_stack_navigation() {
    let mut stack = InfoProfileSectionStackProfile::new();
    assert_eq!(stack.current(), ProfileSectionKind::Main);

    stack.push(ProfileSectionKind::SharedMedia);
    assert_eq!(stack.current(), ProfileSectionKind::SharedMedia);

    stack.push(ProfileSectionKind::Members);
    assert_eq!(stack.current(), ProfileSectionKind::Members);

    assert_eq!(stack.pop(), Some(ProfileSectionKind::Members));
    assert_eq!(stack.current(), ProfileSectionKind::SharedMedia);
    assert_eq!(stack.pop(), Some(ProfileSectionKind::SharedMedia));
    assert_eq!(stack.current(), ProfileSectionKind::Main);
    assert_eq!(stack.pop(), None); // cannot pop root
}

#[test]
fn test_profile_tabs_host_and_strip() {
    let tabs = vec![
        StripTabItem { kind: ProfileTabKind::Media, title: "Media".into(), count: Some(10) },
        StripTabItem { kind: ProfileTabKind::Files, title: "Files".into(), count: Some(5) },
        StripTabItem { kind: ProfileTabKind::Links, title: "Links".into(), count: None },
    ];
    let mut strip = InfoProfileTabsStripProfileTabs::new(tabs);
    assert_eq!(strip.selected_index, 0);
    strip.select_tab(1);
    assert_eq!(strip.selected_index, 1);

    let mut host = InfoProfileTabsHostProfileTabs::new(vec![
        ProfileTabKind::Media,
        ProfileTabKind::Files,
        ProfileTabKind::Links,
    ]);
    assert_eq!(host.active_tab, ProfileTabKind::Media);
    assert!(host.switch_to_tab(ProfileTabKind::Files));
    assert_eq!(host.active_tab, ProfileTabKind::Files);
    assert!(!host.switch_to_tab(ProfileTabKind::Files)); // no-op same tab
}
