use crate::types::{CloneCluster, CloneLocation, ClonePair, CloneType};
use std::collections::HashMap;

/// Disjoint-Set Union (Union-Find) with path compression and rank optimization.
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            if self.rank[root_i] < self.rank[root_j] {
                self.parent[root_i] = root_j;
            } else if self.rank[root_i] > self.rank[root_j] {
                self.parent[root_j] = root_i;
            } else {
                self.parent[root_j] = root_i;
                self.rank[root_i] += 1;
            }
        }
    }
}

/// Clusters pairwise clones into N-way equivalence classes using connected-components graph analysis.
pub fn cluster_clone_pairs(pairs: &[ClonePair]) -> Vec<CloneCluster> {
    if pairs.is_empty() {
        return Vec::new();
    }

    let mut location_map: HashMap<CloneLocation, usize> = HashMap::new();
    let mut locations: Vec<CloneLocation> = Vec::new();

    let mut get_or_insert_loc = |loc: CloneLocation| -> usize {
        if let Some(&idx) = location_map.get(&loc) {
            idx
        } else {
            let idx = locations.len();
            location_map.insert(loc.clone(), idx);
            locations.push(loc);
            idx
        }
    };

    // Pre-register all locations
    let mut edges: Vec<(usize, usize, usize)> = Vec::with_capacity(pairs.len());
    for (pair_idx, pair) in pairs.iter().enumerate() {
        let loc_a = CloneLocation {
            file: pair.file_a.clone(),
            start_line: pair.start_line_a,
            end_line: pair.end_line_a,
            author: pair.author_a.clone(),
        };
        let loc_b = CloneLocation {
            file: pair.file_b.clone(),
            start_line: pair.start_line_b,
            end_line: pair.end_line_b,
            author: pair.author_b.clone(),
        };

        let idx_a = get_or_insert_loc(loc_a);
        let idx_b = get_or_insert_loc(loc_b);
        edges.push((idx_a, idx_b, pair_idx));
    }

    let mut uf = UnionFind::new(locations.len());
    for &(idx_a, idx_b, _) in &edges {
        uf.union(idx_a, idx_b);
    }

    // Group locations and pairs by component root
    let mut component_locations: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut component_pair_indices: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..locations.len() {
        let root = uf.find(i);
        component_locations.entry(root).or_default().push(i);
    }

    for &(_, _, pair_idx) in &edges {
        let root = uf.find(edges[pair_idx].0);
        component_pair_indices
            .entry(root)
            .or_default()
            .push(pair_idx);
    }

    let mut raw_clusters = Vec::new();

    for (root, loc_indices) in component_locations {
        let mut cluster_occurrences: Vec<CloneLocation> = loc_indices
            .into_iter()
            .map(|i| locations[i].clone())
            .collect();

        // Sort occurrences deterministically by file, then start_line
        cluster_occurrences.sort_by(|a, b| {
            a.file
                .cmp(&b.file)
                .then(a.start_line.cmp(&b.start_line))
                .then(a.end_line.cmp(&b.end_line))
        });

        // Deduplicate any identical locations within component
        cluster_occurrences.dedup_by(|a, b| {
            a.file == b.file && a.start_line == b.start_line && a.end_line == b.end_line
        });

        let pair_idxs = component_pair_indices
            .get(&root)
            .cloned()
            .unwrap_or_default();

        let mut max_tokens = 0;
        let mut sim_sum = 0.0;
        let mut sim_count = 0;
        let mut dominant_type = CloneType::Exact;
        let mut rep_hash = String::new();

        for &pidx in &pair_idxs {
            let pair = &pairs[pidx];
            if pair.token_count > max_tokens {
                max_tokens = pair.token_count;
            }
            sim_sum += pair.similarity;
            sim_count += 1;

            if rep_hash.is_empty() {
                rep_hash = pair.fragment_hash.clone();
            }

            // Prioritize type classification severity: Semantic > NearMiss > Renamed > Exact
            match (&dominant_type, &pair.clone_type) {
                (_, CloneType::Semantic) => dominant_type = CloneType::Semantic,
                (CloneType::Exact | CloneType::Renamed, CloneType::NearMiss) => {
                    dominant_type = CloneType::NearMiss;
                }
                (CloneType::Exact, CloneType::Renamed) => {
                    dominant_type = CloneType::Renamed;
                }
                _ => {}
            }
        }

        let similarity = if sim_count > 0 {
            ((sim_sum / sim_count as f64) * 100.0).round() / 100.0
        } else {
            1.0
        };

        raw_clusters.push(CloneCluster {
            id: 0, // Assigned after sorting
            clone_type: dominant_type,
            token_count: max_tokens,
            similarity,
            fragment_hash: rep_hash,
            occurrences: cluster_occurrences,
        });
    }

    // Sort clusters by token count descending, then occurrences count descending, then file path
    raw_clusters.sort_by(|a, b| {
        b.token_count
            .cmp(&a.token_count)
            .then(b.occurrences.len().cmp(&a.occurrences.len()))
            .then_with(|| {
                let first_a = a.occurrences.first().map(|o| &o.file);
                let first_b = b.occurrences.first().map(|o| &o.file);
                first_a.cmp(&first_b)
            })
    });

    // Assign 1-based sequential IDs
    for (idx, cluster) in raw_clusters.iter_mut().enumerate() {
        cluster.id = idx + 1;
    }

    raw_clusters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cluster_empty_pairs() {
        let clusters = cluster_clone_pairs(&[]);
        assert!(clusters.is_empty());
    }

    #[test]
    fn test_cluster_single_pair() {
        let pairs = vec![ClonePair {
            file_a: "src/a.rs".to_string(),
            start_line_a: 10,
            end_line_a: 25,
            file_b: "src/b.rs".to_string(),
            start_line_b: 30,
            end_line_b: 45,
            token_count: 80,
            similarity: 1.0,
            fragment_hash: "hash1".to_string(),
            clone_type: CloneType::Exact,
            author_a: None,
            author_b: None,
        }];

        let clusters = cluster_clone_pairs(&pairs);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].id, 1);
        assert_eq!(clusters[0].occurrences.len(), 2);
        assert_eq!(clusters[0].occurrences[0].file, "src/a.rs");
        assert_eq!(clusters[0].occurrences[1].file, "src/b.rs");
        assert_eq!(clusters[0].token_count, 80);
        assert_eq!(clusters[0].clone_type, CloneType::Exact);
    }

    #[allow(clippy::too_many_arguments)]
    fn make_test_pair(
        file_a: &str,
        start_a: usize,
        end_a: usize,
        file_b: &str,
        start_b: usize,
        end_b: usize,
        tokens: usize,
        sim: f64,
        clone_type: CloneType,
    ) -> ClonePair {
        ClonePair {
            file_a: file_a.to_string(),
            start_line_a: start_a,
            end_line_a: end_a,
            file_b: file_b.to_string(),
            start_line_b: start_b,
            end_line_b: end_b,
            token_count: tokens,
            similarity: sim,
            fragment_hash: "hash_test".to_string(),
            clone_type,
            author_a: None,
            author_b: None,
        }
    }

    #[test]
    fn test_cluster_transitive_triplet() {
        let pairs = vec![
            make_test_pair(
                "src/a.rs",
                10,
                20,
                "src/b.rs",
                30,
                40,
                50,
                1.0,
                CloneType::Exact,
            ),
            make_test_pair(
                "src/b.rs",
                30,
                40,
                "src/c.rs",
                50,
                60,
                55,
                0.95,
                CloneType::Renamed,
            ),
        ];

        let clusters = cluster_clone_pairs(&pairs);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].occurrences.len(), 3);
        assert_eq!(clusters[0].occurrences[0].file, "src/a.rs");
        assert_eq!(clusters[0].occurrences[1].file, "src/b.rs");
        assert_eq!(clusters[0].occurrences[2].file, "src/c.rs");
        assert_eq!(clusters[0].token_count, 55);
        assert_eq!(clusters[0].clone_type, CloneType::Renamed);
    }

    #[test]
    fn test_cluster_multiple_disjoint_components() {
        let pairs = vec![
            make_test_pair(
                "src/a.rs",
                1,
                10,
                "src/b.rs",
                1,
                10,
                100,
                1.0,
                CloneType::Exact,
            ),
            make_test_pair(
                "src/c.rs",
                20,
                30,
                "src/d.rs",
                20,
                30,
                60,
                0.9,
                CloneType::Renamed,
            ),
            make_test_pair(
                "src/d.rs",
                20,
                30,
                "src/e.rs",
                20,
                30,
                60,
                0.85,
                CloneType::NearMiss,
            ),
        ];

        let clusters = cluster_clone_pairs(&pairs);
        assert_eq!(clusters.len(), 2);
        assert_eq!(clusters[0].id, 1);
        assert_eq!(clusters[0].token_count, 100);
        assert_eq!(clusters[0].occurrences.len(), 2);

        assert_eq!(clusters[1].id, 2);
        assert_eq!(clusters[1].token_count, 60);
        assert_eq!(clusters[1].occurrences.len(), 3);
        assert_eq!(clusters[1].clone_type, CloneType::NearMiss);
    }
}
