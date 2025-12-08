use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const CONFIG_PATH: &str = "~/.absolem/config.toml";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub editor_command: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        let path = Self::expand_path(CONFIG_PATH);
        if !Path::new(&path).exists() {
            fs::create_dir_all(Path::new(&path).parent().unwrap()).unwrap();
            OpenOptions::new()
                .create(true)
                .truncate(false)
                .write(true)
                .open(&path)
                .unwrap();
            return Config::default();
        }
        let mut file = OpenOptions::new().read(true).open(&path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        toml::from_str(&content).unwrap_or_default()
    }

    pub fn view(&self) {
        println!("\x1b[32mEditor Command:\x1b[0m {:?}", self.editor_command);
    }

    pub fn set(&mut self, key: &str, value: &str) {
        match key {
            "editor_command" => self.editor_command = Some(value.to_string()),
            _ => {
                eprintln!(
                    "Error: '{}' is not a valid key. Available keys are: editor_command",
                    key
                );
                return;
            }
        }
        self.save();
    }

    pub fn delete(&mut self, key: &str) {
        match key {
            "editor_command" => self.editor_command = None,
            _ => {
                eprintln!(
                    "Error: '{}' is not a valid key. Available keys are: editor_command",
                    key
                );
                return;
            }
        }
        self.save();
    }

    fn save(&self) {
        let path = Self::expand_path(CONFIG_PATH);
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path)
            .unwrap();
        file.write_all(toml::to_string(&self).unwrap().as_bytes())
            .unwrap();
    }

    fn expand_path(path: &str) -> String {
        path.replace("~", std::env::var("HOME").unwrap().as_str())
    }
}
