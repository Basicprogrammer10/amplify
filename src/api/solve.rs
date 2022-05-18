use std::process::{Command, Stdio};
use std::string::String;
use std::time::Instant;

use afire::{Content, Method, Response, Server};
use serde_json::json;

use crate::{App, Arc};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::GET, "/api/solve", move |_req| {
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
                "-e",
                &format!("TIMEOUT={}", &app.cfg.docker_timeout),
                &app.cfg.docker_image,
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();

        Response::new()
            .text(json!(
                {
                    "time": time.elapsed().as_millis() as u64,
                    "status": run.status.code().unwrap_or_default(),
                    "stdout": String::from_utf8_lossy(&run.stdout),
                    "stderr": String::from_utf8_lossy(&run.stderr)
                }
            ))
            .content(Content::JSON)
    });
}
