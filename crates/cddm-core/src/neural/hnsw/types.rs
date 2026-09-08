#![forbid(unsafe_code)]

use std::cmp::Ordering;

/// Configuration parameters for the Hierarchical Navigable Small World (HNSW) vector index.
#[derive(Debug, Clone)]
pub struct HnswConfig {
    pub m: usize,
    pub m0: usize,
    pub ef_construction: usize,
    pub ef_search: usize,
    pub ml: f64,
}

impl Default for HnswConfig {
    fn default() -> Self {
        Self {
            m: 16,
            m0: 32,
            ef_construction: 64,
            ef_search: 32,
            ml: 1.0 / (16.0f64.ln()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HnswNode {
    pub level: usize,
    pub neighbors: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DistItem {
    pub dist: f32,
    pub node_id: usize,
}

impl Eq for DistItem {}
impl Ord for DistItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .partial_cmp(&self.dist)
            .unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for DistItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaxDistItem {
    pub dist: f32,
    pub node_id: usize,
}

impl Eq for MaxDistItem {}
impl Ord for MaxDistItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist
            .partial_cmp(&other.dist)
            .unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for MaxDistItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Helper to generate random layer level using geometrically distributed pseudo-random sequence.
pub fn generate_random_level(seed: usize, ml: f64) -> usize {
    let mut x = (seed as u64)
        .wrapping_mul(0x517cc1b727220a95)
        .wrapping_add(1);
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    let r = ((x.wrapping_mul(0x2545f4914f6cdd1d) >> 11) as f64) / (9007199254740992.0);
    let unif = r.clamp(1e-7, 1.0 - 1e-7);
    ((-unif.ln()) * ml).floor() as usize
}
