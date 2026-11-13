//! Cross-Platform Security-Scoped File Bookmark Interface
//! Upstream Source: Telegram/SourceFiles/platform/platform_file_bookmark.h
//! Subsystem: platform | Feature ID: TCG-N46 | Disposition: rust_domain_or_server_1to1

use gpui_telegram_core::error::Result;
use serde::{Deserialize, Serialize};

/// Security-scoped file bookmark for sandbox persistence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFileBookmark {
    pub raw_bookmark_data: Vec<u8>,
    pub resolved_path: String,
    pub is_access_enabled: bool,
    pub is_stale: bool,
}

impl PlatformFileBookmark {
    pub const BOOKMARK_MAGIC: &'static [u8] = b"SEC_BOOKMARK:";

    pub fn new(bookmark_data: Vec<u8>) -> Self {
        let is_valid = bookmark_data.starts_with(Self::BOOKMARK_MAGIC);
        let path = if is_valid {
            String::from_utf8_lossy(&bookmark_data[Self::BOOKMARK_MAGIC.len()..]).into_owned()
        } else {
            String::new()
        };

        Self {
            raw_bookmark_data: bookmark_data,
            resolved_path: path,
            is_access_enabled: false,
            is_stale: false,
        }
    }

    /// Factory method creating a bookmark from a target file path.
    pub fn create_path_bookmark(path: &str) -> Vec<u8> {
        let mut data = Self::BOOKMARK_MAGIC.to_vec();
        data.extend_from_slice(path.as_bytes());
        data
    }

    pub fn check(&self) -> bool {
        !self.raw_bookmark_data.is_empty() && !self.is_stale
    }

    pub fn enable(&mut self) -> Result<bool> {
        if self.check() {
            self.is_access_enabled = true;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn disable(&mut self) -> Result<()> {
        self.is_access_enabled = false;
        Ok(())
    }

    pub fn name<'a>(&'a self, original: &'a str) -> &'a str {
        if !self.resolved_path.is_empty() {
            &self.resolved_path
        } else {
            original
        }
    }

    pub fn bookmark(&self) -> &[u8] {
        &self.raw_bookmark_data
    }
}

impl Default for PlatformFileBookmark {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_bookmark_lifecycle() {
        let path = "/Users/bhrum/Downloads/photo.jpg";
        let raw = PlatformFileBookmark::create_path_bookmark(path);

        let mut bm = PlatformFileBookmark::new(raw);
        assert!(bm.check());
        assert_eq!(bm.name("default.jpg"), path);

        assert!(!bm.is_access_enabled);
        assert!(bm.enable().unwrap());
        assert!(bm.is_access_enabled);

        bm.disable().unwrap();
        assert!(!bm.is_access_enabled);
    }
}
