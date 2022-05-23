use crate::{App, Arc, Server};

mod generic_self;
mod langs;
mod problem;
mod problems;
mod solve;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    generic_self::attatch(server, app.clone());
    langs::attatch(server, app.clone());
    problem::attatch(server, app.clone());
    problems::attatch(server, app.clone());
    solve::attatch(server, app);
}
