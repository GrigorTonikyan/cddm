#![forbid(unsafe_code)]

use super::types::{
    CrossRepoClonePair, CrossRepoCluster, CrossRepoOccurrence, HubConfig, HubRepoConfig,
    HubScanSummary, RepoDuplicationMetric,
};
use crate::detector::run_scan;
use crate::types::{ScanConfig, ScanResult};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::sync::mpsc;

struct Dsu {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl Dsu {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let p = self.parent[i];
            self.parent[i] = self.find(p);
            self.parent[i]
        }
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

/// Executes an organization-wide Federation Hub scan across all configured repositories.
pub async fn run_hub_scan(config: &HubConfig) -> Result<HubScanSummary, String> {
    if config.repositories.is_empty() {
        return Ok(HubScanSummary {
            hub_name: config.name.clone(),
            total_repos: 0,
            repos: Vec::new(),
            total_files: 0,
            total_tokens: 0,
            total_clones: 0,
            cross_repo_clones: 0,
            cross_repo_clusters: 0,
            organization_dry_score: 100.0,
            cross_repo_duplication_pct: 0.0,
            duplication_matrix: Vec::new(),
            clusters: Vec::new(),
            top_cross_repo_pairs: Vec::new(),
        });
    }

    let mut repo_results: Vec<(HubRepoConfig, ScanResult)> = Vec::new();
    let mut total_files = 0;
    let mut total_tokens = 0;
    let mut total_clones = 0;

    for repo in &config.repositories {
        let repo_path = Path::new(&repo.path);
        if !repo_path.exists() {
            continue;
        }

        let scan_config = ScanConfig {
            directory: repo.path.clone(),
            min_tokens: config.min_tokens,
            ignore_patterns: config.ignore_patterns.clone(),
            cache_dir: None,
            enable_cache: false,
            ..Default::default()
        };

        let (tx, _rx) = mpsc::channel(100);
        let cancel = Arc::new(AtomicBool::new(false));
        if let Ok(res) = run_scan(scan_config, tx, cancel).await {
            total_files += res.total_files;
            total_tokens += res.total_tokens;
            total_clones += res.total_clones;
            repo_results.push((repo.clone(), res));
        }
    }

    // Correlate cross-repo clone pairs
    let (cross_pairs, matrix_map) = find_cross_repo_clones(&repo_results, config.min_tokens);
    let clusters = cluster_cross_repo_pairs(&cross_pairs);

    let cross_repo_clones = cross_pairs.len();
    let cross_dup_tokens: usize = cross_pairs.iter().map(|p| p.tokens).sum();
    let cross_repo_duplication_pct = if total_tokens > 0 {
        ((cross_dup_tokens as f64 / total_tokens as f64) * 100.0).min(100.0)
    } else {
        0.0
    };

    let organization_dry_score = (100.0 - cross_repo_duplication_pct).clamp(0.0, 100.0);

    let mut duplication_matrix = Vec::new();
    for ((repo_a, repo_b), (shared_clones, shared_tokens)) in matrix_map {
        duplication_matrix.push(RepoDuplicationMetric {
            repo_a,
            repo_b,
            shared_clones,
            shared_tokens,
        });
    }
    duplication_matrix.sort_by_key(|b| std::cmp::Reverse(b.shared_tokens));

    let top_cross_repo_pairs = cross_pairs.iter().take(50).cloned().collect();

    Ok(HubScanSummary {
        hub_name: config.name.clone(),
        total_repos: repo_results.len(),
        repos: repo_results.into_iter().map(|(r, _)| r).collect(),
        total_files,
        total_tokens,
        total_clones,
        cross_repo_clones,
        cross_repo_clusters: clusters.len(),
        organization_dry_score,
        cross_repo_duplication_pct,
        duplication_matrix,
        clusters,
        top_cross_repo_pairs,
    })
}

type CrossRepoCloneMap = HashMap<(String, String), (usize, usize)>;

fn find_cross_repo_clones(
    repo_results: &[(HubRepoConfig, ScanResult)],
    min_tokens: usize,
) -> (Vec<CrossRepoClonePair>, CrossRepoCloneMap) {
    let mut cross_pairs = Vec::new();
    let mut matrix_map: CrossRepoCloneMap = HashMap::new();
    let mut id_counter = 1;

    for i in 0..repo_results.len() {
        for j in (i + 1)..repo_results.len() {
            let (repo_a, res_a) = &repo_results[i];
            let (repo_b, res_b) = &repo_results[j];

            let matched_pairs =
                match_clone_pairs_between_repos(repo_a, res_a, repo_b, res_b, min_tokens);
            for mut pair in matched_pairs {
                pair.id = id_counter;
                id_counter += 1;

                let key = if repo_a.name <= repo_b.name {
                    (repo_a.name.clone(), repo_b.name.clone())
                } else {
                    (repo_b.name.clone(), repo_a.name.clone())
                };
                let entry = matrix_map.entry(key).or_insert((0, 0));
                entry.0 += 1;
                entry.1 += pair.tokens;

                cross_pairs.push(pair);
            }
        }
    }

    cross_pairs.sort_by_key(|b| std::cmp::Reverse(b.tokens));
    (cross_pairs, matrix_map)
}

fn match_clone_pairs_between_repos(
    repo_a: &HubRepoConfig,
    res_a: &ScanResult,
    repo_b: &HubRepoConfig,
    res_b: &ScanResult,
    min_tokens: usize,
) -> Vec<CrossRepoClonePair> {
    let mut matches = Vec::new();

    // Match clone pairs sharing the same structural fragment hash
    for pair_a in &res_a.clone_pairs {
        for pair_b in &res_b.clone_pairs {
            if pair_a.token_count >= min_tokens
                && pair_b.token_count >= min_tokens
                && pair_a.fragment_hash == pair_b.fragment_hash
            {
                matches.push(CrossRepoClonePair {
                    id: 0,
                    repo_a: repo_a.name.clone(),
                    file_a: pair_a.file_a.clone(),
                    lines_a: (pair_a.start_line_a, pair_a.end_line_a),
                    repo_b: repo_b.name.clone(),
                    file_b: pair_b.file_a.clone(),
                    lines_b: (pair_b.start_line_a, pair_b.end_line_a),
                    tokens: pair_a.token_count,
                    similarity: pair_a.similarity.min(pair_b.similarity),
                    clone_type: format!("{:?}", pair_a.clone_type),
                });
                break;
            }
        }
    }

    matches
}

fn cluster_cross_repo_pairs(pairs: &[CrossRepoClonePair]) -> Vec<CrossRepoCluster> {
    if pairs.is_empty() {
        return Vec::new();
    }

    let mut occ_list: Vec<CrossRepoOccurrence> = Vec::new();
    let mut occ_indices: HashMap<(String, String, usize, usize), usize> = HashMap::new();

    for p in pairs {
        let occ_a = (p.repo_a.clone(), p.file_a.clone(), p.lines_a.0, p.lines_a.1);
        if !occ_indices.contains_key(&occ_a) {
            occ_indices.insert(occ_a.clone(), occ_list.len());
            occ_list.push(CrossRepoOccurrence {
                repo_name: p.repo_a.clone(),
                file_path: p.file_a.clone(),
                start_line: p.lines_a.0,
                end_line: p.lines_a.1,
                snippet: None,
            });
        }

        let occ_b = (p.repo_b.clone(), p.file_b.clone(), p.lines_b.0, p.lines_b.1);
        if !occ_indices.contains_key(&occ_b) {
            occ_indices.insert(occ_b.clone(), occ_list.len());
            occ_list.push(CrossRepoOccurrence {
                repo_name: p.repo_b.clone(),
                file_path: p.file_b.clone(),
                start_line: p.lines_b.0,
                end_line: p.lines_b.1,
                snippet: None,
            });
        }
    }

    let mut dsu = Dsu::new(occ_list.len());
    for p in pairs {
        let idx_a = *occ_indices
            .get(&(p.repo_a.clone(), p.file_a.clone(), p.lines_a.0, p.lines_a.1))
            .unwrap();
        let idx_b = *occ_indices
            .get(&(p.repo_b.clone(), p.file_b.clone(), p.lines_b.0, p.lines_b.1))
            .unwrap();
        dsu.union(idx_a, idx_b);
    }

    let mut group_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..occ_list.len() {
        let root = dsu.find(i);
        group_map.entry(root).or_default().push(i);
    }

    let mut clusters = Vec::new();
    let mut cluster_id = 1;
    for indices in group_map.values() {
        if indices.len() >= 2 {
            let mut repos_set = HashSet::new();
            let mut occurrences = Vec::new();
            for &idx in indices {
                let occ = occ_list[idx].clone();
                repos_set.insert(occ.repo_name.clone());
                occurrences.push(occ);
            }

            let mut repos: Vec<String> = repos_set.into_iter().collect();
            repos.sort();

            let suggested_pkg = if occurrences[0].file_path.ends_with(".rs") {
                format!("cddm-shared-{}", cluster_id)
            } else if occurrences[0].file_path.ends_with(".py") {
                format!("shared_common_{}", cluster_id)
            } else {
                format!("@org/shared-utils-{}", cluster_id)
            };

            clusters.push(CrossRepoCluster {
                id: cluster_id,
                repos,
                occurrences,
                token_count: pairs.first().map(|p| p.tokens).unwrap_or(50),
                similarity: 1.0,
                suggested_package: suggested_pkg,
            });
            cluster_id += 1;
        }
    }

    clusters
}
