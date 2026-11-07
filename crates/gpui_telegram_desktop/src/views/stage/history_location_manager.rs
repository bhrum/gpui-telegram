//! History Location Manager & Click Handler.
//!
//! Citations:
//! - Upstream: `Telegram/SourceFiles/history/history_location_manager.h:L12-L44`
//! - Upstream: `Telegram/SourceFiles/history/history_location_manager.cpp:L17-L42`
//! - Anonymous/Implementation Constants:
//!   - `LocationClickHandler::Url`: `https://maps.google.com/maps?q={lat},{lon}&ll={lat},{lon}&z=16`
//!   - Default zoom level: `z=16` (history_location_manager.cpp:L41)

use gpui::*;

/// Geographic coordinates for a location message attachment.
/// C++ equivalent: `Data::LocationPoint` (data_location.h).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocationPoint {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy_radius_meters: Option<f64>,
}

impl LocationPoint {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            accuracy_radius_meters: None,
        }
    }

    pub fn with_accuracy(mut self, radius: f64) -> Self {
        self.accuracy_radius_meters = Some(radius);
        self
    }

    /// Formats latitude as string for query URLs.
    pub fn lat_as_string(&self) -> String {
        format!("{:.6}", self.latitude)
    }

    /// Formats longitude as string for query URLs.
    pub fn lon_as_string(&self) -> String {
        format!("{:.6}", self.longitude)
    }
}

/// Handler for location clicks in chat history.
/// C++ equivalent: `LocationClickHandler` (history_location_manager.cpp:L25-L42).
#[derive(Debug, Clone)]
pub struct LocationClickHandler {
    point: LocationPoint,
    url: String,
}

impl LocationClickHandler {
    pub fn new(point: LocationPoint) -> Self {
        let url = Self::url(&point);
        Self { point, url }
    }

    /// Generates Google Maps navigation URL with zoom=16.
    /// C++ citation: `history_location_manager.cpp:L35-L42`.
    pub fn url(point: &LocationPoint) -> String {
        let lat = point.lat_as_string();
        let lon = point.lon_as_string();
        format!("https://maps.google.com/maps?q={},{}&ll={},{}&z=16", lat, lon, lat, lon)
    }

    pub fn point(&self) -> LocationPoint {
        self.point
    }

    pub fn url_str(&self) -> &str {
        &self.url
    }

    /// Returns text formatted for system clipboard copy.
    /// C++ citation: `history_location_manager.cpp:L17-L19`.
    pub fn copy_to_clipboard_text(&self) -> &str {
        &self.url
    }

    /// Context menu item label.
    /// C++ citation: `history_location_manager.cpp:L21-L23` (`lng_context_copy_link`).
    pub fn copy_to_clipboard_context_item_text(&self) -> &'static str {
        "Copy Link"
    }
}

/// History location manager maintaining active geo coordinates and previews.
/// C++ equivalent: `LocationClickHandler` owner and coordinate cache.
pub struct HistoryLocationManager {
    current_point: Option<LocationPoint>,
    venue_title: Option<String>,
    venue_address: Option<String>,
    is_live: bool,
    live_expires_at: Option<u64>,
}

impl HistoryLocationManager {
    pub fn new() -> Self {
        Self {
            current_point: None,
            venue_title: None,
            venue_address: None,
            is_live: false,
            live_expires_at: None,
        }
    }

    pub fn set_point(&mut self, point: LocationPoint) {
        self.current_point = Some(point);
    }

    pub fn set_venue(&mut self, title: impl Into<String>, address: impl Into<String>) {
        self.venue_title = Some(title.into());
        self.venue_address = Some(address.into());
    }

    pub fn set_live(&mut self, expires_at: u64) {
        self.is_live = true;
        self.live_expires_at = Some(expires_at);
    }

    pub fn current_point(&self) -> Option<LocationPoint> {
        self.current_point
    }

    pub fn map_url(&self) -> Option<String> {
        self.current_point.map(|pt| LocationClickHandler::url(&pt))
    }
}

impl Default for HistoryLocationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for HistoryLocationManager {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let (lat_str, lon_str) = match self.current_point {
            Some(pt) => (pt.lat_as_string(), pt.lon_as_string()),
            None => ("0.0".to_string(), "0.0".to_string()),
        };

        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .p(px(8.0))
            .rounded(px(6.0))
            .bg(rgb(0xF4F4F5))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(13.0))
                    .text_color(rgb(0x222222))
                    .child(self.venue_title.clone().unwrap_or_else(|| "Location Point".to_string()))
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x71717A))
                    .child(self.venue_address.clone().unwrap_or_else(|| format!("Lat: {}, Lon: {}", lat_str, lon_str)))
            )
    }
}
