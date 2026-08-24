//! Zero-copy I/O subsystem providing memory-mapped file loading for high-throughput polyglot parsing.

pub mod mmap;

pub use mmap::{FileSource, MMAP_THRESHOLD_BYTES, read_file_source};
