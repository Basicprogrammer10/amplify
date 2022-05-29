SELECT problem_id,
    state
FROM problems
    JOIN sessions
WHERE sessions.user_id = ?;