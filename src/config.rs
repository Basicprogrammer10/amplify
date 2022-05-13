use std::time::Duration;

use simple_config_parser;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub database: String,

    pub github_app_id: String,
    pub github_app_secret: String,
    pub ext_url: String,

    pub req_duration: Duration,
}

impl Config {
    pub fn load(path: &str) -> Option<Self> {
        let cfg = simple_config_parser::Config::new().file(path).ok()?;
        Some(Self {
            host: cfg.get("host").ok()?,
            port: cfg.get("port").ok()?,
            database: cfg.get("database").ok()?,
            github_app_id: cfg.get("github_app_id").ok()?,
            github_app_secret: cfg.get("github_app_secret").ok()?,
            ext_url: cfg.get("ext_url").ok()?,
            req_duration: Duration::from_secs(cfg.get("req_duration").ok()?),
        })
    }
}
