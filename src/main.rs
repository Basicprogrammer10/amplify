use std::fs;
use std::sync::Arc;

use afire::{
    extension::{Logger, ServeStatic},
    Content, Middleware, Response, Server,
};
use serde_json::json;

mod api;
mod app;
mod auth;
mod common;
mod config;
mod database;
mod problems;
use app::App;

fn main() {
    let _ = fs::create_dir("data/tmp");

    let app = Arc::new(App::new());
    database::init(&app);

    let mut server = Server::new(&app.cfg.host, app.cfg.port);
    ServeStatic::new("web/static").attach(&mut server);
    Logger::new().attach(&mut server);

    auth::attach(&mut server, app.clone());
    api::attach(&mut server, app);

    server.error_handler(|req, err| match req {
        Ok(i) => match i.path.split('/').nth(1).unwrap_or_default() {
            "auth" | "api" => Response::new()
                .status(500)
                .text(json!({ "error": err }))
                .content(Content::JSON),
            _ => Response::new()
                .status(500)
                .text(format!("Internal Server Error :/\nError: {}", err))
                .content(Content::TXT),
        },
        Err(e) => Response::new().text(format!("{:?}", e)),
    });

    server.start_threaded(16).unwrap();
}
