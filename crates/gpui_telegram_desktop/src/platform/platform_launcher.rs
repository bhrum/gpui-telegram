//! Platform Launcher Service
//! Upstream: Telegram/SourceFiles/platform/platform_launcher.h
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct PlatformLauncher {
    pub args: Vec<String>,
    pub start_in_tray: bool,
    pub debug_mode: bool,
    pub custom_working_dir: Option<PathBuf>,
    pub launch_url: Option<String>,
    pub is_supported: bool,
    pub is_enabled: bool,
    pub platform_name: String,
}

impl PlatformLauncher {
    pub fn new() -> Self {
        Self::from_args(std::env::args().collect())
    }

    pub fn from_args(args: Vec<String>) -> Self {
        let mut launcher = Self {
            args: args.clone(),
            start_in_tray: false,
            debug_mode: false,
            custom_working_dir: None,
            launch_url: None,
            is_supported: true,
            is_enabled: true,
            platform_name: std::env::consts::OS.to_string(),
        };
        launcher.parse_args(&args);
        launcher
    }

    pub fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn parse_args(&mut self, args: &[String]) {
        for arg in args {
            if arg == "-startintray" || arg == "--startintray" {
                self.start_in_tray = true;
            } else if arg == "-debug" || arg == "--debug" {
                self.debug_mode = true;
            } else if arg.starts_with("tg://") || arg.starts_with("https://t.me/") {
                self.launch_url = Some(arg.clone());
            } else if let Some(dir) = arg.strip_prefix("-workdir=") {
                self.custom_working_dir = Some(PathBuf::from(dir));
            }
        }
    }

    pub fn start_in_tray(&self) -> bool {
        self.start_in_tray
    }

    pub fn debug_mode(&self) -> bool {
        self.debug_mode
    }

    pub fn launch_url(&self) -> Option<&str> {
        self.launch_url.as_deref()
    }
}

impl Default for PlatformLauncher {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PlatformLauncher {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p(px(8.0))
            .gap(px(4.0))
            .bg(rgb(0xFAFAFA))
            .rounded_md()
            .child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .child(format!("Launcher: OS={}", self.platform_name)),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(4.0))
                    .child(if self.start_in_tray {
                        div().px(px(4.0)).bg(rgb(0xBBDEFB)).text_size(px(10.0)).child("Tray")
                    } else {
                        div().px(px(4.0)).bg(rgb(0xE0E0E0)).text_size(px(10.0)).child("Window")
                    })
                    .child(if self.debug_mode {
                        div().px(px(4.0)).bg(rgb(0xFFCDD2)).text_size(px(10.0)).child("Debug")
                    } else {
                        div().px(px(4.0)).bg(rgb(0xC8E6C9)).text_size(px(10.0)).child("Release")
                    }),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launcher_arg_parsing() {
        let args = vec![
            "gpui_telegram_desktop".into(),
            "-startintray".into(),
            "-debug".into(),
            "tg://resolve?domain=telegram".into(),
            "-workdir=/tmp/tdata".into(),
        ];
        let launcher = PlatformLauncher::from_args(args);
        assert!(launcher.start_in_tray());
        assert!(launcher.debug_mode());
        assert_eq!(launcher.launch_url(), Some("tg://resolve?domain=telegram"));
        assert_eq!(launcher.custom_working_dir, Some(PathBuf::from("/tmp/tdata")));
    }
}
