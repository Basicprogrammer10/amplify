CREATE TABLE IF NOT EXISTS users (
   id TEXT NOT NULL,
   name TEXT NOT NULL,
   avatar_url TEXT NOT NULL,
   token TEXT,
   created INTEGER NOT NULL,
   new INTEGER NOT NULL,
   UNIQUE(id)
)