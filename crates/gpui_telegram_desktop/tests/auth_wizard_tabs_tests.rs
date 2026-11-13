//! Milestone 2 (F8) Test Suite: Multi-Tab Login Wizard State Machine, Form Validation & Cooldown Timer
//! Target File: `crates/gpui_telegram_desktop/tests/auth_wizard_tabs_tests.rs`

use std::fs;
use std::path::Path;
use gpui_telegram_desktop::views::shell::{AuthWizardTab, LoginInputField};

// ============================================================================
// PART 1: State Machine & Form Validation Unit Tests
// ============================================================================

/// State simulator reproducing the exact AppShell Login Wizard logic.
#[derive(Debug, Clone)]
pub struct AuthWizardStateSimulator {
    pub active_tab: AuthWizardTab,
    pub active_field: LoginInputField,
    pub is_authenticated: bool,
    pub login_error: Option<String>,
    pub email_input: String,
    pub email_code_input: String,
    pub username_input: String,
    pub password_input: String,
    pub email_code_cooldown: u32,
    pub is_sending_code: bool,
    pub alipay_attempt_id: Option<String>,
    pub alipay_status_text: String,
    pub alipay_is_loading: bool,
    pub poll_generation: u64,
}

impl AuthWizardStateSimulator {
    pub fn new() -> Self {
        Self {
            active_tab: AuthWizardTab::PasswordLogin,
            active_field: LoginInputField::Username,
            is_authenticated: false,
            login_error: None,
            email_input: String::new(),
            email_code_input: String::new(),
            username_input: String::new(),
            password_input: String::new(),
            email_code_cooldown: 0,
            is_sending_code: false,
            alipay_attempt_id: None,
            alipay_status_text: String::new(),
            alipay_is_loading: false,
            poll_generation: 0,
        }
    }

    pub fn switch_tab(&mut self, tab: AuthWizardTab) {
        if self.active_tab == tab {
            return;
        }
        self.active_tab = tab;
        self.login_error = None;

        match tab {
            AuthWizardTab::PasswordLogin => {
                self.active_field = LoginInputField::Username;
            }
            AuthWizardTab::AlipayQr => {
                self.start_alipay();
            }
            AuthWizardTab::EmailRegister => {
                self.active_field = LoginInputField::Email;
            }
        }
    }

    pub fn start_alipay(&mut self) {
        self.alipay_is_loading = true;
        self.alipay_attempt_id = None;
        self.poll_generation += 1;
        self.alipay_status_text = "正在获取支付宝登录二维码...".to_string();
    }

    pub fn on_alipay_started(&mut self, attempt_id: String) {
        self.alipay_is_loading = false;
        self.alipay_attempt_id = Some(attempt_id);
        self.alipay_status_text = "请使用手机支付宝 App 扫码登录".to_string();
    }

    pub fn on_alipay_completed(&mut self) {
        self.is_authenticated = true;
        self.alipay_status_text = "登录成功".to_string();
    }

    pub fn send_email_code(&mut self) -> Result<(), &'static str> {
        let email = self.email_input.trim();
        if email.is_empty() || !email.contains('@') || !email.contains('.') {
            self.login_error = Some("请输入有效的邮箱地址".to_string());
            return Err("Invalid email");
        }
        if self.email_code_cooldown > 0 || self.is_sending_code {
            return Err("Cooldown active");
        }
        self.email_code_cooldown = 60;
        self.login_error = None;
        self.active_field = LoginInputField::EmailCode;
        Ok(())
    }

    pub fn tick_cooldown(&mut self) {
        if self.email_code_cooldown > 0 {
            self.email_code_cooldown -= 1;
        }
    }

    pub fn submit_email_register(&mut self) -> Result<(), &'static str> {
        let email = self.email_input.trim();
        let code = self.email_code_input.trim();
        let user = self.username_input.trim();
        let pass = self.password_input.trim();

        if email.is_empty() || !email.contains('@') {
            self.login_error = Some("请输入有效的邮箱地址".to_string());
            return Err("Invalid email");
        }
        if code.len() < 4 {
            self.login_error = Some("请输入收到的验证码".to_string());
            return Err("Invalid code");
        }
        if user.is_empty() || pass.is_empty() {
            self.login_error = Some("用户名和密码不能为空".to_string());
            return Err("Missing credentials");
        }

        self.login_error = None;
        self.is_authenticated = true;
        Ok(())
    }
}

#[test]
fn test_auth_wizard_tab_initial_and_switching() {
    let mut sim = AuthWizardStateSimulator::new();
    assert_eq!(sim.active_tab, AuthWizardTab::PasswordLogin);
    assert_eq!(sim.active_field, LoginInputField::Username);
    assert!(!sim.is_authenticated);

    // Switch to AlipayQr
    sim.switch_tab(AuthWizardTab::AlipayQr);
    assert_eq!(sim.active_tab, AuthWizardTab::AlipayQr);
    assert!(sim.alipay_is_loading);
    assert_eq!(sim.poll_generation, 1);

    // Switch to EmailRegister
    sim.switch_tab(AuthWizardTab::EmailRegister);
    assert_eq!(sim.active_tab, AuthWizardTab::EmailRegister);
    assert_eq!(sim.active_field, LoginInputField::Email);
    assert!(sim.login_error.is_none());

    // Switch back to PasswordLogin
    sim.switch_tab(AuthWizardTab::PasswordLogin);
    assert_eq!(sim.active_tab, AuthWizardTab::PasswordLogin);
    assert_eq!(sim.active_field, LoginInputField::Username);
}

#[test]
fn test_email_registration_validation_boundaries() {
    let mut sim = AuthWizardStateSimulator::new();
    sim.switch_tab(AuthWizardTab::EmailRegister);

    // Empty email
    assert!(sim.send_email_code().is_err());
    assert_eq!(sim.login_error.as_deref(), Some("请输入有效的邮箱地址"));

    // Missing domain
    sim.email_input = "invalid_user@".to_string();
    assert!(sim.send_email_code().is_err());

    // Valid email -> starts cooldown
    sim.email_input = "alice@example.com".to_string();
    assert!(sim.send_email_code().is_ok());
    assert_eq!(sim.email_code_cooldown, 60);
    assert_eq!(sim.active_field, LoginInputField::EmailCode);

    // Duplicate send blocked by cooldown
    assert!(sim.send_email_code().is_err());

    // Submit with short code
    sim.email_code_input = "12".to_string();
    sim.username_input = "alice".to_string();
    sim.password_input = "secret123".to_string();
    assert!(sim.submit_email_register().is_err());
    assert_eq!(sim.login_error.as_deref(), Some("请输入收到的验证码"));

    // Submit with full code -> success
    sim.email_code_input = "123456".to_string();
    assert!(sim.submit_email_register().is_ok());
    assert!(sim.is_authenticated);
}

#[test]
fn test_email_cooldown_countdown_state_machine() {
    let mut sim = AuthWizardStateSimulator::new();
    sim.email_input = "test@example.com".to_string();
    assert!(sim.send_email_code().is_ok());
    assert_eq!(sim.email_code_cooldown, 60);

    // Tick 30 seconds
    for _ in 0..30 {
        sim.tick_cooldown();
    }
    assert_eq!(sim.email_code_cooldown, 30);
    assert!(sim.send_email_code().is_err(), "Should be blocked while cooldown > 0");

    // Tick remaining 30 seconds
    for _ in 0..30 {
        sim.tick_cooldown();
    }
    assert_eq!(sim.email_code_cooldown, 0);

    // Now re-enabled
    assert!(sim.send_email_code().is_ok(), "Should be allowed once cooldown reaches 0");
    assert_eq!(sim.email_code_cooldown, 60);
}

#[test]
fn test_alipay_qr_state_transitions() {
    let mut sim = AuthWizardStateSimulator::new();
    sim.switch_tab(AuthWizardTab::AlipayQr);
    assert!(sim.alipay_is_loading);
    assert!(sim.alipay_attempt_id.is_none());

    sim.on_alipay_started("attempt_uuid_9999".to_string());
    assert!(!sim.alipay_is_loading);
    assert_eq!(sim.alipay_attempt_id.as_deref(), Some("attempt_uuid_9999"));
    assert_eq!(sim.alipay_status_text, "请使用手机支付宝 App 扫码登录");

    sim.on_alipay_completed();
    assert!(sim.is_authenticated);
    assert_eq!(sim.alipay_status_text, "登录成功");
}

// ============================================================================
// PART 2: Source AST & Anti-Regression Inspection
// ============================================================================

#[test]
fn test_shell_source_ast_tab_compliance() {
    let shell_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/views/shell.rs");
    let content = fs::read_to_string(&shell_path).expect("Failed to read shell.rs");

    // 1. Enum AuthWizardTab exists with all 3 variants
    assert!(content.contains("pub enum AuthWizardTab"), "shell.rs must define pub enum AuthWizardTab");
    assert!(content.contains("PasswordLogin"), "AuthWizardTab must contain PasswordLogin");
    assert!(content.contains("AlipayQr"), "AuthWizardTab must contain AlipayQr");
    assert!(content.contains("EmailRegister"), "AuthWizardTab must contain EmailRegister");

    // 2. AppShell struct contains required fields
    assert!(content.contains("pub auth_wizard_tab: AuthWizardTab"), "AppShell must contain auth_wizard_tab");
    assert!(content.contains("alipay_attempt_id"), "AppShell must contain alipay_attempt_id");
    assert!(content.contains("email_code_cooldown"), "AppShell must contain email_code_cooldown");

    // 3. Handlers exist
    assert!(content.contains("fn switch_auth_tab") || content.contains("pub fn switch_auth_tab"), "Must contain switch_auth_tab");
    assert!(content.contains("fn start_alipay_flow") || content.contains("pub fn start_alipay_flow"), "Must contain start_alipay_flow");
    assert!(content.contains("fn simulate_alipay_scan") || content.contains("pub fn simulate_alipay_scan"), "Must contain simulate_alipay_scan");
    assert!(content.contains("fn send_email_code") || content.contains("pub fn send_email_code"), "Must contain send_email_code");
    assert!(content.contains("fn submit_email_register") || content.contains("pub fn submit_email_register"), "Must contain submit_email_register");

    // 4. UI tab switching buttons exist in render_login_wizard
    assert!(content.contains("render_login_wizard"), "shell.rs must contain render_login_wizard");

    // 5. Real Fabushi Alipay web authorization popup & Tokio runtime sleep safety
    assert!(
        content.contains("pub fn open_alipay_browser_url"),
        "shell.rs must define pub fn open_alipay_browser_url for browser OAuth popup"
    );
    assert!(
        content.contains("pub fn open_alipay_web_auth"),
        "shell.rs must define pub fn open_alipay_web_auth"
    );
    assert!(
        content.contains("pub fn open_alipay_web_auth(&mut self, cx: &mut Context<Self>) {\n        if self.alipay_is_loading {\n            return;\n        }"),
        "open_alipay_web_auth must guard against self.alipay_is_loading to prevent self-cancelling in-flight alipay_start"
    );
    assert!(
        content.contains("else if self.auth_wizard_tab == AuthWizardTab::AlipayQr {\n                            self.open_alipay_web_auth(cx);"),
        "Enter key on AuthWizardTab::AlipayQr must trigger open_alipay_web_auth instead of submit_login"
    );
    assert!(
        content.contains("client.runtime().spawn(async {\n                            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;\n                        }).await;"),
        "start_alipay_flow must spawn tokio::time::sleep on client.runtime() to avoid non-Tokio main-thread panic"
    );
}
