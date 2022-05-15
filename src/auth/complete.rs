use crate::{App, Arc};

use afire::{Method, Query, Response, Server};
use rusqlite::params;
use serde_json::Value;
use ureq;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::GET, "/auth/complete", move |req| {
        // Get Code from URI
        let code = match req.query.get("code") {
            Some(i) => i,
            _ => return Response::new().status(400).text("No Auth Code Found"),
        };

        // Get Access Token
        let resp = ureq::post("https://github.com/login/oauth/access_token")
            .query("client_secret", &app.cfg.github_app_secret)
            .query("client_id", &app.cfg.github_app_id)
            .query("code", &code)
            .timeout(app.cfg.req_duration)
            .call()
            .unwrap()
            .into_string()
            .unwrap();

        // Parse Response and net Token
        let token = Query::from_body(resp)
            .expect("Error Parseing Response")
            .get("access_token")
            .expect("No Access Token!?");

        // Get User Info
        let user_raw = ureq::get("https://api.github.com/user")
            .set("Authorization", &format!("token {}", token))
            .call()
            .unwrap()
            .into_reader();

        // Parse Jason
        let user: Value = serde_json::from_reader(user_raw).unwrap();
        let id = user.get("id").unwrap().as_u64().unwrap();
        let name = user.get("name").unwrap().as_str().unwrap();

        // Check if user in db already
        let new = app
            .db
            .lock()
            .query_row::<u32, _, _>("SELECT COUNT(*) FROM users WHERE id = ?", [id], |row| {
                row.get(0)
            })
            .unwrap()
            == 0;

        if new {
            app.db
                .lock()
                .execute(
                    "INSERT OR IGNORE INTO users (id, name, avatar_url, token) VALUES (?, ?, ?, ?)",
                    params![
                        id,
                        name,
                        user.get("avatar_url").unwrap().as_str().unwrap(),
                        token
                    ],
                )
                .unwrap();

            return Response::new().text(format!(
                "Welcome, {}",
                user.get("name").unwrap().as_str().unwrap()
            ));
        }

        // Update user's name and token
        app.db
            .lock()
            .execute(
                "UPDATE users SET token = ?, name = ? WHERE id = ?",
                params![token, name, id],
            )
            .unwrap();

        Response::new().text(format!(
            "Hello, {}",
            user.get("name").unwrap().as_str().unwrap()
        ))
    });
}
