CREATE TABLE IF NOT EXISTS users (
   id TEXT NOT NULL,
   name TEXT NOT NULL,
   avatar_url TEXT NOT NULL,
   token TEXT,
   new INTEGER NOT NULL,
   UNIQUE(id)
)