#![forbid(unsafe_code)]

use super::types::{EcosystemAlgorithm, RecommendedLibrary};

fn rec(lang: &str, pkg: &str, install: &str, snippet: &str) -> RecommendedLibrary {
    RecommendedLibrary {
        language: lang.to_string(),
        package_name: pkg.to_string(),
        install_command: install.to_string(),
        replacement_snippet: snippet.to_string(),
    }
}

fn algo(
    name: &str,
    category: &str,
    description: &str,
    keywords: &[&str],
    recommendations: Vec<RecommendedLibrary>,
) -> EcosystemAlgorithm {
    EcosystemAlgorithm {
        name: name.to_string(),
        category: category.to_string(),
        description: description.to_string(),
        canonical_keywords: keywords.iter().map(|k| k.to_string()).collect(),
        recommendations,
    }
}

/// Returns the built-in catalog of canonical open-source library algorithms.
pub fn get_canonical_algorithms() -> Vec<EcosystemAlgorithm> {
    vec![
        algo(
            "Array Chunking",
            "Collections",
            "Splitting an array or slice into fixed-size contiguous chunks or batches.",
            &[
                "chunk",
                "chunks",
                "batch",
                "batch_size",
                "chunk_size",
                "step_by",
            ],
            vec![
                rec(
                    "rust",
                    "itertools",
                    "cargo add itertools",
                    "use itertools::Itertools;\nlet chunks = items.iter().chunks(chunk_size);",
                ),
                rec(
                    "typescript",
                    "lodash-es",
                    "bun add lodash-es",
                    "import { chunk } from 'lodash-es';\nconst chunks = chunk(items, size);",
                ),
                rec(
                    "python",
                    "more-itertools",
                    "pip install more-itertools",
                    "from more_itertools import chunked\nchunks = list(chunked(items, size))",
                ),
            ],
        ),
        algo(
            "String Slugify",
            "Text & Strings",
            "Converting arbitrary text into URL-safe ASCII kebab-case slugs with accent stripping.",
            &["slug", "slugify", "kebab_case", "kebab", "normalize_url"],
            vec![
                rec(
                    "rust",
                    "slug",
                    "cargo add slug",
                    "use slug::slugify;\nlet slug = slugify(\"My Title 123\");",
                ),
                rec(
                    "typescript",
                    "slugify",
                    "bun add slugify",
                    "import slugify from 'slugify';\nconst slug = slugify('My Title 123', { \
                     lower: true });",
                ),
                rec(
                    "python",
                    "python-slugify",
                    "pip install python-slugify",
                    "from slugify import slugify\nslug = slugify('My Title 123')",
                ),
            ],
        ),
        algo(
            "Debounce Timer",
            "Async & Timing",
            "Delaying execution of a high-frequency function until idle time has elapsed.",
            &[
                "debounce",
                "clear_timeout",
                "delay_timer",
                "idle_wait",
                "last_invoked",
            ],
            vec![
                rec(
                    "rust",
                    "tokio-util",
                    "cargo add tokio-util",
                    "use tokio_util::sync::CancellationToken;\n// Use CancellationToken or \
                     debounce channel",
                ),
                rec(
                    "typescript",
                    "lodash-es",
                    "bun add lodash-es",
                    "import { debounce } from 'lodash-es';\nconst debounced = debounce(fn, 300);",
                ),
            ],
        ),
        algo(
            "Retry with Exponential Backoff",
            "Async & Timing",
            "Retrying failing operations with progressive backoff delays and jitter.",
            &[
                "retry",
                "backoff",
                "exponential",
                "max_retries",
                "jitter",
                "delay_ms",
            ],
            vec![
                rec(
                    "rust",
                    "backoff",
                    "cargo add backoff --features tokio",
                    "use backoff::ExponentialBackoff;\\
                     nbackoff::future::retry(ExponentialBackoff::default(), operation).await;",
                ),
                rec(
                    "typescript",
                    "p-retry",
                    "bun add p-retry",
                    "import pRetry from 'p-retry';\nconst result = await pRetry(operation, { \
                     retries: 3 });",
                ),
                rec(
                    "python",
                    "tenacity",
                    "pip install tenacity",
                    "from tenacity import retry, stop_after_attempt, \
                     wait_exponential\n@retry(stop=stop_after_attempt(3), \
                     wait=wait_exponential())\ndef operation(): pass",
                ),
            ],
        ),
        algo(
            "Hex Encoding / Decoding",
            "Encoding & Crypto",
            "Converting binary byte buffers to/from lowercase hexadecimal strings.",
            &[
                "hex",
                "encode_hex",
                "decode_hex",
                "from_hex",
                "to_hex",
                "0123456789abcdef",
            ],
            vec![
                rec(
                    "rust",
                    "hex",
                    "cargo add hex",
                    "use hex::{encode, decode};\nlet hex_str = encode(bytes);\nlet raw = \
                     decode(hex_str)?;",
                ),
                rec(
                    "typescript",
                    "uint8array-tools",
                    "bun add uint8array-tools",
                    "import { toHex, fromHex } from 'uint8array-tools';\nconst hex = toHex(bytes);",
                ),
            ],
        ),
        algo(
            "Deep Object Clone",
            "Data Manipulation",
            "Deep recursive structural cloning of nested objects, dictionaries, or data records.",
            &[
                "clone_deep",
                "deep_copy",
                "deep_clone",
                "deepcopy",
                "recursive_copy",
            ],
            vec![
                rec(
                    "typescript",
                    "lodash-es",
                    "bun add lodash-es",
                    "import { cloneDeep } from 'lodash-es';\nconst copy = cloneDeep(obj);",
                ),
                rec(
                    "python",
                    "copy (stdlib)",
                    "# Built-in standard library",
                    "import copy\ncloned = copy.deepcopy(obj)",
                ),
            ],
        ),
    ]
}
