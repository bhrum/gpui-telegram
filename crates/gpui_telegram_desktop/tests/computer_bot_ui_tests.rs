//! Headless GPUI UI verification tests for Bot Computers Inspector, Settings Computers,
//! ComputerWorkbench, and AppShell bot-computer synchronization.
//!
//! Mounts real GPUI desktop entities inside `gpui::Application::headless().with_assets(TelegramAssetSource::new())`
//! using macOS main-thread constructor dispatch so real `Entity<ContextInspector>`,
//! `Entity<SettingsRouter>`, `Entity<SettingsComputersSections>`, `Entity<ComputerWorkbench>`,
//! and `Entity<AppShell>` execute end-to-end under GPUI.

use gpui::{App, AppContext, Application, Entity};
use gpui_telegram_client::net::HermesRichChatDto;
use gpui_telegram_client::{ClientConfig, TelegramClient};
use gpui_telegram_desktop::app::preferences::AppPreferences;
use gpui_telegram_desktop::theme::{TelegramAssetSource, TdesktopThemeMode, TdesktopThemePalette};
use gpui_telegram_desktop::views::computer_workbench::{
    decode_data_url_image, ComputerWorkbench, WorkbenchCommand, WorkbenchTab,
};
use gpui_telegram_desktop::views::context_inspector::ContextInspector;
use gpui_telegram_desktop::views::drawer::bot_computer_inspector::{
    render_bot_computers_subpage, render_bot_config_editor, render_bot_profile_main,
    resolve_bot_computer_binding,
};
use gpui_telegram_desktop::views::drawer::inspector_tabs::{
    BotConfigFocusField, InspectorAction, InspectorSubpage,
};
use gpui_telegram_desktop::views::settings::sections::{
    CloudFormField, SettingsComputersEvent, SettingsComputersPage, SettingsComputersSections,
};
use gpui_telegram_desktop::views::settings::{SettingsRouter, SettingsSectionType};
use gpui_telegram_desktop::views::shell::ActiveWorkspacePanel;
use gpui_telegram_desktop::views::AppShell;
use gpui_telegram_proto::computer_contracts::{
    AuthorizedControllerEntry, AuthorizedControllerKind, BotComputerProfileDto, CloudAuthMethod,
    ComputerNodeDto, ComputerNodeKind, ComputerRunPolicy, UpdateComputerPolicyRequest,
};
use gpui_telegram_proto::miniapp_contracts::BOTFATHER_USER_ID;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

const CHILD_ENV: &str = "GPUI_COMPUTER_BOT_UI_HEADLESS_CHILD";
const CHILD_TARGET_ENV: &str = "GPUI_COMPUTER_BOT_UI_HEADLESS_TARGET";
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(1);

struct TempConfigDirGuard {
    path: PathBuf,
}

impl TempConfigDirGuard {
    fn new(prefix: &str) -> Self {
        let seq = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "gpui_tg_desktop_computer_ui_{prefix}_{}_{}",
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
        "test_gpui_context_inspector_bot_computers_badges_and_subpages" => {
            body_context_inspector_bot_computers_badges_and_subpages
        }
        "test_gpui_settings_router_computers_acl_policy_and_cloud_form" => {
            body_settings_router_computers_acl_policy_and_cloud_form
        }
        "test_gpui_computer_workbench_tabs_commands_and_logs" => {
            body_computer_workbench_tabs_commands_and_logs
        }
        "test_gpui_appshell_created_and_system_bots_computer_wiring" => {
            body_appshell_created_and_system_bots_computer_wiring
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

fn new_isolated_client(base_url: String) -> Arc<TelegramClient> {
    let cfg = ClientConfig {
        server_url: base_url,
        http_timeout_secs: 5,
        ws_ping_interval_secs: 25,
        max_reconnect_attempts: 1,
        ..ClientConfig::default()
    };
    let client = Arc::new(TelegramClient::new(cfg));
    client.set_token(Some("tok_headless_computer_ui_test".to_string()));
    client
}

fn sample_local_node() -> ComputerNodeDto {
    ComputerNodeDto {
        node_id: "local:100:dev-mac".to_string(),
        owner_user_id: 100,
        kind: ComputerNodeKind::LocalDevice,
        display_name: "bhrum-MacBook-Pro".to_string(),
        ip_address: "192.168.1.24".to_string(),
        host: None,
        ssh_port: None,
        ssh_username: None,
        auth_method: None,
        agent_endpoint_url: None,
        workdir: None,
        has_saved_credential: false,
        os_name: "macOS aarch64".to_string(),
        resolution: "1440x900".to_string(),
        online: true,
        is_current_device: true,
        device_id: Some("dev-mac".to_string()),
        tier_badge: "本地".to_string(),
        run_policy: ComputerRunPolicy::AskEveryTime,
        authorized_controllers: Vec::new(),
        bound_bot_ids: Vec::new(),
        last_seen_ms: 1_700_000_000_000,
        latest_frame_data_url: Some(
            "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO7+X8sAAAAASUVORK5CYII=".to_string(),
        ),
    }
}

fn sample_cloud_node() -> ComputerNodeDto {
    ComputerNodeDto {
        node_id: "cloud:100:prod-1".to_string(),
        owner_user_id: 100,
        kind: ComputerNodeKind::CloudServer,
        display_name: "Prod Cloud Agent".to_string(),
        ip_address: "203.0.113.45".to_string(),
        host: Some("203.0.113.45".to_string()),
        ssh_port: Some(8099),
        ssh_username: Some("deploy".to_string()),
        auth_method: Some(CloudAuthMethod::AgentHttpToken),
        agent_endpoint_url: Some("http://203.0.113.45:8099/api/v1/computer-agent/exec".to_string()),
        workdir: Some("/srv/app".to_string()),
        has_saved_credential: true,
        os_name: "Ubuntu 24.04".to_string(),
        resolution: "1920x1080".to_string(),
        online: true,
        is_current_device: false,
        device_id: None,
        tier_badge: "云电脑".to_string(),
        run_policy: ComputerRunPolicy::AlwaysAllowAuthorized,
        authorized_controllers: vec![AuthorizedControllerEntry {
            controller_id: "all_bots".to_string(),
            kind: AuthorizedControllerKind::AccountAllBots,
            display_name: "All Bots".to_string(),
            added_at_ms: 1_700_000_000_100,
        }],
        bound_bot_ids: vec![777001],
        last_seen_ms: 1_700_000_000_200,
        latest_frame_data_url: None,
    }
}

fn sample_sandbox_node() -> ComputerNodeDto {
    ComputerNodeDto {
        node_id: "sandbox:default".to_string(),
        owner_user_id: 100,
        kind: ComputerNodeKind::CloudSandbox,
        display_name: "默认沙箱桌面".to_string(),
        ip_address: "127.0.0.1".to_string(),
        host: None,
        ssh_port: None,
        ssh_username: None,
        auth_method: None,
        agent_endpoint_url: None,
        workdir: None,
        has_saved_credential: false,
        os_name: "Cloud Sandbox".to_string(),
        resolution: "1280x720".to_string(),
        online: true,
        is_current_device: false,
        device_id: None,
        tier_badge: "免费".to_string(),
        run_policy: ComputerRunPolicy::AlwaysAllowAuthorized,
        authorized_controllers: Vec::new(),
        bound_bot_ids: Vec::new(),
        last_seen_ms: 1_700_000_000_000,
        latest_frame_data_url: None,
    }
}

#[test]
fn test_gpui_context_inspector_bot_computers_badges_and_subpages() {
    run_in_headless_subprocess("test_gpui_context_inspector_bot_computers_badges_and_subpages");
}

fn body_context_inspector_bot_computers_badges_and_subpages(cx: &mut App) {
    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Day, 0x2481cc, 14, 12);
    let insp = cx.new(|_| {
        let mut i = ContextInspector::new();
        i.open();
        i.set_palette(pal);
        i.set_language("zh-CN");
        i.set_peer("QuantOps Bot", "AI Agent", "QO", 0x2481cc);
        i.sync_bot_details(
            true,
            777001,
            "You are QuantOps AI Bot.".to_string(),
            vec![("status".to_string(), "Check node health".to_string())],
            None,
            None,
            false,
        );
        i
    });

    // 1. Default Sandbox -> "免费" badge when no cloud/local computer is bound
    let sandbox = sample_sandbox_node();
    let local = sample_local_node();
    let cloud = sample_cloud_node();

    insp.update(cx, |i, _| {
        i.sync_computers(vec![sandbox.clone(), local.clone()]);
        i.sync_bot_computer_profile(Some(BotComputerProfileDto {
            bot_id: 777001,
            bound_node_id: Some(sandbox.node_id.clone()),
            persona_prompt: "QuantOps prompt".to_string(),
            preferred_model: "hermes-local-v1".to_string(),
            approval_mode: ComputerRunPolicy::AskEveryTime,
            allow_cross_device_control: true,
            allow_cloud_self_config: true,
            updated_at_ms: 1_700_000_000_000,
        }));
        assert!(i.profile.is_bot);
        assert_eq!(i.subpage, InspectorSubpage::Main);
        let (explicit, effective) =
            resolve_bot_computer_binding(&i.computers, i.bot_computer_profile.as_ref());
        assert_eq!(explicit.map(|n| n.tier_badge.as_str()), Some("免费"));
        assert_eq!(effective.map(|n| n.tier_badge.as_str()), Some("免费"));

        // Switch binding to LocalDevice -> "本地" badge
        i.sync_bot_computer_profile(Some(BotComputerProfileDto {
            bot_id: 777001,
            bound_node_id: Some(local.node_id.clone()),
            persona_prompt: "QuantOps prompt".to_string(),
            preferred_model: "hermes-local-v1".to_string(),
            approval_mode: ComputerRunPolicy::AlwaysAllowAuthorized,
            allow_cross_device_control: true,
            allow_cloud_self_config: true,
            updated_at_ms: 1_700_000_000_001,
        }));
        let (explicit_local, _) =
            resolve_bot_computer_binding(&i.computers, i.bot_computer_profile.as_ref());
        assert_eq!(explicit_local.map(|n| n.tier_badge.as_str()), Some("本地"));

        // Switch binding to CloudServer -> "云电脑" badge
        i.sync_computers(vec![sandbox.clone(), local.clone(), cloud.clone()]);
        i.sync_bot_computer_profile(Some(BotComputerProfileDto {
            bot_id: 777001,
            bound_node_id: Some(cloud.node_id.clone()),
            persona_prompt: "QuantOps prompt".to_string(),
            preferred_model: "gpt-4o-mini".to_string(),
            approval_mode: ComputerRunPolicy::AlwaysAllowAuthorized,
            allow_cross_device_control: true,
            allow_cloud_self_config: true,
            updated_at_ms: 1_700_000_000_002,
        }));
        let (bound_node, _) =
            resolve_bot_computer_binding(&i.computers, i.bot_computer_profile.as_ref());
        assert_eq!(
            bound_node.map(|n| n.node_id.clone()),
            Some("cloud:100:prod-1".to_string())
        );
        assert_eq!(bound_node.map(|n| n.tier_badge.as_str()), Some("云电脑"));
    });

    // 2. Render GPUI element builders across all 3 subpages (Main, ComputersList empty/populated, BotConfigEditor)
    insp.update(cx, |i, icx| {
        // Main subpage (`<BotName>的电脑`, badge, `查看云电脑和本地电脑`)
        let _main_el = render_bot_profile_main(i, pal, "zh-CN", icx);

        // ComputersList with ONLY LocalDevice (triggers empty Cloud Server state:
        // `每个 Agent 都拥有自己的电脑` / `只有在你的 Agent 需要保持 24/7 运行时，才添加一台常驻云电脑。`)
        i.sync_computers(vec![local.clone()]);
        i.subpage = InspectorSubpage::ComputersList;
        let _empty_cloud_el = render_bot_computers_subpage(i, pal, "zh-CN", icx);

        // ComputersList populated with CloudServer card
        i.sync_computers(vec![local.clone(), cloud.clone()]);
        let _populated_cloud_el = render_bot_computers_subpage(i, pal, "zh-CN", icx);

        // BotConfigEditor (`AI Bot 配置`)
        i.open_bot_config_editor(icx);
        assert_eq!(i.subpage, InspectorSubpage::BotConfigEditor);
        assert_eq!(i.bot_config_focus, BotConfigFocusField::Persona);
        assert_eq!(i.bot_draft_persona, "QuantOps prompt");
        assert_eq!(i.bot_draft_model, "gpt-4o-mini");
        let _cfg_el = render_bot_config_editor(i, pal, "zh-CN", icx);

        // Trigger save action payload and verify independent per-bot payload
        i.bot_draft_persona = "Updated Bot Persona".to_string();
        i.bot_draft_model = "hermes-cloud-v2".to_string();
        i.bot_draft_approval_mode = ComputerRunPolicy::AllowReadOnly;
        i.bot_draft_allow_cross_device = false;
        i.bot_draft_allow_cloud_self_config = false;
        i.pending_action = Some(InspectorAction::SaveBotComputerConfig {
            bot_id: i.profile.bot_id,
            persona_prompt: i.bot_draft_persona.clone(),
            preferred_model: i.bot_draft_model.clone(),
            approval_mode: i.bot_draft_approval_mode,
            allow_cross_device_control: i.bot_draft_allow_cross_device,
            allow_cloud_self_config: i.bot_draft_allow_cloud_self_config,
        });
        match i.take_action() {
            Some(InspectorAction::SaveBotComputerConfig {
                bot_id,
                persona_prompt,
                preferred_model,
                approval_mode,
                allow_cross_device_control,
                allow_cloud_self_config,
            }) => {
                assert_eq!(bot_id, 777001);
                assert_eq!(persona_prompt, "Updated Bot Persona");
                assert_eq!(preferred_model, "hermes-cloud-v2");
                assert_eq!(approval_mode, ComputerRunPolicy::AllowReadOnly);
                assert!(!allow_cross_device_control);
                assert!(!allow_cloud_self_config);
            }
            other => panic!("unexpected inspector action: {other:?}"),
        }
        i.subpage = InspectorSubpage::Main;
    });
}

#[test]
fn test_gpui_settings_router_computers_acl_policy_and_cloud_form() {
    run_in_headless_subprocess("test_gpui_settings_router_computers_acl_policy_and_cloud_form");
}

fn body_settings_router_computers_acl_policy_and_cloud_form(cx: &mut App) {
    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Night, 0x5288C1, 14, 12);
    let sr = cx.new(|_| SettingsRouter::new(pal, true, AppPreferences::default()));

    let sc = sr.update(cx, |router, rcx| {
        assert!(router.settings_computers.is_none());
        router.navigate_to(SettingsSectionType::Computers, true, rcx);
        assert_eq!(router.settings_section, SettingsSectionType::Computers);
        router.ensure_computers(rcx)
    });

    let local = sample_local_node();
    let cloud = sample_cloud_node();

    sc.update(cx, |view, vcx| {
        view.sync_computers(vec![local.clone(), cloud.clone()]);
        view.sync_known_bots(
            vec![(777001, "QuantOps Bot".into()), (1, "BotFather".into())],
            Some(777001),
        );
        assert_eq!(view.page, SettingsComputersPage::Overview);

        // Navigate into `< 计算机详情` for LocalDevice with empty ACL (`没有已授权的设备`)
        view.open_detail(&local.node_id);
        assert_eq!(view.page, SettingsComputersPage::Detail(local.node_id.clone()));
        assert!(view.authorized_controllers_empty_check());

        // Toggle policy dropdown and emit UpdatePolicy
        view.policy_menu_open = true;
        view.pending_event = Some(SettingsComputersEvent::UpdatePolicy {
            node_id: local.node_id.clone(),
            req: UpdateComputerPolicyRequest {
                run_policy: Some(ComputerRunPolicy::AlwaysAllowAuthorized),
                ..Default::default()
            },
        });
        match view.take_event() {
            Some(SettingsComputersEvent::UpdatePolicy { node_id, req }) => {
                assert_eq!(node_id, local.node_id);
                assert_eq!(req.run_policy, Some(ComputerRunPolicy::AlwaysAllowAuthorized));
            }
            other => panic!("expected UpdatePolicy event, got {other:?}"),
        }

        // Add Bot controller authorization via Detail pickers
        view.bot_picker_open = true;
        view.pending_event = Some(SettingsComputersEvent::UpdatePolicy {
            node_id: local.node_id.clone(),
            req: UpdateComputerPolicyRequest {
                add_controller: Some(AuthorizedControllerEntry {
                    controller_id: "all_bots".to_string(),
                    kind: AuthorizedControllerKind::AccountAllBots,
                    display_name: "所有 AI Bot".to_string(),
                    added_at_ms: 0,
                }),
                ..Default::default()
            },
        });
        assert!(matches!(
            view.take_event(),
            Some(SettingsComputersEvent::UpdatePolicy {
                req: UpdateComputerPolicyRequest {
                    add_controller: Some(_),
                    ..
                },
                ..
            })
        ));

        // Open Cloud Server creation form pre-bound to current bot
        view.open_cloud_form(Some(777001), None);
        assert_eq!(view.page, SettingsComputersPage::CloudForm);
        assert_eq!(view.form_focus, CloudFormField::DisplayName);
        assert!(view.form_bind_to_current_bot);
        assert!(!view.submit_cloud_form());
        assert!(view.status_message.is_some());

        view.form_name = "Singapore GPU".to_string();
        view.form_host = "203.0.113.90".to_string();
        view.form_port = "8099".to_string();
        view.form_username = "ubuntu".to_string();
        view.form_auth_method = CloudAuthMethod::AgentHttpToken;
        view.form_credential = "tok_gpu_secret".to_string();
        view.form_agent_url = "http://203.0.113.90:8099".to_string();
        view.form_workdir = "/workspace".to_string();
        assert!(view.submit_cloud_form());

        match view.take_event() {
            Some(SettingsComputersEvent::SaveCloudServer(req)) => {
                assert_eq!(req.display_name, "Singapore GPU");
                assert_eq!(req.auth_method, CloudAuthMethod::AgentHttpToken);
                assert_eq!(req.secret.as_deref(), Some("tok_gpu_secret"));
                assert_eq!(req.bind_bot_id, Some(777001));
                assert!(req.verify_connection);
            }
            other => panic!("expected SaveCloudServer, got {other:?}"),
        }
        vcx.notify();
    });

    // Verify SettingsComputersSections page transitions across Overview, Detail, and CloudForm
    for target_page in [
        SettingsComputersPage::Overview,
        SettingsComputersPage::Detail(local.node_id.clone()),
        SettingsComputersPage::CloudForm,
    ] {
        sc.update(cx, |view, vcx| {
            match target_page {
                SettingsComputersPage::Overview => view.open_overview(),
                SettingsComputersPage::Detail(ref id) => view.open_detail(id),
                SettingsComputersPage::CloudForm => view.open_cloud_form(Some(777001), Some(&cloud)),
            }
            assert_eq!(view.page, target_page);
            vcx.notify();
        });
    }
}

trait EmptyAclExt {
    fn authorized_controllers_empty_check(&self) -> bool;
}

impl EmptyAclExt for SettingsComputersSections {
    fn authorized_controllers_empty_check(&self) -> bool {
        self.computers
            .iter()
            .find(|n| Some(n.node_id.as_str()) == self.selected_node_id())
            .is_some_and(|n| n.authorized_controllers.is_empty())
    }
}

trait SelectedNodeExt {
    fn selected_node_id(&self) -> Option<&str>;
}

impl SelectedNodeExt for SettingsComputersSections {
    fn selected_node_id(&self) -> Option<&str> {
        self.computers.first().map(|n| n.node_id.as_str())
    }
}

#[test]
fn test_gpui_computer_workbench_tabs_commands_and_logs() {
    run_in_headless_subprocess("test_gpui_computer_workbench_tabs_commands_and_logs");
}

fn body_computer_workbench_tabs_commands_and_logs(cx: &mut App) {
    let pal = TdesktopThemePalette::build(TdesktopThemeMode::Day, 0x2481cc, 14, 12);
    let node = sample_local_node();
    let wb = cx.new(|wcx| {
        let mut w = ComputerWorkbench::new_live(node.display_name.clone(), pal, "zh-CN");
        w.set_target_node(&node);
        w.ensure_terminal_input(wcx);
        w
    });

    wb.update(cx, |w, wcx| {
        assert_eq!(w.active_tab, WorkbenchTab::DesktopScreen);
        assert_eq!(w.node_id.as_deref(), Some(node.node_id.as_str()));
        assert!(w.is_connected);

        // Verify valid PNG decoding for desktop screen data URL
        let decoded = decode_data_url_image(
            node.latest_frame_data_url.as_deref().unwrap(),
        );
        assert!(decoded.is_some(), "valid PNG data URL must decode");
        w.update_frame_url(node.latest_frame_data_url.clone());

        // Switch to RemoteTerminal tab, queue ShellExec command, and receive log update
        w.active_tab = WorkbenchTab::RemoteTerminal;
        w.push_terminal_text("uname -a", wcx);
        assert!(w.submit_terminal_draft());
        match w.take_command() {
            Some(WorkbenchCommand::RunShell { command, cwd }) => {
                assert_eq!(command, "uname -a");
                assert_eq!(cwd.as_deref(), Some("~"));
            }
            other => panic!("expected RunShell WorkbenchCommand, got {other:?}"),
        }

        w.push_terminal_output(
            "uname -a".into(),
            "Darwin Kernel Version 24.0.0".into(),
            String::new(),
            0,
            Some("/Users/bhrum".into()),
        );
        assert_eq!(w.remote_cwd, "/Users/bhrum");
        assert_eq!(w.terminal_history.len(), 1);
        assert_eq!(
            w.terminal_history[0].stdout,
            "Darwin Kernel Version 24.0.0"
        );
    });

    // Switch tabs across DesktopScreen and RemoteTerminal and dispatch GUI + file commands
    for tab in [WorkbenchTab::DesktopScreen, WorkbenchTab::RemoteTerminal] {
        wb.update(cx, |w, wcx| {
            w.active_tab = tab;
            wcx.notify();
        });
        assert_eq!(wb.read(cx).active_tab, tab);
    }

    wb.update(cx, |w, _| {
        w.pending_command = Some(WorkbenchCommand::RefreshScreen);
        assert_eq!(w.take_command(), Some(WorkbenchCommand::RefreshScreen));
        w.pending_command = Some(WorkbenchCommand::SendGuiClick { x: 120, y: 240 });
        assert_eq!(
            w.take_command(),
            Some(WorkbenchCommand::SendGuiClick { x: 120, y: 240 })
        );
        w.pending_command = Some(WorkbenchCommand::SendGuiText {
            text: "ls".into(),
        });
        assert_eq!(
            w.take_command(),
            Some(WorkbenchCommand::SendGuiText { text: "ls".into() })
        );
        w.set_remote_files(
            "/srv/app".into(),
            vec!["[DIR] src".into(), "Cargo.toml".into()],
        );
        assert_eq!(w.remote_cwd, "/srv/app");
        assert_eq!(w.remote_files.len(), 2);
        w.pending_command = Some(WorkbenchCommand::ReadRemoteFile {
            path: "/srv/app/Cargo.toml".into(),
        });
        assert_eq!(
            w.take_command(),
            Some(WorkbenchCommand::ReadRemoteFile {
                path: "/srv/app/Cargo.toml".into()
            })
        );
        w.set_file_preview(
            "/srv/app/Cargo.toml".into(),
            "[package]\nname = \"app\"".into(),
        );
        assert_eq!(
            w.file_preview.as_ref().map(|(p, _)| p.as_str()),
            Some("/srv/app/Cargo.toml")
        );
    });
}

#[test]
fn test_gpui_appshell_created_and_system_bots_computer_wiring() {
    run_in_headless_subprocess("test_gpui_appshell_created_and_system_bots_computer_wiring");
}

fn body_appshell_created_and_system_bots_computer_wiring(cx: &mut App) {
    let client = new_isolated_client("http://127.0.0.1:9".to_string());
    let _rt = client.runtime().enter();

    let local = sample_local_node();
    let cloud = sample_cloud_node();

    let shell: Entity<AppShell> = cx.new(|cx| {
        let mut s = AppShell::new_with_client(client.clone(), cx);
        s.is_authenticated = true;
        s.session_sync.user_id = Some(100);
        s.session_sync.session_token = Some("tok_headless_computer_ui_test".to_string());
        s.computers.computers = vec![local.clone(), cloud.clone()];

        // Register BOTH a system bot (BotFather) and a user-created AI Bot (Ops Copilot)
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
        s.session_sync.rich_chats_by_id.insert(
            9002,
            HermesRichChatDto {
                chat_id: 9002,
                title: "Ops Copilot".to_string(),
                is_bot: true,
                peer_user_id: Some(777002),
                peer_username: Some("ops_copilot_bot".to_string()),
                peer_bio: Some("Cloud & device ops bot".to_string()),
                ..Default::default()
            },
        );
        s
    });

    // 1. Select system bot -> inspector opens with `<Bot>的电脑` eligibility (`is_bot == true`)
    shell.update(cx, |s, cx| {
        s.open_or_select_chat(8001, Some("BotFather".into()), true, cx);
        let insp = s.inspector.read(cx);
        assert!(insp.profile.is_bot, "system bot must mark inspector is_bot=true");
        assert_eq!(insp.profile.bot_id, BOTFATHER_USER_ID as u64);
        assert_eq!(insp.profile.name, "BotFather");

        // 2. Select user-created bot -> inspector also exposes `<Bot>的电脑`
        s.open_or_select_chat(9002, Some("Ops Copilot".into()), true, cx);
        let insp2 = s.inspector.read(cx);
        assert!(insp2.profile.is_bot, "created AI bot must mark inspector is_bot=true");
        assert_eq!(insp2.profile.bot_id, 777002);
        assert_eq!(insp2.profile.name, "Ops Copilot");

        // Known bots collection includes both system and created bots
        let known = s.known_bots_for_computers(cx);
        assert!(known.iter().any(|(id, _)| *id == BOTFATHER_USER_ID as u64));
        assert!(known.iter().any(|(id, _)| *id == 777002));

        // 3. Dispatch InspectorAction::OpenRemoteWorkbench -> opens ComputerWorkbench overlay
        assert!(s.handle_computer_inspector_action(
            InspectorAction::OpenRemoteWorkbench {
                node_id: cloud.node_id.clone(),
            },
            cx,
        ));
        let wb_overlay = s
            .overlays
            .computer_overlay
            .clone()
            .expect("computer workbench overlay opened");
        assert_eq!(
            wb_overlay.read(cx).node_id.as_deref(),
            Some(cloud.node_id.as_str())
        );

        // 4. Dispatch InspectorAction::OpenComputerDetailInSettings -> opens Settings -> Computers Detail
        assert!(s.handle_computer_inspector_action(
            InspectorAction::OpenComputerDetailInSettings {
                node_id: local.node_id.clone(),
            },
            cx,
        ));
        assert!(matches!(s.active_panel, ActiveWorkspacePanel::Settings));
        let sc = {
            let sr = s.settings_router.read(cx);
            assert_eq!(sr.settings_section, SettingsSectionType::Computers);
            sr.settings_computers
                .clone()
                .expect("computers section materialized")
        };
        assert_eq!(
            sc.read(cx).page,
            SettingsComputersPage::Detail(local.node_id.clone())
        );

        // 5. Dispatch InspectorAction::OpenCreateCloudComputerModal -> opens CloudForm pre-bound to bot 777002
        assert!(s.handle_computer_inspector_action(
            InspectorAction::OpenCreateCloudComputerModal {
                bind_bot_id: Some(777002),
            },
            cx,
        ));
        assert_eq!(sc.read(cx).page, SettingsComputersPage::CloudForm);
        assert!(sc.read(cx).form_bind_to_current_bot);
    });
}
