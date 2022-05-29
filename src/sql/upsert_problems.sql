INSERT INTO problems
VALUES (
        (
            SELECT user_id
            FROM sessions
            WHERE session_id = ?1
        ),
        ?2,
        ?3
    ) ON CONFLICT DO
UPDATE
SET state = max(?3, excluded.state)