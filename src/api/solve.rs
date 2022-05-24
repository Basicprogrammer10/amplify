use std::io::Write;
use std::process::{Command, Stdio};
use std::string::String;
use std::time::Instant;

use afire::{Content, Method, Response, Server};
use serde_json::{from_str, json, Value};

use super::langs::LANGS;
use crate::{
    common::{get_cookie, json_err},
    problems::PROBLEMS,
    App, Arc,
};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::POST, "/api/solve", move |req| {
        // Get session
        let session_id = match get_cookie(&req, "session") {
            Some(i) => i.value,
            None => return json_err("No Session!?"),
        };

        // Get some values from the JSON body
        let body = from_str::<Value>(&String::from_utf8_lossy(&req.body)).unwrap();
        let code = body
            .get("code")
            .expect("No code defined")
            .as_str()
            .expect("Code is in a string");
        let raw_language = body
            .get("lang")
            .expect("No language defined")
            .as_str()
            .expect("Languge is in a string");
        let raw_problem = body
            .get("problem")
            .expect("No problem defined")
            .as_u64()
            .expect("Problem is not an integer (u64)");

        let language = match LANGS.get(raw_language) {
            Some(i) => i,
            None => return json_err("Undefined Languge"),
        };

        let problem = match PROBLEMS.iter().find(|x| x.id() == raw_problem) {
            Some(i) => i,
            None => return json_err("Undefined Problem"),
        };

        // Get seed from session id
        let seed = app
            .db
            .lock()
            .query_row(
                "SELECT id FROM sessions JOIN users ON sessions.user_id = id WHERE session_id = ?",
                [session_id],
                |row| row.get::<_, String>(0),
            )
            .expect("Invalid Session")
            .parse()
            .unwrap();

        // Gen test cases
        let args = problem.gen(seed);

        // Write code to a temp file
        let mut code_file = tempfile::NamedTempFile::new_in("data/tmp").unwrap();
        code_file.write_all(code.as_bytes()).unwrap();

        // Build and run in a docker container
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
                    language.1
                ),
                "-e",
                &format!("TIMEOUT={}", &app.cfg.docker_timeout),
                "-e",
                &format!("ARGS={}", urlencoding::encode(&args)),
                &language.0,
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();
        let time = time.elapsed().as_millis() as u64;

        // Send response
        let out = String::from_utf8_lossy(&run.stdout);
        let check = problem.check(seed);

        Response::new()
            .text(json!(
                {
                    "time": time,
                    "status": run.status.code().unwrap_or_default(),
                    "stdout": out,
                    "stderr": String::from_utf8_lossy(&run.stderr),
                    "success": check == out.trim(),
                    "input": args,
                    "expected": check
                }
            ))
            .content(Content::JSON)
    });
}
