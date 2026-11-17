//! Window & App Shell Subsystem Coordinator
//! Owns responsive layout adaptation, notifications, connecting banner, lock widgets, and toast layers.

pub use crate::views::app_shell::main_window::*;
pub use crate::views::app_shell::window_adaptive::*;
pub use crate::views::app_shell::window_controller::*;
pub use crate::views::app_shell::window_main_menu::*;
pub use crate::views::app_shell::window_separate_id::*;
pub use crate::views::app_shell::window_session_controller::*;
pub use crate::views::app_shell::notifications_manager::*;
pub use crate::views::app_shell::themes::window_theme::*;
pub use crate::theme::palette::*;

use crate::views::app_shell::{
    menu_send::MenuSend,
    window_chat_switch_process::WindowChatSwitchProcess,
    window_connecting_widget::{ConnectionStatus, WindowConnectingWidget},
    window_filters_menu::WindowFiltersMenu,
    window_history_hider::WindowHistoryHider,
    window_lock_widgets::WindowLockWidgets,
    window_main_menu::AccountEntry,
    window_peer_menu::WindowPeerMenu,
    window_slide_animation::WindowSlideAnimation,
    window_top_bar_wrap::WindowTopBarWrap,
};

/// Unified runtime state holder connecting all `views::app_shell` controllers into `TelegramApp` & `AppShell`.
#[derive(Debug, Clone)]
pub struct WindowSubsystemRuntime {
    pub controller: WindowController,
    pub main_window: MainWindow,
    pub session_controller: WindowSessionController,
    pub adaptive: WindowAdaptive,
    pub main_menu: WindowMainMenu,
    pub connecting_widget: WindowConnectingWidget,
    pub lock_widgets: WindowLockWidgets,
    pub notifications: NotificationsManager,
    pub mac_notifications: crate::platform::mac::MacNotificationManager,
    pub top_bar_wrap: WindowTopBarWrap,
    pub chat_switch: WindowChatSwitchProcess,
    pub history_hider: WindowHistoryHider,
    pub slide_animation: WindowSlideAnimation,
    pub send_menu: MenuSend,
    pub filters_menu: WindowFiltersMenu,
    pub peer_menu: WindowPeerMenu,
}

impl Default for WindowSubsystemRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowSubsystemRuntime {
    pub fn new() -> Self {
        Self {
            controller: WindowController::new(),
            main_window: MainWindow::new(),
            session_controller: WindowSessionController::new(),
            adaptive: WindowAdaptive::new(),
            main_menu: WindowMainMenu::new(),
            connecting_widget: WindowConnectingWidget::new(),
            lock_widgets: WindowLockWidgets::new(),
            notifications: NotificationsManager::new(),
            mac_notifications: crate::platform::mac::MacNotificationManager::new(1),
            top_bar_wrap: WindowTopBarWrap::new(),
            chat_switch: WindowChatSwitchProcess::new(),
            history_hider: WindowHistoryHider::new(),
            slide_animation: WindowSlideAnimation::new(),
            send_menu: MenuSend::new(),
            filters_menu: WindowFiltersMenu::new(),
            peer_menu: WindowPeerMenu::new(),
        }
    }

    /// Synchronizes responsive column mode and compact topbar metrics with the desktop window width.
    pub fn apply_viewport_width(&mut self, width_px: f32) {
        self.adaptive.update_from_width(width_px);
        let is_compact = self.adaptive.is_one_column();
        self.top_bar_wrap.update_adaptive_layout(is_compact);
        if is_compact {
            self.controller.hide_right_column();
        }
    }

    /// Updates WebSocket/HTTP transport connection indicator without fabricating online state.
    pub fn set_connection_status(&mut self, status: ConnectionStatus, proxy_host: Option<String>) {
        self.connecting_widget.set_status(status);
        self.connecting_widget.set_proxy_host(proxy_host);
    }

    /// Enqueues a visible toast notification through `WindowController`.
    pub fn enqueue_toast(&mut self, text: impl Into<String>, duration_ms: u64) {
        self.controller.show_toast(text, duration_ms);
    }

    /// Synchronizes authenticated account identity onto session/menu controllers.
    pub fn bind_session_user(&mut self, user_id: u64, full_name: impl Into<String>, phone: impl Into<String>) {
        self.session_controller = WindowSessionController::with_session(format!("user:{user_id}"));
        let entry = AccountEntry {
            account_id: user_id as u32,
            name: full_name.into(),
            phone: phone.into(),
            unread_count: 0,
            is_active: true,
        };
        if let Some(existing) = self
            .main_menu
            .accounts
            .iter_mut()
            .find(|acc| acc.account_id == entry.account_id)
        {
            *existing = entry;
        } else {
            for acc in &mut self.main_menu.accounts {
                acc.is_active = false;
            }
            self.main_menu.accounts.push(entry);
        }
    }

    /// Clears session-scoped navigation history, menu profile strings, and chat switch transient state.
    pub fn clear_session_user(&mut self) {
        self.session_controller = WindowSessionController::new();
        self.main_menu.accounts.clear();
        self.notifications.clear_all();
        self.chat_switch.abort_switch();
    }
}
