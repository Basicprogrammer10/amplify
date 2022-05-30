INSERT
    OR IGNORE INTO problems
VALUES (
        (
            SELECT user_id
            FROM sessions
                WHERE session_id = ?1
        ),
        ?2,
        strftime('%s', 'now'),
        NULL,
        0,
        1
    );