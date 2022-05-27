SELECT state
FROM solutions
    JOIN sessions
WHERE sessions.user_id = ?
    AND problem_id = ?;