use afire::{Content, Method, Response, Server};
use serde_json::json;

use crate::{problems::PROBLEMS, App, Arc};

pub fn attach(server: &mut Server, app: Arc<App>) {
    server.route(Method::GET, "/api/leaderboard", move |_| {
        let db = app.db.lock();

        let mut events = Vec::new();
        // :p
        events.push(json!({"name": "Connor Slade", "url": "https://github.com/Basicprogrammer10", "avatar": "https://avatars.githubusercontent.com/u/50306817?v=4", "time": 1652400000, "msg": "<em>created amplify</em>"}));

        let mut query = db
            .prepare(include_str!("../sql/query_leaderbord.sql"))
            .unwrap();
        let mut query = query.query([]).unwrap();
        while let Some(i) = query.next().unwrap() {
            let problem_id = i.get::<_, u64>(0).unwrap();
            let problem = PROBLEMS
                .iter()
                .find(|x| x.id() == problem_id)
                .unwrap()
                .name();
            let name = i.get::<_, String>(1).unwrap();
            let avatar_url = i.get::<_, String>(2).unwrap();
            let time = i.get::<_, u64>(3).unwrap();
            let login = i.get::<_, String>(4).unwrap();

            events.push(
                json!({"name": name, "login": format!("https://github.com/{login}"), "avatar": avatar_url, "problem": problem, "problemId": problem_id,  "time": time}),
            );
        }

        let mut joins = db
            .prepare(include_str!("../sql/query_joins.sql"))
            .unwrap();
        let mut joins = joins.query([]).unwrap();
        while let Some(i) = joins.next().unwrap() {
            let name = i.get::<_, String>(0).unwrap();
            let created = i.get::<_, u64>(1).unwrap();
            let avatar_url = i.get::<_, String>(2).unwrap();
            let login = i.get::<_, String>(3).unwrap();

            events.push(json!({"name": name, "url": format!("https://github.com/{login}"), "avatar": avatar_url, "time": created, "msg": "joined"}))
        }

        events.sort_unstable_by(|a, b| b.get("time").unwrap().as_u64().unwrap().cmp(&a.get("time").unwrap().as_u64().unwrap()));

        Response::new()
            .text(json!(events))
            .content(Content::JSON)
    });
}
