//! Authentic GPUI Peer Bubble Widget for Selected Chats & Channel Tags.
//! Reference: Telegram/SourceFiles/ui/widgets/peer_bubble.cpp:L23-L60
//! Reference: Telegram/SourceFiles/ui/widgets/peer_bubble.h:L18-L26

#![allow(dead_code, unused_imports, unused_variables)]

/// Target symbol for compact peer bubble displaying userpic and peer display name.
#[derive(Debug, Clone)]
pub struct PeerBubbleWidgets {
    pub peer_id: u64,
    pub title: String,
    pub userpic_size: f32,
    pub natural_width: f32,
    pub padding_horizontal: f32,
    pub padding_vertical: f32,
    pub max_width: f32,
}

impl Default for PeerBubbleWidgets {
    fn default() -> Self {
        Self::new(0, "Peer")
    }
}

impl PeerBubbleWidgets {
    pub fn new(peer_id: u64, title: impl Into<String>) -> Self {
        let t = title.into();
        let mut bubble = Self {
            peer_id,
            title: t,
            userpic_size: 24.0,
            natural_width: 0.0,
            padding_horizontal: 8.0,
            padding_vertical: 4.0,
            max_width: 240.0,
        };
        bubble.recalculate_natural_width();
        bubble
    }

    /// Computes natural width: `leftSize.width() + rightSize.width() + padding`.
    /// Reference: `peer_bubble.cpp:L45-L56`
    pub fn recalculate_natural_width(&mut self) {
        let text_w = self.title.len() as f32 * 8.0;
        let total = self.userpic_size + text_w + (self.padding_horizontal * 2.0) + 4.0;
        self.natural_width = total.min(self.max_width);
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
        self.recalculate_natural_width();
    }

    pub fn set_max_width(&mut self, max_width: f32) {
        self.max_width = max_width;
        self.recalculate_natural_width();
    }
}
