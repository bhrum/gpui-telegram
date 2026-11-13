//! Challenger M2 Empirical Stress Tests: Desktop UI Countdown State Machine
//! Target: `crates/gpui_telegram_desktop/tests/challenger_m2_desktop_countdown_tests.rs`

use std::fs;
use std::path::Path;
use gpui_telegram_desktop::views::shell::{AuthWizardTab, LoginInputField};

/// Direct replica of AppShell's email cooldown and timer state machine
#[derive(Debug, Clone)]
pub struct DesktopCountdownFsm {
    pub active_tab: AuthWizardTab,
    pub active_field: LoginInputField,
    pub email_input: String,
    pub email_code_input: String,
    pub email_code_cooldown: u32,
    pub email_cooldown_generation: u64,
    pub is_sending_code: bool,
    pub login_error: Option<String>,
}

impl DesktopCountdownFsm {
    pub fn new() -> Self {
        Self {
            active_tab: AuthWizardTab::EmailRegister,
            active_field: LoginInputField::Email,
            email_input: String::new(),
            email_code_input: String::new(),
            email_code_cooldown: 0,
            email_cooldown_generation: 0,
            is_sending_code: false,
            login_error: None,
        }
    }

    pub fn button_label(&self, zh: bool) -> String {
        if self.is_sending_code {
            "发送中...".to_string()
        } else if self.email_code_cooldown > 0 {
            format!("重新获取 ({}s)", self.email_code_cooldown)
        } else if zh {
            "获取验证码".to_string()
        } else {
            "Send Code".to_string()
        }
    }

    pub fn is_button_enabled(&self) -> bool {
        self.email_code_cooldown == 0 && !self.is_sending_code
    }

    pub fn send_email_code(&mut self) -> Result<u64, &'static str> {
        let email = self.email_input.trim();
        if email.is_empty() || !email.contains('@') || !email.contains('.') {
            self.login_error = Some("请输入有效邮箱地址".to_string());
            return Err("Invalid email");
        }
        if self.email_code_cooldown > 0 || self.is_sending_code {
            return Err("Cooldown active or already sending");
        }

        self.email_cooldown_generation += 1;
        let generation = self.email_cooldown_generation;
        self.email_code_cooldown = 60;
        self.active_field = LoginInputField::EmailCode;
        self.login_error = None;
        Ok(generation)
    }

    pub fn tick_second(&mut self, timer_generation: u64) -> bool {
        // Discard stale timer ticks if generation has bumped
        if timer_generation != self.email_cooldown_generation {
            return false;
        }

        if self.email_code_cooldown > 0 {
            self.email_code_cooldown -= 1;
            self.email_code_cooldown > 0
        } else {
            false
        }
    }
}

#[test]
fn test_desktop_countdown_state_machine_second_by_second() {
    let mut fsm = DesktopCountdownFsm::new();

    // 1. Initial State: cooldown 0, button enabled with label "获取验证码"
    assert_eq!(fsm.email_code_cooldown, 0);
    assert!(fsm.is_button_enabled());
    assert_eq!(fsm.button_label(true), "获取验证码");
    assert_eq!(fsm.button_label(false), "Send Code");

    // 2. Trigger send-code
    fsm.email_input = "challenger_tester@example.com".to_string();
    let generation = fsm.send_email_code().expect("send_email_code must succeed");
    assert_eq!(fsm.email_code_cooldown, 60);
    assert!(!fsm.is_button_enabled(), "Button must be disabled while cooldown > 0");
    assert_eq!(fsm.button_label(true), "重新获取 (60s)");
    assert_eq!(fsm.active_field, LoginInputField::EmailCode);

    // 3. Duplicate trigger while cooldown active MUST be rejected
    assert!(fsm.send_email_code().is_err(), "Cannot send code while cooldown active");

    // 4. Decrement second by second from 60 down to 1
    for remaining in (1..=60).rev() {
        assert_eq!(fsm.email_code_cooldown, remaining);
        assert_eq!(fsm.button_label(true), format!("重新获取 ({}s)", remaining));
        assert!(!fsm.is_button_enabled());

        let still_active = fsm.tick_second(generation);
        if remaining > 1 {
            assert!(still_active, "Timer should indicate more ticks remain");
            assert_eq!(fsm.email_code_cooldown, remaining - 1);
        } else {
            assert!(!still_active, "Timer should complete when hitting 0");
            assert_eq!(fsm.email_code_cooldown, 0);
        }
    }

    // 5. Cooldown reaches 0: button re-enables with label "获取验证码"
    assert_eq!(fsm.email_code_cooldown, 0);
    assert!(fsm.is_button_enabled(), "Button must be re-enabled at 0");
    assert_eq!(fsm.button_label(true), "获取验证码");
    assert_eq!(fsm.button_label(false), "Send Code");

    // 6. Sending code is allowed again once cooldown reaches 0
    let generation2 = fsm.send_email_code().expect("Send code must be permitted when cooldown is 0");
    assert_eq!(generation2, generation + 1);
    assert_eq!(fsm.email_code_cooldown, 60);
    assert_eq!(fsm.button_label(true), "重新获取 (60s)");
}

#[test]
fn test_desktop_cooldown_generation_fence_isolation() {
    let mut fsm = DesktopCountdownFsm::new();
    fsm.email_input = "gen_test@example.com".to_string();

    // Start generation 1
    let generation1 = fsm.send_email_code().unwrap();
    assert_eq!(fsm.email_code_cooldown, 60);

    // Tick 5 times
    for _ in 0..5 {
        fsm.tick_second(generation1);
    }
    assert_eq!(fsm.email_code_cooldown, 55);

    // External reset or tab-switch triggers new send code (gen 2)
    fsm.email_code_cooldown = 0; // simulated forced reset
    let generation2 = fsm.send_email_code().unwrap();
    assert_eq!(generation2, generation1 + 1);
    assert_eq!(fsm.email_code_cooldown, 60);

    // Zombie timer from gen1 attempts to tick -> must be discarded and return false
    let result = fsm.tick_second(generation1);
    assert!(!result, "Tick from stale generation must be rejected");
    assert_eq!(fsm.email_code_cooldown, 60, "Cooldown must not be decremented by stale generation");

    // Tick from gen2 succeeds
    let result2 = fsm.tick_second(generation2);
    assert!(result2);
    assert_eq!(fsm.email_code_cooldown, 59);
}

#[test]
fn test_shell_source_ast_countdown_state_machine_compliance() {
    let shell_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/views/shell.rs");
    let content = fs::read_to_string(&shell_path).expect("Failed to read shell.rs");

    // 1. AppShell struct contains email_code_cooldown and email_cooldown_generation
    assert!(
        content.contains("pub email_code_cooldown: u32,"),
        "shell.rs must declare pub email_code_cooldown: u32"
    );
    assert!(
        content.contains("pub email_cooldown_generation: u64,"),
        "shell.rs must declare pub email_cooldown_generation: u64"
    );

    // 2. send_email_code enforces cooldown fence
    assert!(
        content.contains("if self.email_code_cooldown > 0 || self.is_sending_email_code {"),
        "send_email_code must block execution when email_code_cooldown > 0"
    );

    // 3. Ticker loop sleeps 1 second and decrements
    assert!(
        content.contains("tokio::time::sleep(std::time::Duration::from_secs(1)).await;"),
        "Countdown ticker must sleep 1 second per iteration"
    );
    assert!(
        content.contains("s.email_code_cooldown -= 1;"),
        "Countdown ticker must decrement email_code_cooldown"
    );
    assert!(
        content.contains("if s.email_cooldown_generation != cd_gen {"),
        "Countdown ticker must check generation to avoid stale timer races"
    );

    // 4. UI renders countdown vs static button label
    assert!(
        content.contains(r#"format!("重新获取 ({}s)", cooldown)"#),
        "UI must display formatted remaining seconds when cooldown > 0"
    );
    assert!(
        content.contains(r#""获取验证码""#),
        "UI must display '获取验证码' when cooldown == 0"
    );
}
