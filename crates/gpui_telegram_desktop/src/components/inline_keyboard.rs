use gpui::*;

pub struct InlineKeyboardButton {
    pub text: String,
    pub callback_data: String,
}

pub struct InlineKeyboard {
    pub rows: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboard {
    pub fn new(rows: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self { rows }
    }
}

impl Render for InlineKeyboard {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut col = div().flex().flex_col().gap(px(4.0));
        for row in &self.rows {
            let mut row_div = div().flex().gap(px(4.0));
            for btn in row {
                row_div = row_div.child(
                    div()
                        .flex_1()
                        .py(px(4.0))
                        .px(px(8.0))
                        .rounded(px(4.0))
                        .bg(rgb(0xE3F2FD))
                        .text_color(rgb(0x1976D2))
                        .child(btn.text.clone()),
                );
            }
            col = col.child(row_div);
        }
        col
    }
}
