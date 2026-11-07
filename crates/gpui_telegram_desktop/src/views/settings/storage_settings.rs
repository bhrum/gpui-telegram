use gpui::*;

pub struct StorageSettingsView {
    pub cache_size_mb: u64,
}

impl StorageSettingsView {
    pub fn new(cache_size_mb: u64) -> Self {
        Self { cache_size_mb }
    }
}

impl Default for StorageSettingsView {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for StorageSettingsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .p(px(24.0))
            .child(div().font_weight(FontWeight::BOLD).child("Data and Storage"))
            .child(div().child(format!("Local Cache: {} MB", self.cache_size_mb)))
    }
}
