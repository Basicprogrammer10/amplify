use std::time::{SystemTime, UNIX_EPOCH};

use afire::{Content, Response};

pub fn current_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn json_err(err: &str) -> Response {
    Response::new()
        .status(400)
        .text(format!(r#"{{"error": "{}"}}"#, err))
        .content(Content::JSON)
}
