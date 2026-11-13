//! URL routing and IV detection handler
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

#[derive(Debug, Clone, Default)]
pub struct IvUrlHandler;

impl IvUrlHandler {
    pub fn is_instant_view_supported(url: &str) -> bool {
        let u = url.to_lowercase();
        u.starts_with("https://telegra.ph/")
            || u.starts_with("http://telegra.ph/")
            || u.contains("t.me/iv?")
            || u.contains("telegram.org/blog/")
    }

    pub fn extract_rhash(url: &str) -> Option<String> {
        if let Some(pos) = url.find("rhash=") {
            let sub = &url[pos + 6..];
            let end = sub.find('&').unwrap_or(sub.len());
            Some(sub[..end].to_string())
        } else {
            None
        }
    }
}
