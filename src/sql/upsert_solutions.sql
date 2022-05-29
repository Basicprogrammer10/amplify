INSERT
    OR REPLACE INTO solutions
VALUES (
        (
            SELECT user_id
            FROM sessions
            WHERE session_id = ?1
        ),
        ?2,
        ?3,
        ?4,
        strftime('%s', 'now')
    );