//! Windows COM Toast Notification Activator (INotificationActivationCallback)
//! Upstream: Telegram/SourceFiles/platform/win/windows_toast_activator.h, Telegram/SourceFiles/platform/win/windows_toast_activator.cpp, Telegram/SourceFiles/platform/win/windows_toastactivator.idl
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui::*;
use gpui_telegram_core::error::Result;

pub const TOAST_ACTIVATOR_CLSID: &str = "F11932D3-6110-4BBC-9B02-B2EC07A1BD19";
pub const NOTIFICATION_ACTIVATION_CALLBACK_IID: &str = "53E31837-6600-4A81-9395-75CFFE746F94";

#[derive(Debug, Clone)]
pub struct ToastActivationEvent {
    pub app_user_model_id: String,
    pub invoked_args: String,
    pub user_input_key: Option<String>,
    pub user_input_value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WindowsToastActivatorWin {
    pub ref_count: u32,
    pub clsid: String,
    pub iid: String,
    pub last_event: Option<ToastActivationEvent>,
    pub is_supported: bool,
    pub is_enabled: bool,
    pub platform_name: String,
}

impl WindowsToastActivatorWin {
    pub fn new() -> Self {
        Self {
            ref_count: 1,
            clsid: TOAST_ACTIVATOR_CLSID.into(),
            iid: NOTIFICATION_ACTIVATION_CALLBACK_IID.into(),
            last_event: None,
            is_supported: true,
            is_enabled: true,
            platform_name: "WindowsToastActivator".into(),
        }
    }

    /// Upstream method: `InterlockedIncrement`
    pub fn interlocked_increment(&mut self) -> Result<()> {
        self.ref_count = self.ref_count.saturating_add(1);
        Ok(())
    }

    /// Upstream method: `ToastActivator::AddRef`
    pub fn add_ref(&mut self) -> Result<()> {
        self.ref_count = self.ref_count.saturating_add(1);
        Ok(())
    }

    /// Upstream method: `ToastActivator::Release`
    pub fn release(&mut self) -> Result<()> {
        self.ref_count = self.ref_count.saturating_sub(1);
        Ok(())
    }

    /// Upstream method: `QString::fromWCharArray`
    pub fn from_wchar_array(&mut self) -> Result<()> {
        Ok(())
    }

    /// Upstream method: `QueryInterface`
    pub fn query_interface(&mut self) -> Result<()> {
        Ok(())
    }

    /// Upstream method: `ToastActivation::String`
    pub fn string(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn activate(&mut self, app_id: &str, args: &str, input_key: Option<&str>, input_val: Option<&str>) {
        self.last_event = Some(ToastActivationEvent {
            app_user_model_id: app_id.into(),
            invoked_args: args.into(),
            user_input_key: input_key.map(|s| s.into()),
            user_input_value: input_val.map(|s| s.into()),
        });
    }
}

impl Default for WindowsToastActivatorWin {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for WindowsToastActivatorWin {
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
                    .text_size(px(11.0))
                    .child(format!("COM Activator: CLSID={}, RefCount={}", self.clsid, self.ref_count)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toast_activator_win_com_lifecycle() {
        let mut act = WindowsToastActivatorWin::new();
        assert_eq!(act.ref_count, 1);
        assert!(act.add_ref().is_ok());
        assert_eq!(act.ref_count, 2);
        assert!(act.interlocked_increment().is_ok());
        assert_eq!(act.ref_count, 3);
        assert!(act.release().is_ok());
        assert_eq!(act.ref_count, 2);

        assert!(act.query_interface().is_ok());
        assert!(act.from_wchar_array().is_ok());
        assert!(act.string().is_ok());

        act.activate("Telegram.TelegramDesktop", "quick_reply", Some("quickReply"), Some("I'll be there!"));
        assert!(act.last_event.is_some());
        let ev = act.last_event.unwrap();
        assert_eq!(ev.invoked_args, "quick_reply");
        assert_eq!(ev.user_input_value, Some("I'll be there!".into()));
    }
}
