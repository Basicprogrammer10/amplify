SELECT state,
    solutions.created,
    language,
    code
FROM solutions
    JOIN sessions ON sessions.user_id = solutions.user_id
WHERE sessions.user_id = ?
    AND problem_id = ?;
