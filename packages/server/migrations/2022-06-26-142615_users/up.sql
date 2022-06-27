CREATE TABLE users (
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  email VARCHAR(255) UNIQUE NOT NULL,
  first_name VARCHAR(255) NOT NULL,
  id SERIAL PRIMARY KEY,
  last_name VARCHAR(255) NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX ON users(email);
SELECT diesel_manage_updated_at('users');

INSERT INTO users (email, first_name, last_name) 
VALUES ('king@theking.com', 'Elvis', 'Presley');
