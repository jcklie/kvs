//! A simple key-value store that maps strings to strings.
#![deny(missing_docs)]

use std::collections::HashMap;

/// A simple hash map backed key-value store that maps strings to strings.
#[derive(Default)]
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    /// Creates a new, empty, key-value store
    pub fn new() -> KvStore {
        KvStore {
            data: HashMap::new(),
        }
    }

    /// Set the value of a string key to a string
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let mut kvs = KvStore::new();
    /// kvs.set("foo".to_owned(), "bar".to_owned());
    ///
    /// assert_eq!(kvs.get("foo".to_owned()), Some("bar".to_owned()));
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Get the string value of the a string key. If the key does not exist, return `None`.
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let mut kvs = KvStore::new();
    /// kvs.set("foo".to_owned(), "bar".to_owned());
    ///
    /// assert_eq!(kvs.get("foo".to_owned()), Some("bar".to_owned()));
    /// assert_eq!(kvs.get("baz".to_owned()),  None);
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.data.get(&key).map(String::to_owned)
    }

    /// Remove a given key.
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let mut kvs = KvStore::new();
    /// kvs.set("foo".to_owned(), "bar".to_owned());
    ///
    /// assert_eq!(kvs.get("foo".to_owned()), Some("bar".to_owned()));
    /// kvs.remove("foo".to_owned());
    /// assert_eq!(kvs.get("foo".to_owned()),  None);
    /// ```
    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
}
