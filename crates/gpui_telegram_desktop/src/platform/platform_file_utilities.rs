//! Cross-Platform File Utilities and URL Launcher
//! Upstream Source: Telegram/SourceFiles/platform/platform_file_utilities.cpp, Telegram/SourceFiles/platform/platform_file_utilities.h
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// Action type dispatched by platform file utilities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlatformLaunchAction {
    OpenUrl(String),
    OpenEmail(String),
    LaunchFile(String),
    ShowOpenWith(String),
}

/// Cross-platform file utilities, launch safety fuses, and system open dispatchers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformFileUtilities {
    pub last_file_dialog_path: String,
    pub block_launch_fuse: bool,
    pub launch_history: Vec<PlatformLaunchAction>,
    pub blocked_launches_count: usize,
}

impl PlatformFileUtilities {
    pub fn new() -> Self {
        Self {
            last_file_dialog_path: "~/Downloads".into(),
            block_launch_fuse: false,
            launch_history: Vec::new(),
            blocked_launches_count: 0,
        }
    }

    /// Converts `file://` URL to clean local system path.
    pub fn url_to_local(url: &str) -> String {
        if let Some(stripped) = url.strip_prefix("file://") {
            stripped.to_string()
        } else {
            url.to_string()
        }
    }

    /// Sets or clears test launch fuse (`Test::BlockLaunch`).
    pub fn set_block_launch_fuse(&mut self, block: bool) {
        self.block_launch_fuse = block;
    }

    pub fn unsafe_open_url(&mut self, url: &str) -> Result<bool> {
        if self.block_launch_fuse {
            self.blocked_launches_count += 1;
            return Ok(false);
        }
        self.launch_history.push(PlatformLaunchAction::OpenUrl(url.into()));
        Ok(true)
    }

    pub fn unsafe_open_email_link(&mut self, email: &str) -> Result<bool> {
        if self.block_launch_fuse {
            self.blocked_launches_count += 1;
            return Ok(false);
        }
        let mailto = format!("mailto:{}", email);
        self.launch_history.push(PlatformLaunchAction::OpenEmail(mailto));
        Ok(true)
    }

    pub fn unsafe_launch(&mut self, filepath: &str) -> Result<bool> {
        if self.block_launch_fuse {
            self.blocked_launches_count += 1;
            return Ok(false);
        }
        self.launch_history.push(PlatformLaunchAction::LaunchFile(filepath.into()));
        Ok(true)
    }

    pub fn unsafe_show_open_with(&mut self, filepath: &str) -> Result<bool> {
        if self.block_launch_fuse {
            self.blocked_launches_count += 1;
            return Ok(false);
        }
        self.launch_history.push(PlatformLaunchAction::ShowOpenWith(filepath.into()));
        Ok(true)
    }

    pub fn postprocess_downloaded(&mut self, _filepath: &str) -> Result<()> {
        // Platform file attribute post-processing
        Ok(())
    }

    pub fn init_last_path(&mut self, path: impl Into<String>) {
        self.last_file_dialog_path = path.into();
    }
}

impl Default for PlatformFileUtilities {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_file_utilities_and_fuses() {
        let mut utils = PlatformFileUtilities::new();
        assert_eq!(PlatformFileUtilities::url_to_local("file:///Users/photo.jpg"), "/Users/photo.jpg");

        assert!(utils.unsafe_open_url("https://telegram.org").unwrap());
        assert_eq!(utils.launch_history.len(), 1);

        // Turn on test fuse
        utils.set_block_launch_fuse(true);
        assert!(!utils.unsafe_open_url("https://example.com").unwrap());
        assert_eq!(utils.blocked_launches_count, 1);
        assert_eq!(utils.launch_history.len(), 1); // Not appended when blocked

        // Turn off fuse and launch file
        utils.set_block_launch_fuse(false);
        assert!(utils.unsafe_launch("/tmp/document.pdf").unwrap());
        assert_eq!(utils.launch_history.len(), 2);
    }
}
