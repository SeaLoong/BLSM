use config::{Config, FileFormat, Value};
use log::error;
use std::collections::HashMap;
use std::net::IpAddr;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Settings {
    pub debug: bool,
    pub path: String,
    pub ip: IpAddr,
    pub port: u16,
    pub token_files: TokenFiles,
    pub rate_limit: RateLimit,
    pub guard: Guard,
    pub log: Log,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            debug: false,
            path: String::from("config.yml"),
            ip: IpAddr::from([0, 0, 0, 0]),
            port: 8181,
            token_files: TokenFiles::default(),
            rate_limit: RateLimit::default(),
            guard: Guard::default(),
            log: Log::default(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct TokenFiles {
    pub client: String,
    pub server: String,
    pub admin: String,
}

impl Default for TokenFiles {
    fn default() -> Self {
        TokenFiles {
            client: String::from("./client_tokens.txt"),
            server: String::from("./server_tokens.txt"),
            admin: String::from("./admin_tokens.txt"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct RateLimit {
    pub interval: u32,
    pub max_burst: u32,
}

impl Default for RateLimit {
    fn default() -> Self {
        RateLimit {
            interval: 10000,
            max_burst: 6,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Guard {
    pub kick_count: i32,
    pub ban_time: i64,
    pub record_file: String,
}

impl Default for Guard {
    fn default() -> Self {
        Guard {
            kick_count: 10,
            ban_time: 24,
            record_file: String::from("./guard_record"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Log {
    pub enable_console: bool,
    pub enable_file: bool,
    pub file_directory: String,
    pub level: String,
}

impl Default for Log {
    fn default() -> Self {
        Log {
            enable_console: true,
            enable_file: true,
            file_directory: String::from("./logs"),
            level: String::from("INFO"),
        }
    }
}

fn get_str_from_map(map: &HashMap<String, Value>, k: &str) -> Option<String> {
    map.get(k).and_then(|v| v.to_owned().into_str().ok())
}

fn get_int_from_map(map: &HashMap<String, Value>, k: &str) -> Option<i64> {
    map.get(k).and_then(|v| v.to_owned().into_int().ok())
}

fn get_bool_from_map(map: &HashMap<String, Value>, k: &str) -> Option<bool> {
    map.get(k).and_then(|v| v.to_owned().into_bool().ok())
}

fn get_map_from_map(map: &HashMap<String, Value>, k: &str) -> Option<HashMap<String, Value>> {
    map.get(k).and_then(|v| v.to_owned().into_table().ok())
}

fn get_str(matches: &clap::ArgMatches, cfg: &config::Config, k: &str) -> Option<String> {
    let s = matches
        .value_of(k)
        .map(|s| s.to_string())
        .or_else(|| cfg.get_str(k).ok())?;
    if s.trim().is_empty() {
        None
    } else {
        Some(s)
    }
}

fn get_int(matches: &clap::ArgMatches, cfg: &config::Config, k: &str) -> Option<i64> {
    matches
        .value_of(k)
        .and_then(|s| i64::from_str(s).ok())
        .or_else(|| cfg.get_int(k).ok())
}

impl Settings {
    pub fn new(matches: &clap::ArgMatches) -> Result<(Settings, Config), config::ConfigError> {
        let mut settings = Settings::default();
        settings.debug = matches.is_present("debug");

        let path = Path::new(
            matches
                .value_of("config")
                .unwrap_or_else(|| settings.path.as_str()),
        );
        if !path.is_file() && std::fs::write(path, DEFAULT_CONFIG_FILE).is_err() {
            error!("Can't create default config file 'config.yml', use default config and command line args.");
        }

        let mut cfg = Config::default();
        cfg.merge(
            config::File::from(path)
                .required(false)
                .format(FileFormat::Yaml),
        )?;
        Ok((settings, cfg))
    }
    pub fn done(&mut self, matches: clap::ArgMatches, cfg: Config) {
        let cfg = &cfg;

        if let Some(s) = get_str(&matches, cfg, "ip") {
            self.ip = IpAddr::from_str(&s).expect("Can't parse IP Address!");
        }

        if let Some(x) = get_int(&matches, cfg, "port") {
            self.port = x as u16;
        }

        if let Ok(map) = cfg.get_table("token_files") {
            if let Some(x) = get_str_from_map(&map, "client") {
                self.token_files.client = x;
            }
            if let Some(x) = get_str_from_map(&map, "server") {
                self.token_files.server = x;
            }
            if let Some(x) = get_str_from_map(&map, "admin") {
                self.token_files.admin = x;
            }
        }

        if let Ok(map) = cfg.get_table("rate_limit") {
            if let Some(x) = get_int_from_map(&map, "interval") {
                self.rate_limit.interval = x as u32;
            }
            if let Some(x) = get_int_from_map(&map, "max_burst") {
                self.rate_limit.max_burst = x as u32;
            }
        }

        if let Ok(map) = cfg.get_table("guard") {
            if let Some(x) = get_int_from_map(&map, "kick_count") {
                self.guard.kick_count = x as i32;
            }
            if let Some(x) = get_int_from_map(&map, "ban_time") {
                self.guard.ban_time = x;
            }
            if let Some(x) = get_str_from_map(&map, "record_file") {
                self.guard.record_file = x;
            }
        }

        if let Ok(map) = cfg.get_table("log") {
            if let Some(x) = get_bool_from_map(&map, "enable_console") {
                self.log.enable_console = x;
            }
            if let Some(x) = get_bool_from_map(&map, "enable_file") {
                self.log.enable_file = x;
            }
            if let Some(x) = get_str_from_map(&map, "file_directory") {
                self.log.file_directory = x;
            }
            if let Some(x) = get_str_from_map(&map, "level") {
                self.log.level = x;
            }
        }
    }
}

const DEFAULT_CONFIG_FILE: &str = "\
ip: 0.0.0.0
port: 8181
token_files:
  client: ./client_tokens.txt
  server: ./server_tokens.txt
  admin: ./admin_tokens.txt
rate_limit:
  interval: 10000
  max_burst: 6
guard:
  kick_count: 10
  ban_time: 24
  record_file: ./guard_record
log:
  enable_console: true
  enable_file: true
  file_directory: ./logs
  level: INFO
";
