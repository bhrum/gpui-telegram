//! Contact Status Sliding Bar (Add Contact, Block, Share Phone, Spam Report)
//! Faithful upstream C++ extraction:
//! - `history_view_contact_status.cpp:L30-L150`
//! - `history_view_contact_status.h:L30-L70`

use gpui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContactStatusType {
    #[default]
    AddContact,
    SharePhoneNumber,
    BlockUser,
    ReportSpam,
    None,
}

#[derive(Debug, Clone)]
pub struct HistoryViewContactStatusView {
    pub peer_id: u64,
    pub status_type: ContactStatusType,
    pub phone_number: Option<String>,
    pub is_mutual: bool,
    pub is_visible: bool,
    pub slide_height: f32,
}

impl HistoryViewContactStatusView {
    pub fn new(peer_id: u64, status_type: ContactStatusType) -> Self {
        Self {
            peer_id,
            status_type,
            phone_number: None,
            is_mutual: false,
            is_visible: true,
            slide_height: 44.0,
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }
}

impl Default for HistoryViewContactStatusView {
    fn default() -> Self {
        Self::new(0, ContactStatusType::AddContact)
    }
}

impl Render for HistoryViewContactStatusView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.is_visible || self.status_type == ContactStatusType::None {
            return div().into_any_element();
        }

        let label = match self.status_type {
            ContactStatusType::AddContact => "Add to Contacts",
            ContactStatusType::SharePhoneNumber => "Share My Phone Number",
            ContactStatusType::BlockUser => "Block User",
            ContactStatusType::ReportSpam => "Report Spam",
            ContactStatusType::None => "",
        };

        div()
            .flex()
            .items_center()
            .justify_between()
            .h(px(self.slide_height))
            .w_full()
            .px(px(16.0))
            .bg(rgb(0xFFF9E6))
            .border_b_1()
            .border_color(rgb(0xF0E5C8))
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0x444444))
                    .child(format!("User is not in your contacts."))
            )
            .child(
                div()
                    .flex()
                    .gap(px(12.0))
                    .text_size(px(13.0))
                    .font_weight(FontWeight::BOLD)
                    .child(
                        div()
                            .text_color(rgb(0x2AABEE))
                            .hover(|s| s.underline())
                            .child(label)
                    )
                    .child(
                        div()
                            .text_color(rgb(0xE53935))
                            .hover(|s| s.underline())
                            .child("Block")
                    )
            )
            .into_any_element()
    }
}
