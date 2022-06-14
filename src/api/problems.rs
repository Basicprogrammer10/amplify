use crate::{
    common::{current_epoch, get_cookie, json_err},
    problems::PROBLEMS,
    App, Arc,
};

use afire::{Content, Method, Response, Server};
use chrono::NaiveDateTime;
use serde_json::json;

pub fn attach(server: &mut Server, app: Arc<App>) {
    let problems = PROBLEMS
        .iter()
        .map(|x| (x.name(), x.id(), x.time(), 0))
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
        let mut query = user_problems.query([&session.1]).unwrap();
        while let Some(i) = query.next().unwrap() {
            let id = i.get::<_, u64>(0).unwrap();
            let status = i.get::<_, u64>(1).unwrap();

            if let Some(i) = problems.iter_mut().find(|x| x.1 == id) {
                i.3 = status.max(i.3);
            }
        }

        // Gen Json
        let time = 1654056001;
        let out = problems
            .iter()
            .filter(|x| x.2 <= time)
            .map(|x| {
                let data = NaiveDateTime::from_timestamp(x.2 as i64, 0)
                    .format("%b %e %Y")
                    .to_string();
                json!({"name": x.0, "id": x.1, "date": data, "status": x.3}).to_string()
            })
            .collect::<Vec<_>>()
            .join(",");

        // Send Response
        Response::new()
            .text(format!("[{}]", out))
            .content(Content::JSON)
    });
}
