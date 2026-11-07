//! Info Profile Phone Menu component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_phone_menu.cpp:L1-L69`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_phone_menu.h:L1-L22`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhoneMenuAction {
    CopyNumber,
    TelegramCall,
    ExternalCall,
    AddToContacts,
}

/// Context menu triggered on phone numbers in the profile view.
#[derive(Debug, Clone)]
pub struct InfoProfilePhoneMenuProfile {
    pub phone_number: String,
    pub is_contact: bool,
    pub can_call: bool,
}

impl InfoProfilePhoneMenuProfile {
    pub fn new(phone_number: impl Into<String>) -> Self {
        Self {
            phone_number: phone_number.into(),
            is_contact: false,
            can_call: true,
        }
    }

    pub fn set_is_contact(&mut self, is_contact: bool) {
        self.is_contact = is_contact;
    }
}

impl Default for InfoProfilePhoneMenuProfile {
    fn default() -> Self {
        Self::new("")
    }
}

impl Render for InfoProfilePhoneMenuProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut el = div()
            .flex()
            .flex_col()
            .w(px(180.0))
            .bg(rgb(0x232e3c))
            .rounded_md()
            .p(px(4.0))
            .gap(px(2.0))
            .child(div().p(px(6.0)).text_sm().child("Copy Phone Number"));

        if self.can_call {
            el = el.child(div().p(px(6.0)).text_sm().child("Telegram Call"));
        }
        if !self.is_contact {
            el = el.child(div().p(px(6.0)).text_sm().child("Add to Contacts"));
        }
        el
    }
}
