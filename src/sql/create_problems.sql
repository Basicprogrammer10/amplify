CREATE TABLE IF NOT EXISTS problems (
    user_id TEXT NOT NULL,
    problem_id INTEGER NOT NULL,
    start_time INTEGER NOT NULL,
    end_time INTEGER,
    tries INTEGER NOT NULL,
    state INTEGER NOT NULL,
    UNIQUE(user_id, problem_id)
)