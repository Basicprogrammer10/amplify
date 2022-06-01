use afire::{Content, Method, Response, Server};
use serde_json::json;

use crate::{App, Arc};

pub fn attach(server: &mut Server, app: Arc<App>) {
    server.route(Method::GET, "/api/stats", move |_| {
        let (users, tries, solves, time_spent) = app
            .db
            .lock()
            .query_row(include_str!("../sql/query_stats.sql"), [], |row| {
                Ok((
                    row.get::<_, u64>(0).unwrap_or(0),
                    row.get::<_, u64>(1).unwrap_or(0),
                    row.get::<_, u64>(2).unwrap_or(0),
                    row.get::<_, u64>(3).unwrap_or(0),
                ))
            })
            .unwrap();

        Response::new()
            .text(
                json!({"users": users, "tries": tries, "solves": solves, "timeSpent": time_spent}),
            )
            .content(Content::JSON)
    });
}
