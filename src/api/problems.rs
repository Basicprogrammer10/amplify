use crate::{
    common::{get_cookie, json_err},
    problems::PROBLEMS,
    App, Arc,
};

use afire::{Content, Method, Response, Server};
use serde_json::json;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    let problems = PROBLEMS
        .iter()
        .map(|x| (x.name(), x.id(), 0))
        .collect::<Vec<_>>();

    server.route(Method::GET, "/api/problems", move |req| {
        // Get Session
        let session_id = match get_cookie(&req, "session") {
            Some(i) => i.value,
            None => return json_err("No Session!?"),
        };

        let db = app.db.lock();

        // Get userid from session
        let session = db
            .query_row(
                "SELECT Count(*), user_id FROM sessions WHERE session_id = ?;",
                [session_id],
                |row| Ok((row.get::<_, u64>(0)? > 0, row.get::<_, String>(1)?)),
            )
            .unwrap();

        if !session.0 {
            return json_err("Invalid Session");
        }

        // Get users problem status
        let mut problems = problems.clone();
        let mut user_problems = db
            .prepare(include_str!("../sql/query_problem_status.sql"))
            .unwrap();
        while let Some(i) = user_problems.query([&session.1]).unwrap().next().unwrap() {
            let id = i.get::<_, u64>(0).unwrap();
            let status = i.get::<_, u64>(1).unwrap();

            if let Some(i) = problems.iter_mut().find(|x| x.1 == id) {
                i.2 = status.max(i.2);
            }
        }

        // Gen Json
        let out = problems
            .iter()
            .map(|x| json!({"name": x.0, "id": x.1, "date": "todo", "status": x.2}).to_string())
            .collect::<Vec<_>>()
            .join(",");

        // Send Response
        Response::new()
            .text(format!("[{}]", out))
            .content(Content::JSON)
    });
}
