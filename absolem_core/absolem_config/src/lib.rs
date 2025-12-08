use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const CONFIG_PATH: &str = "~/.absolem/config.toml";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub editor_command: Option<String>,
    config_path: String,
}

impl Config {
    pub fn new() -> Self {
        Self::new_with_path(CONFIG_PATH)
    }

    pub fn new_with_path(path: &str) -> Self {
        let path = Self::expand_path(path);
        if !Path::new(&path).exists() {
            fs::create_dir_all(Path::new(&path).parent().unwrap()).unwrap();
            OpenOptions::new()
                .create(true)
                .truncate(false)
                .write(true)
                .open(&path)
                .unwrap();
            return Config {
                editor_command: None,
                config_path: path,
            };
        }
        let mut file = OpenOptions::new().read(true).open(&path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let mut config: Config = toml::from_str(&content).unwrap_or_default();
        config.config_path = path;
        config
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
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.config_path)
            .unwrap();
        file.write_all(toml::to_string(&self).unwrap().as_bytes())
            .unwrap();
    }

    pub fn expand_path(path: &str) -> String {
        path.replace("~", std::env::var("HOME").unwrap().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use serial_test::serial;
    use std::fs;
    use std::path::Path;

    fn setup_test_env(test_path: &str) {
        let test_path = Config::expand_path(test_path);
        if Path::new(&test_path).exists() {
            fs::remove_file(&test_path).unwrap();
        }
    }

    #[test]
    #[serial]
    fn test_new_creates_default_config() {
        let test_path = "/tmp/test_config_new.toml";
        setup_test_env(test_path);
        let config = Config::new_with_path(test_path);
        assert!(config.editor_command.is_none());
    }

    #[test]
    #[serial]
    fn test_set_and_view() {
        let test_path = "/tmp/test_config_set.toml";
        setup_test_env(test_path);
        let mut config = Config::new_with_path(test_path);
        config.set("editor_command", "vim");
        assert_eq!(config.editor_command, Some("vim".to_string()));
    }

    #[test]
    #[serial]
    fn test_delete() {
        let test_path = "/tmp/test_config_delete.toml";
        setup_test_env(test_path);
        let mut config = Config::new_with_path(test_path);
        config.set("editor_command", "vim");
        config.delete("editor_command");
        assert!(config.editor_command.is_none());
    }

    #[test]
    #[serial]
    fn test_invalid_key() {
        let test_path = "/tmp/test_config_invalid.toml";
        setup_test_env(test_path);
        let mut config = Config::new_with_path(test_path);
        config.set("invalid_key", "value");
        assert!(config.editor_command.is_none());
    }
}
