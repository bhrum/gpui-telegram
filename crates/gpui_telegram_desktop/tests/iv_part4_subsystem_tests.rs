//! Subsystem Integration & Unit Tests for Module 13 Part 4: Instant View Markdown & Presentation
//! Covers 19 Targets across iv.json [56..75] and complementary media/navigation/rendering/sharing modules:
//! - IvMarkdownMediaReuseMarkdown
//! - IvMarkdownMicrotexMarkdown
//! - IvMarkdownParseMarkdown
//! - IvMarkdownParseConvertMarkdown
//! - IvMarkdownParseFinalizeMarkdown
//! - IvMarkdownParseValidateMarkdown
//! - IvMarkdownPrepareMarkdown
//! - IvMarkdownPrepareBlocksMarkdown
//! - IvMarkdownPrepareFormulasMarkdown
//! - IvMarkdownPrepareInlineMarkdown
//! - IvMarkdownPrepareLinksMarkdown
//! - IvMarkdownPrepareNativeBlocksMarkdown
//! - IvMarkdownPrepareNativeRichtextMarkdown
//! - IvMarkdownPrepareSerializeMarkdown
//! - IvMarkdownPrepareStateMarkdown
//! - IvMarkdownSlideshowChromeMarkdown
//! - IvMarkdownThemeMarkdown
//! - IvMarkdownViewMarkdown
//! - IvMarkdownViewWidgetMarkdown

use gpui_telegram_desktop::views::iv::markdown::iv_markdown_media_reuse::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_microtex::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_parse::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_parse_convert::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_parse_finalize::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_parse_validate::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_prepare::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_prepare_blocks::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_prepare_formulas::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_prepare_inline::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_prepare_links::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_prepare_native_blocks::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_prepare_native_richtext::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_prepare_serialize::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_prepare_state::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_slideshow_chrome::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_theme::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_view::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_view_widget::*;
use gpui_telegram_desktop::views::iv::media::*;
use gpui_telegram_desktop::views::iv::navigation::*;
use gpui_telegram_desktop::views::iv::rendering::*;
use gpui_telegram_desktop::views::iv::sharing::*;

#[test]
fn test_iv_media_reuse_pool_and_storage() {
    let mut reuse = IvMarkdownMediaReuseMarkdown::new();
    let mut key1 = MediaReuseKey::default();
    key1.backing_id = 1001;
    key1.width = 800;
    key1.height = 600;

    reuse.storage.blocks.insert(
        1001,
        CachedMediaBlockEntry {
            block_id: 1001,
            key: key1.clone(),
            texture_loaded: true,
            view_count: 1,
        },
    );

    reuse.collect_old_media_blocks_for_reuse();

    let new_keys = vec![key1.clone()];
    let reused = reuse.reuse_media_blocks(&new_keys);
    assert_eq!(reused.len(), 1);
    assert_eq!(reused.get(&1001), Some(&key1));
    assert_eq!(reuse.collect_reused_media_blocks(), 1);

    let mut seed = 0usize;
    media_reuse_hash_combine(&mut seed, 42);
    assert_ne!(seed, 0);
}

#[test]
fn test_iv_microtex_math_measurement_and_rendering() {
    let mut microtex = IvMarkdownMicrotexMarkdown::new();
    assert!(microtex.is_initialized);

    let measured = microtex.measure("\\frac{a}{b}", MathKind::Display, 16);
    assert!(measured.width >= 12);
    assert!(measured.height >= 14);
    assert_eq!(measured.ascent + measured.descent, measured.height);

    let render_res = microtex.render("x^2 + y^2 = z^2", MathKind::Inline, 14, 2);
    assert!(render_res.is_ok());
    let result = render_res.unwrap();
    assert!(result.image_data.len() > 0);
    assert_eq!(result.width, (result.measured.width as u32) * 2);
}

#[test]
fn test_iv_markdown_parse_and_limits() {
    let mut parser = IvMarkdownParseMarkdown::new();
    let sample = b"# Main Title\n\nThis is paragraph one.\n\n$$E = mc^2$$\n\n## Subtitle\nParagraph two.";
    let doc = parser.parse(sample).expect("Parse should succeed");

    assert_eq!(doc.heading_count, 2);
    assert_eq!(doc.paragraph_count, 2);
    assert_eq!(doc.formula_count, 1);
    assert_eq!(doc.node_count, 5);

    let mut tiny_limits = MarkdownParseLimits::default();
    tiny_limits.max_source_bytes = 10;
    let fail = parse_markdown_for_iv(sample, Some(tiny_limits));
    assert!(fail.is_err());
}

#[test]
fn test_iv_markdown_parse_convert_and_cmark_budget() {
    let mut converter = IvMarkdownParseConvertMarkdown::new();
    let sample = "Line 1\n<details>\n<summary>Click to view</summary>\nHidden content\n</details>\nLine 2";
    let res = converter.run_budgeted_cmark_parse(sample).expect("Convert should succeed");

    assert_eq!(res.details_blocks.len(), 1);
    assert_eq!(res.details_blocks[0].summary, "Click to view");
    assert_eq!(res.details_blocks[0].content, vec!["Hidden content".to_string()]);
}

#[test]
fn test_iv_markdown_parse_finalize_anchors_and_slugs() {
    let headings = vec![
        "Introduction".to_string(),
        "Architecture Overview".to_string(),
        "Introduction".to_string(), // Duplicate heading
    ];
    let footnotes = vec!["Telegram API doc ref".to_string()];

    let semantics = finalize_document_semantics(&headings, &footnotes);
    assert_eq!(semantics.anchors.len(), 3);
    assert_eq!(semantics.anchors[0].slug, "introduction");
    assert_eq!(semantics.anchors[1].slug, "architecture-overview");
    assert_eq!(semantics.anchors[2].slug, "introduction-1"); // Deduplicated suffix!

    assert!(semantics.local_fragments.contains("#introduction"));
    assert!(semantics.local_fragments.contains("#introduction-1"));
    assert!(semantics.local_fragments.contains("#fn-1"));
}

#[test]
fn test_iv_markdown_parse_validate_boms_and_nulls() {
    let clean = b"Normal text";
    assert_eq!(validate_markdown_source_for_iv(clean).unwrap(), clean);

    // UTF-8 BOM stripped
    let with_utf8_bom = [0xEF, 0xBB, 0xBF, b'a', b'b', b'c'];
    assert_eq!(validate_markdown_source_for_iv(&with_utf8_bom).unwrap(), b"abc");

    // UTF-16 rejected
    let utf16_source = [0xFE, 0xFF, 0x00, 0x41];
    assert!(validate_markdown_source_for_iv(&utf16_source).is_err());

    // Null bytes rejected
    let with_null = [b'h', b'e', 0x00, b'l', b'l', b'o'];
    assert!(validate_markdown_source_for_iv(&with_null).is_err());
}

#[test]
fn test_iv_markdown_prepare_pipeline_and_limits() {
    assert_eq!(heading_level_label(1), "Heading 1");
    assert_eq!(heading_level_label(3), "Heading 3");

    let mut blocks = (0..10).map(|i| format!("Block {}", i)).collect::<Vec<_>>();
    let truncated = apply_soft_prepared_block_limit(&mut blocks, 5);
    assert!(truncated);
    assert_eq!(blocks.len(), 6);
    assert!(blocks.last().unwrap().contains("Article truncated"));
}

#[test]
fn test_iv_markdown_prepare_blocks_and_table_cells() {
    assert_eq!(prepare_table_cell_text("  Cell\ntext  "), "Cell text");

    let fallback = prepare_fallback_blocks(&["Line A".to_string(), "Line B".to_string()]);
    assert_eq!(fallback.len(), 2);
    assert_eq!(fallback[0].content, "Line A");
    assert_eq!(fallback[0].block_type, PreparedBlockType::Paragraph);
}
#[test]
fn test_iv_markdown_prepare_formulas_registry() {
    let mut formulas = IvMarkdownPrepareFormulasMarkdown::new();
    assert_eq!(formula_slot_count("Here is $x$ and another $y$ formula"), 2);

    formulas.measure_prepared_formulas(&["E=mc^2".to_string(), "\\pi r^2".to_string()]);
    assert_eq!(formulas.formula_measurements.len(), 2);
    assert!(formulas.formula_measurements.contains_key("E=mc^2"));
}

#[test]
fn test_iv_markdown_prepare_inline_formatting() {
    assert_eq!(finalize_prepared_external_link("example.com"), "https://example.com");
    assert_eq!(finalize_prepared_external_link("tg://resolve?domain=telegram"), "tg://resolve?domain=telegram");

    let text = "Start math end";
    let formulas = vec![InlineFormulaSource {
        tex: "x+1".to_string(),
        offset: 6,
    }];
    let spans = append_text_with_inline_formulas(text, &formulas);
    assert_eq!(spans.len(), 3);
    assert_eq!(spans[0].text, "Start ");
    assert_eq!(spans[1].text, "$x+1$");
    assert_eq!(spans[2].text, "math end");
}

#[test]
fn test_iv_markdown_prepare_links_indexing() {
    let mut resolver = IvMarkdownPrepareLinksMarkdown::new();
    let data = resolver.resolve_link("https://telegram.org/blog/preview", Some("Blog"));
    assert_eq!(data.display_text, "Blog");
    assert_eq!(data.clean_url, "https://telegram.org/blog/preview");
    assert_eq!(resolver.link_index.len(), 1);
}

#[test]
fn test_iv_markdown_prepare_native_blocks_and_roman_numerals() {
    assert_eq!(to_roman(1), "I");
    assert_eq!(to_roman(4), "IV");
    assert_eq!(to_roman(9), "IX");
    assert_eq!(to_roman(14), "XIV");
    assert_eq!(to_roman(42), "XLII");
    assert_eq!(to_roman(1984), "MCMLXXXIV");

    let fmt = NativeIvOrderedMarkerFormatter { is_roman: true };
    assert_eq!(fmt.format_marker(4), "IV.");
}

#[test]
fn test_iv_markdown_prepare_native_richtext_conversion() {
    let mut conv = IvMarkdownPrepareNativeRichtextMarkdown::new();
    let res = conv.convert_native_entities("Sample paragraph", Alignment::Center);
    assert_eq!(res.text, "Sample paragraph");
    assert_eq!(res.alignment, Alignment::Center);
    assert_eq!(K_DEFAULT_MAP_WIDTH, 400);
    assert_eq!(K_DEFAULT_MAP_HEIGHT, 200);
}

#[test]
fn test_iv_markdown_prepare_serialize_offsets() {
    let mut serializer = IvMarkdownPrepareSerializeMarkdown::new();
    let text = "Click [btn] to proceed";
    let buttons = vec![InlineLinkButtonSpan {
        text: "Submit".to_string(),
        url: "https://example.com".to_string(),
        button_type: Type::Url,
        button_color: Color::Primary,
    }];

    let expanded = serializer.expand_buttons(text, &buttons);
    assert!(expanded.contains("[Button: Submit]"));
    assert!(serializer.last_delta > 0);
}

#[test]
fn test_iv_markdown_prepare_state_warnings_and_failure() {
    let mut state = PrepareState::new();
    state.remember_formula("x = 1");
    state.remember_formula("x = 1"); // Deduplicated
    assert_eq!(state.formulas.len(), 1);

    state.add_prepare_warning("Non-standard tag".to_string());
    assert_eq!(state.warnings.len(), 1);

    state.set_terminal_failure("Out of memory".to_string());
    assert!(state.terminal_failure.is_some());
}

#[test]
fn test_iv_markdown_slideshow_chrome_nav_and_dots() {
    let nav = compute_slideshow_nav_rects(800.0, 600.0, 48.0);
    assert_eq!(nav.prev_x, 16.0);
    assert_eq!(nav.next_x, 800.0 - 48.0 - 16.0);
    assert_eq!(nav.button_size, 48.0);

    let dots = compute_slideshow_dots(4, 5.0, 8.0);
    assert_eq!(dots.dot_radius, 5.0);
    // 4 * 10 + 3 * 8 = 40 + 24 = 64
    assert_eq!(dots.total_width, 64.0);
}

#[test]
fn test_iv_markdown_theme_syntax_highlighting() {
    let theme = create_standalone_chat_theme();
    assert_ne!(theme.syntax.keyword, 0);
    assert_ne!(theme.syntax.string, 0);
    assert_ne!(theme.syntax.comment, 0);
}

#[test]
fn test_iv_markdown_view_zoom_and_search() {
    let mut view = IvMarkdownViewMarkdown::new();
    assert_eq!(view.root.zoom_percent, 100);

    view.zoom_in();
    assert_eq!(view.root.zoom_percent, 110);

    view.zoom_out();
    assert_eq!(view.root.zoom_percent, 100);

    view.set_search_matches(
        "rust".to_string(),
        vec![MarkdownArticleSearchMatch {
            paragraph: 1,
            offset: 5,
            length: 4,
        }],
    );
    assert_eq!(view.search.matches.len(), 1);
}

#[test]
fn test_iv_markdown_view_widget_lifecycle_and_wheel() {
    let mut widget = IvMarkdownViewWidgetMarkdown::new();
    let h = widget.resize_get_height(600.0);
    assert_eq!(h, 900.0);

    widget.set_placeholder_loading(true);
    assert!(widget.widget.is_placeholder_loading);

    widget.clear_placeholder_loading();
    assert!(!widget.widget.is_placeholder_loading);
}

#[test]
fn test_iv_media_navigation_rendering_sharing_modules() {
    // Media
    let mut overlay = IvMediaOverlay::new();
    overlay.show("https://example.com/pic.png", "A photo", false);
    assert!(overlay.is_open);
    overlay.zoom_in();
    assert_eq!(overlay.zoom_level, 1.25);
    overlay.close();
    assert!(!overlay.is_open);

    let mut video = IvVideoPlayer::new();
    video.toggle_play();
    assert!(video.is_playing);

    let mut voice = IvVoiceNotePlayer::new();
    voice.toggle_speed();
    assert!(voice.speed_2x);

    // Navigation
    let mut controller = IvNavigationController::new();
    controller.open_article("https://t.me/iv?url=example.com", "Example Article");
    assert!(controller.is_open);
    assert_eq!(controller.active_url, "https://t.me/iv?url=example.com");

    let mut nav_bar = IvNavigationBar::new("Article Title");
    nav_bar.set_navigation_state(true, false);
    assert!(nav_bar.can_back);

    // Rendering
    let mut canvas = IvCanvasRenderer::new();
    canvas.set_viewport(1024.0, 768.0, 2.0);
    assert_eq!(canvas.scale_factor, 2.0);

    let mut font_mgr = IvFontManager::new();
    font_mgr.set_family(IvFontFamily::Serif);
    font_mgr.set_base_size(18.0);
    assert_eq!(font_mgr.base_size, 18.0);

    let mut cache = IvLayoutCache::new();
    cache.insert_block_geometry(0, BlockGeometry { x: 0.0, y: 0.0, width: 400.0, height: 150.0 });
    assert!(cache.is_valid);
    assert_eq!(cache.total_document_height, 150.0);

    let paint_ctx = IvPaintContext::new();
    assert!(paint_ctx.is_visible(50.0, 200.0));

    let mut pipeline = IvRenderPipeline::new();
    pipeline.execute_layout();

    let mut text_layout = IvTextLayout::new();
    text_layout.compute_layout("A sample line of text for Instant View", 400.0);
    assert!(text_layout.lines.len() >= 1);

    // Sharing
    let mut share = IvShareDialog::new();
    share.open_share("https://telegra.ph/article", "My Telegraph Article");
    assert!(share.is_open);
    share.copy_link();
    assert!(share.link_copied);

    assert!(IvUrlHandler::is_instant_view_supported("https://telegra.ph/test-post"));
    assert!(IvUrlHandler::is_instant_view_supported("https://t.me/iv?url=foo&rhash=123"));
    assert_eq!(IvUrlHandler::extract_rhash("https://t.me/iv?url=foo&rhash=abc1234&extra=1"), Some("abc1234".to_string()));
}
