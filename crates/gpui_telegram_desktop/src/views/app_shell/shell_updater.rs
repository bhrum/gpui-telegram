//! Automatic background update detection, manual check, and one-click self-update orchestration for `AppShell`.

use std::time::Duration;

use gpui::*;
use tokio::sync::mpsc;

use crate::app::updater::{
    DesktopUpdateState, DesktopUpdater, CURRENT_APP_VERSION,
};
use crate::i18n::tr_str;
use crate::views::shell::AppShell;

#[derive(Debug)]
enum DownloadProgressEvent {
    Progress {
        downloaded_bytes: u64,
        total_bytes: Option<u64>,
        progress_pct: u8,
    },
    Finished(Result<std::path::PathBuf, String>),
}

impl AppShell {
    fn sync_updater_state_to_views(&mut self, cx: &mut Context<Self>) {
        let is_zh = self.theme_state.is_chinese_locale;
        let snap = self.overlays.updater_state.to_snapshot(is_zh);
        let available_ver = match &self.overlays.updater_state {
            DesktopUpdateState::Available { info } => Some(info.version.clone()),
            _ => None,
        };
        self.settings_router.update(cx, |sr, scx| {
            sr.sync_update_snapshot(snap, scx);
        });
        self.main_menu_drawer.update(cx, |drawer, dcx| {
            drawer.set_available_update(available_ver);
            dcx.notify();
        });
        cx.notify();
    }

    /// Spawns a periodic background updater loop (initial 2s delay, then every 30 minutes).
    pub(crate) fn spawn_auto_update_loop(&self, cx: &mut Context<Self>) {
        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            app.background_executor().timer(Duration::from_secs(2)).await;
            loop {
                let keep = weak_shell
                    .update(app, |shell, cx| {
                        let force = std::env::var("FABUSHI_FORCE_UPDATE_CHECK")
                            .is_ok_and(|v| v == "1" || v.eq_ignore_ascii_case("true"));
                        if shell.preferences.auto_update_enabled || force {
                            shell.trigger_update_check(false, cx);
                        }
                        true
                    })
                    .unwrap_or(false);
                if !keep {
                    break;
                }
                app.background_executor()
                    .timer(Duration::from_secs(1800))
                    .await;
            }
        })
        .detach();
    }

    /// Checks for a newer Fabushi release from the configured feed or GitHub Releases.
    pub fn trigger_update_check(&mut self, manual: bool, cx: &mut Context<Self>) {
        if self.overlays.update_op_in_flight {
            return;
        }
        self.overlays.update_op_in_flight = true;
        self.overlays.updater_state = DesktopUpdateState::Checking;
        self.sync_updater_state_to_views(cx);

        let feed_override = self.client.config.update_feed_url.clone();
        let runtime = self.client.runtime().clone();

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = runtime
                .spawn(async move {
                    DesktopUpdater::check_for_updates(
                        feed_override.as_deref(),
                        CURRENT_APP_VERSION,
                    )
                    .await
                })
                .await
                .unwrap_or_else(|e| Err(format!("Updater task panicked: {e}")));

            let _ = weak_shell.update(app, |shell, cx| {
                shell.overlays.update_op_in_flight = false;
                let is_zh = shell.theme_state.is_chinese_locale;
                match res {
                    Ok(Some(info)) => {
                        let ver = info.version.clone();
                        shell.overlays.update_banner_dismissed_version = None;
                        shell.overlays.updater_state = DesktopUpdateState::Available { info };
                        let msg = if is_zh {
                            format!("发现新版本 Fabushi v{ver}，可一键更新")
                        } else {
                            format!("Fabushi v{ver} is available for one-click update")
                        };
                        shell.window_runtime.enqueue_toast(msg, 3200);
                    }
                    Ok(None) => {
                        shell.overlays.updater_state = DesktopUpdateState::UpToDate {
                            checked_version: CURRENT_APP_VERSION.to_string(),
                        };
                        if manual {
                            let msg = if is_zh {
                                format!("当前已是最新版本 Fabushi v{CURRENT_APP_VERSION}")
                            } else {
                                format!("You're up to date (Fabushi v{CURRENT_APP_VERSION})")
                            };
                            shell.window_runtime.enqueue_toast(msg, 2400);
                        }
                    }
                    Err(err) => {
                        shell.overlays.updater_state = DesktopUpdateState::Idle {
                            last_checked_ok: false,
                            last_error: Some(err.clone()),
                        };
                        if manual {
                            let prefix = tr_str(
                                shell.active_lang_id(),
                                "检查更新失败",
                                "Failed to check for updates",
                            );
                            shell
                                .window_runtime
                                .enqueue_toast(format!("{prefix}: {err}"), 3000);
                        }
                    }
                }
                shell.sync_updater_state_to_views(cx);
            });
        })
        .detach();
    }

    /// Downloads, verifies SHA-256, unpacks, and atomically installs the pending update before relaunching.
    pub fn trigger_one_click_update(&mut self, cx: &mut Context<Self>) {
        if self.overlays.update_op_in_flight {
            return;
        }
        if let DesktopUpdateState::ReadyToRestart {
            version,
            staged_app_path,
        } = self.overlays.updater_state.clone()
        {
            self.finish_and_relaunch_update(&version, &staged_app_path, cx);
            return;
        }

        let Some(info) = self.overlays.updater_state.actionable_info().cloned() else {
            self.trigger_update_check(true, cx);
            return;
        };

        self.overlays.update_op_in_flight = true;
        self.overlays.update_banner_dismissed_version = None;
        self.overlays.updater_state = DesktopUpdateState::Downloading {
            info: info.clone(),
            downloaded_bytes: 0,
            total_bytes: info.size_bytes,
            progress_pct: 0,
        };
        self.sync_updater_state_to_views(cx);

        let staging_root = self.preferences_dir.join("updates");
        let runtime = self.client.runtime().clone();
        let (tx, mut rx) = mpsc::unbounded_channel::<DownloadProgressEvent>();

        let dl_info = info.clone();
        runtime.spawn(async move {
            let tx_progress = tx.clone();
            let res = DesktopUpdater::download_and_stage_update(
                &dl_info,
                &staging_root,
                move |downloaded_bytes, total_bytes, progress_pct| {
                    let _ = tx_progress.send(DownloadProgressEvent::Progress {
                        downloaded_bytes,
                        total_bytes,
                        progress_pct,
                    });
                },
            )
            .await;
            let _ = tx.send(DownloadProgressEvent::Finished(res));
        });

        cx.spawn(async move |weak_shell: WeakEntity<Self>, app: &mut AsyncApp| {
            while let Some(ev) = rx.recv().await {
                match ev {
                    DownloadProgressEvent::Progress {
                        downloaded_bytes,
                        total_bytes,
                        progress_pct,
                    } => {
                        let ok = weak_shell
                            .update(app, |shell, cx| {
                                shell.overlays.updater_state = DesktopUpdateState::Downloading {
                                    info: info.clone(),
                                    downloaded_bytes,
                                    total_bytes,
                                    progress_pct,
                                };
                                shell.sync_updater_state_to_views(cx);
                            })
                            .is_ok();
                        if !ok {
                            break;
                        }
                    }
                    DownloadProgressEvent::Finished(res) => {
                        let _ = weak_shell.update(app, |shell, cx| {
                            shell.overlays.update_op_in_flight = false;
                            match res {
                                Ok(staged_app) => {
                                    let ver = info.version.clone();
                                    shell.overlays.updater_state =
                                        DesktopUpdateState::ReadyToRestart {
                                            version: ver.clone(),
                                            staged_app_path: staged_app.clone(),
                                        };
                                    shell.sync_updater_state_to_views(cx);
                                    shell.finish_and_relaunch_update(&ver, &staged_app, cx);
                                }
                                Err(err) => {
                                    shell.overlays.updater_state = DesktopUpdateState::Failed {
                                        error: err.clone(),
                                        retry_info: Some(info.clone()),
                                    };
                                    let prefix = tr_str(
                                        shell.active_lang_id(),
                                        "更新安装失败",
                                        "Update installation failed",
                                    );
                                    shell
                                        .window_runtime
                                        .enqueue_toast(format!("{prefix}: {err}"), 3400);
                                    shell.sync_updater_state_to_views(cx);
                                }
                            }
                        });
                        break;
                    }
                }
            }
        })
        .detach();
    }

    fn finish_and_relaunch_update(
        &mut self,
        version: &str,
        staged_app: &std::path::Path,
        cx: &mut Context<Self>,
    ) {
        self.overlays.updater_state = DesktopUpdateState::Installing {
            version: version.to_string(),
        };
        self.sync_updater_state_to_views(cx);

        let exe_path = std::env::current_exe().unwrap_or_default();
        let target_app = DesktopUpdater::resolve_target_app_bundle(&exe_path);
        match DesktopUpdater::apply_staged_update_and_relaunch(
            staged_app,
            &target_app,
            std::process::id(),
            true,
        ) {
            Ok(_) => {
                cx.quit();
            }
            Err(err) => {
                self.overlays.updater_state = DesktopUpdateState::Failed {
                    error: err.clone(),
                    retry_info: None,
                };
                let prefix = tr_str(
                    self.active_lang_id(),
                    "无法启动更新替换程序",
                    "Failed to launch self-updater",
                );
                self.window_runtime
                    .enqueue_toast(format!("{prefix}: {err}"), 3400);
                self.sync_updater_state_to_views(cx);
            }
        }
    }

    /// Renders a non-intrusive top update pill/banner when an update is available, downloading, or failed.
    pub(crate) fn render_in_app_update_banner(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let pal = self.theme_state.palette;
        let is_zh = self.theme_state.is_chinese_locale;
        match &self.overlays.updater_state {
            DesktopUpdateState::Available { info } => {
                if self.overlays.update_banner_dismissed_version.as_deref()
                    == Some(info.version.as_str())
                {
                    return None;
                }
                let ver = info.version.clone();
                let ver_dismiss = ver.clone();
                let title = if is_zh {
                    format!("发现新版本 Fabushi v{ver}")
                } else {
                    format!("Fabushi v{ver} is available")
                };
                let btn_label = if is_zh { "一键更新" } else { "Update & Restart" };
                let later_label = if is_zh { "稍后" } else { "Later" };
                Some(
                    div()
                        .id("in-app-update-available-banner")
                        .w_full()
                        .px(px(14.0))
                        .py(px(7.0))
                        .bg(rgb(pal.card_bg))
                        .border_b_1()
                        .border_color(rgb(pal.border_fg))
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(px(10.0))
                        .child(
                            div()
                                .text_size(px(12.5))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(pal.text_primary))
                                .child(title),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .id("in-app-update-apply-btn")
                                        .cursor_pointer()
                                        .px(px(12.0))
                                        .py(px(4.0))
                                        .rounded(px(6.0))
                                        .bg(rgb(pal.accent))
                                        .hover(move |s| s.bg(rgb(pal.accent_hover)))
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(pal.active_button_fg))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _e, _w, cx| {
                                                this.trigger_one_click_update(cx);
                                            }),
                                        )
                                        .child(btn_label),
                                )
                                .child(
                                    div()
                                        .id("in-app-update-later-btn")
                                        .cursor_pointer()
                                        .px(px(8.0))
                                        .py(px(4.0))
                                        .rounded(px(6.0))
                                        .hover(move |s| s.bg(rgb(pal.dialogs_hover_bg)))
                                        .text_size(px(12.0))
                                        .text_color(rgb(pal.text_secondary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _e, _w, cx| {
                                                this.overlays.update_banner_dismissed_version =
                                                    Some(ver_dismiss.clone());
                                                cx.notify();
                                            }),
                                        )
                                        .child(later_label),
                                ),
                        )
                        .into_any_element(),
                )
            }
            DesktopUpdateState::Downloading {
                info, progress_pct, ..
            } => {
                let text = if is_zh {
                    format!("正在下载 Fabushi v{} ({progress_pct}%)…", info.version)
                } else {
                    format!("Downloading Fabushi v{} ({progress_pct}%)…", info.version)
                };
                Some(
                    div()
                        .id("in-app-update-downloading-banner")
                        .w_full()
                        .px(px(14.0))
                        .py(px(6.0))
                        .bg(rgb(pal.accent))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(pal.active_button_fg))
                        .child(text)
                        .into_any_element(),
                )
            }
            DesktopUpdateState::Installing { version }
            | DesktopUpdateState::ReadyToRestart { version, .. } => {
                let text = if is_zh {
                    format!("正在安装 Fabushi v{version} 并自动重启…")
                } else {
                    format!("Installing Fabushi v{version} & restarting…")
                };
                Some(
                    div()
                        .id("in-app-update-installing-banner")
                        .w_full()
                        .px(px(14.0))
                        .py(px(6.0))
                        .bg(rgb(pal.accent))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(pal.active_button_fg))
                        .child(text)
                        .into_any_element(),
                )
            }
            DesktopUpdateState::Failed { error, retry_info } => retry_info.as_ref().map(|_| {
                let text = if is_zh {
                    format!("更新失败: {error}")
                } else {
                    format!("Update failed: {error}")
                };
                let retry_lbl = if is_zh { "重试更新" } else { "Retry Update" };
                let close_lbl = if is_zh { "关闭" } else { "Dismiss" };
                div()
                    .id("in-app-update-failed-banner")
                    .w_full()
                    .px(px(14.0))
                    .py(px(7.0))
                    .bg(rgb(pal.card_bg))
                    .border_b_1()
                    .border_color(rgb(pal.danger))
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(10.0))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(pal.danger))
                            .child(text),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id("in-app-update-retry-btn")
                                    .cursor_pointer()
                                    .px(px(10.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .bg(rgb(pal.accent))
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(pal.active_button_fg))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.trigger_one_click_update(cx);
                                        }),
                                    )
                                    .child(retry_lbl),
                            )
                            .child(
                                div()
                                    .id("in-app-update-dismiss-err-btn")
                                    .cursor_pointer()
                                    .px(px(8.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .text_size(px(12.0))
                                    .text_color(rgb(pal.text_secondary))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.overlays.updater_state = DesktopUpdateState::default();
                                            this.sync_updater_state_to_views(cx);
                                        }),
                                    )
                                    .child(close_lbl),
                            ),
                    )
                    .into_any_element()
            }),
            DesktopUpdateState::Idle { .. }
            | DesktopUpdateState::Checking
            | DesktopUpdateState::UpToDate { .. } => None,
        }
    }
}
