use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Stat {
    pub total_files: usize,
    pub extension_map: HashMap<String, usize>,
    pub file_names: Vec<String>,
}
impl Stat {
    pub fn new() -> Stat {
        Stat {
            total_files: 0usize,
            file_names: Vec::new(),
            extension_map: HashMap::new(),
        }
    }
    /// Record the amount of each file type
    pub fn add_extension(&mut self, ext: String) {
        if self.extension_map.contains_key(&ext) {
            let cur_total = self.extension_map.get(&ext).unwrap();
            let new_total = cur_total + 1;
            self.extension_map.insert(ext, new_total);
        } else {
            self.extension_map.insert(ext, 1);
        }
        self.total_files += 1;
    }
    pub fn add_file(&mut self, file_name: String) {
        self.file_names.push(file_name);
    }
    pub fn sort_files(&mut self) {
        self.file_names.sort_unstable();
    }
}
