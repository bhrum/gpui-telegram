//! Layout cache for Instant View
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct BlockGeometry {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Default)]
pub struct IvLayoutCache {
    pub cached_geometries: HashMap<usize, BlockGeometry>,
    pub total_document_height: f32,
    pub is_valid: bool,
}

impl IvLayoutCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn invalidate(&mut self) {
        self.cached_geometries.clear();
        self.total_document_height = 0.0;
        self.is_valid = false;
    }

    pub fn insert_block_geometry(&mut self, block_index: usize, geom: BlockGeometry) {
        let bottom = geom.y + geom.height;
        if bottom > self.total_document_height {
            self.total_document_height = bottom;
        }
        self.cached_geometries.insert(block_index, geom);
        self.is_valid = true;
    }

    pub fn get_block_geometry(&self, block_index: usize) -> Option<&BlockGeometry> {
        self.cached_geometries.get(&block_index)
    }
}
