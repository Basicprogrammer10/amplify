use std::collections::HashMap;

use afire::{Content, Method, Response, Server};
use serde_json::json;

use crate::{
    common::{get_cookie, json_err},
    problems::PROBLEMS,
    App, Arc,
};

pub fn attach(server: &mut Server, app: Arc<App>) {
    server.route(Method::GET, "/api/profile", move |req| {
        // Get Session
        let session_id = match get_cookie(&req, "session") {
            Some(i) => i.value,
            None => return json_err("No Session!?"),
        };

        let db = app.db.lock();

        // Get User ID from Session
        let user_id = db
            .query_row(
                "SELECT user_id FROM sessions WHERE session_id=?",
                [session_id],
                |row| row.get::<_, String>(0),
            )
            .unwrap();

        // Get Signup Date
        let signup = db
            .query_row(
                "SELECT created FROM users WHERE users.id=?",
                [&user_id],
                |req| req.get::<_, u64>(0),
            )
            .unwrap();

        // Get Used Langs
        let mut langs = HashMap::new();
        let mut langs_query = db
            .prepare("SELECT language FROM solutions WHERE user_id=? AND state>=2")
            .unwrap();
        let mut langs_query = langs_query.query([&user_id]).unwrap();
        while let Some(i) = langs_query.next().unwrap() {
            *langs.entry(i.get::<_, String>(0).unwrap()).or_insert(0) += 1;
        }

        // Get Completed Problems
        let mut problems = Vec::new();
        let mut problems_query = db
            .prepare("SELECT problem_id, start_time, end_time FROM problems WHERE user_id=? AND state>=2 AND end_time IS NOT NULL;")
            .unwrap();
        let mut problem_query = problems_query.query([&user_id]).unwrap();
        while let Some(i) = problem_query.next().unwrap() {
            let name = PROBLEMS
                    .iter()
                    .find(|x| x.id() == i.get::<_, u64>(0).unwrap())
                    .unwrap()
                    .name();
            let time = i.get::<_, u64>(2).unwrap()- i.get::<_, u64>(1).unwrap();

            problems.push(
                json!({"name": name, "time": time})
            );
        }

        Response::new()
            .text(json!({"signup": signup, "langs": langs, "problems": problems, "totalProblems": PROBLEMS.len()}))
            .content(Content::JSON)
    });
}
