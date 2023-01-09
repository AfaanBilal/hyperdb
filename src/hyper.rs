/**
 * HyperDB
 *
 * In-memory hyper-fast key-value store.
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 */
use std::collections::HashMap;

#[derive(Debug)]
pub struct HyperStore {
    data: HashMap<String, String>,
    file: String,
}

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

    pub fn get(&self, key: &str) -> &str {
        match self.data.get(key) {
            Some(value) => value,
            None => "",
        }
    }

    pub fn has(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
    }

    pub fn len(&mut self) -> usize {
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
