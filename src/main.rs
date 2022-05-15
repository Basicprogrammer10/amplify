use std::sync::Arc;

use afire::{Content, Response, Server};

mod api;
mod app;
mod auth;
mod common;
mod config;
mod database;
mod problems;
use app::App;

fn main() {
    let app = Arc::new(App::new());
    database::init(&app);

    let mut server = Server::new(&app.cfg.host, app.cfg.port);
    auth::attatch(&mut server, app.clone());
    api::attatch(&mut server, app);

    server.error_handler(
        |req, err| match req.path.split('/').nth(1).unwrap_or_default() {
            "auth" | "api" => {
                return Response::new()
                    .status(500)
                    .text(format!(r#"{{"error": "{}"}}"#, err))
                    .content(Content::JSON)
            }
            _ => {
                return Response::new()
                    .status(500)
                    .text(format!("Internal Server Error :/\nError: {}", err))
                    .content(Content::TXT)
            }
        },
    );

    server.start();
}
