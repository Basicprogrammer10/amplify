use crate::{
    common::{current_epoch, json_err, rand_str},
    App, Arc,
};

use afire::{Method, Query, Response, Server, SetCookie};
use rusqlite::params;
use serde_json::Value;
use ureq;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::GET, "/auth/complete", move |req| {
        // Get Code from URI
        let code = match req.query.get("code") {
            Some(i) => i,
            _ => return json_err("No Auth Code Found"),
        };

        // Get and verify state
        let state = match req.query.get("state") {
            Some(i) => i,
            _ => return json_err("No State Found"),
        };

        {
            let mut os = app.oauth_state.lock();
            let real_state = match os.iter().position(|x| x.0 == state) {
                Some(i) => os.remove(i),
                None => return json_err("Invalid State"),
            };

            if current_epoch() - real_state.1 >= 60 * 10 {
                return json_err("State Expired");
            }

            if real_state.0 != state {
                return json_err("State Dosent Match");
            }
        }

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

        // Add to / Update database
        app.db
            .lock()
            .execute(
                include_str!("../sql/upsert_login.sql"),
                params![
                    id,
                    name,
                    user.get("avatar_url").unwrap().as_str().unwrap(),
                    token
                ],
            )
            .unwrap();

        // Make a new session
        let session_token = rand_str(15);
        app.db
            .lock()
            .execute(
                "INSERT INTO sessions (created, user_id, session_id) VALUES (?, ?, ?)",
                params![current_epoch(), id, session_token],
            )
            .unwrap();

        let cookie = SetCookie::new("session", session_token)
            .path("/")
            .max_age(30 * 24 * 60 * 60);

        return Response::new()
            .status(308)
            .header("Location", "/")
            .cookie(cookie);
    });
}
