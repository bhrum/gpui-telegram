use gpui::*;

pub struct EditPeerBoxView {
    pub peer_title: String,
}

impl EditPeerBoxView {
    pub fn new(peer_title: String) -> Self {
        Self { peer_title }
    }
}

impl Render for EditPeerBoxView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(320.0))
            .p(px(16.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .child(format!("Edit Peer: {}", self.peer_title))
    }
}
