use crate::{App, Arc, Server};

mod complete;
mod redirect;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    redirect::attatch(server, app.clone());
    complete::attatch(server, app);
}
