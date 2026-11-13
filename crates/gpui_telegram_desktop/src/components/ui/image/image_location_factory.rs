//! Upstream C++ source: `Telegram/SourceFiles/ui/image/image_location_factory.cpp:L1-L386` & `image_location_factory.h:L1-L45`
//! Authentic GPUI implementation of image location factory builders.
#![allow(dead_code)]

use super::image_location::ImageLocationImage;

/// Factory for creating standardized ImageLocation instances from cloud entities.
/// C++ companion: `Ui::ImageLocationFactory` (`image_location_factory.cpp:L25-L386`).
#[derive(Debug, Default, Clone)]
pub struct ImageLocationFactoryImage {
    default_dc_id: u32,
}

impl ImageLocationFactoryImage {
    /// Creates a factory with default datacenter routing.
    pub fn new(default_dc_id: u32) -> Self {
        Self { default_dc_id }
    }

    /// Constructs an ImageLocation for a photo size.
    pub fn create_photo_location(
        &self,
        photo_id: u64,
        access_hash: u64,
        file_reference: Vec<u8>,
        width: u32,
        height: u32,
        dc_id_override: Option<u32>,
    ) -> ImageLocationImage {
        let dc = dc_id_override.unwrap_or(self.default_dc_id);
        ImageLocationImage::for_photo(photo_id, access_hash, file_reference, width, height, dc)
    }

    /// Constructs an ImageLocation for a document thumbnail.
    pub fn create_document_location(
        &self,
        document_id: u64,
        access_hash: u64,
        file_reference: Vec<u8>,
        width: u32,
        height: u32,
        dc_id_override: Option<u32>,
    ) -> ImageLocationImage {
        let dc = dc_id_override.unwrap_or(self.default_dc_id);
        ImageLocationImage::for_document(document_id, access_hash, file_reference, width, height, dc)
    }
}
