#![recursion_limit = "8192"]

pub mod platform;
pub mod app;
pub mod components;
pub mod domain;
pub mod theme;
pub mod ui;
pub mod views;

pub use app::TelegramApp;
pub use domain::*;

pub mod i18n;
pub use i18n::*;

pub mod layout;
pub use layout::*;

pub mod window;
pub use window::*;
