use crate::{App, Arc, Server};

mod generic_self;
mod problems;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    generic_self::attatch(server, app.clone());
    problems::attatch(server, app);
}
