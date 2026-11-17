//! Observer bindings for lazily materialized settings sections inside `SettingsRouter`.
//! Translates section state mutations into `SettingsRouterEvent` notifications and keeps
//! cross-section state synchronized on demand via `ensure_*`.

use super::settings_router::{SettingsRouter, SettingsRouterEvent};
use crate::i18n::tr_str;
use crate::views::boxes::modal_types::ModalDialogKind;
use crate::views::settings::sections::{
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
use gpui::{Context, Entity};

impl SettingsRouter {
    /// Subscribes to main settings actions (navigation, clipboard copy, interface scale).
    pub(super) fn observe_main(ent: &Entity<SettingsMainSections>, cx: &mut Context<Self>) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             main: Entity<SettingsMainSections>,
             cx: &mut Context<SettingsRouter>| {
                let (target, copied, scale) = main.update(cx, |m, _| {
                    (
                        m.active_section.take(),
                        m.pending_copy_text.take(),
                        m.pending_scale_change.take(),
                    )
                });
                if let Some(txt) = copied {
                    cx.emit(SettingsRouterEvent::CopyClipboard(txt));
                }
                if let Some(pct) = scale {
                    router.preferences.interface_scale_pct = pct;
                    cx.emit(SettingsRouterEvent::UpdateInterfaceScale(pct));
                }
                if let Some(target) = target {
                    router.navigate_to(target, false, cx);
                }
            },
        )
        .detach();
    }

    /// Subscribes to profile editor saves and logout triggers.
    pub(super) fn observe_information(
        ent: &Entity<SettingsInformationSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |_, si: Entity<SettingsInformationSections>, cx: &mut Context<SettingsRouter>| {
                let (do_save, do_logout, first_name, last_name, uname, phone, bio) =
                    si.update(cx, |s, _| {
                        let save = std::mem::take(&mut s.pending_save_profile);
                        let logout = std::mem::take(&mut s.pending_logout);
                        (
                            save,
                            logout,
                            s.first_name.clone(),
                            s.last_name.clone(),
                            s.username.clone(),
                            s.phone.clone(),
                            s.bio.clone(),
                        )
                    });
                if do_logout {
                    cx.emit(SettingsRouterEvent::LogoutRequested);
                    return;
                }
                if do_save {
                    let username = (!uname.trim().is_empty()).then_some(uname);
                    cx.emit(SettingsRouterEvent::SaveProfile {
                        first_name,
                        last_name,
                        username,
                        phone,
                        bio,
                    });
                }
            },
        )
        .detach();
    }

    /// Subscribes to notification preference toggles and emits synced preference events.
    pub(super) fn observe_notifications(
        ent: &Entity<SettingsNotificationsSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sn: Entity<SettingsNotificationsSections>,
             cx: &mut Context<SettingsRouter>| {
                let (nav_sec, sync_opt) = sn.update(cx, |s, _| {
                    let nav = s.pending_nav_section.take();
                    let sync = if !s.pending_sync {
                        None
                    } else {
                        s.pending_sync = false;
                        Some((
                            s.private_chats_enabled,
                            s.groups_enabled,
                            s.channels_enabled,
                            s.sound_enabled,
                            s.flash_taskbar_icon,
                            s.include_muted_in_badge,
                            s.badge_count_type == BadgeCountType::CountMessages,
                        ))
                    };
                    (nav, sync)
                });
                if let Some(sec) = nav_sec {
                    router.navigate_to(sec, false, cx);
                }
                if let Some((
                    private_chats,
                    groups,
                    channels,
                    sound_enabled,
                    flash_taskbar,
                    include_muted_in_badge,
                    badge_count_messages,
                )) = sync_opt
                {
                    let notif = &mut router.preferences.notifications;
                    notif.private_chats = private_chats;
                    notif.groups = groups;
                    notif.channels = channels;
                    notif.sound_enabled = sound_enabled;
                    notif.flash_taskbar = flash_taskbar;
                    notif.include_muted_in_badge = include_muted_in_badge;
                    notif.badge_count_messages = badge_count_messages;
                    cx.emit(SettingsRouterEvent::UpdateNotificationPrefs {
                        private_chats,
                        groups,
                        channels,
                        sound_enabled,
                        flash_taskbar,
                        include_muted_in_badge,
                        badge_count_messages,
                    });
                }
            },
        )
        .detach();
    }

    /// Subscribes to reaction notification preferences.
    pub(super) fn observe_notif_reactions(
        ent: &Entity<SettingsNotificationsReactionsSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sr: Entity<SettingsNotificationsReactionsSections>,
             cx: &mut Context<SettingsRouter>| {
                let sync_opt = sr.update(cx, |s, _| {
                    if !s.pending_sync {
                        return None;
                    }
                    s.pending_sync = false;
                    Some((
                        s.notify_reactions,
                        s.notify_stories,
                        s.source == ReactionsNotifySource::Contacts,
                        s.sound_enabled,
                    ))
                });
                if let Some((notify_reactions, notify_stories, contacts_only, sound_enabled)) =
                    sync_opt
                {
                    let notif = &mut router.preferences.notifications;
                    notif.reaction_notifications = notify_reactions;
                    notif.story_reaction_notifications = notify_stories;
                    notif.reaction_contacts_only = contacts_only;
                    notif.reaction_sound_enabled = sound_enabled;
                    cx.emit(SettingsRouterEvent::UpdateNotificationReactionsPrefs {
                        notify_reactions,
                        notify_stories,
                        contacts_only,
                        sound_enabled,
                    });
                    cx.notify();
                }
            },
        )
        .detach();
    }

    /// Subscribes to chat-type notification overrides.
    pub(super) fn observe_notif_type(
        ent: &Entity<SettingsNotificationsTypeSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             st: Entity<SettingsNotificationsTypeSections>,
             cx: &mut Context<SettingsRouter>| {
                let sync_opt = st.update(cx, |s, _| {
                    if !s.pending_sync {
                        return None;
                    }
                    s.pending_sync = false;
                    Some((s.alerts_enabled, s.message_preview, s.sound_enabled))
                });
                if let Some((alerts_enabled, preview_enabled, sound_enabled)) = sync_opt {
                    let notif = &mut router.preferences.notifications;
                    notif.type_alerts_enabled = alerts_enabled;
                    notif.type_preview_enabled = preview_enabled;
                    notif.type_sound_enabled = sound_enabled;
                    cx.emit(SettingsRouterEvent::UpdateNotificationTypePrefs {
                        alerts_enabled,
                        preview_enabled,
                        sound_enabled,
                    });
                    cx.notify();
                }
            },
        )
        .detach();
    }

    /// Subscribes to privacy/security navigation, modal opening, 2FA mutations, and settings pushes.
    pub(super) fn observe_privacy(
        ent: &Entity<SettingsPrivacySecuritySections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sp: Entity<SettingsPrivacySecuritySections>,
             cx: &mut Context<SettingsRouter>| {
                let (nav_sec, edit_cat, twofa_act, save_cfg) = sp.update(cx, |s, _| {
                    (
                        s.pending_nav_section.take(),
                        s.pending_edit_privacy_category.take(),
                        s.pending_2fa_action.take(),
                        std::mem::take(&mut s.pending_save_settings),
                    )
                });
                if let Some(sec) = nav_sec {
                    router.navigate_to(sec, false, cx);
                }
                if let Some(cat) = edit_cat {
                    cx.emit(SettingsRouterEvent::OpenModal {
                        title: cat.clone(),
                        kind: ModalDialogKind::EditPrivacy { category: cat },
                        width: 420.0,
                        height: 500.0,
                    });
                }
                if let Some(pw_opt) = twofa_act {
                    cx.emit(SettingsRouterEvent::TwoFactorMutation(pw_opt));
                }
                if save_cfg {
                    cx.emit(SettingsRouterEvent::PushUserSettings);
                }
            },
        )
        .detach();
    }

    /// Subscribes to blocked peer unblocking and synchronizes `SettingsPrivacySecuritySections` count.
    pub(super) fn observe_blocked_peers(
        ent: &Entity<SettingsBlockedPeersSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sbp: Entity<SettingsBlockedPeersSections>,
             cx: &mut Context<SettingsRouter>| {
                let (unblock_uid, remaining) = sbp.update(cx, |s, _| {
                    (s.pending_unblock_user_id.take(), s.blocked_list.len())
                });
                if let Some(user_id) = unblock_uid {
                    let priv_ent = router.ensure_privacy(cx);
                    priv_ent.update(cx, |priv_s, p_cx| {
                        priv_s.set_blocked_count(remaining);
                        p_cx.notify();
                    });
                    cx.emit(SettingsRouterEvent::UnblockPeer { user_id, remaining });
                }
            },
        )
        .detach();
    }

    /// Subscribes to active session termination actions.
    pub(super) fn observe_active_sessions(
        ent: &Entity<SettingsActiveSessionsSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |_, sas: Entity<SettingsActiveSessionsSections>, cx: &mut Context<SettingsRouter>| {
                let (term_one, term_all) = sas.update(cx, |s, _| {
                    (
                        s.pending_terminate_session_id.take(),
                        std::mem::take(&mut s.pending_terminate_all_other),
                    )
                });
                if let Some(sid) = term_one {
                    cx.emit(SettingsRouterEvent::RevokeSession(sid));
                }
                if term_all {
                    cx.emit(SettingsRouterEvent::RevokeOtherSessions);
                }
            },
        )
        .detach();
    }

    /// Subscribes to local passcode changes and keeps `SettingsPrivacySecuritySections` synchronized.
    pub(super) fn observe_local_passcode(
        ent: &Entity<SettingsLocalPasscodeSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             slp: Entity<SettingsLocalPasscodeSections>,
             cx: &mut Context<SettingsRouter>| {
                let (open_box, disable_pass, lock_now, prefs_sync, auto_lock_now) =
                    slp.update(cx, |s, _| {
                        let prefs_sync = if s.pending_prefs_sync {
                            s.pending_prefs_sync = false;
                            Some((s.passcode_hash.clone(), s.auto_lock.as_seconds()))
                        } else {
                            None
                        };
                        (
                            std::mem::take(&mut s.pending_open_passcode_box),
                            std::mem::take(&mut s.pending_disable_passcode),
                            std::mem::take(&mut s.pending_lock_now),
                            prefs_sync,
                            s.auto_lock.as_seconds(),
                        )
                    });
                if open_box {
                    cx.emit(SettingsRouterEvent::OpenModal {
                        title: tr_str(router.lang_id, "本地密码锁", "Local Passcode").to_string(),
                        kind: ModalDialogKind::PasscodeSetup,
                        width: 380.0,
                        height: 360.0,
                    });
                }
                if disable_pass {
                    let priv_ent = router.ensure_privacy(cx);
                    priv_ent.update(cx, |priv_s, p_cx| {
                        priv_s.set_passcode_enabled(false);
                        p_cx.notify();
                    });
                    router.preferences.local_passcode_hash = None;
                    cx.emit(SettingsRouterEvent::UpdateLocalPasscode {
                        disable: true,
                        hash: None,
                        auto_lock_seconds: auto_lock_now as u32,
                    });
                    cx.notify();
                } else if let Some((hash, auto_lock_secs)) = prefs_sync {
                    let has_passcode = hash.as_deref().is_some_and(|h| !h.is_empty());
                    let priv_ent = router.ensure_privacy(cx);
                    priv_ent.update(cx, |priv_s, p_cx| {
                        priv_s.set_passcode_enabled(has_passcode);
                        p_cx.notify();
                    });
                    router.preferences.local_passcode_hash = hash.clone();
                    router.preferences.auto_lock_seconds = auto_lock_secs;
                    cx.emit(SettingsRouterEvent::UpdateLocalPasscode {
                        disable: false,
                        hash,
                        auto_lock_seconds: auto_lock_secs as u32,
                    });
                }
                if lock_now {
                    cx.emit(SettingsRouterEvent::LockAppNow);
                }
            },
        )
        .detach();
    }

    /// Subscribes to global auto-delete TTL updates and mirrors them into privacy settings.
    pub(super) fn observe_global_ttl(
        ent: &Entity<SettingsGlobalTtlSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sgt: Entity<SettingsGlobalTtlSections>,
             cx: &mut Context<SettingsRouter>| {
                let (saved_ttl, apply_existing_ttl) = sgt.update(cx, |s, _| {
                    (
                        s.pending_save_global_ttl.take(),
                        s.pending_apply_to_existing.take(),
                    )
                });
                if let Some(ttl) = saved_ttl {
                    let priv_ent = router.ensure_privacy(cx);
                    priv_ent.update(cx, |priv_s, p_cx| {
                        priv_s.global_ttl_seconds = ttl;
                        p_cx.notify();
                    });
                    cx.emit(SettingsRouterEvent::UpdateGlobalTtl(ttl));
                    cx.notify();
                }
                if let Some(ttl) = apply_existing_ttl {
                    cx.emit(SettingsRouterEvent::ApplyGlobalTtlToExistingChats(ttl));
                    cx.notify();
                }
            },
        )
        .detach();
    }

    /// Subscribes to theme and chat appearance changes.
    pub(super) fn observe_chat(ent: &Entity<SettingsChatSections>, cx: &mut Context<Self>) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sc: Entity<SettingsChatSections>,
             cx: &mut Context<SettingsRouter>| {
                let (theme_chg, prefs_chg) =
                    sc.update(cx, |s, _| (s.take_theme_change(), s.take_chat_prefs_change()));
                if let Some((mode, accent, wallpaper)) = theme_chg {
                    cx.emit(SettingsRouterEvent::ApplyTheme {
                        mode,
                        accent,
                        wallpaper,
                    });
                }
                if let Some((
                    font_size,
                    bubble_radius,
                    send_on_enter,
                    suggest_stickers,
                    loop_animated_stickers,
                    blur_wallpaper,
                )) = prefs_chg
                {
                    let app = &mut router.preferences.appearance;
                    app.font_size = font_size;
                    app.bubble_radius = bubble_radius;
                    app.send_on_enter = send_on_enter;
                    app.suggest_stickers_by_emoji = suggest_stickers;
                    app.loop_animated_stickers = loop_animated_stickers;
                    app.blur_wallpaper = blur_wallpaper;
                    cx.emit(SettingsRouterEvent::UpdateChatPrefs {
                        font_size,
                        bubble_radius,
                        send_on_enter,
                        suggest_stickers,
                        loop_animated_stickers,
                        blur_wallpaper,
                    });
                    cx.notify();
                }
            },
        )
        .detach();
    }

    /// Subscribes to chat folder creation, deletion, and folder-tags preference toggles.
    pub(super) fn observe_folders(
        ent: &Entity<SettingsFoldersSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sf: Entity<SettingsFoldersSections>,
             cx: &mut Context<SettingsRouter>| {
                let (created, deleted, prefs_sync) = sf.update(cx, |s, _| {
                    (
                        s.pending_created_folder.take(),
                        s.pending_deleted_folder_id.take(),
                        if s.pending_prefs_sync {
                            s.pending_prefs_sync = false;
                            Some(s.tags_enabled)
                        } else {
                            None
                        },
                    )
                });
                if let Some(tags_enabled) = prefs_sync {
                    router.preferences.folder_tags_enabled = tags_enabled;
                    cx.emit(SettingsRouterEvent::UpdateFolderTags(tags_enabled));
                }
                if let Some((fid, title)) = created {
                    cx.emit(SettingsRouterEvent::CreateFolder {
                        id: fid as u32,
                        title,
                    });
                }
                if let Some(fid) = deleted {
                    cx.emit(SettingsRouterEvent::DeleteFolder(fid as u32));
                }
            },
        )
        .detach();
    }

    /// Subscribes to advanced settings preferences and subpage/modal triggers.
    pub(super) fn observe_advanced(
        ent: &Entity<SettingsAdvancedSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sa: Entity<SettingsAdvancedSections>,
             cx: &mut Context<SettingsRouter>| {
                let (open_ls, open_ps, open_conn, open_exp, check_upd, apply_upd, prefs_sync) =
                    sa.update(cx, |s, _| {
                        let prefs_sync = if s.pending_prefs_sync {
                            s.pending_prefs_sync = false;
                            Some((
                                s.ask_download_path,
                                s.opengl_enabled,
                                s.native_window_frame,
                                s.tray_icon_enabled,
                                s.auto_update_enabled,
                            ))
                        } else {
                            None
                        };
                        (
                            std::mem::take(&mut s.pending_open_local_storage),
                            std::mem::take(&mut s.pending_open_power_saving),
                            std::mem::take(&mut s.pending_open_connection_box),
                            std::mem::take(&mut s.pending_open_export_box),
                            std::mem::take(&mut s.pending_check_updates),
                            std::mem::take(&mut s.pending_apply_update),
                            prefs_sync,
                        )
                    });
                if let Some((
                    ask_download_path,
                    opengl_enabled,
                    native_window_frame,
                    tray_icon_enabled,
                    auto_update_enabled,
                )) = prefs_sync
                {
                    router.preferences.ask_download_path = ask_download_path;
                    router.preferences.opengl_enabled = opengl_enabled;
                    router.preferences.native_window_frame = native_window_frame;
                    router.preferences.tray_icon_enabled = tray_icon_enabled;
                    router.preferences.auto_update_enabled = auto_update_enabled;
                    cx.emit(SettingsRouterEvent::UpdateAdvancedPrefs {
                        ask_download_path,
                        opengl_enabled,
                        native_window_frame,
                        tray_icon_enabled,
                        auto_update_enabled,
                    });
                }
                if check_upd {
                    cx.emit(SettingsRouterEvent::CheckForUpdates);
                }
                if apply_upd {
                    cx.emit(SettingsRouterEvent::TriggerOneClickUpdate);
                }
                if open_ls {
                    router.navigate_to(SettingsSectionType::LocalStorage, false, cx);
                }
                if open_ps {
                    router.navigate_to(SettingsSectionType::PowerSaving, false, cx);
                }
                if open_conn {
                    cx.emit(SettingsRouterEvent::OpenModal {
                        title: tr_str(router.lang_id, "连接与代理设置", "Connection & Proxy")
                            .to_string(),
                        kind: ModalDialogKind::ConnectionProxy,
                        width: 420.0,
                        height: 440.0,
                    });
                }
                if open_exp {
                    cx.emit(SettingsRouterEvent::OpenExportDataModal);
                }
            },
        )
        .detach();
    }

    /// Subscribes to local storage limits and cache cleanup requests.
    pub(super) fn observe_local_storage(
        ent: &Entity<SettingsLocalStorageSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sls: Entity<SettingsLocalStorageSections>,
             cx: &mut Context<SettingsRouter>| {
                let (do_clear, prefs_sync) = sls.update(cx, |s, _| {
                    (
                        std::mem::take(&mut s.pending_clear_cache),
                        if s.pending_prefs_sync {
                            s.pending_prefs_sync = false;
                            Some((s.max_cache_gb, s.retention_weeks))
                        } else {
                            None
                        },
                    )
                });
                if let Some((max_gb, retention_weeks)) = prefs_sync {
                    router.preferences.max_cache_gb = max_gb;
                    router.preferences.retention_weeks = retention_weeks;
                    cx.emit(SettingsRouterEvent::UpdateLocalStoragePrefs {
                        max_cache_gb: max_gb as u32,
                        retention_weeks,
                    });
                }
                if do_clear {
                    cx.emit(SettingsRouterEvent::ClearLocalCache);
                }
            },
        )
        .detach();
    }

    /// Subscribes to power saving toggles and animation limits.
    pub(super) fn observe_power_saving(
        ent: &Entity<SettingsPowerSaving>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sps: Entity<SettingsPowerSaving>,
             cx: &mut Context<SettingsRouter>| {
                let sync_val = sps.update(cx, |s, _| {
                    if !s.pending_sync {
                        return None;
                    }
                    s.pending_sync = false;
                    Some((
                        s.battery_saver_enabled,
                        s.battery_threshold_percent,
                        s.animate_stickers_in_chat,
                        s.animate_stickers_in_panel,
                        s.animate_emoji_in_panel,
                        s.autoplay_gifs,
                        s.autoplay_videos,
                    ))
                });
                if let Some((
                    enabled,
                    threshold_pct,
                    animate_stickers_chat,
                    animate_stickers_panel,
                    animate_emoji_panel,
                    autoplay_gifs,
                    autoplay_videos,
                )) = sync_val
                {
                    router.preferences.power_saving_enabled = enabled;
                    router.preferences.power_saving_threshold_pct = threshold_pct;
                    router.preferences.animate_stickers_chat = animate_stickers_chat;
                    router.preferences.animate_stickers_panel = animate_stickers_panel;
                    router.preferences.animate_emoji_panel = animate_emoji_panel;
                    router.preferences.autoplay_gifs = autoplay_gifs;
                    router.preferences.autoplay_videos = autoplay_videos;
                    cx.emit(SettingsRouterEvent::UpdatePowerSavingPrefs {
                        enabled,
                        threshold_pct,
                        animate_stickers_chat,
                        animate_stickers_panel,
                        animate_emoji_panel,
                        autoplay_gifs,
                        autoplay_videos,
                    });
                }
            },
        )
        .detach();
    }

    /// Subscribes to call device selection, mic test toggles, and call settings saves.
    pub(super) fn observe_calls(ent: &Entity<SettingsCallsSections>, cx: &mut Context<Self>) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sc: Entity<SettingsCallsSections>,
             cx: &mut Context<SettingsRouter>| {
                let (do_save, mic_test_chg, prefs_sync) = sc.update(cx, |s, _| {
                    (
                        std::mem::take(&mut s.pending_save_settings),
                        s.pending_mic_test_change.take(),
                        if s.pending_prefs_sync {
                            s.pending_prefs_sync = false;
                            Some((
                                s.input_device.clone(),
                                s.output_device.clone(),
                                s.camera_device.clone(),
                            ))
                        } else {
                            None
                        },
                    )
                });
                if let Some((input_device, output_device, camera_device)) = prefs_sync {
                    router.preferences.audio_input_device = input_device.clone();
                    router.preferences.audio_output_device = output_device.clone();
                    router.preferences.camera_device = camera_device.clone();
                    cx.emit(SettingsRouterEvent::UpdateCallPrefs {
                        input_device,
                        output_device,
                        camera_device,
                    });
                }
                if let Some(active) = mic_test_chg {
                    cx.emit(SettingsRouterEvent::ToggleMicTest(active));
                }
                if do_save {
                    cx.emit(SettingsRouterEvent::PushUserSettings);
                }
            },
        )
        .detach();
    }

    /// Subscribes to Stars top-up and withdrawal requests from credits settings.
    pub(super) fn observe_credits(
        ent: &Entity<SettingsCreditsSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |_, sc: Entity<SettingsCreditsSections>, cx: &mut Context<SettingsRouter>| {
                let (topup, withdraw) = sc.update(cx, |s, _| {
                    (
                        s.pending_topup_stars.take(),
                        std::mem::take(&mut s.pending_withdraw),
                    )
                });
                if let Some(amount) = topup {
                    cx.emit(SettingsRouterEvent::TopupStars(amount as u32));
                }
                if withdraw {
                    cx.emit(SettingsRouterEvent::WithdrawStars);
                }
            },
        )
        .detach();
    }

    /// Subscribes to passkey registration/removal and persists to `AppPreferences`.
    pub(super) fn observe_passkeys(
        ent: &Entity<SettingsPasskeysSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sp: Entity<SettingsPasskeysSections>,
             cx: &mut Context<SettingsRouter>| {
                let names_opt = sp.update(cx, |s, _| {
                    if !s.pending_sync {
                        return None;
                    }
                    s.pending_sync = false;
                    Some(s.passkey_names())
                });
                if let Some(names) = names_opt {
                    router.preferences.passkey_names = names.clone();
                    cx.emit(SettingsRouterEvent::UpdatePasskeysPrefs(names));
                    cx.notify();
                }
            },
        )
        .detach();
    }

    /// Subscribes to connected website authorization/disconnection and persists to `AppPreferences`.
    pub(super) fn observe_websites(
        ent: &Entity<SettingsWebsitesSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sw: Entity<SettingsWebsitesSections>,
             cx: &mut Context<SettingsRouter>| {
                let domains_opt = sw.update(cx, |s, _| {
                    if !s.pending_sync {
                        return None;
                    }
                    s.pending_sync = false;
                    Some(s.connected_domains())
                });
                if let Some(domains) = domains_opt {
                    router.preferences.connected_website_domains = domains.clone();
                    cx.emit(SettingsRouterEvent::UpdateWebsitesPrefs(domains));
                    cx.notify();
                }
            },
        )
        .detach();
    }

    /// Subscribes to keyboard shortcuts toggle/filter changes and persists to `AppPreferences`.
    pub(super) fn observe_shortcuts(
        ent: &Entity<SettingsShortcutsSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             ss: Entity<SettingsShortcutsSections>,
             cx: &mut Context<SettingsRouter>| {
                let sync_opt = ss.update(cx, |s, _| {
                    if !s.pending_sync {
                        return None;
                    }
                    s.pending_sync = false;
                    Some((s.shortcuts_enabled, s.search_query.clone()))
                });
                if let Some((enabled, filter)) = sync_opt {
                    router.preferences.shortcuts_enabled = enabled;
                    router.preferences.shortcuts_filter = filter.clone();
                    cx.emit(SettingsRouterEvent::UpdateShortcutsPrefs { enabled, filter });
                    cx.notify();
                }
            },
        )
        .detach();
    }

    /// Subscribes to Business tool toggles and persists to `AppPreferences`.
    pub(super) fn observe_business(
        ent: &Entity<SettingsBusinessSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sb: Entity<SettingsBusinessSections>,
             cx: &mut Context<SettingsRouter>| {
                let tools_opt = sb.update(cx, |s, _| {
                    if !s.pending_sync {
                        return None;
                    }
                    s.pending_sync = false;
                    Some(s.configured_tool_titles())
                });
                if let Some(tools) = tools_opt {
                    router.preferences.business_configured_tools = tools.clone();
                    cx.emit(SettingsRouterEvent::UpdateBusinessPrefs(tools));
                    cx.notify();
                }
            },
        )
        .detach();
    }

    /// Subscribes to Premium subscriptions and activates Premium badges across main and business sections.
    pub(super) fn observe_premium(
        ent: &Entity<SettingsPremiumSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sp: Entity<SettingsPremiumSections>,
             cx: &mut Context<SettingsRouter>| {
                if let Some(months) = sp.update(cx, |s, _| s.pending_subscribe_months.take()) {
                    let main_ent = router.ensure_main(cx);
                    main_ent.update(cx, |sm, sm_cx| {
                        sm.profile.is_premium = true;
                        sm_cx.notify();
                    });
                    let biz_ent = router.ensure_business(cx);
                    biz_ent.update(cx, |sb, sb_cx| {
                        sb.show_business();
                        sb_cx.notify();
                    });
                    cx.emit(SettingsRouterEvent::ActivatePremium(months));
                    cx.notify();
                }
            },
        )
        .detach();
    }

    /// Subscribes to computers & cloud server actions and forwards events to `AppShell`.
    pub(super) fn observe_computers(
        ent: &Entity<SettingsComputersSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             sc: Entity<SettingsComputersSections>,
             cx: &mut Context<SettingsRouter>| {
                if let Some(ev) = sc.update(cx, |s, _| s.take_event()) {
                    cx.emit(SettingsRouterEvent::Computers(ev));
                    cx.notify();
                }
            },
        )
        .detach();
    }

    /// Subscribes to AI subscription & Magpie provider actions and forwards events to `AppShell`.
    pub(super) fn observe_ai_providers(
        ent: &Entity<SettingsAiProvidersSections>,
        cx: &mut Context<Self>,
    ) {
        cx.observe(
            ent,
            |_router: &mut SettingsRouter,
             ap: Entity<SettingsAiProvidersSections>,
             cx: &mut Context<SettingsRouter>| {
                if let Some(ev) = ap.update(cx, |s, _| s.take_event()) {
                    cx.emit(SettingsRouterEvent::AiProviders(ev));
                    cx.notify();
                }
            },
        )
        .detach();
    }

    /// Subscribes to settings search result selections and navigates to the chosen section.
    pub(super) fn observe_search(ent: &Entity<SettingsSearch>, cx: &mut Context<Self>) {
        cx.observe(
            ent,
            |router: &mut SettingsRouter,
             ss: Entity<SettingsSearch>,
             cx: &mut Context<SettingsRouter>| {
                if let Some(target) = ss.update(cx, |s, _| s.take_selected_section()) {
                    router.navigate_to(target, false, cx);
                }
            },
        )
        .detach();
    }
}
