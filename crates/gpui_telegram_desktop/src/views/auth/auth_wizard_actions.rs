//! Action handlers, network auth workflows, keyboard dispatch, and IME `EntityInputHandler`
//! implementation for `AuthWizardView`.

use std::ops::Range;
use std::sync::Arc;

use gpui::*;
use gpui_telegram_client::{
    config::ClientConfig, net::http_client::EmailRegisterRequest, AuthResponse, HttpClientError,
    LoginRequest, RegisterRequest, TelegramClient,
};

use crate::components::ui::{
    delete_prev_word, pop_last_char, sanitize_paste_text, utf16_len, utf16_offset_to_utf8,
};

use super::auth_wizard_state::{
    next_login_field, validate_email_address, AuthWizardEvent, AuthWizardTab, LoginInputField,
    LoginWizardStep,
};
use super::auth_wizard_view::AuthWizardView;

impl AuthWizardView {
    fn loc(&self, zh: &str, en: &str) -> String {
        if self.is_chinese_locale { zh } else { en }.to_string()
    }

    fn login_field_mut(&mut self, field: LoginInputField) -> &mut String {
        match field {
            LoginInputField::Username => &mut self.username_input,
            LoginInputField::Password => &mut self.password_input,
            LoginInputField::FirstName => &mut self.first_name_input,
            LoginInputField::Email => &mut self.email_input,
            LoginInputField::EmailCode => &mut self.email_code_input,
        }
    }

    fn apply_auth_result(
        &mut self,
        res: Result<AuthResponse, HttpClientError>,
        err_msg: impl FnOnce(HttpClientError) -> String,
        cx: &mut Context<Self>,
    ) {
        self.is_authenticating = false;
        match res {
            Ok(auth) => self.emit_authenticated(auth, cx),
            Err(err) => {
                self.login_error = Some(err_msg(err));
                cx.notify();
            }
        }
    }

    pub(super) fn emit_authenticated(&mut self, auth: AuthResponse, cx: &mut Context<Self>) {
        self.is_authenticated = true;
        self.is_authenticating = false;
        self.login_error = None;
        self.ime.clear_preedit();
        self.ime.just_committed_ime = false;
        cx.emit(AuthWizardEvent::Authenticated(auth));
        cx.notify();
    }

    pub(super) fn clear_specific_login_field(&mut self, field: LoginInputField) {
        self.login_field_mut(field).clear();
    }

    pub fn submit_login(&mut self, cx: &mut Context<Self>) {
        let (username, password) = (self.username_input.trim().to_string(), self.password_input.clone());
        if username.is_empty() || password.is_empty() {
            self.login_error = Some("Username and password cannot be empty".to_string());
            cx.notify();
            return;
        }
        self.is_authenticating = true;
        self.login_error = None;
        cx.notify();

        let client = self.client.clone();
        let req = LoginRequest { username, password, device_name: "GPUI Desktop Client".to_string() };
        cx.spawn(async move |weak_wizard: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.login(&req).await;
            let _ = weak_wizard.update(app, |wizard, cx| {
                wizard.apply_auth_result(
                    res,
                    |err| match err {
                        HttpClientError::Unauthorized(_) => {
                            "Invalid username or password (401)".to_string()
                        }
                        other => other.to_string(),
                    },
                    cx,
                );
            });
        })
        .detach();
    }

    pub fn submit_register(&mut self, cx: &mut Context<Self>) {
        let username = self.username_input.trim().to_string();
        let password = self.password_input.clone();
        let first_name = if self.first_name_input.trim().is_empty() {
            username.clone()
        } else {
            self.first_name_input.trim().to_string()
        };
        if username.is_empty() || password.is_empty() {
            self.login_error = Some("Username and password cannot be empty".to_string());
            cx.notify();
            return;
        }
        self.is_authenticating = true;
        self.login_error = None;
        cx.notify();

        let client = self.client.clone();
        let req = RegisterRequest {
            username: username.clone(),
            password,
            first_name,
            device_name: "GPUI Desktop Client".to_string(),
        };
        cx.spawn(async move |weak_wizard: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.register(&req).await;
            let _ = weak_wizard.update(app, |wizard, cx| {
                wizard.apply_auth_result(
                    res,
                    |err| match err {
                        HttpClientError::Conflict(_) => {
                            format!("Username '{}' is already registered (409)", username)
                        }
                        other => other.to_string(),
                    },
                    cx,
                );
            });
        })
        .detach();
    }

    pub fn switch_auth_tab(&mut self, tab: AuthWizardTab, cx: &mut Context<Self>) {
        if self.auth_wizard_tab == tab {
            return;
        }
        self.auth_wizard_tab = tab;
        self.login_error = None;
        self.ime.clear_preedit();
        match tab {
            AuthWizardTab::PasswordLogin => self.active_login_field = LoginInputField::Username,
            AuthWizardTab::AlipayQr => self.start_alipay_flow(cx),
            AuthWizardTab::EmailRegister => self.active_login_field = LoginInputField::Email,
        }
        cx.notify();
    }

    pub fn apply_custom_server_url(&mut self, cx: &mut Context<Self>) {
        let raw = self.server_url_draft.trim().to_string();
        if raw.is_empty() {
            self.editing_server_url = false;
            cx.notify();
            return;
        }
        let normalized = ClientConfig::save_server_url(&raw)
            .unwrap_or_else(|_| ClientConfig::normalize_url(&raw));
        let cache_dir = self.client.local_store.cache_dir().map(|p| p.to_path_buf());
        self.client.local_store.clear_all();
        let mut new_cfg = self.client.config.clone();
        new_cfg.server_url = normalized.clone();
        self.client = Arc::new(match cache_dir {
            Some(dir) => TelegramClient::open_with_dir(new_cfg.clone(), dir)
                .unwrap_or_else(|_| TelegramClient::new(new_cfg)),
            None => TelegramClient::new(new_cfg),
        });
        self.server_url_draft = normalized.clone();
        self.editing_server_url = false;
        self.login_error = None;
        cx.emit(AuthWizardEvent::ServerUrlApplied(normalized.clone()));
        cx.emit(AuthWizardEvent::Toast {
            message: self.loc(
                &format!("已切换服务器: {}", normalized),
                &format!("Server URL updated: {}", normalized),
            ),
            duration_ms: 2000,
        });
        if self.auth_wizard_tab == AuthWizardTab::AlipayQr {
            self.start_alipay_flow(cx);
        } else {
            cx.notify();
        }
    }

    pub fn open_alipay_browser_url(url: &str) -> bool {
        if !(url.starts_with("https://") || url.starts_with("http://")) {
            return false;
        }
        #[cfg(target_os = "macos")]
        let cmd = std::process::Command::new("open").arg(url).spawn();
        #[cfg(target_os = "linux")]
        let cmd = std::process::Command::new("xdg-open").arg(url).spawn();
        #[cfg(target_os = "windows")]
        let cmd = std::process::Command::new("cmd").args(["/C", "start", "", url]).spawn();
        cmd.is_ok()
    }

    pub fn open_alipay_web_auth(&mut self, cx: &mut Context<Self>) {
        if self.alipay_is_loading {
            return;
        }
        if let Some(url) = self.alipay_auth_url.clone() {
            if !Self::open_alipay_browser_url(&url) {
                cx.emit(AuthWizardEvent::Toast {
                    message: self.loc(
                        "无法打开系统浏览器，请使用支付宝 App 扫码",
                        "Unable to open browser, please scan QR with Alipay",
                    ),
                    duration_ms: 2200,
                });
            }
            cx.notify();
        } else {
            self.start_alipay_flow(cx);
        }
    }

    pub fn start_alipay_flow(&mut self, cx: &mut Context<Self>) {
        self.alipay_is_loading = true;
        self.alipay_status_text = self.loc(
            "正在获取支付宝登录二维码...",
            "Fetching Alipay login QR code...",
        );
        self.alipay_attempt_id = None;
        self.alipay_auth_url = None;
        self.alipay_qr_payload = None;
        self.alipay_poll_generation += 1;
        let poll_gen = self.alipay_poll_generation;

        let client = self.client.clone();
        cx.spawn(async move |weak_wizard: WeakEntity<Self>, app: &mut AsyncApp| {
            let start_res = client.http_client.alipay_start().await;
            let mut launched_attempt_id: Option<String> = None;
            let ok = weak_wizard
                .update(app, |wizard, cx| {
                    if wizard.alipay_poll_generation != poll_gen
                        || wizard.auth_wizard_tab != AuthWizardTab::AlipayQr
                    {
                        return false;
                    }
                    wizard.alipay_is_loading = false;
                    match start_res {
                        Ok(resp) => {
                            let att_id = resp.attempt_id.clone();
                            launched_attempt_id = Some(att_id.clone());
                            wizard.alipay_attempt_id = Some(att_id);
                            wizard.alipay_auth_url = Some(resp.login_url.clone());
                            wizard.alipay_qr_payload = Some(resp.login_url.clone());
                            wizard.alipay_status_text = wizard.loc(
                                "请使用手机支付宝 App 扫码登录",
                                "Please scan with Alipay mobile app",
                            );
                            cx.notify();
                            true
                        }
                        Err(err) => {
                            wizard.alipay_status_text = format!("启动支付宝登录失败: {}", err);
                            wizard.login_error = Some(err.to_string());
                            cx.notify();
                            false
                        }
                    }
                })
                .unwrap_or(false);

            if ok {
                if let Some(att_id) = launched_attempt_id {
                    loop {
                        let _ = client.runtime().spawn(async {
                            tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
                        }).await;

                        let should_continue = weak_wizard
                            .update(app, |w, _| {
                                w.alipay_poll_generation == poll_gen
                                    && w.auth_wizard_tab == AuthWizardTab::AlipayQr
                                    && !w.is_authenticated
                                    && w.alipay_attempt_id.as_deref() == Some(&att_id)
                            })
                            .unwrap_or(false);
                        if !should_continue {
                            break;
                        }

                        let poll_res = client.http_client.alipay_poll(&att_id).await;
                        let done = weak_wizard
                            .update(app, |wizard, cx| {
                                if wizard.alipay_poll_generation != poll_gen
                                    || wizard.auth_wizard_tab != AuthWizardTab::AlipayQr
                                {
                                    return true;
                                }
                                let Ok(poll) = poll_res else {
                                    return false;
                                };
                                if poll.is_completed() {
                                    if let Some(session) = poll.session {
                                        wizard.emit_authenticated(session, cx);
                                        return true;
                                    } else if let Some(token) = poll.token {
                                        let u_ref = poll.user.as_ref();
                                        let auth = AuthResponse {
                                            user_id: poll.user_id.or_else(|| u_ref.map(|u| u.user_id)).unwrap_or(100003),
                                            username: poll.username.or_else(|| u_ref.and_then(|u| u.username.clone())).unwrap_or_else(|| "alipay_user".to_string()),
                                            session_id: format!("sess_alipay_{}", att_id),
                                            token,
                                            first_name: u_ref.map(|u| u.first_name.clone()).or_else(|| Some("支付宝用户".to_string())),
                                            created_at: None,
                                            expires_at: None,
                                        };
                                        wizard.emit_authenticated(auth, cx);
                                        return true;
                                    }
                                } else if poll.is_expired() {
                                    wizard.alipay_status_text = wizard.loc(
                                        "二维码已失效，请点击刷新",
                                        "QR Code expired, please refresh",
                                    );
                                    cx.notify();
                                    return true;
                                }
                                false
                            })
                            .unwrap_or(true);
                        if done {
                            break;
                        }
                    }
                }
            }
        })
        .detach();
    }

    pub fn simulate_alipay_scan(&mut self, cx: &mut Context<Self>) {
        let Some(attempt_id) = self.alipay_attempt_id.clone() else {
            return;
        };
        self.alipay_status_text =
            self.loc("正在模拟扫码授权...", "Simulating scan authorization...");
        cx.notify();

        let client = self.client.clone();
        cx.spawn(async move |weak_wizard: WeakEntity<Self>, app: &mut AsyncApp| {
            let sim_res = client.http_client.alipay_simulate_scan(&attempt_id).await;
            let _ = weak_wizard.update(app, |wizard, cx| match sim_res {
                Ok(_) => {
                    wizard.alipay_status_text = wizard.loc(
                        "模拟授权成功，正在同步会话...",
                        "Authorization approved, completing login...",
                    );
                    cx.notify();
                }
                Err(err) => {
                    wizard.login_error = Some(format!("模拟授权失败: {}", err));
                    cx.notify();
                }
            });
        })
        .detach();
    }

    pub fn send_email_code(&mut self, cx: &mut Context<Self>) {
        let email = self.email_input.trim().to_string();
        if !validate_email_address(&email) {
            self.login_error =
                Some(self.loc("请输入有效的邮箱地址", "Please enter a valid email address"));
            cx.notify();
            return;
        }
        if self.email_code_cooldown > 0 || self.is_sending_email_code {
            return;
        }

        self.is_sending_email_code = true;
        self.login_error = None;
        self.email_cooldown_generation += 1;
        let cd_gen = self.email_cooldown_generation;
        cx.notify();

        let client = self.client.clone();
        cx.spawn(async move |weak_wizard: WeakEntity<Self>, app: &mut AsyncApp| {
            let res = client.http_client.email_send_code(&email).await;
            let _ = weak_wizard.update(app, |wizard, cx| {
                wizard.is_sending_email_code = false;
                match res {
                    Ok(resp) => {
                        wizard.email_code_cooldown =
                            if resp.cooldown_seconds > 0 { resp.cooldown_seconds } else { 60 };
                        wizard.active_login_field = LoginInputField::EmailCode;
                        wizard.login_error = None;
                        cx.emit(AuthWizardEvent::Toast {
                            message: wizard.loc(
                                "验证码已发送至您的邮箱",
                                "Verification code sent to your email",
                            ),
                            duration_ms: 2200,
                        });
                        cx.notify();

                        let cooldown = wizard.email_code_cooldown;
                        let timer_client = client.clone();
                        cx.spawn(async move |weak_timer: WeakEntity<Self>, timer_app: &mut AsyncApp| {
                            for _ in 0..cooldown {
                                let _ = timer_client
                                    .runtime()
                                    .spawn(async {
                                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                                    })
                                    .await;
                                let should_continue = weak_timer
                                    .update(timer_app, |s, s_cx| {
                                        if s.email_cooldown_generation != cd_gen {
                                            return false;
                                        }
                                        if s.email_code_cooldown > 0 {
                                            s.email_code_cooldown -= 1;
                                            s_cx.notify();
                                            s.email_code_cooldown > 0
                                        } else {
                                            false
                                        }
                                    })
                                    .unwrap_or(false);
                                if !should_continue {
                                    break;
                                }
                            }
                        })
                        .detach();
                    }
                    Err(err) => {
                        wizard.login_error = Some(match err {
                            HttpClientError::Api { message, .. } => message,
                            other => other.to_string(),
                        });
                        cx.notify();
                    }
                }
            });
        })
        .detach();
    }

    pub fn submit_email_register(&mut self, cx: &mut Context<Self>) {
        let email = self.email_input.trim().to_string();
        let code = self.email_code_input.trim().to_string();
        let username = self.username_input.trim().to_string();
        let password = self.password_input.clone();
        let first_name = if self.first_name_input.trim().is_empty() {
            username.clone()
        } else {
            self.first_name_input.trim().to_string()
        };

        if email.is_empty() || !email.contains('@') {
            self.login_error =
                Some(self.loc("请输入有效的邮箱地址", "Please enter a valid email address"));
        } else if code.len() < 4 {
            self.login_error =
                Some(self.loc("请输入收到的验证码", "Please enter the verification code"));
        } else if username.is_empty() || password.is_empty() {
            self.login_error =
                Some(self.loc("用户名和密码不能为空", "Username and password cannot be empty"));
        } else {
            self.is_authenticating = true;
            self.login_error = None;
            cx.notify();

            let client = self.client.clone();
            let req = EmailRegisterRequest { email, code, username, password, first_name };
            cx.spawn(async move |weak_wizard: WeakEntity<Self>, app: &mut AsyncApp| {
                let res = client.http_client.email_register(&req).await;
                let _ = weak_wizard.update(app, |wizard, cx| {
                    wizard.apply_auth_result(
                        res,
                        |err| match err {
                            HttpClientError::Api { message, .. } => message,
                            other => other.to_string(),
                        },
                        cx,
                    );
                });
            })
            .detach();
            return;
        }
        cx.notify();
    }

    fn delete_token(target: &mut String, is_alt: bool) {
        if is_alt {
            delete_prev_word(target);
        } else {
            pop_last_char(target);
        }
    }

    pub fn active_text_and_preedit(&self) -> (String, Option<String>) {
        if self.editing_server_url {
            return self.ime.text_and_preedit(self.server_url_draft.clone());
        }
        match self.login_step {
            LoginWizardStep::Start => {
                let committed = match self.active_login_field {
                    LoginInputField::Username => &self.username_input,
                    LoginInputField::Password => &self.password_input,
                    LoginInputField::FirstName => &self.first_name_input,
                    LoginInputField::Email => &self.email_input,
                    LoginInputField::EmailCode => &self.email_code_input,
                };
                self.ime.text_and_preedit(committed.clone())
            }
            LoginWizardStep::Phone => (self.phone_number.clone(), None),
            LoginWizardStep::Code => (self.verification_code.clone(), None),
            LoginWizardStep::QrCode => (String::new(), None),
        }
    }

    pub fn clear_active_field(&mut self, cx: &mut Context<Self>) {
        self.ime.clear_preedit();
        if self.editing_server_url {
            self.server_url_draft.clear();
            cx.notify();
            return;
        }
        match self.login_step {
            LoginWizardStep::Start => self.clear_specific_login_field(self.active_login_field),
            LoginWizardStep::Phone => self.phone_number.clear(),
            LoginWizardStep::Code => self.verification_code.clear(),
            LoginWizardStep::QrCode => {}
        }
        cx.notify();
    }

    pub fn insert_committed_text(&mut self, raw_text: &str, cx: &mut Context<Self>) {
        self.ime.clear_preedit();
        let clean = sanitize_paste_text(raw_text, false);
        if clean.is_empty() {
            cx.notify();
            return;
        }
        if self.editing_server_url {
            self.server_url_draft.push_str(&clean);
            cx.notify();
            return;
        }

        match self.login_step {
            LoginWizardStep::Start => {
                let field = self.active_login_field;
                if field == LoginInputField::EmailCode {
                    for ch in clean.chars().filter(|c| c.is_ascii_digit()) {
                        if self.email_code_input.len() < 6 {
                            self.email_code_input.push(ch);
                        }
                    }
                } else {
                    self.login_field_mut(field).push_str(&clean);
                }
                cx.notify();
            }
            LoginWizardStep::Phone => {
                let digits: String =
                    clean.chars().filter(|c| c.is_ascii_digit() || *c == ' ' || *c == '-').collect();
                if !digits.is_empty() {
                    self.phone_number.push_str(&digits);
                    cx.notify();
                }
            }
            LoginWizardStep::QrCode => {}
            LoginWizardStep::Code => {
                for ch in clean.chars().filter(|c| c.is_ascii_digit()) {
                    if self.verification_code.len() < 6 {
                        self.verification_code.push(ch);
                    }
                }
                if self.verification_code.len() >= 5 {
                    self.submit_login(cx);
                }
                cx.notify();
            }
        }
    }

    pub fn handle_non_char_key(
        &mut self,
        key: &str,
        _is_shift: bool,
        is_alt: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        if key == "escape" {
            if self.editing_server_url {
                self.ime.clear_preedit();
                self.editing_server_url = false;
                self.server_url_draft = self.client.config.server_url.clone();
                cx.notify();
                return true;
            }
            if self.ime.ime_preedit.take().is_some() {
                cx.notify();
                return true;
            }
            return false;
        }

        if self.editing_server_url {
            match key {
                "enter" => {
                    if let Some(p) = self.ime.ime_preedit.take() {
                        self.server_url_draft.push_str(&sanitize_paste_text(&p, false));
                        cx.notify();
                    } else if self.ime.take_just_committed() {
                        cx.notify();
                    } else {
                        self.apply_custom_server_url(cx);
                    }
                    return true;
                }
                "backspace" => {
                    if self.ime.ime_preedit.take().is_none() {
                        Self::delete_token(&mut self.server_url_draft, is_alt);
                    }
                    cx.notify();
                    return true;
                }
                _ => return false,
            }
        }

        if self.is_authenticated {
            return false;
        }
        match self.login_step {
            LoginWizardStep::Start => match key {
                "enter" => {
                    if let Some(p) = self.ime.ime_preedit.take() {
                        self.insert_committed_text(&p, cx);
                    } else if self.ime.take_just_committed() {
                        cx.notify();
                    } else if self.auth_wizard_tab == AuthWizardTab::AlipayQr {
                            self.open_alipay_web_auth(cx);
                    } else if self.auth_wizard_tab == AuthWizardTab::EmailRegister {
                        self.submit_email_register(cx);
                    } else if self.is_register_mode {
                        self.submit_register(cx);
                    } else {
                        self.submit_login(cx);
                    }
                    true
                }
                "tab" => {
                    self.ime.clear_preedit();
                    self.active_login_field = next_login_field(
                        self.auth_wizard_tab,
                        self.active_login_field,
                        self.is_register_mode,
                    );
                    cx.notify();
                    true
                }
                "backspace" => {
                    if self.ime.ime_preedit.take().is_none() {
                        let field = self.active_login_field;
                        Self::delete_token(self.login_field_mut(field), is_alt);
                    }
                    cx.notify();
                    true
                }
                _ => false,
            },
            LoginWizardStep::Phone => match key {
                "enter" => {
                    if !self.phone_number.trim().is_empty() {
                        self.login_step = LoginWizardStep::Code;
                        self.login_error = None;
                    }
                    cx.notify();
                    true
                }
                "backspace" => {
                    Self::delete_token(&mut self.phone_number, is_alt);
                    cx.notify();
                    true
                }
                _ => false,
            },
            LoginWizardStep::QrCode if key == "enter" => {
                self.submit_login(cx);
                true
            }
            LoginWizardStep::Code if key == "enter" => {
                self.submit_login(cx);
                true
            }
            LoginWizardStep::Code if key == "backspace" => {
                pop_last_char(&mut self.verification_code);
                cx.notify();
                true
            }
            _ => false,
        }
    }

    pub fn handle_shortcut_key(&mut self, key: &str, cx: &mut Context<Self>) -> bool {
        match key.to_ascii_lowercase().as_str() {
            "v" => {
                if let Some(clip_text) = cx.read_from_clipboard().and_then(|item| item.text()) {
                    let clean = sanitize_paste_text(&clip_text, false);
                    self.insert_committed_text(&clean, cx);
                }
                true
            }
            "backspace" | "a" | "u" => {
                self.clear_active_field(cx);
                true
            }
            "d" => {
                cx.emit(AuthWizardEvent::ToggleTheme);
                true
            }
            "l" => {
                cx.emit(AuthWizardEvent::ToggleLanguage);
                true
            }
            _ => false,
        }
    }

    pub fn handle_window_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let key = event.keystroke.key.as_str();
        if event.keystroke.modifiers.platform || event.keystroke.modifiers.control {
            if self.handle_shortcut_key(key, cx) {
                cx.stop_propagation();
            }
            return;
        }
        if self.handle_non_char_key(key, event.keystroke.modifiers.shift, event.keystroke.modifiers.alt, cx) {
            cx.stop_propagation();
        }
    }

    pub fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
        let key = event.keystroke.key.as_str();
        if event.keystroke.modifiers.platform || event.keystroke.modifiers.control {
            self.handle_shortcut_key(key, cx);
            return;
        }
        if self.handle_non_char_key(key, event.keystroke.modifiers.shift, event.keystroke.modifiers.alt, cx) {
            return;
        }
        if key == "space" {
            self.insert_committed_text(" ", cx);
        } else if let Some(ref ch) = event.keystroke.key_char {
            self.insert_committed_text(ch, cx);
        }
    }
}

impl EntityInputHandler for AuthWizardView {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        _adjusted_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let (committed, preedit) = self.active_text_and_preedit();
        let full = preedit.map_or(committed.clone(), |p| format!("{}{}", committed, p));
        let start = utf16_offset_to_utf8(&full, range_utf16.start);
        let end = utf16_offset_to_utf8(&full, range_utf16.end).max(start);
        Some(full.get(start..end).unwrap_or("").to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let (committed, preedit) = self.active_text_and_preedit();
        let range = if preedit.is_some() {
            self.ime.selected_utf16_range(&committed)?
        } else {
            let total = utf16_len(&committed);
            total..total
        };
        Some(UTF16Selection { range, reversed: false })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        let (committed, preedit) = self.active_text_and_preedit();
        preedit.and_then(|_| self.ime.marked_utf16_range(&committed))
    }

    fn unmark_text(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.ime.just_committed_ime = true;
        if let Some(p) = self.ime.ime_preedit.take().filter(|s| !s.is_empty()) {
            self.insert_committed_text(&p, cx);
            return;
        }
        cx.notify();
    }

    fn replace_text_in_range(
        &mut self,
        _range_utf16: Option<Range<usize>>,
        text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let was_composing = self.ime.has_preedit();
        self.insert_committed_text(text, cx);
        self.ime.just_committed_ime = was_composing;
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        _range_utf16: Option<Range<usize>>,
        new_text: &str,
        _new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(committed) = self.ime.handle_replace_and_mark(new_text) {
            self.insert_committed_text(&committed, cx);
        } else {
            cx.notify();
        }
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: Range<usize>,
        element_bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let center_x = element_bounds.origin.x + element_bounds.size.width / 2.0;
        let center_y = element_bounds.origin.y + element_bounds.size.height / 2.0;
        let (committed, preedit) = self.active_text_and_preedit();
        let char_count = committed.chars().count() + preedit.as_deref().map_or(0, |p| p.chars().count());
        let caret_x = center_x - px(140.0) + px((char_count as f32 * 8.0).min(280.0));
        Some(Bounds::new(point(caret_x, center_y + px(10.0)), size(px(2.0), px(20.0))))
    }

    fn character_index_for_point(
        &mut self,
        _point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let (committed, preedit) = self.active_text_and_preedit();
        Some(utf16_len(&committed) + preedit.as_deref().map_or(0, utf16_len))
    }
}
