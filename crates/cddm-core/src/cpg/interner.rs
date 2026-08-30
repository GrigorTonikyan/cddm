#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

/// Unique 32-bit identifier for an interned string or symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SymbolId(pub u32);

/// Thread-safe in-memory string and symbol interner.
/// Provides O(1) integer-based equality checks and eliminates repeated string allocations.
#[derive(Debug)]
pub struct SymbolInterner {
    map: RwLock<HashMap<String, SymbolId>>,
    strings: RwLock<Vec<String>>,
}

impl Default for SymbolInterner {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolInterner {
    /// Creates a new empty symbol interner.
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
            strings: RwLock::new(Vec::new()),
        }
    }

    /// Interns a string slice, returning its unique `SymbolId`.
    pub fn intern(&self, s: &str) -> SymbolId {
        {
            let map = self.map.read().unwrap();
            if let Some(&id) = map.get(s) {
                return id;
            }
        }

        let mut map = self.map.write().unwrap();
        if let Some(&id) = map.get(s) {
            return id;
        }

        let mut strings = self.strings.write().unwrap();
        let id = SymbolId(strings.len() as u32);
        strings.push(s.to_string());
        map.insert(s.to_string(), id);
        id
    }

    /// Resolves a `SymbolId` back to its original string representation.
    pub fn resolve(&self, id: SymbolId) -> Option<String> {
        let strings = self.strings.read().unwrap();
        strings.get(id.0 as usize).cloned()
    }

    /// Returns the total number of unique symbols interned.
    pub fn len(&self) -> usize {
        self.strings.read().unwrap().len()
    }

    /// Returns true if no symbols have been interned.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears all interned symbols.
    pub fn clear(&self) {
        self.map.write().unwrap().clear();
        self.strings.write().unwrap().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intern_and_resolve() {
        let interner = SymbolInterner::new();
        let id1 = interner.intern("calculate_total");
        let id2 = interner.intern("calculate_total");
        let id3 = interner.intern("user_id");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
        assert_eq!(interner.len(), 2);

        assert_eq!(interner.resolve(id1), Some("calculate_total".to_string()));
        assert_eq!(interner.resolve(id3), Some("user_id".to_string()));
        assert_eq!(interner.resolve(SymbolId(999)), None);
    }

    #[test]
    fn test_intern_thread_safety() {
        use std::sync::Arc;
        use std::thread;

        let interner = Arc::new(SymbolInterner::new());
        let mut handles = Vec::new();

        for i in 0..8 {
            let interner_clone = Arc::clone(&interner);
            handles.push(thread::spawn(move || {
                for j in 0..100 {
                    let sym = format!("symbol_{}_{}", i % 2, j % 10);
                    let _ = interner_clone.intern(&sym);
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert!(interner.len() <= 20);
    }
}
