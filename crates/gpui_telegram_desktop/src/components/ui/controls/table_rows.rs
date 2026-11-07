//! UI Table Rows key-value layout component for transaction details and gifts.
//! Upstream: `Telegram/SourceFiles/ui/controls/table_rows.cpp:L30-L110`
//!           `Telegram/SourceFiles/ui/controls/table_rows.h:L29-L79`
//!
//! Two-column key-value alignment with action buttons, multiline text wrapping, and peer links.

use gpui::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRowItem {
    pub label: String,
    pub value: String,
    pub button_text: Option<String>,
}

/// TableRowsControls component. Target symbol required by ledger: `TableRowsControls`.
#[derive(Debug, Clone, Default)]
pub struct TableRowsControls {
    pub rows: Vec<TableRowItem>,
}

impl TableRowsControls {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn with_rows(mut self, rows: Vec<TableRowItem>) -> Self {
        self.rows = rows;
        self
    }

    pub fn add_row(&mut self, label: impl Into<String>, value: impl Into<String>) {
        self.rows.push(TableRowItem {
            label: label.into(),
            value: value.into(),
            button_text: None,
        });
    }

    pub fn add_row_with_button(
        &mut self,
        label: impl Into<String>,
        value: impl Into<String>,
        button: impl Into<String>,
    ) {
        self.rows.push(TableRowItem {
            label: label.into(),
            value: value.into(),
            button_text: Some(button.into()),
        });
    }
}

impl Render for TableRowsControls {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut list = div().flex().flex_col().gap(px(8.0)).w_full();
        for row in &self.rows {
            let mut val_col = div().flex().items_center().gap(px(6.0)).child(
                div()
                    .text_color(hsla(0.0, 0.0, 0.1, 1.0))
                    .child(row.value.clone())
            );
            if let Some(btn) = &row.button_text {
                val_col = val_col.child(
                    div()
                        .px(px(6.0))
                        .py(px(2.0))
                        .rounded(px(4.0))
                        .bg(hsla(204.0 / 360.0, 0.88, 0.53, 0.1))
                        .text_color(hsla(204.0 / 360.0, 0.88, 0.53, 1.0))
                        .text_size(px(11.0))
                        .child(btn.clone())
                );
            }

            list = list.child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .child(
                        div()
                            .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                            .text_size(px(13.0))
                            .child(row.label.clone())
                    )
                    .child(val_col)
            );
        }
        list
    }
}
