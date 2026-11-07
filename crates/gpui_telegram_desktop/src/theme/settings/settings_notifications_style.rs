// Source: Telegram/SourceFiles/settings/sections/settings_notifications.style
// Subsystem: settings | Feature ID: TCG-N31

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SettingsNotificationsSectionsStyleSheet {
    pub box_monitor: f32,
    pub box_screen_top: f32,
    pub box_screen_width: f32,
    pub box_screen_height: f32,
    pub sample_skip: f32,
    pub sample_top_skip: f32,
    pub sample_bottom_skip: f32,
    pub sample_margin: f32,
    pub sample_opacity: f32,
}

impl SettingsNotificationsSectionsStyleSheet {
    pub fn new() -> Self {
        Self {
            box_monitor: 16.0,
            box_screen_top: 10.0,
            box_screen_width: 160.0,
            box_screen_height: 100.0,
            sample_skip: 5.0,
            sample_top_skip: 5.0,
            sample_bottom_skip: 5.0,
            sample_margin: 2.0,
            sample_opacity: 0.8,
        }
    }

    pub fn notifications_box_monitor() -> f32 {
        16.0
    }

    pub fn notifications_box_screen_top() -> f32 {
        10.0
    }

    pub fn notifications_box_screen_size() -> (f32, f32) {
        (160.0, 100.0)
    }

    pub fn notifications_sample_skip() -> f32 {
        5.0
    }

    pub fn notifications_sample_top_skip() -> f32 {
        5.0
    }

    pub fn notifications_sample_bottom_skip() -> f32 {
        5.0
    }

    pub fn notifications_sample_margin() -> f32 {
        2.0
    }

    pub fn notification_sample_opacity() -> f32 {
        0.8
    }
}

impl Default for SettingsNotificationsSectionsStyleSheet {
    fn default() -> Self {
        Self::new()
    }
}
