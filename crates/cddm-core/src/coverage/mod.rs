#![forbid(unsafe_code)]

pub mod correlator;
pub mod parser;
pub mod types;

pub use correlator::correlate_coverage;
pub use parser::{load_coverage_report, normalize_path, parse_coverage_data, parse_lcov};
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ClonePair, CloneType, ScanResult};

    #[test]
    fn test_parse_lcov_format() {
        let sample_lcov = r#"
SF:src/auth.ts
DA:10,5
DA:11,5
DA:12,0
DA:13,10
end_of_record
SF:src/helpers.ts
DA:1,100
DA:2,150
end_of_record
"#;

        let report = parse_coverage_data(sample_lcov, CoverageFormat::Lcov).unwrap();
        assert_eq!(report.file_line_hits.len(), 2);
        assert_eq!(report.total_hits, 270);

        let auth_hits = report.file_line_hits.get("src/auth.ts").unwrap();
        assert_eq!(auth_hits.get(&10), Some(&5));
        assert_eq!(auth_hits.get(&12), Some(&0));
    }

    #[test]
    fn test_parse_cobertura_format() {
        let sample_xml = r#"<?xml version="1.0"?>
<coverage line-rate="0.9">
  <packages>
    <package name="core">
      <classes>
        <class name="engine" filename="src/engine.rs">
          <lines>
            <line number="20" hits="42"/>
            <line number="21" hits="0"/>
          </lines>
        </class>
      </classes>
    </package>
  </packages>
</coverage>"#;

        let report = parse_coverage_data(sample_xml, CoverageFormat::Cobertura).unwrap();
        assert_eq!(report.file_line_hits.len(), 1);
        let engine_hits = report.file_line_hits.get("src/engine.rs").unwrap();
        assert_eq!(engine_hits.get(&20), Some(&42));
        assert_eq!(engine_hits.get(&21), Some(&0));
    }

    #[test]
    fn test_coverage_correlation_with_scan_result() {
        let mut scan_result = ScanResult::default();
        scan_result.clone_pairs.push(ClonePair {
            file_a: "src/auth.ts".to_string(),
            start_line_a: 10,
            end_line_a: 13,
            file_b: "src/helpers.ts".to_string(),
            start_line_b: 1,
            end_line_b: 2,
            token_count: 80,
            similarity: 0.98,
            fragment_hash: "hash123".to_string(),
            clone_type: CloneType::Exact,
            author_a: None,
            author_b: None,
        });

        let mut report = CoverageReport::default();
        report.file_line_hits.insert(
            "src/auth.ts".to_string(),
            [(10, 5), (11, 5), (12, 0), (13, 10)].into_iter().collect(),
        );
        report.file_line_hits.insert(
            "src/helpers.ts".to_string(),
            [(1, 100), (2, 150)].into_iter().collect(),
        );

        let summary = correlate_coverage(&scan_result, &report);
        assert_eq!(summary.total_clone_pairs, 1);
        assert_eq!(summary.dead_code_clones, 0);
        assert_eq!(summary.metrics.len(), 1);

        let m = &summary.metrics[0];
        assert_eq!(m.hits_a, 20);
        assert_eq!(m.hits_b, 250);
        assert_eq!(m.total_combined_hits, 270);
        assert_eq!(m.execution_tier, ExecutionTier::Warm);
    }
}
