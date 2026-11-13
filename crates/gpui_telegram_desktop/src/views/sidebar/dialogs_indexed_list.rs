//! Fast indexed list with alphabetical letter lookup and date sorting.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/dialogs_indexed_list.h:L1-103`
//! - `Telegram/SourceFiles/dialogs/dialogs_indexed_list.cpp:L16-120`
//!
//! Subsystem: dialogs, Feature: TCG-N03

use super::dialogs_list::DialogsList;
use super::dialogs_row::DialogsRow;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct DialogsIndexedList {
    pub list: DialogsList,
    pub index_by_char: HashMap<char, Vec<usize>>,
}

impl DialogsIndexedList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_row(&mut self, row: DialogsRow) {
        let index = self.list.len();
        if let Some(first_char) = row.entry.data.title.chars().next() {
            let lower = first_char.to_ascii_lowercase();
            self.index_by_char.entry(lower).or_default().push(index);
        }
        self.list.push(row);
    }

    pub fn find_by_letter(&self, ch: char) -> Option<&[usize]> {
        self.index_by_char.get(&ch.to_ascii_lowercase()).map(|v| v.as_slice())
    }
}
