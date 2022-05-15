use crate::{common::json_err, App, Arc};

use afire::{Method, Response, Server, SetCookie};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::GET, "/auth/logout", move |req| {
        // Get Session token
        let session = match req.cookies.iter().find(|x| x.name == "session") {
            Some(i) => &i.value,
            None => return json_err("No Session"),
        };

        // Remove token from database
        app.db
            .lock()
            .execute("DELETE FROM sessions WHERE session_id = ?", [session])
            .unwrap();

        // Remove Session Cookie
        Response::new()
            .status(308)
            .header("Location", "/")
            .cookie(SetCookie::new("session", "LOGOUT").path("/").max_age(0))
    });
}
