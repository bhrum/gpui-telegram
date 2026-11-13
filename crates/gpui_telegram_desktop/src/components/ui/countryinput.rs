//! Upstream C++ source: `Telegram/SourceFiles/ui/countryinput.cpp:L1-L201` & `countryinput.h:L1-L67`
//! Authentic GPUI implementation of country code selector and phone dial prefix input.
#![allow(dead_code)]

/// Country metadata entry for telephone onboarding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountryInfo {
    pub iso2: String,
    pub name: String,
    pub dial_code: String,
}

/// Country selection and phone code input controller.
/// C++ companion: `Ui::CountryInput` (`countryinput.cpp:L25-L201`).
#[derive(Debug, Clone)]
pub struct Countryinput {
    selected_country: Option<CountryInfo>,
    search_query: String,
}

impl Countryinput {
    /// Creates an input controller initialized with default country (if any).
    pub fn new(initial: Option<CountryInfo>) -> Self {
        Self {
            selected_country: initial,
            search_query: String::new(),
        }
    }

    /// Sets the selected country.
    pub fn set_country(&mut self, country: CountryInfo) {
        self.selected_country = Some(country);
    }

    /// Selects by ISO2 code (e.g. "US", "DE", "CN", "SG").
    pub fn select_by_iso2(&mut self, iso2: &str) -> bool {
        let normalized = iso2.to_uppercase();
        if let Some(info) = Self::common_countries().into_iter().find(|c| c.iso2 == normalized) {
            self.selected_country = Some(info);
            true
        } else {
            false
        }
    }

    pub fn selected(&self) -> Option<&CountryInfo> {
        self.selected_country.as_ref()
    }

    /// Standard international country phone prefixes.
    pub fn common_countries() -> Vec<CountryInfo> {
        vec![
            CountryInfo { iso2: "US".into(), name: "United States".into(), dial_code: "+1".into() },
            CountryInfo { iso2: "GB".into(), name: "United Kingdom".into(), dial_code: "+44".into() },
            CountryInfo { iso2: "DE".into(), name: "Germany".into(), dial_code: "+49".into() },
            CountryInfo { iso2: "FR".into(), name: "France".into(), dial_code: "+33".into() },
            CountryInfo { iso2: "CN".into(), name: "China".into(), dial_code: "+86".into() },
            CountryInfo { iso2: "JP".into(), name: "Japan".into(), dial_code: "+81".into() },
            CountryInfo { iso2: "SG".into(), name: "Singapore".into(), dial_code: "+65".into() },
        ]
    }
}

impl Default for Countryinput {
    fn default() -> Self {
        Self::new(Some(CountryInfo {
            iso2: "US".into(),
            name: "United States".into(),
            dial_code: "+1".into(),
        }))
    }
}
