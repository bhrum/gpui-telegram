//! Adversarial Stress & Fuzz Test Suite for Feature F11: Preferences Persistence,
//! Atomic POSIX 0600 Mode, LocalStorage TDF Sync, and Boundary Value Clamping.
//!
//! Archetype: EMPIRICAL CHALLENGER (challenger_m3_2)
//! Mandate: Real logic, zero fake mocks, aggressive stress, and empirical verification.

use gpui_telegram_client::storage::Localstorage;
use gpui_telegram_desktop::app::preferences::{AppPreferences, ThemeMode};
use gpui_telegram_desktop::views::settings::sections::{
    SettingsChatSections, SettingsCreditsSections, SettingsLocalStorageSections,
    SettingsPrivacySecuritySections,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

static TEST_DIR_COUNTER: AtomicU64 = AtomicU64::new(1000);

/// RAII Temporary Directory for test isolation with automatic disk cleanup
struct AdversarialTempDir {
    path: PathBuf,
}

impl AdversarialTempDir {
    fn new(prefix: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "gpui_m3_fuzz_{}_{}_{}_{}",
            prefix,
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

impl Drop for AdversarialTempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[cfg(unix)]
type LibcModeT = u16;
#[cfg(all(unix, not(target_os = "macos")))]
type LibcModeT = u32;

#[cfg(unix)]
unsafe extern "C" {
    fn umask(mask: LibcModeT) -> LibcModeT;
}

/// RAII Process Umask Guard to safely test permissions under hostile process umasks
struct UmaskGuard {
    #[cfg(unix)]
    original: LibcModeT,
}

impl UmaskGuard {
    fn set(target_mask: u32) -> Self {
        #[cfg(unix)]
        unsafe {
            let original = umask(target_mask as LibcModeT);
            Self { original }
        }
        #[cfg(not(unix))]
        Self {}
    }
}

impl Drop for UmaskGuard {
    fn drop(&mut self) {
        #[cfg(unix)]
        unsafe {
            umask(self.original);
        }
    }
}

// ============================================================================
// SUITE 1: Concurrency Stress (100 Concurrent Threads & High-Frequency Churn)
// ============================================================================

#[test]
fn test_adversarial_100_concurrent_threads_atomic_save_and_load_integrity() {
    let tmp = AdversarialTempDir::new("concurrency_100");
    let base_dir = tmp.path().to_path_buf();

    // Seed directory with initial valid default preferences
    let initial_prefs = AppPreferences::default();
    initial_prefs
        .save_to_dir(&base_dir)
        .expect("Initial save must succeed");

    let num_threads = 100;
    let cycles_per_thread = 10;
    let successful_writes = Arc::new(AtomicUsize::new(0));
    let successful_reads = Arc::new(AtomicUsize::new(0));
    let corrupt_reads = Arc::new(AtomicUsize::new(0));
    let shutdown_signal = Arc::new(AtomicBool::new(false));

    // Spawn 10 observer reader threads continuously probing preferences.json
    let mut reader_handles = Vec::new();
    for _ in 0..10 {
        let dir = base_dir.clone();
        let s_reads = Arc::clone(&successful_reads);
        let c_reads = Arc::clone(&corrupt_reads);
        let stop = Arc::clone(&shutdown_signal);

        reader_handles.push(thread::spawn(move || {
            while !stop.load(Ordering::Relaxed) {
                match AppPreferences::load_from_dir(&dir) {
                    Some(prefs) => {
                        // Invariant verification: structure must be sane
                        assert!(
                            prefs.theme_mode == ThemeMode::Dark
                                || prefs.theme_mode == ThemeMode::Light
                                || prefs.theme_mode == ThemeMode::System
                        );
                        assert!(prefs.appearance.font_size >= 12 && prefs.appearance.font_size <= 30);
                        s_reads.fetch_add(1, Ordering::Relaxed);
                    }
                    None => {
                        // None is only valid if file was unreadable or torn
                        c_reads.fetch_add(1, Ordering::Relaxed);
                    }
                }
                thread::yield_now();
            }
        }));
    }

    // Spawn 100 concurrent writer threads
    let mut writer_handles = Vec::new();
    for i in 0..num_threads {
        let dir = base_dir.clone();
        let s_writes = Arc::clone(&successful_writes);

        writer_handles.push(thread::spawn(move || {
            for round in 0..cycles_per_thread {
                let mut prefs = AppPreferences::default();
                prefs.theme_mode = match (i + round) % 3 {
                    0 => ThemeMode::Dark,
                    1 => ThemeMode::Light,
                    _ => ThemeMode::System,
                };
                prefs.appearance.font_size = 12 + ((i + round) % 19) as u32; // [12..=30]
                prefs.notifications.sound_enabled = (i + round) % 2 == 0;
                prefs.auto_download.max_file_size_mb = (i * 10 + round) as i64;
                prefs.localization.language_code = format!("zh-t{}-r{}", i, round);
                prefs.interface_scale_pct = 100 + ((i % 5) * 25) as u32;

                match prefs.save_to_dir(&dir) {
                    Ok(()) => {
                        s_writes.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(e) => {
                        panic!("Thread {} write error: {}", i, e);
                    }
                }
                thread::yield_now();
            }
        }));
    }

    // Wait for all 100 writers to complete
    for handle in writer_handles {
        handle.join().expect("Writer thread should not panic");
    }

    // Stop observer readers
    shutdown_signal.store(true, Ordering::Relaxed);
    for handle in reader_handles {
        handle.join().expect("Reader thread should not panic");
    }

    assert_eq!(
        successful_writes.load(Ordering::Relaxed),
        num_threads * cycles_per_thread,
        "All 1000 atomic writes must complete successfully"
    );
    assert_eq!(
        corrupt_reads.load(Ordering::Relaxed),
        0,
        "Zero torn writes or corrupted reads allowed during concurrent execution"
    );
    assert!(
        successful_reads.load(Ordering::Relaxed) > 50,
        "Observer threads must have completed dozens of successful concurrent reads"
    );

    // Verify no temporary swap files were leaked
    let entries: Vec<_> = fs::read_dir(&base_dir)
        .expect("read_dir")
        .filter_map(|e| e.ok())
        .collect();
    for entry in &entries {
        let file_name = entry.file_name().to_string_lossy().to_string();
        assert!(
            !file_name.starts_with(".preferences.tmp"),
            "Leaked temporary swap file found: {}",
            file_name
        );
    }
    assert_eq!(
        entries.len(),
        1,
        "Only the canonical preferences.json should remain"
    );

    // Final read check
    let final_prefs = AppPreferences::load_from_dir(&base_dir)
        .expect("Canonical preferences.json must parse cleanly after all threads complete");
    assert!(final_prefs.appearance.font_size >= 12 && final_prefs.appearance.font_size <= 30);
}

#[test]
fn test_adversarial_concurrent_readers_and_writers_high_frequency_churn() {
    let tmp = AdversarialTempDir::new("high_frequency_churn");
    let base_dir = tmp.path().to_path_buf();

    let initial = AppPreferences::default();
    initial.save_to_dir(&base_dir).unwrap();

    let stop = Arc::new(AtomicBool::new(false));
    let total_reads = Arc::new(AtomicUsize::new(0));
    let total_writes = Arc::new(AtomicUsize::new(0));
    let failed_reads = Arc::new(AtomicUsize::new(0));

    let mut handles = Vec::new();

    // 25 writers
    for id in 0..25 {
        let dir = base_dir.clone();
        let s = Arc::clone(&stop);
        let tw = Arc::clone(&total_writes);
        handles.push(thread::spawn(move || {
            let mut step = 0;
            while !s.load(Ordering::Relaxed) {
                step += 1;
                let mut p = AppPreferences::default();
                p.appearance.font_size = 12 + ((id + step) % 19) as u32;
                p.notifications.sound_enabled = step % 2 == 0;
                p.power_saving_enabled = (id + step) % 3 == 0;
                if p.save_to_dir(&dir).is_ok() {
                    tw.fetch_add(1, Ordering::Relaxed);
                }
            }
        }));
    }

    // 25 readers
    for _ in 0..25 {
        let dir = base_dir.clone();
        let s = Arc::clone(&stop);
        let tr = Arc::clone(&total_reads);
        let fr = Arc::clone(&failed_reads);
        handles.push(thread::spawn(move || {
            while !s.load(Ordering::Relaxed) {
                match AppPreferences::load_from_dir(&dir) {
                    Some(p) => {
                        assert!(p.appearance.font_size >= 12 && p.appearance.font_size <= 30);
                        tr.fetch_add(1, Ordering::Relaxed);
                    }
                    None => {
                        fr.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        }));
    }

    // Let them churn intensely for 800ms
    thread::sleep(Duration::from_millis(800));
    stop.store(true, Ordering::Relaxed);

    for h in handles {
        h.join().expect("Thread should not panic");
    }

    let writes = total_writes.load(Ordering::Relaxed);
    let reads = total_reads.load(Ordering::Relaxed);
    let fails = failed_reads.load(Ordering::Relaxed);

    assert!(writes >= 20, "Should have executed multiple fsync writes (got {})", writes);
    assert!(reads >= 50, "Should have executed multiple concurrent reads (got {})", reads);
    assert_eq!(fails, 0, "Zero read failures allowed during high-frequency churn");
}

// ============================================================================
// SUITE 2: Fault Injection & Robustness Fuzzing (Truncated, Malformed, Binary)
// ============================================================================

#[test]
fn test_adversarial_fault_injection_truncated_json_corpus() {
    let tmp = AdversarialTempDir::new("fault_truncated");
    let base_dir = tmp.path();
    let file_path = base_dir.join("preferences.json");
    let local_storage = Localstorage::new(base_dir);

    let sample_prefs = AppPreferences::default();
    let full_json = serde_json::to_string_pretty(&sample_prefs).expect("Serialize sample prefs");
    assert!(full_json.len() > 100);

    // Test truncation at every single byte index from 1 to len - 1
    for cutoff in 1..full_json.len() {
        let truncated = &full_json[..cutoff];
        fs::write(&file_path, truncated.as_bytes()).expect("Write truncated JSON");

        // AppPreferences::load_from_dir must gracefully return None without panic
        let from_dir = AppPreferences::load_from_dir(base_dir);
        assert!(
            from_dir.is_none(),
            "Truncated JSON (len {}) should not deserialize cleanly",
            cutoff
        );

        // AppPreferences::load must fall back safely to default without panic
        let recovered = AppPreferences::load(base_dir, &local_storage);
        assert_eq!(
            recovered,
            AppPreferences::default(),
            "Must fall back to default preferences on truncated JSON at cutoff {}",
            cutoff
        );
    }
}

#[test]
fn test_adversarial_fault_injection_malformed_syntax() {
    let tmp = AdversarialTempDir::new("fault_malformed");
    let base_dir = tmp.path();
    let file_path = base_dir.join("preferences.json");
    let local_storage = Localstorage::new(base_dir);

    let malformed_cases: &[(&str, &[u8])] = &[
        ("0_byte_empty", b""),
        ("whitespace_only", b"   \n\t  \r\n   "),
        ("unclosed_curly_brace", b"{\"theme_mode\": \"dark\""),
        ("dangling_trailing_comma", b"{\"theme_mode\": \"dark\",}"),
        ("json_primitive_number", b"1234567890"),
        ("json_primitive_string", b"\"just a plain string\""),
        ("json_primitive_boolean", b"true"),
        ("json_primitive_null", b"null"),
        ("json_primitive_array", b"[1, 2, 3, 4, 5]"),
        ("sql_injection", b"' OR '1'='1; DROP TABLE users; --"),
        ("html_markup", b"<html><body><h1>Hack</h1></body></html>"),
        ("xml_document", b"<?xml version=\"1.0\"?><prefs><theme>dark</theme></prefs>"),
        ("embedded_null_byte", b"{\"theme_mode\": \x00 \"dark\"}"),
        ("deeply_nested_arrays", b"[[[[[[[[[[[[[[[1]]]]]]]]]]]]]]]"),
        ("incomplete_key", b"{\"theme_"),
        ("incomplete_value", b"{\"theme_mode\": \"da"),
        ("invalid_unicode_surrogate", b"{\"chat_preset\": \"\xed\xa0\x80\"}"),
        ("repeated_duplicate_keys", b"{\"theme_mode\": \"dark\", \"theme_mode\": \"light\"}"),
    ];

    for (name, payload) in malformed_cases {
        fs::write(&file_path, payload).unwrap_or_else(|e| panic!("Failed to write {}: {}", name, e));

        let res_dir = AppPreferences::load_from_dir(base_dir);
        assert!(
            res_dir.is_none(),
            "Malformed case '{}' must return None from load_from_dir",
            name
        );

        let recovered = AppPreferences::load(base_dir, &local_storage);
        assert_eq!(
            recovered,
            AppPreferences::default(),
            "Malformed case '{}' must gracefully recover default configuration",
            name
        );
    }
}

#[test]
fn test_adversarial_fault_injection_random_binary_fuzzing() {
    let tmp = AdversarialTempDir::new("fault_binary_fuzz");
    let base_dir = tmp.path();
    let file_path = base_dir.join("preferences.json");
    let local_storage = Localstorage::new(base_dir);

    // Pseudorandom pseudo-fuzzer generating 200 arbitrary binary blobs
    let mut state: u64 = 0xdeadbeefcafebabe;
    let lcg = |s: &mut u64| -> u8 {
        *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((*s >> 24) & 0xff) as u8
    };

    for iter in 0..200 {
        let len = ((iter * 37) % 2048) + 1;
        let mut buffer = Vec::with_capacity(len);
        for _ in 0..len {
            buffer.push(lcg(&mut state));
        }

        fs::write(&file_path, &buffer).expect("Write fuzz buffer");

        let loaded = AppPreferences::load_from_dir(base_dir);
        if let Some(prefs) = loaded {
            // Highly unlikely random bytes form valid AppPreferences JSON, but if so, it must be valid
            assert!(
                prefs.theme_mode == ThemeMode::Dark
                    || prefs.theme_mode == ThemeMode::Light
                    || prefs.theme_mode == ThemeMode::System
            );
        } else {
            // Normal expected case: random binary fails gracefully
            let recovered = AppPreferences::load(base_dir, &local_storage);
            assert_eq!(
                recovered,
                AppPreferences::default(),
                "Must recover default preferences on binary fuzz at iter {}",
                iter
            );
        }
    }
}

// ============================================================================
// SUITE 3: POSIX 0600 Security Audit & Umask Resilience
// ============================================================================

#[test]
fn test_adversarial_posix_0600_security_audit_under_hostile_umasks() {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let umask_test_cases = [
            (0o000, "wide_open_0000"),
            (0o077, "strict_0077"),
            (0o022, "standard_0022"),
            (0o002, "group_write_0002"),
        ];

        for (mask, label) in umask_test_cases {
            let tmp = AdversarialTempDir::new(label);
            let base_dir = tmp.path();

            let prefs = AppPreferences::default();

            // Set hostile process umask
            let _guard = UmaskGuard::set(mask);

            // Execute atomic persistence
            prefs
                .save_to_dir(base_dir)
                .unwrap_or_else(|e| panic!("save_to_dir failed under umask {:o}: {}", mask, e));

            let prefs_file = base_dir.join("preferences.json");
            let file_meta = fs::metadata(&prefs_file).expect("File metadata");
            let file_mode = file_meta.permissions().mode() & 0o777;

            // Security assertion 1: Target file must be strictly 0o600
            assert_eq!(
                file_mode, 0o600,
                "Under umask {:o} ({}), preferences.json mode must be strictly 0o600, found 0o{:o}",
                mask, label, file_mode
            );

            // Security assertion 2: No world or group bits leaked
            assert_eq!(
                file_mode & 0o077,
                0,
                "preferences.json must not leak any group or world permissions under umask {:o}",
                mask
            );

            // Security assertion 3: Directory permissions must be 0o700
            let dir_meta = fs::metadata(base_dir).expect("Directory metadata");
            let dir_mode = dir_meta.permissions().mode() & 0o777;
            assert_eq!(
                dir_mode, 0o700,
                "Target directory must be secured to 0o700 under umask {:o}, found 0o{:o}",
                mask, dir_mode
            );
        }
    }
}

#[test]
fn test_adversarial_temp_file_lifecycle_and_non_leakage() {
    let tmp = AdversarialTempDir::new("temp_lifecycle");
    let base_dir = tmp.path();

    let prefs = AppPreferences::default();
    for round in 0..50 {
        prefs.save_to_dir(base_dir).unwrap();

        // Immediately inspect directory: no temp swap files should linger
        let temp_files: Vec<_> = fs::read_dir(base_dir)
            .expect("read_dir")
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with(".preferences.tmp"))
            .collect();

        assert!(
            temp_files.is_empty(),
            "Round {}: Temporary swap files leaked: {:?}",
            round,
            temp_files.iter().map(|f| f.file_name()).collect::<Vec<_>>()
        );
    }
}

// ============================================================================
// SUITE 4: Boundary Values & Clamping (Font Size, Themes, Locales, Section Logic)
// ============================================================================

#[test]
fn test_adversarial_boundary_font_size_clamping() {
    let mut chat = SettingsChatSections::new();

    // Invariant: font_size must clamp strictly within [12, 30]
    let boundary_cases = [
        (0, 12),
        (1, 12),
        (10, 12),
        (11, 12),
        (12, 12),
        (13, 13),
        (20, 20),
        (29, 29),
        (30, 30),
        (31, 30),
        (50, 30),
        (500, 30),
        (99999, 30),
        (u32::MAX, 30),
    ];

    for (input, expected) in boundary_cases {
        chat.set_font_size(input);
        assert_eq!(
            chat.font_size, expected,
            "Font size {} must clamp to {}",
            input, expected
        );
    }

    // JSON edge cases for font_size
    let tmp = AdversarialTempDir::new("boundary_font");
    let base_dir = tmp.path();
    let local_storage = Localstorage::new(base_dir);
    let prefs_file = base_dir.join("preferences.json");

    // Case 1: Negative font size in JSON (-999) fails serde -> falls back to default 13
    let negative_font_json = r#"{
        "theme_mode": "dark",
        "notifications": {
            "enabled": true, "sound_enabled": true, "private_chats": true,
            "groups": true, "channels": false, "flash_taskbar": true,
            "badge_count_messages": true, "include_muted_in_badge": false
        },
        "auto_download": { "photos": true, "videos": true, "files": false, "max_file_size_mb": 10 },
        "appearance": {
            "theme_mode": "dark", "chat_preset": "Day", "font_size": -999,
            "animations_enabled": true, "send_on_enter": true,
            "suggest_stickers_by_emoji": true, "loop_animated_stickers": true,
            "blur_wallpaper": false
        },
        "localization": { "language_code": "zh-CN" },
        "interface_scale_pct": 100,
        "power_saving_enabled": false
    }"#;

    fs::write(&prefs_file, negative_font_json.as_bytes()).unwrap();
    assert!(AppPreferences::load_from_dir(base_dir).is_none());
    let recovered = AppPreferences::load(base_dir, &local_storage);
    assert_eq!(recovered.appearance.font_size, 13);

    // Case 2: Floating point font size (14.5) in JSON fails serde -> falls back to default 13
    let float_font_json = negative_font_json.replace("-999", "14.5");
    fs::write(&prefs_file, float_font_json.as_bytes()).unwrap();
    assert!(AppPreferences::load_from_dir(base_dir).is_none());
    let recovered = AppPreferences::load(base_dir, &local_storage);
    assert_eq!(recovered.appearance.font_size, 13);
}

#[test]
fn test_adversarial_boundary_max_file_size_mb() {
    let tmp = AdversarialTempDir::new("boundary_filesize");
    let base_dir = tmp.path();

    let extreme_sizes = [
        -1i64, // Unlimited or disabled flag
        0i64,
        1i64,
        1024i64,
        100_000i64,
        i64::MAX,
        i64::MIN,
    ];

    for size in extreme_sizes {
        let mut prefs = AppPreferences::default();
        prefs.auto_download.max_file_size_mb = size;

        prefs.save_to_dir(base_dir).expect("save_to_dir");
        let loaded = AppPreferences::load_from_dir(base_dir).expect("load_from_dir");

        assert_eq!(
            loaded.auto_download.max_file_size_mb, size,
            "max_file_size_mb {} must survive disk roundtrip perfectly",
            size
        );
    }
}

#[test]
fn test_adversarial_boundary_unknown_theme_strings() {
    let tmp = AdversarialTempDir::new("boundary_theme");
    let base_dir = tmp.path();
    let local_storage = Localstorage::new(base_dir);
    let prefs_file = base_dir.join("preferences.json");

    let invalid_themes = [
        "\"rainbow\"",
        "\"neon\"",
        "\"DARK\"",   // Case sensitive
        "\"LIGHT\"",  // Case sensitive
        "\"SYSTEM\"", // Case sensitive
        "\"\"",
        "12345",
        "null",
        "true",
        "{}",
        "[]",
    ];

    for theme_repr in invalid_themes {
        let json = format!(
            r#"{{
            "theme_mode": {},
            "notifications": {{
                "enabled": true, "sound_enabled": true, "private_chats": true,
                "groups": true, "channels": false, "flash_taskbar": true,
                "badge_count_messages": true, "include_muted_in_badge": false
            }},
            "auto_download": {{ "photos": true, "videos": true, "files": false, "max_file_size_mb": 10 }},
            "appearance": {{
                "theme_mode": "dark", "chat_preset": "Day", "font_size": 13,
                "animations_enabled": true, "send_on_enter": true,
                "suggest_stickers_by_emoji": true, "loop_animated_stickers": true,
                "blur_wallpaper": false
            }},
            "localization": {{ "language_code": "zh-CN" }},
            "interface_scale_pct": 100,
            "power_saving_enabled": false
        }}"#,
            theme_repr
        );

        fs::write(&prefs_file, json.as_bytes()).unwrap();
        assert!(
            AppPreferences::load_from_dir(base_dir).is_none(),
            "Theme repr {} should fail deserialization",
            theme_repr
        );
        let recovered = AppPreferences::load(base_dir, &local_storage);
        assert_eq!(
            recovered.theme_mode,
            ThemeMode::Dark,
            "Must fall back to default Dark theme for {}",
            theme_repr
        );
    }
}

#[test]
fn test_adversarial_boundary_invalid_locale_tags() {
    let tmp = AdversarialTempDir::new("boundary_locale");
    let base_dir = tmp.path();

    let locale_test_cases = [
        ("", false),
        ("zh", true),
        ("zh-CN", true),
        ("zh-TW", true),
        ("zh-Hans", true),
        ("en-US", false),
        ("ru-RU", false),
        ("🇨🇳", false),
        ("!@#$%^&*()_+", false),
        ("zh!@#", true),
    ];

    for (locale_str, expected_is_zh) in locale_test_cases {
        let mut prefs = AppPreferences::default();
        prefs.localization.language_code = locale_str.to_string();

        prefs.save_to_dir(base_dir).unwrap();
        let loaded = AppPreferences::load_from_dir(base_dir).unwrap();

        assert_eq!(loaded.localization.language_code, locale_str);
        let is_zh = loaded.localization.language_code.starts_with("zh");
        assert_eq!(
            is_zh, expected_is_zh,
            "Locale '{}' expected starts_with('zh') == {}",
            locale_str, expected_is_zh
        );
    }

    // Huge 64KB locale string
    let huge_locale = "zh-".to_string() + &"A".repeat(65536);
    let mut prefs = AppPreferences::default();
    prefs.localization.language_code = huge_locale.clone();
    prefs.save_to_dir(base_dir).unwrap();
    let loaded = AppPreferences::load_from_dir(base_dir).unwrap();
    assert_eq!(loaded.localization.language_code, huge_locale);
    assert!(loaded.localization.language_code.starts_with("zh"));
}

#[test]
fn test_adversarial_boundary_settings_sections_clamping() {
    // 1. SettingsPrivacySecuritySections TTL months fallback
    let mut priv_sec = SettingsPrivacySecuritySections::new();
    let test_ttls = [
        (0, 6),
        (1, 1),
        (2, 6),
        (3, 3),
        (4, 6),
        (5, 6),
        (6, 6),
        (12, 12),
        (18, 18),
        (24, 24),
        (25, 6),
        (99, 6),
        (u32::MAX, 6),
    ];

    for (input, expected) in test_ttls {
        priv_sec.set_ttl_months(input);
        assert_eq!(
            priv_sec.ttl_months, expected,
            "TTL input {} must resolve to {}",
            input, expected
        );
    }

    // 2. SettingsLocalStorageSections total bytes arithmetic
    let storage = SettingsLocalStorageSections::new();
    let total = storage.total_bytes();
    let sum_categories: u64 = storage.categories.iter().map(|c| c.bytes).sum();
    assert_eq!(total, sum_categories);
    assert!(total > 0);

    // 3. SettingsCreditsSections topup
    let mut credits = SettingsCreditsSections::new();
    let initial_balance = credits.balance_stars;
    credits.topup(0);
    assert_eq!(credits.balance_stars, initial_balance);
    credits.topup(5000);
    assert_eq!(credits.balance_stars, initial_balance + 5000);
}

// ============================================================================
// SUITE 5: LocalStorage TDF Block 0x5e Synchronization & Dual-Engine Matrix
// ============================================================================

#[test]
fn test_adversarial_tdf_block_0x5e_sync_fidelity() {
    let tmp = AdversarialTempDir::new("tdf_fidelity");
    let base_dir = tmp.path();

    let mut storage = Localstorage::new(base_dir);

    // Build fully customized preferences
    let mut prefs = AppPreferences::default();
    prefs.theme_mode = ThemeMode::System;
    prefs.notifications.sound_enabled = false;
    prefs.notifications.badge_count_messages = false;
    prefs.auto_download.photos = false;
    prefs.auto_download.videos = false;
    prefs.auto_download.files = true;
    prefs.auto_download.max_file_size_mb = 999;
    prefs.appearance.font_size = 28;
    prefs.appearance.chat_preset = "Tinted".to_string();
    prefs.appearance.animations_enabled = false;
    prefs.appearance.send_on_enter = false;
    prefs.localization.language_code = "zh-Hans-CN".to_string();
    prefs.interface_scale_pct = 150;
    prefs.power_saving_enabled = true;

    // 1. Sync through TDF engine
    prefs.sync_to_localstorage(&mut storage).expect("sync_to_localstorage");
    storage.write_settings().expect("write_settings");

    // 2. Re-open on clean instance
    let mut reloaded_storage = Localstorage::new(base_dir);
    let ctx = reloaded_storage.read_settings().expect("read_settings");
    assert!(!ctx.application_settings_raw.is_empty());

    let decoded: AppPreferences = serde_json::from_slice(&ctx.application_settings_raw)
        .expect("TDF raw payload must deserialize cleanly");

    assert_eq!(decoded, prefs, "TDF block 0x5e roundtrip fidelity mismatch");
}

#[test]
fn test_adversarial_dual_engine_precedence_and_fallback_matrix() {
    let tmp = AdversarialTempDir::new("dual_matrix");
    let base_dir = tmp.path();
    let prefs_file = base_dir.join("preferences.json");

    let default_prefs = AppPreferences::default();

    let mut disk_prefs = AppPreferences::default();
    disk_prefs.appearance.font_size = 14;
    disk_prefs.theme_mode = ThemeMode::Light;

    let mut tdf_prefs = AppPreferences::default();
    tdf_prefs.appearance.font_size = 22;
    tdf_prefs.theme_mode = ThemeMode::System;

    // Matrix Case 1: Valid Disk JSON + Valid TDF -> Disk JSON wins
    {
        disk_prefs.save_to_dir(base_dir).unwrap();
        let mut storage = Localstorage::new(base_dir);
        tdf_prefs.sync_to_localstorage(&mut storage).unwrap();

        let resolved = AppPreferences::load(base_dir, &storage);
        assert_eq!(
            resolved.appearance.font_size, 14,
            "Case 1: Disk JSON must take precedence over TDF"
        );
        assert_eq!(resolved.theme_mode, ThemeMode::Light);
    }

    // Matrix Case 2: Corrupt Disk JSON + Valid TDF -> TDF block 0x5e wins (Graceful recovery!)
    {
        fs::write(&prefs_file, b"CORRUPTED_DISK_PAYLOAD_{{[[").unwrap();
        let mut storage = Localstorage::new(base_dir);
        tdf_prefs.sync_to_localstorage(&mut storage).unwrap();

        let resolved = AppPreferences::load(base_dir, &storage);
        assert_eq!(
            resolved.appearance.font_size, 22,
            "Case 2: TDF block 0x5e must recover state when disk is corrupt"
        );
        assert_eq!(resolved.theme_mode, ThemeMode::System);
    }

    // Matrix Case 3: Missing Disk JSON + Valid TDF -> TDF block 0x5e wins
    {
        let _ = fs::remove_file(&prefs_file);
        let mut storage = Localstorage::new(base_dir);
        tdf_prefs.sync_to_localstorage(&mut storage).unwrap();

        let resolved = AppPreferences::load(base_dir, &storage);
        assert_eq!(
            resolved.appearance.font_size, 22,
            "Case 3: TDF block 0x5e must hydrate state when disk file is absent"
        );
        assert_eq!(resolved.theme_mode, ThemeMode::System);
    }

    // Matrix Case 4: Corrupt Disk JSON + Corrupt TDF -> Default configuration wins
    {
        fs::write(&prefs_file, b"CORRUPTED_DISK").unwrap();
        let mut storage = Localstorage::new(base_dir);
        storage.set_application_settings_raw(b"CORRUPTED_TDF_BYTES_123".to_vec());

        let resolved = AppPreferences::load(base_dir, &storage);
        assert_eq!(
            resolved, default_prefs,
            "Case 4: Default configuration must prevail when both engines are corrupt"
        );
    }

    // Matrix Case 5: Missing Disk JSON + Empty TDF -> Default configuration wins
    {
        let _ = fs::remove_file(&prefs_file);
        let storage = Localstorage::new(base_dir);

        let resolved = AppPreferences::load(base_dir, &storage);
        assert_eq!(
            resolved, default_prefs,
            "Case 5: Default configuration must prevail when both engines are empty"
        );
    }
}

#[test]
fn test_adversarial_tdf_block_0x5e_binary_garbage_resilience() {
    let tmp = AdversarialTempDir::new("tdf_garbage");
    let base_dir = tmp.path();

    let mut storage = Localstorage::new(base_dir);

    // Write completely invalid non-JSON bytes into TDF block 0x5e
    let corrupt_payload = vec![0xde, 0xad, 0xbe, 0xef, 0x00, 0xff, 0x42, 0x7f];
    storage.set_application_settings_raw(corrupt_payload);

    // AppPreferences::load must handle this without panic and return default
    let loaded = AppPreferences::load(base_dir, &storage);
    assert_eq!(loaded, AppPreferences::default());
}
