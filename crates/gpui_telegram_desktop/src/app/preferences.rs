//! Persistent Application Preferences
//! Upstream: Telegram/SourceFiles/storage/localstorage.h (DBI_APPLICATION_SETTINGS 0x5e)
//! Milestone 3: Feature F11 — Preference Binding & Dual-Engine Persistence

use gpui_telegram_client::storage::Localstorage;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

static TMP_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
    System,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub enabled: bool,
    pub sound_enabled: bool,
    pub private_chats: bool,
    pub groups: bool,
    pub channels: bool,
    pub flash_taskbar: bool,
    pub badge_count_messages: bool,
    pub include_muted_in_badge: bool,
}

impl Default for NotificationPreferences {
    fn default() -> Self {
        Self {
            enabled: true,
            sound_enabled: true,
            private_chats: true,
            groups: true,
            channels: false,
            flash_taskbar: true,
            badge_count_messages: true,
            include_muted_in_badge: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoDownloadPreferences {
    pub photos: bool,
    pub videos: bool,
    pub files: bool,
    pub max_file_size_mb: i64,
}

impl Default for AutoDownloadPreferences {
    fn default() -> Self {
        Self {
            photos: true,
            videos: true,
            files: false,
            max_file_size_mb: 10,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatAppearancePreferences {
    pub theme_mode: ThemeMode,
    pub chat_preset: String,
    pub font_size: u32,
    pub animations_enabled: bool,
    pub send_on_enter: bool,
    pub suggest_stickers_by_emoji: bool,
    pub loop_animated_stickers: bool,
    pub blur_wallpaper: bool,
}

impl Default for ChatAppearancePreferences {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::Dark,
            chat_preset: "Day".to_string(),
            font_size: 13,
            animations_enabled: true,
            send_on_enter: true,
            suggest_stickers_by_emoji: true,
            loop_animated_stickers: true,
            blur_wallpaper: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizationPreferences {
    pub language_code: String,
}

impl Default for LocalizationPreferences {
    fn default() -> Self {
        Self {
            language_code: "zh-CN".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppPreferences {
    pub theme_mode: ThemeMode,
    pub notifications: NotificationPreferences,
    pub auto_download: AutoDownloadPreferences,
    pub appearance: ChatAppearancePreferences,
    pub localization: LocalizationPreferences,
    pub interface_scale_pct: u32,
    pub power_saving_enabled: bool,
}

impl Default for AppPreferences {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::Dark,
            notifications: NotificationPreferences::default(),
            auto_download: AutoDownloadPreferences::default(),
            appearance: ChatAppearancePreferences::default(),
            localization: LocalizationPreferences::default(),
            interface_scale_pct: 100,
            power_saving_enabled: false,
        }
    }
}

impl AppPreferences {
    pub fn save_to_dir(&self, dir: &Path) -> Result<(), std::io::Error> {
        fs::create_dir_all(dir)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(dir, fs::Permissions::from_mode(0o700));
        }

        let target_path = dir.join("preferences.json");
        let tmp_path = dir.join(format!(
            ".preferences.tmp.{}.{}.{}",
            std::process::id(),
            TMP_COUNTER.fetch_add(1, Ordering::Relaxed),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let write_res = (|| -> Result<(), std::io::Error> {
            use std::io::Write;
            let mut opts = fs::OpenOptions::new();
            opts.write(true).create(true).truncate(true);

            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                opts.mode(0o600);
            }

            let mut file = opts.open(&tmp_path)?;
            file.write_all(json.as_bytes())?;
            file.sync_all()?;
            drop(file);

            fs::rename(&tmp_path, &target_path)?;
            Ok(())
        })();

        if let Err(e) = write_res {
            let _ = fs::remove_file(&tmp_path);
            return Err(e);
        }
        Ok(())
    }

    pub fn load_from_dir(dir: &Path) -> Option<Self> {
        let path = dir.join("preferences.json");
        let content = fs::read_to_string(&path).ok()?;
        serde_json::from_str(&content).ok()
    }

    pub fn load(dir: &Path, local_storage: &Localstorage) -> Self {
        if let Some(prefs) = Self::load_from_dir(dir) {
            return prefs;
        }

        let raw = local_storage.application_settings_raw();
        if !raw.is_empty() {
            if let Ok(prefs) = serde_json::from_slice::<Self>(raw) {
                return prefs;
            }
        }

        Self::default()
    }

    pub fn sync_to_localstorage(&self, local_storage: &mut Localstorage) -> Result<(), String> {
        let raw = serde_json::to_vec(self).map_err(|e| e.to_string())?;
        local_storage.set_application_settings_raw(raw);
        local_storage.sync()
    }
}
