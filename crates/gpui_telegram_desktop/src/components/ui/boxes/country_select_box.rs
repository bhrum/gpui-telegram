//! Authentic GPUI Country & Calling Code Selector Modal.
//! Reference: Telegram/SourceFiles/ui/boxes/country_select_box.cpp:L35-L786
//! Reference: Telegram/SourceFiles/ui/boxes/country_select_box.h:L20-L59
//! Reference: Telegram/SourceFiles/ui/boxes/country_select_box.style:L1-L23

#![allow(dead_code, unused_imports, unused_variables)]

use gpui::*;

/// Height of an individual country row: 36px.
/// Reference: `country_select_box.style:L14`
pub const COUNTRY_ROW_HEIGHT: f32 = 36.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountrySelectType {
    Phones,
    Countries,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountryEntry {
    pub country: String,
    pub iso2: String,
    pub code: String,
    pub alternative_name: String,
}

/// Target symbol for country selection dialog.
/// Reference: `country_select_box.h:L25-L55`
#[derive(Debug, Clone)]
pub struct CountrySelectBoxBoxes {
    pub select_type: CountrySelectType,
    pub entries: Vec<CountryEntry>,
    pub filter_query: String,
    pub selected_index: usize,
    pub chosen_entry: Option<CountryEntry>,
}

impl Default for CountrySelectBoxBoxes {
    fn default() -> Self {
        Self::new(CountrySelectType::Phones)
    }
}

impl CountrySelectBoxBoxes {
    pub fn new(select_type: CountrySelectType) -> Self {
        let defaults = vec![
            CountryEntry {
                country: "United States".to_string(),
                iso2: "US".to_string(),
                code: "+1".to_string(),
                alternative_name: "USA".to_string(),
            },
            CountryEntry {
                country: "United Kingdom".to_string(),
                iso2: "GB".to_string(),
                code: "+44".to_string(),
                alternative_name: "UK".to_string(),
            },
            CountryEntry {
                country: "Germany".to_string(),
                iso2: "DE".to_string(),
                code: "+49".to_string(),
                alternative_name: "Deutschland".to_string(),
            },
            CountryEntry {
                country: "China".to_string(),
                iso2: "CN".to_string(),
                code: "+86".to_string(),
                alternative_name: "Zhongguo".to_string(),
            },
            CountryEntry {
                country: "Japan".to_string(),
                iso2: "JP".to_string(),
                code: "+81".to_string(),
                alternative_name: "Nippon".to_string(),
            },
        ];
        Self {
            select_type,
            entries: defaults,
            filter_query: String::new(),
            selected_index: 0,
            chosen_entry: None,
        }
    }

    /// Filters entries by matching name, ISO code, calling code, or alternate name.
    /// Reference: `country_select_box.cpp:L250-L280`
    pub fn filtered_entries(&self) -> Vec<&CountryEntry> {
        let q = self.filter_query.trim().to_lowercase();
        if q.is_empty() {
            self.entries.iter().collect()
        } else {
            self.entries
                .iter()
                .filter(|e| {
                    e.country.to_lowercase().contains(&q)
                        || e.iso2.to_lowercase().contains(&q)
                        || e.code.contains(&q)
                        || e.alternative_name.to_lowercase().contains(&q)
                })
                .collect()
        }
    }

    pub fn select_entry(&mut self, entry: CountryEntry) {
        self.chosen_entry = Some(entry);
    }
}
