// Source: Telegram/SourceFiles/settings/cloud_password/settings_cloud_password_step.cpp
// Subsystem: settings | Feature ID: TCG-N02
// Upstream: AbstractStep::showFinishes, AbstractStep::showFinished, AbstractStep::isPasswordInvalidError

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudPasswordStepId {
    Start,
    EnterPassword,
    ConfirmPassword,
    CreateHint,
    RecoveryEmail,
    EmailConfirm,
    Done,
}

#[derive(Debug, Clone)]
pub struct SettingsCloudPasswordStepCloudPassword {
    pub current_step: CloudPasswordStepId,
    pub step_index: usize,
    pub total_steps: usize,
    pub is_finishing: bool,
    pub error_text: Option<String>,
}

impl SettingsCloudPasswordStepCloudPassword {
    pub fn new(initial_step: CloudPasswordStepId) -> Self {
        let (step_index, total_steps) = match initial_step {
            CloudPasswordStepId::Start => (0, 5),
            CloudPasswordStepId::EnterPassword => (1, 5),
            CloudPasswordStepId::ConfirmPassword => (2, 5),
            CloudPasswordStepId::CreateHint => (3, 5),
            CloudPasswordStepId::RecoveryEmail => (4, 5),
            CloudPasswordStepId::EmailConfirm => (5, 5),
            CloudPasswordStepId::Done => (5, 5),
        };
        Self {
            current_step: initial_step,
            step_index,
            total_steps,
            is_finishing: false,
            error_text: None,
        }
    }

    pub fn advance_step(&mut self) {
        match self.current_step {
            CloudPasswordStepId::Start => {
                self.current_step = CloudPasswordStepId::EnterPassword;
                self.step_index = 1;
            }
            CloudPasswordStepId::EnterPassword => {
                self.current_step = CloudPasswordStepId::ConfirmPassword;
                self.step_index = 2;
            }
            CloudPasswordStepId::ConfirmPassword => {
                self.current_step = CloudPasswordStepId::CreateHint;
                self.step_index = 3;
            }
            CloudPasswordStepId::CreateHint => {
                self.current_step = CloudPasswordStepId::RecoveryEmail;
                self.step_index = 4;
            }
            CloudPasswordStepId::RecoveryEmail => {
                self.current_step = CloudPasswordStepId::EmailConfirm;
                self.step_index = 5;
            }
            CloudPasswordStepId::EmailConfirm => {
                self.current_step = CloudPasswordStepId::Done;
                self.is_finishing = true;
            }
            CloudPasswordStepId::Done => {}
        }
        self.error_text = None;
    }

    pub fn go_back(&mut self) -> bool {
        match self.current_step {
            CloudPasswordStepId::Start => false,
            CloudPasswordStepId::EnterPassword => {
                self.current_step = CloudPasswordStepId::Start;
                self.step_index = 0;
                true
            }
            CloudPasswordStepId::ConfirmPassword => {
                self.current_step = CloudPasswordStepId::EnterPassword;
                self.step_index = 1;
                true
            }
            CloudPasswordStepId::CreateHint => {
                self.current_step = CloudPasswordStepId::ConfirmPassword;
                self.step_index = 2;
                true
            }
            CloudPasswordStepId::RecoveryEmail => {
                self.current_step = CloudPasswordStepId::CreateHint;
                self.step_index = 3;
                true
            }
            CloudPasswordStepId::EmailConfirm => {
                self.current_step = CloudPasswordStepId::RecoveryEmail;
                self.step_index = 4;
                true
            }
            CloudPasswordStepId::Done => false,
        }
    }

    pub fn set_error(&mut self, error: impl Into<String>) {
        self.error_text = Some(error.into());
    }

    pub fn clear_error(&mut self) {
        self.error_text = None;
    }

    pub fn progress_pct(&self) -> f32 {
        if self.total_steps == 0 {
            0.0
        } else {
            (self.step_index as f32 / self.total_steps as f32).clamp(0.0, 1.0)
        }
    }
}

impl Default for SettingsCloudPasswordStepCloudPassword {
    fn default() -> Self {
        Self::new(CloudPasswordStepId::EnterPassword)
    }
}

impl Render for SettingsCloudPasswordStepCloudPassword {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut root = div()
            .flex()
            .flex_col()
            .size_full()
            .p(px(20.0))
            .gap(px(12.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x707579))
                            .child(format!("Step {} of {}", self.step_index, self.total_steps))
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x2481cc))
                            .child(format!("{:.0}%", self.progress_pct() * 100.0))
                    )
            )
            .child(
                div()
                    .w_full()
                    .h(px(4.0))
                    .bg(rgb(0xe0e0e0))
                    .rounded_full()
                    .child(
                        div()
                            .w(px(320.0 * self.progress_pct()))
                            .h_full()
                            .bg(rgb(0x2481cc))
                            .rounded_full()
                    )
            );
        if let Some(err) = self.error_text.as_ref() {
            root = root.child(
                div()
                    .p(px(8.0))
                    .rounded(px(6.0))
                    .bg(rgb(0xffebee))
                    .text_color(rgb(0xd32f2f))
                    .text_xs()
                    .child(err.clone())
            );
        }
        root
    }
}
