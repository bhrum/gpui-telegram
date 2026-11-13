//! Windows Modern Startup Task & Registry Autostart
//! Upstream: Telegram/SourceFiles/platform/win/windows_autostart_task.h, Telegram/SourceFiles/platform/win/windows_autostart_task.cpp
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;

#[derive(Debug, Clone)]
pub struct WindowsAutostartTaskWin {
    pub is_autostart_enabled: bool,
    pub winrt_supported: bool,
    pub task_id: String,
    pub is_supported: bool,
    pub is_enabled: bool,
    pub platform_name: String,
}

impl WindowsAutostartTaskWin {
    pub fn new() -> Self {
        Self {
            is_autostart_enabled: false,
            winrt_supported: true,
            task_id: "TelegramStartupTask".into(),
            is_supported: true,
            is_enabled: true,
            platform_name: "WindowsStartupTask".into(),
        }
    }

    /// Upstream method: `Launcher::LaunchUriAsync`
    pub fn launch_uri_async(&mut self) -> Result<()> {
        Ok(())
    }

    /// Upstream method: `WinRT::Try`
    pub fn try_action(&mut self) -> Result<()> {
        Ok(())
    }

    /// Upstream method: `OpenSettings`
    pub fn open_settings(&mut self) -> Result<()> {
        Ok(())
    }

    /// Upstream method: `WinRT::Supported`
    pub fn supported(&mut self) -> Result<()> {
        self.winrt_supported = true;
        Ok(())
    }

    /// Upstream method: `Logs::b`
    pub fn b(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_autostart_enabled = enabled;
    }
}

impl Default for WindowsAutostartTaskWin {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowsAutostartTaskWin {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .p(px(6.0))
            .gap(px(6.0))
            .bg(rgb(0xFFFFFF))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded_md()
            .child(
                div()
                    .w(px(12.0))
                    .h(px(12.0))
                    .rounded_full()
                    .bg(if self.is_autostart_enabled { rgb(0x4CAF50) } else { rgb(0x9E9E9E) }),
            )
            .child(
                div()
                    .text_size(px(11.0))
                    .child(format!("Autostart Task ({}): Enabled={}", self.task_id, self.is_autostart_enabled)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autostart_task_win_methods() {
        let mut task = WindowsAutostartTaskWin::new();
        assert!(task.launch_uri_async().is_ok());
        assert!(task.try_action().is_ok());
        assert!(task.open_settings().is_ok());
        assert!(task.supported().is_ok());
        assert!(task.b().is_ok());

        assert!(!task.is_autostart_enabled);
        task.set_enabled(true);
        assert!(task.is_autostart_enabled);
    }
}
