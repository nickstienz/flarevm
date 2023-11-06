use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct StringPool {
    strings: Vec<Arc<str>>,
    string_indices: HashMap<Arc<str>, u32>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            string_indices: HashMap::new(),
        }
    }

    pub fn intern(&mut self, string: &str) -> u32 {
        if let Some(index) = self.string_indices.get(string) {
            *index
        } else {
            let index = self.strings.len() as u32;
            let string_arc: Arc<str> = string.into();
            self.strings.push(string_arc.clone());
            self.string_indices.insert(string_arc, index);
            index
        }
    }
}
