use std::fs;

pub struct ScriptManager;

impl ScriptManager {
    pub fn new() -> Self {
        ScriptManager
    }

    pub fn load_script(&self, file_name: &str) -> String {
        fs::read_to_string(file_name).unwrap_or_else(|_| String::from(""))
    }

    pub fn save_script(&self, file_name: &str, content: &str) {
        fs::write(file_name, content).expect("Failed to save script");
    }
}
