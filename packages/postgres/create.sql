SELECT 'CREATE DATABASE movie_ratings_development'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'movie_ratings_development')\gexec

SELECT 'CREATE DATABASE movie_ratings_test'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'movie_ratings_test')\gexec
