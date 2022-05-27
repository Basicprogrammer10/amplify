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

        let db = app.db.lock();
        let mut query = db
            .prepare(include_str!("../sql/query_problem_info.sql"))
            .unwrap();
        let mut status = query.query(params![session_id, id]).unwrap();

        let mut sol = Vec::new();
        let mut max_state = 0;
        while let Some(i) = status.next().unwrap() {
            let state = i.get::<_, u64>(0).unwrap();
            let date = i.get::<_, u64>(1).unwrap();
            let lang = i.get::<_, String>(2).unwrap();
            let code = i.get::<_, String>(3).unwrap();

            max_state = max_state.max(state);
            sol.push(json!({"lang": lang, "code": code, "date": date}));
        }

        Response::new()
            .text(json!({
                "name": problem.name(),
                "text": problem.text(),
                "status": max_state,
                "id": id,
                "solutions": sol
            }))
            .content(Content::JSON)
    });
}
