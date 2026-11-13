//! Peer List Section Sticky and Inline Section Headers
//!
//! Upstream C++ Reference:
//! - `Telegram/SourceFiles/boxes/peer_list_section_headers.h:L1-L50`
//! - `Telegram/SourceFiles/boxes/peer_list_section_headers.cpp:L1-L89`
//! - `Telegram/SourceFiles/boxes/peer_list_section_headers.cpp:L25-L60` (Header rebuild & positioning)
//!
//! Subsystem: boxes, Feature: TCG-N11

use gpui::{px, Pixels};

/// Single section header element with content boundaries.
#[derive(Debug, Clone, PartialEq)]
pub struct SectionHeaderEntry {
    pub title: String,
    pub count: usize,
    pub content_top: Pixels,
    pub height: Pixels,
}

/// Section headers controller for virtualized peer list partitions.
/// Target symbol in ledger: `PeerListSectionHeaders`.
#[derive(Debug, Clone)]
pub struct PeerListSectionHeaders {
    pub is_visible: bool,
    pub default_height: Pixels,
    pub headers: Vec<SectionHeaderEntry>,
    pub active_header_index: Option<usize>,
}

impl Default for PeerListSectionHeaders {
    fn default() -> Self {
        Self::new()
    }
}

impl PeerListSectionHeaders {
    pub fn new() -> Self {
        Self {
            is_visible: true,
            default_height: px(32.0),
            headers: Vec::new(),
            active_header_index: None,
        }
    }

    /// Rebuilds header collection from partition list (peer_list_section_headers.cpp:L28-L45).
    pub fn rebuild(&mut self, entries: Vec<(String, usize, Pixels)>) {
        self.headers = entries
            .into_iter()
            .map(|(title, count, content_top)| SectionHeaderEntry {
                title,
                count,
                content_top,
                height: self.default_height,
            })
            .collect();
        self.update_active(px(0.0));
    }

    /// Finds sticky header index for current scroll offset (peer_list_section_headers.cpp:L48-L65).
    pub fn update_active(&mut self, scroll_top: Pixels) -> Option<&SectionHeaderEntry> {
        let mut active = None;
        for (idx, h) in self.headers.iter().enumerate() {
            if h.content_top <= scroll_top {
                active = Some(idx);
            } else {
                break;
            }
        }
        self.active_header_index = active;
        self.active_header()
    }

    pub fn active_header(&self) -> Option<&SectionHeaderEntry> {
        self.active_header_index.and_then(|i| self.headers.get(i))
    }

    pub fn has_header(&self) -> bool {
        !self.headers.is_empty()
    }

    pub fn clear(&mut self) {
        self.headers.clear();
        self.active_header_index = None;
    }
}
