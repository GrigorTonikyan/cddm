#![forbid(unsafe_code)]

use crate::fingerprint::Fingerprint;
use crate::types::LineSpan;

/// Represents an intermediate parsed file in memory during scan pipeline.
#[derive(Clone, Debug)]
pub struct ParsedFile {
    pub path: String,
    pub language: String,
    pub token_count: usize,
    pub token_spans: Vec<LineSpan>,
    pub fingerprints: Vec<Fingerprint>,
}

#[derive(Clone, Debug)]
pub struct Location {
    pub file_idx: usize,
    pub span: LineSpan,
}

pub fn count_tokens_in_line_span(spans: &[LineSpan], start_line: usize, end_line: usize) -> usize {
    spans
        .iter()
        .filter(|s| s.line_start >= start_line && s.line_end <= end_line)
        .count()
}
