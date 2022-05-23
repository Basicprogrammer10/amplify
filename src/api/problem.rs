use afire::{Content, Method, Response, Server};
use serde_json::json;

use crate::problems::PROBLEMS;
use crate::{App, Arc};

pub fn attatch(server: &mut Server, _app: Arc<App>) {
    server.route(Method::GET, "/api/problem/{id}", move |req| {
        let id = req
            .path_param("id")
            .unwrap()
            .parse::<u64>()
            .expect("Id is not a u64");

        let problem = PROBLEMS
            .iter()
            .find(|x| x.id() == id)
            .expect("Problem Not Found");

        Response::new()
            .text(json!({
                "name": problem.name(),
                "text": problem.text()
            }))
            .content(Content::JSON)
    });
}
