use gpui::{Context, Entity};

use crate::app::preferences::{AppPreferences, ThemeMode};
use crate::theme::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::app_shell::ModalContainer;
use crate::views::settings::sections::{
    SettingsActiveSessionsSections, SettingsAdvancedSections, SettingsBlockedPeersSections,
    SettingsBusinessSections, SettingsCallsSections, SettingsChatSections,
    SettingsCreditsSections, SettingsFoldersSections, SettingsGlobalTtlSections,
    SettingsInformationSections, SettingsLocalPasscodeSections, SettingsLocalStorageSections,
    SettingsMainSections, SettingsNotificationsReactionsSections, SettingsNotificationsSections,
    SettingsNotificationsTypeSections, SettingsPasskeysSections, SettingsPremiumSections,
    SettingsPrivacySecuritySections, SettingsShortcutsSections, SettingsWebsitesSections,
};
use crate::views::settings::settings_power_saving::SettingsPowerSaving;
use crate::views::settings::settings_search::SettingsSearch;
use crate::views::tdesktop_theme::resolve_desktop_palette;
use crate::views::{
    CallOverlay, CanonicalComposer, ChatSidebar, ComputerWorkbench, ContextInspector,
    ConversationStage, GlobalTasksWorkspace, IvViewer, MainMenuDrawer, MediaViewer, NavRail,
};

/// Maps persisted app preferences and optional desktop mode override into a full theme palette.
pub fn compute_shell_palette(
    prefs: &AppPreferences,
    explicit_mode: Option<TdesktopThemeMode>,
    accent_override: Option<u32>,
    bubble_radius_px: u32,
    wallpaper_id: Option<&str>,
) -> (TdesktopThemeMode, TdesktopThemePalette) {
    let mode = explicit_mode.unwrap_or_else(|| resolve_mode_from_preferences(prefs));
    let accent = accent_override.unwrap_or_else(|| mode.default_accent());
    let font_size = prefs.appearance.font_size.max(11);
    let palette = resolve_desktop_palette(
        mode.as_name(),
        accent,
        font_size,
        bubble_radius_px,
        wallpaper_id,
    );
    (mode, palette)
}

/// Derives the desktop theme mode from persisted `AppPreferences`.
pub fn resolve_mode_from_preferences(prefs: &AppPreferences) -> TdesktopThemeMode {
    let preset_mode = TdesktopThemeMode::from_name(&prefs.appearance.chat_preset);
    match prefs.theme_mode {
        ThemeMode::Light => {
            if matches!(preset_mode, TdesktopThemeMode::Day | TdesktopThemeMode::Classic) {
                preset_mode
            } else {
                TdesktopThemeMode::Day
            }
        }
        ThemeMode::Dark | ThemeMode::System => {
            if matches!(preset_mode, TdesktopThemeMode::Night | TdesktopThemeMode::Tinted) {
                preset_mode
            } else {
                TdesktopThemeMode::Night
            }
        }
    }
}

/// Maps `TdesktopThemeMode` back into `AppPreferences::theme_mode`.
pub fn tdesktop_mode_to_app_theme(mode: TdesktopThemeMode) -> ThemeMode {
    if mode.is_dark() {
        ThemeMode::Dark
    } else {
        ThemeMode::Light
    }
}

/// Returns the opposite day/night mode when toggling quick Night Mode from NavRail or MainMenu.
pub fn toggle_desktop_theme_mode(current: TdesktopThemeMode) -> TdesktopThemeMode {
    if current.is_dark() {
        TdesktopThemeMode::Day
    } else {
        TdesktopThemeMode::Night
    }
}

/// Immutable slice of all desktop view entities managed by `AppShell` for dynamic theme/locale sync.
pub struct ShellThemeTargets<'a> {
    pub nav_rail: &'a Entity<NavRail>,
    pub main_menu_drawer: &'a Entity<MainMenuDrawer>,
    pub sidebar: &'a Entity<ChatSidebar>,
    pub stage: &'a Entity<ConversationStage>,
    pub composer: &'a Entity<CanonicalComposer>,
    pub inspector: &'a Entity<ContextInspector>,
    pub modal_container: &'a Entity<ModalContainer>,
    pub tasks_workspace: &'a Entity<GlobalTasksWorkspace>,
    pub talk_mode_overlay: Option<&'a Entity<CallOverlay>>,
    pub computer_overlay: Option<&'a Entity<ComputerWorkbench>>,
    pub media_viewer_overlay: Option<&'a Entity<MediaViewer>>,
    pub iv_viewer_overlay: Option<&'a Entity<IvViewer>>,
    pub settings_main: &'a Entity<SettingsMainSections>,
    pub settings_information: &'a Entity<SettingsInformationSections>,
    pub settings_notifications: &'a Entity<SettingsNotificationsSections>,
    pub settings_notif_reactions: &'a Entity<SettingsNotificationsReactionsSections>,
    pub settings_notif_type: &'a Entity<SettingsNotificationsTypeSections>,
    pub settings_privacy: &'a Entity<SettingsPrivacySecuritySections>,
    pub settings_blocked_peers: &'a Entity<SettingsBlockedPeersSections>,
    pub settings_active_sessions: &'a Entity<SettingsActiveSessionsSections>,
    pub settings_local_passcode: &'a Entity<SettingsLocalPasscodeSections>,
    pub settings_passkeys: &'a Entity<SettingsPasskeysSections>,
    pub settings_global_ttl: &'a Entity<SettingsGlobalTtlSections>,
    pub settings_websites: &'a Entity<SettingsWebsitesSections>,
    pub settings_chat: &'a Entity<SettingsChatSections>,
    pub settings_folders: &'a Entity<SettingsFoldersSections>,
    pub settings_advanced: &'a Entity<SettingsAdvancedSections>,
    pub settings_local_storage: &'a Entity<SettingsLocalStorageSections>,
    pub settings_power_saving: &'a Entity<SettingsPowerSaving>,
    pub settings_calls: &'a Entity<SettingsCallsSections>,
    pub settings_credits: &'a Entity<SettingsCreditsSections>,
    pub settings_premium: &'a Entity<SettingsPremiumSections>,
    pub settings_shortcuts: &'a Entity<SettingsShortcutsSections>,
    pub settings_business: &'a Entity<SettingsBusinessSections>,
    pub settings_search: &'a Entity<SettingsSearch>,
}

macro_rules! sync_palette_and_lang {
    ($cx:expr, $pal:expr, $lang:expr, $($ent:expr),* $(,)?) => {
        $(
            $ent.update($cx, |view, _| {
                view.set_palette($pal.clone());
                view.set_language($lang);
            });
        )*
    };
}

/// Propagates the active `TdesktopThemePalette` and `i18n` locale string to every child view and modal.
pub fn propagate_theme_and_locale<V: 'static>(
    targets: ShellThemeTargets<'_>,
    palette: &TdesktopThemePalette,
    lang_id: &str,
    cx: &mut Context<V>,
) {
    sync_palette_and_lang!(
        cx,
        palette,
        lang_id,
        targets.nav_rail,
        targets.main_menu_drawer,
        targets.sidebar,
        targets.inspector,
        targets.modal_container,
        targets.tasks_workspace,
        targets.settings_main,
        targets.settings_information,
        targets.settings_notifications,
        targets.settings_notif_reactions,
        targets.settings_notif_type,
        targets.settings_privacy,
        targets.settings_blocked_peers,
        targets.settings_active_sessions,
        targets.settings_local_passcode,
        targets.settings_passkeys,
        targets.settings_global_ttl,
        targets.settings_websites,
        targets.settings_chat,
        targets.settings_folders,
        targets.settings_advanced,
        targets.settings_local_storage,
        targets.settings_power_saving,
        targets.settings_calls,
        targets.settings_credits,
        targets.settings_premium,
        targets.settings_shortcuts,
        targets.settings_business,
        targets.settings_search,
    );

    targets.stage.update(cx, |s, _| s.apply_theme(palette.clone(), lang_id));
    targets.composer.update(cx, |c, _| c.apply_theme(palette.clone(), lang_id));

    if let Some(overlay) = targets.talk_mode_overlay {
        overlay.update(cx, |o, _| {
            o.set_palette(palette.clone());
            o.set_language(lang_id);
        });
    }
    if let Some(overlay) = targets.computer_overlay {
        overlay.update(cx, |o, _| {
            o.set_palette(palette.clone());
            o.set_language(lang_id);
        });
    }
    if let Some(overlay) = targets.media_viewer_overlay {
        overlay.update(cx, |o, _| o.apply_theme(palette.clone(), lang_id));
    }
    if let Some(overlay) = targets.iv_viewer_overlay {
        overlay.update(cx, |o, _| o.apply_theme(palette.clone(), lang_id));
    }
}
