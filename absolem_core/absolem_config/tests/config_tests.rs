mod tests {
    use absolem_config::Config;
    use std::fs;
    use std::path::Path;

    const TEST_CONFIG_PATH: &str = "~/.absolem/test_config.toml";

    fn setup_test_env() {
        let test_path = Config::expand_path(TEST_CONFIG_PATH);
        if Path::new(&test_path).exists() {
            fs::remove_file(&test_path).unwrap();
        }
    }

    #[test]
    fn test_new_creates_default_config() {
        setup_test_env();
        let config = Config::new();
        assert!(config.editor_command.is_none());
    }

    #[test]
    fn test_set_and_view() {
        setup_test_env();
        let mut config = Config::new();
        config.set("editor_command", "vim");
        assert_eq!(config.editor_command, Some("vim".to_string()));
    }

    #[test]
    fn test_delete() {
        setup_test_env();
        let mut config = Config::new();
        config.set("editor_command", "vim");
        config.delete("editor_command");
        assert!(config.editor_command.is_none());
    }

    #[test]
    fn test_invalid_key() {
        setup_test_env();
        let mut config = Config::new();
        config.set("invalid_key", "value");
        assert!(config.editor_command.is_none());
    }
}