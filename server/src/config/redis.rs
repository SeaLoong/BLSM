#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Redis {
    #[serde(default = "_connection_parameter")]
    pub connection_parameter: String,
    #[serde(default = "_timeout")]
    pub timeout: i32,
}

fn _connection_parameter() -> String {
    String::from("redis://127.0.0.1:6379")
}

fn _timeout() -> i32 {
    30
}

impl Default for Redis {
    fn default() -> Self {
        Self {
            connection_parameter: _connection_parameter(),
            timeout: _timeout(),
        }
    }
}
