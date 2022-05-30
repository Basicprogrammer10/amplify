INSERT INTO users (
        id,
        name,
        login,
        avatar_url,
        token,
        created,
        new
    )
VALUES (?1, ?2, ?5, ?3, ?4, strftime('%s', 'now'), 1) ON CONFLICT DO
UPDATE
SET token = ?4,
    name = ?2,
    login = ?5;