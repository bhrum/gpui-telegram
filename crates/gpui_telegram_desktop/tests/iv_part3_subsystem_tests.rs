//! Subsystem Integration & Unit Tests for Module 13 Part 3: Instant View Architecture
//! Covers 19 Targets:
//! 1. IvZoomControls
//! 2. IvMarkdownArticleMarkdown
//! 3. IvMarkdownArticleCaptionHitMarkdown
//! 4. IvMarkdownArticleLayoutBlocksMarkdown
//! 5. IvMarkdownArticleLayoutStructureMarkdown
//! 6. IvMarkdownArticlePaintMarkdown
//! 7. IvMarkdownArticleQuoteMarkdown
//! 8. IvMarkdownArticleScrollForwarderMarkdown
//! 9. IvMarkdownArticleSelectionMarkdown
//! 10. IvMarkdownArticleTextMarkdown
//! 11. IvMarkdownButtonRowMarkdown
//! 12. IvMarkdownCommonMarkdown
//! 13. IvMarkdownControllerMarkdown
//! 14. IvMarkdownDocumentMarkdown
//! 15. IvMarkdownEmbedOverlayMarkdown
//! 16. IvMarkdownHistoryViewMediaMarkdown
//! 17. IvMarkdownMathMarkdown
//! 18. IvMarkdownMathRendererMarkdown
//! 19. IvMarkdownMediaBlockMarkdown

use gpui_telegram_desktop::views::iv::iv_zoom_controls::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_article::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_article_caption_hit::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_article_layout_blocks::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_article_layout_structure::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_article_paint::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_article_quote::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_article_scroll_forwarder::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_article_selection::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_article_text::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_button_row::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_common::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_controller::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_document::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_embed_overlay::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_history_view_media::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_math::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_math_renderer::*;
use gpui_telegram_desktop::views::iv::markdown::iv_markdown_media_block::*;

#[test]
fn test_iv_zoom_controls() {
    let mut controls = IvZoomControls::new();
    assert_eq!(controls.zoom_percentage(), 100);
    assert!(controls.can_zoom_in());
    assert!(controls.can_zoom_out());

    // Zoom in with normal step (10%)
    controls.zoom_in(ZoomStepKind::Normal);
    assert_eq!(controls.zoom_percentage(), 110);

    // Zoom in with small step (5%)
    controls.zoom_in(ZoomStepKind::Small);
    assert_eq!(controls.zoom_percentage(), 115);

    // Zoom in with tiny step (2%)
    controls.zoom_in(ZoomStepKind::Tiny);
    assert_eq!(controls.zoom_percentage(), 117);

    // Reset zoom
    controls.reset_zoom();
    assert_eq!(controls.zoom_percentage(), 100);

    // Clamp at maximum zoom (300%)
    controls.set_zoom(5.0);
    assert_eq!(controls.zoom_percentage(), 300);
    assert!(!controls.can_zoom_in());

    // Clamp at minimum zoom (50%)
    controls.set_zoom(0.1);
    assert_eq!(controls.zoom_percentage(), 50);
    assert!(!controls.can_zoom_out());
}

#[test]
fn test_iv_markdown_article() {
    let mut article = IvMarkdownArticleMarkdown::with_article(1001, "Telegram Blog", "Instant View 2.0");
    article.set_metadata("Telegram Team", "April 17, 2026");

    assert_eq!(article.page_id, 1001);
    assert_eq!(article.site_name, "Telegram Blog");
    assert_eq!(article.article_title, "Instant View 2.0");

    // Highlight triggers and decay
    article.trigger_block_highlight(42, 1000);
    assert_eq!(article.highlights.len(), 1);
    assert_eq!(article.highlights[0].alpha, 1.0);

    // After 300ms (half of 600ms), alpha should be ~0.5
    article.update_highlights(1300);
    assert_eq!(article.highlights.len(), 1);
    assert!((article.highlights[0].alpha - 0.5).abs() < 0.05);

    // After 700ms, should be faded and removed
    article.update_highlights(1750);
    assert_eq!(article.highlights.len(), 0);

    // Selection
    let start = ArticleSelectionPosition { block_index: 0, char_offset: 5 };
    let end = ArticleSelectionPosition { block_index: 2, char_offset: 20 };
    article.set_selection(start, end);
    assert!(article.selection_range.is_some());
    article.clear_selection();
    assert!(article.selection_range.is_none());

    // Related articles
    article.add_related_article(RelatedArticleItem {
        url: "https://telegram.org/blog/iv".into(),
        title: "Previous IV Announcement".into(),
        site_name: "Telegram Blog".into(),
        preview_url: None,
    });
    assert_eq!(article.related_articles.len(), 1);
}

#[test]
fn test_iv_markdown_article_caption_hit() {
    let mut caption = IvMarkdownArticleCaptionHitMarkdown::new(CaptionTargetKind::Photo);
    caption.set_caption("Photo caption with a link");

    caption.add_hit_box(10.0, 20.0, 100.0, 15.0, CaptionHitResult::Link("https://telegram.org".into()));
    caption.add_hit_box(120.0, 20.0, 60.0, 15.0, CaptionHitResult::Hashtag("#news".into()));

    // Direct hit on link
    assert_eq!(
        caption.hit_test(50.0, 25.0),
        CaptionHitResult::Link("https://telegram.org".into())
    );

    // Hit within tolerance margin (+-4px)
    assert_eq!(
        caption.hit_test(8.0, 22.0),
        CaptionHitResult::Link("https://telegram.org".into())
    );

    // Hit on hashtag
    assert_eq!(
        caption.hit_test(140.0, 25.0),
        CaptionHitResult::Hashtag("#news".into())
    );

    // Miss
    assert_eq!(caption.hit_test(300.0, 300.0), CaptionHitResult::None);
}

#[test]
fn test_iv_markdown_article_layout_blocks() {
    let mut layout = IvMarkdownArticleLayoutBlocksMarkdown::with_content_width(720.0);
    assert_eq!(layout.content_width, 720.0);

    layout.add_block_metric(1, 720.0, 40.0, 10.0, 10.0); // 60.0
    layout.add_block_metric(2, 720.0, 120.0, 5.0, 15.0); // 140.0
    assert_eq!(layout.block_metrics.len(), 2);
    assert_eq!(layout.total_layout_height, 200.0);

    layout.clear_metrics();
    assert_eq!(layout.block_metrics.len(), 0);
    assert_eq!(layout.total_layout_height, 0.0);
}

#[test]
fn test_iv_markdown_article_layout_structure() {
    let mut structure = IvMarkdownArticleLayoutStructureMarkdown::new(800.0);

    structure.add_analysis_node(1, 100.0, 800.0, 0);
    structure.add_analysis_node(2, 80.0, 760.0, 1);
    assert_eq!(structure.analysis_nodes.len(), 2);

    // Table column distribution
    let weights = [1.0, 2.0, 1.0];
    let dist = structure.distribute_table_columns(&weights, 400.0);
    assert_eq!(dist.column_widths.len(), 3);
    assert_eq!(dist.column_widths[0], 100.0);
    assert_eq!(dist.column_widths[1], 200.0);
    assert_eq!(dist.column_widths[2], 100.0);

    // Quote collapsing
    assert!(!structure.should_collapse_quote(2));
    assert!(!structure.should_collapse_quote(3));
    assert!(structure.should_collapse_quote(4));
}

#[test]
fn test_iv_markdown_article_paint() {
    let mut painter = IvMarkdownArticlePaintMarkdown::new(true);
    assert!(painter.is_dark_theme);

    // Thinking gradient advance
    painter.update_gradient(600.0);
    assert!((painter.gradient_state.phase() - 0.5).abs() < 0.05);

    painter.update_gradient(700.0);
    assert_eq!(painter.gradient_state.phase(), 1.0);

    // Table ownership
    painter.set_table_ownership(10, 2, 3);
    assert!(painter.ownership_slot.is_some());
    painter.clear_table_ownership();
    assert!(painter.ownership_slot.is_none());
}

#[test]
fn test_iv_markdown_article_quote() {
    let mut quotes = IvMarkdownArticleQuoteMarkdown::new();
    quotes.add_interval(101, 0, 150);
    quotes.add_interval(102, 200, 350);

    assert_eq!(quotes.find_quote_at(50), Some(101));
    assert_eq!(quotes.find_quote_at(250), Some(102));
    assert_eq!(quotes.find_quote_at(180), None);

    quotes.set_highlight(101);
    assert_eq!(quotes.highlighted_id, Some(101));
    quotes.clear_highlight();
    assert_eq!(quotes.highlighted_id, None);
}

#[test]
fn test_iv_markdown_article_scroll_forwarder() {
    let mut forwarder = IvMarkdownArticleScrollForwarderMarkdown::with_damping(0.8);

    // Small scroll below threshold (2.0)
    let forwarded = forwarder.forward_wheel(0.0, 1.0, true);
    assert!(forwarded.is_none());

    // Second scroll pushes past threshold
    let forwarded2 = forwarder.forward_wheel(0.0, 2.0, true);
    assert!(forwarded2.is_some());
    assert_eq!(forwarder.forwarded_events_count, 1);
}

#[test]
fn test_iv_markdown_article_selection() {
    let mut sel = IvMarkdownArticleSelectionMarkdown::new();
    sel.start_selection();

    sel.add_segment(1, SelectableSegmentKind::Text, 0, 11, "Hello World");
    sel.add_segment(2, SelectableSegmentKind::Math, 0, 7, "E = mc^2");
    sel.add_segment(3, SelectableSegmentKind::Media, 0, 0, "");

    let text = sel.format_clipboard_text();
    assert!(text.contains("Hello World"));
    assert!(text.contains("$E = mc^2$"));
    assert!(text.contains(PHOTO_COPY_LABEL));

    sel.clear();
    assert_eq!(sel.segments.len(), 0);
}

#[test]
fn test_iv_markdown_article_text() {
    let mut text_elem = IvMarkdownArticleTextMarkdown::new("This is a bold link text.");
    assert_eq!(text_elem.raw_text, "This is a bold link text.");

    text_elem.add_entity(10, 4, TextEntityKind::Bold);
    text_elem.add_entity(15, 4, TextEntityKind::Link("https://telegram.org".into()));
    assert_eq!(text_elem.entities.len(), 2);

    text_elem.set_rtl(true);
    assert!(text_elem.is_rtl);

    text_elem.clear_entities();
    assert_eq!(text_elem.entities.len(), 0);
}

#[test]
fn test_iv_markdown_button_row() {
    let mut row = IvMarkdownButtonRowMarkdown::new(0);
    row.add_url_button(1, "Open Channel", "https://t.me/telegram");
    row.add_callback_button(2, "Confirm", vec![1, 2, 3]);
    assert_eq!(row.buttons.len(), 2);

    row.set_loading(1, true);
    assert!(row.buttons[0].is_loading);
    assert!(!row.buttons[1].is_loading);

    row.advance_glare(700.0);
    assert!((row.glare_phase() - 0.5).abs() < 0.05);
}

#[test]
fn test_iv_markdown_common_media_contracts() {
    let common = IvMarkdownCommonMarkdown::new();

    // Clamp dimensions
    let dims = common.clamp_dimensions(2400.0, 1200.0);
    assert_eq!(dims.width, 1200.0);
    assert_eq!(dims.height, 600.0);

    // Extreme aspect ratio clamp
    let narrow = common.clamp_dimensions(100.0, 1000.0); // 0.1 ratio -> clamped to 0.25
    assert_eq!(narrow.width / narrow.height, 0.25);

    let wide = common.clamp_dimensions(1000.0, 100.0); // 10.0 ratio -> clamped to 4.0
    assert_eq!(wide.width / wide.height, 4.0);
}

#[test]
fn test_iv_markdown_controller_navigation() {
    let mut controller = IvMarkdownControllerMarkdown::new();
    assert!(!controller.can_go_back());
    assert!(!controller.can_go_forward());

    controller.navigate_to("https://telegram.org/page1", 101);
    controller.navigate_to("https://telegram.org/page2", 102);
    assert_eq!(controller.current_url, "https://telegram.org/page2");
    assert!(controller.can_go_back());
    assert!(!controller.can_go_forward());

    controller.go_back();
    assert_eq!(controller.current_url, "https://telegram.org/page1");
    assert_eq!(controller.current_page_id, 101);
    assert!(controller.can_go_forward());

    controller.go_forward();
    assert_eq!(controller.current_url, "https://telegram.org/page2");
    assert_eq!(controller.current_page_id, 102);

    // Zoom state
    controller.set_zoom(1.25);
    assert_eq!(controller.zoom_state.current_zoom, 1.25);
}

#[test]
fn test_iv_markdown_document_ast() {
    let mut doc = IvMarkdownDocumentMarkdown::new();
    let h1_id = doc.add_node(DocumentNodeKind::Heading(1), "Instant View Architecture");
    let p_id = doc.add_node(DocumentNodeKind::Paragraph, "Telegram Instant View delivers zero-delay articles.");
    let list_id = doc.add_node(DocumentNodeKind::List(ListType::Bullet), "");
    let item_id = doc.add_node(DocumentNodeKind::Paragraph, "First bullet item");

    doc.add_child(list_id, item_id);
    assert_eq!(doc.node_count(), 4);

    let list_node = doc.find_node(list_id).unwrap();
    assert_eq!(list_node.child_ids, vec![item_id]);
    assert_eq!(doc.root_ids, vec![h1_id, p_id, list_id]);
}

#[test]
fn test_iv_markdown_embed_overlay() {
    let mut embed = IvMarkdownEmbedOverlayMarkdown::new("https://www.youtube.com/embed/dQw4w9WgXcQ");
    assert_eq!(embed.state, EmbedState::Loading);
    assert!(!embed.is_ready());

    embed.set_html("<iframe src=\"https://www.youtube.com/embed/dQw4w9WgXcQ\"></iframe>");
    embed.mark_ready();
    assert!(embed.is_ready());
    assert_eq!(embed.state, EmbedState::Ready);

    embed.mark_failed("Network timeout");
    assert!(!embed.is_ready());
}

#[test]
fn test_iv_markdown_history_view_media() {
    let mut gallery = IvMarkdownHistoryViewMediaMarkdown::new();
    gallery.add_media_item(1, "https://telegram.org/photo1.jpg", "Photo 1", 1.5);
    gallery.add_media_item(2, "https://telegram.org/photo2.jpg", "Photo 2", 1.33);
    gallery.add_media_item(3, "https://telegram.org/photo3.jpg", "Photo 3", 1.77);

    assert_eq!(gallery.items.len(), 3);
    assert_eq!(gallery.current_index, 0);

    assert!(gallery.next_slide());
    assert_eq!(gallery.current_index, 1);
    assert_eq!(gallery.current_item().unwrap().caption, "Photo 2");

    assert!(gallery.next_slide());
    assert_eq!(gallery.current_index, 2);
    assert!(!gallery.next_slide()); // at end

    assert!(gallery.previous_slide());
    assert_eq!(gallery.current_index, 1);

    // Drag commit with high velocity
    gallery.commit_drag(400.0, -600.0);
    assert_eq!(gallery.current_index, 2);
}

#[test]
fn test_iv_markdown_math_scanner() {
    let mut scanner = IvMarkdownMathMarkdown::new();
    let text = "Inline formula is $a^2 + b^2 = c^2$ and block is $$\\sum_{i=1}^n i = \\frac{n(n+1)}{2}$$.";
    scanner.scan_source(text);

    assert_eq!(scanner.tokens.len(), 2);
    assert_eq!(scanner.tokens[0].delimiter, MathDelimiterKind::InlineSingleDollar);
    assert_eq!(scanner.tokens[0].expression, "a^2 + b^2 = c^2");
    assert_eq!(scanner.tokens[1].delimiter, MathDelimiterKind::DisplayDoubleDollar);
    assert_eq!(scanner.tokens[1].expression, "\\sum_{i=1}^n i = \\frac{n(n+1)}{2}");
    assert_eq!(scanner.syntax_errors_count, 0);

    // Syntax validation
    assert!(IvMarkdownMathMarkdown::is_valid_expression("{a + b}"));
    assert!(!IvMarkdownMathMarkdown::is_valid_expression("{a + b"));
}

#[test]
fn test_iv_markdown_math_renderer_cache() {
    let mut renderer = IvMarkdownMathRendererMarkdown::new();

    let key1 = FormulaCacheKey {
        expression: "E = mc^2".into(),
        font_size_pt: 14,
        is_display_block: false,
        color_rgb: 0xffffff,
    };

    // First query: cache miss
    let m1 = renderer.query_or_render(key1.clone());
    assert_eq!(renderer.cache_misses, 1);
    assert_eq!(renderer.cache_hits, 0);
    assert!(m1.width > 0.0);

    // Second query: cache hit
    let m2 = renderer.query_or_render(key1.clone());
    assert_eq!(renderer.cache_hits, 1);
    assert_eq!(renderer.cache_misses, 1);
    assert_eq!(m1, m2);
    assert_eq!(renderer.hit_rate(), 0.5);

    renderer.clear_cache();
    assert_eq!(renderer.entries.len(), 0);
    assert_eq!(renderer.cache_hits, 0);
}

#[test]
fn test_iv_markdown_media_block_presentation() {
    let channel_card = ChannelBlockData {
        channel_title: "Telegram News".into(),
        channel_username: "@telegram".into(),
        message_text: "Major update released!".into(),
        members_count: 5000000,
        avatar_url: None,
    };
    let block1 = IvMarkdownMediaBlockMarkdown::new_channel(1, channel_card);
    assert!(matches!(block1.presentation, MediaBlockPresentation::Channel(_)));

    let doc_card = DocumentBlockData {
        filename: "InstantViewSpec.pdf".into(),
        file_size_bytes: 1048576,
        mime_type: "application/pdf".into(),
        url: "https://telegram.org/doc.pdf".into(),
    };
    let block2 = IvMarkdownMediaBlockMarkdown::new_document(2, doc_card);
    assert!(matches!(block2.presentation, MediaBlockPresentation::Document(_)));

    let collage_items = vec![
        GroupedMediaItem { media_id: 10, thumbnail_url: "url1".into(), width: 200.0, height: 200.0 },
        GroupedMediaItem { media_id: 11, thumbnail_url: "url2".into(), width: 200.0, height: 200.0 },
    ];
    let block3 = IvMarkdownMediaBlockMarkdown::new_collage(3, collage_items);
    assert!(matches!(block3.presentation, MediaBlockPresentation::Collage(_)));
}
