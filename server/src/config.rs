use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::path::Path;
use std::str::FromStr;

pub mod guard;
pub mod log;
pub mod pool;
pub mod rate_limit;
pub mod redis;
pub mod token_files;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "_ip")]
    pub ip: IpAddr,
    #[serde(default = "_port")]
    pub port: u16,
    #[serde(default)]
    pub token_config: token_files::TokenFiles,
    #[serde(default)]
    pub rate_limit: rate_limit::RateLimit,
    #[serde(default)]
    pub guard: guard::Guard,
    #[serde(default)]
    pub pool: pool::Pool,
    #[serde(default)]
    pub log: log::Log,
    #[serde(default)]
    pub redis: redis::Redis,
}

fn _ip() -> IpAddr {
    IpAddr::from([0, 0, 0, 0])
}

fn _port() -> u16 {
    8181
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: _ip(),
            port: _port(),
            token_config: token_files::TokenFiles::default(),
            rate_limit: rate_limit::RateLimit::default(),
            guard: guard::Guard::default(),
            pool: pool::Pool::default(),
            log: log::Log::default(),
            redis: redis::Redis::default(),
        }
    }
}

pub fn load<P: AsRef<Path>>(path: P) -> Config {
    match std::fs::File::open(path) {
        Ok(f) => match serde_yaml::from_reader(f) {
            Ok(cfg) => return cfg,
            Err(e) => error!(e),
        },
        Err(e) => error!(e),
    }
    error!("Can't load config file, use default config.");
    Config::default()
}

pub fn reload<P: AsRef<Path>>(cfg: &mut Config, path: P) {
    std::mem::replace(cfg, load(path));
}

pub fn save<P: AsRef<Path>>(path: P, cfg: &Config) -> bool {
    match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
    {
        Ok(f) => match serde_yaml::to_writer(f, cfg) {
            Ok(_) => return true,
            Err(e) => error!(e),
        },
        Err(e) => error!(e),
    }
    error!("Can't save config file.");
    false
}

pub fn load_and_save<P: AsRef<Path>>(path: P) -> Config {
    let cfg = load(path.as_ref());
    save(path.as_ref(), &cfg);
    cfg
}

pub fn display(cfg: &Config) {
    info!("Listening address: {}:{}", cfg.ip, cfg.port);
    info!("Log level: {}", cfg.log.level);
    info!("Log to file: {}", cfg.log.enable_file);
    info!("Log file directory: {}", cfg.log.file_directory);
    info!("Rate limiter interval: {}ms", cfg.rate_limit.interval);
    info!("Rate limiter max burst: {}", cfg.rate_limit.max_burst);
    info!("Client tokens file path: {}", cfg.token_config.client);
    info!("Server tokens file path: {}", cfg.token_config.server);
    info!("Admin tokens file path: {}", cfg.token_config.admin);
    info!("Guard ban time: {}h", cfg.guard.ban_time);
    info!("Guard kick count: {}", cfg.guard.kick_count);
    info!("Guard record file path: {}", cfg.guard.record_file);
}
