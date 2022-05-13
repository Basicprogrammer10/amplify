use crate::{App, Arc};

use afire::{Method, Response, Server};

pub fn attatch(server: &mut Server, app: &Arc<App>) {
    let path = format!(
        "https://github.com/login/oauth/authorize?client_id={}",
        app.cfg.github_app_id
    );

    server.route(Method::GET, "/auth/redirect", move |_| {
        Response::new().status(308).header("Location", &path)
    });
}
