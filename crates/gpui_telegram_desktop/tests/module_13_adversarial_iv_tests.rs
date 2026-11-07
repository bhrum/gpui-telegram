//! Module 13 Adversarial Stress & Boundary Tests: Instant View (IV)
//! Rigorously challenges parser limits, corrupted/recursive blocks, table grid edge cases,
//! quote truncation, formula splitting, and zoom control clamping.

use gpui_telegram_desktop::views::iv::editor::iv_editor_page_table_grid::*;
use gpui_telegram_desktop::views::iv::iv_rich_page::*;
use gpui_telegram_desktop::views::iv::iv_rich_page_quote::*;
use gpui_telegram_desktop::views::iv::iv_rich_page_quote_builder::*;
use gpui_telegram_desktop::views::iv::iv_zoom_controls::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_parse::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_parse_validate::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_prepare_blocks::*;

#[test]
fn test_adversarial_iv_parse_corrupted_utf8_and_limits() {
    // 1. Corrupted non-UTF8 sequence
    let corrupted_bytes = vec![0xFF, 0xFE, 0xFD, 0x80, 0xBF];
    let res = parse_markdown_for_iv(&corrupted_bytes, None);
    assert!(res.is_err(), "Parser must reject corrupted UTF-8");
    assert!(res.unwrap_err().contains("Invalid UTF-8 source"));

    // 2. Empty byte slice
    let empty_res = parse_markdown_for_iv(b"", None);
    assert!(empty_res.is_ok(), "Empty input should parse to 0 nodes");
    let doc = empty_res.unwrap();
    assert_eq!(doc.node_count, 0);
    assert_eq!(doc.source_bytes, 0);

    // 3. Oversized source exceeding byte limits
    let custom_limits = MarkdownParseLimits {
        max_source_bytes: 64,
        max_tree_depth: 16,
        max_ast_nodes: 10,
    };
    let huge_content = "A".repeat(128);
    let size_err = parse_markdown_for_iv(huge_content.as_bytes(), Some(custom_limits.clone()));
    assert!(size_err.is_err());
    assert!(size_err.unwrap_err().contains("exceeds limit"));

    // 4. AST node count exhaustion
    let node_limits = MarkdownParseLimits {
        max_source_bytes: 10_000,
        max_tree_depth: 16,
        max_ast_nodes: 10,
    };
    let many_lines = (0..50).map(|i| format!("Line {}\n", i)).collect::<String>();
    let ast_err = parse_markdown_for_iv(many_lines.as_bytes(), Some(node_limits));
    assert!(ast_err.is_err());
    assert!(ast_err.unwrap_err().contains("AST node count limit exceeded"));
}

#[test]
fn test_adversarial_iv_parse_validate_boms_and_null_bytes() {
    let mut validator = IvMarkdownParseValidateMarkdown::new();

    // 1. Source empty
    let empty_res = validator.validate(b"");
    assert_eq!(empty_res.unwrap_err().reason, ValidationFailureReason::SourceEmpty);

    // 2. UTF-16 BE / LE BOM
    let utf16_be = [0xFE, 0xFF, b'H', b'i'];
    let utf16_le = [0xFF, 0xFE, b'H', b'i'];
    assert_eq!(
        validator.validate(&utf16_be).unwrap_err().reason,
        ValidationFailureReason::UnsupportedEncodingUtf16
    );
    assert_eq!(
        validator.validate(&utf16_le).unwrap_err().reason,
        ValidationFailureReason::UnsupportedEncodingUtf16
    );

    // 3. UTF-32 BE / LE BOM
    let utf32_be = [0x00, 0x00, 0xFE, 0xFF, 0x00, 0x00, 0x00, 0x41];
    let utf32_le = [0xFF, 0xFE, 0x00, 0x00, 0x41, 0x00, 0x00, 0x00];
    assert_eq!(
        validator.validate(&utf32_be).unwrap_err().reason,
        ValidationFailureReason::UnsupportedEncodingUtf32
    );
    assert_eq!(
        validator.validate(&utf32_le).unwrap_err().reason,
        ValidationFailureReason::UnsupportedEncodingUtf32
    );

    // 4. UTF-8 BOM must be stripped
    let utf8_bom = [0xEF, 0xBB, 0xBF, b'#', b' ', b'T', b'e', b'x', b't'];
    let valid_bom_res = validator.validate(&utf8_bom);
    assert!(valid_bom_res.is_ok());
    assert_eq!(valid_bom_res.unwrap(), b"# Text");

    // 5. Embedded null byte attack
    let with_null = b"Heading 1\nSome valid text\0hidden payload\nEnd";
    let null_res = validator.validate(with_null);
    assert!(null_res.is_err());
    let err = null_res.unwrap_err();
    assert_eq!(err.reason, ValidationFailureReason::ContainsNullBytes);
    assert_eq!(err.byte_offset, 25);
    assert!(err.message.contains("Forbidden null byte detected at offset 25"));
}

#[test]
fn test_adversarial_iv_table_grid_corrupted_and_zero_dimensions() {
    // 1. Completely empty table
    let empty_grid = build_table_grid(&[]);
    assert_eq!(empty_grid.row_count, 0);
    assert_eq!(empty_grid.column_count, 1);
    assert!(empty_grid.cells.is_empty());

    // 2. Table with 0-span cells (adversarial input)
    let zero_span_table = vec![
        vec![(0, 0), (0, 1)],
        vec![(1, 0), (0, 0)],
    ];
    let grid_zeros = build_table_grid(&zero_span_table);
    assert_eq!(grid_zeros.row_count, 2);
    // Spans are clamped to at least 1x1
    for cell in &grid_zeros.cells {
        assert!(cell.row_till > cell.row_from);
        assert!(cell.column_till > cell.column_from);
    }

    // 3. Huge span cells exceeding grid boundaries
    let huge_span_table = vec![
        vec![(500, 500)],
        vec![(1, 1)],
    ];
    let grid_huge = build_table_grid(&huge_span_table);
    assert_eq!(grid_huge.row_count, 2);
    // Rowspan clamped to row_count - r_idx
    let first = &grid_huge.cells[0];
    assert_eq!(first.row_till - first.row_from, 2);

    // 4. Asymmetrical / ragged rows
    let ragged_table = vec![
        vec![(1, 1), (1, 1), (1, 1), (1, 1)],
        vec![],
        vec![(1, 1)],
    ];
    let grid_ragged = build_table_grid(&ragged_table);
    assert_eq!(grid_ragged.row_count, 3);
    assert_eq!(grid_ragged.column_count, 4);

    // 5. Intersect queries with inverted or out-of-grid bounds
    let mut editor = IvEditorPageTableGridEditor::new();
    editor.grid = grid_ragged;
    // Inverted range r_from > r_till
    editor.select_range(5, 1, 10, 0);
    assert!(editor.selected_cells.is_empty(), "Inverted range must return 0 selected cells");

    // Negative range
    editor.select_range(-10, -5, -20, -1);
    assert!(editor.selected_cells.is_empty(), "Negative range must return 0 cells");
}


#[test]
fn test_adversarial_iv_quote_builder_utf8_multibyte_and_formulas() {
    let mut builder = IvRichPageQuoteBuilder::new();

    // 1. Truncation with multi-byte UTF-8 emojis at the cutoff boundary
    let emojis = "🔥🚀💎🎉✨🌟⚡️🎯🎲🏆🏅🥇🎖️".repeat(30);
    let quote_res = builder.build_quote(&emojis, 100);
    assert!(quote_res.overflown);
    assert!(quote_res.fragments[0].text.ends_with("..."));
    // Verify valid UTF-8 and no panic on boundary slicing
    assert!(quote_res.fragments[0].text.chars().count() > 0);

    // 2. Extremely short limit (budget underflow protection: limit.saturating_sub(3))
    let tiny_quote = builder.build_quote("Hello World", 2);
    assert!(tiny_quote.overflown);
    assert_eq!(tiny_quote.fragments[0].text, "...");

    // 3. Exact limit matching
    let exact = "1234567890";
    let exact_quote = builder.build_quote(exact, 10);
    assert!(!exact_quote.overflown);
    assert_eq!(exact_quote.fragments[0].text, "1234567890");

    // 4. Formula splitting check
    let quote = IvRichPageQuote::new(1, "Sample text $E = mc^2$ and more.");
    assert!(!quote.splits_formula("This has $x + y = z$ intact", 5));
    // Index inside formula should detect formula split
    assert!(quote.splits_formula("Prefix $formula_body$ suffix", 12));
    // Index after formula
    assert!(!quote.splits_formula("Prefix $formula_body$ suffix", 24));

    // 5. Roman numeral bounds (0, >3999, negative logic)
    assert_eq!(to_roman_numeral(0, false), "0");
    assert_eq!(to_roman_numeral(4000, true), "4000");
    assert_eq!(to_roman_numeral(1, true), "I");
    assert_eq!(to_roman_numeral(3999, true), "MMMCMXCIX");
    assert_eq!(to_roman_numeral(4, false), "iv");
}

#[test]
fn test_adversarial_iv_zoom_controls_boundary_clamping() {
    let mut zoom = IvZoomControls::new();
    assert_eq!(zoom.zoom_percentage(), 100);

    // 1. Astronomical zoom level
    zoom.set_zoom(100_000.0);
    assert_eq!(zoom.zoom_percentage(), 300, "Maximum zoom must be clamped to 300%");
    assert!(!zoom.can_zoom_in());
    assert!(zoom.can_zoom_out());

    // 2. Negative / zero zoom level
    zoom.set_zoom(-500.0);
    assert_eq!(zoom.zoom_percentage(), 50, "Minimum zoom must be clamped to 50%");
    assert!(zoom.can_zoom_out() == false);
    assert!(zoom.can_zoom_in());

    // 3. Repeated tiny zoom steps at boundaries
    for _ in 0..150 {
        zoom.zoom_in(ZoomStepKind::Tiny);
    }
    assert_eq!(zoom.zoom_percentage(), 300);

    for _ in 0..150 {
        zoom.zoom_out(ZoomStepKind::Tiny);
    }
    assert_eq!(zoom.zoom_percentage(), 50);

    // 4. Reset zoom recovery
    zoom.reset_zoom();
    assert_eq!(zoom.zoom_percentage(), 100);
}
