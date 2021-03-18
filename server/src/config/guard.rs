#[derive(Debug, Serialize, Deserialize)]
pub struct Guard {
    #[serde(default = "_kick_count")]
    pub kick_count: i32,
    #[serde(default = "_ban_time")]
    pub ban_time: i32, // 单位：h
}

fn _kick_count() -> i32 {
    10
}

fn _ban_time() -> i32 {
    24
}

impl Default for Guard {
    fn default() -> Self {
        Self {
            kick_count: _kick_count(),
            ban_time: _ban_time(),
        }
    }
}
