use crate::theme::TdesktopThemePalette;
use gpui::*;
use std::sync::Arc;

pub struct InlineKeyboardButton {
    pub text: String,
    pub callback_data: String,
}

pub struct InlineKeyboard {
    pub rows: Vec<Vec<InlineKeyboardButton>>,
    pub palette: TdesktopThemePalette,
    pub last_clicked_callback: Option<String>,
    pub on_button_click: Option<Arc<dyn Fn(String) + Send + Sync>>,
}

impl InlineKeyboard {
    pub fn new(rows: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self {
            rows,
            palette: TdesktopThemePalette::default(),
            last_clicked_callback: None,
            on_button_click: None,
        }
    }

    pub fn with_palette(mut self, palette: TdesktopThemePalette) -> Self {
        self.palette = palette;
        self
    }

    pub fn trigger_button(&mut self, callback_data: &str) {
        self.last_clicked_callback = Some(callback_data.to_string());
        if let Some(ref cb) = self.on_button_click {
            cb(callback_data.to_string());
        }
    }
}

impl Render for InlineKeyboard {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let pal = self.palette;
        let btn_bg = TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.16);
        let btn_hover = TdesktopThemePalette::rgba_with_alpha(pal.accent, 0.28);

        let mut col = div().flex().flex_col().gap(px(4.0));
        for row in &self.rows {
            let mut row_div = div().flex().gap(px(4.0));
            for btn in row {
                let data = btn.callback_data.clone();
                row_div = row_div.child(
                    div()
                        .cursor_pointer()
                        .flex_1()
                        .py(px(6.0))
                        .px(px(10.0))
                        .rounded(px(6.0))
                        .bg(btn_bg)
                        .hover(move |s| s.bg(btn_hover))
                        .border_1()
                        .border_color(rgb(pal.border_fg))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(12.5))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(pal.text_link))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                this.trigger_button(&data);
                                cx.notify();
                            }),
                        )
                        .child(btn.text.clone()),
                );
            }
            col = col.child(row_div);
        }
        col
    }
}
