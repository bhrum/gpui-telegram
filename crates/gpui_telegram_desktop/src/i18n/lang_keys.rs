//! Target Symbol: LangKeys
//! Telegram Desktop 1:1 Submodule: `LangKeys`
//! Subsystem: `lang`, Feature ID: `TCG-N44`
//! Disposition: `gpui_ui_1to1`
//! Upstream Source: `Telegram/SourceFiles/lang/lang_keys.h`, `lang_keys.cpp`
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use chrono::{DateTime, Datelike, NaiveDate, Timelike, Utc};

pub const K_DEFAULT_LANGUAGE: &str = "en";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LangKeys;

impl LangKeys {
    /// Determines whether the locale conventions place the first name after the last name (e.g. Eastern name order).
    /// Upstream: `langFirstNameGoesSecond()`
    pub fn first_name_goes_second(lang_code: &str) -> bool {
        let base = lang_code.split('-').next().unwrap_or(lang_code).to_ascii_lowercase();
        matches!(base.as_str(), "zh" | "ja" | "ko" | "vi" | "hu")
    }

    /// Formats person full name adhering to locale order conventions.
    /// Upstream: `langFullName(firstName, lastName)`
    pub fn full_name(first_name: &str, last_name: &str, lang_code: &str) -> String {
        let first = first_name.trim();
        let last = last_name.trim();

        if first.is_empty() {
            last.to_string()
        } else if last.is_empty() {
            first.to_string()
        } else if Self::first_name_goes_second(lang_code) {
            format!("{last} {first}")
        } else {
            format!("{first} {last}")
        }
    }

    /// Short month name lookup (1..=12).
    /// Upstream: `MonthSmall(month)`
    pub fn month_small(month: u32) -> &'static str {
        match month {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "",
        }
    }

    /// Full month name lookup (1..=12).
    /// Upstream: `Month(month)`
    pub fn month_full(month: u32) -> &'static str {
        match month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "",
        }
    }

    /// Weekday short name lookup (1..=7, 1=Monday).
    /// Upstream: `Weekday(index)`
    pub fn weekday_short(weekday: u32) -> &'static str {
        match weekday {
            1 => "Mon",
            2 => "Tue",
            3 => "Wed",
            4 => "Thu",
            5 => "Fri",
            6 => "Sat",
            7 => "Sun",
            _ => "",
        }
    }

    /// Weekday full name lookup (1..=7, 1=Monday).
    /// Upstream: `WeekdayFull(index)`
    pub fn weekday_full(weekday: u32) -> &'static str {
        match weekday {
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
            7 => "Sunday",
            _ => "",
        }
    }

    /// Formats day and month (e.g. "Apr 15" or "Apr 15, 2024" if year differs).
    /// Upstream: `langDayOfMonth(date)`
    pub fn day_of_month(date: NaiveDate, current_date: NaiveDate) -> String {
        let m = Self::month_small(date.month());
        let d = date.day();
        let y = date.year();

        if y != current_date.year() {
            format!("{m} {d}, {y}")
        } else {
            format!("{m} {d}")
        }
    }

    /// Formats day and full month name (e.g. "April 15, 2026").
    /// Upstream: `langDayOfMonthFull(date)`
    pub fn day_of_month_full(date: NaiveDate) -> String {
        let m = Self::month_full(date.month());
        let d = date.day();
        let y = date.year();
        format!("{m} {d}, {y}")
    }

    /// Formats month and year (e.g. "April 2026").
    /// Upstream: `langMonthOfYear(month, year)`
    pub fn month_of_year(month: u32, year: i32) -> String {
        let m = Self::month_full(month);
        format!("{m} {year}")
    }

    /// Formats full date and time string.
    /// Upstream: `langDateTime(date)`
    pub fn date_time(date: DateTime<Utc>, current_date: NaiveDate) -> String {
        let naive = date.date_naive();
        let date_part = Self::day_of_month(naive, current_date);
        let time_part = format!("{:02}:{:02}", date.hour(), date.minute());
        format!("{date_part} at {time_part}")
    }
}
