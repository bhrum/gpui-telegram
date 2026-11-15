use gpui::{Context, Entity};

use crate::app::preferences::{AppPreferences, ThemeMode};
use crate::theme::{TdesktopThemeMode, TdesktopThemePalette};
use crate::components::bots::bot_attach_web_view::BotAttachWebView;
use crate::views::app_shell::ModalContainer;
use crate::views::auth::AuthWizardView;
use crate::views::boxes::peers::create_managed_bot_box::CreateManagedBotBoxPeers;
use crate::views::settings::SettingsRouter;
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
    )
    .with_ui_scale(prefs.interface_scale_pct)
    .with_blur_wallpaper(prefs.appearance.blur_wallpaper);
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
    pub auth_wizard: &'a Entity<AuthWizardView>,
    pub settings_router: &'a Entity<SettingsRouter>,
    pub miniapp_overlay: Option<&'a Entity<BotAttachWebView>>,
    pub create_bot_box: Option<&'a Entity<CreateManagedBotBoxPeers>>,
}

macro_rules! sync_palette_and_lang {
    ($cx:expr, $pal:expr, $lang:expr, $($ent:expr),* $(,)?) => {
        $(
            $ent.update($cx, |view, vcx| {
                view.set_palette($pal.clone());
                view.set_language($lang);
                vcx.notify();
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
    );

    let is_zh = lang_id.starts_with("zh");
    targets.auth_wizard.update(cx, |w, wcx| {
        w.apply_theme_and_locale(palette.clone(), is_zh);
        wcx.notify();
    });
    targets
        .settings_router
        .update(cx, |sr, scx| sr.apply_theme_and_locale(palette.clone(), is_zh, scx));

    targets.stage.update(cx, |s, scx| {
        s.apply_theme(palette.clone(), lang_id);
        scx.notify();
    });
    targets.composer.update(cx, |c, ccx| {
        c.apply_theme(palette.clone(), lang_id);
        ccx.notify();
    });

    if let Some(overlay) = targets.talk_mode_overlay {
        sync_palette_and_lang!(cx, palette, lang_id, overlay);
    }
    if let Some(overlay) = targets.computer_overlay {
        sync_palette_and_lang!(cx, palette, lang_id, overlay);
    }
    if let Some(overlay) = targets.miniapp_overlay {
        sync_palette_and_lang!(cx, palette, lang_id, overlay);
    }
    if let Some(box_ent) = targets.create_bot_box {
        sync_palette_and_lang!(cx, palette, lang_id, box_ent);
    }
    if let Some(overlay) = targets.media_viewer_overlay {
        overlay.update(cx, |o, ocx| {
            o.apply_theme(palette.clone(), lang_id);
            ocx.notify();
        });
    }
    if let Some(overlay) = targets.iv_viewer_overlay {
        overlay.update(cx, |o, ocx| {
            o.apply_theme(palette.clone(), lang_id);
            ocx.notify();
        });
    }
}
