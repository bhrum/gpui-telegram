//! Subsystem Integration & Unit Tests for Module 13 Part 1: Instant View Editor
//! Covers 18 Targets: IvStyleSheet, IvEditorArticleStyleEditor, IvEditorAutoPairEditor,
//! IvEditorBoxEditor, IvEditorButtonBoxEditor, IvEditorClipboardEditor,
//! IvEditorClipboardImportEditor, IvEditorCommandsEditor, IvEditorInsertSuggestionsEditor,
//! IvEditorMathBoxEditor, IvEditorPageBlocksEditor, IvEditorPageListEditor,
//! IvEditorPageMediaEditor, IvEditorPagePathEditor, IvEditorPageTableGridEditor,
//! IvEditorPreparedSelectionEditor, IvEditorSessionEditor, IvEditorStateEditor.

use gpui_telegram_desktop::theme::iv::iv_style::IvStyleSheet;
use gpui_telegram_desktop::views::iv::editor::iv_editor_article_style::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_auto_pair::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_box::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_button_box::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_clipboard::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_clipboard_import::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_commands::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_insert_suggestions::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_math_box::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_page_blocks::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_page_list::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_page_media::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_page_path::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_page_table_grid::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_prepared_selection::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_session::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_state::*;

#[test]
fn test_iv_style_sheet_tokens() {
    let sheet = IvStyleSheet::default();
    let min_sz = sheet.min_window_size();
    assert_eq!(min_sz.width, gpui::px(384.0));
    assert_eq!(min_sz.height, gpui::px(480.0));

    let def_sz = sheet.default_window_size();
    assert_eq!(def_sz.width, gpui::px(800.0));
    assert_eq!(def_sz.height, gpui::px(600.0));

    assert_eq!(sheet.toolbar_button_size, gpui::px(36.0));
    assert_eq!(sheet.suggestions_max_rows, 5);
}

#[test]
fn test_iv_editor_article_style_line_metrics() {
    let mut editor = IvEditorArticleStyleEditor::new();
    let mut doc = ArticleTextDocument::new();
    doc = doc.with_block("Heading 1", vec![120.0]);
    doc = doc.with_block("A very long paragraph line that stretches across", vec![350.0, 200.0]);

    let max_w = editor.max_visual_line_width(&doc);
    assert_eq!(max_w, 350.0);

    let constrained_w = editor.max_visual_line_width_for_width(&doc, 300.0);
    assert_eq!(constrained_w, 300.0);
}

#[test]
fn test_iv_editor_auto_pair_brackets_and_backspace() {
    let mut buf = AutoPairBuffer::new("Hello ");
    assert!(buf.handle_char('('));
    assert_eq!(buf.text, "Hello ()");
    assert_eq!(buf.cursor, 7);

    // Typing closing bracket skips
    assert!(buf.handle_char(')'));
    assert_eq!(buf.text, "Hello ()");
    assert_eq!(buf.cursor, 8);

    // Backspace removes pair
    let mut buf2 = AutoPairBuffer::new("test");
    buf2.cursor = 4;
    buf2.handle_char('{');
    assert_eq!(buf2.text, "test{}");
    assert_eq!(buf2.cursor, 5);
    assert!(buf2.handle_backspace());
    assert_eq!(buf2.text, "test");
    assert_eq!(buf2.cursor, 4);

    // Selection wrap
    let mut buf3 = AutoPairBuffer::new("wrapped text");
    buf3.selection = Some((0, 7));
    assert!(buf3.handle_char('"'));
    assert_eq!(buf3.text, "\"wrapped\" text");
}

#[test]
fn test_iv_editor_box_toolbar_actions() {
    let mut box_editor = IvEditorBoxEditor::new();
    assert!(box_editor.is_open);
    assert!(box_editor.toolbar_actions.len() >= 10);

    assert!(box_editor.trigger_action(ToolbarActionId::Bold));
    assert_eq!(box_editor.active_action, Some(ToolbarActionId::Bold));
    assert_eq!(box_editor.action_history.len(), 1);

    // Toggling again deactivates it
    assert!(box_editor.trigger_action(ToolbarActionId::Bold));
    let bold_item = box_editor.toolbar_actions.iter().find(|a| a.id == ToolbarActionId::Bold).unwrap();
    assert_eq!(bold_item.state, ToolbarButtonState::Inactive);

    box_editor.close();
    assert!(!box_editor.is_open);
}

#[test]
fn test_iv_editor_button_box_validation() {
    assert!(validate_button_url("https://t.me/telegram").is_ok());
    assert!(validate_button_url("tg://resolve?domain=telegram").is_ok());
    assert!(validate_button_url("ftp://invalid.com").is_err());
    assert!(validate_button_url("").is_err());

    let mut btn_editor = IvEditorButtonBoxEditor::new();
    btn_editor.set_label("Read More");
    btn_editor.set_url("https://telegram.org");
    btn_editor.set_button_color(MarkupButtonColor::Primary);
    btn_editor.set_separate_line(true);

    assert!(btn_editor.commit());
    assert!(!btn_editor.is_open);
    let res = btn_editor.committed_result.unwrap();
    assert_eq!(res.data.button_text, "Read More");
    assert_eq!(res.data.url, "https://telegram.org");
    assert!(res.separate_line);
}

#[test]
fn test_iv_editor_clipboard_serialization() {
    let mut clip = IvEditorClipboardEditor::new();
    let blocks = vec![
        RichPageBlockCopy {
            kind: "paragraph".to_string(),
            text: "Hello world".to_string(),
            language: None,
            heading_level: None,
        },
        RichPageBlockCopy {
            kind: "code".to_string(),
            text: "fn main() {}".to_string(),
            language: Some("rust".to_string()),
            heading_level: None,
        },
    ];

    clip.copy_blocks(blocks.clone());
    let data = clip.paste().unwrap();
    assert_eq!(data.block_count(), 2);

    let bytes = serialize_clipboard_data(&data);
    let deserialized = deserialize_clipboard_data(&bytes).unwrap();
    assert_eq!(deserialized, data);
}

#[test]
fn test_iv_editor_clipboard_import_markdown() {
    let md = "# Title 1\n\n> This is a quote\n\n- List item 1\n- List item 2\n\n```rust\nlet x = 42;\n```\n\nTrailing paragraph.";
    let res = blocks_from_markdown(md, 50);
    assert_eq!(res.blocks.len(), 6);
    assert_eq!(res.blocks[0].kind, "heading");
    assert_eq!(res.blocks[0].level, Some(1));
    assert_eq!(res.blocks[1].kind, "blockquote");
    assert_eq!(res.blocks[2].kind, "bullet_list_item");
    assert_eq!(res.blocks[4].kind, "code");
    assert_eq!(res.blocks[5].kind, "paragraph");

    let pid = imported_media_placeholder_id(42);
    assert_eq!(imported_media_placeholder_index(pid), Some(42));
    assert_eq!(imported_media_placeholder_index(100), None);
}

#[test]
fn test_iv_editor_commands_enter_pipeline() {
    let mut editor = IvEditorCommandsEditor::new();
    let mut ctx = CommandContext {
        caret_at_start: true,
        has_previous_editable: false,
        is_top_level_paragraph_or_heading: false,
        has_active_list_surface: false,
        ..Default::default()
    };
    let cmd = editor.dispatch_enter(ctx.clone());
    assert_eq!(cmd, Some(EnterCommandKind::InsertLeadingParagraph));

    ctx.caret_at_start = false;
    ctx.enter.in_list = true;
    let cmd2 = editor.dispatch_enter(ctx.clone());
    assert_eq!(cmd2, Some(EnterCommandKind::ListEnter));

    assert_eq!(editor.executed_commands.len(), 2);
}

#[test]
fn test_iv_editor_insert_suggestions_filtering() {
    let mut editor = IvEditorInsertSuggestionsEditor::new();
    assert!(editor.apply_query("h1"));
    assert_eq!(editor.filtered.len(), 1);
    assert_eq!(editor.filtered[0].command, InsertSuggestionCommand::Heading1);

    let chosen = editor.choose_selected();
    assert_eq!(chosen, Some(InsertSuggestionCommand::Heading1));
    assert!(!editor.is_active);

    editor.apply_query("list");
    assert!(editor.filtered.len() >= 3);
    editor.select_next();
    assert_eq!(editor.selected_index, 1);
}

#[test]
fn test_iv_editor_math_box_validation_and_presets() {
    assert!(validate_latex_math("e^{i\\pi}=-1").is_ok());
    assert!(validate_latex_math("\\frac{a}{b}").is_ok());
    assert!(validate_latex_math("\\frac{a}{b").is_err());
    assert!(validate_latex_math("").is_err());

    let mut math_editor = IvEditorMathBoxEditor::new();
    math_editor.apply_sample(2);
    assert_eq!(math_editor.source, "\\sin^2\\alpha+\\cos^2\\alpha=1");
    math_editor.set_separate_line(true);

    assert!(math_editor.commit());
    assert!(!math_editor.is_open);
    let res = math_editor.committed_result.unwrap();
    assert_eq!(res.source, "\\sin^2\\alpha+\\cos^2\\alpha=1");
    assert!(res.separate_line);
}

#[test]
fn test_iv_editor_page_blocks_factory_and_seam() {
    let mut p1 = make_paragraph_block();
    p1.text = "First paragraph.".to_string();

    let mut p2 = make_paragraph_block();
    p2.text = "Second paragraph.".to_string();

    let seam = append_paragraph_seam(&mut p1, p2);
    assert_eq!(seam, 16);
    assert_eq!(p1.text, "First paragraph. Second paragraph.");

    let mut blocks_editor = IvEditorPageBlocksEditor::new();
    let pos = blocks_editor.insert_block(InsertAction {
        block_type: InsertBlockType::Heading,
        heading_level: 2,
        ..Default::default()
    });
    assert_eq!(pos, 1);
    assert_eq!(blocks_editor.blocks[1].kind, InsertBlockType::Heading);
    assert_eq!(blocks_editor.blocks[1].level, 2);
}

#[test]
fn test_iv_editor_page_list_marker_and_roman() {
    assert_eq!(to_roman_numeral(1), "I");
    assert_eq!(to_roman_numeral(4), "IV");
    assert_eq!(to_roman_numeral(9), "IX");
    assert_eq!(to_roman_numeral(42), "XLII");

    let mut list_editor = IvEditorPageListEditor::new();
    list_editor.set_style(ListStyle::Task);
    list_editor.add_item("Buy milk");
    assert_eq!(list_editor.list.items[1].task_state, TaskState::Unchecked);

    assert!(list_editor.toggle_task(1));
    assert_eq!(list_editor.list.items[1].task_state, TaskState::Checked);
}

#[test]
fn test_iv_editor_page_media_albums_and_spoilers() {
    let mut media_editor = IvEditorPageMediaEditor::new();
    media_editor.add_media(MediaKind::Video, 1920, 1080, "Video caption");
    assert_eq!(media_editor.items.len(), 2);

    assert!(media_editor.toggle_spoiler(1));
    assert!(media_editor.items[1].has_spoiler);

    let albums = group_media_items(&media_editor.items);
    assert_eq!(albums.len(), 1);
    assert_eq!(albums[0].len(), 2);
}

#[test]
fn test_iv_editor_page_path_breadcrumbs() {
    let path_editor = IvEditorPagePathEditor::new();
    let breadcrumb = path_editor.breadcrumb_string();
    assert!(breadcrumb.contains("Document"));
    assert!(breadcrumb.contains("Block[0]"));
    assert!(breadcrumb.contains("Text"));

    let child_container = block_children_container(&path_editor.current_leaf.block);
    assert_eq!(child_container.steps.len(), 1);
}

#[test]
fn test_iv_editor_page_table_grid_occupancy() {
    let spans = vec![
        vec![(1, 2), (1, 1)], // col 0 has colspan 2, col 2 has colspan 1
        vec![(1, 1), (1, 1), (1, 1)],
    ];
    let grid = build_table_grid(&spans);
    assert_eq!(grid.row_count, 2);
    assert_eq!(grid.column_count, 3);
    assert_eq!(grid.cells.len(), 5);

    let mut table_editor = IvEditorPageTableGridEditor::new();
    table_editor.grid = grid;
    table_editor.select_range(0, 1, 0, 2);
    assert_eq!(table_editor.selected_cells.len(), 1);
}

#[test]
fn test_iv_editor_prepared_selection_and_session() {
    let mut sel_editor = IvEditorPreparedSelectionEditor::new();
    sel_editor.set_range(5, 12, true);
    assert_eq!(sel_editor.selection.range.from, 5);
    assert_eq!(sel_editor.selection.range.till, 12);
    assert_eq!(sel_editor.selection.range.len(), 7);

    let mut session_editor = IvEditorSessionEditor::with_session(42, true);
    assert!(session_editor.can_author_rich_messages());
    assert!(!session_editor.premium_emoji_forbidden(100, true));

    session_editor.open_compose_box(1001, 0, "Test Article");
    assert!(session_editor.is_compose_box_open(1001, 0));
    assert!(session_editor.close_window(1001, 0));
    assert!(!session_editor.is_compose_box_open(1001, 0));
}

#[test]
fn test_iv_editor_state_machine_undo_redo() {
    let mut state = IvEditorStateEditor::new();
    assert_eq!(state.blocks.len(), 1);

    state.cursor_position = state.blocks[0].len();
    state.insert_text(" Additional text.");
    assert_eq!(state.undo_stack.len(), 1);
    assert!(state.blocks[0].ends_with("Additional text."));

    assert!(state.undo());
    assert_eq!(state.blocks[0], "Welcome to Instant View Editor.");
    assert_eq!(state.redo_stack.len(), 1);

    assert!(state.redo());
    assert!(state.blocks[0].ends_with("Additional text."));

    // Splitting active block
    state.cursor_position = 7;
    assert!(state.split_active_block());
    assert_eq!(state.blocks.len(), 2);
    assert_eq!(state.blocks[0], "Welcome");

    // Formatting selection
    state.selection = Some((0, 7));
    state.active_block_index = 0;
    assert!(state.apply_formatting(TextFormattingAction::Bold));
    assert_eq!(state.blocks[0], "**Welcome**");
}
