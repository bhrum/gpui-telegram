//! Instant View Editor Commands & Enter Execution Chain (`iv_editor_commands.h/.cpp`)
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

/// Enter command actions processed in prioritized sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnterCommandKind {
    InsertLeadingParagraph,
    ListEnter,
    EscapeEmptyLine,
    HeadingEnter,
    FooterEnter,
    ParagraphEnter,
    QuoteEnter,
    SubmitSingleLineField,
}

/// Contextual state at the active caret when Enter key is pressed.
#[derive(Debug, Clone, Copy, Default)]
pub struct ActiveEnterContext {
    pub in_list: bool,
    pub in_heading: bool,
    pub in_quote: bool,
    pub in_footer: bool,
    pub in_single_line_field: bool,
    pub is_empty_line: bool,
}

/// Parameters passed through the Enter command chain.
#[derive(Debug, Clone, Default)]
pub struct CommandContext {
    pub caret_at_start: bool,
    pub caret_at_end: bool,
    pub target_ordinal: i32,
    pub enter: ActiveEnterContext,
    pub has_previous_editable: bool,
    pub is_top_level_paragraph_or_heading: bool,
    pub has_active_list_surface: bool,
}

/// Evaluates if a leading paragraph should be inserted before active element.
pub fn insert_leading_paragraph(context: &mut CommandContext) -> bool {
    if !context.caret_at_start
        || context.has_previous_editable
        || context.is_top_level_paragraph_or_heading
        || context.has_active_list_surface
    {
        return false;
    }
    context.target_ordinal = 0;
    true
}

pub fn list_enter(context: &mut CommandContext) -> bool {
    if !context.enter.in_list {
        return false;
    }
    context.target_ordinal += 1;
    true
}

pub fn escape_empty_line(context: &mut CommandContext) -> bool {
    if !context.enter.is_empty_line {
        return false;
    }
    context.target_ordinal += 1;
    true
}

pub fn heading_enter(context: &mut CommandContext) -> bool {
    if !context.enter.in_heading {
        return false;
    }
    context.target_ordinal += 1;
    true
}

pub fn footer_enter(context: &mut CommandContext) -> bool {
    if !context.enter.in_footer {
        return false;
    }
    context.target_ordinal += 1;
    true
}

pub fn paragraph_enter(context: &mut CommandContext) -> bool {
    if context.enter.in_list || context.enter.in_heading || context.enter.in_quote {
        return false;
    }
    context.target_ordinal += 1;
    true
}

pub fn quote_enter(context: &mut CommandContext) -> bool {
    if !context.enter.in_quote {
        return false;
    }
    context.target_ordinal += 1;
    true
}

pub fn submit_single_line_field(context: &mut CommandContext) -> bool {
    if !context.enter.in_single_line_field {
        return false;
    }
    context.target_ordinal = -1;
    true
}

/// Evaluates the 8-stage Enter command pipeline in priority order.
/// Equivalent to C++ RunEnterChain.
pub fn run_enter_chain(context: &mut CommandContext) -> Option<EnterCommandKind> {
    if insert_leading_paragraph(context) {
        return Some(EnterCommandKind::InsertLeadingParagraph);
    }
    if list_enter(context) {
        return Some(EnterCommandKind::ListEnter);
    }
    if escape_empty_line(context) {
        return Some(EnterCommandKind::EscapeEmptyLine);
    }
    if heading_enter(context) {
        return Some(EnterCommandKind::HeadingEnter);
    }
    if footer_enter(context) {
        return Some(EnterCommandKind::FooterEnter);
    }
    if paragraph_enter(context) {
        return Some(EnterCommandKind::ParagraphEnter);
    }
    if quote_enter(context) {
        return Some(EnterCommandKind::QuoteEnter);
    }
    if submit_single_line_field(context) {
        return Some(EnterCommandKind::SubmitSingleLineField);
    }
    None
}

/// Instant View command dispatcher and history tracker.
#[derive(Debug, Clone)]
pub struct IvEditorCommandsEditor {
    pub is_active: bool,
    pub executed_commands: Vec<EnterCommandKind>,
    pub last_target_ordinal: i32,
}

impl Default for IvEditorCommandsEditor {
    fn default() -> Self {
        Self {
            is_active: false,
            executed_commands: Vec::new(),
            last_target_ordinal: -1,
        }
    }
}

impl IvEditorCommandsEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dispatch_enter(&mut self, mut context: CommandContext) -> Option<EnterCommandKind> {
        let cmd = run_enter_chain(&mut context);
        if let Some(c) = cmd {
            self.executed_commands.push(c);
            self.last_target_ordinal = context.target_ordinal;
        }
        cmd
    }
}

impl Render for IvEditorCommandsEditor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(12.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child("Instant View Enter Command Pipeline")
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .child(format!(
                        "Executed commands: {} | Last target: {}",
                        self.executed_commands.len(), self.last_target_ordinal
                    ))
            )
    }
}
