//! MenuAntispamValidator: Administrative privilege check for anti-spam moderation actions
//! Upstream: Telegram/SourceFiles/menu/menu_antispam_validator.h, menu_antispam_validator.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct MenuAntispamValidator {
    pub is_admin: bool,
    pub can_delete_messages: bool,
    pub can_ban_users: bool,
}

impl MenuAntispamValidator {
    pub fn new() -> Self {
        Self {
            is_admin: false,
            can_delete_messages: false,
            can_ban_users: false,
        }
    }

    pub fn with_privileges(is_admin: bool, can_delete: bool, can_ban: bool) -> Self {
        Self {
            is_admin,
            can_delete_messages: can_delete,
            can_ban_users: can_ban,
        }
    }

    pub fn can_moderate(&self) -> bool {
        self.is_admin && (self.can_delete_messages || self.can_ban_users)
    }

    pub fn can_report_spam(&self) -> bool {
        true
    }
}

impl Default for MenuAntispamValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MenuAntispamValidator {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("AntispamValidator: can_moderate={}", self.can_moderate()))
    }
}
