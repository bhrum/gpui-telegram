//! Settings Premium View and State Machine
//!
//! Provides the complete state machine, business logic, and UI representation
//! for Telegram Premium membership purchase (Alipay RSA2), redeem code activation,
//! quota comparisons, and active limit multipliers.

use gpui_telegram_client::net::http_client::{
    CreateAlipayOrderResponse, HttpClient, HttpClientError, UseRedeemCodeResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Telegram Premium feature quotas and limits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PremiumLimits {
    pub max_channels: usize,
    pub max_pinned_dialogs: usize,
    pub max_upload_size_mb: usize,
    pub max_accounts: usize,
    pub voice_to_text: bool,
    pub star_badge: bool,
    pub doubled_limits: bool,
}

impl Default for PremiumLimits {
    fn default() -> Self {
        Self::standard()
    }
}

impl PremiumLimits {
    /// Free/Standard user limits
    pub fn standard() -> Self {
        Self {
            max_channels: 500,
            max_pinned_dialogs: 5,
            max_upload_size_mb: 2048, // 2 GB
            max_accounts: 3,
            voice_to_text: false,
            star_badge: false,
            doubled_limits: false,
        }
    }

    /// Premium member limits (2x capacity & exclusive features)
    pub fn premium() -> Self {
        Self {
            max_channels: 1000,
            max_pinned_dialogs: 10,
            max_upload_size_mb: 4096, // 4 GB
            max_accounts: 6,
            voice_to_text: true,
            star_badge: true,
            doubled_limits: true,
        }
    }
}

/// Catalog plan item
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PremiumPlanOption {
    pub id: &'static str,
    pub name: &'static str,
    pub duration_days: u32,
    pub price_cny: &'static str,
    pub monthly_equivalent: &'static str,
    pub discount_tag: Option<&'static str>,
}

pub const AVAILABLE_PLANS: &[PremiumPlanOption] = &[
    PremiumPlanOption {
        id: "monthly",
        name: "Monthly Plan",
        duration_days: 30,
        price_cny: "21.00",
        monthly_equivalent: "21.00",
        discount_tag: None,
    },
    PremiumPlanOption {
        id: "quarterly",
        name: "Quarterly Plan",
        duration_days: 90,
        price_cny: "63.00",
        monthly_equivalent: "21.00",
        discount_tag: None,
    },
    PremiumPlanOption {
        id: "yearly",
        name: "Annual Plan",
        duration_days: 365,
        price_cny: "252.00",
        monthly_equivalent: "21.00",
        discount_tag: Some("BEST VALUE - Save 17%"),
    },
];

/// State machine for Premium Settings UI
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PremiumViewState {
    Loading,
    FreeUser {
        selected_plan: String,
        redeem_code_input: String,
        is_submitting: bool,
        error_message: Option<String>,
    },
    Purchasing {
        order_id: String,
        payment_url: String,
        order_string: Option<String>,
        polling_active: bool,
        status_message: String,
    },
    PremiumActive {
        plan_type: String,
        days_left: u32,
        expires_at: Option<String>,
        limits: PremiumLimits,
    },
    Error(String),
}

/// Controller managing the Premium Settings view lifecycle
pub struct SettingsPremiumController {
    http_client: Arc<HttpClient>,
    token: String,
    state: PremiumViewState,
}

impl SettingsPremiumController {
    pub fn new(http_client: Arc<HttpClient>, token: String) -> Self {
        Self {
            http_client,
            token,
            state: PremiumViewState::Loading,
        }
    }

    pub fn state(&self) -> &PremiumViewState {
        &self.state
    }

    pub fn set_state(&mut self, state: PremiumViewState) {
        self.state = state;
    }

    /// Fetches membership status from server and initializes view state
    pub async fn load_status(&mut self) -> Result<&PremiumViewState, HttpClientError> {
        self.state = PremiumViewState::Loading;
        match self.http_client.check_membership(&self.token).await {
            Ok(resp) => {
                if resp.is_premium {
                    self.state = PremiumViewState::PremiumActive {
                        plan_type: resp.plan,
                        days_left: resp.days_left,
                        expires_at: resp.expires_at_ms.map(|ms| {
                            chrono::DateTime::from_timestamp_millis(ms)
                                .map(|dt| dt.to_rfc3339())
                                .unwrap_or_else(|| ms.to_string())
                        }),
                        limits: PremiumLimits::premium(),
                    };
                } else {
                    self.state = PremiumViewState::FreeUser {
                        selected_plan: "monthly".into(),
                        redeem_code_input: String::new(),
                        is_submitting: false,
                        error_message: None,
                    };
                }
                Ok(&self.state)
            }
            Err(e) => {
                let err_msg = e.to_string();
                self.state = PremiumViewState::Error(err_msg);
                Err(e)
            }
        }
    }

    /// Selects a subscription plan
    pub fn select_plan(&mut self, plan_id: &str) {
        if let PremiumViewState::FreeUser { selected_plan, .. } = &mut self.state {
            *selected_plan = plan_id.to_string();
        }
    }

    /// Sets the redeem code input buffer
    pub fn set_redeem_code_input(&mut self, code: &str) {
        if let PremiumViewState::FreeUser { redeem_code_input, .. } = &mut self.state {
            *redeem_code_input = code.trim().to_uppercase();
        }
    }

    /// Submits a redeem code for instant activation
    pub async fn submit_redeem_code(&mut self) -> Result<UseRedeemCodeResponse, HttpClientError> {
        let code = match &self.state {
            PremiumViewState::FreeUser { redeem_code_input, .. } => redeem_code_input.clone(),
            _ => {
                return Err(HttpClientError::Api {
                    status: 400,
                    message: "Invalid state for redeem code submission".into(),
                });
            }
        };

        if code.len() < 8 || code.len() > 64 {
            let msg = "Redeem code must be between 8 and 64 characters".to_string();
            if let PremiumViewState::FreeUser { error_message, .. } = &mut self.state {
                *error_message = Some(msg.clone());
            }
            return Err(HttpClientError::Api {
                status: 400,
                message: msg,
            });
        }

        if let PremiumViewState::FreeUser { is_submitting, .. } = &mut self.state {
            *is_submitting = true;
        }

        match self.http_client.use_redeem_code(&self.token, &code).await {
            Ok(resp) => {
                // Immediately transition to active
                self.state = PremiumViewState::PremiumActive {
                    plan_type: resp.plan.clone(),
                    days_left: resp.duration_days,
                    expires_at: chrono::DateTime::from_timestamp_millis(resp.expires_at)
                        .map(|dt| dt.to_rfc3339()),
                    limits: PremiumLimits::premium(),
                };
                Ok(resp)
            }
            Err(e) => {
                let msg = e.to_string();
                if let PremiumViewState::FreeUser { is_submitting, error_message, .. } = &mut self.state {
                    *is_submitting = false;
                    *error_message = Some(msg);
                }
                Err(e)
            }
        }
    }

    /// Starts Alipay payment checkout
    pub async fn start_alipay_order(&mut self, platform: &str) -> Result<CreateAlipayOrderResponse, HttpClientError> {
        let plan = match &self.state {
            PremiumViewState::FreeUser { selected_plan, .. } => selected_plan.clone(),
            _ => "monthly".into(),
        };

        match self.http_client.create_alipay_order(&self.token, &plan, platform).await {
            Ok(resp) => {
                let payment_url = resp.payment_url.clone().unwrap_or_default();
                self.state = PremiumViewState::Purchasing {
                    order_id: resp.order_id.clone(),
                    payment_url,
                    order_string: resp.order_string.clone(),
                    polling_active: true,
                    status_message: "Waiting for payment confirmation...".into(),
                };
                Ok(resp)
            }
            Err(e) => {
                let msg = e.to_string();
                if let PremiumViewState::FreeUser { error_message, .. } = &mut self.state {
                    *error_message = Some(msg);
                }
                Err(e)
            }
        }
    }

    /// Polls Alipay order status. If paid, updates state to PremiumActive.
    pub async fn poll_payment_status(&mut self) -> Result<bool, HttpClientError> {
        let order_id = match &self.state {
            PremiumViewState::Purchasing { order_id, .. } => order_id.clone(),
            _ => return Ok(false),
        };

        let query = self.http_client.query_alipay_order(&self.token, &order_id).await?;
        if query.status == "PAID" {
            // Re-fetch membership status to update expiration and active limits
            let mem = self.http_client.check_membership(&self.token).await?;
            self.state = PremiumViewState::PremiumActive {
                plan_type: mem.plan,
                days_left: mem.days_left,
                expires_at: mem.expires_at_ms.map(|ms| {
                    chrono::DateTime::from_timestamp_millis(ms)
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_else(|| ms.to_string())
                }),
                limits: PremiumLimits::premium(),
            };
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Header badge gradient styling constants
    pub fn badge_gradient_colors() -> (&'static str, &'static str) {
        ("#6C5CE7", "#A29BFE")
    }

    /// Quota comparison rows for presentation rendering
    pub fn quota_comparisons() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("File Upload Size", "2 GB", "4 GB"),
            ("Channel Capacity", "500", "1,000"),
            ("Pinned Dialogs", "5", "10"),
            ("Multi-Account Slots", "3", "6"),
            ("Voice-to-Text Transcription", "Disabled", "Unlocked"),
            ("Premium Star Badge", "No", "Yes"),
        ]
    }
}
