use std::cmp::Ordering;
use std::fs;
use std::path::Path;
use std::process::Command;

use gpui_telegram_desktop::app::{
    compare_semver, is_newer_version, parse_github_release_json, parse_release_manifest,
    DesktopUpdateState, DesktopUpdater, CURRENT_APP_VERSION,
};
use gpui_telegram_desktop::views::boxes::AboutBox;
use gpui_telegram_desktop::views::settings::sections::SettingsAdvancedSections;
use sha2::{Digest, Sha256};

fn create_mock_app_bundle(root: &Path, version: &str) -> std::path::PathBuf {
    let app = root.join("Fabushi.app");
    let contents = app.join("Contents");
    let macos = contents.join("MacOS");
    let resources = contents.join("Resources");
    fs::create_dir_all(&macos).unwrap();
    fs::create_dir_all(&resources).unwrap();

    let plist = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>Fabushi</string>
    <key>CFBundleDisplayName</key>
    <string>Fabushi</string>
    <key>CFBundleIdentifier</key>
    <string>com.ombhrum.fabushi</string>
    <key>CFBundleShortVersionString</key>
    <string>{version}</string>
    <key>CFBundleVersion</key>
    <string>{version}</string>
    <key>CFBundleExecutable</key>
    <string>gpui_telegram_desktop</string>
</dict>
</plist>
"#
    );
    fs::write(contents.join("Info.plist"), plist).unwrap();

    let exe = macos.join("gpui_telegram_desktop");
    let exe_script = format!("#!/bin/sh\necho \"Fabushi Desktop v{version}\"\n");
    fs::write(&exe, exe_script).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&exe, fs::Permissions::from_mode(0o755)).unwrap();
    }
    app
}

fn sha256_hex(path: &Path) -> String {
    let bytes = fs::read(path).unwrap();
    let digest = Sha256::digest(&bytes);
    format!("{:x}", digest)
}

#[test]
fn test_semver_comparison_and_prerelease_rules() {
    assert_eq!(compare_semver("1.2.1", "1.2.0"), Ordering::Greater);
    assert_eq!(compare_semver("v1.3.0", "1.2.9"), Ordering::Greater);
    assert_eq!(
        compare_semver("macos-main-v1.2.78", "1.2.77"),
        Ordering::Greater
    );
    assert_eq!(compare_semver("1.2.0", "1.2.0-beta.1"), Ordering::Greater);
    assert_eq!(compare_semver("1.2.0", "v1.2.0"), Ordering::Equal);

    assert!(is_newer_version("1.2.0", "1.2.1"));
    assert!(is_newer_version("1.2.0-rc.1", "1.2.0"));
    assert!(is_newer_version("1.2.0", "v2.0.0"));
    assert!(!is_newer_version("1.2.0", "1.2.0"));
    assert!(!is_newer_version("1.2.0", "1.1.9"));
    assert!(!is_newer_version("1.2.0", "1.2.0-beta.2"));
    assert!(!is_newer_version("1.2.0", ""));
}

#[test]
fn test_parse_manifest_and_github_release_fallback_json() {
    let manifest_json = r#"{
        "version": "1.3.0",
        "tag_name": "v1.3.0",
        "release_notes": "Auto-update support",
        "platforms": {
            "darwin-arm64": {
                "zip_url": "https://github.com/bhrum/gpui-telegram/releases/download/v1.3.0/Fabushi-1.3.0-macos-arm64.zip",
                "zip_sha256": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                "zip_size": 12345678,
                "dmg_url": "https://github.com/bhrum/gpui-telegram/releases/download/v1.3.0/Fabushi-1.3.0-macos-arm64.dmg",
                "pkg_url": "https://github.com/bhrum/gpui-telegram/releases/download/v1.3.0/Fabushi-1.3.0-macos-arm64.pkg"
            },
            "darwin-x86_64": {
                "zip_url": "https://github.com/bhrum/gpui-telegram/releases/download/v1.3.0/Fabushi-1.3.0-macos-x86_64.zip",
                "zip_sha256": "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB",
                "zip_size": 12345678
            }
        }
    }"#;
    let parsed = parse_release_manifest(manifest_json).expect("manifest should parse");
    assert_eq!(parsed.version, "1.3.0");
    assert!(parsed.asset_url.contains("Fabushi-1.3.0-macos-"));
    assert!(parsed.sha256.as_ref().is_some_and(|h| h.len() == 64));

    let gh_json = r#"{
        "tag_name": "v1.4.0",
        "name": "Fabushi v1.4.0",
        "body": "Release notes body",
        "assets": [
            {
                "name": "gpui_telegram-v1.4.0-linux-x86_64.tar.gz",
                "browser_download_url": "https://example.com/linux.tar.gz",
                "size": 100
            },
            {
                "name": "Fabushi-1.4.0-macos-arm64.dmg",
                "browser_download_url": "https://example.com/Fabushi-1.4.0-macos-arm64.dmg",
                "size": 200
            },
            {
                "name": "Fabushi-1.4.0-macos-arm64.pkg",
                "browser_download_url": "https://example.com/Fabushi-1.4.0-macos-arm64.pkg",
                "size": 210
            },
            {
                "name": "Fabushi-1.4.0-macos-arm64.zip",
                "browser_download_url": "https://example.com/Fabushi-1.4.0-macos-arm64.zip",
                "size": 300,
                "digest": "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            },
            {
                "name": "Fabushi-1.4.0-macos-x86_64.zip",
                "browser_download_url": "https://example.com/Fabushi-1.4.0-macos-x86_64.zip",
                "size": 305,
                "digest": "sha256:fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210"
            }
        ]
    }"#;
    let gh_parsed = parse_github_release_json(gh_json).expect("GitHub release JSON should parse");
    assert_eq!(gh_parsed.version, "1.4.0");
    assert!(gh_parsed.asset_url.ends_with(".zip"));
    assert!(gh_parsed.sha256.is_some());
    assert!(gh_parsed.dmg_url.as_deref().is_some_and(|u| u.ends_with(".dmg")));
    assert!(gh_parsed.pkg_url.as_deref().is_some_and(|u| u.ends_with(".pkg")));
}

#[test]
fn test_end_to_end_check_sha256_rejection_and_atomic_bundle_replacement() {
    let tmp = tempfile::tempdir().unwrap();
    let apps_dir = tmp.path().join("Applications");
    fs::create_dir_all(&apps_dir).unwrap();
    let installed_app = create_mock_app_bundle(&apps_dir, "1.2.0");

    let resolved = DesktopUpdater::resolve_target_app_bundle(
        &installed_app.join("Contents/MacOS/gpui_telegram_desktop"),
    );
    assert_eq!(resolved, installed_app);

    let build_dir = tmp.path().join("build_v130");
    let upgraded_app = create_mock_app_bundle(&build_dir, "1.3.0");
    let zip_path = tmp.path().join("Fabushi-1.3.0-macos-arm64.zip");

    let zip_ok = if Path::new("/usr/bin/ditto").exists() {
        Command::new("/usr/bin/ditto")
            .args([
                "-c",
                "-k",
                "--sequesterRsrc",
                "--keepParent",
                &upgraded_app.to_string_lossy(),
                &zip_path.to_string_lossy(),
            ])
            .status()
            .unwrap()
            .success()
    } else {
        Command::new("zip")
            .current_dir(&build_dir)
            .args(["-qr", &zip_path.to_string_lossy(), "Fabushi.app"])
            .status()
            .unwrap()
            .success()
    };
    assert!(zip_ok, "archive creation for e2e test must succeed");

    let real_sha = sha256_hex(&zip_path);
    let zip_url = format!("file://{}", zip_path.display());

    let bad_manifest_path = tmp.path().join("bad-latest-mac.json");
    let bad_manifest = serde_json::json!({
        "version": "1.3.0",
        "tag_name": "v1.3.0",
        "platforms": {
            "darwin-universal": {
                "zip_url": zip_url,
                "zip_sha256": "0000000000000000000000000000000000000000000000000000000000000000"
            }
        }
    });
    fs::write(&bad_manifest_path, serde_json::to_string(&bad_manifest).unwrap()).unwrap();

    let good_manifest_path = tmp.path().join("latest-mac.json");
    let good_manifest = serde_json::json!({
        "version": "1.3.0",
        "tag_name": "v1.3.0",
        "platforms": {
            "darwin-universal": {
                "zip_url": zip_url,
                "zip_sha256": real_sha,
                "zip_size": fs::metadata(&zip_path).unwrap().len()
            }
        }
    });
    fs::write(&good_manifest_path, serde_json::to_string(&good_manifest).unwrap()).unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let bad_feed = format!("file://{}", bad_manifest_path.display());
        let bad_info = DesktopUpdater::check_for_updates(Some(&bad_feed), "1.2.0")
            .await
            .unwrap()
            .expect("should detect 1.3.0");
        let staging_bad = tmp.path().join("staging_bad");
        let bad_res = DesktopUpdater::download_and_stage_update(&bad_info, &staging_bad, |_, _, _| {}).await;
        assert!(bad_res.is_err(), "tampered SHA-256 must fail verification");
        assert!(bad_res.unwrap_err().contains("SHA-256 verification failed"));

        let plist_still_old = fs::read_to_string(installed_app.join("Contents/Info.plist")).unwrap();
        assert!(plist_still_old.contains("<string>1.2.0</string>"));

        let good_feed = format!("file://{}", good_manifest_path.display());
        let up_to_date = DesktopUpdater::check_for_updates(Some(&good_feed), "1.3.0")
            .await
            .unwrap();
        assert!(up_to_date.is_none());

        let good_info = DesktopUpdater::check_for_updates(Some(&good_feed), "1.2.0")
            .await
            .unwrap()
            .expect("should detect newer 1.3.0");
        let staging_good = tmp.path().join("staging_good");
        let mut saw_progress = false;
        let staged_app = DesktopUpdater::download_and_stage_update(
            &good_info,
            &staging_good,
            |_, _, pct| {
                if pct == 100 {
                    saw_progress = true;
                }
            },
        )
        .await
        .expect("valid archive should stage cleanly");
        assert!(saw_progress);

        DesktopUpdater::replace_app_bundle_in_place(&staged_app, &installed_app)
            .expect("atomic replacement should succeed");

        let plist_new = fs::read_to_string(installed_app.join("Contents/Info.plist")).unwrap();
        assert!(plist_new.contains("<string>1.3.0</string>"));
        let out = Command::new(installed_app.join("Contents/MacOS/gpui_telegram_desktop"))
            .output()
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "Fabushi Desktop v1.3.0");
    });
}

#[test]
fn test_settings_advanced_and_about_fabushi_integration() {
    let about_legacy = AboutBox::new();
    assert!(about_legacy.version_text.contains("5.8.1"));

    let about_fabushi = AboutBox::new_fabushi();
    assert!(about_fabushi.version_text.contains(CURRENT_APP_VERSION));
    assert_eq!(about_fabushi.github_url, "https://github.com/bhrum/gpui-telegram");

    let mut adv = SettingsAdvancedSections::new();
    assert_eq!(adv.current_version, CURRENT_APP_VERSION);
    assert!(adv.auto_update_enabled);
    assert_eq!(adv.download_path, "~/Downloads/Fabushi");

    let avail = DesktopUpdateState::Available {
        info: gpui_telegram_desktop::app::AvailableUpdateInfo {
            version: "1.9.0".to_string(),
            asset_url: "https://example.com/Fabushi-1.9.0-macos-arm64.zip".to_string(),
            sha256: None,
            size_bytes: None,
            release_notes: None,
            dmg_url: None,
            pkg_url: None,
        },
    };
    let snap = avail.to_snapshot(true);
    adv.sync_update_snapshot(&snap);
    assert_eq!(adv.latest_version.as_deref(), Some("1.9.0"));
    assert!(adv.update_available);
}
