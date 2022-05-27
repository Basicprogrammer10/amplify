use afire::{Content, Method, Response, Server};
use rusqlite::params;
use serde_json::json;

use crate::{
    common::{get_cookie, json_err},
    problems::PROBLEMS,
    App, Arc,
};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::GET, "/api/problem/{id}", move |req| {
        // Get Session
        let session_id = match get_cookie(&req, "session") {
            Some(i) => i.value,
            None => return json_err("No Session!?"),
        };

        let id = req
            .path_param("id")
            .unwrap()
            .parse::<u64>()
            .expect("Id is not a u64");

        let problem = PROBLEMS
            .iter()
            .find(|x| x.id() == id)
            .expect("Problem Not Found");

        let status = app
            .db
            .lock()
            .query_row(
                include_str!("../sql/query_problem_info.sql"),
                params![session_id, id],
                |row| row.get::<_, u64>(0),
            )
            .unwrap_or(0);

        // TODO: Send last solved languge and code from that atempt

        Response::new()
            .text(json!({
                "name": problem.name(),
                "text": problem.text(),
                "status": status
            }))
            .content(Content::JSON)
    });
}
