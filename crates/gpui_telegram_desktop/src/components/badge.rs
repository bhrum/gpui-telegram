use crate::theme::TdesktopThemePalette;
use gpui::*;

pub struct Badge {
    pub count: u32,
    pub is_muted: bool,
    pub palette: TdesktopThemePalette,
}

impl Badge {
    pub fn new(count: u32, is_muted: bool) -> Self {
        Self {
            count,
            is_muted,
            palette: TdesktopThemePalette::default(),
        }
    }

    pub fn with_palette(mut self, palette: TdesktopThemePalette) -> Self {
        self.palette = palette;
        self
    }
}

impl Render for Badge {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let bg_color = if self.is_muted {
            rgb(pal.muted_badge_bg)
        } else {
            rgb(pal.unread_badge_bg)
        };

        div()
            .min_w(px(20.0))
            .h(px(20.0))
            .px(px(6.0))
            .rounded_full()
            .bg(bg_color)
            .flex()
            .items_center()
            .justify_center()
            .text_color(rgb(pal.unread_badge_fg))
            .text_size(px(11.0))
            .font_weight(FontWeight::BOLD)
            .child(self.count.to_string())
    }
}
