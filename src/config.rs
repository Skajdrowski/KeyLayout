use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub rectangle_color: u32,
}

impl Config {
    pub fn load_from_file(file_path: &str) -> Self {
        let contents = fs::read_to_string(file_path).unwrap_or_default();
        serde_ini::from_str(&contents).unwrap_or(Config {
            rectangle_color: 0xFF808080,
        })
    }

    pub fn save_to_file(&self, file_path: &str) {
        let contents = serde_ini::to_string(self).expect("Failed to serialize config");
        fs::write(file_path, contents).expect("Failed to write to file");
    }
}