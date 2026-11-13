//! Adversarial and Stress Test Suite for Module 04: Layout and Window Subsystems
//! Subsystems tested:
//! 1. `kMatrixRowShift = 40000` encoding/decoding invariants (fuzzing 10,000 pairs + boundary values)
//! 2. Grouped media album selection bitmask math (15-item subgroup, bit overflow > 15 rejection)
//! 3. Mosaic layout algorithm & hit-testing (extreme aspect ratios, zero/negative bounds, row wrap, boundaries)
//! 4. WindowController layer stack stress (500 modals, 500 toasts, lock screen preemption, FIFO/LIFO invariants)
//! 5. Document 4-color categorization fuzzing (100+ MIME/exts, non-ASCII/case-insensitive, modulo fallback)

use std::panic::catch_unwind;

use gpui_telegram_desktop::layout::{
    AbstractLayoutItem, DocumentColorIndex, LayoutDocumentGenericPreview, LayoutMosaic,
    LayoutPosition, LayoutSelection, Position, TextSelection, FULL_SELECTION,
    INLINE_ITEMS_MAX_PER_ROW, INLINE_RESULTS_MIN_WIDTH, MATRIX_ROW_SHIFT,
    MAX_GROUP_SELECTION_ITEMS,
};
use gpui_telegram_desktop::window::{LayerKind, WindowController, WindowId};

/// Deterministic Pseudo-Random Number Generator (Xorshift64) for reproducible adversarial fuzzing.
struct FuzzRng {
    state: u64,
}

impl FuzzRng {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x853c49e6748fea9b } else { seed },
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    fn gen_range(&mut self, min: i32, max: i32) -> i32 {
        assert!(min <= max);
        let range = (max as i64 - min as i64 + 1) as u64;
        (min as i64 + (self.next_u64() % range) as i64) as i32
    }
}

// ============================================================================
// 1. MATRIX ROW SHIFT = 40000 ENCODING / DECODING ADVERSARIAL STRESS
// ============================================================================

#[test]
fn test_adversarial_matrix_row_shift_fuzz_10000() {
    assert_eq!(MATRIX_ROW_SHIFT, 40000);

    let mut rng = FuzzRng::new(0xDEADBEEFCAFE);

    // 1. Explicit boundary and extreme values
    let boundary_cases = [
        (0, 0),
        (0, 1),
        (0, 39999),
        (1, 0),
        (1, 1),
        (1, 39999),
        (10, 250),
        (100, 15000),
        (1000, 39999),
        (10000, 0),
        (10000, 39999),
        (40000, 0),
        (40000, 39999),
        (50000, 0),
        (50000, 39999),
    ];

    for &(row, col) in &boundary_cases {
        let expected_index = row * MATRIX_ROW_SHIFT + col;
        let index = LayoutPosition::position_to_index(row, col);
        assert_eq!(index, expected_index, "position_to_index mismatch for ({row}, {col})");

        let pos = LayoutPosition::index_to_position(index);
        assert_eq!(pos.row, row, "Decoded row mismatch for index {index}");
        assert_eq!(pos.column, col, "Decoded column mismatch for index {index}");
        assert!(pos.is_valid(), "Decoded position should be valid for ({row}, {col})");
        assert_eq!(pos.to_index(), index);
        assert_eq!(LayoutPosition::from_position(&pos), index);
    }

    // 2. Fuzz 10,000 random (row, col) pairs across 0..=50,000 rows and 0..=39,999 columns
    for i in 0..10_000 {
        let row = rng.gen_range(0, 50_000);
        let col = rng.gen_range(0, 39_999);

        let index = LayoutPosition::position_to_index(row, col);
        let expected_index = row * 40000 + col;
        assert_eq!(index, expected_index, "Iteration {i}: index calculation failed");

        let decoded = LayoutPosition::index_to_position(index);
        assert_eq!(decoded.row, row, "Iteration {i}: row mismatch");
        assert_eq!(decoded.column, col, "Iteration {i}: col mismatch");
        assert!(decoded.is_valid(), "Iteration {i}: validity failed");
        assert_eq!(decoded.to_index(), index);
    }

    // 3. Overflow and column >= 40,000 behavior
    // If col == 40,000, position_to_index(row, 40000) = (row + 1) * 40000.
    // Decoding then yields row = row + 1, col = 0.
    for row in [0, 1, 100, 1000, 50000] {
        let index = LayoutPosition::position_to_index(row, 40000);
        let decoded = LayoutPosition::index_to_position(index);
        assert_eq!(decoded.row, row + 1);
        assert_eq!(decoded.column, 0);
    }

    // 4. Negative and invalid index handling
    let negative_indices = [
        -1,
        -2,
        -100,
        -40000,
        -50000,
        -1000000,
        i32::MIN,
    ];
    for &neg_idx in &negative_indices {
        let decoded = LayoutPosition::index_to_position(neg_idx);
        assert_eq!(decoded, Position::INVALID, "Negative index {neg_idx} should decode to INVALID");
        assert_eq!(decoded.row, -1);
        assert_eq!(decoded.column, -1);
        assert!(!decoded.is_valid());
    }

    // 5. Position coordinate validity
    assert!(!Position::new(-1, 0).is_valid());
    assert!(!Position::new(0, -1).is_valid());
    assert!(!Position::new(-5, -5).is_valid());
    assert!(Position::new(0, 0).is_valid());
}

// ============================================================================
// 2. GROUPED MEDIA ALBUM SELECTION BITMASK MATH ADVERSARIAL STRESS
// ============================================================================

#[test]
fn test_adversarial_grouped_album_selection_bitmask() {
    assert_eq!(MAX_GROUP_SELECTION_ITEMS, 15);

    // 1. TextSelection empty and full invariants
    let empty = TextSelection::EMPTY;
    assert_eq!(empty.from, 0);
    assert_eq!(empty.to, 0);
    assert!(empty.is_empty());
    assert!(!empty.is_full());
    assert!(!LayoutSelection::is_sub_group_selection(empty));
    assert_eq!(LayoutSelection::first_group_item_index(empty), None);

    let full = FULL_SELECTION;
    assert_eq!(full.from, 0xFFFF);
    assert_eq!(full.to, 0xFFFF);
    assert!(full.is_full());
    assert!(!full.is_empty());
    assert!(!LayoutSelection::is_sub_group_selection(full));
    assert_eq!(LayoutSelection::first_group_item_index(full), None);

    for i in 0..=15 {
        assert!(!LayoutSelection::is_group_item_selection(empty, i));
        assert!(!LayoutSelection::is_group_item_selection(full, i));
    }

    // 2. Comprehensive verification of all 15 sub-group items individually
    for i in 0..15 {
        let sel = LayoutSelection::add_group_item_selection(TextSelection::EMPTY, i);
        assert_eq!(sel.from, 0xFFFF);
        assert_eq!(sel.to, 1u16 << i);
        assert!(LayoutSelection::is_sub_group_selection(sel));
        assert!(LayoutSelection::is_group_item_selection(sel, i));
        assert_eq!(LayoutSelection::first_group_item_index(sel), Some(i));

        // Ensure other items are not selected
        for j in 0..15 {
            if j != i {
                assert!(!LayoutSelection::is_group_item_selection(sel, j), "Item {j} should not be selected");
            }
        }

        // Removal returns back to EMPTY
        let removed = LayoutSelection::remove_group_item_selection(sel, i);
        assert_eq!(removed, TextSelection::EMPTY);
        assert!(!LayoutSelection::is_sub_group_selection(removed));
        assert_eq!(LayoutSelection::first_group_item_index(removed), None);
    }

    // 3. Sequential addition of all 15 items (0..=14)
    let mut cumulative = TextSelection::EMPTY;
    for i in 0..15 {
        cumulative = LayoutSelection::add_group_item_selection(cumulative, i);
        assert!(LayoutSelection::is_sub_group_selection(cumulative));
        assert_eq!(LayoutSelection::first_group_item_index(cumulative), Some(0));
        for k in 0..=i {
            assert!(LayoutSelection::is_group_item_selection(cumulative, k));
        }
    }
    // All 15 bits set: 0x7FFF (bit 15 must be 0)
    assert_eq!(cumulative.to, 0x7FFF);
    assert!(LayoutSelection::is_sub_group_selection(cumulative));

    // Sequential removal from 0 up to 14
    for i in 0..15 {
        assert_eq!(LayoutSelection::first_group_item_index(cumulative), Some(i));
        assert!(LayoutSelection::is_group_item_selection(cumulative, i));
        cumulative = LayoutSelection::remove_group_item_selection(cumulative, i);
        assert!(!LayoutSelection::is_group_item_selection(cumulative, i));
    }
    assert_eq!(cumulative, TextSelection::EMPTY);

    // 4. Bit overflow rejection > 15 items
    // `add_group_item_selection` asserts index < MAX_GROUP_SELECTION_ITEMS (15)
    for invalid_idx in [15, 16, 20, 100, usize::MAX] {
        let panic_res = catch_unwind(|| {
            LayoutSelection::add_group_item_selection(TextSelection::EMPTY, invalid_idx);
        });
        assert!(panic_res.is_err(), "Index {invalid_idx} must panic on add_group_item_selection");

        // `is_group_item_selection` returns false safely without panicking
        assert!(!LayoutSelection::is_group_item_selection(TextSelection::new(0xFFFF, 0x7FFF), invalid_idx));

        // `remove_group_item_selection` returns selection unchanged safely
        let sel = TextSelection::new(0xFFFF, 0x000F);
        assert_eq!(LayoutSelection::remove_group_item_selection(sel, invalid_idx), sel);
    }

    // 5. Random subset selection fuzzing
    let mut rng = FuzzRng::new(0xFACEFEED);
    for _ in 0..500 {
        let mask = (rng.next_u32() & 0x7FFF) as u16; // 15 bits
        let mut expected_items = Vec::new();
        for i in 0..15 {
            if (mask & (1 << i)) != 0 {
                expected_items.push(i);
            }
        }

        let mut sel = TextSelection::EMPTY;
        for &item in &expected_items {
            sel = LayoutSelection::add_group_item_selection(sel, item);
        }

        if expected_items.is_empty() {
            assert_eq!(sel, TextSelection::EMPTY);
            assert_eq!(LayoutSelection::first_group_item_index(sel), None);
        } else {
            assert!(LayoutSelection::is_sub_group_selection(sel));
            assert_eq!(LayoutSelection::first_group_item_index(sel), Some(expected_items[0]));
            for i in 0..15 {
                let should_be = expected_items.contains(&i);
                assert_eq!(LayoutSelection::is_group_item_selection(sel, i), should_be);
            }
        }
    }
}

// ============================================================================
// 3. MOSAIC LAYOUT ALGORITHM & HIT-TESTING ADVERSARIAL STRESS
// ============================================================================

#[test]
fn test_adversarial_mosaic_layout_and_hit_testing() {
    assert_eq!(INLINE_ITEMS_MAX_PER_ROW, 5);
    assert_eq!(INLINE_RESULTS_MIN_WIDTH, 40);

    // 1. Extreme aspect ratios (extremely narrow 0.1, extremely wide 10.0, zero/negative bounds)
    // Within container width 800, all 5 items fit in 1 row
    let mut mosaic = LayoutMosaic::new(800);
    mosaic.set_padding(10, 10, 10, 10);
    mosaic.set_right_skip(6);

    let extreme_items = vec![
        AbstractLayoutItem::new(10, 100),   // narrow aspect ratio 0.1
        AbstractLayoutItem::new(500, 50),   // wide aspect ratio 10.0
        AbstractLayoutItem::new(0, 0),      // zero bounds
        AbstractLayoutItem::new(-50, -20),  // negative bounds
        AbstractLayoutItem::new(100, 80),   // normal item
    ];

    mosaic.add_items(extreme_items);
    assert_eq!(mosaic.rows_count(), 1, "5 items within width 800 must form exactly 1 row");

    let computed_height = mosaic.count_desired_height(800);
    assert!(computed_height >= 100 + 20, "Height must encompass tallest item (100) + padding (20)");

    // Demonstrating width-triggered row wrap: with big_width=400, item with max_width 500 forces wrapping
    let mut wrap_width_mosaic = LayoutMosaic::new(400);
    wrap_width_mosaic.set_right_skip(6);
    wrap_width_mosaic.add_items(vec![
        AbstractLayoutItem::new(10, 100),
        AbstractLayoutItem::new(500, 50), // exceeds 400
        AbstractLayoutItem::new(100, 80),
    ]);
    assert_eq!(wrap_width_mosaic.rows_count(), 2, "Item exceeding big_width must finalize row into 2 rows");

    // 2. Row wrap math with INLINE_ITEMS_MAX_PER_ROW = 5 cap
    let mut wrap_mosaic = LayoutMosaic::new(1000);
    wrap_mosaic.set_padding(0, 0, 0, 0);
    wrap_mosaic.set_right_skip(4);

    // Adding 12 items (width 50 each fits in 1000) -> wraps into 3 rows strictly capped at 5 per row (5, 5, 2)
    let items_12: Vec<_> = (0..12).map(|_| AbstractLayoutItem::new(50, 60)).collect();
    wrap_mosaic.add_items(items_12);
    assert_eq!(wrap_mosaic.rows_count(), 3);
    assert_eq!(wrap_mosaic.rows[0].items.len(), 5);
    assert_eq!(wrap_mosaic.rows[1].items.len(), 5);
    assert_eq!(wrap_mosaic.rows[2].items.len(), 2);

    // Verify row and column indices encoded into items match grid coordinates
    for (r_idx, row) in wrap_mosaic.rows.iter().enumerate() {
        for (c_idx, item) in row.items.iter().enumerate() {
            let pos = LayoutPosition::index_to_position(item.get_position());
            assert_eq!(pos.row, r_idx as i32);
            assert_eq!(pos.column, c_idx as i32);
        }
    }

    // Also verify premature wrap when accumulated width exceeds big_width (300px with 80px items -> wraps at 4 items)
    let mut width_capped_mosaic = LayoutMosaic::new(300);
    width_capped_mosaic.set_right_skip(4);
    let items_80: Vec<_> = (0..8).map(|_| AbstractLayoutItem::new(80, 60)).collect();
    width_capped_mosaic.add_items(items_80);
    assert_eq!(width_capped_mosaic.rows_count(), 2);
    assert_eq!(width_capped_mosaic.rows[0].items.len(), 4);
    assert_eq!(width_capped_mosaic.rows[1].items.len(), 4);

    // 3. Hit-testing boundary coordinate edge cases
    let mut hit_mosaic = LayoutMosaic::new(320);
    hit_mosaic.set_padding(12, 12, 12, 12);
    hit_mosaic.set_right_skip(4);

    let row_items = vec![
        AbstractLayoutItem::new(100, 50),
        AbstractLayoutItem::new(100, 50),
        AbstractLayoutItem::new(100, 50),
    ];
    hit_mosaic.add_items(row_items);
    let total_h = hit_mosaic.count_desired_height(320);
    assert_eq!(total_h, 50 + 24);

    // Top-left corner: exactly at padding (12, 12)
    let hit_tl = hit_mosaic.find_by_point(12, 12);
    assert_eq!(hit_tl.index, 0);
    assert!(hit_tl.exact);
    assert_eq!(hit_tl.relative_x, 0);
    assert_eq!(hit_tl.relative_y, 0);

    // Top-left boundary: negative / out-of-bounds coords clamp gracefully with exact=false
    let hit_neg = hit_mosaic.find_by_point(-50, -50);
    assert_eq!(hit_neg.index, 0);
    // Right boundary overflow within valid row: clamps col_idx to last item in the row with exact=false
    let hit_right = hit_mosaic.find_by_point(10_000, 20);
    assert_eq!(hit_right.index, 2); // 3 items in row 0 -> index 2
    assert!(!hit_right.exact);

    // Bottom boundary overflow below all rows:
    // When sy >= padding_top + total_height, clamps to the last row with exact = false.
    let hit_below = hit_mosaic.find_by_point(20, 10_000);
    assert_eq!(hit_below.index, 0);
    assert_eq!(hit_below.exact, false);

    // Bounding box query
    let rect0 = hit_mosaic.find_rect(0);
    assert!(rect0.is_some());
    let (rx, ry, rw, rh) = rect0.unwrap();
    assert_eq!((rx, ry), (12, 12));
    assert!(rw >= INLINE_RESULTS_MIN_WIDTH);
    assert_eq!(rh, 50);

    // Invalid index bounding box query returns None
    assert_eq!(hit_mosaic.find_rect(-1), None);
    assert_eq!(hit_mosaic.find_rect(999), None);

    // 4. Empty mosaic safe hit-testing
    let empty_mosaic = LayoutMosaic::new(300);
    assert!(empty_mosaic.is_empty());
    assert_eq!(empty_mosaic.rows_count(), 0);
    let empty_hit = empty_mosaic.find_by_point(50, 50);
    assert_eq!(empty_hit.index, -1);
    assert_eq!(empty_hit.exact, false);
    assert_eq!(empty_mosaic.find_rect(0), None);
}

// ============================================================================
// 4. WINDOW CONTROLLER LAYER STACK & TOAST ADVERSARIAL STRESS
// ============================================================================

#[test]
fn test_adversarial_window_controller_layer_stack_and_toasts() {
    let mut controller = WindowController::new();
    assert!(controller.is_primary());
    assert_eq!(controller.id, WindowId::Primary);
    assert!(!controller.is_layer_shown());
    assert!(!controller.locked());

    // 1. Rapid push of 500 modal dialogs
    for i in 0..500 {
        controller.show_box(format!("BoxModal_{i}"));
    }
    assert_eq!(controller.active_layers.len(), 500);
    assert!(controller.is_layer_shown());

    // 2. Rapid LIFO pop of all 500 dialogs
    for i in (0..500).rev() {
        let popped = controller.hide_layer();
        assert_eq!(popped, Some(LayerKind::BoxModal(format!("BoxModal_{i}"))));
    }
    assert_eq!(controller.active_layers.len(), 0);
    assert!(!controller.is_layer_shown());

    // 3. Rapid enqueue and FIFO sequence of 500 toast notifications
    for i in 1..=500 {
        controller.show_toast(format!("ToastNotification_{i}"), 1500);
    }
    assert!(controller.current_toast.is_some());
    assert_eq!(controller.current_toast.as_ref().unwrap().text, "ToastNotification_1");
    assert_eq!(controller.toast_queue.len(), 499);

    for i in 1..=500 {
        let current = controller.current_toast.as_ref().expect("Toast should exist");
        assert_eq!(current.text, format!("ToastNotification_{i}"));
        controller.dismiss_current_toast();
    }
    assert!(controller.current_toast.is_none());
    assert!(controller.toast_queue.is_empty());

    // 4. Lock screen preemption invariant
    // Modals present before lock
    controller.show_box("UnderLockDialog1");
    controller.show_box("UnderLockDialog2");
    controller.show_layer(LayerKind::Settings);
    assert_eq!(controller.active_layers.len(), 3);

    // Engage Passcode Lock
    controller.setup_passcode_lock();
    assert!(controller.is_locked);
    assert!(controller.locked());
    assert!(controller.active_layers.contains(&LayerKind::PasscodeLock));

    // While locked, back button must NOT dismiss layers
    for _ in 0..20 {
        assert!(!controller.close_layer_by_back_button(), "Back button must be preempted when locked");
    }
    assert!(controller.active_layers.contains(&LayerKind::PasscodeLock));
    assert_eq!(controller.active_layers.len(), 4);

    // Clear Passcode Lock
    controller.clear_passcode_lock();
    assert!(!controller.is_locked);
    assert!(!controller.locked());
    assert!(!controller.active_layers.contains(&LayerKind::PasscodeLock));
    assert_eq!(controller.active_layers.len(), 3);

    // Now back button closes layers in strict LIFO order
    assert!(controller.close_layer_by_back_button()); // Settings
    assert!(controller.close_layer_by_back_button()); // UnderLockDialog2
    assert!(controller.close_layer_by_back_button()); // UnderLockDialog1
    assert!(!controller.close_layer_by_back_button()); // Nothing left
    assert_eq!(controller.active_layers.len(), 0);

    // 5. Setup Email lock preemption
    controller.setup_setup_email_lock();
    assert!(controller.is_email_lock);
    assert!(controller.locked());
    assert!(controller.active_layers.contains(&LayerKind::SetupEmail));
    controller.clear_setup_email_lock();
    assert!(!controller.locked());
    assert_eq!(controller.active_layers.len(), 0);

    // 6. Selective hide_settings_and_layer preservation of lock layers
    controller.show_box("ModalBox1");
    controller.show_layer(LayerKind::Settings);
    controller.show_layer(LayerKind::ThemeEditor);
    controller.setup_passcode_lock();
    controller.setup_setup_email_lock();

    controller.hide_settings_and_layer();
    // ModalBox1, Settings, ThemeEditor should be removed, but PasscodeLock and SetupEmail must survive!
    assert!(controller.active_layers.contains(&LayerKind::PasscodeLock));
    assert!(controller.active_layers.contains(&LayerKind::SetupEmail));
    assert!(!controller.active_layers.contains(&LayerKind::Settings));
    assert!(!controller.active_layers.contains(&LayerKind::ThemeEditor));
    assert_eq!(controller.active_layers.len(), 2);
}

// ============================================================================
// 5. DOCUMENT 4-COLOR CATEGORIZATION FUZZING & ADVERSARIAL CASES
// ============================================================================

#[test]
fn test_adversarial_document_four_color_categorization_fuzzing() {
    // 1. Category 0: Blue (Documents, Plain text, PSD, text/* MIME)
    let blue_filenames = [
        "doc.doc", "DOCUMENT.DOC", "letter.docx", "CONTRACT.DOCX",
        "notes.txt", "README.TXT", "design.psd", "MOCKUP.PSD",
        "mixed.dOcX", "file.tXt", "PHOTO.Psd",
    ];
    for name in &blue_filenames {
        let preview = LayoutDocumentGenericPreview::create(name);
        assert_eq!(preview.color_palette(), DocumentColorIndex::Blue, "Failed for {name}");
        assert_eq!(preview.index, 0);
    }

    // 30+ diverse text/* MIME types all classify as Blue
    let text_mimes = [
        "text/plain", "text/html", "text/css", "text/javascript", "text/csv",
        "text/xml", "text/markdown", "text/x-rust", "text/x-python", "text/x-c",
        "text/x-java", "text/yaml", "text/toml", "text/calendar", "text/rtf",
        "text/tab-separated-values", "text/uri-list", "text/vcard", "text/cache-manifest",
        "TEXT/PLAIN", "Text/Html", "text/prs.lines.tag", "text/vnd.curl",
    ];
    for mime in &text_mimes {
        let preview = LayoutDocumentGenericPreview::create_with_mime("arbitrary_file.ext", Some(mime));
        assert_eq!(preview.color_palette(), DocumentColorIndex::Blue, "Failed for MIME {mime}");
        assert_eq!(preview.index, 0);
    }

    // 2. Category 1: Green (Spreadsheets: xls, xlsx, csv without text/* mime)
    let green_filenames = [
        "budget.xls", "FINANCE.XLS", "table.xlsx", "METRICS.XLSX",
        "export.csv", "DATA.CSV", "Sheet.XlSx", "log.CsV",
    ];
    for name in &green_filenames {
        let preview = LayoutDocumentGenericPreview::create(name);
        assert_eq!(preview.color_palette(), DocumentColorIndex::Green, "Failed for {name}");
        assert_eq!(preview.index, 1);
    }

    // 3. Category 2: Red (PDF, Presentations: pdf, ppt, pptx, key)
    let red_filenames = [
        "whitepaper.pdf", "REPORT.PDF", "slides.ppt", "TALK.PPT",
        "deck.pptx", "KEYNOTE.PPTX", "pitch.key", "PRESENTATION.KEY",
        "Paper.PdF", "Seminar.PpTx",
    ];
    for name in &red_filenames {
        let preview = LayoutDocumentGenericPreview::create(name);
        assert_eq!(preview.color_palette(), DocumentColorIndex::Red, "Failed for {name}");
        assert_eq!(preview.index, 2);
    }

    // 4. Category 3: Yellow (Archives & Media: zip, rar, ai, mp3, mov, avi)
    let yellow_filenames = [
        "bundle.zip", "ARCHIVE.ZIP", "backup.rar", "FILES.RAR",
        "vector.ai", "LOGO.AI", "song.mp3", "AUDIO.MP3",
        "video.mov", "CLIP.MOV", "movie.avi", "FILM.AVI",
        "Pack.ZiP", "Sound.Mp3", "Footage.MoV",
    ];
    for name in &yellow_filenames {
        let preview = LayoutDocumentGenericPreview::create(name);
        assert_eq!(preview.color_palette(), DocumentColorIndex::Yellow, "Failed for {name}");
        assert_eq!(preview.index, 3);
    }

    // 5. Non-ASCII and Unicode extensions & filenames
    let unicode_cases = [
        ("документ.docx", DocumentColorIndex::Blue),
        ("отчет.xlsx", DocumentColorIndex::Green),
        ("презентация.pdf", DocumentColorIndex::Red),
        ("архив.zip", DocumentColorIndex::Yellow),
    ];
    for (name, expected_color) in &unicode_cases {
        let preview = LayoutDocumentGenericPreview::create(name);
        assert_eq!(preview.color_palette(), *expected_color, "Failed for unicode {name}");
    }

    // 6. Unknown extensions modulo fallback: ((first_char as usize) % 4) & 3
    let unknown_extensions = [
        "test.bin", "data.dat", "firmware.hex", "script.sh", "code.rs",
        "project.toml", "config.json", "styles.sass", "font.ttf", "vector.svg",
        "image.webp", "model.onnx", "db.sqlite", "blob.o", "lib.rlib",
        "package.tar", "document.pages", "book.epub", "ebook.mobi",
    ];
    for file in &unknown_extensions {
        let ext = file.split('.').last().unwrap();
        let first_ch = ext.chars().next().unwrap();
        let expected_modulo = ((first_ch as usize) % 4) & 3;

        let preview = LayoutDocumentGenericPreview::create(file);
        assert_eq!(preview.index, expected_modulo, "Modulo mismatch for {file}");
        assert!(preview.index < 4);
    }

    // 7. Extreme edge cases: no extension, hidden files, empty filename, trailing dots
    let edge_cases = [
        "",
        ".",
        "..",
        "...",
        ".gitignore",
        ".env",
        "LICENSE",
        "Makefile",
        "file.",
        "archive.tar.gz",
        "my.deeply.nested.folder.file.pdf",
    ];

    for name in &edge_cases {
        let preview = LayoutDocumentGenericPreview::create(name);
        assert!(preview.index < 4, "Color index must strictly be 0..3 for '{name}'");
        assert_ne!(preview.color, 0, "Color cannot be 0");
        assert_ne!(preview.dark, 0, "Dark color cannot be 0");
        assert_ne!(preview.over, 0, "Over color cannot be 0");
        assert_eq!(preview.selected, 0xFFFFFFFF);
        assert!(preview.icon.starts_with("mediaviewFile"));
    }

    // 8. Fuzz 100+ random synthesized filenames with arbitrary extensions and MIME types
    let mut rng = FuzzRng::new(0xBADC0FFEE0);
    for _ in 0..150 {
        let name_len = rng.gen_range(1, 20) as usize;
        let ext_len = rng.gen_range(1, 8) as usize;

        let name: String = (0..name_len)
            .map(|_| (b'a' + (rng.next_u32() % 26) as u8) as char)
            .collect();
        let ext: String = (0..ext_len)
            .map(|_| (b'a' + (rng.next_u32() % 26) as u8) as char)
            .collect();

        let filename = format!("{name}.{ext}");
        let preview = LayoutDocumentGenericPreview::create(&filename);

        assert!(preview.index < 4);
        assert_eq!(preview.color_palette().to_index(), preview.index);
    }
}
