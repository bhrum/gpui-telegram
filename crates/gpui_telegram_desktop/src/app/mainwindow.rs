//! Top-level GPUI window container, layer stack, and work mode state machine
//! Upstream: Telegram/SourceFiles/mainwindow.h, mainwindow.cpp
//! Subsystem: main | Feature ID: TCG-N46 | Disposition: gpui_ui_1to1

use super::mainwidget::Mainwidget;
use serde::{Deserialize, Serialize};

/// High-level window operating mode controlling top-level view rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WindowWorkMode {
    #[default]
    PasscodeLock,
    SetupEmailLock,
    Intro,
    MainContent,
}

/// Primary desktop window coordinator managing modal layers and content widgets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mainwindow {
    pub width: i32,
    pub height: i32,
    pub work_mode: WindowWorkMode,
    pub main_widget: Option<Mainwidget>,
    pub layer_stack: Vec<String>,
    pub is_active: bool,
    pub first_show_finished: bool,
}

impl Default for Mainwindow {
    fn default() -> Self {
        Self::new(1280, 800)
    }
}

impl Mainwindow {
    /// Creates a new main window instance with initial dimensions.
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            work_mode: WindowWorkMode::MainContent,
            main_widget: Some(Mainwidget::new(width, height)),
            layer_stack: Vec::new(),
            is_active: true,
            first_show_finished: false,
        }
    }

    /// Current high-level window operating mode.
    #[inline]
    pub fn work_mode(&self) -> WindowWorkMode {
        self.work_mode
    }

    /// Transitions window to Passcode Lock screen.
    pub fn setup_passcode_lock(&mut self) {
        self.work_mode = WindowWorkMode::PasscodeLock;
    }

    /// Clears Passcode Lock and returns to main content.
    pub fn clear_passcode_lock(&mut self) {
        if self.work_mode == WindowWorkMode::PasscodeLock {
            self.work_mode = WindowWorkMode::MainContent;
        }
    }

    /// Transitions window to Setup Email Lock screen.
    pub fn setup_setup_email_lock(&mut self) {
        self.work_mode = WindowWorkMode::SetupEmailLock;
    }

    /// Clears Setup Email Lock and returns to main content.
    pub fn clear_setup_email_lock(&mut self) {
        if self.work_mode == WindowWorkMode::SetupEmailLock {
            self.work_mode = WindowWorkMode::MainContent;
        }
    }

    /// Transitions window to onboarding login flow (Intro).
    pub fn setup_intro(&mut self) {
        self.work_mode = WindowWorkMode::Intro;
    }

    /// Sets up main conversation view.
    pub fn setup_main(&mut self) {
        self.work_mode = WindowWorkMode::MainContent;
        if self.main_widget.is_none() {
            self.main_widget = Some(Mainwidget::new(self.width, self.height));
        }
    }

    /// Finalizes first window presentation.
    pub fn finish_first_show(&mut self) {
        self.first_show_finished = true;
    }

    /// Pushes a modal overlay layer (e.g. Media Viewer, Settings, Modal Box).
    pub fn push_layer(&mut self, layer_id: String) {
        self.layer_stack.push(layer_id);
    }

    /// Pops the topmost modal layer. Returns true if a layer was dismissed.
    pub fn pop_layer(&mut self) -> bool {
        self.layer_stack.pop().is_some()
    }

    /// Returns true if any modal overlay layer is currently presented.
    #[inline]
    pub fn is_layer_shown(&self) -> bool {
        !self.layer_stack.is_empty()
    }

    /// Returns immutable reference to the main session widget.
    pub fn session_content(&self) -> Option<&Mainwidget> {
        self.main_widget.as_ref()
    }

    /// Returns mutable reference to the main session widget.
    pub fn session_content_mut(&mut self) -> Option<&mut Mainwidget> {
        self.main_widget.as_mut()
    }

    /// Updates window size and propagates to internal child widgets.
    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        if let Some(ref mut mw) = self.main_widget {
            mw.resize(width, height);
        }
    }

    /// Handles window activation / deactivation events.
    pub fn check_activation(&mut self, active: bool) {
        self.is_active = active;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_window_work_modes_and_layers() {
        let mut window = Mainwindow::new(1280, 800);
        assert_eq!(window.work_mode(), WindowWorkMode::MainContent);

        window.setup_passcode_lock();
        assert_eq!(window.work_mode(), WindowWorkMode::PasscodeLock);

        window.clear_passcode_lock();
        assert_eq!(window.work_mode(), WindowWorkMode::MainContent);

        assert!(!window.is_layer_shown());
        window.push_layer("SettingsBox".into());
        assert!(window.is_layer_shown());
        assert!(window.pop_layer());
        assert!(!window.is_layer_shown());
    }
}
