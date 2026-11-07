//! Reaction Horizontal Strip with Hover Scaling
//! Faithful upstream C++ extraction:
//! - `history_view_reactions_strip.cpp:L20-L80`

use gpui::*;
use super::button::ReactionKey;

pub const K_SIZE_FOR_DOWNSCALE: usize = 96;
pub const K_EMOJI_CACHE_INDEX: usize = 0;
pub const K_HOVER_SCALE_DURATION_MS: u64 = 200;
pub const K_HOVER_SCALE: f32 = 1.24;

#[derive(Debug, Clone)]
pub struct StripReactionItem {
    pub key: ReactionKey,
    pub is_selected: bool,
    pub scale: f32,
}

impl StripReactionItem {
    pub fn new(key: ReactionKey) -> Self {
        Self {
            key,
            is_selected: false,
            scale: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReactionsStrip {
    pub items: Vec<StripReactionItem>,
    pub hovered_index: Option<usize>,
    pub has_expand_button: bool,
}

impl ReactionsStrip {
    pub fn new(reactions: Vec<ReactionKey>) -> Self {
        let items = reactions.into_iter().map(StripReactionItem::new).collect();
        Self {
            items,
            hovered_index: None,
            has_expand_button: true,
        }
    }

    pub fn set_hovered(&mut self, index: Option<usize>) {
        self.hovered_index = index;
        for (i, item) in self.items.iter_mut().enumerate() {
            if Some(i) == index {
                item.scale = K_HOVER_SCALE;
            } else {
                item.scale = 1.0;
            }
        }
    }

    pub fn select_reaction(&mut self, index: usize) -> Option<ReactionKey> {
        if index < self.items.len() {
            for (i, item) in self.items.iter_mut().enumerate() {
                item.is_selected = i == index;
            }
            Some(self.items[index].key.clone())
        } else {
            None
        }
    }
}

impl Default for ReactionsStrip {
    fn default() -> Self {
        Self::new(vec![
            ReactionKey::Emoji("👍".into()),
            ReactionKey::Emoji("❤️".into()),
            ReactionKey::Emoji("🔥".into()),
            ReactionKey::Emoji("🎉".into()),
            ReactionKey::Emoji("👏".into()),
            ReactionKey::Paid,
        ])
    }
}

impl Render for ReactionsStrip {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let hovered = self.hovered_index;
        
        let mut strip_div = div()
            .flex()
            .items_center()
            .h(px(40.0))
            .px(px(8.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(20.0))
            .shadow_md()
            .border_1()
            .border_color(rgb(0xE8E8E8))
            .gap(px(6.0))
            .children(self.items.iter().enumerate().map(|(idx, item)| {
                let is_hovered = hovered == Some(idx);
                let label = item.key.display_label();
                
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(28.0))
                    .h(px(28.0))
                    .rounded_full()
                    .bg(if item.is_selected { rgba(0x2AABEE20) } else { rgba(0x00000000) })
                    .hover(|s| s.bg(rgba(0x00000010)))
                    .text_size(if is_hovered { px(20.0) } else { px(16.0) })
                    .child(label)
            }));

        if self.has_expand_button {
            strip_div = strip_div.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(24.0))
                    .h(px(24.0))
                    .rounded_full()
                    .bg(rgba(0x00000008))
                    .hover(|s| s.bg(rgba(0x00000015)))
                    .text_size(px(14.0))
                    .text_color(rgb(0x666666))
                    .child("+"),
            );
        }

        strip_div
    }
}
