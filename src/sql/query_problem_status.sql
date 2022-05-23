SELECT problem_id,
    state
FROM solutions
    JOIN sessions
WHERE sessions.user_id = ?;