CREATE TABLE IF NOT EXISTS solutions (
    user_id TEXT NOT NULL,
    problem_id INTEGER NOT NULL,
    state INTEGER NOT NULL,
    code TEXT NOT NULL,
    language TEXT NOT NULL,
    created INTEGER NOT NULL,
    UNIQUE(user_id, problem_id, language)
)