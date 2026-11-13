use gpui::*;

pub struct PrivacySettingsView {
    pub last_seen_privacy: String,
    pub phone_number_privacy: String,
}

impl PrivacySettingsView {
    pub fn new() -> Self {
        Self {
            last_seen_privacy: "Everybody".into(),
            phone_number_privacy: "My Contacts".into(),
        }
    }
}

impl Default for PrivacySettingsView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for PrivacySettingsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .p(px(24.0))
            .child(div().font_weight(FontWeight::BOLD).child("Privacy and Security"))
            .child(div().child(format!("Last Seen: {}", self.last_seen_privacy)))
            .child(div().child(format!("Phone Number: {}", self.phone_number_privacy)))
    }
}
