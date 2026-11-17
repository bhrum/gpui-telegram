//! Suite 2: GPUI Headless Desktop Acceptance Tests for BotFather & Mini Apps.
//!
//! Mounts real GPUI desktop entities inside `gpui::Application::headless().with_assets(TelegramAssetSource::new())`:
//! - `AppShell`, `BotAttachWebView`, `AttachBotWebviewChatAttach`,
//!   `HistoryViewBotMenuButtonViewControls`, `ConversationStage`, `ChatSidebar`,
//!   `MainMenuDrawer`, `CreateManagedBotBoxPeers`, `CanonicalComposer`, `SidebarSearchView`.
//!
//! Covers:
//! 1. Component mounting & navigation (`MainMenuDrawer` BotFather + Mini Apps entries,
//!    `ChatSidebar` / `SidebarSearchView` `Apps` tab empty query, `CreateManagedBotBoxPeers`,
//!    `AttachBotWebviewChatAttach` fullscreen toggle).
//! 2. `ConversationStage` `InlineButtonType::WebApp` vs `Callback` dispatch in `AppShell`:
//!    clicking `WebApp` launches `BotAttachWebView` directly (NEVER copies URL to clipboard
//!    or shows "Copied WebApp URL" toast), whereas `Callback` leaves `miniapp_overlay` closed.
//! 3. `BotAttachWebView` full runtime transitions driven by live `TelegramClient` HTTP payloads:
//!    `load_context`, `accept_disclaimer`, `MainButton` (`send_data`), `BackButton`,
//!    `Minimize` / `Restore` / `Fullscreen`, `AI Co-Pilot` prompt dispatch, and `CloudStorage`.
//! 4. Companion bot menu button (`HistoryViewBotMenuButtonViewControls` + `CanonicalComposer` +
//!    `AppShell::sync_composer_and_inspector_miniapp_state` / `launch_miniapp_from_composer`) and
//!    attach-menu peer-type filtering (`USER`, `BOT`, `GROUP`, `BROADCAST`).

use futures::FutureExt;
use gpui::{px, App, AppContext, Application, AsyncApp, Entity, Hsla, Size};
use gpui_telegram_client::net::{HermesRichChatDto, PeerBotActionResult};
use gpui_telegram_client::{AccountSessionRecord, ClientConfig, TelegramClient};
use gpui_telegram_desktop::components::bots::bot_attach_web_view::{
    peer_matches_types, peer_types, peer_types_mask_from_strings, AttachBotEntry,
    BotAttachWebView, BotWebViewInputTarget, BotWebViewPendingAction,
};
use gpui_telegram_desktop::components::ui::chat::attach::attach_bot_webview::AttachBotWebviewChatAttach;
use gpui_telegram_desktop::theme::{TelegramAssetSource, TdesktopThemePalette};
use gpui_telegram_desktop::views::boxes::peers::{
    BotProfileBoxAction, CreateManagedBotBoxPeers,
};
use gpui_telegram_desktop::views::canonical_composer::BotSlashCommandItem;
use gpui_telegram_desktop::views::chat_sidebar::{
    SidebarCatalogAppItem, TELEGRAM_SEARCH_CATEGORIES,
};
use gpui_telegram_desktop::views::composer::attachments_drawer::ComposerAttachMiniApp;
use gpui_telegram_desktop::views::composer::popups_ui::{
    render_attachments_drawer_with_apps, ComposerPopupAction,
};
use gpui_telegram_desktop::views::drawer::bot_computer_inspector::{
    format_bot_mau_subtitle, render_bot_profile_main,
};
use gpui_telegram_desktop::views::drawer::inspector_tabs::{InspectorAction, InspectorTab};
use gpui_telegram_desktop::views::sidebar::dialogs_row_view::render_dialog_row_item;
use gpui_telegram_desktop::views::sidebar::sidebar_search_view::{
    render_apps_catalog_section, render_apps_catalog_section_with_recent,
};
use gpui_telegram_desktop::views::stage::cards::{
    DeliveryStatus, InlineButtonType, InlineKeyboardButton, MessageContent, MessageDirection,
    MessageItemModel, ServiceType,
};
use gpui_telegram_desktop::views::stage::cards_interactive::InteractiveCardAction;
use gpui_telegram_desktop::views::stage::stage_bubble::BubbleUiAction;
use gpui_telegram_desktop::views::stage::view::controls::HistoryViewBotMenuButtonViewControls;
use gpui_telegram_desktop::views::{
    AppShell, CanonicalComposer, ChatSidebar, ConversationStage, MainMenuDrawer,
};
use gpui_telegram_proto::miniapp_contracts::{
    AllowSendMessageResponse, BotCommandSpec, BotPublicProfileDto, BotRegistryEntry,
    CloudStorageItem, FabushiBotSummaryDto, MiniAppActionButton, MiniAppActionRequest,
    MiniAppActionResponse, MiniAppEntry, MiniAppInitDataDto, MiniAppLaunchContext,
    MiniAppLaunchMode, MiniAppThemeParams, MiniAppUiSchema, MiniAppUiWidget, MiniAppWidgetType,
    BOTFATHER_USER_ID,
};
use serde_json::json;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const CHILD_ENV: &str = "GPUI_MINIAPP_HEADLESS_CHILD";
const CHILD_TARGET_ENV: &str = "GPUI_MINIAPP_HEADLESS_TARGET";
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(1);

struct TempConfigDirGuard {
    path: PathBuf,
}

impl TempConfigDirGuard {
    fn new(prefix: &str) -> Self {
        let seq = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "gpui_tg_desktop_miniapp_{prefix}_{}_{}",
            std::process::id(),
            seq
        ));
        std::fs::create_dir_all(&dir).expect("create isolated temp config dir");
        Self { path: dir }
    }
}

impl Drop for TempConfigDirGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

fn panic_payload_to_string(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "panic".to_string()
    }
}

fn dispatch_headless_target(target: &str) -> ! {
    enum Runner {
        Sync(fn(&mut App)),
        Async(fn(AsyncApp) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + 'static>>),
    }

    let runner = match target {
        "test_headless_gpui_mount_all_miniapp_botfather_components_and_navigation" => {
            Runner::Sync(headless_body_mount_all_miniapp_botfather_components_and_navigation)
        }
        "test_headless_gpui_stage_webapp_launch_no_clipboard_and_webview_lifecycle" => {
            Runner::Sync(headless_body_stage_webapp_launch_no_clipboard_and_webview_lifecycle)
        }
        "test_headless_gpui_companion_bot_menu_button_and_attach_peer_filtering" => {
            Runner::Sync(headless_body_companion_bot_menu_button_and_attach_peer_filtering)
        }
        "test_headless_gpui_bot_context_inspector_no_members_tab_and_bot_profile" => {
            Runner::Sync(headless_body_bot_context_inspector_no_members_tab_and_bot_profile)
        }
        "test_headless_gpui_composer_per_bot_slash_commands_isolation" => {
            Runner::Sync(headless_body_composer_per_bot_slash_commands_isolation)
        }
        "test_headless_gpui_apps_catalog_recent_popular_bot_profile_box_and_prestart_chat" => {
            Runner::Async(|app| {
                Box::pin(
                    headless_body_apps_catalog_recent_popular_bot_profile_box_and_prestart_chat(
                        app,
                    ),
                )
            })
        }
        "test_headless_gpui_tos_confirm_modal_write_access_popup_and_dialog_open_pill" => {
            Runner::Async(|app| {
                Box::pin(
                    headless_body_tos_confirm_modal_write_access_popup_and_dialog_open_pill(app),
                )
            })
        }
        "test_headless_gpui_altery_inline_webapp_custom_url_and_live_bot_pin_banner" => {
            Runner::Async(|app| {
                Box::pin(
                    headless_body_altery_inline_webapp_custom_url_and_live_bot_pin_banner(app),
                )
            })
        }
        other => {
            eprintln!("unknown headless gpui test target: {other}");
            std::process::exit(101);
        }
    };
    let target_owned = target.to_string();
    let outer = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Application::headless()
            .with_assets(TelegramAssetSource::new())
            .run(move |cx: &mut App| match runner {
                Runner::Sync(body) => {
                    let inner = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        body(cx);
                    }));
                    match inner {
                        Ok(()) => std::process::exit(0),
                        Err(payload) => {
                            let msg = panic_payload_to_string(payload);
                            eprintln!("[headless child panic] {target_owned}: {msg}");
                            std::process::exit(101);
                        }
                    }
                }
                Runner::Async(async_fn) => {
                    let t = target_owned.clone();
                    cx.spawn(async move |app: &mut AsyncApp| {
                        let app_clone = app.clone();
                        let res = std::panic::AssertUnwindSafe(async_fn(app_clone))
                            .catch_unwind()
                            .await;
                        match res {
                            Ok(()) => std::process::exit(0),
                            Err(payload) => {
                                let msg = panic_payload_to_string(payload);
                                eprintln!("[headless async child panic] {t}: {msg}");
                                std::process::exit(101);
                            }
                        }
                    })
                    .detach();
                }
            });
    }));
    if let Err(payload) = outer {
        let msg = panic_payload_to_string(payload);
        eprintln!("[headless outer panic] {target}: {msg}");
        std::process::exit(101);
    }
    std::process::exit(0);
}

#[cfg(target_os = "macos")]
#[used]
#[unsafe(link_section = "__DATA,__mod_init_func")]
static MACOS_HEADLESS_MAIN_THREAD_ENTRY: extern "C" fn() = macos_mod_init_headless_entry;

#[cfg(target_os = "macos")]
extern "C" fn macos_mod_init_headless_entry() {
    if std::env::var(CHILD_ENV).as_deref() == Ok("1") {
        if let Ok(target) = std::env::var(CHILD_TARGET_ENV) {
            dispatch_headless_target(&target);
        }
    }
}

fn run_in_headless_subprocess(test_fn_name: &str) {
    if std::env::var(CHILD_ENV).as_deref() == Ok("1") {
        dispatch_headless_target(test_fn_name);
    }

    let cfg_guard = TempConfigDirGuard::new(test_fn_name);
    let exe = std::env::current_exe().expect("current_exe");
    let output = std::process::Command::new(exe)
        .env(CHILD_ENV, "1")
        .env(CHILD_TARGET_ENV, test_fn_name)
        .env("GPUI_TELEGRAM_CONFIG_DIR", &cfg_guard.path)
        .env("GPUI_OFFLINE_TEST", "1")
        .arg("--exact")
        .arg(test_fn_name)
        .arg("--nocapture")
        .output()
        .expect("spawn headless gpui child");

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "headless child test `{test_fn_name}` failed (status {:?})\n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}",
            output.status.code()
        );
    }
}

fn build_sample_bot() -> BotRegistryEntry {
    BotRegistryEntry {
        bot_user_id: 600_005,
        owner_user_id: 100_001,
        username: "CoffeeOrderBot".to_string(),
        display_name: "Coffee Order AI".to_string(),
        description: "AI Coffee assistant and Mini App".to_string(),
        about: "Order espresso cleanly".to_string(),
        ai_persona: "Friendly barista AI".to_string(),
        menu_button_text: "点单小程序".to_string(),
        menu_button_url: "gpui-miniapp://CoffeeOrderBot/order".to_string(),
        is_official: false,
        is_inline: true,
        allow_attach_menu: true,
        attach_peer_types: vec!["users".into(), "bots".into(), "groups".into()],
        commands: vec![],
        created_at: 1_700_000_000_000,
        ..Default::default()
    }
}

fn build_sample_app(installed: bool, companion_chat_id: Option<i64>) -> MiniAppEntry {
    MiniAppEntry {
        app_id: "coffee_order_app".to_string(),
        bot_user_id: 600_005,
        bot_username: "CoffeeOrderBot".to_string(),
        bot_display_name: "Coffee Order AI".to_string(),
        short_name: "order".to_string(),
        title: "精品咖啡点单".to_string(),
        description: "每日手冲与拿铁点单".to_string(),
        category: "shopping".to_string(),
        icon_emoji: "☕".to_string(),
        web_url: "gpui-miniapp://CoffeeOrderBot/order".to_string(),
        ui_schema: MiniAppUiSchema {
            title: "精品咖啡点单".to_string(),
            subtitle: "AI 点单工作台".to_string(),
            theme: MiniAppThemeParams::default(),
            widgets: vec![
                MiniAppUiWidget {
                    id: "header".to_string(),
                    kind: MiniAppWidgetType::Header,
                    label: "今日现磨推荐".to_string(),
                    ..Default::default()
                },
                MiniAppUiWidget {
                    id: "drink_note".to_string(),
                    kind: MiniAppWidgetType::TextInput,
                    label: "备注".to_string(),
                    value: String::new(),
                    placeholder: Some("冰美式少冰".to_string()),
                    ..Default::default()
                },
            ],
            main_button: Some(MiniAppActionButton {
                text: "提交咖啡订单".to_string(),
                is_visible: true,
                is_active: true,
                sends_data: true,
                action_id: "submit_coffee".to_string(),
                ..Default::default()
            }),
            secondary_button: Some(MiniAppActionButton {
                text: "清空".to_string(),
                is_visible: true,
                is_active: true,
                action_id: "reset_coffee".to_string(),
                ..Default::default()
            }),
            back_button_visible: true,
            settings_button_visible: true,
            copilot_suggestions: vec!["推荐低因饮品".to_string()],
        },
        peer_types: vec!["users".into(), "bots".into(), "groups".into()],
        install_count: 12,
        is_installed: installed,
        has_write_access: installed,
        direct_chat_id: companion_chat_id,
        created_by_nl: false,
        updated_at: 1_700_000_000_000,
        ..Default::default()
    }
}

fn spawn_local_miniapp_http_server(
    client: &Arc<TelegramClient>,
    companion_cid: i64,
) -> (String, Arc<Mutex<Vec<(String, String, String)>>>) {
    let recorded: Arc<Mutex<Vec<(String, String, String)>>> = Arc::new(Mutex::new(Vec::new()));
    let recorded_srv = recorded.clone();
    let base_url = client.runtime().block_on(async move {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind local test http listener");
        let addr = listener.local_addr().expect("local_addr");
        tokio::spawn(async move {
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    break;
                };
                let rec = recorded_srv.clone();
                tokio::spawn(async move {
                    let mut buf = vec![0u8; 32 * 1024];
                    let Ok(n) = stream.read(&mut buf).await else {
                        return;
                    };
                    if n == 0 {
                        return;
                    }
                    let req_str = String::from_utf8_lossy(&buf[..n]);
                    let mut lines = req_str.split("\r\n");
                    let req_line = lines.next().unwrap_or("");
                    let mut parts = req_line.split_whitespace();
                    let method = parts.next().unwrap_or("GET").to_string();
                    let path = parts.next().unwrap_or("/").to_string();
                    let body = req_str.split("\r\n\r\n").nth(1).unwrap_or("").to_string();
                    rec.lock().unwrap().push((method.clone(), path.clone(), body.clone()));

                    let json_body = if path.ends_with("/launch") {
                        let ctx = MiniAppLaunchContext {
                            app: build_sample_app(true, Some(companion_cid)),
                            companion_bot: build_sample_bot(),
                            companion_chat_id: companion_cid,
                            source_chat_id: Some(companion_cid),
                            launch_mode: MiniAppLaunchMode::InlineKeyboard,
                            start_param: None,
                            init_data: MiniAppInitDataDto {
                                query_id: "qid_desktop_1".to_string(),
                                user_json: "{\"id\":100001,\"username\":\"alice\"}".to_string(),
                                auth_date: 1_700_000_000,
                                hash: "a".repeat(64),
                                start_param: None,
                                chat_type: Some("sender".to_string()),
                                chat_instance: Some("100001".to_string()),
                                raw_query_string: "query_id=qid_desktop_1&auth_date=1700000000&hash=aaa".to_string(),
                                hmac_verified: true,
                            },
                            cloud_storage: vec![CloudStorageItem {
                                key: "fav_drink".to_string(),
                                value: "oat_latte".to_string(),
                                updated_at: 1_700_000_000_000,
                            }],
                            state_json: json!({ "order_count": 1 }),
                        };
                        serde_json::to_string(&ctx).unwrap()
                    } else if path.ends_with("/action") {
                        let parsed: MiniAppActionRequest =
                            serde_json::from_str(&body).unwrap_or_default();
                        let mut schema = build_sample_app(true, Some(companion_cid)).ui_schema;
                        schema.subtitle = format!("Action: {}", parsed.action_type);
                        let resp = MiniAppActionResponse {
                            ok: true,
                            ui_schema: schema,
                            state_json: json!({ "last_action": parsed.action_type }),
                            companion_bot_message: Some("咖啡订单已送达伴侣机器人！".to_string()),
                            companion_chat_id: Some(companion_cid),
                            toast: Some(format!("Executed {}", parsed.action_type)),
                            close_app: false,
                            ai_reply: Some("AI Co-Pilot: 建议选择低因燕麦拿铁。".to_string()),
                        };
                        serde_json::to_string(&resp).unwrap()
                    } else if path.contains("/storage") && method == "PUT" {
                        let v: serde_json::Value =
                            serde_json::from_str(&body).unwrap_or_else(|_| json!({}));
                        let item = CloudStorageItem {
                            key: v["key"].as_str().unwrap_or("k").to_string(),
                            value: v["value"].as_str().unwrap_or("v").to_string(),
                            updated_at: 1_700_000_001_000,
                        };
                        serde_json::to_string(&item).unwrap()
                    } else if path.contains("/storage") {
                        serde_json::to_string(&json!({
                            "items": [{
                                "key": "fav_drink",
                                "value": "oat_latte",
                                "updated_at": 1_700_000_000_000i64
                            }]
                        }))
                        .unwrap()
                    } else if path.ends_with("/install") {
                        let res = PeerBotActionResult {
                            ok: true,
                            app: Some(build_sample_app(true, Some(companion_cid))),
                            companion_bot: Some(build_sample_bot()),
                            companion_chat_id: Some(companion_cid),
                            system_message: Some("Companion bot attached".to_string()),
                            reply_message_id: None,
                            toast: Some("Installed Coffee App".to_string()),
                            updated_text: None,
                            inline_keyboard: None,
                        };
                        serde_json::to_string(&res).unwrap()
                    } else if path.contains("/api/v1/miniapps") {
                        serde_json::to_string(&json!({
                            "apps": [build_sample_app(true, Some(companion_cid))]
                        }))
                        .unwrap()
                    } else if path.contains("/callback") {
                        let res = PeerBotActionResult {
                            ok: true,
                            app: None,
                            companion_bot: None,
                            companion_chat_id: None,
                            system_message: None,
                            reply_message_id: Some(100),
                            toast: Some("Callback ok".to_string()),
                            updated_text: None,
                            inline_keyboard: None,
                        };
                        serde_json::to_string(&res).unwrap()
                    } else {
                        serde_json::to_string(&json!({ "ok": true, "chats": [] })).unwrap()
                    };

                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        json_body.len(),
                        json_body
                    );
                    let _ = stream.write_all(resp.as_bytes()).await;
                });
            }
        });
        format!("http://{}", addr)
    });
    (base_url, recorded)
}

fn new_isolated_client(base_url: String) -> Arc<TelegramClient> {
    let cfg = ClientConfig {
        server_url: base_url,
        http_timeout_secs: 5,
        ws_ping_interval_secs: 25,
        max_reconnect_attempts: 1,
        ..ClientConfig::default()
    };
    let client = Arc::new(TelegramClient::new(cfg));
    client.set_token(Some("tok_headless_desktop_test".to_string()));
    client
}

#[test]
fn test_headless_gpui_mount_all_miniapp_botfather_components_and_navigation() {
    run_in_headless_subprocess(
        "test_headless_gpui_mount_all_miniapp_botfather_components_and_navigation",
    );
}

fn headless_body_mount_all_miniapp_botfather_components_and_navigation(cx: &mut App) {
            let dummy_client = new_isolated_client("http://127.0.0.1:9".to_string());
            let _rt_guard = dummy_client.runtime().enter();

            // 1. Mount AttachBotWebviewChatAttach
            let attach_webview: Entity<AttachBotWebviewChatAttach> = cx.new(|_| {
                AttachBotWebviewChatAttach::new("CoffeeOrderBot", "gpui-miniapp://CoffeeOrderBot/order")
                    .with_fullscreen(true)
            });
            attach_webview.update(cx, |v, _| {
                assert_eq!(v.bot_name, "CoffeeOrderBot");
                assert!(v.is_fullscreen);
                v.is_expanded = false;
                v.header_color = Hsla::default();
            });
            assert!(!attach_webview.read(cx).is_expanded);

            // 2. Mount standalone HistoryViewBotMenuButtonViewControls & CreateManagedBotBoxPeers
            let menu_btn: Entity<HistoryViewBotMenuButtonViewControls> =
                cx.new(|_| HistoryViewBotMenuButtonViewControls::new());
            menu_btn.update(cx, |ctrl, _| {
                ctrl.set_bot_menu(
                    600_005,
                    "CoffeeOrderBot",
                    "点单小程序",
                    "gpui-miniapp://CoffeeOrderBot/order",
                );
                assert!(ctrl.is_active);
                assert!(ctrl.width_px >= 72.0);
                ctrl.pending_launch = Some((
                    600_005,
                    "CoffeeOrderBot".into(),
                    "order".into(),
                    "gpui-miniapp://CoffeeOrderBot/order".into(),
                ));
            });
            assert_eq!(
                menu_btn.update(cx, |c, _| c.take_launch()),
                Some((
                    600_005,
                    "CoffeeOrderBot".into(),
                    "order".into(),
                    "gpui-miniapp://CoffeeOrderBot/order".into(),
                ))
            );

            let bot_box: Entity<CreateManagedBotBoxPeers> =
                cx.new(|_| CreateManagedBotBoxPeers::new());
            bot_box.update(cx, |b, _| {
                b.open_ai_creator();
                assert!(b.is_ai_mode);
                b.push_text("帮我创建一个咖啡点单机器人和点单小程序");
                b.submit();
                assert_eq!(
                    b.take_submit_nl().as_deref(),
                    Some("帮我创建一个咖啡点单机器人和点单小程序")
                );
                b.is_ai_mode = false;
                b.set_credentials("Coffee Helper", "CoffeeOrderBot", "600005:AAFakeSecret");
                assert!(b.is_valid());
                b.submit();
                let creds = b.take_submit_credentials().expect("manual credentials");
                assert_eq!(creds.1, "CoffeeOrderBot");
            });

            // 3. Mount full AppShell and verify MainMenuDrawer & SidebarSearchView ("Apps" empty query)
            let shell: Entity<AppShell> = cx.new(|cx| {
                let mut s = AppShell::new_with_client(dummy_client.clone(), cx);
                s.is_authenticated = true;
                s.session_sync.user_id = Some(100_001);
                s.session_sync.session_token = Some("tok_headless_desktop_test".to_string());
                s.session_sync.rich_chats_by_id.insert(
                    8001,
                    HermesRichChatDto {
                        chat_id: 8001,
                        title: "BotFather".to_string(),
                        is_bot: true,
                        peer_user_id: Some(BOTFATHER_USER_ID as u64),
                        peer_username: Some("BotFather".to_string()),
                        ..Default::default()
                    },
                );
                s.miniapps.catalog_miniapps = vec![build_sample_app(true, Some(9001))];
                s.miniapps.installed_miniapps = vec![build_sample_app(true, Some(9001))];
                s
            });

            // Trigger MainMenuDrawer BotFather entry -> AppShell observer routes to BotFather DM!
            let drawer = shell.read(cx).main_menu_drawer.clone();
            drawer.update(cx, |d, d_cx| {
                d.accounts = vec![AccountSessionRecord {
                    account_id: 1,
                    display_name: "Alice".to_string(),
                    username: "alice".to_string(),
                    phone: "+100001".to_string(),
                    bio: String::new(),
                    emoji_status: String::new(),
                    is_premium: false,
                    is_verified: false,
                    avatar_initials: "AL".to_string(),
                    avatar_color: 0x2AABEE,
                    unread_chats: 0,
                }];
                d.open();
                d.pending_botfather_open = true;
                d_cx.notify();
            });
            assert_eq!(
                shell.read(cx).stage.read(cx).active_chat_id,
                Some(8001),
                "MainMenuDrawer BotFather entry must select @BotFather DM chat"
            );
            assert!(!drawer.read(cx).is_open, "drawer must close after opening BotFather");

            // Trigger MainMenuDrawer MiniApps Center -> switches Sidebar to Apps tab with empty query!
            drawer.update(cx, |d, d_cx| {
                d.open();
                d.pending_miniapps_center = true;
                d_cx.notify();
            });
            let sidebar: Entity<ChatSidebar> = shell.read(cx).sidebar.clone();
            assert!(sidebar.read(cx).search_focused);
            assert_eq!(sidebar.read(cx).active_search_category, "Apps");
            assert_eq!(sidebar.read(cx).search_query, "");
            assert!(
                TELEGRAM_SEARCH_CATEGORIES
                    .iter()
                    .any(|(id, _)| *id == "Apps"),
                "Apps search tab must exist"
            );

            // Populate Sidebar Apps catalog under EMPTY query and verify render_apps_catalog_section
            let catalog_item = SidebarCatalogAppItem {
                bot_id: 600_005,
                bot_username: "CoffeeOrderBot".to_string(),
                title: "精品咖啡点单".to_string(),
                short_name: "order".to_string(),
                description: "每日手冲与拿铁点单".to_string(),
                category: "shopping".to_string(),
                icon_initials: "☕".to_string(),
                avatar_color: 0x2481cc,
                installs_count: 12,
                is_installed: true,
                is_mine: false,
                webview_url: "gpui-miniapp://CoffeeOrderBot/order".to_string(),
                companion_chat_id: Some(9001),
                mau_count: 12,
                is_verified: false,
            };
            sidebar.update(cx, |sb, sb_cx| {
                sb.set_catalog_mini_apps(vec![catalog_item.clone()]);
                let palette = TdesktopThemePalette::default();
                let elems = render_apps_catalog_section(
                    "",
                    &sb.catalog_mini_apps,
                    &palette,
                    "zh-CN",
                    sb_cx,
                    |_, _| {},
                    |_, _, _| {},
                    |_, _, _| {},
                );
                assert!(
                    elems.len() >= 2,
                    "Apps tab with empty query must render BotFather creator card + catalog apps, got {}",
                    elems.len()
                );
                // Clicking open on an installed catalog item triggers AppShell observer -> launches overlay
                sb.pending_open_miniapp_only = Some(catalog_item);
                sb_cx.notify();
            });

            assert!(
                shell.read(cx).miniapps.miniapp_overlay.is_some(),
                "clicking catalog app in SidebarSearchView must open BotAttachWebView overlay"
            );
            assert_eq!(
                shell
                    .read(cx)
                    .miniapps
                    .miniapp_overlay
                    .as_ref()
                    .unwrap()
                    .read(cx)
                    .bot_username,
                "CoffeeOrderBot"
            );
}

#[test]
fn test_headless_gpui_stage_webapp_launch_no_clipboard_and_webview_lifecycle() {
    run_in_headless_subprocess(
        "test_headless_gpui_stage_webapp_launch_no_clipboard_and_webview_lifecycle",
    );
}

fn headless_body_stage_webapp_launch_no_clipboard_and_webview_lifecycle(cx: &mut App) {
            let companion_cid = 9001i64;
            let _server_client = new_isolated_client("http://127.0.0.1:9".to_string());
            let (base_url, recorded_http) = spawn_local_miniapp_http_server(&_server_client, companion_cid);

            let client = new_isolated_client(base_url);
            let _rt_guard = client.runtime().enter();

            let shell: Entity<AppShell> = cx.new(|cx| {
                let mut s = AppShell::new_with_client(client.clone(), cx);
                s.is_authenticated = true;
                s.session_sync.user_id = Some(100_001);
                s.session_sync.session_token = Some("tok_headless_desktop_test".to_string());
                s.miniapps.catalog_miniapps = vec![build_sample_app(false, Some(companion_cid))];
                s.miniapps.installed_miniapps = vec![build_sample_app(true, Some(companion_cid))];
                s.session_sync.rich_chats_by_id.insert(
                    companion_cid as u64,
                    HermesRichChatDto {
                        chat_id: companion_cid,
                        title: "Coffee Order AI".to_string(),
                        is_bot: true,
                        peer_user_id: Some(600_005),
                        peer_username: Some("CoffeeOrderBot".to_string()),
                        menu_button_text: Some("点单小程序".to_string()),
                        menu_button_url: Some("gpui-miniapp://CoffeeOrderBot/order".to_string()),
                        bot_short_name: Some("order".to_string()),
                        has_mini_app: true,
                        ..Default::default()
                    },
                );
                s
            });

            // Mount messages into ConversationStage with both Callback and WebApp buttons
            let stage: Entity<ConversationStage> = shell.read(cx).stage.clone();
            stage.update(cx, |st, st_cx| {
                st.set_chat(companion_cid as u64, "Coffee Order AI", "Bot", "CO", 0x2481cc);
                st.add_message(MessageItemModel {
                    id: 501,
                    sender_id: 600_005,
                    sender_name: "Coffee Order AI".to_string(),
                    avatar_initials: "CO".to_string(),
                    avatar_color: 0x2481cc,
                    direction: MessageDirection::Incoming,
                    timestamp: "10:00".to_string(),
                    delivery_status: DeliveryStatus::Read,
                    reply_to: None,
                    reactions: vec![],
                    content: MessageContent::Text {
                        text: "点此打开精品咖啡小程序".to_string(),
                        is_edited: false,
                    },
                    bot_keyboard: Some(vec![vec![
                        InlineKeyboardButton {
                            text: "🚀 打开小程序".to_string(),
                            button_type: InlineButtonType::WebApp(
                                "gpui-miniapp://CoffeeOrderBot/order".to_string(),
                            ),
                        },
                        InlineKeyboardButton {
                            text: "⚙️ BotFather 回调".to_string(),
                            button_type: InlineButtonType::Callback(
                                "bf:manage_bot:CoffeeOrderBot".to_string(),
                            ),
                        },
                    ]]),
                    is_pinned: false,
                });
                st_cx.notify();
            });

            // Capture clipboard before clicking any stage buttons (avoid altering user clipboard)
            let clip_before = cx.read_from_clipboard().and_then(|c| c.text());

            // 1. Trigger Callback button in ConversationStage -> MUST NOT open miniapp_overlay or touch clipboard
            stage.update(cx, |st, st_cx| {
                st.handle_bubble_action(
                    BubbleUiAction::Interactive(InteractiveCardAction::BotButton(
                        501,
                        InlineButtonType::Callback("bf:manage_bot:CoffeeOrderBot".to_string()),
                    )),
                    st_cx,
                );
            });
            assert!(
                shell.read(cx).miniapps.miniapp_overlay.is_none(),
                "Callback inline button must NOT open BotAttachWebView overlay"
            );
            assert_eq!(
                cx.read_from_clipboard().and_then(|c| c.text()),
                clip_before,
                "Callback inline button must not alter clipboard"
            );

            // 2. Trigger WebApp button in ConversationStage -> shows ToS confirmation first when unaccepted,
            //    NEVER copies URL, and confirming launches BotAttachWebView!
            stage.update(cx, |st, st_cx| {
                st.handle_bubble_action(
                    BubbleUiAction::Interactive(InteractiveCardAction::BotButton(
                        501,
                        InlineButtonType::WebApp(
                            "gpui-miniapp://CoffeeOrderBot/order".to_string(),
                        ),
                    )),
                    st_cx,
                );
            });
            assert!(
                shell.read(cx).miniapps.pending_miniapp_confirm.is_some(),
                "First-time WebApp launch from chat must open ToS confirmation modal"
            );
            shell.update(cx, |s, cx| {
                s.confirm_pending_miniapp_launch(cx);
            });
            let overlay = shell
                .read(cx)
                .miniapps
                .miniapp_overlay
                .clone()
                .expect("WebApp button MUST mount BotAttachWebView overlay in AppShell");
            assert!(overlay.read(cx).is_open, "BotAttachWebView must be open");
            assert_eq!(overlay.read(cx).bot_username, "CoffeeOrderBot");
            assert!(!shell.read(cx).miniapps.is_miniapp_minimized);
            assert_eq!(
                cx.read_from_clipboard().and_then(|c| c.text()),
                clip_before,
                "Clicking Mini App WebApp button must NEVER copy gpui-miniapp:// URL to clipboard"
            );
            assert_ne!(
                cx.read_from_clipboard().and_then(|c| c.text()).as_deref(),
                Some("gpui-miniapp://CoffeeOrderBot/order")
            );
            assert!(
                shell
                    .read(cx)
                    .window_runtime
                    .controller
                    .current_toast
                    .as_ref()
                    .is_none_or(|t| !t.text.contains("Copied WebApp URL") && !t.text.contains("已复制应用链接")),
                "WebApp button must NOT enqueue copy-URL toast"
            );

            // 3. Load real MiniAppLaunchContext from local HTTP server into BotAttachWebView
            let launch_ctx = client
                .runtime()
                .block_on(client.launch_mini_app(
                    "CoffeeOrderBot",
                    "order",
                    Some(companion_cid),
                    MiniAppLaunchMode::InlineKeyboard,
                    None,
                ))
                .expect("launch_mini_app http");

            // Check disclaimer flow on first-time launch, then accept
            let mut unverified_ctx = launch_ctx.clone();
            unverified_ctx.app.is_installed = false;
            unverified_ctx.app.app_tos_accepted = false;
            overlay.update(cx, |w, _| {
                w.load_context(unverified_ctx);
                assert!(!w.disclaimer_accepted);
                w.accept_disclaimer();
                assert!(w.disclaimer_accepted);
                w.load_context(launch_ctx);
                assert!(w.disclaimer_accepted);
                assert_eq!(w.cloud_storage.len(), 1);
                assert_eq!(w.cloud_storage[0].key, "fav_drink");
                assert!(w.ui_schema.back_button_visible);
                assert!(w.ui_schema.main_button.as_ref().is_some_and(|b| b.is_visible && b.sends_data));
            });

            // 4. Test Widget Input + MainButton (`sendData`) + BackButton + AI Co-Pilot + CloudStorage
            overlay.update(cx, |w, w_cx| {
                // Type in widget input
                w.focused_input = Some(BotWebViewInputTarget::WidgetText("drink_note".to_string()));
                assert!(w.push_text_to_focused_input("热美式"));
                assert!(w.submit_focused_input());

                // MainButton sendData
                let payload = serde_json::to_string(&w.ui_schema.widgets).unwrap();
                w.pending_actions.push_back(BotWebViewPendingAction::SendAppAction {
                    bot_username: w.bot_username.clone(),
                    short_name: "order".to_string(),
                    req: MiniAppActionRequest::send_data(payload, Some(companion_cid)),
                });

                // BackButton click
                w.pending_actions.push_back(BotWebViewPendingAction::SendAppAction {
                    bot_username: w.bot_username.clone(),
                    short_name: "order".to_string(),
                    req: MiniAppActionRequest {
                        action_type: "back_button_click".to_string(),
                        chat_id: Some(companion_cid),
                        ..Default::default()
                    },
                });

                // AI Co-Pilot prompt
                w.focused_input = Some(BotWebViewInputTarget::CopilotPrompt);
                assert!(w.push_text_to_focused_input("推荐低咖啡因"));
                w.trigger_ai_copilot();
                assert!(w.copilot_prompt.is_empty());

                // CloudStorage PUT trigger
                w.cloud_key_draft = "cup_size".to_string();
                w.cloud_value_draft = "grande".to_string();
                w.trigger_save_cloud_storage();

                let queued = w.take_pending_actions();
                assert_eq!(queued.len(), 5, "expected 5 queued actions (widget, send_data, back, ai_copilot, storage): {queued:?}");

                // Execute those queued HTTP actions against the local test server & apply responses
                for act in queued {
                    match act {
                        BotWebViewPendingAction::SendAppAction {
                            bot_username,
                            short_name,
                            req,
                        } => {
                            let resp = client
                                .runtime()
                                .block_on(client.send_mini_app_action(&bot_username, &short_name, &req))
                                .expect("send_mini_app_action http");
                            w.apply_action_response(resp);
                        }
                        BotWebViewPendingAction::CloudStorageSet {
                            bot_username,
                            short_name,
                            key,
                            value,
                        } => {
                            let item = client
                                .runtime()
                                .block_on(client.set_cloud_storage_item(
                                    &bot_username,
                                    &short_name,
                                    &key,
                                    &value,
                                ))
                                .expect("set_cloud_storage_item http");
                            w.apply_cloud_storage(vec![item]);
                        }
                        other => panic!("unexpected pending action: {other:?}"),
                    }
                }
                assert!(w.copilot_expanded);
                assert!(w
                    .copilot_last_reply
                    .as_deref()
                    .is_some_and(|r| r.contains("AI Co-Pilot")));
                assert_eq!(w.cloud_storage.len(), 1);
                assert_eq!(w.cloud_storage[0].key, "cup_size");
                assert_eq!(w.cloud_storage[0].value, "grande");

                // 5. Fullscreen toggle
                assert!(!w.is_fullscreen);
                w.set_fullscreen(true);
                assert!(w.is_fullscreen);
                w.set_fullscreen(false);
                assert!(!w.is_fullscreen);

                // 6. Minimize -> triggers AppShell observer -> is_miniapp_minimized = true
                w.is_open = false;
                w.pending_actions.push_back(BotWebViewPendingAction::Minimize);
                w_cx.notify();
            });
            assert!(
                shell.read(cx).miniapps.is_miniapp_minimized,
                "Minimize pending action must set shell.is_miniapp_minimized = true"
            );
            assert!(
                shell.read(cx).miniapps.miniapp_overlay.is_some(),
                "Minimizing must retain miniapp_overlay for Restore"
            );

            // 7. Restore -> sets is_miniapp_minimized = false and re-opens overlay
            shell.update(cx, |s, cx| {
                s.miniapps.is_miniapp_minimized = false;
                if let Some(ref ov) = s.miniapps.miniapp_overlay {
                    ov.update(cx, |w, w_cx| {
                        w.is_open = true;
                        w_cx.notify();
                    });
                }
                cx.notify();
            });
            assert!(!shell.read(cx).miniapps.is_miniapp_minimized);
            assert!(overlay.read(cx).is_open);

            // 8. Close -> tears down miniapp_overlay cleanly
            overlay.update(cx, |w, w_cx| {
                w.close();
                w.pending_actions.push_back(BotWebViewPendingAction::Close);
                w_cx.notify();
            });
            assert!(shell.read(cx).miniapps.miniapp_overlay.is_none());
            assert!(!shell.read(cx).miniapps.is_miniapp_minimized);

            // Verify real HTTP traffic hit launch, action (send_data, back_button_click, ai_copilot), and storage
            let http_log = recorded_http.lock().unwrap();
            assert!(http_log.iter().any(|(_, p, _)| p.ends_with("/launch")));
            assert!(http_log.iter().any(|(_, p, b)| p.ends_with("/action") && b.contains("send_data")));
            assert!(http_log.iter().any(|(_, p, b)| p.ends_with("/action") && b.contains("back_button_click")));
            assert!(http_log.iter().any(|(_, p, b)| p.ends_with("/action") && b.contains("ai_copilot")));
            assert!(http_log.iter().any(|(m, p, _)| m == "PUT" && p.contains("/storage")));
}

#[test]
fn test_headless_gpui_companion_bot_menu_button_and_attach_peer_filtering() {
    run_in_headless_subprocess(
        "test_headless_gpui_companion_bot_menu_button_and_attach_peer_filtering",
    );
}

fn headless_body_companion_bot_menu_button_and_attach_peer_filtering(cx: &mut App) {
            let client = new_isolated_client("http://127.0.0.1:9".to_string());
            let _rt_guard = client.runtime().enter();

            let companion_cid = 9101u64;
            let human_cid = 9102u64;
            let group_cid = 9103u64;
            let channel_cid = 9104u64;

            let mut bot_app = build_sample_app(true, Some(companion_cid as i64));
            bot_app.peer_types = vec!["bots".to_string()];

            let mut dm_group_app = build_sample_app(true, Some(companion_cid as i64));
            dm_group_app.app_id = "split_app".to_string();
            dm_group_app.bot_user_id = 777_003;
            dm_group_app.bot_username = "SplitBillBot".to_string();
            dm_group_app.short_name = "split".to_string();
            dm_group_app.title = "Group Split".to_string();
            dm_group_app.peer_types = vec!["users".to_string(), "groups".to_string()];

            let mut channel_app = build_sample_app(true, Some(companion_cid as i64));
            channel_app.app_id = "chan_app".to_string();
            channel_app.bot_user_id = 777_004;
            channel_app.bot_username = "ChannelPollBot".to_string();
            channel_app.short_name = "poll".to_string();
            channel_app.title = "Channel Poll".to_string();
            channel_app.peer_types = vec!["channels".to_string()];

            let shell: Entity<AppShell> = cx.new(|cx| {
                let mut s = AppShell::new_with_client(client.clone(), cx);
                s.is_authenticated = true;
                s.session_sync.user_id = Some(100_001);
                s.session_sync.session_token = Some("tok_headless_desktop_test".to_string());
                s.miniapps.installed_miniapps = vec![bot_app.clone(), dm_group_app.clone(), channel_app.clone()];
                s.miniapps.catalog_miniapps = s.miniapps.installed_miniapps.clone();
                s.session_sync.rich_chats_by_id.insert(
                    companion_cid,
                    HermesRichChatDto {
                        chat_id: companion_cid as i64,
                        title: "Coffee Order AI".to_string(),
                        is_bot: true,
                        peer_user_id: Some(600_005),
                        peer_username: Some("CoffeeOrderBot".to_string()),
                        menu_button_text: Some("打开咖啡小程序".to_string()),
                        menu_button_url: Some("gpui-miniapp://CoffeeOrderBot/order".to_string()),
                        bot_short_name: Some("order".to_string()),
                        has_mini_app: true,
                        ..Default::default()
                    },
                );
                s.session_sync.rich_chats_by_id.insert(
                    human_cid,
                    HermesRichChatDto {
                        chat_id: human_cid as i64,
                        title: "Bob Human".to_string(),
                        is_bot: false,
                        peer_user_id: Some(100_002),
                        peer_username: Some("bob".to_string()),
                        ..Default::default()
                    },
                );
                s.session_sync.rich_chats_by_id.insert(
                    group_cid,
                    HermesRichChatDto {
                        chat_id: group_cid as i64,
                        title: "Weekend Coffee Group".to_string(),
                        chat_type: 1,
                        is_group: true,
                        ..Default::default()
                    },
                );
                s.session_sync.rich_chats_by_id.insert(
                    channel_cid,
                    HermesRichChatDto {
                        chat_id: channel_cid as i64,
                        title: "Announcements Channel".to_string(),
                        chat_type: 2,
                        is_channel: true,
                        ..Default::default()
                    },
                );
                s
            });

            // 1. Select companion bot chat -> composer bot menu button activates + peer mask is BOT
            shell.update(cx, |s, cx| {
                s.stage.update(cx, |st, _| st.active_chat_id = Some(companion_cid));
                s.sync_composer_and_inspector_miniapp_state(cx);
            });
            let composer = shell.read(cx).composer.clone();
            {
                let comp = composer.read(cx);
                assert!(comp.bot_menu_controls.is_active);
                assert_eq!(comp.bot_menu_controls.bot_username, "CoffeeOrderBot");
                assert_eq!(comp.bot_menu_controls.text, "打开咖啡小程序");
                assert_eq!(comp.current_peer_type_mask, peer_types::BOT);
                assert_eq!(comp.attach_mini_apps.len(), 3);
            }

            // Simulate user clicking the mounted HistoryViewBotMenuButtonViewControls entity!
            let menu_ent = composer.update(cx, |c, c_cx| c.ensure_bot_menu_entity(c_cx));
            menu_ent.update(cx, |btn, btn_cx| {
                btn.pending_launch = Some((
                    600_005,
                    "CoffeeOrderBot".to_string(),
                    "order".to_string(),
                    "gpui-miniapp://CoffeeOrderBot/order".to_string(),
                ));
                btn_cx.notify();
            });
            assert!(
                shell.read(cx).miniapps.pending_miniapp_confirm.is_some(),
                "MenuButton click on unaccepted Mini App must open ToS confirm modal"
            );
            shell.update(cx, |s, cx| s.confirm_pending_miniapp_launch(cx));
            let ov = shell
                .read(cx)
                .miniapps
                .miniapp_overlay
                .clone()
                .expect("HistoryViewBotMenuButtonViewControls click + ToS confirm must launch Mini App via AppShell");
            assert!(ov.read(cx).is_open);
            assert_eq!(ov.read(cx).bot_username, "CoffeeOrderBot");

            // 2. Verify attach menu filters by peer type across Human DM, Bot DM, Group, Channel
            let filter_matching = |mask: u8, apps: &[ComposerAttachMiniApp]| -> Vec<String> {
                apps.iter()
                    .filter(|a| mask == 0 || a.peer_types_mask == 0 || peer_matches_types(mask, a.peer_types_mask))
                    .map(|a| a.bot_username.clone())
                    .collect()
            };

            // Verify peer_types_mask_from_strings & drawer helper render cleanly under each peer mask
            assert_eq!(
                peer_types_mask_from_strings(&["users".into(), "groups".into()]),
                peer_types::USER | peer_types::GROUP
            );

            // Switch to Human DM -> mask = USER -> only SplitBillBot matches, bot menu cleared
            shell.update(cx, |s, cx| {
                s.stage.update(cx, |st, _| st.active_chat_id = Some(human_cid));
                s.sync_composer_and_inspector_miniapp_state(cx);
            });
            composer.update(cx, |comp, c_cx| {
                assert!(!comp.bot_menu_controls.is_active);
                assert_eq!(comp.current_peer_type_mask, peer_types::USER);
                assert_eq!(
                    filter_matching(comp.current_peer_type_mask, &comp.attach_mini_apps),
                    vec!["SplitBillBot".to_string()]
                );
                let _el = render_attachments_drawer_with_apps(
                    &comp.attach_mini_apps,
                    comp.current_peer_type_mask,
                    &TdesktopThemePalette::default(),
                    "en",
                    c_cx,
                    |_, _act: ComposerPopupAction, _| {},
                );
            });

            // Switch to Group -> mask = GROUP -> only SplitBillBot matches
            shell.update(cx, |s, cx| {
                s.stage.update(cx, |st, _| st.active_chat_id = Some(group_cid));
                s.sync_composer_and_inspector_miniapp_state(cx);
            });
            assert_eq!(composer.read(cx).current_peer_type_mask, peer_types::GROUP);
            assert_eq!(
                filter_matching(
                    composer.read(cx).current_peer_type_mask,
                    &composer.read(cx).attach_mini_apps
                ),
                vec!["SplitBillBot".to_string()]
            );

            // Switch to Channel -> mask = BROADCAST -> only ChannelPollBot matches
            shell.update(cx, |s, cx| {
                s.stage.update(cx, |st, _| st.active_chat_id = Some(channel_cid));
                s.sync_composer_and_inspector_miniapp_state(cx);
            });
            assert_eq!(
                composer.read(cx).current_peer_type_mask,
                peer_types::BROADCAST
            );
            assert_eq!(
                filter_matching(
                    composer.read(cx).current_peer_type_mask,
                    &composer.read(cx).attach_mini_apps
                ),
                vec!["ChannelPollBot".to_string()]
            );
}

#[test]
fn test_unit_attach_bot_webview_peer_masks_and_botfather_entry_models() {
    let entry = AttachBotEntry {
        bot_id: 600_005,
        name: "Coffee Helper".to_string(),
        short_name: "order".to_string(),
        allowed_peer_types: peer_types::USER | peer_types::GROUP,
        in_main_menu: true,
        in_attach_menu: true,
        disclaimer_required: true,
        request_write_access: true,
    };
    let mut wv = BotAttachWebView::new();
    wv.launch(entry, "gpui-miniapp://CoffeeOrderBot/order");
    assert!(wv.is_open);
    assert!(!wv.disclaimer_accepted);
    wv.accept_disclaimer();
    assert!(wv.disclaimer_accepted);

    assert!(peer_matches_types(peer_types::USER, peer_types::USER | peer_types::GROUP));
    assert!(!peer_matches_types(peer_types::BROADCAST, peer_types::USER | peer_types::GROUP));
    assert!(peer_matches_types(
        peer_types::BOT,
        peer_types_mask_from_strings(&["bots".to_string()])
    ));

    let drawer = MainMenuDrawer::new(vec![], 1);
    assert!(!drawer.is_open);
    assert!(!drawer.pending_botfather_open);

    let comp = CanonicalComposer::new();
    assert_eq!(comp.current_peer_type_mask, 0);
    assert!(!comp.hermes_slash_enabled);
    let _ = Size {
        width: px(400.0),
        height: px(600.0),
    };
}

fn build_third_party_shop_bot() -> BotRegistryEntry {
    BotRegistryEntry {
        bot_user_id: 600_777,
        owner_user_id: 200_999,
        username: "PizzaShopBot".to_string(),
        display_name: "Pizza Shop".to_string(),
        description: "Order wood-fired pizza right inside Telegram.".to_string(),
        about: "Official Pizza Shop Mini App".to_string(),
        ai_persona: String::new(),
        menu_button_text: "Order Pizza".to_string(),
        menu_button_url: "gpui-miniapp://PizzaShopBot/shop".to_string(),
        is_official: true,
        is_inline: true,
        allow_attach_menu: false,
        attach_peer_types: vec!["users".into()],
        commands: vec![
            BotCommandSpec {
                command: "menu".to_string(),
                description: "Browse pizza menu".to_string(),
            },
            BotCommandSpec {
                command: "help".to_string(),
                description: "Order support".to_string(),
            },
        ],
        created_at: 1_700_000_000_000,
        is_fabushi_bot: false,
        menu_button_type: "web_app".to_string(),
        has_main_app: true,
        main_app_short_name: "shop".to_string(),
        privacy_policy_url: "https://pizza.example.com/privacy".to_string(),
        has_settings: true,
        can_join_groups: true,
        mau_count: 12_345,
        allows_write_to_pm: false,
        app_tos_accepted: false,
        is_blocked_by_viewer: false,
        ..Default::default()
    }
}

fn build_shop_miniapp(tos_accepted: bool, allows_write: bool, chat_id: Option<i64>) -> MiniAppEntry {
    MiniAppEntry {
        app_id: "pizza_shop_app".to_string(),
        bot_user_id: 600_777,
        bot_username: "PizzaShopBot".to_string(),
        bot_display_name: "Pizza Shop".to_string(),
        short_name: "shop".to_string(),
        title: "Pizza Mini App".to_string(),
        description: "Fast hot pizza delivery".to_string(),
        category: "shopping".to_string(),
        icon_emoji: "🍕".to_string(),
        web_url: "gpui-miniapp://PizzaShopBot/shop".to_string(),
        ui_schema: MiniAppUiSchema {
            title: "Pizza Mini App".to_string(),
            subtitle: "Fresh wood-fired pizza".to_string(),
            theme: MiniAppThemeParams::default(),
            widgets: Vec::new(),
            main_button: None,
            secondary_button: None,
            back_button_visible: false,
            settings_button_visible: false,
            copilot_suggestions: Vec::new(),
        },
        peer_types: vec!["users".into()],
        install_count: 0,
        is_installed: false,
        has_write_access: allows_write,
        direct_chat_id: chat_id,
        created_by_nl: false,
        updated_at: 1_700_000_000_000,
        mau_count: 12_345,
        is_main_app: true,
        is_recent_used: true,
        last_launched_at: 1_700_000_001_000,
        privacy_policy_url: "https://pizza.example.com/privacy".to_string(),
        app_tos_accepted: tos_accepted,
        header_color: Some("#FF5500".to_string()),
        background_color: Some("#FFFDF9".to_string()),
    }
}

fn build_shop_public_profile(
    existing_chat_id: Option<i64>,
    has_started_chat: bool,
    allows_write_to_pm: bool,
    app_tos_accepted: bool,
    is_blocked: bool,
) -> BotPublicProfileDto {
    let mut bot = build_third_party_shop_bot();
    bot.allows_write_to_pm = allows_write_to_pm;
    bot.app_tos_accepted = app_tos_accepted;
    bot.is_blocked_by_viewer = is_blocked;
    let app = build_shop_miniapp(app_tos_accepted, allows_write_to_pm, existing_chat_id);
    BotPublicProfileDto {
        bot,
        main_app: Some(app.clone()),
        apps: vec![app],
        existing_chat_id,
        has_started_chat,
        allows_write_to_pm,
        app_tos_accepted,
        is_blocked,
        is_muted: false,
    }
}

#[derive(Clone, Default)]
struct ParityMockState {
    chat_created: bool,
    chat_id: i64,
    allows_write_to_pm: bool,
    app_tos_accepted: bool,
    is_blocked: bool,
    has_started_chat: bool,
    launched_once: bool,
}

fn spawn_parity_mock_http_server(
    client: &Arc<TelegramClient>,
    initial: ParityMockState,
) -> (
    String,
    Arc<Mutex<Vec<(String, String, String)>>>,
    Arc<Mutex<ParityMockState>>,
) {
    let recorded: Arc<Mutex<Vec<(String, String, String)>>> = Arc::new(Mutex::new(Vec::new()));
    let state = Arc::new(Mutex::new(initial));
    let rec_srv = recorded.clone();
    let state_srv = state.clone();
    let base_url = client.runtime().block_on(async move {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind parity mock listener");
        let addr = listener.local_addr().expect("local_addr");
        tokio::spawn(async move {
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    break;
                };
                let rec = rec_srv.clone();
                let st_arc = state_srv.clone();
                tokio::spawn(async move {
                    let mut buf = vec![0u8; 32 * 1024];
                    let Ok(n) = stream.read(&mut buf).await else {
                        return;
                    };
                    if n == 0 {
                        return;
                    }
                    let req_str = String::from_utf8_lossy(&buf[..n]);
                    let mut lines = req_str.split("\r\n");
                    let req_line = lines.next().unwrap_or("");
                    let mut parts = req_line.split_whitespace();
                    let method = parts.next().unwrap_or("GET").to_string();
                    let path = parts.next().unwrap_or("/").to_string();
                    let body = req_str.split("\r\n\r\n").nth(1).unwrap_or("").to_string();
                    rec.lock()
                        .unwrap()
                        .push((method.clone(), path.clone(), body.clone()));

                    let json_body = {
                        let mut st = st_arc.lock().unwrap();
                        if path.starts_with("/api/v1/bots/profile/") {
                        let cid = st.chat_created.then_some(st.chat_id);
                        let prof = build_shop_public_profile(
                            cid,
                            st.has_started_chat,
                            st.allows_write_to_pm,
                            st.app_tos_accepted,
                            st.is_blocked,
                        );
                        serde_json::to_string(&prof).unwrap()
                    } else if path == "/api/v1/miniapps/recent" {
                        let apps = if st.launched_once {
                            let cid = st.chat_created.then_some(st.chat_id);
                            vec![build_shop_miniapp(
                                st.app_tos_accepted,
                                st.allows_write_to_pm,
                                cid,
                            )]
                        } else {
                            Vec::new()
                        };
                        serde_json::to_string(&json!({ "apps": apps, "total": apps.len() }))
                            .unwrap()
                    } else if path.ends_with("/accept-app-tos") {
                        st.app_tos_accepted = true;
                        serde_json::to_string(&json!({ "ok": true, "app_tos_accepted": true }))
                            .unwrap()
                    } else if path.ends_with("/allow-send-message") {
                        st.chat_created = true;
                        st.allows_write_to_pm = true;
                        st.is_blocked = false;
                        let resp = AllowSendMessageResponse {
                            ok: true,
                            chat_id: st.chat_id,
                            service_message_id: Some(701),
                            service_text: "您已允许该机器人在其网页应用中向您发送消息。"
                                .to_string(),
                        };
                        serde_json::to_string(&resp).unwrap()
                    } else if path.ends_with("/block") {
                        let v: serde_json::Value =
                            serde_json::from_str(&body).unwrap_or_else(|_| json!({}));
                        st.is_blocked = v["blocked"].as_bool().unwrap_or(false);
                        if st.is_blocked {
                            st.allows_write_to_pm = false;
                        }
                        serde_json::to_string(&json!({ "ok": true, "blocked": st.is_blocked }))
                            .unwrap()
                    } else if path == "/api/v1/contacts/add" {
                        st.chat_created = true;
                        st.has_started_chat = true;
                        serde_json::to_string(&json!({
                            "user_id": 600_777_u64,
                            "chat_id": st.chat_id,
                            "title": "Pizza Shop",
                            "username": "PizzaShopBot",
                            "is_bot": true
                        }))
                        .unwrap()
                    } else if path == "/api/v1/chats" {
                        let chats = if st.chat_created {
                            vec![json!({
                                "chat_id": st.chat_id,
                                "title": "Pizza Shop",
                                "chat_type": 4,
                                "last_message": "您已允许该机器人在其网页应用中向您发送消息。",
                                "unread_count": 0,
                                "is_group": false,
                                "is_bot": true,
                                "avatar_color": 0x2481cc,
                                "updated_at": 1_700_000_002_000_i64
                            })]
                        } else {
                            Vec::new()
                        };
                        serde_json::to_string(&json!({ "chats": chats })).unwrap()
                    } else if path.ends_with("/launch") {
                        st.launched_once = true;
                        let req_json: serde_json::Value =
                            serde_json::from_str(&body).unwrap_or_else(|_| json!({}));
                        let cid_opt = st.chat_created.then_some(st.chat_id);
                        let mut app =
                            build_shop_miniapp(st.app_tos_accepted, st.allows_write_to_pm, cid_opt);
                        app.is_installed = false;
                        if let Some(cu) = req_json
                            .get("custom_url")
                            .and_then(|v| v.as_str())
                            .filter(|s| !s.is_empty())
                        {
                            app.web_url = cu.to_string();
                        }
                        let mut bot = build_third_party_shop_bot();
                        bot.allows_write_to_pm = st.allows_write_to_pm;
                        bot.app_tos_accepted = st.app_tos_accepted;
                        let ctx = MiniAppLaunchContext {
                            app,
                            companion_bot: bot,
                            companion_chat_id: cid_opt.unwrap_or(0),
                            source_chat_id: None,
                            launch_mode: MiniAppLaunchMode::MainMiniApp,
                            start_param: None,
                            init_data: MiniAppInitDataDto {
                                query_id: "qid_parity_1".to_string(),
                                user_json: format!(
                                    "{{\"id\":100001,\"username\":\"alice\",\"allows_write_to_pm\":{}}}",
                                    st.allows_write_to_pm
                                ),
                                auth_date: 1_700_000_000,
                                hash: "b".repeat(64),
                                start_param: None,
                                chat_type: Some("sender".to_string()),
                                chat_instance: Some("app_600777".to_string()),
                                raw_query_string: "query_id=qid_parity_1&hash=bbb".to_string(),
                                hmac_verified: true,
                            },
                            cloud_storage: Vec::new(),
                            state_json: json!({}),
                        };
                        serde_json::to_string(&ctx).unwrap()
                    } else if path.contains("/messages") && method == "POST" {
                        st.has_started_chat = true;
                        serde_json::to_string(&json!({ "message_id": 702 })).unwrap()
                    } else if path.contains("/messages") {
                        let msgs = if st.chat_created {
                            vec![json!({
                                "id": 701,
                                "sender_id": 600_777,
                                "text": "您已允许该机器人在其网页应用中向您发送消息。",
                                "date_ms": 1_700_000_001_000_i64
                            })]
                        } else {
                            Vec::new()
                        };
                        serde_json::to_string(&json!({ "messages": msgs })).unwrap()
                    } else {
                        serde_json::to_string(&json!({ "ok": true, "apps": [], "bots": [], "chats": [] }))
                            .unwrap()
                    }
                    };

                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        json_body.len(),
                        json_body
                    );
                    let _ = stream.write_all(resp.as_bytes()).await;
                });
            }
        });
        format!("http://{}", addr)
    });
    (base_url, recorded, state)
}

#[test]
fn test_headless_gpui_bot_context_inspector_no_members_tab_and_bot_profile() {
    run_in_headless_subprocess(
        "test_headless_gpui_bot_context_inspector_no_members_tab_and_bot_profile",
    );
}

fn headless_body_bot_context_inspector_no_members_tab_and_bot_profile(cx: &mut App) {
    let client = new_isolated_client("http://127.0.0.1:9".to_string());
    let _rt = client.runtime().enter();

    assert_eq!(format_bot_mau_subtitle(12_345, "zh-CN"), "12,345 名每月活跃用户");
    assert_eq!(format_bot_mau_subtitle(0, "zh-CN"), "机器人");
    assert_eq!(format_bot_mau_subtitle(1_200, "en"), "1,200 monthly users");

    let bot_cid = 88_001u64;
    let bot_entry = build_third_party_shop_bot();
    let prof = build_shop_public_profile(Some(bot_cid as i64), true, false, false, false);

    let shell = cx.new(|cx| {
        let mut s = AppShell::new_with_client(client.clone(), cx);
        s.is_authenticated = true;
        s.session_sync.user_id = Some(100_001);
        s.session_sync.session_token = Some("tok_insp".to_string());
        s.miniapps
            .bot_profiles_cache
            .insert("pizzashopbot".to_string(), prof);
        s.miniapps.attach_menu_bots = vec![bot_entry];
        s.session_sync.rich_chats_by_id.insert(
            bot_cid,
            HermesRichChatDto {
                chat_id: bot_cid as i64,
                title: "Pizza Shop".to_string(),
                is_bot: true,
                peer_user_id: Some(600_777),
                peer_username: Some("PizzaShopBot".to_string()),
                peer_bio: Some("Order wood-fired pizza right inside Telegram.".to_string()),
                menu_button_text: Some("Order Pizza".to_string()),
                menu_button_url: Some("gpui-miniapp://PizzaShopBot/shop".to_string()),
                has_mini_app: true,
                ..Default::default()
            },
        );
        s
    });

    // Simulate selecting a Bot chat in Inspector even if Inspector previously sat on Members tab
    shell.update(cx, |s, cx| {
        s.inspector.update(cx, |insp, _| {
            insp.open();
            insp.set_language("zh-CN");
            insp.set_peer("Pizza Shop", "Order pizza", "PS", 0x2481cc);
            insp.set_tab(InspectorTab::Members);
            insp.sync_members(vec![], String::new(), false);
        });
        s.stage.update(cx, |st, _| st.active_chat_id = Some(bot_cid));
        s.sync_composer_and_inspector_miniapp_state(cx);
    });

    let insp_ent = shell.read(cx).inspector.clone();
    insp_ent.update(cx, |insp, icx| {
        assert!(insp.profile.is_bot);
        assert!(!insp.members.is_group_or_channel, "Bot DM must NEVER mark members.is_group_or_channel = true");
        assert!(insp.members.members.is_empty());
        assert_eq!(insp.profile.bot_mau_count, 12_345);
        assert!(insp.profile.bot_has_main_app);
        assert_eq!(
            insp.profile.bot_privacy_policy_url,
            "https://pizza.example.com/privacy"
        );
        assert!(insp.profile.bot_has_settings);
        assert!(insp.profile.bot_can_join_groups);
        assert!(!insp.profile.is_viewer_owner);
        assert!(insp.profile.is_verified);
        assert!(insp.active_fabushi_bot.is_none());

        // Render bot profile main without panicking (third-party bot: no owner Fabushi/Computer cards)
        let _bot_main = render_bot_profile_main(insp, TdesktopThemePalette::default(), "zh-CN", icx);
    });

    // Trigger InspectorAction::OpenMiniAppTos -> AppShell opens miniapp_tos_modal_open
    insp_ent.update(cx, |insp, icx| {
        insp.pending_action = Some(InspectorAction::OpenMiniAppTos);
        icx.notify();
    });
    assert!(
        shell.read(cx).miniapps.miniapp_tos_modal_open,
        "Inspector OpenMiniAppTos must open Mini App ToS modal"
    );
    shell.update(cx, |s, _| s.miniapps.miniapp_tos_modal_open = false);

    // Trigger InspectorAction::FocusBotComposer -> focuses composer
    insp_ent.update(cx, |insp, icx| {
        insp.pending_action = Some(InspectorAction::FocusBotComposer);
        icx.notify();
    });
    assert!(shell.read(cx).composer.read(cx).is_input_focused);

    // Verify owner Fabushi bot card gate (`is_viewer_owner && active_fabushi_bot.is_some()`)
    insp_ent.update(cx, |insp, icx| {
        insp.sync_bot_profile_extra(0, false, String::new(), false, false, true, false, false);
        insp.set_active_fabushi_bot(Some(FabushiBotSummaryDto {
            bot_user_id: 600_900,
            owner_user_id: 100_001,
            agent_id: "my-ops-bot".to_string(),
            username: "MyOpsBot".to_string(),
            display_name: "My Ops Bot".to_string(),
            ..Default::default()
        }));
        let _owner_main = render_bot_profile_main(insp, TdesktopThemePalette::default(), "zh-CN", icx);
    });
}

#[test]
fn test_headless_gpui_composer_per_bot_slash_commands_isolation() {
    run_in_headless_subprocess("test_headless_gpui_composer_per_bot_slash_commands_isolation");
}

fn headless_body_composer_per_bot_slash_commands_isolation(cx: &mut App) {
    let client = new_isolated_client("http://127.0.0.1:9".to_string());
    let _rt = client.runtime().enter();

    let pizza_cid = 9_201u64;
    let silent_bot_cid = 9_202u64;
    let botfather_cid = 9_203u64;
    let fabushi_cid = 9_204u64;

    let mut pizza_bot = build_third_party_shop_bot();
    pizza_bot.menu_button_type = "default".to_string();
    pizza_bot.menu_button_url.clear();
    pizza_bot.menu_button_text.clear();

    let silent_bot = BotRegistryEntry {
        bot_user_id: 600_888,
        owner_user_id: 200_999,
        username: "PlainWebhookBot".to_string(),
        display_name: "Plain Webhook Bot".to_string(),
        commands: Vec::new(),
        is_fabushi_bot: false,
        menu_button_type: "default".to_string(),
        ..Default::default()
    };

    let my_fabushi_bot = BotRegistryEntry {
        bot_user_id: 600_999,
        owner_user_id: 100_001,
        username: "MyHermesBot".to_string(),
        display_name: "My Hermes Agent".to_string(),
        commands: Vec::new(),
        is_fabushi_bot: true,
        menu_button_type: "web_app".to_string(),
        menu_button_text: "点单控制台".to_string(),
        menu_button_url: "gpui-miniapp://MyHermesBot/panel".to_string(),
        ..Default::default()
    };

    let shell = cx.new(|cx| {
        let mut s = AppShell::new_with_client(client.clone(), cx);
        s.is_authenticated = true;
        s.session_sync.user_id = Some(100_001);
        s.session_sync.session_token = Some("tok_slash".to_string());
        s.miniapps.attach_menu_bots = vec![pizza_bot.clone(), silent_bot.clone()];
        s.miniapps.my_bots = vec![my_fabushi_bot.clone()];
        s.miniapps.fabushi_bots = vec![FabushiBotSummaryDto {
            bot_user_id: 600_999,
            owner_user_id: 100_001,
            agent_id: "my-hermes-bot".to_string(),
            username: "MyHermesBot".to_string(),
            display_name: "My Hermes Agent".to_string(),
            companion_chat_id: Some(fabushi_cid as i64),
            ..Default::default()
        }];
        s.session_sync.rich_chats_by_id.insert(
            pizza_cid,
            HermesRichChatDto {
                chat_id: pizza_cid as i64,
                title: "Pizza Shop".to_string(),
                is_bot: true,
                peer_user_id: Some(600_777),
                peer_username: Some("PizzaShopBot".to_string()),
                ..Default::default()
            },
        );
        s.session_sync.rich_chats_by_id.insert(
            silent_bot_cid,
            HermesRichChatDto {
                chat_id: silent_bot_cid as i64,
                title: "Plain Webhook Bot".to_string(),
                is_bot: true,
                peer_user_id: Some(600_888),
                peer_username: Some("PlainWebhookBot".to_string()),
                ..Default::default()
            },
        );
        s.session_sync.rich_chats_by_id.insert(
            botfather_cid,
            HermesRichChatDto {
                chat_id: botfather_cid as i64,
                title: "BotFather".to_string(),
                is_bot: true,
                peer_user_id: Some(BOTFATHER_USER_ID as u64),
                peer_username: Some("BotFather".to_string()),
                ..Default::default()
            },
        );
        s.session_sync.rich_chats_by_id.insert(
            fabushi_cid,
            HermesRichChatDto {
                chat_id: fabushi_cid as i64,
                title: "My Hermes Agent".to_string(),
                is_bot: true,
                peer_user_id: Some(600_999),
                peer_username: Some("MyHermesBot".to_string()),
                menu_button_text: Some("点单控制台".to_string()),
                menu_button_url: Some("gpui-miniapp://MyHermesBot/panel".to_string()),
                has_mini_app: true,
                ..Default::default()
            },
        );
        s
    });

    let composer = shell.read(cx).composer.clone();
    assert!(!composer.read(cx).hermes_slash_enabled);

    // 1. Standard bot with 2 commands: typing `/` shows ONLY those 2 bot commands (never 55 Hermes commands!)
    //    and menu_button_type="default" leaves bot_menu_controls inactive.
    shell.update(cx, |s, cx| {
        s.stage.update(cx, |st, _| st.active_chat_id = Some(pizza_cid));
        s.sync_composer_and_inspector_miniapp_state(cx);
    });
    composer.update(cx, |c, _| {
        assert!(!c.hermes_slash_enabled);
        assert!(!c.bot_menu_controls.is_active);
        assert_eq!(c.bot_slash_commands.len(), 2);
        c.set_text("/");
        c.ime_machine.cursor_index = 1;
        c.sync_autocomplete();
        assert!(c.bot_ac_visible);
        assert_eq!(c.bot_ac_matches.len(), 2);
        let names: Vec<&str> = c.bot_ac_matches.iter().map(|m| m.command.as_str()).collect();
        assert_eq!(names, vec!["menu", "help"]);
        assert!(!c.hermes_ac_visible);
        assert!(c.hermes_ac_matches.is_empty());
    });

    // 2. Standard bot with 0 commands: typing `/` shows NOTHING (neither bot_ac nor 55 Hermes commands!)
    shell.update(cx, |s, cx| {
        s.stage
            .update(cx, |st, _| st.active_chat_id = Some(silent_bot_cid));
        s.sync_composer_and_inspector_miniapp_state(cx);
    });
    composer.update(cx, |c, _| {
        assert!(!c.hermes_slash_enabled);
        assert!(c.bot_slash_commands.is_empty());
        c.set_text("/");
        c.ime_machine.cursor_index = 1;
        c.sync_autocomplete();
        assert!(!c.bot_ac_visible);
        assert!(!c.hermes_ac_visible);
        assert!(c.hermes_ac_matches.is_empty());
    });

    // 3. @BotFather: hermes_slash_enabled is false, only BotFather commands appear on `/`
    shell.update(cx, |s, cx| {
        s.stage
            .update(cx, |st, _| st.active_chat_id = Some(botfather_cid));
        s.sync_composer_and_inspector_miniapp_state(cx);
    });
    composer.update(cx, |c, _| {
        assert!(!c.hermes_slash_enabled);
        assert!(c.bot_slash_commands.iter().any(|b| b.command == "newbot"));
        c.set_text("/");
        c.ime_machine.cursor_index = 1;
        c.sync_autocomplete();
        assert!(c.bot_ac_visible);
        assert!(!c.hermes_ac_visible);
    });

    // 4. Viewer-owned Fabushi bot: hermes_slash_enabled is true + menu_button_type="web_app" uses exact label
    shell.update(cx, |s, cx| {
        s.stage.update(cx, |st, _| st.active_chat_id = Some(fabushi_cid));
        s.sync_composer_and_inspector_miniapp_state(cx);
    });
    composer.update(cx, |c, _| {
        assert!(c.hermes_slash_enabled);
        assert!(c.bot_menu_controls.is_active);
        assert_eq!(c.bot_menu_controls.text, "点单控制台");
        c.set_text("/mem");
        c.ime_machine.cursor_index = 4;
        c.sync_autocomplete();
        assert!(c.hermes_ac_visible);
        assert!(!c.bot_ac_visible);
    });

    // Standalone unit check for BotSlashCommandItem popup rendering
    composer.update(cx, |c, _| {
        c.set_bot_slash_commands(vec![BotSlashCommandItem {
            command: "status".to_string(),
            description: "Check order status".to_string(),
            bot_username: "PizzaShopBot".to_string(),
            avatar_initials: "PS".to_string(),
            avatar_color: 0x2481cc,
        }]);
        c.set_text("/sta");
        c.ime_machine.cursor_index = 4;
        c.sync_autocomplete();
        assert!(c.bot_ac_visible);
        assert_eq!(c.bot_ac_matches.len(), 1);
    });
}

#[test]
fn test_headless_gpui_apps_catalog_recent_popular_bot_profile_box_and_prestart_chat() {
    run_in_headless_subprocess(
        "test_headless_gpui_apps_catalog_recent_popular_bot_profile_box_and_prestart_chat",
    );
}

async fn headless_body_apps_catalog_recent_popular_bot_profile_box_and_prestart_chat(
    app: AsyncApp,
) {
    let seed_client = new_isolated_client("http://127.0.0.1:9".to_string());
    let (base_url, recorded_http, _mock_state) = spawn_parity_mock_http_server(
        &seed_client,
        ParityMockState {
            chat_created: false,
            chat_id: 600_000_777,
            allows_write_to_pm: false,
            app_tos_accepted: false,
            is_blocked: false,
            has_started_chat: false,
            launched_once: true,
        },
    );
    let client = new_isolated_client(base_url);

    let shell = app
        .update(|cx| {
            cx.new(|cx| {
                let mut s = AppShell::new_with_client(client.clone(), cx);
                s.is_authenticated = true;
                s.session_sync.user_id = Some(100_001);
                s.session_sync.session_token = Some("tok_apps_catalog".to_string());
                s
            })
        })
        .expect("create shell");

    // 1. Render Sidebar `应用` catalog with `您用过的应用` & `畅销应用` (NO + 添加并打开 buttons)
    let clicked_profile_un: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let clicked_profile_un_cb = clicked_profile_un.clone();
    app.update(|cx| {
        let shop_item = SidebarCatalogAppItem {
            bot_id: 600_777,
            bot_username: "PizzaShopBot".to_string(),
            title: "Pizza Mini App".to_string(),
            short_name: "shop".to_string(),
            description: "Fast hot pizza delivery".to_string(),
            category: "shopping".to_string(),
            icon_initials: "🍕".to_string(),
            avatar_color: 0xFF5500,
            installs_count: 0,
            is_installed: false,
            is_mine: false,
            webview_url: "gpui-miniapp://PizzaShopBot/shop".to_string(),
            companion_chat_id: None,
            mau_count: 12_345,
            is_verified: true,
        };
        let sb = shell.read(cx).sidebar.clone();
        sb.update(cx, |sb, sb_cx| {
            sb.set_recent_mini_apps(vec![shop_item.clone()]);
            sb.set_catalog_mini_apps(vec![shop_item.clone()]);
            let pal = TdesktopThemePalette::default();
            let elems = render_apps_catalog_section_with_recent(
                "",
                &sb.recent_mini_apps,
                &sb.catalog_mini_apps,
                &pal,
                "zh-CN",
                sb_cx,
                |_, _| {},
                move |_, item, _| {
                    *clicked_profile_un_cb.lock().unwrap() = Some(item.bot_username);
                },
            );
            // BotFather card + "您用过的应用" header + row + "畅销应用" header + row = 5
            assert_eq!(elems.len(), 5, "expected BotFather + 2 section headers + 2 rows");

            // Clicking catalog app triggers pending_open_bot_profile (NOT install or launch!)
            sb.pending_open_bot_profile = Some("PizzaShopBot".to_string());
            sb_cx.notify();
        });
        assert!(clicked_profile_un.lock().unwrap().is_none());
    })
    .expect("update sidebar catalog");

    // Pump AsyncApp until `open_bot_profile_popup` finishes HTTP fetch and opens BotProfileBox
    for _ in 0..40 {
        app.background_executor()
            .timer(std::time::Duration::from_millis(20))
            .await;
        let ready = app
            .update(|cx| {
                shell
                    .read(cx)
                    .miniapps
                    .bot_profile_box
                    .as_ref()
                    .is_some_and(|b| b.read(cx).is_visible)
            })
            .unwrap();
        if ready {
            break;
        }
    }

    // Verify BotProfileBox opened WITHOUT creating any chat or calling install/contacts/add!
    app.update(|cx| {
        let s = shell.read(cx);
        let bx = s
            .miniapps
            .bot_profile_box
            .as_ref()
            .expect("BotProfileBox must be open after clicking catalog app");
        assert!(bx.read(cx).is_visible);
        assert_eq!(bx.read(cx).profile.bot.username, "PizzaShopBot");
        assert_eq!(bx.read(cx).profile.bot.mau_count, 12_345);
        assert_eq!(bx.read(cx).profile.existing_chat_id, None);
        assert!(s.sidebar.read(cx).dialogs.is_empty(), "NO DM chat may be created");
        assert!(s.session_sync.rich_chats_by_id.is_empty());

        let log = recorded_http.lock().unwrap();
        assert!(
            log.iter()
                .any(|(m, p, _)| m == "GET" && p == "/api/v1/bots/profile/PizzaShopBot"),
            "expected GET /api/v1/bots/profile/PizzaShopBot, got: {log:?}"
        );
        assert!(
            !log.iter()
                .any(|(_, p, _)| p.contains("/contacts/add") || p.contains("/install")),
            "opening BotProfileBox from Apps catalog must NEVER call /contacts/add or /install: {log:?}"
        );
    })
    .expect("assert BotProfileBox");

    // 2. Click `消息` inside BotProfileBox -> opens virtual pre-start conversation on ConversationStage
    //    with `它能做什么？` card & `开始` bar (still NO DB chat created!)
    app.update(|cx| {
        let bx = shell
            .read(cx)
            .miniapps
            .bot_profile_box
            .clone()
            .expect("bot_profile_box");
        bx.update(cx, |b, b_cx| {
            b.emit(BotProfileBoxAction::OpenMessageChat {
                bot_id: 600_777,
                bot_username: "PizzaShopBot".to_string(),
                display_name: "Pizza Shop".to_string(),
                description: "Order wood-fired pizza right inside Telegram.".to_string(),
                existing_chat_id: None,
                has_started_chat: false,
            });
            b_cx.notify();
        });
    })
    .expect("emit BotProfileBox OpenMessageChat");

    app.update(|cx| {
        let s = shell.read(cx);
        assert!(s.miniapps.bot_profile_box.is_none());
        assert!(s.miniapps.pending_prestart_bot.is_some());
        let st = s.stage.read(cx);
        assert_eq!(st.active_chat_id, None, "pre-start bot stage has no DB chat yet");
        assert_eq!(
            st.bot_description_card.as_deref(),
            Some("Order wood-fired pizza right inside Telegram.")
        );
        assert!(st.show_bot_start_bar);
        assert!(!st.is_viewer_owner_fabushi_bot);
        assert!(s.sidebar.read(cx).dialogs.is_empty());
    })
    .expect("open prestart bot stage");

    // 3. Click `开始` (`stage.pending_bot_start = true`) -> materializes chat via /contacts/add
    app.update(|cx| {
        let stage = shell.read(cx).stage.clone();
        stage.update(cx, |st, st_cx| {
            st.pending_bot_start = true;
            st_cx.notify();
        });
    })
    .expect("click start bar");

    for _ in 0..40 {
        app.background_executor()
            .timer(std::time::Duration::from_millis(20))
            .await;
        let materialized = app
            .update(|cx| {
                shell.read(cx).stage.read(cx).active_chat_id == Some(600_000_777)
            })
            .unwrap();
        if materialized {
            break;
        }
    }

    app.update(|cx| {
        assert_eq!(
            shell.read(cx).stage.read(cx).active_chat_id,
            Some(600_000_777),
            "clicking 开始 must materialize bot DM chat"
        );
        let log = recorded_http.lock().unwrap();
        assert!(
            log.iter().any(|(m, p, _)| m == "POST" && p == "/api/v1/contacts/add"),
            "clicking 开始 must create DM chat via /contacts/add: {log:?}"
        );
    })
    .expect("assert materialized start");
}

#[test]
fn test_headless_gpui_tos_confirm_modal_write_access_popup_and_dialog_open_pill() {
    run_in_headless_subprocess(
        "test_headless_gpui_tos_confirm_modal_write_access_popup_and_dialog_open_pill",
    );
}

async fn headless_body_tos_confirm_modal_write_access_popup_and_dialog_open_pill(
    app: AsyncApp,
) {
    let seed_client = new_isolated_client("http://127.0.0.1:9".to_string());
    let (base_url, recorded_http, mock_state) = spawn_parity_mock_http_server(
        &seed_client,
        ParityMockState {
            chat_created: false,
            chat_id: 600_000_777,
            allows_write_to_pm: false,
            app_tos_accepted: false,
            is_blocked: false,
            has_started_chat: false,
            launched_once: false,
        },
    );
    let client = new_isolated_client(base_url);
    let bot_cid = 600_000_777u64;

    let shell = app
        .update(|cx| {
            cx.new(|cx| {
                let mut s = AppShell::new_with_client(client.clone(), cx);
                s.is_authenticated = true;
                s.session_sync.user_id = Some(100_001);
                s.session_sync.session_token = Some("tok_tos_write".to_string());
                s.miniapps.catalog_miniapps = vec![build_shop_miniapp(false, false, None)];
                s.miniapps.attach_menu_bots = vec![build_third_party_shop_bot()];
                s.session_sync.rich_chats_by_id.insert(
                    bot_cid,
                    HermesRichChatDto {
                        chat_id: bot_cid as i64,
                        title: "Pizza Shop".to_string(),
                        is_bot: true,
                        peer_user_id: Some(600_777),
                        peer_username: Some("PizzaShopBot".to_string()),
                        menu_button_text: Some("Order Pizza".to_string()),
                        menu_button_url: Some("gpui-miniapp://PizzaShopBot/shop".to_string()),
                        has_mini_app: true,
                        ..Default::default()
                    },
                );
                s
            })
        })
        .expect("create shell");

    // 1. Trigger dialog `打开` pill / launch_miniapp_from_uri while !app_tos_accepted -> opens ToS confirm modal
    app.update(|cx| {
        let sb = shell.read(cx).sidebar.clone();
        sb.update(cx, |sb, sb_cx| {
            let mut map = std::collections::HashMap::new();
            map.insert(bot_cid, "gpui-miniapp://PizzaShopBot/shop".to_string());
            sb.set_bot_main_app_uris(map);
            let dlg = gpui_telegram_desktop::views::chat_sidebar::DialogItemModel {
                id: bot_cid,
                peer_name: "Pizza Shop".to_string(),
                avatar_initials: "PS".to_string(),
                avatar_color: 0x2481cc,
                last_message: String::new(),
                timestamp: String::new(),
                unread_count: 0,
                is_muted: false,
                is_pinned: false,
                is_online: true,
                typing_indicator: None,
                delivery_status: gpui_telegram_desktop::views::chat_sidebar::DialogDeliveryStatus::Read,
                folder: "Bots".to_string(),
            };
            let meta = sb.dialog_meta(&dlg);
            assert_eq!(
                meta.miniapp_open_uri.as_deref(),
                Some("gpui-miniapp://PizzaShopBot/shop"),
                "Dialog row with bot Main Mini App must carry miniapp_open_uri for `打开` pill"
            );
            let _row_el = render_dialog_row_item(
                &dlg,
                0,
                false,
                false,
                &meta,
                &TdesktopThemePalette::default(),
                "zh-CN",
                sb_cx,
                |_, _| {},
                |_, _, _| {},
                |_, _| {},
                |_, _, _| {},
            );
            sb.pending_launch_dialog_miniapp =
                Some((bot_cid, "gpui-miniapp://PizzaShopBot/shop".to_string()));
            sb_cx.notify();
        });
    })
    .expect("notify dialog miniapp launch");

    app.update(|cx| {
        let s = shell.read(cx);
        assert!(
            s.miniapps.pending_miniapp_confirm.is_some(),
            "first-time launch must show ToS confirmation modal"
        );
        assert!(s.miniapps.miniapp_overlay.is_none());
    })
    .expect("trigger ToS confirm");

    // Confirm ToS -> calls POST /api/v1/bots/600777/accept-app-tos and launches BotAttachWebView!
    app.update(|cx| {
        shell.update(cx, |s, cx| {
            s.miniapps.miniapp_tos_modal_open = true;
            s.miniapps.miniapp_tos_modal_open = false;
            s.confirm_pending_miniapp_launch(cx);
        });
    })
    .expect("confirm launch");

    for _ in 0..40 {
        app.background_executor()
            .timer(std::time::Duration::from_millis(20))
            .await;
        let launched = app
            .update(|cx| {
                shell
                    .read(cx)
                    .miniapps
                    .miniapp_overlay
                    .as_ref()
                    .is_some_and(|ov| !ov.read(cx).is_loading && ov.read(cx).bot_id == 600_777)
            })
            .unwrap();
        if launched {
            break;
        }
    }

    app.update(|cx| {
        let ov = shell
            .read(cx)
            .miniapps
            .miniapp_overlay
            .clone()
            .expect("overlay open after ToS confirm");
        assert!(ov.read(cx).is_open);
        assert!(!ov.read(cx).allows_write_to_pm);
        assert!(mock_state.lock().unwrap().app_tos_accepted);

        // 2. JS calls Telegram.WebApp.requestWriteAccess() -> `web_app_request_write_access`
        ov.update(cx, |w, w_cx| {
            w.handle_bridge_event("web_app_request_write_access", "{}");
            assert!(w.write_access_prompt_open, "must open 允许发送消息 modal");
            assert!(!w.allows_write_to_pm);

            // Test MainButton / SecondaryButton / BackButton / ClosingConfirmation / showPopup bridge events
            w.handle_bridge_event(
                "web_app_setup_main_button",
                r#"{"is_visible":true,"is_active":true,"text":"Checkout $18"}"#,
            );
            assert_eq!(
                w.ui_schema.main_button.as_ref().map(|b| b.text.as_str()),
                Some("Checkout $18")
            );
            w.handle_bridge_event("web_app_setup_back_button", r#"{"is_visible":true}"#);
            assert!(w.ui_schema.back_button_visible);
            w.handle_bridge_event(
                "web_app_setup_closing_behavior",
                r#"{"need_confirmation":true}"#,
            );
            assert!(w.closing_confirmation);
            w.handle_bridge_event(
                "web_app_open_popup",
                r#"{"title":"Confirm Order","message":"1x Margherita","buttons":[{"id":"yes","type":"ok","text":"OK"}]}"#,
            );
            assert!(w.custom_popup.is_some());
            w.resolve_custom_popup(Some("yes"));
            assert!(w.custom_popup.is_none());

            // 3. User clicks `允许` in `允许发送消息` popup -> queues AllowBotSendMessage
            w.write_access_prompt_open = false;
            w.pending_actions
                .push_back(BotWebViewPendingAction::AllowBotSendMessage {
                    bot_id: w.bot_id,
                    bot_username: w.bot_username.clone(),
                    app_id: w.app_id.clone(),
                    app_title: w.ui_schema.title.clone(),
                });
            w_cx.notify();
        });
    })
    .expect("trigger write access prompt & allow");

    // Pump AsyncApp until allow-send-message finishes and resolves write_access + refreshes chats
    for _ in 0..50 {
        app.background_executor()
            .timer(std::time::Duration::from_millis(20))
            .await;
        let done = app
            .update(|cx| {
                let s = shell.read(cx);
                s.miniapps
                    .miniapp_overlay
                    .as_ref()
                    .is_some_and(|ov| ov.read(cx).allows_write_to_pm)
                    && !s.sidebar.read(cx).dialogs.is_empty()
            })
            .unwrap();
        if done {
            break;
        }
    }

    app.update(|cx| {
        let s = shell.read(cx);
        let ov = s.miniapps.miniapp_overlay.as_ref().unwrap().read(cx);
        assert!(ov.allows_write_to_pm, "Allow click must resolve allows_write_to_pm = true");
        assert!(!ov.write_access_prompt_open);
        assert!(
            s.sidebar.read(cx).dialogs.iter().any(|d| d.id == bot_cid),
            "allowing write access must refresh sidebar with created bot chat"
        );
        let log = recorded_http.lock().unwrap();
        assert!(log
            .iter()
            .any(|(m, p, _)| m == "POST" && p == "/api/v1/bots/600777/accept-app-tos"));
        assert!(log
            .iter()
            .any(|(m, p, _)| m == "POST" && p == "/api/v1/bots/600777/allow-send-message"));
        assert!(log
            .iter()
            .any(|(m, p, _)| m == "GET" && p == "/api/v1/miniapps/recent"));
    })
    .expect("assert write access granted & dialog refreshed");
}

#[test]
fn test_headless_gpui_altery_inline_webapp_custom_url_and_live_bot_pin_banner() {
    run_in_headless_subprocess(
        "test_headless_gpui_altery_inline_webapp_custom_url_and_live_bot_pin_banner",
    );
}

async fn headless_body_altery_inline_webapp_custom_url_and_live_bot_pin_banner(app: AsyncApp) {
    let seed_client = new_isolated_client("http://127.0.0.1:9".to_string());
    let (base_url, recorded_http, _mock_state) = spawn_parity_mock_http_server(
        &seed_client,
        ParityMockState {
            chat_created: true,
            chat_id: 600_000_777,
            allows_write_to_pm: true,
            app_tos_accepted: false,
            is_blocked: false,
            has_started_chat: true,
            launched_once: false,
        },
    );
    let client = new_isolated_client(base_url);
    let bot_cid = 600_000_777u64;
    let bot_uid = 600_777u64;
    let resume_url = "https://app.altery.example/kyc?resume=1";

    let shell = app
        .update(|cx| {
            cx.new(|cx| {
                let mut s = AppShell::new_with_client(client.clone(), cx);
                s.is_authenticated = true;
                s.session_sync.user_id = Some(100_001);
                s.session_sync.session_token = Some("tok_altery_e2e".to_string());
                s.session_sync.rich_chats_by_id.insert(
                    bot_cid,
                    HermesRichChatDto {
                        chat_id: bot_cid as i64,
                        title: "Altery".to_string(),
                        is_bot: true,
                        peer_user_id: Some(bot_uid),
                        peer_username: Some("AlteryVerifyBot".to_string()),
                        menu_button_text: Some("Launch Altery".to_string()),
                        menu_button_url: Some("https://app.altery.example/start".to_string()),
                        has_mini_app: false,
                        ..Default::default()
                    },
                );
                s.stage.update(cx, |st, _| {
                    st.set_chat(bot_cid, "Altery", "bot", "AL", 0x2481cc);
                });
                s.sidebar.update(cx, |sb, _| {
                    sb.dialogs.push(
                        gpui_telegram_desktop::views::chat_sidebar::DialogItemModel {
                            id: bot_cid,
                            peer_name: "Altery".to_string(),
                            avatar_initials: "AL".to_string(),
                            avatar_color: 0x2481cc,
                            last_message: "📌 Welcome to Altery".to_string(),
                            timestamp: "06:00".to_string(),
                            unread_count: 0,
                            is_muted: false,
                            is_pinned: false,
                            is_online: true,
                            typing_indicator: None,
                            delivery_status:
                                gpui_telegram_desktop::views::chat_sidebar::DialogDeliveryStatus::Read,
                            folder: "Bots".to_string(),
                        },
                    );
                });
                s
            })
        })
        .expect("create shell");

    // 1. Simulate live incoming `UpdateEvent::NewMessage` carrying an inline `web_app` button
    app.update(|cx| {
        shell.update(cx, |s, cx| {
            let mut incoming = gpui_telegram_proto::message::MessageItem::new(
                gpui_telegram_core::types::MsgId(501),
                gpui_telegram_core::types::ChatId(bot_cid as i64),
                gpui_telegram_core::types::UserId(bot_uid),
                gpui_telegram_core::types::PeerId::Chat(gpui_telegram_core::types::ChatId(
                    bot_cid as i64,
                )),
                gpui_telegram_proto::message::MessageContent::plain_text(
                    "⏳ Remember to Get Verified\nTap here to verify your Altery account.",
                ),
            );
            incoming.flags.is_out = false;
            incoming.inline_keyboard = Some(vec![vec![
                gpui_telegram_proto::message::InlineKeyboardButton {
                    text: "🆔 Verify Account".to_string(),
                    callback_data: None,
                    url: None,
                    web_app_url: Some(resume_url.to_string()),
                },
            ]]);
            s.apply_update_event(
                gpui_telegram_core::types::ChatId(bot_cid as i64),
                gpui_telegram_proto::sync::UpdateEvent::NewMessage(incoming),
                cx,
            );
        });

        let st = shell.read(cx).stage.read(cx);
        let msg = st
            .messages
            .iter()
            .find(|m| m.id == 501)
            .expect("live NewMessage 501 on stage");
        let kb = msg
            .bot_keyboard
            .as_ref()
            .expect("live NewMessage must populate bot_keyboard immediately");
        assert_eq!(kb[0][0].text, "🆔 Verify Account");
        assert_eq!(
            kb[0][0].button_type,
            InlineButtonType::WebApp(resume_url.to_string())
        );
    })
    .expect("assert live NewMessage inline keyboard");

    // 2. Simulate live `UpdateEvent::PinnedMessagesChanged` from Bot `pinChatMessage`
    app.update(|cx| {
        shell.update(cx, |s, cx| {
            s.apply_update_event(
                gpui_telegram_core::types::ChatId(bot_cid as i64),
                gpui_telegram_proto::sync::UpdateEvent::PinnedMessagesChanged {
                    chat_id: gpui_telegram_core::types::ChatId(bot_cid as i64),
                    pinned_ids: vec![gpui_telegram_core::types::MsgId(501)],
                },
                cx,
            );
        });

        let s = shell.read(cx);
        let st = s.stage.read(cx);
        assert_eq!(
            st.pinned_message.as_deref(),
            Some("⏳ Remember to Get Verified")
        );
        let banner = st
            .pinned_banner_info()
            .expect("pinned_banner_info must be populated");
        assert_eq!(banner.0, "⏳ Remember to Get Verified");

        let has_service_pin_row = st.messages.iter().any(|m| {
            matches!(
                &m.content,
                MessageContent::Service {
                    service_type: ServiceType::PinnedMessage,
                    text,
                } if text.contains("⏳ Remember to Get Verified")
            )
        });
        assert!(
            has_service_pin_row,
            "stage must append centered ServiceType::PinnedMessage notice"
        );
        let dlg_preview = &s.sidebar.read(cx).dialogs[0].last_message;
        assert!(
            dlg_preview.contains("⏳ Remember to Get Verified"),
            "sidebar preview must reflect pin notice: {dlg_preview}"
        );
    })
    .expect("assert live PinnedMessagesChanged banner & service row");

    // 3. Click `🆔 Verify Account` (`InlineButtonType::WebApp(resume_url)`) -> ToS confirm -> launch with `custom_url`
    app.update(|cx| {
        shell.update(cx, |s, cx| {
            s.handle_stage_bot_button(
                501,
                InlineButtonType::WebApp(resume_url.to_string()),
                cx,
            );
        });
        assert!(
            shell.read(cx).miniapps.pending_miniapp_confirm.is_some(),
            "first-time inline WebApp click opens ToS confirmation"
        );
        shell.update(cx, |s, cx| {
            s.confirm_pending_miniapp_launch(cx);
        });
    })
    .expect("click inline WebApp button and confirm ToS");

    for _ in 0..50 {
        app.background_executor()
            .timer(std::time::Duration::from_millis(20))
            .await;
        let ready = app
            .update(|cx| {
                shell
                    .read(cx)
                    .miniapps
                    .miniapp_overlay
                    .as_ref()
                    .is_some_and(|ov| ov.read(cx).launch_context.is_some())
            })
            .unwrap();
        if ready {
            break;
        }
    }

    app.update(|cx| {
        let s = shell.read(cx);
        let ov = s
            .miniapps
            .miniapp_overlay
            .as_ref()
            .expect("miniapp_overlay open")
            .read(cx);
        let resolved = ov.resolved_webview_url();
        assert!(
            resolved.starts_with("https://app.altery.example/kyc?resume=1#tgWebAppData="),
            "resolved_webview_url must preserve custom_url and append #tgWebAppData: {resolved}"
        );

        let log = recorded_http.lock().unwrap();
        let launch_call = log
            .iter()
            .find(|(m, p, _)| m == "POST" && p == "/api/v1/miniapps/AlteryVerifyBot/app/launch")
            .expect("expected POST /api/v1/miniapps/AlteryVerifyBot/app/launch");
        let sent_json: serde_json::Value = serde_json::from_str(&launch_call.2).unwrap();
        assert_eq!(sent_json["custom_url"], resume_url);
    })
    .expect("assert custom_url sent and resolved with #tgWebAppData");
}

