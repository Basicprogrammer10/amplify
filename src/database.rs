use crate::{App, Arc};

pub fn init(app: &Arc<App>) {
    let mut conn = app.db.lock();

    conn.pragma_update(None, "journal_mode", "WAL").unwrap();
    conn.pragma_update(None, "synchronous", "NORMAL").unwrap();
    let trans = conn.transaction().unwrap();

    // Init tables
    for i in [include_str!("sql/create_users.sql")] {
        trans.execute(i, []).unwrap();
    }
    trans.commit().unwrap();
}
