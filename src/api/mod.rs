use crate::{App, Arc, Server};

mod generic_self;
mod langs;
mod leaderboard;
mod problem;
mod problems;
mod profile;
mod solve;
mod stats;

pub fn attach(server: &mut Server, app: Arc<App>) {
    generic_self::attach(server, app.clone());
    langs::attach(server, app.clone());
    leaderboard::attach(server, app.clone());
    problem::attach(server, app.clone());
    problems::attach(server, app.clone());
    profile::attach(server, app.clone());
    solve::attach(server, app.clone());
    stats::attach(server, app);
}
