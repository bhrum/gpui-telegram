//! Authentic GPUI Implementation: Text Parsing Options & Presets
//!
//! Subsystem: `ui/text` | Feature: `TCG-N31`
//! Upstream Source: `Telegram/SourceFiles/ui/text/text_options.cpp:L16-L152`
//! Upstream Header: `Telegram/SourceFiles/ui/text/text_options.h:L14-L28`

use gpui::*;

pub const TEXT_PARSE_LINKS: u32 = 0x0001;
pub const TEXT_PARSE_MENTIONS: u32 = 0x0002;
pub const TEXT_PARSE_HASHTAGS: u32 = 0x0004;
pub const TEXT_PARSE_BOT_COMMANDS: u32 = 0x0008;
pub const TEXT_PARSE_MULTILINE: u32 = 0x0010;
pub const TEXT_PARSE_MARKDOWN: u32 = 0x0020;
pub const TEXT_PARSE_COLORIZED: u32 = 0x0040;
pub const TEXT_PARSE_CUSTOM_EMOJI: u32 = 0x0080;

/// Text layout direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayoutDirection {
    #[default]
    Auto,
    LeftToRight,
    RightToLeft,
}

/// Text parsing and layout limits configuration.
/// Citing `Telegram/SourceFiles/ui/text/text_options.cpp:L16-L25`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextParseOptions {
    pub flags: u32,
    pub maxw: i32,
    pub maxh: i32,
    pub dir: LayoutDirection,
}

impl Default for TextParseOptions {
    fn default() -> Self {
        Self {
            flags: 0,
            maxw: 0,
            maxh: 0,
            dir: LayoutDirection::Auto,
        }
    }
}

impl TextParseOptions {
    pub fn has_flag(&self, flag: u32) -> bool {
        (self.flags & flag) != 0
    }
}

/// Standard preset for chat history messages.
/// Citing `Telegram/SourceFiles/ui/text/text_options.cpp:L16-L25`
pub fn item_text_default_options() -> TextParseOptions {
    TextParseOptions {
        flags: TEXT_PARSE_LINKS
            | TEXT_PARSE_MENTIONS
            | TEXT_PARSE_HASHTAGS
            | TEXT_PARSE_MULTILINE
            | TEXT_PARSE_MARKDOWN
            | TEXT_PARSE_CUSTOM_EMOJI,
        maxw: 0,
        maxh: 0,
        dir: LayoutDirection::Auto,
    }
}

/// Standard preset for bot messages with clickable bot commands.
/// Citing `Telegram/SourceFiles/ui/text/text_options.cpp:L27-L37`
pub fn item_text_bot_default_options() -> TextParseOptions {
    TextParseOptions {
        flags: TEXT_PARSE_LINKS
            | TEXT_PARSE_MENTIONS
            | TEXT_PARSE_HASHTAGS
            | TEXT_PARSE_BOT_COMMANDS
            | TEXT_PARSE_MULTILINE
            | TEXT_PARSE_MARKDOWN
            | TEXT_PARSE_CUSTOM_EMOJI,
        maxw: 0,
        maxh: 0,
        dir: LayoutDirection::Auto,
    }
}

/// Preset for service messages (e.g., user joined, pin changed).
/// Citing `Telegram/SourceFiles/ui/text/text_options.cpp:L39-L49`
pub fn item_text_service_options() -> TextParseOptions {
    TextParseOptions {
        flags: TEXT_PARSE_LINKS
            | TEXT_PARSE_MENTIONS
            | TEXT_PARSE_HASHTAGS
            | TEXT_PARSE_MARKDOWN,
        maxw: 0,
        maxh: 0,
        dir: LayoutDirection::Auto,
    }
}

/// Preset for history messages without markdown monospace backticks.
/// Citing `Telegram/SourceFiles/ui/text/text_options.cpp:L50-L59`
pub fn item_text_no_mono_options() -> TextParseOptions {
    TextParseOptions {
        flags: TEXT_PARSE_LINKS
            | TEXT_PARSE_MENTIONS
            | TEXT_PARSE_HASHTAGS
            | TEXT_PARSE_MULTILINE,
        maxw: 0,
        maxh: 0,
        dir: LayoutDirection::Auto,
    }
}

/// Single-line clamped preset for user/chat display names.
/// Citing `Telegram/SourceFiles/ui/text/text_options.cpp:L71-L76`
pub fn name_text_options() -> TextParseOptions {
    TextParseOptions {
        flags: 0,
        maxw: 4096,
        maxh: 1,
        dir: LayoutDirection::Auto,
    }
}

/// Single-line dialog snippet preview in sidebar.
/// Citing `Telegram/SourceFiles/ui/text/text_options.cpp:L78-L83`
pub fn dialog_text_options(column_maximal_width_left: i32) -> TextParseOptions {
    TextParseOptions {
        flags: TEXT_PARSE_COLORIZED | TEXT_PARSE_MARKDOWN,
        maxw: column_maximal_width_left * 2,
        maxh: 1,
        dir: LayoutDirection::Auto,
    }
}

/// Webpage preview title and description options with quote padding adjustment.
/// Citing `Telegram/SourceFiles/ui/text/text_options.cpp:L85-L115`
pub fn webpage_text_options(
    msg_max_width: i32,
    msg_padding_x: i32,
    quote_padding_x: i32,
    title_line_height: i32,
) -> (TextParseOptions, TextParseOptions) {
    let available_w = (msg_max_width - msg_padding_x - quote_padding_x).max(100);
    let title = TextParseOptions {
        flags: TEXT_PARSE_MULTILINE,
        maxw: available_w,
        maxh: title_line_height * 2,
        dir: LayoutDirection::Auto,
    };
    let desc = TextParseOptions {
        flags: TEXT_PARSE_LINKS
            | TEXT_PARSE_MENTIONS
            | TEXT_PARSE_HASHTAGS
            | TEXT_PARSE_MULTILINE
            | TEXT_PARSE_MARKDOWN,
        maxw: available_w,
        maxh: 0,
        dir: LayoutDirection::Auto,
    };
    (title, desc)
}

/// Authentic GPUI Render View for text options indicator.
/// Registered in Ledger as `TextOptionsText`.
#[derive(Debug, Clone)]
pub struct TextOptionsText {
    pub options: TextParseOptions,
    pub title: String,
}

impl TextOptionsText {
    pub fn new(title: impl Into<String>, options: TextParseOptions) -> Self {
        Self {
            title: title.into(),
            options,
        }
    }
}

impl Default for TextOptionsText {
    fn default() -> Self {
        Self::new("Default Text Options", item_text_default_options())
    }
}

impl Render for TextOptionsText {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let flags = self.options.flags;
        let mut badges = Vec::new();
        if (flags & TEXT_PARSE_LINKS) != 0 { badges.push("Links"); }
        if (flags & TEXT_PARSE_MENTIONS) != 0 { badges.push("Mentions"); }
        if (flags & TEXT_PARSE_HASHTAGS) != 0 { badges.push("Hashtags"); }
        if (flags & TEXT_PARSE_BOT_COMMANDS) != 0 { badges.push("Commands"); }
        if (flags & TEXT_PARSE_MARKDOWN) != 0 { badges.push("Markdown"); }
        if (flags & TEXT_PARSE_MULTILINE) != 0 { badges.push("Multiline"); }

        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .text_size(px(13.0))
                    .font_weight(FontWeight::BOLD)
                    .child(self.title.clone())
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(4.0))
                    .children(badges.into_iter().map(|b| {
                        div()
                            .px(px(6.0))
                            .py(px(2.0))
                            .bg(rgb(0xE8F4FD))
                            .text_color(rgb(0x1B84CD))
                            .rounded(px(4.0))
                            .text_size(px(11.0))
                            .child(b)
                    }))
            )
    }
}
