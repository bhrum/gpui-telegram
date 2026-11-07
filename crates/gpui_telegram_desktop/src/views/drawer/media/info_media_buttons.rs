//! Info Media Buttons component
//! Upstream: `Telegram/SourceFiles/info/media/info_media_buttons.cpp:L1-L284`
//! Upstream: `Telegram/SourceFiles/info/media/info_media_buttons.h:L1-L73`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;
use std::collections::HashMap;
use super::info_media_common::SharedMediaType;

/// Button bar for switching between shared media sections (Media, Files, Links, Music, Voice, GIFs)
/// with item count badges and animated selection indicator.
#[derive(Debug, Clone)]
pub struct InfoMediaButtonsMedia {
    pub active_type: SharedMediaType,
    pub section_counts: HashMap<SharedMediaType, usize>,
    pub is_expanded: bool,
    pub peer_id: i64,
}

impl InfoMediaButtonsMedia {
    pub fn new(peer_id: i64) -> Self {
        let mut section_counts = HashMap::new();
        section_counts.insert(SharedMediaType::PhotoVideo, 0);
        section_counts.insert(SharedMediaType::Files, 0);
        section_counts.insert(SharedMediaType::SharedLinks, 0);
        section_counts.insert(SharedMediaType::AudioMusic, 0);
        section_counts.insert(SharedMediaType::VoiceMessages, 0);
        section_counts.insert(SharedMediaType::Gifs, 0);

        Self {
            active_type: SharedMediaType::PhotoVideo,
            section_counts,
            is_expanded: false,
            peer_id,
        }
    }

    pub fn set_active_type(&mut self, media_type: SharedMediaType) {
        self.active_type = media_type;
    }

    pub fn set_count(&mut self, media_type: SharedMediaType, count: usize) {
        self.section_counts.insert(media_type, count);
    }

    pub fn count_for(&self, media_type: SharedMediaType) -> usize {
        self.section_counts.get(&media_type).copied().unwrap_or(0)
    }

    pub fn toggle_expanded(&mut self) {
        self.is_expanded = !self.is_expanded;
    }
}

impl Default for InfoMediaButtonsMedia {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for InfoMediaButtonsMedia {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let active = self.active_type;
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .px(px(8.0))
            .py(px(4.0))
            .bg(rgb(0x202b36))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(4.0))
                    .children(SharedMediaType::all().into_iter().map(|t| {
                        let is_active = t == active;
                        let count = self.count_for(t);
                        div()
                            .flex()
                            .items_center()
                            .px(px(10.0))
                            .py(px(6.0))
                            .rounded_md()
                            .bg(if is_active { rgb(0x2b5278) } else { rgb(0x18222d) })
                            .text_color(if is_active { rgb(0xffffff) } else { rgb(0x7f91a4) })
                            .child(format!("{} ({})", t.label(), count))
                    }))
            )
    }
}
