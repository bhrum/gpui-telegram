//! Authentic GPUI Time Part Input Field Component with Dynamic Localized Placeholder.
//! Reference: Telegram/SourceFiles/ui/widgets/fields/time_part_input_with_placeholder.cpp:L15-L60
//! Reference: Telegram/SourceFiles/ui/widgets/fields/time_part_input_with_placeholder.h:L18-L40

#![allow(dead_code, unused_imports, unused_variables)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimePartKind {
    Hours,
    #[default]
    Minutes,
    Seconds,
}

/// Target symbol for formatted numeric time-part input field (e.g. self-destruct TTL hours/minutes).
#[derive(Debug, Clone)]
pub struct TimePartInputWithPlaceholderWidgetsFields {
    pub kind: TimePartKind,
    pub value: u32,
    pub max_value: u32,
    pub placeholder_suffix: String,
    pub field_width: f32,
    pub text_margins_left: f32,
}

impl Default for TimePartInputWithPlaceholderWidgetsFields {
    fn default() -> Self {
        Self::new(TimePartKind::Minutes)
    }
}

impl TimePartInputWithPlaceholderWidgetsFields {
    pub fn new(kind: TimePartKind) -> Self {
        let (max_val, suffix) = match kind {
            TimePartKind::Hours => (24, "hours"),
            TimePartKind::Minutes => (60, "minutes"),
            TimePartKind::Seconds => (60, "seconds"),
        };
        let mut field = Self {
            kind,
            value: 0,
            max_value: max_val,
            placeholder_suffix: suffix.to_string(),
            field_width: 120.0,
            text_margins_left: 0.0,
        };
        field.update_margins();
        field
    }

    pub fn set_value(&mut self, val: u32) {
        self.value = val.min(self.max_value);
        self.update_margins();
    }

    pub fn set_field_width(&mut self, width: f32) {
        self.field_width = width;
        self.update_margins();
    }

    /// Formats value string with zero padding if under 10.
    pub fn formatted_text(&self) -> String {
        format!("{:02}", self.value)
    }

    /// Computes centered layout margins with placeholder offset:
    /// `leftMargins = (width - placeholder.width) / 2 + placeholder.leftOffset`.
    /// Reference: `fields/time_part_input_with_placeholder.cpp:L57-L59`
    pub fn update_margins(&mut self) {
        let val_w = self.formatted_text().len() as f32 * 8.0;
        let suffix_w = self.placeholder_suffix.len() as f32 * 8.0;
        let total_w = val_w + suffix_w + 4.0;
        self.text_margins_left = ((self.field_width - total_w) / 2.0).max(0.0);
    }
}
