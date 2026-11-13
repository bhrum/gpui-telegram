//! Platform Integration Base Service
//! Upstream: Telegram/SourceFiles/platform/platform_integration.h, Telegram/SourceFiles/platform/platform_integration.cpp
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PlatformIntegration {
    pub is_initialized: Arc<AtomicBool>,
    pub is_supported: bool,
    pub is_enabled: bool,
    pub platform_name: String,
    pub system_dark_mode: Arc<AtomicBool>,
    pub sleep_inhibit_count: Arc<AtomicU32>,
}

impl PlatformIntegration {
    pub fn new() -> Self {
        Self {
            is_initialized: Arc::new(AtomicBool::new(false)),
            is_supported: true,
            is_enabled: true,
            platform_name: std::env::consts::OS.to_string(),
            system_dark_mode: Arc::new(AtomicBool::new(false)),
            sleep_inhibit_count: Arc::new(AtomicU32::new(0)),
        }
    }

    /// Upstream method: `init`
    pub fn init(&mut self) -> Result<()> {
        self.is_initialized.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Upstream method: `~Integration`
    pub fn integration(&mut self) -> Result<()> {
        self.is_initialized.store(false, Ordering::SeqCst);
        Ok(())
    }

    /// Upstream method: `Integration::Create`
    pub fn create(&mut self) -> Result<()> {
        self.init()
    }

    /// Upstream method: `Integration::Instance`
    pub fn instance(&mut self) -> Result<()> {
        if !self.is_initialized.load(Ordering::SeqCst) {
            self.init()?;
        }
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.is_initialized.load(Ordering::SeqCst)
    }

    pub fn system_dark_mode(&self) -> bool {
        self.system_dark_mode.load(Ordering::SeqCst)
    }

    pub fn set_system_dark_mode(&mut self, dark: bool) {
        self.system_dark_mode.store(dark, Ordering::SeqCst);
    }

    pub fn inhibit_sleep(&mut self) -> u32 {
        self.sleep_inhibit_count.fetch_add(1, Ordering::SeqCst) + 1
    }

    pub fn release_sleep(&mut self) -> u32 {
        self.sleep_inhibit_count.fetch_sub(1, Ordering::SeqCst).saturating_sub(1)
    }
}

impl Default for PlatformIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PlatformIntegration {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let initialized = self.is_initialized.load(Ordering::SeqCst);
        let dark = self.system_dark_mode.load(Ordering::SeqCst);
        let status_color = if initialized { rgb(0x4CAF50) } else { rgb(0xF44336) };

        div()
            .flex()
            .flex_row()
            .items_center()
            .px(px(8.0))
            .py(px(4.0))
            .gap(px(6.0))
            .rounded_md()
            .bg(if dark { rgb(0x212121) } else { rgb(0xF5F5F5) })
            .child(
                div()
                    .w(px(8.0))
                    .h(px(8.0))
                    .rounded_full()
                    .bg(status_color),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(if dark { rgb(0xEEEEEE) } else { rgb(0x212121) })
                    .child(format!("Platform: {} ({})", self.platform_name, if initialized { "Ready" } else { "Uninit" })),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_integration_lifecycle() {
        let mut integration = PlatformIntegration::new();
        assert!(!integration.is_initialized());
        assert!(integration.create().is_ok());
        assert!(integration.is_initialized());
        assert_eq!(integration.inhibit_sleep(), 1);
        assert_eq!(integration.release_sleep(), 0);
        assert!(integration.integration().is_ok());
        assert!(!integration.is_initialized());
    }
}
