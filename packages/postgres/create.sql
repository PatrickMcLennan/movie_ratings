SELECT 'CREATE DATABASE movie_ratings_dev'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'movie_ratings_dev')\gexec

SELECT 'CREATE DATABASE movie_ratings_test'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'movie_ratings_test')\gexec
