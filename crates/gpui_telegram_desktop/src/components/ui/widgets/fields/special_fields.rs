//! Authentic GPUI Specialized Input Fields: Country Code, Phone Number, and Username.
//! Reference: Telegram/SourceFiles/ui/widgets/fields/special_fields.cpp:L18-L60
//! Reference: Telegram/SourceFiles/ui/widgets/fields/special_fields.h:L20-L147

#![allow(dead_code, unused_imports, unused_variables)]

/// Maximum allowed length of a Telegram public @username: 32 chars.
/// Reference: `fields/special_fields.cpp:L19`
pub const K_MAX_USERNAME_LENGTH: usize = 32;

/// Maximum length of phone number tail (without country code): 32 chars.
/// Reference: `fields/special_fields.cpp:L23`
pub const K_MAX_PHONE_TAIL_LENGTH: usize = 32;

/// Maximum length of international country phone code digits: 4 chars.
/// Reference: `fields/special_fields.cpp:L26`
pub const K_MAX_PHONE_CODE_LENGTH: usize = 4;

/// Target symbol managing specialized phone number, country code, and username inputs.
#[derive(Debug, Clone)]
pub struct SpecialFieldsWidgetsFields {
    pub country_code: String,
    pub phone_tail: String,
    pub username: String,
    pub is_country_code_valid: bool,
    pub is_username_valid: bool,
}

impl Default for SpecialFieldsWidgetsFields {
    fn default() -> Self {
        Self::new()
    }
}

impl SpecialFieldsWidgetsFields {
    pub fn new() -> Self {
        Self {
            country_code: "+1".to_string(),
            phone_tail: String::new(),
            username: String::new(),
            is_country_code_valid: true,
            is_username_valid: true,
        }
    }

    /// Sets and sanitizes country phone code with `+` prefix:
    /// Reference: `fields/special_fields.cpp:L43-L53`
    pub fn set_country_code(&mut self, code: &str) {
        let digits: String = code.chars().filter(|c| c.is_ascii_digit()).take(K_MAX_PHONE_CODE_LENGTH).collect();
        self.is_country_code_valid = !digits.is_empty();
        self.country_code = format!("+{digits}");
    }

    /// Sets and sanitizes phone tail digits:
    pub fn set_phone_tail(&mut self, tail: &str) {
        self.phone_tail = tail.chars().filter(|c| c.is_ascii_digit()).take(K_MAX_PHONE_TAIL_LENGTH).collect();
    }

    /// Returns the full E.164 formatted international telephone number.
    pub fn full_phone_number(&self) -> String {
        format!("{}{}", self.country_code, self.phone_tail)
    }

    /// Validates username against Telegram username specification:
    /// 5-32 characters, [a-zA-Z0-9_], cannot start with a digit.
    pub fn set_username(&mut self, name: &str) -> bool {
        let clean = name.trim_start_matches('@');
        let len_ok = clean.len() >= 5 && clean.len() <= K_MAX_USERNAME_LENGTH;
        let chars_ok = clean.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
        let start_ok = clean.chars().next().map_or(false, |c| !c.is_ascii_digit());

        self.username = clean.to_string();
        self.is_username_valid = len_ok && chars_ok && start_ok;
        self.is_username_valid
    }
}
