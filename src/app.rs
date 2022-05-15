use parking_lot::Mutex;
use rusqlite::Connection;

use crate::config::Config;

pub struct App {
    pub cfg: Config,
    pub db: Mutex<Connection>,
}

impl App {
    pub fn new() -> Self {
        let cfg = Config::load("config.cfg").unwrap();

        Self {
            db: Mutex::new(Connection::open(&cfg.database).unwrap()),
            cfg,
        }
    }
}
