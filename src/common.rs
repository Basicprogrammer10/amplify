use std::time::{SystemTime, UNIX_EPOCH};

use afire::{Content, Response};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;

pub fn current_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn json_err(err: &str) -> Response {
    Response::new()
        .status(400)
        .text(json!({ "error": err }))
        .content(Content::JSON)
}

pub fn rand_str(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect::<String>()
}
