INSERT INTO users (id, name, avatar_url, token, new) VALUES (?1, ?2, ?3, ?4, 1) ON CONFLICT DO UPDATE SET token = ?4, name = ?2
