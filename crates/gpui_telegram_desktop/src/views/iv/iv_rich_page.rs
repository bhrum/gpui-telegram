//! Target Symbol: IvRichPage
//! Module 13: Instant View Rich Page Hierarchical Document AST Model
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const COLLAGE_MAX_ITEMS: usize = 10;
pub const MAX_PARSED_BLOCK_DEPTH: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OrderedMarkerType {
    #[default]
    Numbers,
    LettersLower,
    LettersUpper,
    RomanLower,
    RomanUpper,
}

pub fn to_roman_numeral(mut num: usize, uppercase: bool) -> String {
    if num == 0 || num > 3999 {
        return num.to_string();
    }
    let table = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut result = String::new();
    for (val, symbol) in table {
        while num >= val {
            result.push_str(symbol);
            num -= val;
        }
    }

    if uppercase {
        result
    } else {
        result.to_lowercase()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RichBlock {
    Heading { level: u8, text: String, anchor: Option<String> },
    Paragraph { text: String },
    Blockquote { text: String },
    Code { lang: Option<String>, text: String },
    List { marker: OrderedMarkerType, items: Vec<String> },
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    Divider,
}

#[derive(Debug, Clone)]
pub struct IvRichPage {
    pub page_id: u64,
    pub title: String,
    pub author: String,
    pub date: String,
    pub blocks: Vec<RichBlock>,
}

impl Default for IvRichPage {
    fn default() -> Self {
        Self {
            page_id: 1,
            title: "Instant View Article".to_string(),
            author: "Telegram".to_string(),
            date: "November 2026".to_string(),
            blocks: vec![
                RichBlock::Heading {
                    level: 1,
                    text: "Introduction to Instant View".to_string(),
                    anchor: Some("intro".to_string()),
                },
                RichBlock::Paragraph {
                    text: "Instant View pages load instantly with zero data waste.".to_string(),
                },
                RichBlock::List {
                    marker: OrderedMarkerType::Numbers,
                    items: vec![
                        "High performance rendering".to_string(),
                        "Distraction-free reading".to_string(),
                        "Offline archive export".to_string(),
                    ],
                },
            ],
        }
    }
}

impl IvRichPage {
    pub fn new(page_id: u64, title: impl Into<String>) -> Self {
        Self {
            page_id,
            title: title.into(),
            author: String::new(),
            date: String::new(),
            blocks: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: RichBlock) {
        self.blocks.push(block);
    }

    pub fn total_blocks(&self) -> usize {
        self.blocks.len()
    }
}

impl Render for IvRichPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut article = div()
            .flex()
            .flex_col()
            .w_full()
            .max_w(px(680.0))
            .p(px(24.0))
            .bg(rgb(0x191D24))
            .rounded(px(12.0))
            .gap(px(14.0))
            .shadow_lg();

        // Title and Byline
        article = article.child(
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(26.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xFFFFFF))
                        .child(self.title.clone()),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgb(0x7F8796))
                        .child(format!("{} • {}", self.author, self.date)),
                ),
        );

        // Blocks
        for block in &self.blocks {
            let el = match block {
                RichBlock::Heading { level, text, .. } => {
                    let size = match level {
                        1 => px(20.0),
                        2 => px(17.0),
                        _ => px(15.0),
                    };
                    div()
                        .text_size(size)
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xEEF1F6))
                        .child(text.clone())
                }
                RichBlock::Paragraph { text } => div()
                    .text_size(px(14.0))
                    .text_color(rgb(0xD1D5DF))
                    .line_height(relative(1.5))
                    .child(text.clone()),
                RichBlock::Blockquote { text } => div()
                    .pl(px(12.0))
                    .border_l_2()
                    .border_color(rgb(0x2AABEE))
                    .text_size(px(14.0))
                    .text_color(rgb(0x88C0D0))
                    .child(text.clone()),
                RichBlock::Code { text, .. } => div()
                    .p(px(10.0))
                    .rounded(px(6.0))
                    .bg(rgb(0x13161C))
                    .text_size(px(12.0))
                    .text_color(rgb(0xA3BE8C))
                    .child(text.clone()),
                RichBlock::List { marker, items } => {
                    let mut col = div().flex().flex_col().gap(px(4.0));
                    for (i, item) in items.iter().enumerate() {
                        let bullet = match marker {
                            OrderedMarkerType::Numbers => format!("{}. ", i + 1),
                            OrderedMarkerType::RomanLower => {
                                format!("{}. ", to_roman_numeral(i + 1, false))
                            }
                            OrderedMarkerType::RomanUpper => {
                                format!("{}. ", to_roman_numeral(i + 1, true))
                            }
                            OrderedMarkerType::LettersLower => {
                                format!("{}. ", (b'a' + (i as u8 % 26)) as char)
                            }
                            OrderedMarkerType::LettersUpper => {
                                format!("{}. ", (b'A' + (i as u8 % 26)) as char)
                            }
                        };
                        col = col.child(
                            div()
                                .text_size(px(14.0))
                                .text_color(rgb(0xD1D5DF))
                                .child(format!("{}{}", bullet, item)),
                        );
                    }
                    col
                }
                RichBlock::Table { headers, rows } => div()
                    .text_size(px(13.0))
                    .text_color(rgb(0x81A1C1))
                    .child(format!("Table: {} cols, {} rows", headers.len(), rows.len())),
                RichBlock::Divider => div().h(px(1.0)).bg(rgb(0x2C323E)),
            };
            article = article.child(el);
        }

        article
    }
}
