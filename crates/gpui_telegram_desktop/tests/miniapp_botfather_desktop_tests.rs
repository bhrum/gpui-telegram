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

use gpui::{px, App, AppContext, Application, Entity, Hsla, Size};
use gpui_telegram_client::net::{HermesRichChatDto, PeerBotActionResult};
use gpui_telegram_client::{AccountSessionRecord, ClientConfig, TelegramClient};
use gpui_telegram_desktop::components::bots::bot_attach_web_view::{
    peer_matches_types, peer_types, peer_types_mask_from_strings, AttachBotEntry,
    BotAttachWebView, BotWebViewInputTarget, BotWebViewPendingAction,
};
use gpui_telegram_desktop::components::ui::chat::attach::attach_bot_webview::AttachBotWebviewChatAttach;
use gpui_telegram_desktop::theme::{TelegramAssetSource, TdesktopThemePalette};
use gpui_telegram_desktop::views::boxes::peers::CreateManagedBotBoxPeers;
use gpui_telegram_desktop::views::chat_sidebar::{
    SidebarCatalogAppItem, TELEGRAM_SEARCH_CATEGORIES,
};
use gpui_telegram_desktop::views::composer::attachments_drawer::ComposerAttachMiniApp;
use gpui_telegram_desktop::views::composer::popups_ui::{
    render_attachments_drawer_with_apps, ComposerPopupAction,
};
use gpui_telegram_desktop::views::sidebar::sidebar_search_view::render_apps_catalog_section;
use gpui_telegram_desktop::views::stage::cards::{
    DeliveryStatus, InlineButtonType, InlineKeyboardButton, MessageContent, MessageDirection,
    MessageItemModel,
};
use gpui_telegram_desktop::views::stage::cards_interactive::InteractiveCardAction;
use gpui_telegram_desktop::views::stage::stage_bubble::BubbleUiAction;
use gpui_telegram_desktop::views::stage::view::controls::HistoryViewBotMenuButtonViewControls;
use gpui_telegram_desktop::views::{
    AppShell, CanonicalComposer, ChatSidebar, ConversationStage, MainMenuDrawer,
};
use gpui_telegram_proto::miniapp_contracts::{
    BotRegistryEntry, CloudStorageItem, MiniAppActionButton, MiniAppActionRequest,
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

fn dispatch_headless_target(target: &str) -> ! {
    let body: fn(&mut App) = match target {
        "test_headless_gpui_mount_all_miniapp_botfather_components_and_navigation" => {
            headless_body_mount_all_miniapp_botfather_components_and_navigation
        }
        "test_headless_gpui_stage_webapp_launch_no_clipboard_and_webview_lifecycle" => {
            headless_body_stage_webapp_launch_no_clipboard_and_webview_lifecycle
        }
        "test_headless_gpui_companion_bot_menu_button_and_attach_peer_filtering" => {
            headless_body_companion_bot_menu_button_and_attach_peer_filtering
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
            .run(move |cx: &mut App| {
                let inner = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    body(cx);
                }));
                match inner {
                    Ok(()) => std::process::exit(0),
                    Err(payload) => {
                        let msg = payload
                            .downcast_ref::<&str>()
                            .copied()
                            .or_else(|| payload.downcast_ref::<String>().map(|s| s.as_str()))
                            .unwrap_or("panic");
                        eprintln!("[headless child panic] {target_owned}: {msg}");
                        std::process::exit(101);
                    }
                }
            });
    }));
    if let Err(payload) = outer {
        let msg = payload
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| payload.downcast_ref::<String>().map(|s| s.as_str()))
            .unwrap_or("panic");
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

            // 2. Trigger WebApp button in ConversationStage -> MUST launch BotAttachWebView, NEVER copy URL!
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
            let ov = shell
                .read(cx)
                .miniapps
                .miniapp_overlay
                .clone()
                .expect("HistoryViewBotMenuButtonViewControls click must launch Mini App via AppShell");
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
    let _ = Size {
        width: px(400.0),
        height: px(600.0),
    };
}
