//! Target Symbol: IvMarkdownMediaReuseMarkdown
//! Subsystem: iv, Feature: TCG-N42
//! Upstream: Telegram/SourceFiles/iv/markdown/iv_markdown_media_reuse.h/.cpp
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PreparedMediaItemKind {
    #[default]
    Photo,
    Video,
    Document,
    Audio,
    Sticker,
    Location,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PreparedGroupedMediaIntent {
    #[default]
    Collage,
    Slideshow,
    Isolated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PreparedBlockKind {
    #[default]
    Paragraph,
    Heading,
    List,
    Table,
    Media,
    Code,
    Divider,
    Details,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct MediaReuseGroupedItem {
    pub kind: PreparedMediaItemKind,
    pub backing_id: u64,
    pub width: i32,
    pub height: i32,
    pub spoiler: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MediaReuseKey {
    pub kind: PreparedBlockKind,
    pub media_kind: PreparedMediaItemKind,
    pub grouped_intent: PreparedGroupedMediaIntent,
    pub backing_id: u64,
    pub access_hash: u64,
    pub width: i32,
    pub height: i32,
    pub zoom: i32,
    pub duration: i32,
    pub latitude: f64,
}

impl Eq for MediaReuseKey {}

impl Hash for MediaReuseKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.media_kind.hash(state);
        self.grouped_intent.hash(state);
        self.backing_id.hash(state);
        self.access_hash.hash(state);
        self.width.hash(state);
        self.height.hash(state);
        self.zoom.hash(state);
        self.duration.hash(state);
        self.latitude.to_bits().hash(state);
    }
}

pub fn media_reuse_hash_combine(seed: &mut usize, value: usize) {
    *seed ^= value
        .wrapping_add(0x9e3779b9)
        .wrapping_add(*seed << 6)
        .wrapping_add(*seed >> 2);
}

#[derive(Debug, Clone, Default)]
pub struct MediaReuseKeyHasher;

impl MediaReuseKeyHasher {
    pub fn compute_hash(key: &MediaReuseKey) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize
    }
}

#[derive(Debug, Clone, Default)]
pub struct CachedMediaBlockEntry {
    pub block_id: u64,
    pub key: MediaReuseKey,
    pub texture_loaded: bool,
    pub view_count: usize,
}

#[derive(Debug, Clone, Default)]
pub struct MediaBlockStorage {
    pub blocks: HashMap<u64, CachedMediaBlockEntry>,
}

impl MediaBlockStorage {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.blocks.clear();
    }
}

pub fn clear_media_block_storage(blocks: &mut MediaBlockStorage) {
    blocks.clear();
}

#[derive(Debug, Clone, Default)]
pub struct MediaBlockReusePool {
    pub pool: HashMap<MediaReuseKey, Vec<u64>>,
    pub reuse_hits: usize,
    pub reuse_misses: usize,
}

impl MediaBlockReusePool {
    pub fn new() -> Self {
        Self {
            pool: HashMap::new(),
            reuse_hits: 0,
            reuse_misses: 0,
        }
    }

    pub fn clear(&mut self) {
        self.pool.clear();
    }

    pub fn insert(&mut self, key: MediaReuseKey, id: u64) {
        self.pool.entry(key).or_default().push(id);
    }

    pub fn take(&mut self, key: &MediaReuseKey) -> Option<u64> {
        if let Some(list) = self.pool.get_mut(key) {
            if let Some(id) = list.pop() {
                self.reuse_hits += 1;
                return Some(id);
            }
        }
        self.reuse_misses += 1;
        None
    }
}

pub fn clear_media_block_reuse_pool(pool: &mut MediaBlockReusePool) {
    pool.clear();
}

#[derive(Debug, Clone)]
pub struct IvMarkdownMediaReuseMarkdown {
    pub storage: MediaBlockStorage,
    pub pool: MediaBlockReusePool,
    pub is_active: bool,
}

impl Default for IvMarkdownMediaReuseMarkdown {
    fn default() -> Self {
        Self::new()
    }
}

impl IvMarkdownMediaReuseMarkdown {
    pub fn new() -> Self {
        Self {
            storage: MediaBlockStorage::new(),
            pool: MediaBlockReusePool::new(),
            is_active: true,
        }
    }

    pub fn clear_media_block_reuse_pool(&mut self) {
        self.pool.clear();
    }

    pub fn clear_media_block_storage(&mut self) {
        self.storage.clear();
    }

    pub fn collect_old_media_blocks_for_reuse(&mut self) {
        for (id, entry) in &self.storage.blocks {
            self.pool.insert(entry.key.clone(), *id);
        }
    }

    pub fn collect_reused_media_blocks(&mut self) -> usize {
        self.pool.reuse_hits
    }

    pub fn reuse_media_blocks(
        &mut self,
        new_keys: &[MediaReuseKey],
    ) -> HashMap<u64, MediaReuseKey> {
        let mut reused = HashMap::new();
        for key in new_keys {
            if let Some(id) = self.pool.take(key) {
                reused.insert(id, key.clone());
            }
        }
        reused
    }
}

impl Render for IvMarkdownMediaReuseMarkdown {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let cached_count = self.storage.blocks.len();
        let hits = self.pool.reuse_hits;
        let misses = self.pool.reuse_misses;

        div()
            .flex()
            .flex_col()
            .gap_2()
            .p_3()
            .bg(rgb(0x1a1d21))
            .text_color(rgb(0xe0e0e0))
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child("Instant View Media Reuse Pool"),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .text_sm()
                    .child(format!("Cached Blocks: {}", cached_count))
                    .child(format!("Hits: {}", hits))
                    .child(format!("Misses: {}", misses)),
            )
    }
}
