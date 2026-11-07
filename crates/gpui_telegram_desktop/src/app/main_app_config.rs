//! Dynamic server application configuration dictionary
//! Upstream: Telegram/SourceFiles/main/main_app_config.h, main_app_config.cpp
//! Subsystem: main | Feature ID: TCG-N46 | Disposition: gpui_ui_1to1

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Remote application configuration loaded via MTProto `help.getAppConfig`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainAppConfig {
    pub values: HashMap<String, serde_json::Value>,
    pub hash: i32,
    pub last_refresh: i64,
}

impl Default for MainAppConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl MainAppConfig {
    /// Creates a new config repository with upstream default values.
    pub fn new() -> Self {
        let mut cfg = Self {
            values: HashMap::new(),
            hash: 0,
            last_refresh: 0,
        };
        cfg.init_defaults();
        cfg
    }

    fn init_defaults(&mut self) {
        self.set_i64("quote_length_max", 1024);
        self.set_i64("stargift_convert_period_max", 7776000); // 90 days
        self.set_i64("no_forwards_request_expire_period", 86400);
        self.set_bool("starref_setup_allowed", true);
        self.set_bool("starref_join_allowed", true);
        self.set_i64("starref_commission_min", 0);
        self.set_i64("starref_commission_max", 100);
        self.set_i64("stars_withdraw_max", 1000000);
        self.set_f64("stars_withdraw_rate", 0.013);
        self.set_f64("currency_withdraw_rate", 1.0);
        self.set_f64("stars_sell_rate", 0.013);
        self.set_i64("poll_options_limit", 10);
        self.set_i64("todo_list_items_limit", 30);
        self.set_i64("confcall_size_limit", 1000);
        self.set_str("phone_country_iso2", "US");
        self.set_bool("wallet_available", true);
        self.set_bool("paid_messages_available", true);
        self.set_i64("paid_message_stars_max", 10000);
    }

    /// Retrieves an integer configuration value with fallback.
    pub fn get_i64(&self, key: &str, fallback: i64) -> i64 {
        self.values
            .get(key)
            .and_then(|v| v.as_i64())
            .unwrap_or(fallback)
    }

    /// Retrieves a floating-point configuration value with fallback.
    pub fn get_f64(&self, key: &str, fallback: f64) -> f64 {
        self.values
            .get(key)
            .and_then(|v| v.as_f64())
            .unwrap_or(fallback)
    }

    /// Retrieves a boolean configuration value with fallback.
    pub fn get_bool(&self, key: &str, fallback: bool) -> bool {
        self.values
            .get(key)
            .and_then(|v| v.as_bool())
            .unwrap_or(fallback)
    }

    /// Retrieves a string configuration value with fallback.
    pub fn get_string(&self, key: &str, fallback: &str) -> String {
        self.values
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| fallback.to_string())
    }

    /// Sets an integer configuration key.
    pub fn set_i64(&mut self, key: &str, val: i64) {
        self.values.insert(key.to_string(), serde_json::Value::from(val));
    }

    /// Sets a floating point configuration key.
    pub fn set_f64(&mut self, key: &str, val: f64) {
        if let Some(num) = serde_json::Number::from_f64(val) {
            self.values.insert(key.to_string(), serde_json::Value::Number(num));
        }
    }

    /// Sets a boolean configuration key.
    pub fn set_bool(&mut self, key: &str, val: bool) {
        self.values.insert(key.to_string(), serde_json::Value::from(val));
    }

    /// Sets a string configuration key.
    pub fn set_str(&mut self, key: &str, val: &str) {
        self.values.insert(key.to_string(), serde_json::Value::from(val));
    }

    /// Parses and updates configuration from JSON text payload.
    pub fn load_from_json(&mut self, json_str: &str) -> Result<(), serde_json::Error> {
        let parsed: HashMap<String, serde_json::Value> = serde_json::from_str(json_str)?;
        for (k, v) in parsed {
            self.values.insert(k, v);
        }
        self.last_refresh = chrono::Utc::now().timestamp();
        Ok(())
    }

    // Upstream typed property getters:
    pub fn quote_length_max(&self) -> i32 {
        self.get_i64("quote_length_max", 1024) as i32
    }

    pub fn stargift_convert_period_max(&self) -> i32 {
        self.get_i64("stargift_convert_period_max", 7776000) as i32
    }

    pub fn no_forwards_request_expire_period(&self) -> i32 {
        self.get_i64("no_forwards_request_expire_period", 86400) as i32
    }

    pub fn starref_setup_allowed(&self) -> bool {
        self.get_bool("starref_setup_allowed", true)
    }

    pub fn starref_join_allowed(&self) -> bool {
        self.get_bool("starref_join_allowed", true)
    }

    pub fn starref_commission_min(&self) -> i32 {
        self.get_i64("starref_commission_min", 0) as i32
    }

    pub fn starref_commission_max(&self) -> i32 {
        self.get_i64("starref_commission_max", 100) as i32
    }

    pub fn stars_withdraw_max(&self) -> i32 {
        self.get_i64("stars_withdraw_max", 1000000) as i32
    }

    pub fn stars_withdraw_rate(&self) -> f64 {
        self.get_f64("stars_withdraw_rate", 0.013)
    }

    pub fn stars_sell_rate(&self) -> f64 {
        self.get_f64("stars_sell_rate", 0.013)
    }

    pub fn currency_withdraw_rate(&self) -> f64 {
        self.get_f64("currency_withdraw_rate", 1.0)
    }

    pub fn poll_options_limit(&self) -> i32 {
        self.get_i64("poll_options_limit", 10) as i32
    }

    pub fn todo_list_items_limit(&self) -> i32 {
        self.get_i64("todo_list_items_limit", 30) as i32
    }

    pub fn confcall_size_limit(&self) -> i32 {
        self.get_i64("confcall_size_limit", 1000) as i32
    }

    pub fn phone_country_iso2(&self) -> String {
        self.get_string("phone_country_iso2", "US")
    }

    pub fn wallet_available(&self) -> bool {
        self.get_bool("wallet_available", true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_defaults_and_override() {
        let mut cfg = MainAppConfig::new();
        assert_eq!(cfg.quote_length_max(), 1024);
        assert_eq!(cfg.poll_options_limit(), 10);
        assert_eq!(cfg.stars_withdraw_rate(), 0.013);
        assert!(cfg.wallet_available());

        let json = r#"{"quote_length_max": 2048, "custom_key": "custom_val"}"#;
        assert!(cfg.load_from_json(json).is_ok());
        assert_eq!(cfg.quote_length_max(), 2048);
        assert_eq!(cfg.get_string("custom_key", ""), "custom_val");
    }
}
