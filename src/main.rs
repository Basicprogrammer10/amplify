use std::sync::Arc;

use afire::Server;

mod api;
mod app;
mod config;
mod database;
mod problems;
use app::App;

fn main() {
    let app = Arc::new(App::new());
    database::init(&app);

    let mut server = Server::new(&app.cfg.host, app.cfg.port);
    api::attatch(&mut server, app);

    server.start();
}
