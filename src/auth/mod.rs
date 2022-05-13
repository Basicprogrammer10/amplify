use crate::{App, Arc, Server};

mod auth_redirect;

pub fn attatch(server: &mut Server, app: &Arc<App>) {
    auth_redirect::attatch(server, app);
}
