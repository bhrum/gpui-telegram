//! MenuTtlValidator: Permission validator for setting auto-delete timeouts in channels and groups
//! Upstream: Telegram/SourceFiles/menu/menu_ttl_validator.h, menu_ttl_validator.cpp

use gpui::*;

#[derive(Debug, Clone)]
pub struct MenuTtlValidator {
    pub is_admin: bool,
    pub can_change_info: bool,
}

impl MenuTtlValidator {
    pub fn new() -> Self {
        Self {
            is_admin: true,
            can_change_info: true,
        }
    }

    pub fn with_rights(is_admin: bool, can_change_info: bool) -> Self {
        Self {
            is_admin,
            can_change_info,
        }
    }

    pub fn can_modify_ttl(&self) -> bool {
        self.is_admin && self.can_change_info
    }
}

impl Default for MenuTtlValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MenuTtlValidator {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .text_xs()
            .text_color(rgb(0x82868A))
            .child(format!("TtlValidator: allowed={}", self.can_modify_ttl()))
    }
}
