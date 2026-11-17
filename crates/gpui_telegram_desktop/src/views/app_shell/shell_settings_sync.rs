//! Settings truth synchronization, preference hydration, privacy rules, and local storage cleanup.

use std::collections::HashMap;
use std::path::Path;

use gpui::{AsyncApp, Context, WeakEntity};
use gpui_telegram_client::net::{WalletBalanceDto, WalletTransactionDto};
use gpui_telegram_proto::contracts::auth::UserPrivacySettingsDto;
use gpui_telegram_proto::contracts::chats::ChatFolderDto;

use crate::i18n::tr_str;
use crate::views::app_shell::shell_truth::SessionRequestFence;
use crate::views::boxes::edit_privacy_box::PrivacyRuleOption;
use crate::views::nav_rail::NavFolderItem;
use crate::views::settings::sections::settings_active_sessions::SessionRowData;
use crate::views::settings::sections::settings_blocked_peers::BlockedPeerItem;
use crate::views::settings::sections::settings_credits::StarsTransaction;
use crate::views::settings::sections::settings_folders::ChatFolderItem;
use crate::views::settings::sections::settings_notifications::BadgeCountType;
use crate::views::shell::AppShell;

pub(crate) fn format_ms_human(ms: i64) -> String {
    if ms <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp_millis(ms)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| ms.to_string())
}

fn is_persistent_store_file(path: &Path) -> bool {
    let fname = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    fname == "session.json"
        || fname == "preferences.json"
        || fname.ends_with(".db")
        || fname.ends_with(".db-wal")
        || fname.ends_with(".db-shm")
        || fname.ends_with(".sqlite")
        || fname.ends_with(".sqlite-wal")
        || fname.ends_with(".sqlite-shm")
        || fname.starts_with("localstorage")
}

/// Scans cached media files under `dir` into the 6 local-storage byte slots.
pub fn scan_local_cache_bytes_by_category(dir: &Path) -> [u64; 6] {
    let mut slots = [0_u64; 6];
    fn visit(path: &Path, slots: &mut [u64; 6]) {
        let Ok(entries) = std::fs::read_dir(path) else {
            return;
        };
        for entry in entries.flatten() {
            let p = entry.path();
            let Ok(meta) = entry.metadata() else {
                continue;
            };
            if meta.is_dir() {
                visit(&p, slots);
                continue;
            }
            if is_persistent_store_file(&p) {
                continue;
            }
            let len = meta.len();
            let ext = p
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();
            match ext.as_str() {
                "jpg" | "jpeg" | "png" | "webp" | "gif" | "bmp" | "heic" => slots[0] += len,
                "mp4" | "mov" | "mkv" | "webm" | "avi" => slots[1] += len,
                "ogg" | "opus" | "mp3" | "wav" | "m4a" | "flac" => slots[3] += len,
                "tgs" | "lottie" | "svg" => slots[4] += len,
                "tmp" | "cache" | "bin" => slots[5] += len,
                _ => slots[2] += len,
            }
        }
    }
    visit(dir, &mut slots);
    slots
}

/// Deletes removable cached media files under `dir` while preserving session/database state.
pub fn delete_cached_media_in_dir(dir: &Path) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        let Ok(meta) = entry.metadata() else {
            continue;
        };
        if meta.is_dir() {
            delete_cached_media_in_dir(&p);
            continue;
        }
        if is_persistent_store_file(&p) {
            continue;
        }
        let _ = std::fs::remove_file(&p);
    }
}

fn stable_session_hash(session_id: &str, idx: usize) -> u64 {
    let mut h: u64 = 14695981039346656037;
    for b in session_id.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    if h == 0 {
        (idx as u64) + 1
    } else {
        h
    }
}

pub(crate) fn apply_folders_to_shell(
    shell: &mut AppShell,
    folders: &[ChatFolderDto],
    cx: &mut Context<AppShell>,
) {
    let (nav_folders, cfg_folders) = shell.sidebar.update(cx, |sb, sb_cx| {
        sb.set_custom_folders(folders.to_vec());
        let nav: Vec<NavFolderItem> = folders
            .iter()
            .map(|f| {
                let unread: u32 = sb
                    .dialogs
                    .iter()
                    .filter(|d| sb.dialog_matches_custom_folder(d, f))
                    .map(|d| sb.dialog_unread_contribution(d))
                    .sum();
                NavFolderItem {
                    folder_id: f.id as i32,
                    title: f.title.clone(),
                    icon_hint: f.emoticon.clone().unwrap_or_default(),
                    unread_badge: unread,
                    is_muted: f.exclude_muted,
                }
            })
            .collect();
        let cfg: Vec<ChatFolderItem> = folders
            .iter()
            .map(|f| {
                let matched_count = sb
                    .dialogs
                    .iter()
                    .filter(|d| sb.dialog_matches_custom_folder(d, f))
                    .count()
                    .max(f.include_chats.len());
                let unread: usize = sb
                    .dialogs
                    .iter()
                    .filter(|d| sb.dialog_matches_custom_folder(d, f))
                    .map(|d| sb.dialog_unread_contribution(d) as usize)
                    .sum();
                ChatFolderItem {
                    id: f.id as i32,
                    title: f.title.clone(),
                    icon_name: f.emoticon.clone().unwrap_or_else(|| "Folder".into()),
                    included_peers_count: matched_count,
                    excluded_peers_count: f.exclude_chats.len(),
                    unread_count: unread,
                    is_removed: false,
                }
            })
            .collect();
        sb_cx.notify();
        (nav, cfg)
    });
    shell.nav_rail.update(cx, |r, _| r.set_folders(nav_folders));
    shell.settings_router.update(cx, |sr, scx| {
        let sf = sr.ensure_folders(scx);
        sf.update(scx, |sf, sf_cx| {
            sf.sync_folders(cfg_folders);
            sf_cx.notify();
        });
    });
}

pub(crate) fn apply_wallet_to_shell(
    shell: &mut AppShell,
    bal: &WalletBalanceDto,
    txs: &[WalletTransactionDto],
    cx: &mut Context<AppShell>,
) {
    let mapped: Vec<StarsTransaction> = txs
        .iter()
        .map(|t| StarsTransaction {
            id: t.tx_id.clone(),
            title: t.description.clone(),
            date: format_ms_human(t.created_at_ms),
            amount: t.amount_stars,
            is_outgoing: t.amount_stars < 0 || t.kind == "gift_sent",
        })
        .collect();
    let ton = bal.ton_nano as f64 / 1_000_000_000.0;
    let stars = bal.stars_balance.max(0) as u64;
    shell.settings_router.update(cx, |sr, scx| {
        let sc = sr.ensure_credits(scx);
        sc.update(scx, |sc, sc_cx| {
            sc.sync_wallet(stars, ton, mapped);
            sc_cx.notify();
        });
    });
}

impl AppShell {
    /// Clears local media cache files and updates local-storage metrics in settings.
    pub fn clear_local_cache_files(&mut self, cx: &mut Context<Self>) {
        let app_dir = self.preferences_dir.clone();
        delete_cached_media_in_dir(&app_dir);
        let slots = scan_local_cache_bytes_by_category(&app_dir);
        self.settings_router.update(cx, |sr, scx| {
            let ls = sr.ensure_local_storage(scx);
            ls.update(scx, |ls, ls_cx| {
                ls.sync_category_bytes(slots);
                ls_cx.notify();
            });
        });
        let lang = self.active_lang_id();
        self.window_runtime.enqueue_toast(
            tr_str(lang, "本地缓存已清理", "Local media cache cleared"),
            2200,
        );
        cx.notify();
    }

    /// Reads current settings state lazily from `SettingsRouter` / preferences and pushes to backend.
    pub fn push_user_settings_to_backend(&self, cx: &mut Context<Self>) {
        if !self.is_authenticated || self.session_sync.session_token.is_none() {
            return;
        }
        let sr = self.settings_router.read(cx);
        let sp = sr.settings_privacy.as_ref().map(|e| e.read(cx));
        let gt = sr.settings_global_ttl.as_ref().map(|e| e.read(cx));
        let sn = sr.settings_notifications.as_ref().map(|e| e.read(cx));
        let sc = sr.settings_calls.as_ref().map(|e| e.read(cx));
        let notif_prefs = &self.preferences.notifications;
        let has_passcode = self
            .preferences
            .local_passcode_hash
            .as_deref()
            .is_some_and(|h| !h.is_empty());
        let is_premium = sr
            .settings_premium
            .as_ref()
            .is_some_and(|e| e.read(cx).is_premium_active)
            || sr
                .settings_main
                .as_ref()
                .is_some_and(|e| e.read(cx).profile.is_premium);
        let dto = UserPrivacySettingsDto {
            phone_privacy: sp.map_or_else(|| "Contacts".to_string(), |s| s.phone_privacy.clone()),
            last_seen_privacy: sp
                .map_or_else(|| "Everybody".to_string(), |s| s.last_seen_privacy.clone()),
            photo_privacy: sp.map_or_else(|| "Everybody".to_string(), |s| s.photo_privacy.clone()),
            calls_privacy: sp.map_or_else(|| "Contacts".to_string(), |s| s.calls_privacy.clone()),
            forwards_privacy: sp
                .map_or_else(|| "Everybody".to_string(), |s| s.forwards_privacy.clone()),
            groups_privacy: sp.map_or_else(|| "Contacts".to_string(), |s| s.groups_privacy.clone()),
            ttl_months: sp.map_or(6, |s| s.ttl_months),
            global_ttl_seconds: gt
                .map(|s| s.current_ttl)
                .unwrap_or_else(|| sp.map_or(0, |s| s.global_ttl_seconds)),
            sensitive_content_enabled: sp.is_some_and(|s| s.sensitive_content_enabled),
            passcode_enabled: sp.map_or(has_passcode, |s| s.passcode_enabled),
            notifications_private: sn
                .map_or(notif_prefs.private_chats, |s| s.private_chats_enabled),
            notifications_groups: sn.map_or(notif_prefs.groups, |s| s.groups_enabled),
            notifications_channels: sn.map_or(notif_prefs.channels, |s| s.channels_enabled),
            notifications_sound: sn.map_or(notif_prefs.sound_enabled, |s| s.sound_enabled),
            include_muted_in_badge: sn
                .map_or(notif_prefs.include_muted_in_badge, |s| s.include_muted_in_badge),
            badge_count_messages: sn.map_or(notif_prefs.badge_count_messages, |s| {
                s.badge_count_type == BadgeCountType::CountMessages
            }),
            p2p_mode: sc.map_or_else(|| "Contacts".to_string(), |s| s.p2p_mode.clone()),
            accept_calls: sc.map_or(true, |s| s.accept_calls),
            is_premium,
        };
        let client = self.client.clone();
        cx.spawn(async move |_: WeakEntity<Self>, _: &mut AsyncApp| {
            let _ = client.save_my_settings(&dto).await;
        })
        .detach();
    }

    /// Applies an updated privacy rule option to settings sections and syncs to backend.
    pub fn apply_privacy_rule_update(
        &mut self,
        category: &str,
        rule: PrivacyRuleOption,
        cx: &mut Context<Self>,
    ) {
        let label = match rule {
            PrivacyRuleOption::Everyone => "Everybody",
            PrivacyRuleOption::Contacts => "Contacts",
            PrivacyRuleOption::Nobody => "Nobody",
            PrivacyRuleOption::Premium => "Premium",
            PrivacyRuleOption::ChargeStars => "Paid (Stars)",
        }
        .to_string();
        let cat_lower = category.to_ascii_lowercase();
        let update_calls = cat_lower.contains("call") || category.contains("通话");
        self.settings_router.update(cx, |sr, scx| {
            let sp_ent = sr.ensure_privacy(scx);
            sp_ent.update(scx, |sp, sp_cx| {
                if cat_lower.contains("phone") || category.contains("手机") {
                    sp.phone_privacy = label.clone();
                } else if cat_lower.contains("last") || category.contains("上线") {
                    sp.last_seen_privacy = label.clone();
                } else if cat_lower.contains("photo") || category.contains("头像") {
                    sp.photo_privacy = label.clone();
                } else if update_calls {
                    sp.calls_privacy = label.clone();
                } else if cat_lower.contains("forward") || category.contains("转发") {
                    sp.forwards_privacy = label.clone();
                } else {
                    sp.groups_privacy = label.clone();
                }
                sp_cx.notify();
            });
            if update_calls {
                let p2p = match rule {
                    PrivacyRuleOption::Everyone => "Everybody",
                    PrivacyRuleOption::Nobody => "Nobody",
                    _ => "Contacts",
                };
                let sc_ent = sr.ensure_calls(scx);
                sc_ent.update(scx, |sc, sc_cx| {
                    sc.p2p_mode = p2p.to_string();
                    sc_cx.notify();
                });
            }
        });
        self.push_user_settings_to_backend(cx);
        let lang = self.active_lang_id();
        self.window_runtime.enqueue_toast(
            tr_str(lang, "隐私规则已保存", "Privacy rule updated"),
            2000,
        );
        cx.notify();
    }

    /// Scans local cache and fetches live profile, sessions, privacy, blocked peers, folders, proxies, and wallet.
    pub fn refresh_settings_truth(&mut self, cx: &mut Context<Self>) {
        let slots = scan_local_cache_bytes_by_category(&self.preferences_dir);
        self.settings_router.update(cx, |sr, scx| {
            let ls = sr.ensure_local_storage(scx);
            ls.update(scx, |ls, ls_cx| {
                ls.sync_category_bytes(slots);
                ls_cx.notify();
            });
        });

        let (Some(token), Some(uid)) =
            (self.session_sync.session_token.clone(), self.session_sync.user_id)
        else {
            return;
        };
        let fence = SessionRequestFence {
            auth_generation: self.session_sync.auth_generation,
            request_generation: 0,
            token,
            user_id: uid,
        };
        let tok_magpie = fence.token.clone();
        let client = self.client.clone();
        cx.spawn(async move |weak: WeakEntity<Self>, app: &mut AsyncApp| {
            let me_res = client.get_my_profile().await.ok();
            let sessions_res = client.list_sessions().await.ok();
            let twofa_res = client.get_2fa_status().await.ok();
            let blocked_res = client.list_blocked_users().await.ok();
            let my_settings_res = client.get_my_settings().await.ok();
            let folders_res = client.list_folders().await.ok();
            let proxies_res = client.list_proxies().await.ok();
            let bal_res = client.wallet_balance().await.ok();
            let txs_res = client.wallet_transactions().await.unwrap_or_default();
            let magpie_res = client.hermes.magpie_state(&tok_magpie).await.ok();

            let _ = weak.update(app, |shell, cx| {
                if !fence.matches(
                    shell.is_authenticated,
                    shell.session_sync.auth_generation,
                    0,
                    shell.session_sync.session_token.as_deref(),
                    shell.session_sync.user_id,
                ) {
                    return;
                }

                let existing_chats_count = shell.sidebar.read(cx).dialogs.len();

                if let Some(me) = me_res {
                    let first = me.first_name.clone();
                    let last = me.last_name.clone().unwrap_or_default();
                    let full_name = format!("{} {}", first, last).trim().to_string();
                    let uname = me.username.clone().unwrap_or_default();
                    let phone = me.phone.clone().unwrap_or_default();
                    let bio = me.bio.clone().unwrap_or_default();
                    let display_name = if full_name.is_empty() {
                        if !uname.is_empty() {
                            uname.clone()
                        } else {
                            format!("User #{}", me.user_id)
                        }
                    } else {
                        full_name
                    };
                    if !uname.is_empty() {
                        shell.session_sync.username = Some(uname.clone());
                    }
                    shell.settings_router.update(cx, |sr, scx| {
                        let sm = sr.ensure_main(scx);
                        sm.update(scx, |sm, sm_cx| {
                            sm.sync_profile(
                                display_name.clone(),
                                phone.clone(),
                                uname.clone(),
                                bio.clone(),
                                me.is_verified,
                                me.is_premium,
                            );
                            sm_cx.notify();
                        });
                        let si = sr.ensure_information(scx);
                        si.update(scx, |si, si_cx| {
                            si.sync_from_profile(
                                me.user_id,
                                first,
                                last,
                                uname,
                                phone.clone(),
                                bio,
                                me.is_premium,
                            );
                            si_cx.notify();
                        });
                        let sprem = sr.ensure_premium(scx);
                        sprem.update(scx, |sprem, sprem_cx| {
                            sprem.is_premium_active = me.is_premium;
                            sprem_cx.notify();
                        });
                        let sbiz = sr.ensure_business(scx);
                        sbiz.update(scx, |biz, biz_cx| {
                            biz.business_active = me.is_premium;
                            biz_cx.notify();
                        });
                    });
                    shell.main_menu_drawer.update(cx, |drawer, d_cx| {
                        if let Some(acc) = drawer
                            .accounts
                            .iter_mut()
                            .find(|a| a.account_id == drawer.active_account_id)
                        {
                            acc.display_name = display_name;
                            if !phone.is_empty() {
                                acc.phone = phone;
                            }
                            acc.is_premium = me.is_premium;
                        }
                        d_cx.notify();
                    });
                }

                if let Some(sessions) = sessions_res {
                    let count = sessions.len();
                    let mut ids_map = HashMap::new();
                    let rows: Vec<SessionRowData> = sessions
                        .into_iter()
                        .enumerate()
                        .map(|(idx, s)| {
                            let h = stable_session_hash(&s.session_id, idx);
                            ids_map.insert(h, s.session_id.clone());
                            SessionRowData {
                                hash: h,
                                app_name: "Fabushi Desktop".to_string(),
                                device_model: if s.device_name.is_empty() {
                                    "Desktop Session".to_string()
                                } else {
                                    s.device_name
                                },
                                platform: s.platform.unwrap_or_else(|| "macOS".to_string()),
                                ip: s.ip_address.unwrap_or_else(|| "127.0.0.1".to_string()),
                                location: if s.is_current {
                                    "Current Session".to_string()
                                } else {
                                    "Active".to_string()
                                },
                                is_current: s.is_current,
                            }
                        })
                        .collect();
                    shell.settings_router.update(cx, |sr, scx| {
                        let sas = sr.ensure_active_sessions(scx);
                        sas.update(scx, |sas, sas_cx| {
                            sas.sessions = rows;
                            sas.session_ids_by_hash = ids_map;
                            sas_cx.notify();
                        });
                        let sp = sr.ensure_privacy(scx);
                        sp.update(scx, |sp, sp_cx| {
                            sp.active_sessions_count = count;
                            sp_cx.notify();
                        });
                    });
                }

                if let Some(twofa) = twofa_res {
                    shell.settings_router.update(cx, |sr, scx| {
                        let sp = sr.ensure_privacy(scx);
                        sp.update(scx, |sp, sp_cx| {
                            sp.set_two_factor_enabled(twofa.has_password);
                            sp_cx.notify();
                        });
                    });
                }

                if let Some(blocked) = blocked_res {
                    let count = blocked.len();
                    let items: Vec<BlockedPeerItem> = blocked
                        .into_iter()
                        .map(|b| {
                            let full = format!("{} {}", b.first_name, b.last_name.unwrap_or_default())
                                .trim()
                                .to_string();
                            let name = if !full.is_empty() {
                                full
                            } else if let Some(u) = b.username {
                                format!("@{}", u)
                            } else {
                                format!("User #{}", b.user_id)
                            };
                            BlockedPeerItem {
                                user_id: b.user_id,
                                name,
                                blocked_date: format_ms_human(b.blocked_at_ms),
                            }
                        })
                        .collect();
                    shell.settings_router.update(cx, |sr, scx| {
                        let sbp = sr.ensure_blocked_peers(scx);
                        sbp.update(scx, |sbp, sbp_cx| {
                            sbp.blocked_list = items;
                            sbp_cx.notify();
                        });
                        let sp = sr.ensure_privacy(scx);
                        sp.update(scx, |sp, sp_cx| {
                            sp.set_blocked_count(count);
                            sp_cx.notify();
                        });
                    });
                }

                if let Some(cfg) = my_settings_res {
                    let inc_muted = cfg.include_muted_in_badge;
                    let cnt_msgs = cfg.badge_count_messages;
                    let local_passcode_enabled = shell.has_local_passcode_configured();
                    shell.preferences.notifications.private_chats = cfg.notifications_private;
                    shell.preferences.notifications.groups = cfg.notifications_groups;
                    shell.preferences.notifications.channels = cfg.notifications_channels;
                    shell.preferences.notifications.sound_enabled = cfg.notifications_sound;
                    shell.preferences.notifications.include_muted_in_badge = inc_muted;
                    shell.preferences.notifications.badge_count_messages = cnt_msgs;
                    let _ = shell.preferences.save_to_dir(&shell.preferences_dir);
                    shell.sidebar.update(cx, |sb, sb_cx| {
                        sb.set_notification_badge_rules(cnt_msgs, inc_muted);
                        sb_cx.notify();
                    });
                    shell.settings_router.update(cx, |sr, scx| {
                        let sp = sr.ensure_privacy(scx);
                        sp.update(scx, |sp, sp_cx| {
                            sp.phone_privacy = cfg.phone_privacy;
                            sp.last_seen_privacy = cfg.last_seen_privacy;
                            sp.photo_privacy = cfg.photo_privacy;
                            sp.calls_privacy = cfg.calls_privacy;
                            sp.forwards_privacy = cfg.forwards_privacy;
                            sp.groups_privacy = cfg.groups_privacy;
                            sp.set_ttl_months(cfg.ttl_months);
                            sp.global_ttl_seconds = cfg.global_ttl_seconds;
                            sp.sensitive_content_enabled = cfg.sensitive_content_enabled;
                            sp.passcode_enabled = local_passcode_enabled;
                            sp_cx.notify();
                        });
                        let gt = sr.ensure_global_ttl(scx);
                        gt.update(scx, |gt, gt_cx| {
                            gt.current_ttl = cfg.global_ttl_seconds;
                            gt.existing_chats_count = existing_chats_count;
                            gt_cx.notify();
                        });
                        let sn = sr.ensure_notifications(scx);
                        sn.update(scx, |sn, sn_cx| {
                            sn.private_chats_enabled = cfg.notifications_private;
                            sn.groups_enabled = cfg.notifications_groups;
                            sn.channels_enabled = cfg.notifications_channels;
                            sn.sound_enabled = cfg.notifications_sound;
                            sn.include_muted_in_badge = cfg.include_muted_in_badge;
                            sn.badge_count_type = if cfg.badge_count_messages {
                                BadgeCountType::CountMessages
                            } else {
                                BadgeCountType::CountChats
                            };
                            sn_cx.notify();
                        });
                        let sc = sr.ensure_calls(scx);
                        sc.update(scx, |sc, sc_cx| {
                            sc.p2p_mode = cfg.p2p_mode;
                            sc.accept_calls = cfg.accept_calls;
                            sc_cx.notify();
                        });
                        if cfg.is_premium {
                            let sprem = sr.ensure_premium(scx);
                            sprem.update(scx, |sprem, sprem_cx| {
                                sprem.is_premium_active = true;
                                sprem_cx.notify();
                            });
                            let sm = sr.ensure_main(scx);
                            sm.update(scx, |sm, sm_cx| {
                                sm.profile.is_premium = true;
                                sm_cx.notify();
                            });
                            let sbiz = sr.ensure_business(scx);
                            sbiz.update(scx, |biz, biz_cx| {
                                biz.business_active = true;
                                biz_cx.notify();
                            });
                        }
                    });
                }

                if let Some(proxies) = proxies_res {
                    let active_proxy = proxies
                        .iter()
                        .find(|p| p.enabled && !p.kind.eq_ignore_ascii_case("Direct"))
                        .or_else(|| proxies.iter().find(|p| p.enabled));
                    shell.settings_router.update(cx, |sr, scx| {
                        let sa = sr.ensure_advanced(scx);
                        sa.update(scx, |sa, sa_cx| {
                            if let Some(p) = active_proxy {
                                sa.sync_proxy(&p.kind, &p.host, p.port, p.enabled);
                            } else {
                                sa.sync_proxy("Direct", "", 1080, false);
                            }
                            sa_cx.notify();
                        });
                    });
                }

                if let Some(folders) = folders_res {
                    apply_folders_to_shell(shell, &folders, cx);
                }

                if let Some(b) = bal_res {
                    apply_wallet_to_shell(shell, &b, &txs_res, cx);
                }

                if let Some(ms) = magpie_res {
                    shell.apply_magpie_state_to_views(ms, cx);
                }

                cx.notify();
            });
        })
        .detach();
    }

    /// Hydrates composer/stage/sidebar behavior and lazily syncs persisted `AppPreferences` into `SettingsRouter`.
    pub fn hydrate_settings_from_preferences(&mut self, cx: &mut Context<Self>) {
        let prefs = self.preferences.clone();
        self.theme_state.bubble_radius_px = prefs.appearance.bubble_radius;
        self.composer.update(cx, |comp, _| {
            comp.send_on_enter = prefs.appearance.send_on_enter;
            comp.suggest_stickers_by_emoji = prefs.appearance.suggest_stickers_by_emoji;
            comp.loop_animated_stickers = prefs.appearance.loop_animated_stickers;
            comp.stickers_list.loop_animated = prefs.appearance.loop_animated_stickers;
        });
        self.stage.update(cx, |st, _| {
            st.loop_animated_stickers = prefs.appearance.loop_animated_stickers;
        });
        self.sidebar.update(cx, |sb, _| {
            sb.folder_tags_enabled = prefs.folder_tags_enabled;
            sb.set_notification_badge_rules(
                prefs.notifications.badge_count_messages,
                prefs.notifications.include_muted_in_badge,
            );
        });
        self.settings_router.update(cx, |sr, scx| {
            sr.sync_preferences(prefs, scx);
        });
        self.sync_theme_and_locale_to_views(cx);
    }
}
