use crate::{
    common::{current_epoch, rand_str},
    App, Arc,
};

use afire::{Method, Response, Server};

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::GET, "/auth/redirect", move |_| {
        let state = rand_str(10);

        app.oauth_state
            .lock()
            .push((state.clone(), current_epoch()));

        Response::new().status(308).header(
            "Location",
            &format!(
                "https://github.com/login/oauth/authorize?client_id={}&state={}",
                app.cfg.github_app_id, state
            ),
        )
    });
}
