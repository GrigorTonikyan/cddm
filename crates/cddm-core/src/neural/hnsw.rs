#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use super::embedder::NeuralCodeEmbedder;
use super::types::{CodeEmbeddingVector, EquivalenceConfidence, NeuralClonePair};

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
struct HnswNode {
    level: usize,
    neighbors: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DistItem {
    dist: f32,
    node_id: usize,
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
struct MaxDistItem {
    dist: f32,
    node_id: usize,
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

/// High-performance Pure-Rust HNSW multi-layer vector index for dense code embeddings.
#[derive(Debug, Clone)]
pub struct HnswVectorIndex {
    config: HnswConfig,
    vectors: Vec<Vec<f32>>,
    nodes: Vec<HnswNode>,
    entry_point: Option<usize>,
    max_level: usize,
}

impl Default for HnswVectorIndex {
    fn default() -> Self {
        Self::new(HnswConfig::default())
    }
}

impl HnswVectorIndex {
    pub fn new(config: HnswConfig) -> Self {
        Self {
            config,
            vectors: Vec::new(),
            nodes: Vec::new(),
            entry_point: None,
            max_level: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.vectors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vectors.is_empty()
    }

    pub fn insert(&mut self, vector: Vec<f32>) -> usize {
        let node_id = self.vectors.len();
        let level = self.generate_random_level(node_id);
        let mut node = HnswNode {
            level,
            neighbors: vec![Vec::new(); level + 1],
        };

        if self.entry_point.is_none() {
            self.vectors.push(vector);
            self.nodes.push(node);
            self.entry_point = Some(node_id);
            self.max_level = level;
            return node_id;
        }

        let ep = self.entry_point.expect("Entry point must exist");
        let mut curr_ep = ep;
        let ep_level = self.nodes[ep].level;

        if ep_level > level {
            for lc in (level + 1..=ep_level).rev() {
                curr_ep = self.greedy_search_layer(&vector, curr_ep, lc);
            }
        }

        let start_layer = level.min(ep_level);
        for lc in (0..=start_layer).rev() {
            let candidates =
                self.search_layer(&vector, &[curr_ep], self.config.ef_construction, lc);
            let m_max = if lc == 0 {
                self.config.m0
            } else {
                self.config.m
            };
            let selected = self.select_neighbors(&candidates, m_max);
            for &neighbor_id in &selected {
                node.neighbors[lc].push(neighbor_id);
            }
            if let Some(closest) = candidates.peek() {
                curr_ep = closest.node_id;
            }
        }

        self.vectors.push(vector);
        self.nodes.push(node);

        for lc in 0..=start_layer {
            let m_max = if lc == 0 {
                self.config.m0
            } else {
                self.config.m
            };
            let neighbor_ids = self.nodes[node_id].neighbors[lc].clone();
            for neighbor_id in neighbor_ids {
                self.nodes[neighbor_id].neighbors[lc].push(node_id);
                if self.nodes[neighbor_id].neighbors[lc].len() > m_max {
                    let n_vec = &self.vectors[neighbor_id];
                    let curr_neighbors = self.nodes[neighbor_id].neighbors[lc].clone();
                    let pruned = self.select_neighbors_from_ids(n_vec, &curr_neighbors, m_max);
                    self.nodes[neighbor_id].neighbors[lc] = pruned;
                }
            }
        }

        if level > self.max_level {
            self.max_level = level;
            self.entry_point = Some(node_id);
        }

        node_id
    }

    pub fn search_top_k(
        &self,
        query: &[f32],
        top_k: usize,
        min_similarity: f32,
    ) -> Vec<(usize, f32)> {
        if self.is_empty() || top_k == 0 {
            return Vec::new();
        }

        let ep = match self.entry_point {
            Some(e) => e,
            None => return Vec::new(),
        };

        let mut curr_ep = ep;
        for lc in (1..=self.max_level).rev() {
            curr_ep = self.greedy_search_layer(query, curr_ep, lc);
        }

        let ef = self.config.ef_search.max(top_k);
        let candidates = self.search_layer(query, &[curr_ep], ef, 0);
        let max_dist = 1.0f32 - min_similarity;
        let mut results = Vec::new();

        for item in candidates {
            if item.dist <= max_dist {
                let similarity = (1.0f32 - item.dist).clamp(0.0, 1.0);
                results.push((item.node_id, similarity));
            }
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        if results.len() > top_k {
            results.truncate(top_k);
        }
        results
    }

    pub fn batch_build(items: &[CodeEmbeddingVector], config: Option<HnswConfig>) -> Self {
        let mut index = Self::new(config.unwrap_or_default());
        for item in items {
            index.insert(item.vector.clone());
        }
        index
    }

    pub fn find_all_pairs(
        items: &[CodeEmbeddingVector],
        config: Option<HnswConfig>,
        min_similarity: f32,
    ) -> Vec<NeuralClonePair> {
        if items.len() < 2 {
            return Vec::new();
        }

        let index = Self::batch_build(items, config);
        let mut pairs = Vec::new();
        let mut seen_pairs = HashSet::new();

        for (i, item) in items.iter().enumerate() {
            let neighbors = index.search_top_k(&item.vector, 16, min_similarity);
            for (j, similarity) in neighbors {
                if i == j {
                    continue;
                }
                let (min_idx, max_idx) = if i < j { (i, j) } else { (j, i) };
                if !seen_pairs.insert((min_idx, max_idx)) {
                    continue;
                }

                let vec_a = &items[min_idx];
                let vec_b = &items[max_idx];
                if vec_a.file_path == vec_b.file_path && vec_a.start_line == vec_b.start_line {
                    continue;
                }

                let confidence = if similarity >= super::constants::HIGH_CONFIDENCE_THRESHOLD {
                    EquivalenceConfidence::High
                } else if similarity >= super::constants::MEDIUM_CONFIDENCE_THRESHOLD {
                    EquivalenceConfidence::Medium
                } else {
                    EquivalenceConfidence::Low
                };

                let rationale = format!(
                    "HNSW index cosine similarity {:.1}% across {} and {}",
                    similarity * 100.0,
                    vec_a.language,
                    vec_b.language
                );

                pairs.push(NeuralClonePair {
                    file_a: vec_a.file_path.clone(),
                    start_line_a: vec_a.start_line,
                    end_line_a: vec_a.end_line,
                    language_a: vec_a.language.clone(),
                    file_b: vec_b.file_path.clone(),
                    start_line_b: vec_b.start_line,
                    end_line_b: vec_b.end_line,
                    language_b: vec_b.language.clone(),
                    similarity,
                    confidence,
                    semantic_rationale: rationale,
                });
            }
        }

        pairs.sort_by(|a, b| {
            b.similarity
                .partial_cmp(&a.similarity)
                .unwrap_or(Ordering::Equal)
        });
        pairs
    }

    #[inline]
    fn compute_dist(&self, vec_a: &[f32], node_id: usize) -> f32 {
        let vec_b = &self.vectors[node_id];
        (1.0f32 - NeuralCodeEmbedder::cosine_similarity(vec_a, vec_b)).max(0.0)
    }

    fn greedy_search_layer(&self, query: &[f32], mut curr: usize, layer: usize) -> usize {
        let mut curr_dist = self.compute_dist(query, curr);
        loop {
            let mut changed = false;
            for &neighbor in &self.nodes[curr].neighbors[layer] {
                let dist = self.compute_dist(query, neighbor);
                if dist < curr_dist {
                    curr_dist = dist;
                    curr = neighbor;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        curr
    }

    fn search_layer(
        &self,
        query: &[f32],
        entry_points: &[usize],
        ef: usize,
        layer: usize,
    ) -> BinaryHeap<MaxDistItem> {
        let mut visited = HashSet::new();
        let mut candidates = BinaryHeap::new();
        let mut results = BinaryHeap::new();

        for &ep in entry_points {
            let dist = self.compute_dist(query, ep);
            visited.insert(ep);
            candidates.push(DistItem { dist, node_id: ep });
            results.push(MaxDistItem { dist, node_id: ep });
        }

        while let Some(candidate) = candidates.pop() {
            let worst_result = results.peek().copied();
            if let Some(worst) = worst_result
                && candidate.dist > worst.dist
                && results.len() >= ef
            {
                break;
            }

            if layer < self.nodes[candidate.node_id].neighbors.len() {
                for &neighbor in &self.nodes[candidate.node_id].neighbors[layer] {
                    if visited.insert(neighbor) {
                        let dist = self.compute_dist(query, neighbor);
                        let worst_dist = results.peek().map(|w| w.dist).unwrap_or(f32::MAX);

                        if dist < worst_dist || results.len() < ef {
                            candidates.push(DistItem {
                                dist,
                                node_id: neighbor,
                            });
                            results.push(MaxDistItem {
                                dist,
                                node_id: neighbor,
                            });
                            if results.len() > ef {
                                results.pop();
                            }
                        }
                    }
                }
            }
        }

        results
    }

    fn select_neighbors(&self, candidates: &BinaryHeap<MaxDistItem>, m_max: usize) -> Vec<usize> {
        let mut items: Vec<_> = candidates.iter().copied().collect();
        items.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap_or(Ordering::Equal));
        items.into_iter().take(m_max).map(|i| i.node_id).collect()
    }

    fn select_neighbors_from_ids(
        &self,
        query: &[f32],
        neighbor_ids: &[usize],
        m_max: usize,
    ) -> Vec<usize> {
        let mut items: Vec<_> = neighbor_ids
            .iter()
            .map(|&id| MaxDistItem {
                dist: self.compute_dist(query, id),
                node_id: id,
            })
            .collect();
        items.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap_or(Ordering::Equal));
        items.into_iter().take(m_max).map(|i| i.node_id).collect()
    }

    fn generate_random_level(&self, seed: usize) -> usize {
        let mut x = (seed as u64)
            .wrapping_mul(0x517cc1b727220a95)
            .wrapping_add(1);
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        let r = ((x.wrapping_mul(0x2545f4914f6cdd1d) >> 11) as f64) / (9007199254740992.0);
        let unif = r.clamp(1e-7, 1.0 - 1e-7);
        ((-unif.ln()) * self.config.ml).floor() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hnsw_empty_and_single_insert() {
        let mut index = HnswVectorIndex::default();
        assert!(index.is_empty());
        assert_eq!(index.len(), 0);

        let vec1 = vec![1.0, 0.0, 0.0];
        let id1 = index.insert(vec1.clone());
        assert_eq!(id1, 0);
        assert_eq!(index.len(), 1);

        let results = index.search_top_k(&vec1, 1, 0.9);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, 0);
        assert!(results[0].1 >= 0.99);
    }

    #[test]
    fn test_hnsw_top_k_search() {
        let mut index = HnswVectorIndex::default();
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![0.9, 0.1, 0.0];
        let v3 = vec![0.0, 1.0, 0.0];

        index.insert(v1.clone());
        index.insert(v2);
        index.insert(v3);

        let res = index.search_top_k(&v1, 2, 0.5);
        assert!(res.len() >= 2);
        assert_eq!(res[0].0, 0);
        assert_eq!(res[1].0, 1);
    }

    #[test]
    fn test_hnsw_find_all_pairs() {
        let v1 = CodeEmbeddingVector {
            file_path: "src/a.rs".to_string(),
            start_line: 1,
            end_line: 10,
            language: "Rust".to_string(),
            vector: vec![0.95, 0.05, 0.0],
            norm: 1.0,
        };
        let v2 = CodeEmbeddingVector {
            file_path: "src/b.rs".to_string(),
            start_line: 20,
            end_line: 30,
            language: "Rust".to_string(),
            vector: vec![0.93, 0.07, 0.0],
            norm: 1.0,
        };
        let items = vec![v1, v2];
        let pairs = HnswVectorIndex::find_all_pairs(&items, None, 0.85);

        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0].file_a, "src/a.rs");
        assert_eq!(pairs[0].file_b, "src/b.rs");
    }
}
