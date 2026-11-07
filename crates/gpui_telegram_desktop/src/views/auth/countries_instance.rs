//! Upstream Source: Telegram/SourceFiles/countries/countries_instance.cpp, countries_instance.h
//! Subsystem: countries | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CallingCodeInfo {
    pub calling_code: String,
    pub prefixes: Vec<String>,
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CountryInfo {
    pub name: String,
    pub iso2: String,
    pub alternative_name: String,
    pub is_hidden: bool,
    pub codes: Vec<CallingCodeInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FormatResult {
    pub formatted: String,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FormatArgs {
    pub phone: String,
    pub only_code: bool,
}

#[derive(Debug, Clone)]
pub struct CountriesInstance {
    countries: Vec<CountryInfo>,
    by_code_map: HashMap<String, usize>,
    by_iso2_map: HashMap<String, usize>,
}

impl Default for CountriesInstance {
    fn default() -> Self {
        Self::new()
    }
}

impl CountriesInstance {
    pub fn new() -> Self {
        let default_countries = vec![
            CountryInfo {
                name: "United States".into(),
                iso2: "US".into(),
                alternative_name: "United States of America".into(),
                is_hidden: false,
                codes: vec![CallingCodeInfo {
                    calling_code: "1".into(),
                    prefixes: vec![],
                    patterns: vec!["(XXX) XXX-XXXX".into()],
                }],
            },
            CountryInfo {
                name: "United Kingdom".into(),
                iso2: "GB".into(),
                alternative_name: "Great Britain".into(),
                is_hidden: false,
                codes: vec![CallingCodeInfo {
                    calling_code: "44".into(),
                    prefixes: vec![],
                    patterns: vec!["XXXX XXXXXX".into()],
                }],
            },
            CountryInfo {
                name: "Germany".into(),
                iso2: "DE".into(),
                alternative_name: "Deutschland".into(),
                is_hidden: false,
                codes: vec![CallingCodeInfo {
                    calling_code: "49".into(),
                    prefixes: vec![],
                    patterns: vec!["XXX XXXXXXXX".into()],
                }],
            },
            CountryInfo {
                name: "France".into(),
                iso2: "FR".into(),
                alternative_name: "".into(),
                is_hidden: false,
                codes: vec![CallingCodeInfo {
                    calling_code: "33".into(),
                    prefixes: vec![],
                    patterns: vec!["X XX XX XX XX".into()],
                }],
            },
            CountryInfo {
                name: "Japan".into(),
                iso2: "JP".into(),
                alternative_name: "Nippon".into(),
                is_hidden: false,
                codes: vec![CallingCodeInfo {
                    calling_code: "81".into(),
                    prefixes: vec![],
                    patterns: vec!["XX XXXX XXXX".into()],
                }],
            },
            CountryInfo {
                name: "China".into(),
                iso2: "CN".into(),
                alternative_name: "PRC".into(),
                is_hidden: false,
                codes: vec![CallingCodeInfo {
                    calling_code: "86".into(),
                    prefixes: vec![],
                    patterns: vec!["XXX XXXX XXXX".into()],
                }],
            },
        ];

        let mut instance = Self {
            countries: Vec::new(),
            by_code_map: HashMap::new(),
            by_iso2_map: HashMap::new(),
        };
        instance.set_list(default_countries);
        instance
    }

    pub fn list(&self) -> &[CountryInfo] {
        &self.countries
    }

    pub fn set_list(&mut self, list: Vec<CountryInfo>) {
        self.countries = list;
        self.by_code_map.clear();
        self.by_iso2_map.clear();

        for (idx, country) in self.countries.iter().enumerate() {
            self.by_iso2_map.insert(country.iso2.to_uppercase(), idx);
            for code in &country.codes {
                self.by_code_map.entry(code.calling_code.clone()).or_insert(idx);
            }
        }
    }

    pub fn by_code(&self, code: &str) -> Option<&CountryInfo> {
        let clean = code.trim_start_matches('+');
        self.by_code_map.get(clean).map(|&idx| &self.countries[idx])
    }

    pub fn by_iso2(&self, iso2: &str) -> Option<&CountryInfo> {
        self.by_iso2_map.get(&iso2.to_uppercase()).map(|&idx| &self.countries[idx])
    }

    pub fn country_name_by_iso2(&self, iso2: &str) -> String {
        self.by_iso2(iso2).map(|c| c.name.clone()).unwrap_or_default()
    }

    pub fn country_iso2_by_phone(&self, phone: &str) -> String {
        let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
        for len in (1..=4).rev() {
            if digits.len() >= len {
                let prefix = &digits[..len];
                if let Some(c) = self.by_code(prefix) {
                    return c.iso2.clone();
                }
            }
        }
        String::new()
    }

    pub fn flag_emoji_by_iso2(iso: &str) -> String {
        if iso.len() != 2 || !iso.chars().all(|c| c.is_ascii_uppercase()) {
            return String::new();
        }
        if iso == "FT" {
            return "\u{1F3F4}\u{200D}\u{2620}\u{FE0F}".to_string();
        }
        let mut result = String::with_capacity(8);
        for c in iso.chars() {
            if let Some(ch) = char::from_u32(0x1F1E6 + (c as u32 - 'A' as u32)) {
                result.push(ch);
            }
        }
        result
    }

    pub fn extract_phone_code(&self, phone: &str) -> String {
        self.format(FormatArgs {
            phone: phone.to_string(),
            only_code: true,
        }).code
    }

    pub fn format(&self, args: FormatArgs) -> FormatResult {
        let digits: String = args.phone.chars().filter(|c| c.is_ascii_digit()).collect();
        let mut matched_code = String::new();

        for len in (1..=4).rev() {
            if digits.len() >= len {
                let prefix = &digits[..len];
                if self.by_code(prefix).is_some() {
                    matched_code = prefix.to_string();
                    break;
                }
            }
        }

        if args.only_code {
            return FormatResult {
                formatted: format!("+{}", matched_code),
                code: matched_code,
            };
        }

        let body = if digits.len() >= matched_code.len() {
            &digits[matched_code.len()..]
        } else {
            ""
        };

        let formatted = if matched_code.is_empty() {
            format!("+{}", digits)
        } else if body.is_empty() {
            format!("+{}", matched_code)
        } else {
            format!("+{} {}", matched_code, body)
        };

        FormatResult {
            formatted,
            code: matched_code,
        }
    }
}
