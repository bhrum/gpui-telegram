use gpui::*;

pub struct ProfileSettingsView {
    pub display_name: String,
    pub username: String,
    pub bio: String,
}

impl ProfileSettingsView {
    pub fn new(display_name: String, username: String, bio: String) -> Self {
        Self {
            display_name,
            username,
            bio,
        }
    }
}

impl Render for ProfileSettingsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .p(px(24.0))
            .child(div().font_weight(FontWeight::BOLD).child("Edit Profile"))
            .child(div().child(format!("Name: {}", self.display_name)))
            .child(div().child(format!("Username: @{}", self.username)))
            .child(div().child(format!("Bio: {}", self.bio)))
    }
}
