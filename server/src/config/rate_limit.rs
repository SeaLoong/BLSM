#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimit {
    #[serde(default = "_interval")]
    pub interval: u32, // 单位：ms
    #[serde(default = "_max_burst")]
    pub max_burst: u32,
}

fn _interval() -> u32 {
    10000
}

fn _max_burst() -> u32 {
    6
}

impl Default for RateLimit {
    fn default() -> Self {
        Self {
            interval: _interval(),
            max_burst: _max_burst(),
        }
    }
}
