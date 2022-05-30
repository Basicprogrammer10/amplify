use crate::{App, Arc, Server};

mod complete;
mod logout;
mod redirect;

pub fn attach(server: &mut Server, app: Arc<App>) {
    redirect::attach(server, app.clone());
    complete::attach(server, app.clone());
    logout::attach(server, app);
}
