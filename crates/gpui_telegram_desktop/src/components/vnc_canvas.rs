use crate::theme::{assets::{IconName, tg_icon}, TdesktopThemePalette};
use gpui::*;

pub struct VncCanvas {
    pub remote_connected: bool,
    pub frame_source: Option<String>,
    pub resolution: (u32, u32),
    pub host_name: String,
    pub palette: TdesktopThemePalette,
}

impl VncCanvas {
    pub fn new() -> Self {
        Self {
            remote_connected: false,
            frame_source: None,
            resolution: (0, 0),
            host_name: String::new(),
            palette: TdesktopThemePalette::default(),
        }
    }

    pub fn with_palette(mut self, palette: TdesktopThemePalette) -> Self {
        self.palette = palette;
        self
    }

    pub fn set_frame(&mut self, host_name: impl Into<String>, source: Option<String>, resolution: (u32, u32)) {
        self.host_name = host_name.into();
        self.frame_source = source;
        self.resolution = resolution;
        self.remote_connected = self.frame_source.is_some();
    }
}

impl Default for VncCanvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for VncCanvas {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;

        if let Some(ref src) = self.frame_source {
            return div()
                .w_full()
                .h_full()
                .bg(rgb(pal.window_bg))
                .flex()
                .items_center()
                .justify_center()
                .overflow_hidden()
                .child(img(src.clone()).w_full().h_full().object_fit(ObjectFit::Contain))
                .into_any_element();
        }

        div()
            .w_full()
            .h_full()
            .bg(rgb(pal.window_bg))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(10.0))
            .text_color(rgb(pal.text_primary))
            .child(tg_icon(IconName::ComputerDesktop, 44.0, pal.text_secondary))
            .child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(if self.remote_connected {
                        if self.host_name.is_empty() {
                            "Remote Desktop Connected".to_string()
                        } else {
                            format!("{} ({}x{})", self.host_name, self.resolution.0, self.resolution.1)
                        }
                    } else {
                        "No active remote desktop stream".to_string()
                    }),
            )
            .into_any_element()
    }
}
