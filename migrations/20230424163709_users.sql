CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY NOT NULL,
  name TEXT,
  email TEXT,
  password TEXT,
  created_at TIMESTAMP
);
