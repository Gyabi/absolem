pub struct AbsolemCore;

impl AbsolemCore {
    pub fn new() -> Self {
        AbsolemCore
    }

    pub fn call_shell_script(&self, path: Option<String>, script_args: Vec<String>) {
        mock_call(path, script_args);
    }

    pub fn register_shell_script(&self, script_path: String, path: Option<String>) {
        mock_register(script_path, path);
    }

    pub fn create_shell_script(&self, path: Option<String>, open: bool) {
        mock_create(path, open);
    }

    pub fn record_shell_script(&self, disable_replace_absolute_path: bool) {
        mock_record(disable_replace_absolute_path);
    }

    pub fn move_shell_script(&self, before_path: String, after_path: String) {
        mock_move(before_path, after_path);
    }

    pub fn delete_shell_script(&self, path: String) {
        mock_delete(path);
    }

    pub fn print_shell_script(&self, path: Option<String>) {
        mock_print(path);
    }

    pub fn list_shell_scripts(&self) {
        mock_list();
    }

    pub fn config_view(&self) {
        mock_config_view();
    }

    pub fn config_set(&self, key: String, value: String) {
        mock_config_set(key, value);
    }

    pub fn config_delete(&self, key: String) {
        mock_config_delete(key);
    }
}


fn mock_call(path: Option<String>, script_args: Vec<String>) {
    println!("Mock call: {:?}, args: {:?}", path, script_args);
}

fn mock_register(script_path: String, path: Option<String>) {
    println!("Mock register: {}, path: {:?}", script_path, path);
}

fn mock_create(path: Option<String>, open: bool) {
    println!("Mock create: {:?}, open: {}", path, open);
}

fn mock_record(disable_replace_absolute_path: bool) {
    println!("Mock record: disable_replace_absolute_path: {}", disable_replace_absolute_path);
}

fn mock_move(before_path: String, after_path: String) {
    println!("Mock move: {} -> {}", before_path, after_path);
}

fn mock_delete(path: String) {
    println!("Mock delete: {}", path);
}

fn mock_print(path: Option<String>) {
    println!("Mock print: {:?}", path);
}

fn mock_list() {
    println!("Mock list");
}

fn mock_config_view() {
    println!("Mock config view");
}

fn mock_config_set(key: String, value: String) {
    println!("Mock config set: {} = {}", key, value);
}

fn mock_config_delete(key: String) {
    println!("Mock config delete: {}", key);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
