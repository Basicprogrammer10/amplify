INSERT INTO users (
        id,
        name,
        avatar_url,
        token,
        created,
        new
    )
VALUES (?1, ?2, ?3, ?4, strftime('%s', 'now'), 1) ON CONFLICT DO
UPDATE
SET token = ?4,
    name = ?2