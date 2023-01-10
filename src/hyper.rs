/**
 * HyperDB
 *
 * In-memory hyper-fast key-value store.
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 */
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HyperStore {
    data: HashMap<String, String>,
    file: String,
}

#[allow(dead_code)]
impl HyperStore {
    pub fn new(file: &str) -> HyperStore {
        HyperStore {
            data: HashMap::new(),
            file: file.to_string(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> String {
        match self.data.get(key) {
            Some(value) => value.to_string(),
            None => String::from(""),
        }
    }

    pub fn has(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
    }

    pub fn all(&self) -> String {
        format!("{:#?}", self.data)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn print_all(&self) {
        println!("{:#?}", self.data)
    }

    pub fn get_file(&self) -> &String {
        &self.file
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_FILE: &str = "store.hyper";

    #[test]
    fn has_file() {
        let hs = HyperStore::new(DEFAULT_FILE);
        assert_eq!(hs.get_file(), DEFAULT_FILE);
    }

    #[test]
    fn start_from_empty() {
        let hs = HyperStore::new(DEFAULT_FILE);
        assert_eq!(hs.is_empty(), true);
    }

    #[test]
    fn start_len_zero() {
        let hs = HyperStore::new(DEFAULT_FILE);
        assert_eq!(hs.len(), 0);
    }

    #[test]
    fn key_not_present() {
        let hs = HyperStore::new(DEFAULT_FILE);
        assert_eq!(hs.has("hyper"), false);
    }

    #[test]
    fn key_is_stored() {
        let mut hs = HyperStore::new(DEFAULT_FILE);
        hs.set("hyper", "db");
        assert_eq!(hs.has("hyper"), true);
    }

    #[test]
    fn key_is_deleted() {
        let mut hs = HyperStore::new(DEFAULT_FILE);
        hs.set("hyper", "db");
        hs.delete("hyper");
        assert_eq!(hs.has("hyper"), false);
    }

    #[test]
    fn value_is_stored() {
        let mut hs = HyperStore::new(DEFAULT_FILE);
        hs.set("hyper", "db");
        assert_eq!(hs.get("hyper"), "db");
    }

    #[test]
    fn not_empty() {
        let mut hs = HyperStore::new(DEFAULT_FILE);
        hs.set("hyper", "db");
        assert_ne!(hs.is_empty(), true);
    }

    #[test]
    fn len_not_zero() {
        let mut hs = HyperStore::new(DEFAULT_FILE);
        hs.set("hyper", "db");
        assert_ne!(hs.len(), 0);
    }

    #[test]
    fn it_clears() {
        let mut hs = HyperStore::new(DEFAULT_FILE);
        hs.set("hyper", "db");
        hs.clear();
        assert_eq!(hs.is_empty(), true);
    }
}
