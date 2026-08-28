#![forbid(unsafe_code)]

use super::types::{EcosystemAlgorithm, RecommendedLibrary};

/// Returns the built-in catalog of canonical open-source library algorithms.
pub fn get_canonical_algorithms() -> Vec<EcosystemAlgorithm> {
    vec![
        EcosystemAlgorithm {
            name: "Array Chunking".to_string(),
            category: "Collections".to_string(),
            description: "Splitting an array or slice into fixed-size contiguous chunks or \
                          batches."
                .to_string(),
            canonical_keywords: vec![
                "chunk".to_string(),
                "chunks".to_string(),
                "batch".to_string(),
                "batch_size".to_string(),
                "chunk_size".to_string(),
                "step_by".to_string(),
            ],
            recommendations: vec![
                RecommendedLibrary {
                    language: "rust".to_string(),
                    package_name: "itertools".to_string(),
                    install_command: "cargo add itertools".to_string(),
                    replacement_snippet: "use itertools::Itertools;\nlet chunks = \
                                          items.iter().chunks(chunk_size);"
                        .to_string(),
                },
                RecommendedLibrary {
                    language: "typescript".to_string(),
                    package_name: "lodash-es".to_string(),
                    install_command: "bun add lodash-es".to_string(),
                    replacement_snippet: "import { chunk } from 'lodash-es';\nconst chunks = \
                                          chunk(items, size);"
                        .to_string(),
                },
                RecommendedLibrary {
                    language: "python".to_string(),
                    package_name: "more-itertools".to_string(),
                    install_command: "pip install more-itertools".to_string(),
                    replacement_snippet: "from more_itertools import chunked\nchunks = \
                                          list(chunked(items, size))"
                        .to_string(),
                },
            ],
        },
        EcosystemAlgorithm {
            name: "String Slugify".to_string(),
            category: "Text & Strings".to_string(),
            description: "Converting arbitrary text into URL-safe ASCII kebab-case slugs with \
                          accent stripping."
                .to_string(),
            canonical_keywords: vec![
                "slug".to_string(),
                "slugify".to_string(),
                "kebab_case".to_string(),
                "kebab".to_string(),
                "normalize_url".to_string(),
            ],
            recommendations: vec![
                RecommendedLibrary {
                    language: "rust".to_string(),
                    package_name: "slug".to_string(),
                    install_command: "cargo add slug".to_string(),
                    replacement_snippet: "use slug::slugify;\nlet slug = slugify(\"My Title \
                                          123\");"
                        .to_string(),
                },
                RecommendedLibrary {
                    language: "typescript".to_string(),
                    package_name: "slugify".to_string(),
                    install_command: "bun add slugify".to_string(),
                    replacement_snippet: "import slugify from 'slugify';\nconst slug = \
                                          slugify('My Title 123', { lower: true });"
                        .to_string(),
                },
                RecommendedLibrary {
                    language: "python".to_string(),
                    package_name: "python-slugify".to_string(),
                    install_command: "pip install python-slugify".to_string(),
                    replacement_snippet: "from slugify import slugify\nslug = slugify('My Title \
                                          123')"
                        .to_string(),
                },
            ],
        },
        EcosystemAlgorithm {
            name: "Debounce Timer".to_string(),
            category: "Async & Timing".to_string(),
            description: "Delaying execution of a high-frequency function until idle time has \
                          elapsed."
                .to_string(),
            canonical_keywords: vec![
                "debounce".to_string(),
                "clear_timeout".to_string(),
                "delay_timer".to_string(),
                "idle_wait".to_string(),
                "last_invoked".to_string(),
            ],
            recommendations: vec![
                RecommendedLibrary {
                    language: "rust".to_string(),
                    package_name: "tokio-util".to_string(),
                    install_command: "cargo add tokio-util".to_string(),
                    replacement_snippet: "use tokio_util::sync::CancellationToken;\n// Use \
                                          CancellationToken or debounce channel"
                        .to_string(),
                },
                RecommendedLibrary {
                    language: "typescript".to_string(),
                    package_name: "lodash-es".to_string(),
                    install_command: "bun add lodash-es".to_string(),
                    replacement_snippet: "import { debounce } from 'lodash-es';\nconst debounced \
                                          = debounce(fn, 300);"
                        .to_string(),
                },
            ],
        },
        EcosystemAlgorithm {
            name: "Retry with Exponential Backoff".to_string(),
            category: "Async & Timing".to_string(),
            description: "Retrying failing operations with progressive backoff delays and jitter."
                .to_string(),
            canonical_keywords: vec![
                "retry".to_string(),
                "backoff".to_string(),
                "exponential".to_string(),
                "max_retries".to_string(),
                "jitter".to_string(),
                "delay_ms".to_string(),
            ],
            recommendations: vec![
                RecommendedLibrary {
                    language: "rust".to_string(),
                    package_name: "backoff".to_string(),
                    install_command: "cargo add backoff --features tokio".to_string(),
                    replacement_snippet: "use backoff::ExponentialBackoff;\\
                                          nbackoff::future::retry(ExponentialBackoff::default(), \
                                          operation).await;"
                        .to_string(),
                },
                RecommendedLibrary {
                    language: "typescript".to_string(),
                    package_name: "p-retry".to_string(),
                    install_command: "bun add p-retry".to_string(),
                    replacement_snippet: "import pRetry from 'p-retry';\nconst result = await \
                                          pRetry(operation, { retries: 3 });"
                        .to_string(),
                },
                RecommendedLibrary {
                    language: "python".to_string(),
                    package_name: "tenacity".to_string(),
                    install_command: "pip install tenacity".to_string(),
                    replacement_snippet: "from tenacity import retry, stop_after_attempt, \
                                          wait_exponential\n@retry(stop=stop_after_attempt(3), \
                                          wait=wait_exponential())\ndef operation(): pass"
                        .to_string(),
                },
            ],
        },
        EcosystemAlgorithm {
            name: "Hex Encoding / Decoding".to_string(),
            category: "Encoding & Crypto".to_string(),
            description: "Converting binary byte buffers to/from lowercase hexadecimal strings."
                .to_string(),
            canonical_keywords: vec![
                "hex".to_string(),
                "encode_hex".to_string(),
                "decode_hex".to_string(),
                "from_hex".to_string(),
                "to_hex".to_string(),
                "0123456789abcdef".to_string(),
            ],
            recommendations: vec![
                RecommendedLibrary {
                    language: "rust".to_string(),
                    package_name: "hex".to_string(),
                    install_command: "cargo add hex".to_string(),
                    replacement_snippet: "use hex::{encode, decode};\nlet hex_str = \
                                          encode(bytes);\nlet raw = decode(hex_str)?;"
                        .to_string(),
                },
                RecommendedLibrary {
                    language: "typescript".to_string(),
                    package_name: "uint8array-tools".to_string(),
                    install_command: "bun add uint8array-tools".to_string(),
                    replacement_snippet: "import { toHex, fromHex } from \
                                          'uint8array-tools';\nconst hex = toHex(bytes);"
                        .to_string(),
                },
            ],
        },
        EcosystemAlgorithm {
            name: "Deep Object Clone".to_string(),
            category: "Data Manipulation".to_string(),
            description: "Deep recursive structural cloning of nested objects, dictionaries, or \
                          data records."
                .to_string(),
            canonical_keywords: vec![
                "clone_deep".to_string(),
                "deep_copy".to_string(),
                "deep_clone".to_string(),
                "deepcopy".to_string(),
                "recursive_copy".to_string(),
            ],
            recommendations: vec![
                RecommendedLibrary {
                    language: "typescript".to_string(),
                    package_name: "lodash-es".to_string(),
                    install_command: "bun add lodash-es".to_string(),
                    replacement_snippet: "import { cloneDeep } from 'lodash-es';\nconst copy = \
                                          cloneDeep(obj);"
                        .to_string(),
                },
                RecommendedLibrary {
                    language: "python".to_string(),
                    package_name: "copy (stdlib)".to_string(),
                    install_command: "# Built-in standard library".to_string(),
                    replacement_snippet: "import copy\ncloned = copy.deepcopy(obj)".to_string(),
                },
            ],
        },
    ]
}
