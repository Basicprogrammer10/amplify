use crate::{App, Arc, Server};

mod get_generic_self;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    get_generic_self::attatch(server, app);
}
