//! Info Profile Section Stack component & Visibility Optimizer
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_section_stack.cpp:L1-L248`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_section_stack.h:L1-L63`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileSectionKind {
    Main,
    SharedMedia,
    Members,
    SimilarChannels,
    Gifts,
}

/// Upstream `SectionStack::RowType` (`info_profile_section_stack.h:L40-L44`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RowType {
    Block,
    PlainSeparator,
    TextSeparator,
}

/// Upstream `SectionStack` visibility computation preventing duplicate or edge separators.
pub struct SectionStackOptimizer;

impl SectionStackOptimizer {
    /// Upstream `SectionStack::ComputeVisibility(kinds, intrinsic)` (`info_profile_section_stack.cpp:L150-L176`).
    pub fn compute_visibility(kinds: &[RowType], intrinsic: &[bool]) -> Vec<bool> {
        let count = kinds.len();
        let mut result = vec![false; count];
        for i in 0..count {
            if kinds[i] != RowType::PlainSeparator {
                result[i] = intrinsic.get(i).copied().unwrap_or(false);
            }
        }

        let block_visible_above = |separator: usize| -> bool {
            for j in (0..separator).rev() {
                match kinds[j] {
                    RowType::Block => {
                        if intrinsic.get(j).copied().unwrap_or(false) {
                            return true;
                        }
                    }
                    RowType::TextSeparator => {
                        if intrinsic.get(j).copied().unwrap_or(false) {
                            return false;
                        }
                    }
                    RowType::PlainSeparator => return false,
                }
            }
            false
        };

        let block_visible_below = |separator: usize| -> bool {
            for j in (separator + 1)..count {
                match kinds[j] {
                    RowType::Block => {
                        if intrinsic.get(j).copied().unwrap_or(false) {
                            return true;
                        }
                    }
                    RowType::TextSeparator => {
                        if intrinsic.get(j).copied().unwrap_or(false) {
                            return false;
                        }
                    }
                    RowType::PlainSeparator => return false,
                }
            }
            false
        };

        for i in 0..count {
            if kinds[i] == RowType::PlainSeparator {
                result[i] = block_visible_above(i) && block_visible_below(i);
            }
        }

        result
    }
}

/// Navigation stack coordinator managing hierarchical drill-down and separator deduplication in the profile drawer.
#[derive(Debug, Clone)]
pub struct InfoProfileSectionStackProfile {
    pub stack: Vec<ProfileSectionKind>,
    pub rows: Vec<(RowType, bool)>,
}

impl InfoProfileSectionStackProfile {
    pub fn new() -> Self {
        Self {
            stack: vec![ProfileSectionKind::Main],
            rows: Vec::new(),
        }
    }

    pub fn push(&mut self, section: ProfileSectionKind) {
        self.stack.push(section);
    }

    pub fn pop(&mut self) -> Option<ProfileSectionKind> {
        if self.stack.len() > 1 {
            self.stack.pop()
        } else {
            None
        }
    }

    pub fn current(&self) -> ProfileSectionKind {
        self.stack.last().copied().unwrap_or(ProfileSectionKind::Main)
    }

    pub fn add_block(&mut self, visible: bool) {
        self.rows.push((RowType::Block, visible));
    }

    pub fn add_plain_separator(&mut self) {
        self.rows.push((RowType::PlainSeparator, true));
    }

    pub fn add_text_separator(&mut self, visible: bool) {
        self.rows.push((RowType::TextSeparator, visible));
    }

    pub fn compute_row_visibility(&self) -> Vec<bool> {
        let kinds: Vec<RowType> = self.rows.iter().map(|(k, _)| *k).collect();
        let intrinsic: Vec<bool> = self.rows.iter().map(|(_, v)| *v).collect();
        SectionStackOptimizer::compute_visibility(&kinds, &intrinsic)
    }
}

impl Default for InfoProfileSectionStackProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InfoProfileSectionStackProfile {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let visibility = self.compute_row_visibility();
        let active_count = visibility.iter().filter(|&&v| v).count();

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(format!("Section depth: {}, Visible rows: {}", self.stack.len(), active_count))
    }
}
