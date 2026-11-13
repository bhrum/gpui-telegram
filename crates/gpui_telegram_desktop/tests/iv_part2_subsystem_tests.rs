//! Subsystem Integration & Unit Tests for Module 13 Part 2: Instant View Architecture
//! Covers 19 Targets: IvEditorStructureMenuEditor, IvEditorTextEntitiesEditor,
//! IvEditorToolbarPillEditor, IvEditorWidgetEditor, IvEditorWindowEditor,
//! IvCachedMedia, IvController, IvData, IvDelegate, IvDelegateImpl,
//! IvInstance, IvPch, IvRichMessageHtmlExport, IvRichMessageSerializer,
//! IvRichPage, IvRichPageQuote, IvRichPageQuoteBuilder, IvSearchBar, IvSearchController.

use gpui_telegram_desktop::views::iv::editor::iv_editor_structure_menu::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_text_entities::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_toolbar_pill::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_widget::*;
use gpui_telegram_desktop::views::iv::editor::iv_editor_window::*;
use gpui_telegram_desktop::views::iv::iv_cached_media::*;
use gpui_telegram_desktop::views::iv::iv_controller::*;
use gpui_telegram_desktop::views::iv::iv_data::*;
use gpui_telegram_desktop::views::iv::iv_delegate::*;
use gpui_telegram_desktop::views::iv::iv_delegate_impl::*;
use gpui_telegram_desktop::views::iv::iv_instance::*;
use gpui_telegram_desktop::views::iv::iv_pch::*;
use gpui_telegram_desktop::views::iv::iv_rich_message_html_export::*;
use gpui_telegram_desktop::views::iv::iv_rich_message_serializer::*;
use gpui_telegram_desktop::views::iv::iv_rich_page::*;
use gpui_telegram_desktop::views::iv::iv_rich_page_quote::*;
use gpui_telegram_desktop::views::iv::iv_rich_page_quote_builder::*;
use gpui_telegram_desktop::views::iv::iv_search_bar::*;
use gpui_telegram_desktop::views::iv::iv_search_controller::*;

#[test]
fn test_iv_editor_structure_menu() {
    let mut menu = IvEditorStructureMenuEditor::new();
    assert!(!menu.is_visible);

    menu.fill_list_change_menu(ListFormatType::Bullet);
    assert!(menu.is_visible);
    assert_eq!(menu.items.len(), 4);
    assert!(menu.items[0].is_checked); // Bullet checked

    let action = menu.select_item(1); // Select Numbered
    assert_eq!(action, Some(StructureMenuAction::ChangeListType(ListFormatType::Numbers)));
    assert!(!menu.is_visible);

    menu.fill_table_change_menu(TableCellAlignment::Left);
    assert!(menu.is_visible);
    assert_eq!(menu.items.len(), 7);
}

#[test]
fn test_iv_editor_text_entities_formulas_and_ranges() {
    assert!(IvEditorTextEntitiesEditor::range_inside_text("Hello World", 0, 5));
    assert!(!IvEditorTextEntitiesEditor::range_inside_text("Hello", 2, 5));

    let input = "Energy is $E = mc^2$ and block is $$\\int x dx$$.";
    let (processed, formulas) = IvEditorTextEntitiesEditor::parse_formulas(input);
    assert_eq!(formulas.len(), 2);
    assert!(!formulas[0].is_block);
    assert_eq!(formulas[0].source, "E = mc^2");
    assert!(formulas[1].is_block);
    assert_eq!(formulas[1].source, "\\int x dx");
    assert!(processed.contains("[formula]"));

    let reps = vec![RichTextEditorOffsetReplacement {
        rich_offset: 10,
        rich_length: 5,
        editor_length: 9,
    }];
    assert_eq!(IvEditorTextEntitiesEditor::map_rich_offset_to_editor_offset(&reps, 5), 5);
    assert_eq!(IvEditorTextEntitiesEditor::map_rich_offset_to_editor_offset(&reps, 20), 24);
}

#[test]
fn test_iv_editor_toolbar_pill() {
    let mut pill = IvEditorToolbarPillEditor::new();
    assert!(pill.is_visible);
    assert_eq!(pill.buttons.len(), 7);

    let (w, h) = pill.natural_size();
    assert!(w > 0.0);
    assert_eq!(h, 40.0); // 32 + 4 * 2

    pill.toggle_button(ToolbarButtonId::Bold);
    assert_eq!(pill.buttons[0].state, ToolbarButtonState::Active);
    pill.toggle_button(ToolbarButtonId::Bold);
    assert_eq!(pill.buttons[0].state, ToolbarButtonState::Inactive);
}

#[test]
fn test_iv_editor_widget_blocks_and_undo() {
    let mut widget = IvEditorWidgetEditor::new();
    assert_eq!(widget.blocks.len(), 3);
    assert_eq!(widget.word_count(), 15);

    let block_id = widget.insert_block(EditorBlockKind::Paragraph, "Newly added paragraph text");
    assert_eq!(widget.blocks.len(), 4);

    widget.apply_heading(1);
    assert_eq!(widget.blocks.last().unwrap().kind, EditorBlockKind::Heading { level: 1 });

    assert!(widget.undo());
    assert_eq!(widget.blocks.last().unwrap().kind, EditorBlockKind::Paragraph);

    assert!(widget.undo());
    assert_eq!(widget.blocks.len(), 3);

    assert!(widget.redo());
    assert_eq!(widget.blocks.len(), 4);

    assert!(widget.delete_block(block_id));
    assert_eq!(widget.blocks.len(), 3);
}

#[test]
fn test_iv_editor_window_layers_and_ime() {
    let mut window = IvEditorWindowEditor::new();
    assert!(!window.is_modal_shown());

    window.show_modal("Edit Link", ModalLayerKind::LinkEditor {
        url: "https://t.me".to_string(),
        text: "Telegram".to_string(),
    });
    assert!(window.is_modal_shown());

    window.request_close();
    assert!(!window.is_modal_shown()); // Dismisses modal first
    assert!(!window.close_requested);

    window.request_close();
    assert!(window.close_requested); // Closes window when no modal

    window.on_ime_start();
    assert!(window.is_ime_active);
    window.on_ime_end();
    assert!(!window.is_ime_active);
}

#[test]
fn test_iv_cached_media() {
    let mut media = IvCachedMedia::new();
    media.register_photo(101, "https://example.com/img.jpg", 1920, 1080);
    media.register_map(102, 37.7749, -122.4194, 15);
    media.register_document(103, "https://example.com/doc.pdf", "report.pdf", 409600);

    assert_eq!(media.items.len(), 3);

    let photo = media.get_item(101).expect("photo should exist");
    assert_eq!(photo.state, MediaLoadingState::Idle);

    media.update_progress(101, 0.75);
    assert_eq!(
        media.get_item(101).unwrap().state,
        MediaLoadingState::Downloading { progress: 0.75 }
    );

    media.mark_ready(101, "/tmp/cache/img101.jpg");
    assert_eq!(media.get_item(101).unwrap().state, MediaLoadingState::Ready);
}

#[test]
fn test_iv_controller_navigation_and_zoom() {
    assert!(IvController::is_good_ton_site_url("ton://wallet"));
    assert!(IvController::is_good_ton_site_url("https://foundation.ton/"));
    assert!(!IvController::is_good_ton_site_url("https://google.com/"));

    let mut controller = IvController::new();
    assert_eq!(controller.zoom, 100);

    controller.zoom_in();
    assert_eq!(controller.zoom, 110);
    controller.zoom_out();
    assert_eq!(controller.zoom, 100);

    controller.navigate_to("https://t.me/iv?url=example.com");
    controller.navigate_to("https://t.me/iv?url=another.com");
    assert_eq!(controller.current_url, "https://t.me/iv?url=another.com");
    assert!(controller.can_go_back());

    assert!(controller.go_back());
    assert_eq!(controller.current_url, "https://t.me/iv?url=example.com");

    assert!(controller.go_forward());
    assert_eq!(controller.current_url, "https://t.me/iv?url=another.com");
}

#[test]
fn test_iv_data_site_name_extraction() {
    assert_eq!(site_name_from_url("https://www.theverge.com/tech/news"), "Theverge");
    assert_eq!(site_name_from_url("http://github.com/rust-lang"), "Github");

    let data = IvData::new(554433, 998877, "https://blog.telegram.org/instant-view", false);
    assert_eq!(data.page_id(), 554433);
    assert_eq!(data.hash(), 998877);
    assert_eq!(data.name(), "Telegram");
    assert!(!data.partial());
}

#[test]
fn test_iv_delegate_and_delegate_impl() {
    let mut delegate = IvDelegate::new();
    assert_eq!(delegate.iv_zoom(), 100);

    delegate.iv_set_zoom(150);
    assert_eq!(delegate.iv_zoom(), 150);

    delegate.iv_set_zoom(400); // Clamped to 300
    assert_eq!(delegate.iv_zoom(), 300);

    let mut impl_delegate = IvDelegateImpl::new();
    let init_geom = impl_delegate.count_initial_geometry(1920, 1080);
    assert_eq!(init_geom.width, 860);
    assert_eq!(init_geom.height, 980);

    impl_delegate.iv_save_geometry(init_geom);
    assert_eq!(impl_delegate.iv_geometry(), init_geom);
}

#[test]
fn test_iv_instance_lifecycle() {
    assert!(IvInstance::is_iv_preferred("https://t.me/iv?url=test.com"));
    assert!(IvInstance::is_iv_preferred("ton://dns"));
    assert!(!IvInstance::is_iv_preferred("https://randomsite.xyz"));

    let mut instance = IvInstance::new();
    assert!(!instance.has_active_window());

    let w1 = instance.open_page(1001, "https://t.me/iv?url=test.com", "Article 1");
    assert!(instance.has_active_window());
    assert_eq!(instance.windows.len(), 1);

    let w2 = instance.open_page(1002, "https://t.me/iv?url=test2.com", "Article 2");
    assert_eq!(instance.windows.len(), 2);
    assert_eq!(instance.active_window_id, Some(w2));

    assert!(instance.close_window(w1));
    assert_eq!(instance.windows.len(), 1);

    assert!(instance.minimize_active());
    assert!(instance.windows[0].is_minimized);

    instance.close_all();
    assert!(!instance.has_active_window());
}

#[test]
fn test_iv_pch_capabilities() {
    let pch = IvPch::new();
    assert!(pch.is_feature_supported("math"));
    assert!(pch.is_feature_supported("latex"));
    assert!(pch.is_feature_supported("ton"));
    assert!(pch.is_feature_supported("export"));
    assert!(!pch.is_feature_supported("quantum_computing"));
    assert_eq!(pch.version_string(), "v2.1");
}

#[test]
fn test_iv_rich_message_html_export() {
    let mut exporter = IvRichMessageHtmlExport::new().with_title("Comprehensive Analysis");
    assert_eq!(exporter.progress(), 0.0);

    let j1 = exporter.add_media_job("hero.png", 204800);
    let j2 = exporter.add_media_job("diagram.svg", 51200);
    assert_eq!(exporter.jobs.len(), 2);

    exporter.complete_job(j1);
    assert_eq!(exporter.progress(), 0.5);

    exporter.complete_job(j2);
    assert_eq!(exporter.progress(), 1.0);
    assert!(exporter.is_settled);

    let html = exporter.generate_standalone_html();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Comprehensive Analysis"));
}

#[test]
fn test_iv_rich_message_serializer() {
    let mut serializer = IvRichMessageSerializer::new(SerializeMode::Draft);
    let res = serializer.serialize_document("Draft Title", &["Draft paragraph".to_string()]);
    assert_eq!(res.status, SerializeStatus::Success);
    assert_eq!(res.block_count, 2);

    let mut strict = IvRichMessageSerializer::new(SerializeMode::FinalSubmit);
    let empty_title_res = strict.serialize_document("", &["Content with no title".to_string()]);
    assert_eq!(empty_title_res.status, SerializeStatus::MissingRequiredTitle);

    let valid_strict_res = strict.serialize_document("Valid Title", &["Valid paragraph".to_string()]);
    assert_eq!(valid_strict_res.status, SerializeStatus::Success);
}

#[test]
fn test_iv_rich_page_ast_and_roman_numerals() {
    assert_eq!(to_roman_numeral(1, false), "i");
    assert_eq!(to_roman_numeral(4, false), "iv");
    assert_eq!(to_roman_numeral(9, false), "ix");
    assert_eq!(to_roman_numeral(14, true), "XIV");
    assert_eq!(to_roman_numeral(42, true), "XLII");

    let mut page = IvRichPage::new(1234, "Test Article");
    page.add_block(RichBlock::Heading {
        level: 1,
        text: "Main Header".to_string(),
        anchor: None,
    });
    page.add_block(RichBlock::Paragraph {
        text: "Body paragraph content".to_string(),
    });
    page.add_block(RichBlock::Divider);

    assert_eq!(page.total_blocks(), 3);
}

#[test]
fn test_iv_rich_page_quote() {
    let quote = IvRichPageQuote::new(101, "A selected phrase for reply context");
    assert_eq!(quote.word_count(), 6);
    assert!(quote.is_valid);

    assert!(quote.splits_formula("This is $E=mc^2$ formula", 12));
    assert!(!quote.splits_formula("This is $E=mc^2$ formula", 20));
}

#[test]
fn test_iv_rich_page_quote_builder_truncation() {
    let mut builder = IvRichPageQuoteBuilder::new();

    let short_quote = "Short quote within budget.";
    let b1 = builder.build_quote(short_quote, 100);
    assert!(!b1.overflown);
    assert_eq!(b1.fragments[0].text, short_quote);

    let long_quote = "This is a very long paragraph that will certainly exceed the tight character budget limit imposed on quote fragments.";
    let b2 = builder.build_quote(long_quote, 40);
    assert!(b2.overflown);
    assert!(b2.fragments[0].text.ends_with("..."));
    assert!(b2.fragments[0].text.len() <= 40);
}

#[test]
fn test_iv_search_bar_navigation() {
    let mut bar = IvSearchBar::new(SearchBarMode::WindowStrip);
    assert!(!bar.is_shown);

    bar.toggle();
    assert!(bar.is_shown);
    assert!(bar.is_focused);

    bar.set_results(1, 5);
    assert_eq!(bar.current_match, 1);

    bar.next_match();
    assert_eq!(bar.current_match, 2);

    bar.previous_match();
    assert_eq!(bar.current_match, 1);

    bar.previous_match(); // wraps to end
    assert_eq!(bar.current_match, 5);

    bar.hide();
    assert!(!bar.is_shown);
}

#[test]
fn test_iv_search_controller_indexing() {
    let mut controller = IvSearchController::new();
    controller.set_sources(vec![
        SearchSourceSegment {
            segment_id: 1,
            text: "Instant View delivers fast articles with instant view performance.".to_string(),
        },
        SearchSourceSegment {
            segment_id: 2,
            text: "Another segment mentioning Telegram instant view.".to_string(),
        },
    ]);

    let matches = controller.perform_search("instant view");
    assert_eq!(matches, 3);
    assert_eq!(controller.current_index, 0);

    let m1 = controller.current_match().unwrap();
    assert_eq!(m1.segment_id, 1);
    assert_eq!(m1.start, 0);

    let m2 = controller.next_match().unwrap();
    assert_eq!(m2.segment_id, 1);
    assert_eq!(m2.start, 41);

    let m3 = controller.next_match().unwrap();
    assert_eq!(m3.segment_id, 2);

    let m_wrap = controller.next_match().unwrap(); // circular wrap
    assert_eq!(m_wrap.segment_id, 1);
    assert_eq!(m_wrap.start, 0);

    controller.clear_search();
    assert_eq!(controller.matches.len(), 0);
    assert!(!controller.is_searching);
}
