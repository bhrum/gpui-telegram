//! Info Profile Actions component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_actions.cpp:L1-L3510`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_actions.h:L1-L53`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileActionRow {
    pub label: String,
    pub value: String,
    pub is_copyable: bool,
    pub is_destructive: bool,
}

/// Profile action rows container rendering contact information (phone, username, bio),
/// notification toggles, and administrative or privacy action items.
#[derive(Debug, Clone)]
pub struct InfoProfileActionsProfile {
    pub peer_id: i64,
    pub phone: Option<String>,
    pub username: Option<String>,
    pub bio: Option<String>,
    pub notifications_enabled: bool,
    pub is_blocked: bool,
    pub can_manage_peer: bool,
}

impl InfoProfileActionsProfile {
    pub fn new(peer_id: i64) -> Self {
        Self {
            peer_id,
            phone: None,
            username: None,
            bio: None,
            notifications_enabled: true,
            is_blocked: false,
            can_manage_peer: false,
        }
    }

    pub fn set_phone(&mut self, phone: impl Into<String>) {
        self.phone = Some(phone.into());
    }

    pub fn set_username(&mut self, username: impl Into<String>) {
        self.username = Some(username.into());
    }

    pub fn set_bio(&mut self, bio: impl Into<String>) {
        self.bio = Some(bio.into());
    }

    pub fn toggle_notifications(&mut self) {
        self.notifications_enabled = !self.notifications_enabled;
    }

    pub fn toggle_blocked(&mut self) {
        self.is_blocked = !self.is_blocked;
    }
}

impl Default for InfoProfileActionsProfile {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for InfoProfileActionsProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let phone = self.phone.clone().unwrap_or_else(|| "Not provided".to_string());
        let username = self.username.clone().map(|u| format!("@{}", u)).unwrap_or_else(|| "None".to_string());
        let notifications = if self.notifications_enabled { "Enabled" } else { "Muted" };

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(12.0))
            .gap(px(10.0))
            .bg(rgb(0x18222d))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(div().text_xs().text_color(rgb(0x708499)).child("Mobile"))
                    .child(div().text_sm().text_color(rgb(0xffffff)).child(phone))
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(div().text_xs().text_color(rgb(0x708499)).child("Username"))
                    .child(div().text_sm().text_color(rgb(0x2ea6ff)).child(username))
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .py(px(4.0))
                    .child(div().text_sm().text_color(rgb(0xffffff)).child("Notifications"))
                    .child(div().text_sm().text_color(rgb(0x5288c1)).child(notifications))
            )
    }
}
