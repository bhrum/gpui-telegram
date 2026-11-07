//! Dynamic application configuration value extractors and URL resolvers
//! Upstream: Telegram/SourceFiles/main/main_app_config_values.h, main_app_config_values.cpp
//! Subsystem: main | Feature ID: TCG-N46 | Disposition: gpui_ui_1to1

use serde::{Deserialize, Serialize};

/// Application configuration values and Ton Fragment / referral link resolvers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MainAppConfigValues {
    pub fragment_prefixes: Vec<String>,
    pub starref_prefixes: Vec<String>,
}

impl Default for MainAppConfigValues {
    fn default() -> Self {
        Self::new()
    }
}

impl MainAppConfigValues {
    /// Creates a new config values resolver with standard Telegram prefixes.
    pub fn new() -> Self {
        Self {
            fragment_prefixes: vec![
                "https://fragment.com/".into(),
                "https://t.me/".into(),
            ],
            starref_prefixes: vec![
                "https://t.me/starref/".into(),
            ],
        }
    }

    /// Resolves a Fragment username auction or purchase link.
    /// In accordance with C++ `AppConfig::FragmentLink`:
    pub fn fragment_link(&self, username: &str) -> String {
        let clean = username.trim().trim_start_matches('@');
        format!("https://fragment.com/username/{}", clean)
    }

    /// Resolves a Telegram Stars referral program setup or join URL.
    pub fn starref_link(&self, ref_hash: &str) -> String {
        format!("https://t.me/starref/{}", ref_hash)
    }

    /// Checks if a given URL matches configured Fragment link prefixes.
    pub fn is_fragment_url(&self, url: &str) -> bool {
        self.fragment_prefixes.iter().any(|prefix| url.starts_with(prefix))
    }

    /// Checks if a given URL matches configured StarRef link prefixes.
    pub fn is_starref_url(&self, url: &str) -> bool {
        self.starref_prefixes.iter().any(|prefix| url.starts_with(prefix))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fragment_link_formatting() {
        let values = MainAppConfigValues::new();
        assert_eq!(values.fragment_link("@durov"), "https://fragment.com/username/durov");
        assert_eq!(values.fragment_link("telegram"), "https://fragment.com/username/telegram");
        assert!(values.is_fragment_url("https://fragment.com/username/test"));
    }
}
