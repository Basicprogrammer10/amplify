use std::io::Write;
use std::process::{Command, Stdio};
use std::string::String;
use std::time::Instant;

use afire::{Content, Method, Response, Server};
use serde_json::{from_str, json, Value};
use tempfile;
use urlencoding;

use super::langs::LANGS;
use crate::{common::json_err, App, Arc};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::POST, "/api/solve", move |req| {
        let body = from_str::<Value>(&String::from_utf8_lossy(&req.body)).unwrap();
        let code = body
            .get("code")
            .expect("No code defined")
            .as_str()
            .expect("Code is in a string");
        let raw_languge = body
            .get("lang")
            .expect("No languge defined")
            .as_str()
            .expect("Languge is in a string");

        let languge = match LANGS.get(raw_languge) {
            Some(i) => i,
            None => return json_err("Undefined Languge"),
        };

        let args = "HELLO WORLD";

        let mut code_file = tempfile::NamedTempFile::new_in("data/tmp").unwrap();
        code_file.write_all(code.as_bytes()).unwrap();

        let time = Instant::now();
        let run = Command::new(&app.cfg.docker_command)
            .args([
                "run",
                "--rm",
                "--cap-drop=ALL",
                "--security-opt=no-new-privileges",
                "--net",
                "none",
                "--memory",
                "128m",
                "--memory-swap",
                "256m",
                "--pids-limit",
                "512",
                "-v",
                &format!(
                    "{}:/runner/{}",
                    code_file.path().to_string_lossy(),
                    languge.1
                ),
                "-e",
                &format!("TIMEOUT={}", &app.cfg.docker_timeout),
                "-e",
                &format!("ARGS={}", urlencoding::encode(args)),
                &languge.0,
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();
        let time = time.elapsed().as_millis() as u64;

        Response::new()
            .text(json!(
                {
                    "time": time,
                    "status": run.status.code().unwrap_or_default(),
                    "stdout": String::from_utf8_lossy(&run.stdout),
                    "stderr": String::from_utf8_lossy(&run.stderr)
                }
            ))
            .content(Content::JSON)
    });
}
