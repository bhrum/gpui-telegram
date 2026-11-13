//! Info Profile Shared Media Classic component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_shared_media_classic.cpp:L1-L157`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_shared_media_classic.h:L1-L36`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use gpui::*;

/// Classic shared media row list in profile cards displaying counts for Photos, Videos, Files, Links, and Music.
#[derive(Debug, Clone)]
pub struct InfoProfileSharedMediaClassicProfile {
    pub peer_id: i64,
    pub photos_count: usize,
    pub videos_count: usize,
    pub files_count: usize,
    pub audio_count: usize,
    pub links_count: usize,
    pub voice_count: usize,
}

impl InfoProfileSharedMediaClassicProfile {
    pub fn new(peer_id: i64) -> Self {
        Self {
            peer_id,
            photos_count: 0,
            videos_count: 0,
            files_count: 0,
            audio_count: 0,
            links_count: 0,
            voice_count: 0,
        }
    }

    pub fn set_counts(&mut self, photos: usize, videos: usize, files: usize, audio: usize, links: usize, voice: usize) {
        self.photos_count = photos;
        self.videos_count = videos;
        self.files_count = files;
        self.audio_count = audio;
        self.links_count = links;
        self.voice_count = voice;
    }
}

impl Default for InfoProfileSharedMediaClassicProfile {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Render for InfoProfileSharedMediaClassicProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(8.0))
            .gap(px(4.0))
            .child(div().text_sm().child(format!("Photos & Videos: {}", self.photos_count + self.videos_count)))
            .child(div().text_sm().child(format!("Files: {}", self.files_count)))
            .child(div().text_sm().child(format!("Shared Links: {}", self.links_count)))
            .child(div().text_sm().child(format!("Audio Files: {}", self.audio_count)))
    }
}
