use gpui::*;

pub struct NetworkSettingsView {
    pub proxy_enabled: bool,
    pub proxy_server: String,
}

impl NetworkSettingsView {
    pub fn new() -> Self {
        Self {
            proxy_enabled: false,
            proxy_server: String::new(),
        }
    }
}

impl Default for NetworkSettingsView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for NetworkSettingsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .p(px(24.0))
            .child(div().font_weight(FontWeight::BOLD).child("Connection & Proxy"))
            .child(div().child(format!(
                "Proxy: {}",
                if self.proxy_enabled { &self.proxy_server } else { "Disabled" }
            )))
    }
}
