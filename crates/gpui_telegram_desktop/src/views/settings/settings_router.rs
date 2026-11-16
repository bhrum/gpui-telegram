//! Lazy-materialized Settings Router (`SettingsRouter` + `SettingsRouterEvent`).
//! Eliminates eager startup allocation of all 23 settings section entities by
//! initializing each section on demand, hydrating from `AppPreferences`,
//! applying active palette/locale, and wiring `cx.observe` subscriptions.

use crate::app::preferences::AppPreferences;
use crate::components::ui::TextInputController;
use crate::theme::{TdesktopThemeMode, TdesktopThemePalette};
use crate::views::boxes::modal_types::ModalDialogKind;
use crate::views::settings::sections::{
    settings_ai_providers::SettingsAiProvidersEvent,
    settings_computers::SettingsComputersEvent,
    settings_folders::ChatFolderItem,
    settings_information::ProfileEditField,
    settings_local_passcode::AutoLockTimeout,
    settings_notifications::BadgeCountType,
    settings_notifications_reactions::ReactionsNotifySource,
    SettingsActiveSessionsSections, SettingsAdvancedSections, SettingsAiProvidersSections,
    SettingsBlockedPeersSections, SettingsBusinessSections, SettingsCallsSections,
    SettingsChatSections, SettingsComputersSections, SettingsCreditsSections,
    SettingsFoldersSections, SettingsGlobalTtlSections, SettingsInformationSections,
    SettingsLocalPasscodeSections, SettingsLocalStorageSections, SettingsMainSections,
    SettingsNotificationsReactionsSections, SettingsNotificationsSections,
    SettingsNotificationsTypeSections, SettingsPasskeysSections, SettingsPremiumSections,
    SettingsPrivacySecuritySections, SettingsShortcutsSections, SettingsWebsitesSections,
};
use crate::views::settings::settings_power_saving::SettingsPowerSaving;
use crate::views::settings::settings_search::SettingsSearch;
use crate::views::settings::settings_type::SettingsSectionType;
use gpui::{App, AppContext as _, Context, Entity, EventEmitter};

/// High-level events emitted by `SettingsRouter` to `AppShell`.
#[derive(Debug, Clone)]
pub enum SettingsRouterEvent {
    CloseRequested,
    LogoutRequested,
    LockAppNow,
    CopyClipboard(String),
    UpdateInterfaceScale(u32),
    OpenModal {
        title: String,
        kind: ModalDialogKind,
        width: f32,
        height: f32,
    },
    ApplyTheme {
        mode: TdesktopThemeMode,
        accent: u32,
        wallpaper: String,
    },
    UpdateChatPrefs {
        font_size: u32,
        bubble_radius: u32,
        send_on_enter: bool,
        suggest_stickers: bool,
        loop_animated_stickers: bool,
        blur_wallpaper: bool,
    },
    SaveProfile {
        first_name: String,
        last_name: String,
        username: Option<String>,
        phone: String,
        bio: String,
    },
    TwoFactorMutation(Option<String>),
    PushUserSettings,
    RevokeSession(String),
    RevokeOtherSessions,
    UnblockPeer {
        user_id: u64,
        remaining: usize,
    },
    UpdateLocalPasscode {
        disable: bool,
        hash: Option<String>,
        auto_lock_seconds: u32,
    },
    UpdateGlobalTtl(u32),
    ApplyGlobalTtlToExistingChats(u32),
    UpdateNotificationPrefs {
        private_chats: bool,
        groups: bool,
        channels: bool,
        sound_enabled: bool,
        flash_taskbar: bool,
        include_muted_in_badge: bool,
        badge_count_messages: bool,
    },
    UpdateNotificationReactionsPrefs {
        notify_reactions: bool,
        notify_stories: bool,
        contacts_only: bool,
        sound_enabled: bool,
    },
    UpdateNotificationTypePrefs {
        alerts_enabled: bool,
        preview_enabled: bool,
        sound_enabled: bool,
    },
    UpdateFolderTags(bool),
    CreateFolder {
        id: u32,
        title: String,
    },
    DeleteFolder(u32),
    UpdateAdvancedPrefs {
        ask_download_path: bool,
        opengl_enabled: bool,
        native_window_frame: bool,
        tray_icon_enabled: bool,
        auto_update_enabled: bool,
    },
    OpenExportDataModal,
    UpdateLocalStoragePrefs {
        max_cache_gb: u32,
        retention_weeks: u32,
    },
    ClearLocalCache,
    UpdateCallPrefs {
        input_device: String,
        output_device: String,
        camera_device: String,
    },
    ToggleMicTest(bool),
    UpdatePowerSavingPrefs {
        enabled: bool,
        threshold_pct: u32,
        animate_stickers_chat: bool,
        animate_stickers_panel: bool,
        animate_emoji_panel: bool,
        autoplay_gifs: bool,
        autoplay_videos: bool,
    },
    TopupStars(u32),
    WithdrawStars,
    ActivatePremium(u32),
    UpdatePasskeysPrefs(Vec<String>),
    UpdateWebsitesPrefs(Vec<String>),
    UpdateShortcutsPrefs {
        enabled: bool,
        filter: String,
    },
    UpdateBusinessPrefs(Vec<String>),
    Computers(SettingsComputersEvent),
    AiProviders(SettingsAiProvidersEvent),
    SelectLanguage(&'static str),
    ToggleLanguage,
    Navigated(SettingsSectionType),
}

/// Manages settings navigation stack, modal rendering, and lazy instantiation of all settings sections.
pub struct SettingsRouter {
    pub settings_section: SettingsSectionType,
    pub settings_stack: Vec<SettingsSectionType>,
    pub palette: TdesktopThemePalette,
    pub lang_id: &'static str,
    pub is_chinese_locale: bool,
    pub preferences: AppPreferences,
    pub settings_main: Option<Entity<SettingsMainSections>>,
    pub settings_information: Option<Entity<SettingsInformationSections>>,
    pub settings_notifications: Option<Entity<SettingsNotificationsSections>>,
    pub settings_notif_reactions: Option<Entity<SettingsNotificationsReactionsSections>>,
    pub settings_notif_type: Option<Entity<SettingsNotificationsTypeSections>>,
    pub settings_privacy: Option<Entity<SettingsPrivacySecuritySections>>,
    pub settings_blocked_peers: Option<Entity<SettingsBlockedPeersSections>>,
    pub settings_active_sessions: Option<Entity<SettingsActiveSessionsSections>>,
    pub settings_local_passcode: Option<Entity<SettingsLocalPasscodeSections>>,
    pub settings_passkeys: Option<Entity<SettingsPasskeysSections>>,
    pub settings_global_ttl: Option<Entity<SettingsGlobalTtlSections>>,
    pub settings_websites: Option<Entity<SettingsWebsitesSections>>,
    pub settings_chat: Option<Entity<SettingsChatSections>>,
    pub settings_folders: Option<Entity<SettingsFoldersSections>>,
    pub settings_advanced: Option<Entity<SettingsAdvancedSections>>,
    pub settings_local_storage: Option<Entity<SettingsLocalStorageSections>>,
    pub settings_power_saving: Option<Entity<SettingsPowerSaving>>,
    pub settings_calls: Option<Entity<SettingsCallsSections>>,
    pub settings_credits: Option<Entity<SettingsCreditsSections>>,
    pub settings_premium: Option<Entity<SettingsPremiumSections>>,
    pub settings_shortcuts: Option<Entity<SettingsShortcutsSections>>,
    pub settings_business: Option<Entity<SettingsBusinessSections>>,
    pub settings_computers: Option<Entity<SettingsComputersSections>>,
    pub settings_ai_providers: Option<Entity<SettingsAiProvidersSections>>,
    pub settings_search: Option<Entity<SettingsSearch>>,
}

impl EventEmitter<SettingsRouterEvent> for SettingsRouter {}

macro_rules! sync_loaded_theme {
    ($palette:expr, $lang_id:expr, $cx:expr, $($slot:expr),* $(,)?) => {
        $(
            if let Some(ref ent) = $slot {
                ent.update($cx, |view, view_cx| {
                    view.set_palette($palette);
                    view.set_language($lang_id);
                    view_cx.notify();
                });
            }
        )*
    };
}

macro_rules! lazy_section {
    ($self:ident, $cx:expr, $slot:ident, $build:expr, $observer:path) => {{
        if let Some(ref existing) = $self.$slot {
            return existing.clone();
        }
        let pal = $self.palette;
        let lang = $self.lang_id;
        let build = $build;
        let ent = $cx.new(|_| {
            let mut view = build(pal, lang);
            view.set_palette(pal);
            view.set_language(lang);
            view
        });
        $observer(&ent, $cx);
        $self.$slot = Some(ent.clone());
        ent
    }};
    ($self:ident, $cx:expr, $slot:ident, $build:expr, $hydrate:path, $observer:path) => {{
        if let Some(ref existing) = $self.$slot {
            return existing.clone();
        }
        let pal = $self.palette;
        let lang = $self.lang_id;
        let prefs = &$self.preferences;
        let build = $build;
        let ent = $cx.new(|_| {
            let mut view = build(pal, lang);
            $hydrate(prefs, &mut view);
            view.set_palette(pal);
            view.set_language(lang);
            view
        });
        $observer(&ent, $cx);
        $self.$slot = Some(ent.clone());
        ent
    }};
}

impl SettingsRouter {
    /// Creates a new `SettingsRouter` with all 23 settings section entities unallocated (`None`).
    pub fn new(
        palette: TdesktopThemePalette,
        is_chinese_locale: bool,
        preferences: AppPreferences,
    ) -> Self {
        let lang_id = if is_chinese_locale { "zh-CN" } else { "en" };
        Self {
            settings_section: SettingsSectionType::Main,
            settings_stack: Vec::new(),
            palette,
            lang_id,
            is_chinese_locale,
            preferences,
            settings_main: None,
            settings_information: None,
            settings_notifications: None,
            settings_notif_reactions: None,
            settings_notif_type: None,
            settings_privacy: None,
            settings_blocked_peers: None,
            settings_active_sessions: None,
            settings_local_passcode: None,
            settings_passkeys: None,
            settings_global_ttl: None,
            settings_websites: None,
            settings_chat: None,
            settings_folders: None,
            settings_advanced: None,
            settings_local_storage: None,
            settings_power_saving: None,
            settings_calls: None,
            settings_credits: None,
            settings_premium: None,
            settings_shortcuts: None,
            settings_business: None,
            settings_computers: None,
            settings_ai_providers: None,
            settings_search: None,
        }
    }

    /// Updates active theme palette and locale on currently materialized sections without allocating `None` slots.
    pub fn apply_theme_and_locale(
        &mut self,
        palette: TdesktopThemePalette,
        is_chinese_locale: bool,
        cx: &mut Context<Self>,
    ) {
        self.palette = palette;
        self.is_chinese_locale = is_chinese_locale;
        self.lang_id = if is_chinese_locale { "zh-CN" } else { "en" };
        sync_loaded_theme!(
            self.palette,
            self.lang_id,
            cx,
            self.settings_main,
            self.settings_information,
            self.settings_notifications,
            self.settings_notif_reactions,
            self.settings_notif_type,
            self.settings_privacy,
            self.settings_blocked_peers,
            self.settings_active_sessions,
            self.settings_local_passcode,
            self.settings_passkeys,
            self.settings_global_ttl,
            self.settings_websites,
            self.settings_chat,
            self.settings_folders,
            self.settings_advanced,
            self.settings_local_storage,
            self.settings_power_saving,
            self.settings_calls,
            self.settings_credits,
            self.settings_premium,
            self.settings_shortcuts,
            self.settings_business,
            self.settings_computers,
            self.settings_ai_providers,
            self.settings_search,
        );
        cx.notify();
    }

    /// Stores updated preferences and re-hydrates any already-materialized sections without allocating `None` slots.
    pub fn sync_preferences(&mut self, prefs: AppPreferences, cx: &mut Context<Self>) {
        macro_rules! rehydrate_loaded {
            ($prefs:expr, $cx:expr, $(($slot:expr, $hydrate:path)),* $(,)?) => {
                $(
                    if let Some(ref ent) = $slot {
                        ent.update($cx, |sec, sec_cx| {
                            $hydrate($prefs, sec);
                            sec_cx.notify();
                        });
                    }
                )*
            };
        }
        self.preferences = prefs;
        let p = &self.preferences;
        rehydrate_loaded!(
            p,
            cx,
            (self.settings_main, Self::hydrate_main),
            (self.settings_chat, Self::hydrate_chat),
            (self.settings_notifications, Self::hydrate_notifications),
            (self.settings_notif_reactions, Self::hydrate_notif_reactions),
            (self.settings_notif_type, Self::hydrate_notif_type),
            (self.settings_local_storage, Self::hydrate_local_storage),
            (self.settings_advanced, Self::hydrate_advanced),
            (self.settings_power_saving, Self::hydrate_power_saving),
            (self.settings_calls, Self::hydrate_calls),
            (self.settings_local_passcode, Self::hydrate_local_passcode),
            (self.settings_privacy, Self::hydrate_privacy),
            (self.settings_folders, Self::hydrate_folders),
            (self.settings_passkeys, Self::hydrate_passkeys),
            (self.settings_websites, Self::hydrate_websites),
            (self.settings_shortcuts, Self::hydrate_shortcuts),
            (self.settings_business, Self::hydrate_business),
        );
        cx.notify();
    }

    fn hydrate_main(prefs: &AppPreferences, m: &mut SettingsMainSections) {
        m.interface_scale_pct = prefs.interface_scale_pct.clamp(100, 300);
    }

    fn hydrate_notifications(prefs: &AppPreferences, sn: &mut SettingsNotificationsSections) {
        sn.private_chats_enabled = prefs.notifications.private_chats;
        sn.groups_enabled = prefs.notifications.groups;
        sn.channels_enabled = prefs.notifications.channels;
        sn.sound_enabled = prefs.notifications.sound_enabled;
        sn.flash_taskbar_icon = prefs.notifications.flash_taskbar;
        sn.include_muted_in_badge = prefs.notifications.include_muted_in_badge;
        sn.badge_count_type = if prefs.notifications.badge_count_messages {
            BadgeCountType::CountMessages
        } else {
            BadgeCountType::CountChats
        };
    }

    fn hydrate_notif_reactions(
        prefs: &AppPreferences,
        sr: &mut SettingsNotificationsReactionsSections,
    ) {
        sr.notify_reactions = prefs.notifications.reaction_notifications;
        sr.notify_stories = prefs.notifications.story_reaction_notifications;
        sr.source = if prefs.notifications.reaction_contacts_only {
            ReactionsNotifySource::Contacts
        } else {
            ReactionsNotifySource::All
        };
        sr.sound_enabled = prefs.notifications.reaction_sound_enabled;
    }

    fn hydrate_notif_type(prefs: &AppPreferences, st: &mut SettingsNotificationsTypeSections) {
        st.alerts_enabled = prefs.notifications.type_alerts_enabled;
        st.message_preview = prefs.notifications.type_preview_enabled;
        st.sound_enabled = prefs.notifications.type_sound_enabled;
    }

    fn hydrate_privacy(prefs: &AppPreferences, sp: &mut SettingsPrivacySecuritySections) {
        sp.passcode_enabled = prefs
            .local_passcode_hash
            .as_deref()
            .is_some_and(|h| !h.is_empty());
    }

    fn hydrate_local_passcode(prefs: &AppPreferences, lp: &mut SettingsLocalPasscodeSections) {
        lp.auto_lock = AutoLockTimeout::from_seconds(prefs.auto_lock_seconds);
        if let Some(ref hash) = prefs.local_passcode_hash {
            if !hash.is_empty() {
                lp.passcode_hash = Some(hash.clone());
                lp.has_passcode = true;
            } else {
                lp.passcode_hash = None;
                lp.has_passcode = false;
            }
        } else {
            lp.passcode_hash = None;
            lp.has_passcode = false;
        }
    }

    fn hydrate_chat(prefs: &AppPreferences, sc: &mut SettingsChatSections) {
        sc.font_size = prefs.appearance.font_size;
        sc.bubble_radius = prefs.appearance.bubble_radius;
        sc.send_on_enter = prefs.appearance.send_on_enter;
        sc.suggest_stickers_by_emoji = prefs.appearance.suggest_stickers_by_emoji;
        sc.loop_animated_stickers = prefs.appearance.loop_animated_stickers;
        sc.blur_wallpaper = prefs.appearance.blur_wallpaper;
    }

    fn hydrate_folders(prefs: &AppPreferences, sf: &mut SettingsFoldersSections) {
        sf.tags_enabled = prefs.folder_tags_enabled;
    }

    fn hydrate_advanced(prefs: &AppPreferences, sa: &mut SettingsAdvancedSections) {
        sa.ask_download_path = prefs.ask_download_path;
        sa.opengl_enabled = prefs.opengl_enabled;
        sa.native_window_frame = prefs.native_window_frame;
        sa.tray_icon_enabled = prefs.tray_icon_enabled;
        sa.auto_update_enabled = prefs.auto_update_enabled;
    }

    fn hydrate_local_storage(prefs: &AppPreferences, ls: &mut SettingsLocalStorageSections) {
        ls.max_cache_gb = prefs.max_cache_gb;
        ls.retention_weeks = prefs.retention_weeks;
    }

    fn hydrate_power_saving(prefs: &AppPreferences, ps: &mut SettingsPowerSaving) {
        ps.battery_saver_enabled = prefs.power_saving_enabled;
        ps.battery_threshold_percent = prefs.power_saving_threshold_pct;
        ps.animate_stickers_in_chat = prefs.animate_stickers_chat;
        ps.animate_stickers_in_panel = prefs.animate_stickers_panel;
        ps.animate_emoji_in_panel = prefs.animate_emoji_panel;
        ps.autoplay_gifs = prefs.autoplay_gifs;
        ps.autoplay_videos = prefs.autoplay_videos;
    }

    fn hydrate_calls(prefs: &AppPreferences, sc: &mut SettingsCallsSections) {
        if !prefs.audio_input_device.is_empty() {
            if !sc.available_inputs.contains(&prefs.audio_input_device) {
                sc.available_inputs.insert(0, prefs.audio_input_device.clone());
            }
            sc.input_device = prefs.audio_input_device.clone();
        }
        if !prefs.audio_output_device.is_empty() {
            if !sc.available_outputs.contains(&prefs.audio_output_device) {
                sc.available_outputs.insert(0, prefs.audio_output_device.clone());
            }
            sc.output_device = prefs.audio_output_device.clone();
        }
        if !prefs.camera_device.is_empty() {
            if !sc.available_cameras.contains(&prefs.camera_device) {
                sc.available_cameras.insert(0, prefs.camera_device.clone());
            }
            sc.camera_device = prefs.camera_device.clone();
        }
    }

    fn hydrate_passkeys(prefs: &AppPreferences, sp: &mut SettingsPasskeysSections) {
        sp.sync_from_names(&prefs.passkey_names);
    }

    fn hydrate_websites(prefs: &AppPreferences, sw: &mut SettingsWebsitesSections) {
        sw.sync_from_domains(&prefs.connected_website_domains);
    }

    fn hydrate_shortcuts(prefs: &AppPreferences, ss: &mut SettingsShortcutsSections) {
        ss.shortcuts_enabled = prefs.shortcuts_enabled;
        ss.search_query = prefs.shortcuts_filter.clone();
    }

    fn hydrate_business(prefs: &AppPreferences, sb: &mut SettingsBusinessSections) {
        sb.sync_configured_tools(&prefs.business_configured_tools);
    }

    fn stop_mic_test_if_active(&mut self, cx: &mut Context<Self>) {
        if let Some(ref sc_ent) = self.settings_calls {
            let was_active = sc_ent.update(cx, |sc, sc_cx| {
                if sc.mic_test_active {
                    sc.stop_mic_test_silent();
                    sc_cx.notify();
                    true
                } else {
                    false
                }
            });
            if was_active {
                cx.emit(SettingsRouterEvent::ToggleMicTest(false));
            }
        }
    }

    /// Lazily materializes `SettingsMainSections`, hydrates preferences, applies theme/locale, and attaches its observer.
    pub fn ensure_main(&mut self, cx: &mut Context<Self>) -> Entity<SettingsMainSections> {
        lazy_section!(self, cx, settings_main, |_, _| SettingsMainSections::new_live(), Self::hydrate_main, Self::observe_main)
    }

    /// Lazily materializes `SettingsInformationSections`, applies theme/locale, and attaches its observer.
    pub fn ensure_information(&mut self, cx: &mut Context<Self>) -> Entity<SettingsInformationSections> {
        lazy_section!(self, cx, settings_information, |_, _| SettingsInformationSections::new_live(), Self::observe_information)
    }

    /// Lazily materializes `SettingsNotificationsSections`, hydrates preferences, applies theme/locale, and attaches its observer.
    pub fn ensure_notifications(&mut self, cx: &mut Context<Self>) -> Entity<SettingsNotificationsSections> {
        lazy_section!(self, cx, settings_notifications, |_, _| SettingsNotificationsSections::new_live(), Self::hydrate_notifications, Self::observe_notifications)
    }

    /// Lazily materializes `SettingsNotificationsReactionsSections`, applies theme/locale, and attaches its observer.
    pub fn ensure_notif_reactions(&mut self, cx: &mut Context<Self>) -> Entity<SettingsNotificationsReactionsSections> {
        lazy_section!(self, cx, settings_notif_reactions, |_, _| SettingsNotificationsReactionsSections::new(), Self::hydrate_notif_reactions, Self::observe_notif_reactions)
    }

    /// Lazily materializes `SettingsNotificationsTypeSections`, applies theme/locale, and attaches its observer.
    pub fn ensure_notif_type(&mut self, cx: &mut Context<Self>) -> Entity<SettingsNotificationsTypeSections> {
        lazy_section!(self, cx, settings_notif_type, |_, _| SettingsNotificationsTypeSections::new("All Chats"), Self::hydrate_notif_type, Self::observe_notif_type)
    }

    /// Lazily materializes `SettingsPrivacySecuritySections`, hydrates preferences, applies theme/locale, and attaches its observer.
    pub fn ensure_privacy(&mut self, cx: &mut Context<Self>) -> Entity<SettingsPrivacySecuritySections> {
        lazy_section!(self, cx, settings_privacy, |_, _| SettingsPrivacySecuritySections::new_live(), Self::hydrate_privacy, Self::observe_privacy)
    }

    /// Lazily materializes `SettingsBlockedPeersSections`, applies theme/locale, and attaches its observer.
    pub fn ensure_blocked_peers(&mut self, cx: &mut Context<Self>) -> Entity<SettingsBlockedPeersSections> {
        lazy_section!(self, cx, settings_blocked_peers, |_, _| SettingsBlockedPeersSections::new_live(), Self::observe_blocked_peers)
    }

    /// Lazily materializes `SettingsActiveSessionsSections`, applies theme/locale, and attaches its observer.
    pub fn ensure_active_sessions(&mut self, cx: &mut Context<Self>) -> Entity<SettingsActiveSessionsSections> {
        lazy_section!(self, cx, settings_active_sessions, |_, _| SettingsActiveSessionsSections::new_live(), Self::observe_active_sessions)
    }

    /// Lazily materializes `SettingsLocalPasscodeSections`, hydrates preferences, applies theme/locale, and attaches its observer.
    pub fn ensure_local_passcode(&mut self, cx: &mut Context<Self>) -> Entity<SettingsLocalPasscodeSections> {
        lazy_section!(self, cx, settings_local_passcode, |_, _| SettingsLocalPasscodeSections::new_live(), Self::hydrate_local_passcode, Self::observe_local_passcode)
    }

    /// Lazily materializes `SettingsPasskeysSections`, applies theme/locale, and attaches its observer.
    pub fn ensure_passkeys(&mut self, cx: &mut Context<Self>) -> Entity<SettingsPasskeysSections> {
        lazy_section!(self, cx, settings_passkeys, |_, _| SettingsPasskeysSections::new_live(), Self::hydrate_passkeys, Self::observe_passkeys)
    }

    /// Lazily materializes `SettingsGlobalTtlSections`, applies theme/locale, and attaches its observer.
    pub fn ensure_global_ttl(&mut self, cx: &mut Context<Self>) -> Entity<SettingsGlobalTtlSections> {
        lazy_section!(self, cx, settings_global_ttl, |_, _| SettingsGlobalTtlSections::new_live(), Self::observe_global_ttl)
    }

    /// Lazily materializes `SettingsWebsitesSections`, applies theme/locale, and attaches its observer.
    pub fn ensure_websites(&mut self, cx: &mut Context<Self>) -> Entity<SettingsWebsitesSections> {
        lazy_section!(self, cx, settings_websites, |_, _| SettingsWebsitesSections::new_live(), Self::hydrate_websites, Self::observe_websites)
    }

    /// Lazily materializes `SettingsChatSections`, hydrates preferences, applies theme/locale, and attaches its observer.
    pub fn ensure_chat(&mut self, cx: &mut Context<Self>) -> Entity<SettingsChatSections> {
        lazy_section!(self, cx, settings_chat, |pal, lang| SettingsChatSections::new_live(pal, lang), Self::hydrate_chat, Self::observe_chat)
    }

    /// Lazily materializes `SettingsFoldersSections`, hydrates preferences, applies theme/locale, and attaches its observer.
    pub fn ensure_folders(&mut self, cx: &mut Context<Self>) -> Entity<SettingsFoldersSections> {
        lazy_section!(self, cx, settings_folders, |pal, lang| SettingsFoldersSections::new_live(pal, lang), Self::hydrate_folders, Self::observe_folders)
    }

    /// Lazily materializes `SettingsAdvancedSections`, hydrates preferences, applies theme/locale, and attaches its observer.
    pub fn ensure_advanced(&mut self, cx: &mut Context<Self>) -> Entity<SettingsAdvancedSections> {
        lazy_section!(self, cx, settings_advanced, |pal, lang| SettingsAdvancedSections::new_live(pal, lang), Self::hydrate_advanced, Self::observe_advanced)
    }

    /// Lazily materializes `SettingsLocalStorageSections`, hydrates preferences, applies theme/locale, and attaches its observer.
    pub fn ensure_local_storage(&mut self, cx: &mut Context<Self>) -> Entity<SettingsLocalStorageSections> {
        lazy_section!(self, cx, settings_local_storage, |pal, lang| SettingsLocalStorageSections::new_live(pal, lang), Self::hydrate_local_storage, Self::observe_local_storage)
    }

    /// Lazily materializes `SettingsPowerSaving`, hydrates preferences, applies theme/locale, and attaches its observer.
    pub fn ensure_power_saving(&mut self, cx: &mut Context<Self>) -> Entity<SettingsPowerSaving> {
        lazy_section!(self, cx, settings_power_saving, |pal, lang| SettingsPowerSaving::new_live(pal, lang), Self::hydrate_power_saving, Self::observe_power_saving)
    }

    /// Lazily materializes `SettingsCallsSections`, hydrates preferences, applies theme/locale, and attaches its observer.
    pub fn ensure_calls(&mut self, cx: &mut Context<Self>) -> Entity<SettingsCallsSections> {
        let (host_inputs, host_outputs) = gpui_telegram_media::enumerate_audio_devices();
        lazy_section!(
            self,
            cx,
            settings_calls,
            move |pal, lang| {
                let mut view = SettingsCallsSections::new_live(pal, lang);
                view.sync_devices(host_inputs.clone(), host_outputs.clone(), Vec::new());
                view
            },
            Self::hydrate_calls,
            Self::observe_calls
        )
    }

    /// Lazily materializes `SettingsCreditsSections`, applies theme/locale, and attaches its observer.
    pub fn ensure_credits(&mut self, cx: &mut Context<Self>) -> Entity<SettingsCreditsSections> {
        lazy_section!(self, cx, settings_credits, |pal, lang| SettingsCreditsSections::new_live(pal, lang), Self::observe_credits)
    }

    /// Lazily materializes `SettingsPremiumSections`, applies theme/locale, and attaches its observer.
    pub fn ensure_premium(&mut self, cx: &mut Context<Self>) -> Entity<SettingsPremiumSections> {
        let is_premium = self
            .settings_main
            .as_ref()
            .is_some_and(|sm| sm.read(cx).profile.is_premium);
        lazy_section!(self, cx, settings_premium, move |pal, lang| SettingsPremiumSections::new_live(is_premium, pal, lang), Self::observe_premium)
    }

    /// Lazily materializes `SettingsShortcutsSections`, applies theme/locale, and attaches its observer.
    pub fn ensure_shortcuts(&mut self, cx: &mut Context<Self>) -> Entity<SettingsShortcutsSections> {
        lazy_section!(self, cx, settings_shortcuts, |pal, lang| SettingsShortcutsSections::new_live(pal, lang), Self::hydrate_shortcuts, Self::observe_shortcuts)
    }

    /// Lazily materializes `SettingsBusinessSections`, applies theme/locale, and attaches its observer.
    pub fn ensure_business(&mut self, cx: &mut Context<Self>) -> Entity<SettingsBusinessSections> {
        let is_business = self
            .settings_premium
            .as_ref()
            .is_some_and(|sp| sp.read(cx).is_premium_active)
            || self
                .settings_main
                .as_ref()
                .is_some_and(|sm| sm.read(cx).profile.is_premium);
        lazy_section!(self, cx, settings_business, move |pal, lang| SettingsBusinessSections::new_live(is_business, pal, lang), Self::hydrate_business, Self::observe_business)
    }

    /// Lazily materializes `SettingsComputersSections`, applies theme/locale, and attaches its observer.
    pub fn ensure_computers(&mut self, cx: &mut Context<Self>) -> Entity<SettingsComputersSections> {
        lazy_section!(self, cx, settings_computers, |pal, lang| SettingsComputersSections::new_live(pal, lang), Self::observe_computers)
    }

    /// Lazily materializes `SettingsAiProvidersSections`, applies theme/locale, and attaches its observer.
    pub fn ensure_ai_providers(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Entity<SettingsAiProvidersSections> {
        lazy_section!(
            self,
            cx,
            settings_ai_providers,
            |pal, lang| SettingsAiProvidersSections::new_live(pal, lang),
            Self::observe_ai_providers
        )
    }

    /// Lazily materializes `SettingsSearch`, applies theme/locale, and attaches its observer.
    pub fn ensure_search(&mut self, cx: &mut Context<Self>) -> Entity<SettingsSearch> {
        lazy_section!(self, cx, settings_search, |pal, lang| SettingsSearch::new_live(pal, lang), Self::observe_search)
    }

    /// Navigates to `target`, clearing history when entering the settings modal or pushing onto the history stack.
    pub fn navigate_to(
        &mut self,
        target: SettingsSectionType,
        entering_settings_modal: bool,
        cx: &mut Context<Self>,
    ) {
        if self.settings_section == SettingsSectionType::Calls && target != SettingsSectionType::Calls {
            self.stop_mic_test_if_active(cx);
        }
        if entering_settings_modal {
            self.settings_stack.clear();
            self.settings_section = target;
            cx.emit(SettingsRouterEvent::Navigated(target));
            cx.notify();
            return;
        }
        if self.settings_section == target {
            return;
        }
        if self.settings_stack.last().copied() != Some(self.settings_section) {
            self.settings_stack.push(self.settings_section);
        }
        self.settings_section = target;
        cx.emit(SettingsRouterEvent::Navigated(target));
        cx.notify();
    }

    /// Unwinds one step in the settings navigation stack or closes settings when already at `Main`.
    pub fn navigate_back(&mut self, cx: &mut Context<Self>) -> bool {
        if self.settings_section == SettingsSectionType::Calls {
            self.stop_mic_test_if_active(cx);
        }
        if let Some(prev) = self.settings_stack.pop() {
            self.settings_section = prev;
            cx.emit(SettingsRouterEvent::Navigated(prev));
            cx.notify();
            true
        } else if !self.settings_section.is_root() {
            let parent = self
                .settings_section
                .parent_section()
                .unwrap_or(SettingsSectionType::Main);
            self.settings_section = parent;
            cx.emit(SettingsRouterEvent::Navigated(parent));
            cx.notify();
            true
        } else {
            self.close(cx);
            false
        }
    }

    /// Resets section and navigation stack and emits `SettingsRouterEvent::CloseRequested`.
    pub fn close(&mut self, cx: &mut Context<Self>) {
        self.stop_mic_test_if_active(cx);
        self.settings_section = SettingsSectionType::Main;
        self.settings_stack.clear();
        cx.emit(SettingsRouterEvent::CloseRequested);
        cx.notify();
    }

    /// Updates the selected interface language locally and emits `SettingsRouterEvent::SelectLanguage`.
    pub fn select_language_code(&mut self, lang_code: &'static str, cx: &mut Context<Self>) {
        self.preferences.localization.language_code = lang_code.to_string();
        let is_zh = lang_code.starts_with("zh");
        self.apply_theme_and_locale(self.palette, is_zh, cx);
        cx.emit(SettingsRouterEvent::SelectLanguage(lang_code));
    }

    /// Flips between Chinese and English UI locales locally and emits `SettingsRouterEvent::ToggleLanguage`.
    pub fn toggle_language(&mut self, cx: &mut Context<Self>) {
        let next_zh = !self.is_chinese_locale;
        self.preferences.localization.language_code = if next_zh {
            "zh-CN".to_string()
        } else {
            "en-US".to_string()
        };
        self.apply_theme_and_locale(self.palette, next_zh, cx);
        cx.emit(SettingsRouterEvent::ToggleLanguage);
    }

    /// Returns `true` when the active settings section currently focuses a text input field.
    pub fn is_input_active(&self, cx: &App) -> bool {
        match self.settings_section {
            SettingsSectionType::Information => self
                .settings_information
                .as_ref()
                .is_some_and(|si| si.read(cx).active_field != ProfileEditField::None),
            SettingsSectionType::PrivacySecurity => self
                .settings_privacy
                .as_ref()
                .is_some_and(|sp| sp.read(cx).two_factor_editing),
            SettingsSectionType::Folders => self
                .settings_folders
                .as_ref()
                .is_some_and(|sf| sf.read(cx).is_editing_title),
            SettingsSectionType::Computers => self
                .settings_computers
                .as_ref()
                .is_some_and(|sc| sc.read(cx).is_input_active()),
            SettingsSectionType::AiProviders => self
                .settings_ai_providers
                .as_ref()
                .is_some_and(|ap| ap.read(cx).is_input_active()),
            SettingsSectionType::Search => true,
            _ => false,
        }
    }

    /// Returns `(committed, preedit)` for the active settings text input if one is focused.
    pub fn active_input_text_and_preedit(
        &self,
        ime: &TextInputController,
        cx: &App,
    ) -> Option<(String, Option<String>)> {
        if !self.is_input_active(cx) {
            return None;
        }
        let committed = match self.settings_section {
            SettingsSectionType::Information => self
                .settings_information
                .as_ref()
                .and_then(|si| si.read(cx).active_field_text())
                .unwrap_or_default(),
            SettingsSectionType::PrivacySecurity => self
                .settings_privacy
                .as_ref()
                .map(|sp| sp.read(cx).two_factor_password_draft.clone())
                .unwrap_or_default(),
            SettingsSectionType::Folders => self
                .settings_folders
                .as_ref()
                .map(|sf| sf.read(cx).new_folder_title.clone())
                .unwrap_or_default(),
            SettingsSectionType::Computers => self
                .settings_computers
                .as_ref()
                .map(|sc| sc.read(cx).active_field_text().to_string())
                .unwrap_or_default(),
            SettingsSectionType::AiProviders => self
                .settings_ai_providers
                .as_ref()
                .map(|ap| ap.read(cx).active_field_text().to_string())
                .unwrap_or_default(),
            SettingsSectionType::Search => self
                .settings_search
                .as_ref()
                .map(|ss| ss.read(cx).query.clone())
                .unwrap_or_default(),
            _ => return None,
        };
        Some(ime.text_and_preedit(committed))
    }

    /// Mirrors IME preedit text onto the active settings section input controller (`Computers` or `AiProviders`).
    pub fn set_active_input_ime_preedit(
        &mut self,
        preedit: Option<&str>,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.is_input_active(cx) {
            return false;
        }
        match self.settings_section {
            SettingsSectionType::Computers => {
                let sc = self.ensure_computers(cx);
                sc.update(cx, |sc, sc_cx| sc.set_ime_preedit(preedit, sc_cx));
                cx.notify();
                true
            }
            SettingsSectionType::AiProviders => {
                let ap = self.ensure_ai_providers(cx);
                ap.update(cx, |ap, ap_cx| ap.set_ime_preedit(preedit, ap_cx));
                cx.notify();
                true
            }
            _ => false,
        }
    }

    /// Clears the active settings input field and returns `true` if a settings input was active.
    pub fn clear_active_input(&mut self, cx: &mut Context<Self>) -> bool {
        if !self.is_input_active(cx) {
            return false;
        }
        match self.settings_section {
            SettingsSectionType::Information => {
                let si = self.ensure_information(cx);
                si.update(cx, |si, si_cx| {
                    si.clear_field_text();
                    si_cx.notify();
                });
            }
            SettingsSectionType::PrivacySecurity => {
                let sp = self.ensure_privacy(cx);
                sp.update(cx, |sp, sp_cx| {
                    sp.two_factor_password_draft.clear();
                    sp_cx.notify();
                });
            }
            SettingsSectionType::Folders => {
                let sf = self.ensure_folders(cx);
                sf.update(cx, |sf, sf_cx| {
                    sf.new_folder_title.clear();
                    sf_cx.notify();
                });
            }
            SettingsSectionType::Computers => {
                let sc = self.ensure_computers(cx);
                sc.update(cx, |sc, sc_cx| {
                    sc.clear_active_input(sc_cx);
                });
            }
            SettingsSectionType::AiProviders => {
                let ap = self.ensure_ai_providers(cx);
                ap.update(cx, |ap, ap_cx| {
                    ap.clear_active_input(ap_cx);
                });
            }
            SettingsSectionType::Search => {
                let ss = self.ensure_search(cx);
                ss.update(cx, |ss, ss_cx| {
                    ss.set_query("");
                    ss_cx.notify();
                });
            }
            _ => return false,
        }
        cx.notify();
        true
    }

    /// Appends committed text into the active settings input field and returns `true` if handled.
    pub fn push_input_text(&mut self, text: &str, cx: &mut Context<Self>) -> bool {
        if !self.is_input_active(cx) {
            return false;
        }
        match self.settings_section {
            SettingsSectionType::Information => {
                let si = self.ensure_information(cx);
                si.update(cx, |si, si_cx| {
                    si.push_field_text(text);
                    si_cx.notify();
                });
            }
            SettingsSectionType::PrivacySecurity => {
                let sp = self.ensure_privacy(cx);
                sp.update(cx, |sp, sp_cx| {
                    sp.two_factor_password_draft.push_str(text);
                    sp_cx.notify();
                });
            }
            SettingsSectionType::Folders => {
                let sf = self.ensure_folders(cx);
                sf.update(cx, |sf, sf_cx| {
                    sf.new_folder_title.push_str(text);
                    sf_cx.notify();
                });
            }
            SettingsSectionType::Computers => {
                let sc = self.ensure_computers(cx);
                sc.update(cx, |sc, sc_cx| {
                    sc.push_input_text(text, sc_cx);
                });
            }
            SettingsSectionType::AiProviders => {
                let ap = self.ensure_ai_providers(cx);
                ap.update(cx, |ap, ap_cx| {
                    ap.push_input_text(text, ap_cx);
                });
            }
            SettingsSectionType::Search => {
                let ss = self.ensure_search(cx);
                ss.update(cx, |ss, ss_cx| {
                    let mut q = ss.query.clone();
                    q.push_str(text);
                    ss.set_query(q);
                    ss_cx.notify();
                });
            }
            _ => return false,
        }
        cx.notify();
        true
    }

    /// Handles non-character key navigation/submission for an active settings text field.
    pub fn handle_non_char_key(
        &mut self,
        key: &str,
        _is_shift: bool,
        cx: &mut Context<Self>,
    ) -> Option<bool> {
        if !self.is_input_active(cx) {
            return None;
        }
        let handled = match self.settings_section {
            SettingsSectionType::Information => {
                if !matches!(key, "tab" | "backspace" | "enter") {
                    return Some(false);
                }
                let si = self.ensure_information(cx);
                si.update(cx, |si, si_cx| {
                    match key {
                        "tab" => si.cycle_field(),
                        "backspace" => si.backspace_field_text(),
                        "enter" => {
                            si.pending_save_profile = true;
                            si.save_pending = false;
                        }
                        _ => {}
                    }
                    si_cx.notify();
                });
                true
            }
            SettingsSectionType::PrivacySecurity => {
                if !matches!(key, "backspace" | "enter") {
                    return Some(false);
                }
                let sp = self.ensure_privacy(cx);
                sp.update(cx, |sp, sp_cx| {
                    if key == "backspace" {
                        sp.two_factor_password_draft.pop();
                        sp_cx.notify();
                    } else if !sp.two_factor_password_draft.trim().is_empty() {
                        let pw = sp.two_factor_password_draft.trim().to_string();
                        sp.pending_2fa_action = Some(Some(pw));
                        sp.two_factor_editing = false;
                        sp.two_factor_password_draft.clear();
                        sp_cx.notify();
                    }
                });
                true
            }
            SettingsSectionType::Folders => {
                if !matches!(key, "backspace" | "enter") {
                    return Some(false);
                }
                let sf = self.ensure_folders(cx);
                sf.update(cx, |sf, sf_cx| {
                    if key == "backspace" {
                        sf.new_folder_title.pop();
                        sf_cx.notify();
                    } else if !sf.new_folder_title.trim().is_empty() {
                        let next_id = sf.folders.iter().map(|f| f.id).max().unwrap_or(0) + 1;
                        let title = sf.new_folder_title.trim().to_string();
                        sf.folders.push(ChatFolderItem {
                            id: next_id,
                            title: title.clone(),
                            icon_name: "folder".into(),
                            included_peers_count: 0,
                            excluded_peers_count: 0,
                            unread_count: 0,
                            is_removed: false,
                        });
                        sf.selected_folder_id = Some(next_id);
                        sf.new_folder_title.clear();
                        sf.is_editing_title = false;
                        sf.pending_create_folder = true;
                        sf.pending_created_folder = Some((next_id, title));
                        sf_cx.notify();
                    }
                });
                true
            }
            SettingsSectionType::Computers => {
                if !matches!(key, "backspace" | "delete" | "tab" | "enter") {
                    return Some(false);
                }
                let sc = self.ensure_computers(cx);
                let handled = sc.update(cx, |sc, sc_cx| {
                    sc.handle_non_char_key(key, _is_shift, false, sc_cx)
                });
                handled
            }
            SettingsSectionType::AiProviders => {
                if !matches!(key, "backspace" | "delete" | "tab" | "enter") {
                    return Some(false);
                }
                let ap = self.ensure_ai_providers(cx);
                let handled = ap.update(cx, |ap, ap_cx| {
                    ap.handle_non_char_key(key, _is_shift, false, ap_cx)
                });
                handled
            }
            SettingsSectionType::Search => {
                if !matches!(key, "backspace" | "up" | "down" | "enter") {
                    return Some(false);
                }
                let ss = self.ensure_search(cx);
                ss.update(cx, |ss, ss_cx| {
                    ss.handle_key_input(key);
                    ss_cx.notify();
                });
                true
            }
            _ => return None,
        };
        cx.notify();
        Some(handled)
    }
}
