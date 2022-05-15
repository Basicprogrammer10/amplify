use std::time::{SystemTime, UNIX_EPOCH};

use afire::{Content, Cookie, Request, Response};
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

pub fn get_cookie(req: &Request, name: &str) -> Option<Cookie> {
    req.cookies
        .iter()
        .find(|x| x.name == name)
        .map(|x| x.to_owned())
}
