//! Upstream Source: Telegram/SourceFiles/countries/countries_manager.cpp, countries_manager.h
//! Subsystem: countries | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use super::countries_instance::{CountriesInstance, CountryInfo};

pub const COUNTRIES_DATA_VERSION: i32 = 2;

#[derive(Debug, Clone, Default)]
pub struct FileData {
    pub hash: i32,
    pub infos: Vec<CountryInfo>,
}

#[derive(Debug, Clone)]
pub struct CountriesManager {
    instance: CountriesInstance,
    hash: i32,
    is_loading: bool,
    version: i32,
}

impl Default for CountriesManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CountriesManager {
    pub fn new() -> Self {
        Self {
            instance: CountriesInstance::new(),
            hash: 0,
            is_loading: false,
            version: COUNTRIES_DATA_VERSION,
        }
    }

    pub fn instance(&self) -> &CountriesInstance {
        &self.instance
    }

    pub fn instance_mut(&mut self) -> &mut CountriesInstance {
        &mut self.instance
    }

    pub fn hash(&self) -> i32 {
        self.hash
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn process_alternative_name(mut info: CountryInfo) -> CountryInfo {
        if info.name == "USA" {
            info.alternative_name = "United States of America".into();
        }
        info
    }

    pub fn set_country_list(&mut self, hash: i32, mut list: Vec<CountryInfo>) {
        self.hash = hash;
        for item in &mut list {
            if item.name == "USA" {
                item.alternative_name = "United States of America".into();
            }
        }
        self.instance.set_list(list);
        self.is_loading = false;
    }

    pub fn search(&self, query: &str) -> Vec<CountryInfo> {
        let q = query.trim().to_lowercase();
        if q.is_empty() {
            return self.instance.list().to_vec();
        }

        self.instance
            .list()
            .iter()
            .filter(|c| {
                c.name.to_lowercase().contains(&q)
                    || c.alternative_name.to_lowercase().contains(&q)
                    || c.iso2.to_lowercase().contains(&q)
                    || c.codes.iter().any(|code| code.calling_code.contains(&q))
            })
            .cloned()
            .collect()
    }

    pub fn serialize_data(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.version.to_le_bytes());
        buffer.extend_from_slice(&self.hash.to_le_bytes());
        let count = self.instance.list().len() as u32;
        buffer.extend_from_slice(&count.to_le_bytes());

        for item in self.instance.list() {
            let name_bytes = item.name.as_bytes();
            buffer.extend_from_slice(&(name_bytes.len() as u32).to_le_bytes());
            buffer.extend_from_slice(name_bytes);

            let iso_bytes = item.iso2.as_bytes();
            buffer.extend_from_slice(&(iso_bytes.len() as u32).to_le_bytes());
            buffer.extend_from_slice(iso_bytes);
        }
        buffer
    }

    pub fn deserialize_data(&mut self, data: &[u8]) -> bool {
        if data.len() < 12 {
            return false;
        }
        let ver = i32::from_le_bytes(data[0..4].try_into().unwrap_or_default());
        if ver != COUNTRIES_DATA_VERSION {
            return false;
        }
        let hash = i32::from_le_bytes(data[4..8].try_into().unwrap_or_default());
        let count = u32::from_le_bytes(data[8..12].try_into().unwrap_or_default()) as usize;

        self.hash = hash;
        let mut offset = 12;
        let mut loaded = Vec::with_capacity(count);

        for _ in 0..count {
            if offset + 4 > data.len() {
                break;
            }
            let name_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap_or_default()) as usize;
            offset += 4;
            if offset + name_len > data.len() {
                break;
            }
            let name = String::from_utf8_lossy(&data[offset..offset + name_len]).into_owned();
            offset += name_len;

            if offset + 4 > data.len() {
                break;
            }
            let iso_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap_or_default()) as usize;
            offset += 4;
            if offset + iso_len > data.len() {
                break;
            }
            let iso2 = String::from_utf8_lossy(&data[offset..offset + iso_len]).into_owned();
            offset += iso_len;

            loaded.push(Self::process_alternative_name(CountryInfo {
                name,
                iso2,
                alternative_name: String::new(),
                is_hidden: false,
                codes: Vec::new(),
            }));
        }

        if !loaded.is_empty() {
            self.instance.set_list(loaded);
            true
        } else {
            false
        }
    }
}
