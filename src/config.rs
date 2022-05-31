use std::time::Duration;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub database: String,

    pub github_app_id: String,
    pub github_app_secret: String,
    pub ext_url: String,
    pub req_duration: Duration,

    pub docker_command: String,
    pub docker_timeout: String,
    pub tmp_folder: String,
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
            docker_command: cfg.get("docker_command").ok()?,
            docker_timeout: cfg.get("docker_timeout").ok()?,
            tmp_folder: cfg.get("tmp_folder").ok()?,
        })
    }
}
