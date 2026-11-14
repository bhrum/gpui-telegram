use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusResponse {
    pub server: String,
    pub status: String,
    pub version: String,
    #[serde(default)]
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user_id: u64,
    pub username: String,
    pub session_id: String,
    pub token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_generation: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneSendCodeRequest {
    pub phone: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneSendCodeResponse {
    pub ok: bool,
    pub sent: bool,
    pub phone_code_hash: String,
    pub expires_at_ms: i64,
    pub resend_after_ms: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneVerifyRequest {
    pub phone: String,
    #[serde(default)]
    pub phone_code_hash: Option<String>,
    pub code: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub device_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoFactorStatusResponse {
    pub has_password: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_email_pattern: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoFactorSetRequest {
    #[serde(default)]
    pub current_password: Option<String>,
    pub new_password: String,
    #[serde(default)]
    pub hint: Option<String>,
    #[serde(default)]
    pub recovery_email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwoFactorVerifyRequest {
    pub challenge_token: String,
    pub cloud_password: String,
    #[serde(default)]
    pub device_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QrPairInitResponse {
    pub pair_token: String,
    pub qr_url: String,
    pub expires_at_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QrPairAcceptRequest {
    pub pair_token: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QrPairPollResponse {
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pair_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<AuthResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserProfileDto>,
}

pub type QrPairStatusResponse = QrPairPollResponse;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionDto {
    pub session_id: String,
    pub device_name: String,
    #[serde(default)]
    pub platform: Option<String>,
    #[serde(default)]
    pub ip_address: Option<String>,
    pub created_at_ms: i64,
    pub last_active_ms: i64,
    pub is_current: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default, alias = "photo_file_id")]
    pub avatar_media_id: Option<String>,
    #[serde(default)]
    pub emoji_status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfileDto {
    pub user_id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    pub first_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_media_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photo_file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji_status: Option<String>,
    #[serde(default)]
    pub is_bot: bool,
    #[serde(default)]
    pub is_verified: bool,
    #[serde(default)]
    pub is_premium: bool,
    #[serde(default)]
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_ms: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContactDto {
    pub user_id: u64,
    pub first_name: String,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub is_bot: bool,
    #[serde(default)]
    pub is_mutual: bool,
    #[serde(default)]
    pub chat_id: Option<i64>,
}
