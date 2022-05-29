SELECT Count(*),
      name,
      avatar_url,
      id,
      new,
(
            SELECT language
            FROM solutions
            WHERE user_id = sessions.user_id
            ORDER BY created DESC
            LIMIT 1
      )
FROM sessions
      JOIN users ON sessions.user_id = users.id
WHERE session_id = ?;