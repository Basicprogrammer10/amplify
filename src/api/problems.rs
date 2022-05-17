use crate::{App, Arc};

use afire::{Content, Method, Response, Server};
use serde_json::json;

pub fn attatch(server: &mut Server, app: Arc<App>) {
    server.route(Method::GET, "/api/problems", |req| {
        Response::new()
            .text(json!([
                {"name": "Simple Math", "date": "May 15 2022", "status": 0},
                {"name": "Increase Count", "date": "May 14 2022", "status": 1},
                {"name": "Pascal Triangle", "date": "May 13 2022", "status": 2}
            ]))
            .content(Content::JSON)
    });
}
