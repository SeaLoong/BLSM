#[derive(Debug, Serialize, Deserialize)]
pub struct Pool {
    #[serde(default = "_size")]
    pub size: usize,
    #[serde(default = "_refresh_interval")]
    pub refresh_interval: i32, // 单位：min
    #[serde(default = "_validity")]
    pub validity: i32,
}

fn _size() -> usize {
    10000
}

fn _refresh_interval() -> i32 {
    10
}

fn _validity() -> i32 {
    3
}

impl Default for Pool {
    fn default() -> Self {
        Self {
            size: _size(),
            refresh_interval: _refresh_interval(),
            validity: _validity(),
        }
    }
}
