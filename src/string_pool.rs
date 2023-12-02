use std::collections::HashMap;

#[derive(Debug)]
pub struct StringPool {
    strings: Vec<String>,
    index_map: HashMap<String, u32>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            index_map: HashMap::new(),
        }
    }

    pub fn intern(&mut self, string: &str) -> u32 {
        if let Some(index) = self.index_map.get(string) {
            *index
        } else {
            let index = self.strings.len() as u32;
            self.strings.push(string.to_string());
            self.index_map.insert(string.to_string(), index);
            index
        }
    }

    pub fn get_string(&self, index: u32) -> &str {
        self.strings
            .get(index as usize)
            .map(String::as_str)
            .unwrap()
    }

    pub fn clear(&mut self) {
        self.strings.clear();
        self.index_map.clear();
    }
}
