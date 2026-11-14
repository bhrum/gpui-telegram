//! Fullscreen MediaViewer Overlay (`MediaViewer` & `MediaViewerItem`).

use gpui::*;

use crate::i18n::tr_key_or;
use crate::theme::TdesktopThemePalette;
use crate::views::stage::media_viewer_ui::{render_media_viewer_overlay, MediaViewerUiAction};

#[derive(Debug, Clone)]
pub struct MediaViewerItem {
    pub id: u64,
    pub url: String,
    pub is_video: bool,
    pub caption: Option<String>,
    pub sender_name: String,
    pub date: String,
    pub duration_secs: Option<u32>,
}

pub struct MediaViewer {
    pub items: Vec<MediaViewerItem>,
    pub current_index: usize,
    pub zoom_level: f32,
    pub is_playing: bool,
    pub is_open: bool,
    pub should_close: bool,
    pub rotation_deg: u32,
    pub open_editor_requested: bool,
    pub saved_notice: Option<String>,
    pub pan_offset: Point<Pixels>,
    pub drag_origin: Option<Point<Pixels>>,
    pub pending_download_url: Option<String>,
    pub pending_forward_item_id: Option<u64>,
    pub palette: TdesktopThemePalette,
    pub lang_id: String,
    pub focus_handle: Option<FocusHandle>,
}

impl MediaViewer {
    /// Truthful single-item viewer: only displays the actual clicked media item.
    pub fn new(media_url: String, is_video: bool) -> Self {
        let item = MediaViewerItem {
            id: 1,
            url: media_url,
            is_video,
            caption: None,
            sender_name: String::new(),
            date: String::new(),
            duration_secs: if is_video { Some(0) } else { None },
        };
        Self::from_gallery(vec![item], 0)
    }

    /// Explicit 3-item gallery fixture for legacy MediaViewer navigation unit tests.
    pub fn new_demo() -> Self {
        let items = vec![
            MediaViewerItem {
                id: 1,
                url: "https://example.com/photo1.jpg".to_string(),
                is_video: false,
                caption: Some("Telegram Desktop GPUI Architecture Showcase".to_string()),
                sender_name: "Alice Smith".to_string(),
                date: "Today at 10:15".to_string(),
                duration_secs: None,
            },
            MediaViewerItem {
                id: 2,
                url: "https://example.com/photo2.jpg".to_string(),
                is_video: false,
                caption: Some("Virtualized Message Stage Design".to_string()),
                sender_name: "Alice Smith".to_string(),
                date: "Today at 10:16".to_string(),
                duration_secs: None,
            },
            MediaViewerItem {
                id: 3,
                url: "https://example.com/clip.mp4".to_string(),
                is_video: true,
                caption: Some("Hardware Accelerated Canvas Scroll Recording".to_string()),
                sender_name: "Me".to_string(),
                date: "Today at 10:20".to_string(),
                duration_secs: Some(45),
            },
        ];
        Self::from_gallery(items, 0)
    }

    pub fn from_gallery(items: Vec<MediaViewerItem>, start_index: usize) -> Self {
        let idx = if items.is_empty() { 0 } else { start_index.min(items.len() - 1) };
        Self {
            items,
            current_index: idx,
            zoom_level: 1.0,
            is_playing: false,
            is_open: true,
            should_close: false,
            rotation_deg: 0,
            open_editor_requested: false,
            saved_notice: None,
            pan_offset: Point::default(),
            drag_origin: None,
            pending_download_url: None,
            pending_forward_item_id: None,
            palette: TdesktopThemePalette::default(),
            lang_id: "en".to_string(),
            focus_handle: None,
        }
    }

    pub fn apply_theme(&mut self, palette: TdesktopThemePalette, lang_id: impl Into<String>) {
        self.palette = palette;
        self.lang_id = lang_id.into();
    }

    pub fn next(&mut self) {
        if self.current_index + 1 < self.items.len() {
            self.current_index += 1;
            self.reset_zoom();
            self.is_playing = false;
        }
    }

    pub fn prev(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.reset_zoom();
            self.is_playing = false;
        }
    }

    pub fn zoom_in(&mut self) {
        self.zoom_level = (self.zoom_level + 0.25).min(3.0);
    }

    pub fn zoom_out(&mut self) {
        self.zoom_level = (self.zoom_level - 0.25).max(0.5);
    }

    pub fn reset_zoom(&mut self) {
        self.zoom_level = 1.0;
        self.rotation_deg = 0;
        self.pan_offset = Point::default();
    }

    pub fn rotate_right(&mut self) {
        self.rotation_deg = (self.rotation_deg + 90) % 360;
    }

    pub fn toggle_play(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn current_item(&self) -> Option<&MediaViewerItem> {
        self.items.get(self.current_index)
    }

    fn handle_action(&mut self, action: MediaViewerUiAction, cx: &mut Context<Self>) {
        match action {
            MediaViewerUiAction::Close => self.should_close = true,
            MediaViewerUiAction::Prev => self.prev(),
            MediaViewerUiAction::Next => self.next(),
            MediaViewerUiAction::SelectIndex(i) => {
                if i < self.items.len() {
                    self.current_index = i;
                    self.reset_zoom();
                    self.is_playing = false;
                }
            }
            MediaViewerUiAction::ZoomIn => self.zoom_in(),
            MediaViewerUiAction::ZoomOut => self.zoom_out(),
            MediaViewerUiAction::ResetZoom => self.reset_zoom(),
            MediaViewerUiAction::RotateRight => self.rotate_right(),
            MediaViewerUiAction::TogglePlay => self.toggle_play(),
            MediaViewerUiAction::SaveDownload => {
                if let Some(url) = self.current_item().map(|i| i.url.clone()) {
                    self.pending_download_url = Some(url);
                    self.saved_notice = Some(tr_key_or(&self.lang_id, "lng_mediaview_saved", "Saved").to_string());
                }
            }
            MediaViewerUiAction::Forward => {
                self.pending_forward_item_id = self.current_item().map(|i| i.id);
            }
            MediaViewerUiAction::OpenEditor => self.open_editor_requested = true,
            MediaViewerUiAction::PanStart(p) => self.drag_origin = Some(p),
            MediaViewerUiAction::PanMove(p) => {
                if let Some(orig) = self.drag_origin {
                    self.pan_offset = point(self.pan_offset.x + (p.x - orig.x), self.pan_offset.y + (p.y - orig.y));
                    self.drag_origin = Some(p);
                }
            }
            MediaViewerUiAction::PanEnd => self.drag_origin = None,
        }
        cx.notify();
    }
}

impl Default for MediaViewer {
    fn default() -> Self {
        Self::new(String::new(), false)
    }
}

impl Render for MediaViewer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.focus_handle.is_none() {
            self.focus_handle = Some(cx.focus_handle());
        }
        let focus = self.focus_handle.clone().unwrap();

        div()
            .id("media-viewer-root")
            .track_focus(&focus)
            .key_context("MediaViewer")
            .on_key_down(cx.listener(|this, ev: &KeyDownEvent, _, cx| {
                match ev.keystroke.key.as_str() {
                    "escape" => this.should_close = true,
                    "left" => this.prev(),
                    "right" => this.next(),
                    "r" => this.rotate_right(),
                    "0" => this.reset_zoom(),
                    "=" | "+" => this.zoom_in(),
                    "-" => this.zoom_out(),
                    "space" => this.toggle_play(),
                    _ => return,
                }
                cx.notify();
            }))
            .size_full()
            .child(render_media_viewer_overlay(
                &self.items,
                self.current_index,
                self.zoom_level,
                self.rotation_deg,
                self.pan_offset,
                self.is_playing,
                self.saved_notice.as_deref(),
                &self.palette,
                &self.lang_id,
                cx,
                |this, act, cx| this.handle_action(act, cx),
            ))
    }
}
