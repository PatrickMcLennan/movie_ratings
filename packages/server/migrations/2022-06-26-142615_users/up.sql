CREATE TABLE users (
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  email VARCHAR(320) UNIQUE NOT NULL,
  first_name VARCHAR(80) NOT NULL,
  id SERIAL PRIMARY KEY,
  last_name VARCHAR(80) NOT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX ON users(email);
SELECT diesel_manage_updated_at('users');
