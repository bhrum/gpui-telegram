//! Tier 3 headless GPUI entity graph, asset source, theme, i18n, and interaction driver.

use std::sync::Arc;

use gpui::{AppContext, Application, AssetSource, Entity};
use gpui_telegram_client::net::http_client::HistoricalMessage;
use gpui_telegram_client::{ChatSummary, ClientConfig, TelegramClient};
use gpui_telegram_desktop::i18n::{tr_key_or, tr_str};
use gpui_telegram_desktop::theme::{
    IconName, TdesktopThemeMode, TdesktopThemePalette, TelegramAssetSource,
};
use gpui_telegram_desktop::views::app_shell::shell_truth::{
    apply_fenced_chat_list_response, apply_fenced_history_response, select_authoritative_dialog,
    try_consume_composer_for_send, SessionRequestFence,
};
use gpui_telegram_desktop::views::composer::KeyAction;
use gpui_telegram_desktop::views::settings::settings_type::SettingsSectionType;
use gpui_telegram_desktop::views::shell::ActiveWorkspacePanel;
use gpui_telegram_desktop::views::AppShell;

/// Drives headless GPUI desktop entity graph, SVG/locale asset checks, and theme transitions.
pub struct HeadlessTranscriptDriver;

impl HeadlessTranscriptDriver {
    /// Verifies all 64 embedded SVG icons and localization packs load through `AssetSource`.
    pub fn verify_all_embedded_assets() {
        let src = TelegramAssetSource::new();
        assert_eq!(IconName::ALL.len(), 64, "Expected 64 canonical SVG icons");
        for &icon in IconName::ALL {
            let loaded = src
                .load(icon.path())
                .expect("AssetSource::load must succeed")
                .expect("Embedded SVG icon must exist");
            let text = std::str::from_utf8(loaded.as_ref()).expect("SVG bytes must be UTF-8");
            assert!(text.contains("<svg"), "Icon {} missing <svg>", icon.path());
        }
        for locale_path in ["langs/en.strings", "langs/zh-CN.strings"] {
            let data = src
                .load(locale_path)
                .expect("load locale strings")
                .expect("locale strings file present");
            let s = std::str::from_utf8(data.as_ref()).expect("locale file UTF-8");
            assert!(s.contains('='), "Locale file {locale_path} must contain entries");
        }
        let icon_entries = src.list("icons").expect("list icons");
        assert!(icon_entries.len() >= IconName::ALL.len());
        let lang_entries = src.list("langs").expect("list langs");
        assert!(lang_entries.iter().any(|p| p.as_ref() == "langs/en.strings"));
        assert!(lang_entries.iter().any(|p| p.as_ref() == "langs/zh-CN.strings"));
    }

    /// Verifies `Night`, `Tinted`, `Day`, and `Classic` palettes plus `en`/`zh-CN` strings.
    pub fn verify_themes_and_localization() {
        let night = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x2AABEE, 14, 12);
        let tinted = TdesktopThemePalette::build(TdesktopThemeMode::Tinted, 0x2AABEE, 14, 12);
        let day = TdesktopThemePalette::build(TdesktopThemeMode::Day, 0x3390EC, 15, 10);
        let classic = TdesktopThemePalette::build(TdesktopThemeMode::Classic, 0x3F8E47, 14, 12);

        assert!(night.mode.is_dark() && tinted.mode.is_dark());
        assert!(!day.mode.is_dark() && !classic.mode.is_dark());
        assert_ne!(night.window_bg, day.window_bg);
        assert_ne!(tinted.window_bg, night.window_bg);
        assert_ne!(classic.msg_out_bg, night.msg_out_bg);
        let aurora = night.with_wallpaper("aurora_dusk");
        assert_ne!(aurora.wallpaper_top, aurora.wallpaper_bottom);
        let (top_c, bot_c) = night.peer_avatar_gradient(42);
        assert_ne!(top_c, 0);
        assert_ne!(bot_c, 0);

        assert_eq!(tr_str("zh-CN", "设置", "Settings"), "设置");
        assert_eq!(tr_str("en", "设置", "Settings"), "Settings");
        assert!(!tr_key_or("en", "lng_settings_chat", "Chat Settings").is_empty());
        assert!(!tr_key_or("zh-CN", "lng_settings_chat", "聊天设置").is_empty());
    }

    /// Boots `gpui::Application::headless()` with `TelegramAssetSource`, instantiates `AppShell`
    /// and all child entities, drives theme, settings routing, chat/history truth sync, and
    /// composer send gating, then exits cleanly (`process::exit(0)` to avoid `CFRunLoopRun`).
    pub fn run_gpui_headless_app_shell_child(server_url: &str) -> ! {
        let config = ClientConfig {
            server_url: server_url.to_string(),
            ..Default::default()
        };
        let client = Arc::new(TelegramClient::new(config));
        let _tokio_guard = client.runtime().enter();

        Application::headless()
            .with_assets(TelegramAssetSource::new())
            .run(move |cx| {
                let svg_probe = cx
                    .asset_source()
                    .load("icons/search.svg")
                    .expect("load search.svg")
                    .expect("search.svg present");
                assert!(std::str::from_utf8(svg_probe.as_ref()).unwrap().contains("<svg"));

                let shell: Entity<AppShell> = cx.new(|cx| AppShell::new_with_client(client.clone(), cx));
                shell.update(cx, |s, cx| {
                    assert!(!s.is_authenticated);
                    assert!(s.sidebar.read(cx).dialogs.is_empty());
                    assert!(s.stage.read(cx).messages.is_empty());

                    s.is_authenticated = true;
                    s.user_id = Some(9001);
                    s.session_token = Some("headless_tok".into());
                    s.auth_generation = 1;
                    s.chat_list_request_generation = 1;
                    s.history_request_generation = 1;

                    s.main_menu_drawer.update(cx, |d, _| d.open());
                    assert!(s.main_menu_drawer.read(cx).is_open);
                    s.main_menu_drawer.update(cx, |d, _| d.close());

                    s.apply_theme_selection(
                        TdesktopThemeMode::Day,
                        Some(0x3390EC),
                        Some("default_day".into()),
                        cx,
                    );
                    assert_eq!(s.theme_mode, TdesktopThemeMode::Day);
                    assert!(!s.composer.read(cx).palette.mode.is_dark());
                    assert!(!s.nav_rail.read(cx).palette.mode.is_dark());
                    assert!(!s.main_menu_drawer.read(cx).palette.mode.is_dark());

                    s.apply_theme_selection(TdesktopThemeMode::Night, None, None, cx);
                    assert_eq!(s.theme_mode, TdesktopThemeMode::Night);

                    assert!(
                        s.sidebar.read(cx).nav_rail.is_some(),
                        "unified sidebar should be attached to NavRail in AppShell"
                    );
                    let was_collapsed = s.sidebar.read(cx).is_collapsed;
                    s.sidebar.update(cx, |sb, sb_cx| {
                        sb.toggle_collapsed_from_user();
                        sb_cx.notify();
                    });
                    assert_ne!(s.sidebar.read(cx).is_collapsed, was_collapsed);
                    assert!(
                        s.sidebar.read(cx).is_collapsed,
                        "toggling unified sidebar should collapse it"
                    );
                    assert!(
                        (s.sidebar.read(cx).effective_width() - 64.0).abs() < f32::EPSILON
                    );
                    s.sidebar.update(cx, |sb, sb_cx| {
                        sb.toggle_collapsed_from_user();
                        sb_cx.notify();
                    });
                    assert!(
                        !s.sidebar.read(cx).is_collapsed,
                        "toggling unified sidebar again should expand it"
                    );
                    assert!(
                        (s.sidebar.read(cx).effective_width() - 288.0).abs() < f32::EPSILON
                    );

                    s.navigate_settings_to(SettingsSectionType::Chat, cx);
                    assert_eq!(s.active_panel, ActiveWorkspacePanel::Settings);
                    assert_eq!(s.settings_section, SettingsSectionType::Chat);
                    s.active_panel = ActiveWorkspacePanel::Chats;

                    let fence = SessionRequestFence {
                        auth_generation: 1,
                        request_generation: 1,
                        token: "headless_tok".into(),
                        user_id: 9001,
                    };
                    let chats = vec![ChatSummary {
                        chat_id: 501,
                        title: "Design Guild".into(),
                        chat_type: 1,
                        unread_count: 1,
                        last_message: Some("Hello GPUI".into()),
                        is_group: Some(true),
                        is_bot: Some(false),
                        avatar_color: Some(0x2AABEE),
                        updated_at: Some(1_700_000_000_000),
                    }];
                    let sidebar_ent = s.sidebar.clone();
                    let stage_ent = s.stage.clone();
                    let insp_ent = s.inspector.clone();
                    let comp_ent = s.composer.clone();
                    let is_auth = s.is_authenticated;
                    let tok = s.session_token.clone();
                    let uid = s.user_id;

                    let (sb_applied, consumed) = sidebar_ent.update(cx, |sb, cx| {
                        stage_ent.update(cx, |st, cx| {
                            insp_ent.update(cx, |insp, cx| {
                                let ok = apply_fenced_chat_list_response(
                                    &fence, true, 1, 1, Some("headless_tok"), Some(9001),
                                    sb, st, insp, &chats,
                                );
                                assert!(ok && select_authoritative_dialog(sb, st, insp, 501));
                                let hist_ok = apply_fenced_history_response(
                                    &fence, true, 1, 1, Some("headless_tok"), Some(9001),
                                    sb, st, 501,
                                    vec![HistoricalMessage {
                                        message_id: 77,
                                        sender_id: 9001,
                                        text: "Synced msg".into(),
                                        content: None,
                                        date: Some(1_700_000_000_000),
                                        reply_to: None,
                                        is_outgoing: Some(true),
                                        ..Default::default()
                                    }],
                                );
                                let sent = comp_ent.update(cx, |c, _| {
                                    c.set_text("Line 1");
                                    assert_eq!(c.handle_enter(true), KeyAction::InsertNewline);
                                    assert_eq!(c.text(), "Line 1\n");
                                    assert_eq!(c.handle_enter(false), KeyAction::SendMessage);
                                    c.submit_primary();
                                    try_consume_composer_for_send(
                                        is_auth, tok.as_deref(), uid, sb, st, c,
                                    )
                                });
                                (hist_ok, sent)
                            })
                        })
                    });
                    assert!(sb_applied);
                    assert_eq!(s.stage.read(cx).messages.len(), 1);
                    assert_eq!(consumed.expect("send target").1, "Line 1");
                });

                println!("GPUI_HEADLESS_APP_SHELL_OK");
                std::process::exit(0);
            });

        std::process::exit(1);
    }
}
