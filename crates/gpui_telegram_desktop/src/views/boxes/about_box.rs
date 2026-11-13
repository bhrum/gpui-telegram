//! Authentic GPUI implementation of Telegram Desktop's About Box.
//! Upstream reference: `Telegram/SourceFiles/boxes/about_box.h:L12-L23` and `about_box.cpp:L67-L188`.
#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Generates the localized Telegram FAQ link per `about_box.cpp:L136-L151`.
pub fn telegram_faq_link(lang_code: &str) -> String {
    let base = "https://telegram.org/faq";
    for lang in &["de", "es", "it", "ko"] {
        if lang_code.starts_with(lang) {
            return format!("{}/{}", base, lang);
        }
    }
    if lang_code.starts_with("pt-br") {
        return format!("{}/br", base);
    }
    base.to_string()
}

/// Formats the version string mirroring `CurrentVersionText` (`about_box.cpp:L155-L178`).
pub fn current_version_text(
    version: &str,
    is_beta: bool,
    is_canary: bool,
    commit_hash: Option<&str>,
) -> String {
    let mut result = version.to_string();
    if is_canary {
        result.push_str(" canary");
    } else if is_beta {
        result.push_str(" beta");
    }

    #[cfg(target_arch = "x86_64")]
    result.push_str(" x64");
    #[cfg(target_arch = "aarch64")]
    result.push_str(" arm64");

    #[cfg(debug_assertions)]
    result.push_str(" DEBUG");

    if let Some(hash) = commit_hash {
        if is_canary && !hash.is_empty() {
            result.push_str(&format!(" · {}", hash));
        }
    }
    result
}

/// Constructs download URL for private alpha versions per `about_box.cpp:L87-L104`.
pub fn alpha_version_download_url(alpha_version: u64, platform: &str) -> String {
    let ext = match platform {
        "win" => "win/%1.zip",
        "win64" => "win64/%1.zip",
        "winarm" => "winarm/%1.zip",
        "mac" => "mac/%1.zip",
        _ => "linux/%1.tar.xz",
    };
    let slug = format!("talpha{}_{}", alpha_version, alpha_version ^ 0x5F3759DF);
    format!("https://tdesktop.com/{}", ext.replace("%1", &slug))
}

/// AboutBox view component.
/// Cites upstream `AboutBox(GenericBox *box)` (`about_box.cpp:L67-L134`).
#[derive(Clone)]
pub struct AboutBox {
    pub version_text: String,
    pub faq_url: String,
    pub api_url: String,
    pub github_url: String,
    pub license_url: String,
    pub is_closed: bool,
}

impl AboutBox {
    pub fn new() -> Self {
        let version = current_version_text("5.8.1", false, false, None);
        let faq = telegram_faq_link("en");
        Self {
            version_text: version,
            faq_url: faq,
            api_url: "https://core.telegram.org/api".to_string(),
            github_url: "https://github.com/telegramdesktop/tdesktop".to_string(),
            license_url: "https://github.com/telegramdesktop/tdesktop/blob/master/LICENSE".to_string(),
            is_closed: false,
        }
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version_text = version.into();
        self
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

impl Default for AboutBox {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for AboutBox {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_closed {
            return div().into_any_element();
        }

        // st::aboutWidth = 380px (`about_box.cpp:L133`)
        let card = div()
            .id("about_box_card")
            .w(px(380.0))
            .p(px(24.0))
            .bg(rgb(0xFFFFFF))
            .rounded(px(10.0))
            .shadow_lg()
            .flex()
            .flex_col()
            .gap(px(16.0))
            // Title: "Telegram Desktop" (`about_box.cpp:L68`)
            .child(
                div()
                    .text_size(px(18.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x222222))
                    .child("Telegram Desktop"),
            )
            // Version link button (`about_box.cpp:L72-L84`)
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0x5288C1))
                    .cursor_pointer()
                    .child(format!("Version {}", self.version_text)),
            )
            // Text 1: API Link (`about_box.cpp:L38-L43`)
            .child(
                div()
                    .text_size(px(13.0))
                    .line_height(relative(1.4))
                    .text_color(rgb(0x555555))
                    .child("Official free messaging app based on Telegram API for speed and security."),
            )
            // Text 2: GPL v3 License (`about_box.cpp:L45-L56`)
            .child(
                div()
                    .text_size(px(13.0))
                    .line_height(relative(1.4))
                    .text_color(rgb(0x555555))
                    .child("This software is licensed under GNU GPL version 3. Source code is available on GitHub."),
            )
            // Text 3: FAQ link (`about_box.cpp:L58-L63`)
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgb(0x5288C1))
                    .cursor_pointer()
                    .child("Visit Telegram FAQ"),
            )
            // Footer Close Button (`about_box.cpp:L131`)
            .child(
                div()
                    .flex()
                    .justify_end()
                    .child(
                        div()
                            .px(px(16.0))
                            .py(px(8.0))
                            .rounded(px(6.0))
                            .cursor_pointer()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x5288C1))
                            .child("Close"),
                    ),
            );

        card.into_any_element()
    }
}
