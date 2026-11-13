//! Ordered linear list of dialog rows.
//!
//! Upstream: `Telegram/SourceFiles/dialogs/dialogs_list.h:L1-105`
//! Subsystem: dialogs, Feature: TCG-N03

use super::dialogs_row::DialogsRow;

#[derive(Debug, Clone, Default)]
pub struct DialogsList {
    pub rows: Vec<DialogsRow>,
}

impl DialogsList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn push(&mut self, row: DialogsRow) {
        self.rows.push(row);
    }
}
