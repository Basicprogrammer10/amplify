SELECT Count(*),
      name,
      avatar_url,
      id
FROM   sessions
      JOIN users
        ON sessions.user_id = users.id
WHERE  session_id = ?
