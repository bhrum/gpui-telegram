//! Upstream C++ source: `Telegram/SourceFiles/ui/image/image_location.cpp:L20-L1070` & `image_location.h:L1-L150`
//! Authentic GPUI implementation of image locations, storage keys, and cache tags.
#![allow(dead_code)]

/// Document cache bitmask tags from `image_location.cpp:L22-L25`.
pub const K_DOCUMENT_BASE_CACHE_TAG: u64 = 0x0000_0000_0001_0000;
pub const K_DOCUMENT_BASE_CACHE_MASK: u64 = 0x0000_0000_0000_FF00;
pub const K_PHOTO_BASE_CACHE_TAG: u64 = 0x0000_0000_0002_0000;
pub const K_PHOTO_BASE_CACHE_MASK: u64 = 0x0000_0000_0000_FF00;

/// Serialization flags from `image_location.cpp:L27-L30`.
pub const K_NON_STORAGE_LOCATION_TOKEN: u8 = 0x10;
pub const K_LEGACY_IN_MESSAGE_PEER_ID_FLAG: u8 = 0x08;
pub const K_MODERN_LOCATION_FLAG: u8 = 0x20;
pub const K_IN_MESSAGE_FIELDS_FLAG: u8 = 0x40;

/// Location type indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageLocationType {
    Null,
    Photo,
    Document,
    WebFile,
    LocalFile,
}

/// Identifies an image location in storage or cloud cache.
/// C++ companion: `StorageImageLocation` (`image_location.cpp:L35-L300`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageLocationImage {
    location_type: ImageLocationType,
    id: u64,
    access_hash: u64,
    file_reference: Vec<u8>,
    width: u32,
    height: u32,
    dc_id: u32,
}

impl ImageLocationImage {
    /// Creates a null location.
    pub fn null() -> Self {
        Self {
            location_type: ImageLocationType::Null,
            id: 0,
            access_hash: 0,
            file_reference: Vec::new(),
            width: 0,
            height: 0,
            dc_id: 0,
        }
    }

    /// Creates a photo location with specific DC and access hash.
    pub fn for_photo(id: u64, access_hash: u64, file_reference: Vec<u8>, width: u32, height: u32, dc_id: u32) -> Self {
        Self {
            location_type: ImageLocationType::Photo,
            id,
            access_hash,
            file_reference,
            width,
            height,
            dc_id,
        }
    }

    /// Creates a document thumbnail location.
    pub fn for_document(id: u64, access_hash: u64, file_reference: Vec<u8>, width: u32, height: u32, dc_id: u32) -> Self {
        Self {
            location_type: ImageLocationType::Document,
            id,
            access_hash,
            file_reference,
            width,
            height,
            dc_id,
        }
    }

    /// Computes the 64-bit cache key tag according to `image_location.cpp:L120-L160`.
    pub fn cache_key_tag(&self) -> u64 {
        match self.location_type {
            ImageLocationType::Photo => K_PHOTO_BASE_CACHE_TAG | (self.id & !K_PHOTO_BASE_CACHE_MASK),
            ImageLocationType::Document => K_DOCUMENT_BASE_CACHE_TAG | (self.id & !K_DOCUMENT_BASE_CACHE_MASK),
            _ => self.id,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.location_type != ImageLocationType::Null && self.id != 0
    }

    pub fn dc_id(&self) -> u32 {
        self.dc_id
    }
}

impl Default for ImageLocationImage {
    fn default() -> Self {
        Self::null()
    }
}
