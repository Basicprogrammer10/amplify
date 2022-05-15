use crate::{App, Arc, Server};

mod complete;
mod logout;
mod redirect;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    redirect::attatch(server, app.clone());
    complete::attatch(server, app.clone());
    logout::attatch(server, app);
}
