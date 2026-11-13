//! Info Profile Values component
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_values.cpp:L1-L843`
//! Upstream: `Telegram/SourceFiles/info/profile/info_profile_values.h:L1-L158`
//! Subsystem: info | Feature ID: TCG-N22 | Disposition: gpui_ui_1to1

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelLocationInfo {
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BirthdayInfo {
    pub day: u8,
    pub month: u8,
    pub year: Option<i32>,
}

/// Reactive value extractor and provider for dynamic profile metrics
/// (members count, online count, mutual contacts, birthday, and location).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoProfileValuesProfile {
    pub peer_id: i64,
    pub members_count: Option<usize>,
    pub online_count: Option<usize>,
    pub common_groups_count: usize,
    pub location: Option<ChannelLocationInfo>,
    pub birthday: Option<BirthdayInfo>,
}

impl InfoProfileValuesProfile {
    pub fn new(peer_id: i64) -> Self {
        Self {
            peer_id,
            members_count: None,
            online_count: None,
            common_groups_count: 0,
            location: None,
            birthday: None,
        }
    }

    pub fn set_members(&mut self, members: usize, online: usize) {
        self.members_count = Some(members);
        self.online_count = Some(online);
    }

    pub fn set_common_groups(&mut self, count: usize) {
        self.common_groups_count = count;
    }

    pub fn set_location(&mut self, address: impl Into<String>, lat: f64, lon: f64) {
        self.location = Some(ChannelLocationInfo {
            address: address.into(),
            latitude: lat,
            longitude: lon,
        });
    }

    pub fn set_birthday(&mut self, day: u8, month: u8, year: Option<i32>) {
        self.birthday = Some(BirthdayInfo { day, month, year });
    }
}

impl Default for InfoProfileValuesProfile {
    fn default() -> Self {
        Self::new(0)
    }
}
