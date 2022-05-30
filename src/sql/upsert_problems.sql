INSERT INTO problems
VALUES (
        (
            SELECT user_id
            FROM sessions
            WHERE session_id = ?1
        ),
        ?2,
        strftime('%s', 'now'),
        CASE
            ?3
            WHEN 1 THEN NULL
            ELSE strftime('%s', 'now')
        END,
        0,
        ?3
    ) ON CONFLICT DO
UPDATE
SET state = max(?3, excluded.state),
    end_time = CASE
        ?3
        WHEN 1 THEN NULL
        ELSE strftime('%s', 'now')
    END,
    tries = CASE
        (
            SELECT state
            FROM problems
            WHERE user_id = excluded.user_id
                AND problem_id = excluded.problem_id
        )
        WHEN 2 THEN tries
        ELSE tries + 1
    END