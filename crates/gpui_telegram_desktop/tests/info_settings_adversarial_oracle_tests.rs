//! Module 11 (`info & settings`) Adversarial Oracle & Stress Tests
//!
//! Empirical edge-case validation, state transitions, oversized lists, and concurrent stress
//! across Info Drawer tabs (Members, Media, Files, Music, Voice, Links, Groups in Common)
//! and Settings navigation transitions (Main, Edit Profile, General, Notifications, Privacy & Security, Chat Settings, Advanced, Folders).

use std::sync::{Arc, RwLock};
use std::thread;

use gpui_telegram_desktop::views::drawer::common_groups::info_common_groups_inner_widget::{
    CommonGroupRow, InfoCommonGroupsInnerWidgetCommonGroups,
};
use gpui_telegram_desktop::views::drawer::common_groups::info_common_groups_widget::InfoCommonGroupsWidgetCommonGroups;
use gpui_telegram_desktop::views::drawer::media::info_media_buttons::InfoMediaButtonsMedia;
use gpui_telegram_desktop::views::drawer::media::info_media_common::{
    InfoMediaCommonMedia, SharedMediaType,
};
use gpui_telegram_desktop::views::drawer::media::info_media_grid_zoom::InfoMediaGridZoomMedia;
use gpui_telegram_desktop::views::drawer::profile::info_profile_section_stack::{
    InfoProfileSectionStackProfile, ProfileSectionKind, RowType, SectionStackOptimizer,
};
use gpui_telegram_desktop::views::drawer::profile::tabs::info_profile_tab_content::ProfileTabKind;
use gpui_telegram_desktop::views::drawer::profile::tabs::info_profile_tabs_host::InfoProfileTabsHostProfileTabs;
use gpui_telegram_desktop::views::drawer::profile::tabs::info_profile_tabs_strip::{
    InfoProfileTabsStripProfileTabs, StripTabItem,
};
use gpui_telegram_desktop::views::settings::cloud_password::settings_cloud_password_common::{
    CloudPasswordStage, SettingsCloudPasswordCommonCloudPassword,
};
use gpui_telegram_desktop::views::settings::sections::settings_advanced::{
    ProxyType, SettingsAdvancedSections,
};
use gpui_telegram_desktop::views::settings::sections::settings_folders::{
    ChatFolderItem, SettingsFoldersSections,
};
use gpui_telegram_desktop::views::settings::sections::settings_information::SettingsInformationSections;
use gpui_telegram_desktop::views::settings::sections::settings_main::SettingsMainSections;
use gpui_telegram_desktop::views::settings::settings_faq_suggestions::SettingsFaqSuggestions;
use gpui_telegram_desktop::views::settings::settings_search::SettingsSearch;
use gpui_telegram_desktop::views::settings::settings_type::{SettingsSectionType, SettingsType};

// ============================================================================
// 1. Info Drawer Tabs State Transitions & Adversarial Scenarios
// ============================================================================

#[test]
fn test_adversarial_info_drawer_tabs_transitions() {
    // 7 Core Drawer Tabs: Members, Media, Files, Music, Voice, Links, Groups in Common
    let all_tabs = vec![
        ProfileTabKind::Members,
        ProfileTabKind::Media,
        ProfileTabKind::Files,
        ProfileTabKind::Audio, // Music
        ProfileTabKind::Voice,
        ProfileTabKind::Links,
    ];

    let mut host = InfoProfileTabsHostProfileTabs::new(all_tabs.clone());
    assert_eq!(host.active_tab, ProfileTabKind::Members);

    // Permutation walk across every tab transition
    for &target_tab in &all_tabs {
        if host.active_tab != target_tab {
            assert!(host.switch_to_tab(target_tab), "Transition to {:?} must succeed", target_tab);
            assert_eq!(host.active_tab, target_tab);
        }
        // Idempotency check: switching to currently active tab returns false
        assert!(!host.switch_to_tab(target_tab), "Idempotent switch must return false");
        assert_eq!(host.active_tab, target_tab);
    }

    // Adversarial: attempt transition to un-registered tabs
    assert!(!host.switch_to_tab(ProfileTabKind::Polls));
    assert!(!host.switch_to_tab(ProfileTabKind::SavedSublists));
    assert!(!host.switch_to_tab(ProfileTabKind::SimilarChannels));
    // Verify active_tab didn't get corrupted
    assert_eq!(host.active_tab, ProfileTabKind::Links);

    // Groups in Common container state transitions
    let mut common_groups = InfoCommonGroupsWidgetCommonGroups::new(12345);
    assert_eq!(common_groups.user_id, 12345);
    assert_eq!(common_groups.title(), "Common Groups");
    assert_eq!(common_groups.group_count(), 0);

    common_groups.set_search_focused(true);
    assert!(common_groups.is_search_field_focused);
    common_groups.set_search_focused(false);
    assert!(!common_groups.is_search_field_focused);
}

#[test]
fn test_adversarial_tabs_strip_and_host_edge_cases() {
    // Edge case 1: empty available tabs in host
    let mut empty_host = InfoProfileTabsHostProfileTabs::new(vec![]);
    assert_eq!(empty_host.active_tab, ProfileTabKind::Media); // Safe default fallback
    assert!(!empty_host.switch_to_tab(ProfileTabKind::Audio));
    assert_eq!(empty_host.active_tab, ProfileTabKind::Media);

    // Edge case 2: empty tabs in strip
    let mut empty_strip = InfoProfileTabsStripProfileTabs::new(vec![]);
    assert_eq!(empty_strip.selected_index, 0);
    empty_strip.select_tab(0);
    assert_eq!(empty_strip.selected_index, 0);
    empty_strip.select_tab(999);
    assert_eq!(empty_strip.selected_index, 0);

    // Edge case 3: Extreme scroll physics
    empty_strip.scroll_by(-1000.0);
    assert_eq!(empty_strip.scroll_left, 0.0); // Clamped at 0.0
    empty_strip.scroll_by(500.0);
    assert_eq!(empty_strip.scroll_left, 250.0); // 500.0 * K_F(0.5)
    empty_strip.scroll_by(-1000.0);
    assert_eq!(empty_strip.scroll_left, 0.0); // Clamped back to 0.0

    // Out-of-bounds selection on populated strip
    let tabs = vec![
        StripTabItem { kind: ProfileTabKind::Media, title: "Media".into(), count: Some(10) },
        StripTabItem { kind: ProfileTabKind::Files, title: "Files".into(), count: Some(0) },
    ];
    let mut strip = InfoProfileTabsStripProfileTabs::new(tabs);
    strip.select_tab(1);
    assert_eq!(strip.selected_index, 1);
    strip.select_tab(50); // Out of bounds: should be ignored
    assert_eq!(strip.selected_index, 1);
    assert_eq!(strip.indicator_x, 64.0);
}


// ============================================================================
// 2. Info Drawer Oversized Lists & Edge Scenarios
// ============================================================================

#[test]
fn test_adversarial_common_groups_oversized_list_and_unicode_search() {
    let mut inner = InfoCommonGroupsInnerWidgetCommonGroups::new(9999);
    assert_eq!(inner.groups.len(), 0);
    assert_eq!(inner.filtered_groups().len(), 0);
    assert_eq!(inner.total_height(), 0.0);

    // Adversarial load: 10,000 groups with varying titles, unicode, and IDs
    for i in 0..10_000 {
        inner.add_group(CommonGroupRow {
            peer_id: i as u64,
            title: format!("Group #{:05} 🚀 [Developers & Engineers]", i),
            username: format!("group_{:05}", i),
            member_count: (i * 7) as u32 % 50_000,
            is_selected: false,
        });
    }
    assert_eq!(inner.groups.len(), 10_000);
    assert_eq!(inner.filtered_groups().len(), 10_000);
    assert_eq!(inner.total_height(), 10_000.0 * 52.0);

    // Deduplication test: re-adding existing peer_ids must not create duplicates
    for i in 0..100 {
        inner.add_group(CommonGroupRow {
            peer_id: i as u64,
            title: "Duplicate Attempt".into(),
            username: "dup".into(),
            member_count: 999,
            is_selected: true,
        });
    }
    assert_eq!(inner.groups.len(), 10_000);

    // Search query filtering stress
    inner.set_search_query("9999");
    assert_eq!(inner.filtered_groups().len(), 1);
    assert_eq!(inner.filtered_groups()[0].peer_id, 9999);

    // Unicode search
    inner.set_search_query("🚀");
    assert_eq!(inner.filtered_groups().len(), 10_000);

    // Non-existent search query
    inner.set_search_query("NON_EXISTENT_MATCH_XYZ_123");
    assert_eq!(inner.filtered_groups().len(), 0);
    assert_eq!(inner.total_height(), 0.0);

    // Empty search query restores all
    inner.set_search_query("   ");
    assert_eq!(inner.filtered_groups().len(), 10_000);
}

#[test]
fn test_adversarial_shared_media_preload_and_zoom_edge_cases() {
    let media_common = InfoMediaCommonMedia::new();

    // Adversarial preload inputs: current > total (scrolled past end)
    assert!(media_common.should_preload(15.0, 10.0)); // total - current = -5.0 < 2.0 -> true
    assert!(!media_common.should_preload(0.0, 10.0)); // 10.0 >= 2.0 -> false
    assert!(media_common.should_preload(8.01, 10.0)); // 1.99 < 2.0 -> true
    assert!(!media_common.should_preload(8.0, 10.0));  // 2.0 >= 2.0 -> false

    // Zoom grid bounds: min columns 1, max columns
    let mut zoom = InfoMediaGridZoomMedia::new();
    // Max zoom in repeatedly
    for _ in 0..10 {
        zoom.zoom_in();
    }
    assert_eq!(zoom.current_columns, 1);
    assert!(!zoom.zoom_in()); // Cannot zoom beyond 1 column

    // Max zoom out repeatedly
    for _ in 0..20 {
        zoom.zoom_out();
    }
    let max_cols = zoom.current_columns;
    assert!(max_cols >= 6);
    assert!(!zoom.zoom_out()); // At boundary

    // Item size calculations with 0 width: clamps to min 24px safety floor
    let w_zero = zoom.item_size(0.0, 4.0);
    assert_eq!(w_zero, 24.0);

    // Buttons count with usize::MAX
    let mut buttons = InfoMediaButtonsMedia::new(777);
    buttons.set_count(SharedMediaType::PhotoVideo, usize::MAX);
    assert_eq!(buttons.count_for(SharedMediaType::PhotoVideo), usize::MAX);
    assert_eq!(buttons.count_for(SharedMediaType::AudioMusic), 0);
}

#[test]
fn test_adversarial_section_stack_optimizer_deep_invariants() {
    let mut stack = InfoProfileSectionStackProfile::new();
    assert_eq!(stack.current(), ProfileSectionKind::Main);

    // Push 5,000 sections
    for _ in 0..5_000 {
        stack.push(ProfileSectionKind::SharedMedia);
    }
    assert_eq!(stack.current(), ProfileSectionKind::SharedMedia);

    // Pop back to root
    for _ in 0..5_000 {
        assert_eq!(stack.pop(), Some(ProfileSectionKind::SharedMedia));
    }
    assert_eq!(stack.current(), ProfileSectionKind::Main);
    assert_eq!(stack.pop(), None); // Root cannot be popped

    // Consecutive and edge plain separators
    let kinds = vec![
        RowType::PlainSeparator, // Leading separator
        RowType::PlainSeparator,
        RowType::Block,
        RowType::PlainSeparator,
        RowType::PlainSeparator, // Double separator
        RowType::Block,
        RowType::PlainSeparator, // Trailing separator
    ];
    let intrinsic = vec![true, true, true, true, true, true, true];
    let visibility = SectionStackOptimizer::compute_visibility(&kinds, &intrinsic);

    // Leading and trailing plain separators must NEVER be visible
    assert!(!visibility[0]);
    assert!(!visibility[1]);
    assert!(!visibility[6]);
    // The separator between two visible blocks is evaluated
    assert!(visibility[2]); // Block 1
    assert!(visibility[5]); // Block 2
}


// ============================================================================
// 3. Settings Navigation State Transitions & Section Matrix
// ============================================================================

#[test]
fn test_adversarial_settings_navigation_transitions_matrix() {
    // 8 Required Settings Sections: Main, Edit Profile, General (FAQ / Help), Notifications,
    // Privacy & Security, Chat Settings, Advanced, Folders
    let nav_sections = [
        (SettingsSectionType::Main, "Settings", "main", true),
        (SettingsSectionType::Information, "Edit Profile", "information", false),
        (SettingsSectionType::Notifications, "Notifications and Sounds", "notifications", false),
        (SettingsSectionType::PrivacySecurity, "Privacy and Security", "privacy", false),
        (SettingsSectionType::Chat, "Chat Settings", "chat", false),
        (SettingsSectionType::Folders, "Chat Folders", "folders", false),
        (SettingsSectionType::Advanced, "Advanced", "advanced", false),
    ];

    let mut main_view = SettingsMainSections::new();
    assert_eq!(main_view.active_section, None);

    for &(sec, title, id_str, is_root) in &nav_sections {
        assert_eq!(sec.display_title(), title);
        assert_eq!(sec.id_str(), id_str);
        assert_eq!(sec.is_root(), is_root);

        // Transition in main_view
        main_view.navigate_to(sec);
        assert_eq!(main_view.active_section, Some(sec));

        // SettingsType view model update
        let mut type_view = SettingsType::new();
        type_view.set_section(sec);
        assert_eq!(type_view.current_section, sec);
        assert_eq!(type_view.title, title);
    }

    // General Settings via FAQ suggestions integration
    let faq = SettingsFaqSuggestions::new();
    let general_queries = faq.query("general");
    assert!(!general_queries.is_empty());
    assert_eq!(general_queries[0].title, "General Questions");
    assert_eq!(general_queries[0].anchor, "general");

    // UI scale boundary clamping
    main_view.set_interface_scale(50);
    assert_eq!(main_view.interface_scale_pct, 100); // Clamped min
    main_view.set_interface_scale(500);
    assert_eq!(main_view.interface_scale_pct, 300); // Clamped max
    main_view.set_interface_scale(150);
    assert_eq!(main_view.interface_scale_pct, 150);
}

#[test]
fn test_adversarial_chat_folders_oversized_and_empty_states() {
    let mut folders_sec = SettingsFoldersSections::new();
    assert_eq!(folders_sec.folders.len(), 3);
    assert!(!folders_sec.lng_filters_no_chats(1));

    // Remove all default folders
    folders_sec.set_removed(1);
    folders_sec.set_removed(2);
    folders_sec.set_removed(3);

    assert!(folders_sec.find(1).is_none());
    assert!(folders_sec.lng_filters_no_chats(1));

    // Adversarial load: 5,000 folders
    for i in 100..5_100 {
        folders_sec.folders.push(ChatFolderItem {
            id: i,
            title: format!("Folder #{:05} 📁", i),
            icon_name: "folder_custom".into(),
            included_peers_count: (i as usize) % 100,
            excluded_peers_count: 0,
            unread_count: (i as usize) % 10,
            is_removed: false,
        });
    }

    assert_eq!(folders_sec.folders.len(), 5003);
    assert_eq!(folders_sec.lng_filters_chats_count(5000), 5000 % 100);

    // Update data with oversized string
    let huge_name = "LongFolderName".repeat(500);
    folders_sec.update_data(5000, huge_name.clone());
    assert_eq!(folders_sec.find(5000).unwrap().title, huge_name);

    // Remove and verify find returns None
    folders_sec.set_removed(5000);
    assert!(folders_sec.find(5000).is_none());
}

#[test]
fn test_adversarial_advanced_proxy_and_profile_bio_bounds() {
    let mut adv = SettingsAdvancedSections::new();
    assert_eq!(adv.proxy_type, ProxyType::Direct);

    // Proxy transition cycle
    adv.lng_connection_proxy();
    assert_eq!(adv.proxy_type, ProxyType::Socks5);
    adv.setup_connection_type();
    assert_eq!(adv.proxy_type, ProxyType::Direct);

    // Path separator normalization
    adv.download_path = "C:/Users/Test/Telegram/Downloads".into();
    adv.to_native_separators();
    assert_eq!(adv.download_path, "C:\\Users\\Test\\Telegram\\Downloads");

    // Profile Settings bio character limits & validation
    let mut info = SettingsInformationSections::new();
    // Valid bio up to 70 chars
    assert!(info.update_bio("Rust GPUI high performance Telegram desktop client."));
    // Exactly 70 chars
    let exact_70 = "A".repeat(70);
    assert!(info.update_bio(&exact_70));
    assert_eq!(info.bio, exact_70);

    // Empty bio
    assert!(info.update_bio(""));
    assert_eq!(info.bio, "");

    // Adversarial oversized bio (100,000 chars) must be rejected cleanly
    let oversized_bio = "Z".repeat(100_000);
    assert!(!info.update_bio(&oversized_bio));
    assert_eq!(info.bio, ""); // Must remain previous value without panic
}


// ============================================================================
// 4. Adversarial Search, 2FA Stage Machine & Concurrent Updates
// ============================================================================

#[test]
fn test_adversarial_settings_search_scaling() {
    let mut search = SettingsSearch::new();

    // Empty search returns empty results
    let empty_hits = search.search("anything");
    assert!(empty_hits.is_empty());

    // Index 10,000 items
    for i in 0..10_000 {
        search.index_item(
            format!("item_{:05}", i),
            format!("Preference Item Number {:05}", i),
            "General Category",
            SettingsSectionType::Advanced,
        );
    }

    // Specific hit
    let hits = search.search("09876");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].id, "item_09876");

    // Case-insensitive hit
    let case_hits = search.search("preFERence item");
    assert!(!case_hits.is_empty());

    // Non-existent search
    let zero_hits = search.search("XYZ_UNMATCHED_QUERY");
    assert!(zero_hits.is_empty());
}

#[test]
fn test_adversarial_cloud_password_stage_machine_fuzzing() {
    let mut cp = SettingsCloudPasswordCommonCloudPassword::new();
    assert_eq!(cp.current_stage, CloudPasswordStage::Start);

    let stages = [
        CloudPasswordStage::SetPassword,
        CloudPasswordStage::ConfirmPassword,
        CloudPasswordStage::PasswordHint,
        CloudPasswordStage::RecoveryEmail,
        CloudPasswordStage::EmailConfirm,
        CloudPasswordStage::LoginEmail,
        CloudPasswordStage::LoginEmailConfirm,
        CloudPasswordStage::Manage,
    ];

    // Progression fuzzing
    for stage in stages {
        cp.set_stage(stage.clone());
        assert_eq!(cp.current_stage, stage);
    }

    // Field resets and error states
    cp.update_hint("SecretHint123");
    cp.update_email("user@telegram.org");
    assert_eq!(cp.fields.hint, "SecretHint123");
    assert_eq!(cp.fields.email, "user@telegram.org");
    assert!(cp.fields.has_recovery);

    cp.set_error("Verification code expired");
    assert_eq!(cp.error_text.as_deref(), Some("Verification code expired"));

    cp.clear_error();
    assert!(cp.error_text.is_none());

    cp.reset_fields();
    assert!(cp.fields.email.is_empty());
    assert!(cp.fields.hint.is_empty());
    assert!(!cp.fields.has_recovery);
}

#[test]
fn test_adversarial_concurrent_updates_stress_harness() {
    // Shared state models wrapped in Arc<RwLock<...>> simulating multi-threaded UI + network sync
    let shared_host = Arc::new(RwLock::new(InfoProfileTabsHostProfileTabs::new(vec![
        ProfileTabKind::Members,
        ProfileTabKind::Media,
        ProfileTabKind::Files,
        ProfileTabKind::Audio,
        ProfileTabKind::Voice,
        ProfileTabKind::Links,
    ])));

    let shared_main = Arc::new(RwLock::new(SettingsMainSections::new()));
    let shared_groups = Arc::new(RwLock::new(InfoCommonGroupsInnerWidgetCommonGroups::new(1001)));

    let mut handles = Vec::new();

    // Spawn 16 concurrent worker threads
    for thread_idx in 0..16 {
        let host_clone = Arc::clone(&shared_host);
        let main_clone = Arc::clone(&shared_main);
        let groups_clone = Arc::clone(&shared_groups);

        handles.push(thread::spawn(move || {
            let tab_cycle = [
                ProfileTabKind::Members,
                ProfileTabKind::Media,
                ProfileTabKind::Files,
                ProfileTabKind::Audio,
                ProfileTabKind::Voice,
                ProfileTabKind::Links,
            ];
            let sec_cycle = [
                SettingsSectionType::Main,
                SettingsSectionType::Information,
                SettingsSectionType::Notifications,
                SettingsSectionType::PrivacySecurity,
                SettingsSectionType::Chat,
                SettingsSectionType::Folders,
                SettingsSectionType::Advanced,
            ];

            for iter in 0..1_000 {
                // 1. Tab switching
                {
                    let mut host = host_clone.write().unwrap();
                    let target_tab = tab_cycle[(thread_idx + iter) % tab_cycle.len()];
                    host.switch_to_tab(target_tab);
                }

                // 2. Settings navigation
                {
                    let mut main = main_clone.write().unwrap();
                    let target_sec = sec_cycle[(thread_idx + iter) % sec_cycle.len()];
                    main.navigate_to(target_sec);
                    main.set_interface_scale(100 + ((iter as u32) % 200));
                }

                // 3. Common groups concurrent addition & search
                {
                    let mut groups = groups_clone.write().unwrap();
                    let peer_id = (thread_idx * 1_000 + (iter % 100)) as u64;
                    groups.add_group(CommonGroupRow {
                        peer_id,
                        title: format!("Thread #{} Group #{}", thread_idx, iter),
                        username: format!("t{}_g{}", thread_idx, iter),
                        member_count: iter as u32,
                        is_selected: false,
                    });
                    groups.set_search_query(format!("Thread #{}", thread_idx));
                    let _ = groups.filtered_groups().len();
                }
            }
        }));
    }

    // Join all 16 threads and verify zero deadlocks, zero panics
    for handle in handles {
        handle.join().expect("Concurrent stress thread panicked");
    }

    // Final integrity checks after heavy multi-threaded contention
    let final_host = shared_host.read().unwrap();
    assert!(final_host.available_tabs.contains(&final_host.active_tab));

    let final_main = shared_main.read().unwrap();
    assert!(final_main.active_section.is_some());
    assert!(final_main.interface_scale_pct >= 100 && final_main.interface_scale_pct <= 300);

    let final_groups = shared_groups.read().unwrap();
    assert!(final_groups.groups.len() > 0);
}
