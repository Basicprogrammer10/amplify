SELECT (
        SELECT state
        FROM problems
        WHERE user_id = sessions.user_id
            AND problem_id = ?2
    ),
    solutions.created,
    language,
    code
FROM solutions
    JOIN sessions ON sessions.user_id = solutions.user_id
WHERE sessions.session_id = ?1
    AND problem_id = ?2
ORDER BY solutions.created DESC;