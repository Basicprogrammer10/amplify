INSERT INTO users (id, name, avatar_url, token) VALUES (?1, ?2, ?3, ?4) ON CONFLICT DO UPDATE SET token = ?4, name = ?2
