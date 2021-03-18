#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Log {
    #[serde(default = "_enable_file")]
    pub enable_file: bool,
    #[serde(default = "_file_directory")]
    pub file_directory: String,
    #[serde(default = "_level")]
    pub level: String,
}

fn _enable_file() -> bool {
    true
}

fn _file_directory() -> String {
    String::from("./logs")
}

fn _level() -> String {
    String::from("INFO")
}

impl Default for Log {
    fn default() -> Self {
        Self {
            enable_file: _enable_file(),
            file_directory: _file_directory(),
            level: _level(),
        }
    }
}
