#![allow(unsafe_code)]

use std::fs::File;
use std::io::{Error, ErrorKind};
use std::ops::Deref;
use std::path::Path;

/// Threshold file size in bytes above which zero-copy memory mapping (`memmap2`) is used.
/// Files below or equal to this threshold use standard heap-buffered reads to avoid syscall overhead.
pub const MMAP_THRESHOLD_BYTES: u64 = 64 * 1024; // 64 KB

/// File content buffer backed either by a heap-allocated `String` or a zero-copy read-only `memmap2::Mmap`.
pub enum FileSource {
    /// Heap-allocated string for small files (<= 64 KB).
    Heap(String),
    /// Zero-copy read-only memory map for large files (> 64 KB).
    Mmap(memmap2::Mmap),
}

impl FileSource {
    /// Borrows the source buffer as a string slice (`&str`).
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            FileSource::Heap(s) => s.as_str(),
            FileSource::Mmap(m) => {
                // SAFETY: UTF-8 validity is guaranteed by the constructor `read_file_source`
                // which explicitly checks `std::str::from_utf8(m.as_ref())` before returning `Mmap`.
                unsafe { std::str::from_utf8_unchecked(m.as_ref()) }
            }
        }
    }

    /// Returns the length of the string buffer in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.as_str().len()
    }

    /// Returns `true` if the buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }

    /// Returns `true` if this buffer is backed by a memory-mapped file.
    #[inline]
    pub fn is_mmap(&self) -> bool {
        matches!(self, FileSource::Mmap(_))
    }

    /// Returns `true` if this buffer is backed by a heap-allocated string.
    #[inline]
    pub fn is_heap(&self) -> bool {
        matches!(self, FileSource::Heap(_))
    }
}

impl Deref for FileSource {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for FileSource {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Debug for FileSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileSource::Heap(_) => f
                .debug_struct("FileSource::Heap")
                .field("len_bytes", &self.len())
                .finish(),
            FileSource::Mmap(_) => f
                .debug_struct("FileSource::Mmap")
                .field("len_bytes", &self.len())
                .finish(),
        }
    }
}

/// Reads the target file into a `FileSource` buffer.
///
/// For files larger than `MMAP_THRESHOLD_BYTES` (64 KB), a read-only memory map is created.
/// For smaller files or empty files, standard heap reading is utilized.
pub fn read_file_source<P: AsRef<Path>>(path: P) -> Result<FileSource, Error> {
    let path_ref = path.as_ref();
    let file = File::open(path_ref)?;
    let metadata = file.metadata()?;
    let file_len = metadata.len();

    if file_len == 0 {
        return Ok(FileSource::Heap(String::new()));
    }

    if file_len > MMAP_THRESHOLD_BYTES {
        // SAFETY: The file handle is opened in read-only mode and is not modified by CDDM.
        // Memory map is created read-only for the duration of parsing.
        let mmap_res = unsafe { memmap2::Mmap::map(&file) };
        match mmap_res {
            Ok(mmap) => {
                // Verify valid UTF-8 before creating the Mmap variant
                if std::str::from_utf8(mmap.as_ref()).is_ok() {
                    return Ok(FileSource::Mmap(mmap));
                } else {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "File content is not valid UTF-8",
                    ));
                }
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to memory-map file {}: {}; falling back to heap read",
                    path_ref.display(),
                    e
                );
            }
        }
    }

    // Standard heap read fallback for files <= 64KB or when mmap fails
    let content = std::fs::read_to_string(path_ref)?;
    Ok(FileSource::Heap(content))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_small_file_uses_heap() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "fn small_test() {{ println!(\"hello\"); }}").unwrap();
        file.flush().unwrap();

        let source = read_file_source(file.path()).unwrap();
        assert!(source.is_heap());
        assert!(!source.is_mmap());
        assert!(source.contains("fn small_test()"));
        assert_eq!(source.as_str(), source.deref());
    }

    #[test]
    fn test_read_large_file_uses_mmap() {
        let mut file = NamedTempFile::new().unwrap();
        // Write ~70KB of data (above 64KB threshold)
        let chunk = "pub fn large_function_test_line() -> i32 { 42 }\n";
        let repeat_count = (MMAP_THRESHOLD_BYTES as usize / chunk.len()) + 50;
        for _ in 0..repeat_count {
            file.write_all(chunk.as_bytes()).unwrap();
        }
        file.flush().unwrap();

        let source = read_file_source(file.path()).unwrap();
        assert!(source.is_mmap());
        assert!(!source.is_heap());
        assert!(source.len() > MMAP_THRESHOLD_BYTES as usize);
        assert!(source.starts_with("pub fn large_function"));
    }

    #[test]
    fn test_read_empty_file() {
        let file = NamedTempFile::new().unwrap();
        let source = read_file_source(file.path()).unwrap();
        assert!(source.is_heap());
        assert!(source.is_empty());
        assert_eq!(source.len(), 0);
    }

    #[test]
    fn test_read_nonexistent_file() {
        let res = read_file_source(Path::new("non_existent_path_cddm_test.rs"));
        assert!(res.is_err());
    }

    #[test]
    fn test_read_non_utf8_large_file() {
        let mut file = NamedTempFile::new().unwrap();
        // Write invalid UTF-8 bytes > 64KB
        let mut invalid_bytes = vec![0xFF, 0xFE, 0xFD, 0x80];
        invalid_bytes.resize((MMAP_THRESHOLD_BYTES as usize) + 100, 0x80);
        file.write_all(&invalid_bytes).unwrap();
        file.flush().unwrap();

        let res = read_file_source(file.path());
        assert!(res.is_err());
    }

    #[test]
    fn test_debug_formatting() {
        let heap_source = FileSource::Heap("test".to_string());
        let debug_str = format!("{:?}", heap_source);
        assert!(debug_str.contains("FileSource::Heap"));
    }
}
