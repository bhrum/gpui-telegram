use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub media_storage_path: String,
    pub session_timeout_secs: u64,
    pub ws_heartbeat_interval_secs: u64,
    pub ws_inactivity_timeout_secs: u64,
    #[serde(default = "default_agent_model")]
    pub default_agent_model: String,
}

fn default_agent_model() -> String {
    "grok-3".into()
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 8080,
            database_url: "sqlite://data/gpui_telegram.db".into(),
            media_storage_path: "data/media".into(),
            session_timeout_secs: 30 * 24 * 3600, // 30 days
            ws_heartbeat_interval_secs: 30,
            ws_inactivity_timeout_secs: 60,
            default_agent_model: "grok-3".into(),
        }
    }
}
