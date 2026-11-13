//! Milestone 3 Comprehensive Settings Regression Test Suite
//! Features:
//! - Feature F9: Settings Navigation Router & Breadcrumb Stack
//! - Feature F10: Materialize All 22 Settings Sections in Settings Panel
//! - Feature F11: LocalStorage Preference Binding & Dual-Engine Persistence
//! Mandate: Genuine state machine and real logic, zero mocks, zero fake hardcoded passes.

use gpui_telegram_client::storage::Localstorage;
use gpui_telegram_desktop::app::preferences::{AppPreferences, ThemeMode};

use gpui_telegram_desktop::views::nav_rail::NavDestination;
use gpui_telegram_desktop::views::settings::sections::{
    BadgeCountType, ChatThemePreset, SettingsActiveSessionsSections, SettingsAdvancedSections,
    SettingsBlockedPeersSections, SettingsBusinessSections, SettingsCallsSections,
    SettingsChatSections, SettingsCreditsSections, SettingsFoldersSections,
    SettingsGlobalTtlSections, SettingsInformationSections, SettingsLocalPasscodeSections,
    SettingsLocalStorageSections, SettingsMainSections, SettingsNotificationsReactionsSections,
    SettingsNotificationsSections, SettingsNotificationsTypeSections, SettingsPasskeysSections,
    SettingsPremiumSections, SettingsPrivacySecuritySections, SettingsShortcutsSections,
    SettingsWebsitesSections,
};
use gpui_telegram_desktop::views::settings::settings_power_saving::SettingsPowerSaving;
use gpui_telegram_desktop::views::settings::settings_search::SettingsSearch;
use gpui_telegram_desktop::views::settings::settings_type::SettingsSectionType;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static TEST_DIR_COUNTER: AtomicU64 = AtomicU64::new(1);

/// RAII Temporary Directory for test isolation with automatic disk cleanup
struct TestTempDir {
    path: PathBuf,
}

impl TestTempDir {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "gpui_m3_settings_test_{}_{}_{}",
            std::process::id(),
            TEST_DIR_COUNTER.fetch_add(1, Ordering::Relaxed),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&path);
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestTempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

// ============================================================================
// SUITE 1: Feature F9 Settings Router & Navigation Breadcrumb Stack Tests
// ============================================================================

/// Navigation Router Simulator replicating the exact AppShell settings routing contract
struct SettingsRouterSimulator {
    current_section: SettingsSectionType,
    history_stack: Vec<SettingsSectionType>,
    active_destination: NavDestination,
}

impl SettingsRouterSimulator {
    fn new() -> Self {
        Self {
            current_section: SettingsSectionType::Main,
            history_stack: Vec::new(),
            active_destination: NavDestination::Settings,
        }
    }

    fn push(&mut self, next: SettingsSectionType) {
        if self.current_section == next {
            return;
        }
        if self.history_stack.last().copied() != Some(self.current_section) {
            self.history_stack.push(self.current_section);
        }
        self.current_section = next;
        self.active_destination = NavDestination::Settings;
    }

    fn pop(&mut self) -> bool {
        if let Some(prev) = self.history_stack.pop() {
            self.current_section = prev;
            true
        } else if !self.current_section.is_root() {
            let parent = self.current_section.parent_section().unwrap_or(SettingsSectionType::Main);
            self.current_section = parent;
            true
        } else {
            self.close();
            false
        }
    }

    fn close(&mut self) {
        self.history_stack.clear();
        self.current_section = SettingsSectionType::Main;
        self.active_destination = NavDestination::Chats;
    }
}

#[test]
fn test_settings_section_catalog_and_metadata_24_variants() {
    let all_variants = [
        SettingsSectionType::Main,
        SettingsSectionType::Information,
        SettingsSectionType::Notifications,
        SettingsSectionType::NotificationsType,
        SettingsSectionType::NotificationsReactions,
        SettingsSectionType::PrivacySecurity,
        SettingsSectionType::BlockedPeers,
        SettingsSectionType::ActiveSessions,
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
        SettingsSectionType::GlobalTtl,
        SettingsSectionType::PowerSaving,
        SettingsSectionType::LocalStorage,
        SettingsSectionType::Business,
        SettingsSectionType::Language,
        SettingsSectionType::Search,
    ];

    assert_eq!(all_variants.len(), 24, "Must cover all 24 settings section types");

    for section in all_variants {
        // Unique slug identifier
        assert!(!section.id_str().is_empty());

        // Localized titles
        let en_title = section.display_title_localized(false);
        let zh_title = section.display_title_localized(true);
        assert!(!en_title.is_empty(), "English title must not be empty");
        assert!(!zh_title.is_empty(), "Chinese title must not be empty");
        assert_ne!(en_title, zh_title, "Bilingual titles must be distinct for {}", section.id_str());

        // Emoji Icon
        assert!(!section.icon().is_empty(), "Icon must not be empty for {}", section.id_str());

        // Root status
        if section == SettingsSectionType::Main {
            assert!(section.is_root());
            assert_eq!(section.parent_section(), None);
        } else {
            assert!(!section.is_root());
            assert!(section.parent_section().is_some());
        }

        // Breadcrumb path must start with Main and end with current section
        let path = section.breadcrumb_path();
        assert!(!path.is_empty());
        assert_eq!(path[0], SettingsSectionType::Main);
        assert_eq!(*path.last().unwrap(), section);
    }
}

#[test]
fn test_settings_sub_section_hierarchy_mapping() {
    // Privacy & Security sub-sections
    assert_eq!(SettingsSectionType::BlockedPeers.parent_section(), Some(SettingsSectionType::PrivacySecurity));
    assert_eq!(SettingsSectionType::ActiveSessions.parent_section(), Some(SettingsSectionType::PrivacySecurity));
    assert_eq!(SettingsSectionType::LocalPasscode.parent_section(), Some(SettingsSectionType::PrivacySecurity));
    assert_eq!(SettingsSectionType::Passkeys.parent_section(), Some(SettingsSectionType::PrivacySecurity));
    assert_eq!(SettingsSectionType::GlobalTtl.parent_section(), Some(SettingsSectionType::PrivacySecurity));
    assert_eq!(SettingsSectionType::Websites.parent_section(), Some(SettingsSectionType::PrivacySecurity));

    // Notifications sub-sections
    assert_eq!(SettingsSectionType::NotificationsType.parent_section(), Some(SettingsSectionType::Notifications));
    assert_eq!(SettingsSectionType::NotificationsReactions.parent_section(), Some(SettingsSectionType::Notifications));

    // Advanced sub-sections
    assert_eq!(SettingsSectionType::LocalStorage.parent_section(), Some(SettingsSectionType::Advanced));
    assert_eq!(SettingsSectionType::PowerSaving.parent_section(), Some(SettingsSectionType::Advanced));

    // Breadcrumb path lengths
    assert_eq!(SettingsSectionType::Main.breadcrumb_path().len(), 1);
    assert_eq!(
        SettingsSectionType::BlockedPeers.breadcrumb_path(),
        vec![
            SettingsSectionType::Main,
            SettingsSectionType::PrivacySecurity,
            SettingsSectionType::BlockedPeers
        ]
    );
    assert_eq!(
        SettingsSectionType::LocalStorage.breadcrumb_path(),
        vec![
            SettingsSectionType::Main,
            SettingsSectionType::Advanced,
            SettingsSectionType::LocalStorage
        ]
    );
}

#[test]
fn test_settings_router_simulator_lifecycle_and_stack() {
    let mut router = SettingsRouterSimulator::new();
    assert_eq!(router.current_section, SettingsSectionType::Main);
    assert!(router.history_stack.is_empty());
    assert_eq!(router.active_destination, NavDestination::Settings);

    // Forward navigation: Main -> PrivacySecurity
    router.push(SettingsSectionType::PrivacySecurity);
    assert_eq!(router.current_section, SettingsSectionType::PrivacySecurity);
    assert_eq!(router.history_stack, vec![SettingsSectionType::Main]);

    // Forward navigation: PrivacySecurity -> BlockedPeers
    router.push(SettingsSectionType::BlockedPeers);
    assert_eq!(router.current_section, SettingsSectionType::BlockedPeers);
    assert_eq!(
        router.history_stack,
        vec![SettingsSectionType::Main, SettingsSectionType::PrivacySecurity]
    );

    // Duplicate push is safely ignored (stack doesn't duplicate)
    router.push(SettingsSectionType::BlockedPeers);
    assert_eq!(
        router.history_stack,
        vec![SettingsSectionType::Main, SettingsSectionType::PrivacySecurity]
    );

    // Backward navigation: BlockedPeers -> PrivacySecurity
    assert!(router.pop());
    assert_eq!(router.current_section, SettingsSectionType::PrivacySecurity);
    assert_eq!(router.history_stack, vec![SettingsSectionType::Main]);

    // Backward navigation: PrivacySecurity -> Main
    assert!(router.pop());
    assert_eq!(router.current_section, SettingsSectionType::Main);
    assert!(router.history_stack.is_empty());

    // Back from root Main exits settings back to Chats
    assert!(!router.pop());
    assert_eq!(router.active_destination, NavDestination::Chats);
    assert_eq!(router.current_section, SettingsSectionType::Main);

    // Close resets stack
    router.push(SettingsSectionType::Chat);
    router.push(SettingsSectionType::Language);
    router.close();
    assert_eq!(router.current_section, SettingsSectionType::Main);
    assert!(router.history_stack.is_empty());
    assert_eq!(router.active_destination, NavDestination::Chats);
}

// ============================================================================
// SUITE 2: Feature F10 22 Section Views Materialization & Panic-Free Rendering
// ============================================================================

#[test]
fn test_materialize_all_22_settings_sections() {
    // 1. SettingsMainSections
    let main = SettingsMainSections::new();
    assert_eq!(main.interface_scale_pct, 100);
    assert!(main.is_visible);
    assert!(main.active_section.is_none());

    // 2. SettingsInformationSections
    let info = SettingsInformationSections::new();
    assert_eq!(info.first_name, "Pavel");
    assert!(info.bio_limit() >= 70);

    // 3. SettingsNotificationsSections
    let mut notif = SettingsNotificationsSections::new();
    assert!(notif.private_chats_enabled);
    assert!(notif.sound_enabled);
    assert_eq!(notif.badge_count_type, BadgeCountType::CountMessages);
    notif.toggle_sound();
    assert!(!notif.sound_enabled);

    // 4. SettingsNotificationsReactionsSections
    let react = SettingsNotificationsReactionsSections::new();
    assert!(react.notify_reactions);
    assert!(react.sound_enabled);

    // 5. SettingsNotificationsTypeSections
    let mut ntype = SettingsNotificationsTypeSections::new("Channel Updates");
    assert_eq!(ntype.chat_type_name, "Channel Updates");
    assert!(ntype.alerts_enabled);
    ntype.toggle_alerts();
    assert!(!ntype.alerts_enabled);

    // 6. SettingsPrivacySecuritySections
    let mut priv_sec = SettingsPrivacySecuritySections::new();
    assert_eq!(priv_sec.ttl_months, 6);
    assert!(priv_sec.two_factor_enabled);
    priv_sec.set_ttl_months(12);
    assert_eq!(priv_sec.ttl_months, 12);

    // 7. SettingsBlockedPeersSections
    let mut blocked = SettingsBlockedPeersSections::new();
    assert_eq!(blocked.blocked_list.len(), 0);
    blocked.block_peer(5001, "Spammer", "2026-11-08");
    assert_eq!(blocked.blocked_list.len(), 1);
    assert!(blocked.unblock_peer(5001));
    assert_eq!(blocked.blocked_list.len(), 0);

    // 8. SettingsActiveSessionsSections
    let sessions = SettingsActiveSessionsSections::new();
    assert!(sessions.sessions.len() >= 1);

    // 9. SettingsGlobalTtlSections
    let ttl = SettingsGlobalTtlSections::new();
    assert_eq!(ttl.current_ttl, 0);

    // 10. SettingsPasskeysSections
    let passkeys = SettingsPasskeysSections::new();
    assert_eq!(passkeys.passkeys.len(), 2);
    assert!(passkeys.can_register);


    // 11. SettingsLocalPasscodeSections
    let mut passcode = SettingsLocalPasscodeSections::new();
    assert!(passcode.has_passcode);
    assert!(passcode.remove_passcode("correct"));
    assert!(!passcode.has_passcode);


    // 12. SettingsChatSections
    let mut chat = SettingsChatSections::new();
    assert_eq!(chat.font_size, 13);
    assert_eq!(chat.theme_preset, ChatThemePreset::Day);
    assert!(chat.send_on_enter);
    chat.set_font_size(18);
    assert_eq!(chat.font_size, 18);

    // 13. SettingsFoldersSections
    let folders = SettingsFoldersSections::new();
    assert!(folders.folders.len() >= 2);

    // 14. SettingsAdvancedSections
    let adv = SettingsAdvancedSections::new();
    assert!(adv.opengl_enabled);
    assert!(adv.tray_icon_enabled);

    // 15. SettingsLocalStorageSections
    let storage = SettingsLocalStorageSections::new();
    assert_eq!(storage.max_cache_gb, 16);
    assert!(storage.total_bytes() > 0);

    // 16. SettingsCallsSections
    let mut calls = SettingsCallsSections::new();
    assert!(!calls.mic_test_active);
    calls.toggle_mic_test();
    assert!(calls.mic_test_active);

    // 17. SettingsCreditsSections
    let mut credits = SettingsCreditsSections::new();
    assert_eq!(credits.balance_stars, 1500);
    assert!(credits.is_withdrawal_enabled);
    credits.topup(500);
    assert_eq!(credits.balance_stars, 2000);

    // 18. SettingsPremiumSections
    let mut prem = SettingsPremiumSections::new();
    assert_eq!(prem.tiers.len(), 3);
    assert!(prem.is_premium_active);
    prem.select_tier(6);
    assert_eq!(prem.selected_months, 6);


    // 19. SettingsShortcutsSections
    let shortcuts = SettingsShortcutsSections::new();
    assert!(shortcuts.shortcuts.len() >= 5);

    // 20. SettingsWebsitesSections
    let web = SettingsWebsitesSections::new();
    assert!(web.websites.len() >= 1);

    // 21. SettingsBusinessSections
    let biz = SettingsBusinessSections::new();
    assert!(biz.business_active);
    assert_eq!(biz.tools.len(), 6);


    // 22. SettingsPowerSaving
    let mut power = SettingsPowerSaving::new();
    assert!(!power.battery_saver_enabled);
    assert_eq!(power.battery_threshold_percent, 20);
    assert!(power.animate_stickers_in_chat);
    power.battery_saver_enabled = true;
    assert!(power.battery_saver_enabled);

    // Search Component
    let search = SettingsSearch::with_default_entries();
    assert!(!search.all_entries.is_empty());
}

#[test]
fn test_section_value_clamping_and_boundary_safety() {
    let mut chat = SettingsChatSections::new();
    // Font size clamping: [12, 30]
    chat.set_font_size(0);
    assert_eq!(chat.font_size, 12, "Should clamp below minimum to 12");
    chat.set_font_size(500);
    assert_eq!(chat.font_size, 30, "Should clamp above maximum to 30");

    let mut priv_sec = SettingsPrivacySecuritySections::new();
    // Invalid TTL months fallback
    priv_sec.set_ttl_months(99);
    assert_eq!(priv_sec.ttl_months, 6, "Invalid TTL should fallback to default 6");

    let mut ntype = SettingsNotificationsTypeSections::new("Work");
    ntype.add_exception(1001, "Alice", true, Some("custom.wav".to_string()));
    assert_eq!(ntype.exceptions.len(), 1);
    assert_eq!(ntype.exceptions[0].peer_id, 1001);
    assert_eq!(ntype.exceptions[0].name, "Alice");
    assert!(ntype.exceptions[0].is_muted);

    // Adding same peer exception replaces it
    ntype.add_exception(1001, "Alice Muted", false, None);
    assert_eq!(ntype.exceptions.len(), 1);
    assert!(!ntype.exceptions[0].is_muted);
    assert_eq!(ntype.exceptions[0].name, "Alice Muted");
}

// ============================================================================
// SUITE 3: Feature F11 Preference Persistence & TDF Synchronization Tests
// ============================================================================

#[test]
fn test_preferences_full_roundtrip_serialization() {
    let mut prefs = AppPreferences::default();
    prefs.theme_mode = ThemeMode::Light;
    prefs.notifications.sound_enabled = false;
    prefs.notifications.private_chats = false;
    prefs.auto_download.photos = false;
    prefs.auto_download.max_file_size_mb = 50;
    prefs.appearance.font_size = 18;
    prefs.appearance.chat_preset = "Tinted".to_string();
    prefs.appearance.animations_enabled = false;
    prefs.localization.language_code = "en-US".to_string();
    prefs.interface_scale_pct = 125;
    prefs.power_saving_enabled = true;

    let json = serde_json::to_string_pretty(&prefs).expect("JSON serialization must succeed");
    let loaded: AppPreferences = serde_json::from_str(&json).expect("JSON deserialization must succeed");

    assert_eq!(loaded, prefs);
    assert_eq!(loaded.theme_mode, ThemeMode::Light);
    assert!(!loaded.notifications.sound_enabled);
    assert_eq!(loaded.auto_download.max_file_size_mb, 50);
    assert_eq!(loaded.appearance.font_size, 18);
    assert_eq!(loaded.appearance.chat_preset, "Tinted");
    assert!(!loaded.appearance.animations_enabled);
    assert_eq!(loaded.localization.language_code, "en-US");
    assert_eq!(loaded.interface_scale_pct, 125);
    assert!(loaded.power_saving_enabled);
}

#[test]
fn test_preferences_atomic_disk_persistence_with_posix_0600() {
    let tmp = TestTempDir::new();
    let base_dir = tmp.path();

    let mut prefs = AppPreferences::default();
    prefs.theme_mode = ThemeMode::Dark;
    prefs.notifications.sound_enabled = false;
    prefs.localization.language_code = "zh-CN".to_string();
    prefs.appearance.font_size = 16;

    // Save preferences atomically
    prefs.save_to_dir(base_dir).expect("save_to_dir must succeed");

    let prefs_file = base_dir.join("preferences.json");
    assert!(prefs_file.is_file(), "preferences.json must exist after save");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(&prefs_file).expect("File metadata must be accessible");
        let mode = metadata.permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "File permissions must be strictly POSIX 0600 mode");
    }

    // Simulate complete application reboot: load from disk
    let loaded = AppPreferences::load_from_dir(base_dir).expect("Must load valid JSON preferences");
    assert_eq!(loaded.theme_mode, ThemeMode::Dark);
    assert!(!loaded.notifications.sound_enabled);
    assert_eq!(loaded.localization.language_code, "zh-CN");
    assert_eq!(loaded.appearance.font_size, 16);
}

#[test]
fn test_localstorage_tdf_block_0x5e_persistence_roundtrip() {
    let tmp = TestTempDir::new();
    let base_dir = tmp.path();

    let mut prefs = AppPreferences::default();
    prefs.theme_mode = ThemeMode::Light;
    prefs.appearance.font_size = 22;
    prefs.localization.language_code = "en-US".to_string();

    let serialized = serde_json::to_vec(&prefs).expect("Serialize to bytes");

    // 1. Write through Localstorage TDF engine
    let mut storage = Localstorage::new(base_dir);
    storage.set_application_settings_raw(serialized.clone());
    assert_eq!(storage.application_settings_raw(), serialized.as_slice());
    storage.write_settings().expect("write_settings must succeed");

    // 2. Simulate re-opening Localstorage from disk
    let mut fresh_storage = Localstorage::new(base_dir);
    let ctx = fresh_storage.read_settings().expect("read_settings must succeed");
    assert_eq!(ctx.application_settings_raw, serialized);

    let decoded: AppPreferences = serde_json::from_slice(&ctx.application_settings_raw)
        .expect("Deserialization from TDF block 0x5e must succeed");
    assert_eq!(decoded.theme_mode, ThemeMode::Light);
    assert_eq!(decoded.appearance.font_size, 22);
    assert_eq!(decoded.localization.language_code, "en-US");
}

#[test]
fn test_preferences_missing_and_corrupt_file_graceful_fallback() {
    let tmp = TestTempDir::new();
    let base_dir = tmp.path();

    let default_prefs = AppPreferences::default();

    // 1. Empty directory (missing preferences.json): load returns default
    let loaded = AppPreferences::load_from_dir(base_dir);
    assert!(loaded.is_none());

    let mut local_storage = Localstorage::new(base_dir);
    let resolved = AppPreferences::load(base_dir, &local_storage);
    assert_eq!(resolved, default_prefs);

    // 2. Corrupt JSON on disk: load gracefully falls back to default
    let corrupt_file = base_dir.join("preferences.json");
    fs::write(&corrupt_file, b"MALFORMED_JSON_GARBAGE_{}{]").expect("Write corrupt file");

    let loaded_corrupt = AppPreferences::load_from_dir(base_dir);
    assert!(loaded_corrupt.is_none());

    let resolved_corrupt = AppPreferences::load(base_dir, &local_storage);
    assert_eq!(resolved_corrupt, default_prefs);

    // 3. Fallback to Localstorage when disk JSON is corrupt
    let mut custom_prefs = AppPreferences::default();
    custom_prefs.appearance.font_size = 26;
    let tdf_bytes = serde_json::to_vec(&custom_prefs).unwrap();
    local_storage.set_application_settings_raw(tdf_bytes);

    let fallback_tdf = AppPreferences::load(base_dir, &local_storage);
    assert_eq!(fallback_tdf.appearance.font_size, 26);
}

#[test]
fn test_dual_engine_sync_consistency() {
    let tmp = TestTempDir::new();
    let base_dir = tmp.path();

    let mut storage = Localstorage::new(base_dir);
    let mut prefs = AppPreferences::default();
    prefs.theme_mode = ThemeMode::Dark;
    prefs.appearance.font_size = 19;
    prefs.localization.language_code = "zh-CN".to_string();

    // Persist via dual engine
    prefs.save_to_dir(base_dir).expect("save_to_dir");
    prefs.sync_to_localstorage(&mut storage).expect("sync_to_localstorage");

    // Verify both engines hold exact identical serialized representations
    let disk_prefs = AppPreferences::load_from_dir(base_dir).expect("disk read");
    let tdf_prefs: AppPreferences = serde_json::from_slice(storage.application_settings_raw()).expect("tdf decode");

    assert_eq!(disk_prefs, prefs);
    assert_eq!(tdf_prefs, prefs);
}
