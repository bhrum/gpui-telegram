//! Theme and Style Sheet Tests for Passport
//! Upstream: `Telegram/SourceFiles/passport/passport.style`

use gpui::px;
use gpui_telegram_desktop::theme::passport::PassportStyleSheet;

#[test]
fn test_passport_style_sheet_metrics() {
    let style = PassportStyleSheet::new();

    // Verify 1:1 upstream metrics from passport.style
    assert_eq!(style.panel_width, px(392.0));
    assert_eq!(style.panel_height, px(600.0));
    assert_eq!(style.password_submit_width, px(200.0));
    assert_eq!(style.password_submit_height, px(44.0));
    assert_eq!(style.password_userpic_size, px(80.0));
    assert_eq!(style.form_userpic_size, px(60.0));
    assert_eq!(style.details_field_left, px(116.0));

    let (w, h) = style.panel_size();
    assert_eq!(w, px(392.0));
    assert_eq!(h, px(600.0));

    let content_w = style.content_width(px(392.0));
    assert_eq!(content_w, px(392.0 - 22.0 - 25.0));
}
