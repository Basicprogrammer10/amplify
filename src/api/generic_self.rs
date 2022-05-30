use crate::{
    common::{get_cookie, json_err},
    App, Arc,
};

use afire::{Content, Method, Response, Server};
use serde_json::json;

pub fn attach(server: &mut Server, app: Arc<App>) {
    server.route(Method::GET, "/api/generic_self", move |req| {
        // Get Session
        let session_id = match get_cookie(&req, "session") {
            Some(i) => i.value,
            None => return json_err("No Session!?"),
        };

        // Query Database
        let session = app
            .db
            .lock()
            .query_row::<(u64, String, String, String, u64, Option<String>), _, _>(
                include_str!("../sql/query_generic_info.sql"),
                [&session_id],
                |row| {
                    Ok((
                        row.get(0)?,
                        row.get(1).unwrap_or_default(),
                        row.get(2).unwrap_or_default(),
                        row.get(3).unwrap_or_default(),
                        row.get(4).unwrap_or_default(),
                        row.get(5).ok(),
                    ))
                },
            )
            .unwrap();
        let new = session.4 > 0;

        // Set User to not new
        app.db
            .lock()
            .execute(include_str!("../sql/update_not_new.sql"), [session_id])
            .unwrap();

        if session.0 == 0 {
            return json_err("Session Not Found");
        }

        // Send Response
        Response::new()
            .text(json!({"name": session.1, "avatar": session.2, "id": session.3, "new": new, "lang": session.5}).to_string())
            .content(Content::JSON)
    });
}
